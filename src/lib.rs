#![no_std]

use embassy_time::{Duration, Timer};
use esp_hal::rmt::{ConstChannelAccess, Tx};
use esp_hal_smartled::SmartLedsAdapter;
use log::info;
use rs_matter::{
    dm::{
        clusters::on_off::{self, ClusterHandler as _},
        Dataver,
    },
    tlv::AsNullable,
    utils::{cell::RefCell, maybe::Maybe},
    with,
};

use crate::color_control::{ColorMode, CommandId, Feature};
use smart_leds::{SmartLedsWrite as _, RGB8, RGBA};

pub mod color_control;
pub mod credentials;
pub mod identify;
pub mod level_control;
pub mod nvs;
pub mod util;

#[macro_export]
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

pub const LED_COUNT: usize = 1;

pub const LED_SIZE: usize = 1 + LED_COUNT * 32;

type Led = SmartLedsAdapter<ConstChannelAccess<Tx, 0>, LED_SIZE, RGBA<u8>>;

/// Tracks an active transition/animation
#[derive(Clone, Copy)]
struct TransitionState {
    active: bool,
    transition_type: u8, // 0=level, 1=HS, 2=XY, 3=CT
    // Start values
    start_level: u8,
    start_hue: u8,
    start_sat: u8,
    start_x: u16,
    start_y: u16,
    start_ct: u16,
    // Target values
    target_level: u8,
    target_hue: u8,
    target_sat: u8,
    target_x: u16,
    target_y: u16,
    target_ct: u16,
    // Timing (in deciseconds, 1ds = 100ms)
    elapsed_ds: u16,
    total_ds: u16,
}

impl TransitionState {
    const fn inactive() -> Self {
        TransitionState {
            active: false,
            transition_type: 0,
            start_level: 0,
            start_hue: 0,
            start_sat: 0,
            start_x: 0,
            start_y: 0,
            start_ct: 0,
            target_level: 0,
            target_hue: 0,
            target_sat: 0,
            target_x: 0,
            target_y: 0,
            target_ct: 0,
            elapsed_ds: 0,
            total_ds: 0,
        }
    }
}

pub struct LightController {
    dataver: Dataver,
    led: RefCell<Led>,
    on_off: RefCell<bool>,
    hue: RefCell<u8>,
    saturation: RefCell<u8>,
    // LevelControl cluster state
    current_level: RefCell<u8>, // 0-254 (1=minimum light, 254=maximum)
    // ColorControl XY mode state
    current_x: RefCell<u16>, // 0-65535 (0.0-1.0)
    current_y: RefCell<u16>, // 0-65535 (0.0-1.0)
    color_mode: RefCell<u8>, // 0=HS, 1=XY, 2=CT
    // ColorControl Color Temperature mode state
    color_temperature_mireds: RefCell<u16>, // 153-500 mireds (6500K-2000K)
    // Identify cluster state
    identify_time_ds: RefCell<u16>, // Remaining identify time in deciseconds
    identify_phase: RefCell<u8>,    // Blink animation phase (0-9 for 1Hz blink)
    // Transition state for smooth animations
    transition: RefCell<TransitionState>,
}

impl LightController {
    pub const ON_OFF_CLUSTER: rs_matter::dm::Cluster<'static> = on_off::OnOffHandler::CLUSTER;
    pub const COLOR_CONTROL_CLUSTER: rs_matter::dm::Cluster<'static> = color_control::FULL_CLUSTER
        .with_attrs(with!(required))
        .with_cmds(with!(
            CommandId::MoveToHue
                | CommandId::MoveHue
                | CommandId::StepHue
                | CommandId::MoveToSaturation
                | CommandId::MoveSaturation
                | CommandId::StepSaturation
                | CommandId::MoveToHueAndSaturation
                | CommandId::MoveToColor
                | CommandId::MoveColor
                | CommandId::StepColor
                | CommandId::MoveToColorTemperature
                | CommandId::EnhancedMoveToHue
                | CommandId::EnhancedMoveHue
                | CommandId::EnhancedStepHue
                | CommandId::EnhancedMoveToHueAndSaturation
                | CommandId::ColorLoopSet
                | CommandId::StopMoveStep
                | CommandId::MoveColorTemperature
                | CommandId::StepColorTemperature
        ))
        .with_features(Feature::all().bits());

    pub const LEVEL_CONTROL_CLUSTER: rs_matter::dm::Cluster<'static> =
        crate::level_control::FULL_CLUSTER.with_attrs(with!(required));

    pub const IDENTIFY_CLUSTER: rs_matter::dm::Cluster<'static> =
        crate::identify::FULL_CLUSTER.with_attrs(with!(required));

    pub fn new(dataver: Dataver, led: Led) -> Self {
        LightController {
            dataver,
            led: RefCell::new(led),
            on_off: RefCell::new(false),
            hue: RefCell::new(0),
            saturation: RefCell::new(0),
            current_level: RefCell::new(254), // Start at maximum brightness
            current_x: RefCell::new(0),       // Start at x=0.0
            current_y: RefCell::new(0),       // Start at y=0.0
            color_mode: RefCell::new(0),      // Start in HS mode
            color_temperature_mireds: RefCell::new(250), // Start at 4000K (neutral white)
            identify_time_ds: RefCell::new(0), // Not identifying
            identify_phase: RefCell::new(0),  // Blink phase
            transition: RefCell::new(TransitionState::inactive()),
        }
    }

    fn update_led(&self) {
        let mut led = self.led.borrow_mut();

        // Identify mode takes priority over normal operation
        if *self.identify_time_ds.borrow() > 0 {
            let phase = *self.identify_phase.borrow();
            if phase < 5 {
                // First half of blink cycle - bright white
                led.write(
                    [RGBA {
                        r: 255,
                        g: 255,
                        b: 255,
                        a: 0,
                    }; LED_COUNT],
                )
                .unwrap();
            } else {
                // Second half - off
                led.write(
                    [RGBA {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0,
                    }; LED_COUNT],
                )
                .unwrap();
            }
            return;
        }

        // Normal operation
        let on = *self.on_off.borrow();

        if on {
            let level = *self.current_level.borrow();
            let mode = *self.color_mode.borrow();

            let RGB8 { r, g, b } = match mode {
                // Mode 0: Hue and Saturation
                0 => {
                    let hue = *self.hue.borrow();
                    let sat = *self.saturation.borrow();
                    // Convert level (0-254) to value (0-100) for HSV
                    let value = ((level as u16 * 100) / 254) as u8;
                    hsv_to_rgb(hue, sat, value)
                }
                // Mode 1: XY
                1 => {
                    let x = *self.current_x.borrow();
                    let y = *self.current_y.borrow();
                    xy_to_rgb(x, y, level)
                }
                // Mode 2: Color Temperature
                2 => {
                    let mireds = *self.color_temperature_mireds.borrow();
                    // Convert mireds to Kelvin: K = 1,000,000 / mireds
                    let kelvin = (1_000_000 / mireds as u32) as u16;
                    kelvin_to_rgb(kelvin, level)
                }
                // Unknown mode - default to black
                _ => RGB8 { r: 0, g: 0, b: 0 },
            };

            // Note: LED has swapped R and G channels
            led.write(
                [RGBA {
                    r: g,
                    g: r,
                    b,
                    a: 0,
                }; LED_COUNT],
            )
            .unwrap();
        } else {
            led.write(
                [RGBA {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0,
                }; LED_COUNT],
            )
            .unwrap();
        }
    }

    /// Update transition state - called periodically (every 100ms = 1 decisecond)
    /// Returns true if a transition is active
    pub fn update_transition(&self) -> bool {
        let mut transition = self.transition.borrow_mut();

        if !transition.active {
            return false;
        }

        // Increment elapsed time
        transition.elapsed_ds += 1;

        // Check if transition is complete
        if transition.elapsed_ds >= transition.total_ds {
            // Transition complete - set final values
            match transition.transition_type {
                0 => {
                    // Level transition
                    *self.current_level.borrow_mut() = transition.target_level;
                }
                1 => {
                    // HS transition
                    *self.hue.borrow_mut() = transition.target_hue;
                    *self.saturation.borrow_mut() = transition.target_sat;
                    *self.current_level.borrow_mut() = transition.target_level;
                }
                2 => {
                    // XY transition
                    *self.current_x.borrow_mut() = transition.target_x;
                    *self.current_y.borrow_mut() = transition.target_y;
                    *self.current_level.borrow_mut() = transition.target_level;
                }
                3 => {
                    // CT transition
                    *self.color_temperature_mireds.borrow_mut() = transition.target_ct;
                    *self.current_level.borrow_mut() = transition.target_level;
                }
                _ => {}
            }
            transition.active = false;
            drop(transition);
            self.update_led();
            self.notify_dataver_changed();
            return false;
        }

        // Calculate interpolation factor (0.0 to 1.0)
        let progress = transition.elapsed_ds as f32 / transition.total_ds as f32;

        // Linear interpolation helper
        let lerp_u8 = |start: u8, target: u8, t: f32| -> u8 {
            let start_f = start as f32;
            let target_f = target as f32;
            (start_f + (target_f - start_f) * t) as u8
        };
        let lerp_u16 = |start: u16, target: u16, t: f32| -> u16 {
            let start_f = start as f32;
            let target_f = target as f32;
            (start_f + (target_f - start_f) * t) as u16
        };

        // Interpolate based on transition type
        match transition.transition_type {
            0 => {
                // Level transition
                let new_level = lerp_u8(transition.start_level, transition.target_level, progress);
                *self.current_level.borrow_mut() = new_level;
            }
            1 => {
                // HS transition
                let new_hue = lerp_u8(transition.start_hue, transition.target_hue, progress);
                let new_sat = lerp_u8(transition.start_sat, transition.target_sat, progress);
                let new_level = lerp_u8(transition.start_level, transition.target_level, progress);
                *self.hue.borrow_mut() = new_hue;
                *self.saturation.borrow_mut() = new_sat;
                *self.current_level.borrow_mut() = new_level;
            }
            2 => {
                // XY transition
                let new_x = lerp_u16(transition.start_x, transition.target_x, progress);
                let new_y = lerp_u16(transition.start_y, transition.target_y, progress);
                let new_level = lerp_u8(transition.start_level, transition.target_level, progress);
                *self.current_x.borrow_mut() = new_x;
                *self.current_y.borrow_mut() = new_y;
                *self.current_level.borrow_mut() = new_level;
            }
            3 => {
                // CT transition
                let new_ct = lerp_u16(transition.start_ct, transition.target_ct, progress);
                let new_level = lerp_u8(transition.start_level, transition.target_level, progress);
                *self.color_temperature_mireds.borrow_mut() = new_ct;
                *self.current_level.borrow_mut() = new_level;
            }
            _ => {}
        }

        drop(transition);
        self.update_led();
        true
    }

    /// Update identify state - called by transition task every decisecond
    /// Returns true if currently identifying
    fn update_identify(&self) -> bool {
        let mut identify_time = self.identify_time_ds.borrow_mut();
        if *identify_time > 0 {
            *identify_time -= 1;
            // Update blink phase (0-9 for 1Hz blink: 0-4 = on, 5-9 = off)
            let current_phase = *self.identify_phase.borrow();
            *self.identify_phase.borrow_mut() = (current_phase + 1) % 10;
            true
        } else {
            false
        }
    }

    pub fn get_on_off(&self) -> bool {
        *self.on_off.borrow()
    }

    pub fn set_on_off(&self, value: bool) {
        *self.on_off.borrow_mut() = value;
        self.update_led();
    }

    fn notify_dataver_changed(&self) {
        self.dataver.changed();
    }
}

fn hsv_to_rgb(h: u8, s: u8, v: u8) -> RGB8 {
    // Convert from u8 ranges to normal ranges
    let h_degrees = (h as f32 / 255.0) * 360.0;
    let s_norm = s as f32 / 255.0;
    let v_norm = v as f32 / 255.0;

    // Determine which sector of the color wheel we're in (0-5)
    let h_sector = h_degrees / 60.0;
    let sector = h_sector as i32;
    let f = h_sector - sector as f32;

    // Calculate intermediate values
    let p = v_norm * (1.0 - s_norm);
    let q = v_norm * (1.0 - s_norm * f);
    let t = v_norm * (1.0 - s_norm * (1.0 - f));

    // Assign RGB based on sector
    let (r, g, b) = match sector {
        0 => (v_norm, t, p),
        1 => (q, v_norm, p),
        2 => (p, v_norm, t),
        3 => (p, q, v_norm),
        4 => (t, p, v_norm),
        _ => (v_norm, p, q), // 5 or wraps to 6
    };

    RGB8 {
        r: (r * 255.0) as u8,
        g: (g * 255.0) as u8,
        b: (b * 255.0) as u8,
    }
}

/// Convert CIE 1931 XY coordinates to RGB
fn xy_to_rgb(x: u16, y: u16, brightness: u8) -> RGB8 {
    // Convert u16 (0-65535) to float (0.0-1.0)
    let x_norm = x as f32 / 65535.0;
    let y_norm = y as f32 / 65535.0;

    // Handle edge case: y = 0 would cause division by zero
    if y_norm < 0.0001 {
        return RGB8 { r: 0, g: 0, b: 0 };
    }

    // Convert XY to XYZ color space
    let z = 1.0 - x_norm - y_norm;
    let y = brightness as f32 / 254.0; // Normalized brightness
    let x = (y / y_norm) * x_norm;
    let z = (y / y_norm) * z;

    // Apply XYZ to sRGB matrix (D65 illuminant)
    let r_linear = x * 3.2406 + y * -1.5372 + z * -0.4986;
    let g_linear = x * -0.9689 + y * 1.8758 + z * 0.0415;
    let b_linear = x * 0.0557 + y * -0.2040 + z * 1.0570;

    // Apply sRGB gamma correction
    // TODO: Proper gamma correction requires powf which needs libm crate
    // For now, use linear RGB (will look slightly washed out but functional)
    let r_gamma = r_linear;
    let g_gamma = g_linear;
    let b_gamma = b_linear;

    // Clamp to valid range and convert to u8
    let clamp = |c: f32| -> u8 {
        if c < 0.0 {
            0
        } else if c > 1.0 {
            255
        } else {
            (c * 255.0) as u8
        }
    };

    RGB8 {
        r: clamp(r_gamma),
        g: clamp(g_gamma),
        b: clamp(b_gamma),
    }
}

/// Convert RGB to CIE 1931 XY coordinates
fn rgb_to_xy(r: u8, g: u8, b: u8) -> (u16, u16) {
    // Normalize RGB to 0.0-1.0
    let r_norm = r as f32 / 255.0;
    let g_norm = g as f32 / 255.0;
    let b_norm = b as f32 / 255.0;

    // Reverse gamma correction (sRGB to linear)
    // TODO: Proper gamma expansion requires powf which needs libm crate
    // For now, use linear RGB
    let r_linear = r_norm;
    let g_linear = g_norm;
    let b_linear = b_norm;

    // Apply sRGB to XYZ matrix (D65 illuminant)
    let x = r_linear * 0.4124 + g_linear * 0.3576 + b_linear * 0.1805;
    let y = r_linear * 0.2126 + g_linear * 0.7152 + b_linear * 0.0722;
    let z = r_linear * 0.0193 + g_linear * 0.1192 + b_linear * 0.9505;

    // Convert XYZ to xy chromaticity coordinates
    let sum = x + y + z;
    if sum < 0.0001 {
        // Black or very dark - return default point
        return (0, 0);
    }

    let x = x / sum;
    let y = y / sum;

    // Convert to u16 (0-65535)
    let x_u16 = (x * 65535.0).clamp(0.0, 65535.0) as u16;
    let y_u16 = (y * 65535.0).clamp(0.0, 65535.0) as u16;

    (x_u16, y_u16)
}

/// Convert color temperature (in Kelvin) to RGB
/// Simplified linear interpolation approach (no powf/ln needed for no_std)
fn kelvin_to_rgb(kelvin: u16, brightness: u8) -> RGB8 {
    // Clamp kelvin to typical range (2000K-6500K for white LEDs)
    let kelvin = kelvin.clamp(2000, 6500);

    let r: f32;
    let g: f32;
    let b: f32;

    // Simplified color temperature approximation using linear interpolation
    // 2000K = warm (255, 147, 44)
    // 4000K = neutral (255, 228, 206)
    // 6500K = cool (255, 254, 250)

    if kelvin <= 4000 {
        // Interpolate between warm (2000K) and neutral (4000K)
        let t = (kelvin - 2000) as f32 / 2000.0; // 0.0 to 1.0
        r = 255.0;
        g = 147.0 + (228.0 - 147.0) * t;
        b = 44.0 + (206.0 - 44.0) * t;
    } else {
        // Interpolate between neutral (4000K) and cool (6500K)
        let t = (kelvin - 4000) as f32 / 2500.0; // 0.0 to 1.0
        r = 255.0;
        g = 228.0 + (254.0 - 228.0) * t;
        b = 206.0 + (250.0 - 206.0) * t;
    }

    // Scale by brightness
    let brightness_factor = brightness as f32 / 254.0;

    RGB8 {
        r: (r * brightness_factor) as u8,
        g: (g * brightness_factor) as u8,
        b: (b * brightness_factor) as u8,
    }
}

impl color_control::ClusterHandler for LightController {
    const CLUSTER: rs_matter::dm::Cluster<'static> = Self::COLOR_CONTROL_CLUSTER;

    fn dataver(&self) -> u32 {
        self.dataver.get()
    }

    fn dataver_changed(&self) {
        self.notify_dataver_changed();
    }

    fn color_mode(
        &self,
        _ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<u8, rs_matter::error::Error> {
        Ok(*self.color_mode.borrow())
    }

    fn options(&self, ctx: impl rs_matter::dm::ReadContext) -> Result<u8, rs_matter::error::Error> {
        Ok(1)
    }

    fn number_of_primaries(
        &self,
        ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
        Ok(Maybe::some(1))
    }

    fn enhanced_color_mode(
        &self,
        _ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<u8, rs_matter::error::Error> {
        Ok(*self.color_mode.borrow())
    }

    fn color_capabilities(
        &self,
        ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<u16, rs_matter::error::Error> {
        Ok(color_control::ColorCapabilities::all().bits())
    }

    fn current_x(
        &self,
        _ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<u16, rs_matter::error::Error> {
        Ok(*self.current_x.borrow())
    }

    fn current_y(
        &self,
        _ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<u16, rs_matter::error::Error> {
        Ok(*self.current_y.borrow())
    }

    fn color_temperature_mireds(
        &self,
        _ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<u16, rs_matter::error::Error> {
        Ok(*self.color_temperature_mireds.borrow())
    }

    fn color_temp_physical_min_mireds(
        &self,
        _ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<u16, rs_matter::error::Error> {
        Ok(153) // 6500K (cool white)
    }

    fn color_temp_physical_max_mireds(
        &self,
        _ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<u16, rs_matter::error::Error> {
        Ok(500) // 2000K (warm white)
    }

    fn set_options(
        &self,
        ctx: impl rs_matter::dm::WriteContext,
        value: u8,
    ) -> Result<(), rs_matter::error::Error> {
        info!("set_options: {value}");
        todo!()
    }

    fn handle_move_to_hue(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveToHueRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("move to hue {request:?}");
        todo!()
    }

    fn handle_move_hue(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveHueRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("move hue {request:?}");
        todo!()
    }

    fn handle_step_hue(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::StepHueRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("step hue {request:?}");
        todo!()
    }

    fn handle_move_to_saturation(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveToSaturationRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("move to sat {request:?}");
        todo!()
    }

    fn handle_move_saturation(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveSaturationRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("move sat {request:?}");
        todo!()
    }

    fn handle_step_saturation(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::StepSaturationRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("set sat {request:?}");
        todo!()
    }

    fn handle_move_to_hue_and_saturation(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveToHueAndSaturationRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        let hue = request.hue()?;
        let saturation = request.saturation()?;
        let transition_time = request.transition_time()?;
        info!(
            "move to hue {} sat {} over {} ds",
            hue, saturation, transition_time
        );

        // Switch to HS color mode
        *self.color_mode.borrow_mut() = ColorMode::CurrentHueAndCurrentSaturation as u8;

        if transition_time == 0 {
            // Instant change
            *self.hue.borrow_mut() = hue;
            *self.saturation.borrow_mut() = saturation;

            // Sync XY values
            let level = *self.current_level.borrow();
            let value = ((level as u16 * 100) / 254) as u8;
            let RGB8 { r, g, b } = hsv_to_rgb(hue, saturation, value);
            let (x, y) = rgb_to_xy(r, g, b);
            *self.current_x.borrow_mut() = x;
            *self.current_y.borrow_mut() = y;

            self.update_led();
            self.notify_dataver_changed();
        } else {
            // Start transition
            let mut transition = self.transition.borrow_mut();
            *transition = TransitionState {
                active: true,
                transition_type: 1, // HS transition
                start_level: *self.current_level.borrow(),
                start_hue: *self.hue.borrow(),
                start_sat: *self.saturation.borrow(),
                target_level: *self.current_level.borrow(), // Keep level same
                target_hue: hue,
                target_sat: saturation,
                start_x: 0,
                start_y: 0,
                start_ct: 0,
                target_x: 0,
                target_y: 0,
                target_ct: 0,
                elapsed_ds: 0,
                total_ds: transition_time,
            };
        }

        Ok(())
    }

    fn handle_move_to_color(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveToColorRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        let x = request.color_x()?;
        let y = request.color_y()?;
        let transition_time = request.transition_time()?;
        info!("move to color x={}, y={} over {} ds", x, y, transition_time);

        // Switch to XY color mode
        *self.color_mode.borrow_mut() = ColorMode::CurrentXAndCurrentY as u8;

        if transition_time == 0 {
            // Instant change
            *self.current_x.borrow_mut() = x;
            *self.current_y.borrow_mut() = y;
            self.update_led();
            self.notify_dataver_changed();
        } else {
            // Start transition
            let mut transition = self.transition.borrow_mut();
            *transition = TransitionState {
                active: true,
                transition_type: 2, // XY transition
                start_level: *self.current_level.borrow(),
                start_x: *self.current_x.borrow(),
                start_y: *self.current_y.borrow(),
                target_level: *self.current_level.borrow(), // Keep level same
                target_x: x,
                target_y: y,
                start_hue: 0,
                start_sat: 0,
                start_ct: 0,
                target_hue: 0,
                target_sat: 0,
                target_ct: 0,
                elapsed_ds: 0,
                total_ds: transition_time,
            };
        }

        Ok(())
    }

    fn handle_move_color(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveColorRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("move color {request:?}");
        todo!()
    }

    fn handle_step_color(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::StepColorRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("step color {request:?}");
        todo!()
    }

    fn handle_move_to_color_temperature(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveToColorTemperatureRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        let mireds = request.color_temperature_mireds()?;
        let transition_time = request.transition_time()?;

        // Clamp to physical range (153-500 mireds = 6500K-2000K)
        let mireds = mireds.clamp(153, 500);
        info!(
            "move to color temperature: {} mireds over {} ds",
            mireds, transition_time
        );

        // Switch to Color Temperature mode
        *self.color_mode.borrow_mut() = ColorMode::ColorTemperature as u8;

        if transition_time == 0 {
            // Instant change
            *self.color_temperature_mireds.borrow_mut() = mireds;
            self.update_led();
            self.notify_dataver_changed();
        } else {
            // Start transition
            let mut transition = self.transition.borrow_mut();
            *transition = TransitionState {
                active: true,
                transition_type: 3, // CT transition
                start_level: *self.current_level.borrow(),
                start_ct: *self.color_temperature_mireds.borrow(),
                target_level: *self.current_level.borrow(), // Keep level same
                target_ct: mireds,
                start_hue: 0,
                start_sat: 0,
                start_x: 0,
                start_y: 0,
                target_hue: 0,
                target_sat: 0,
                target_x: 0,
                target_y: 0,
                elapsed_ds: 0,
                total_ds: transition_time,
            };
        }

        Ok(())
    }

    fn handle_enhanced_move_to_hue(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::EnhancedMoveToHueRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("move enh to hue {request:?}");
        todo!()
    }

    fn handle_enhanced_move_hue(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::EnhancedMoveHueRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("move enh hue {request:?}");
        todo!()
    }

    fn handle_enhanced_step_hue(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::EnhancedStepHueRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("step enh hue {request:?}");
        todo!()
    }

    fn handle_enhanced_move_to_hue_and_saturation(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::EnhancedMoveToHueAndSaturationRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("move enh to hue sat {request:?}");
        todo!()
    }

    fn handle_color_loop_set(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::ColorLoopSetRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("color loop set {request:?}");
        todo!()
    }

    fn handle_stop_move_step(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::StopMoveStepRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("stop move step {request:?}");
        todo!()
    }

    fn handle_move_color_temperature(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveColorTemperatureRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("move temp {request:?}");
        todo!()
    }

    fn handle_step_color_temperature(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::StepColorTemperatureRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("step temp {request:?}");
        todo!()
    }
}

impl on_off::ClusterHandler for LightController {
    const CLUSTER: rs_matter::dm::Cluster<'static> = Self::ON_OFF_CLUSTER;

    fn dataver(&self) -> u32 {
        self.dataver.get()
    }

    fn dataver_changed(&self) {
        self.notify_dataver_changed();
    }

    fn on_off(
        &self,
        _ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<bool, rs_matter::error::Error> {
        Ok(self.get_on_off())
    }

    fn handle_on(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
    ) -> Result<(), rs_matter::error::Error> {
        info!("on");
        self.set_on_off(true);
        self.notify_dataver_changed();
        Ok(())
    }

    fn handle_off(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
    ) -> Result<(), rs_matter::error::Error> {
        info!("off");
        self.set_on_off(false);
        self.notify_dataver_changed();
        Ok(())
    }

    fn handle_toggle(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
    ) -> Result<(), rs_matter::error::Error> {
        info!("toggle");
        let current = self.get_on_off();
        self.set_on_off(!current);
        self.notify_dataver_changed();
        Ok(())
    }

    fn handle_off_with_effect(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        _request: on_off::OffWithEffectRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("off with effect");
        // For simplicity, treat same as regular off
        self.handle_off(_ctx)
    }

    fn handle_on_with_recall_global_scene(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
    ) -> Result<(), rs_matter::error::Error> {
        info!("on with recall global scene");
        // For simplicity, treat same as regular on
        self.handle_on(_ctx)
    }

    fn handle_on_with_timed_off(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        _request: on_off::OnWithTimedOffRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("on with timed off");
        // For simplicity, treat same as regular on
        self.handle_on(_ctx)
    }
}

impl crate::level_control::ClusterHandler for LightController {
    const CLUSTER: rs_matter::dm::Cluster<'static> = Self::LEVEL_CONTROL_CLUSTER;

    fn dataver(&self) -> u32 {
        self.dataver.get()
    }

    fn dataver_changed(&self) {
        self.notify_dataver_changed();
    }

    fn current_level(
        &self,
        _ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<Maybe<u8, AsNullable>, rs_matter::error::Error> {
        Ok(Maybe::some(*self.current_level.borrow()))
    }

    fn handle_move_to_level(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        request: crate::level_control::MoveToLevelRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        let level = request.level()?;
        info!("move to level: {}", level);

        // TODO: transition_time support for LevelControl
        // The transition_time field returns Maybe<u16, AsNullable> which doesn't have easy unwrap
        // For now, always do instant change. Color commands support transitions.
        *self.current_level.borrow_mut() = level;
        self.update_led();
        self.notify_dataver_changed();

        Ok(())
    }

    fn handle_move_to_level_with_on_off(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        request: crate::level_control::MoveToLevelWithOnOffRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        let level = request.level()?;
        info!("move to level with on/off: {}", level);

        // Turn on if level > 0, off if level == 0
        *self.on_off.borrow_mut() = level > 0;
        *self.current_level.borrow_mut() = level;
        self.update_led();
        self.notify_dataver_changed();

        Ok(())
    }

    // Stub out remaining required methods
    fn options(
        &self,
        _ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<crate::level_control::OptionsBitmap, rs_matter::error::Error> {
        Ok(crate::level_control::OptionsBitmap::empty())
    }

    fn on_level(
        &self,
        _ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<Maybe<u8, AsNullable>, rs_matter::error::Error> {
        Ok(Maybe::none())
    }

    fn set_options(
        &self,
        _ctx: impl rs_matter::dm::WriteContext,
        _value: crate::level_control::OptionsBitmap,
    ) -> Result<(), rs_matter::error::Error> {
        Ok(())
    }

    fn set_on_level(
        &self,
        _ctx: impl rs_matter::dm::WriteContext,
        _value: Maybe<u8, AsNullable>,
    ) -> Result<(), rs_matter::error::Error> {
        Ok(())
    }

    fn handle_move(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        _request: crate::level_control::MoveRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("move (continuous) - not implemented");
        Ok(())
    }

    fn handle_step(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        _request: crate::level_control::StepRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("step - not implemented");
        Ok(())
    }

    fn handle_stop(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        _request: crate::level_control::StopRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("stop - not implemented");
        Ok(())
    }

    fn handle_move_with_on_off(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        _request: crate::level_control::MoveWithOnOffRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("move with on/off - not implemented");
        Ok(())
    }

    fn handle_step_with_on_off(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        _request: crate::level_control::StepWithOnOffRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("step with on/off - not implemented");
        Ok(())
    }

    fn handle_stop_with_on_off(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        _request: crate::level_control::StopWithOnOffRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("stop with on/off - not implemented");
        Ok(())
    }

    fn handle_move_to_closest_frequency(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        _request: crate::level_control::MoveToClosestFrequencyRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("move to closest frequency - not implemented");
        Ok(())
    }
}

impl crate::identify::ClusterHandler for LightController {
    const CLUSTER: rs_matter::dm::Cluster<'static> = crate::identify::FULL_CLUSTER;

    fn dataver(&self) -> u32 {
        self.dataver.get()
    }

    fn dataver_changed(&self) {
        self.notify_dataver_changed();
    }

    fn identify_time(
        &self,
        _ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<u16, rs_matter::error::Error> {
        // Convert deciseconds to seconds
        let ds = *self.identify_time_ds.borrow();
        Ok(ds / 10)
    }

    fn set_identify_time(
        &self,
        _ctx: impl rs_matter::dm::WriteContext,
        value: u16,
    ) -> Result<(), rs_matter::error::Error> {
        // Convert seconds to deciseconds
        *self.identify_time_ds.borrow_mut() = value * 10;
        *self.identify_phase.borrow_mut() = 0;
        Ok(())
    }

    fn identify_type(
        &self,
        _ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<crate::identify::IdentifyTypeEnum, rs_matter::error::Error> {
        // LightOutput (0x01) - we're a light device
        // Try different variants to find the right one
        Ok(crate::identify::IdentifyTypeEnum::LightOutput)
    }

    fn handle_identify(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        request: crate::identify::IdentifyRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        let seconds = request.identify_time()?;
        info!("identify for {} seconds", seconds);

        // Convert seconds to deciseconds
        *self.identify_time_ds.borrow_mut() = seconds * 10;
        // Reset blink phase
        *self.identify_phase.borrow_mut() = 0;

        Ok(())
    }

    fn handle_trigger_effect(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        request: crate::identify::TriggerEffectRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        let effect_id = request.effect_identifier()?;
        let effect_variant = request.effect_variant()?;
        info!(
            "trigger effect id={:?} variant={:?}",
            effect_id, effect_variant
        );
        // TODO: Implement fancy effects (breathe, blink, okay, channel change)
        // For now, just treat it as a 2-second identify
        *self.identify_time_ds.borrow_mut() = 20; // 2 seconds
        *self.identify_phase.borrow_mut() = 0;
        Ok(())
    }
}

/// Background task that updates transitions and identify every 100ms (1 decisecond)
/// Must be spawned from main.rs with a 'static reference to LightController
#[embassy_executor::task]
pub async fn transition_task(controller: &'static LightController) {
    loop {
        Timer::after(Duration::from_millis(100)).await;
        controller.update_transition();

        // Also update identify countdown and blink animation
        if controller.update_identify() {
            controller.update_led();
        }
    }
}
