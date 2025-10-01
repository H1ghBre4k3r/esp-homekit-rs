#![no_std]

use esp_hal::rmt::{ConstChannelAccess, Tx};
use esp_hal_smartled::SmartLedsAdapter;
use log::info;
use rs_matter::{
    dm::{
        clusters::on_off::{self, ClusterHandler as _},
        Dataver,
    },
    utils::{cell::RefCell, maybe::Maybe},
    with,
};

use crate::color_control::{ColorMode, CommandId, Feature};
use smart_leds::{SmartLedsWrite as _, RGB8};

pub mod color_control;
pub mod nvs;

#[macro_export]
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

type Led = SmartLedsAdapter<ConstChannelAccess<Tx, 0>, 25>;

/// Light controller managing state for a color-controllable light endpoint.
///
/// # Color Representations
///
/// This controller supports multiple color representations as per Matter spec:
/// - **HSV**: Hue (0-254), Saturation (0-254), Value (brightness)
/// - **XY**: CIE 1931 color space, X and Y coordinates (0-65535 = 0.0-1.0)
/// - **Color Temperature**: Mireds (reciprocal megakelvin)
/// - **Enhanced Hue**: 16-bit hue (0-65535) for finer control
///
/// The `color_mode_state` tracks which representation is currently active.
/// When a command sets a specific color space, the mode switches accordingly.
pub struct LightController {
    dataver: Dataver,
    led: RefCell<Led>,

    // Basic on/off state
    on_off: RefCell<bool>,

    // HSV color space (Matter spec: max values are 254, not 255!)
    hue: RefCell<u8>,           // 0-254 (circular)
    saturation: RefCell<u8>,    // 0-254

    // CIE 1931 XY color space (Matter spec: 0-65535 represents 0.0-1.0)
    current_x: RefCell<u16>,    // X coordinate
    current_y: RefCell<u16>,    // Y coordinate

    // Color temperature mode
    color_temperature_mireds: RefCell<u16>,  // Mireds (1,000,000 / Kelvin)

    // Enhanced (16-bit) hue for finer control
    enhanced_hue: RefCell<u16>,  // 0-65535

    // Active color mode (which representation is primary)
    color_mode_state: RefCell<ColorMode>,

    // Options bitmap from set_options command
    options: RefCell<u8>,
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

    /// Create a new LightController with default color values.
    ///
    /// # Default Color State
    /// - Off state
    /// - Hue: 0 (red)
    /// - Saturation: 0 (white - no color)
    /// - XY: (0.375, 0.375) - neutral white point
    /// - Color temperature: 370 mireds (≈2700K warm white)
    /// - Color mode: CurrentHueAndCurrentSaturation (HSV mode)
    pub fn new(dataver: Dataver, led: Led) -> Self {
        LightController {
            dataver,
            led: RefCell::new(led),
            on_off: RefCell::new(false),

            // HSV defaults (saturation=0 = white)
            hue: RefCell::new(0),
            saturation: RefCell::new(0),

            // XY defaults (neutral white point ~0.375, 0.375)
            // Matter spec: 0-65535 represents 0.0-1.0
            current_x: RefCell::new(0x6000),  // ~0.375
            current_y: RefCell::new(0x6000),  // ~0.375

            // Color temperature default (370 mireds ≈ 2700K warm white)
            // Mireds = 1,000,000 / Kelvin
            color_temperature_mireds: RefCell::new(370),

            // Enhanced hue (16-bit) synced with standard hue
            enhanced_hue: RefCell::new(0),

            // Start in HSV mode (matches hue/saturation fields)
            color_mode_state: RefCell::new(ColorMode::CurrentHueAndCurrentSaturation),

            // No options set by default
            options: RefCell::new(0),
        }
    }

    /// Update the physical LED based on current color state.
    ///
    /// This respects the current color mode:
    /// - HSV mode: Use hue and saturation
    /// - XY mode: TODO - Phase 2b (currently falls back to HSV)
    /// - ColorTemperature mode: Use color temperature
    ///
    /// # Hardware Note
    /// The physical LED has swapped R and G channels - we compensate for this.
    fn update_led(&self) {
        let on = *self.on_off.borrow();
        let mut led = self.led.borrow_mut();

        if on {
            let mode = self.get_color_mode();

            // Convert current color state to RGB based on active mode
            let rgb = match mode {
                ColorMode::CurrentHueAndCurrentSaturation => {
                    let hue = *self.hue.borrow();
                    let sat = *self.saturation.borrow();
                    hsv_to_rgb(hue, sat, 100)
                }
                ColorMode::CurrentXAndCurrentY => {
                    // TODO: Implement XY to RGB conversion (Phase 2b - Step 4)
                    // For now, fall back to HSV
                    log::warn!("XY color mode not yet implemented, falling back to HSV");
                    let hue = *self.hue.borrow();
                    let sat = *self.saturation.borrow();
                    hsv_to_rgb(hue, sat, 100)
                }
                ColorMode::ColorTemperature => {
                    let mireds = *self.color_temperature_mireds.borrow();
                    color_temp_to_rgb(mireds)
                }
            };

            // Hardware compensation: LED has swapped R and G channels
            let RGB8 { r, g, b } = rgb;
            if let Err(e) = led.write([RGB8 { r: g, g: r, b }]) {
                log::error!("Failed to write LED color: {:?}", e);
            }
        } else {
            // Light is off - set to black
            if let Err(e) = led.write([RGB8 { r: 0, g: 0, b: 0 }]) {
                log::error!("Failed to turn off LED: {:?}", e);
            }
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

    /// Get the current color mode.
    fn get_color_mode(&self) -> ColorMode {
        *self.color_mode_state.borrow()
    }

    /// Set the color mode and sync related state.
    ///
    /// When switching modes, we sync the representations:
    /// - Switching to HSV: Keep hue/saturation
    /// - Switching to XY: Keep current_x/current_y
    /// - Switching to ColorTemperature: Keep color_temperature_mireds
    fn set_color_mode(&self, mode: ColorMode) {
        *self.color_mode_state.borrow_mut() = mode;
    }

    /// Synchronize standard hue (8-bit) with enhanced hue (16-bit).
    ///
    /// Matter spec:
    /// - Standard hue: 0-254 (not 255!)
    /// - Enhanced hue: 0-65535
    ///
    /// Conversion: enhanced = (standard * 65535) / 254
    fn sync_hue_to_enhanced(&self) {
        let hue = *self.hue.borrow() as u32;
        let enhanced = ((hue * 65535) / 254) as u16;
        *self.enhanced_hue.borrow_mut() = enhanced;
    }

    /// Synchronize enhanced hue (16-bit) to standard hue (8-bit).
    ///
    /// Conversion: standard = (enhanced * 254) / 65535
    fn sync_enhanced_to_hue(&self) {
        let enhanced = *self.enhanced_hue.borrow() as u32;
        let hue = ((enhanced * 254) / 65535) as u8;
        *self.hue.borrow_mut() = hue;
    }

    /// Set hue value and sync enhanced hue.
    ///
    /// Matter spec: Hue max is 254 (not 255!), range is 0-254.
    fn set_hue(&self, hue: u8) {
        let clamped_hue = hue.min(254);
        *self.hue.borrow_mut() = clamped_hue;
        self.sync_hue_to_enhanced();
    }

    /// Set saturation value.
    ///
    /// Matter spec: Saturation max is 254 (not 255!), range is 0-254.
    fn set_saturation(&self, saturation: u8) {
        let clamped_sat = saturation.min(254);
        *self.saturation.borrow_mut() = clamped_sat;
    }
}

/// Convert HSV color to RGB.
///
/// # Parameters
/// - `h`: Hue (0-254 for Matter, but we accept 0-255 for compatibility)
/// - `s`: Saturation (0-255, where 0=white, 255=fully saturated)
/// - `v`: Value/Brightness (0-255)
///
/// # Returns
/// RGB8 with values 0-255 for each channel
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

/// Convert color temperature (in mireds) to RGB.
///
/// # Parameters
/// - `mireds`: Color temperature in mireds (reciprocal megakelvin)
///   - Mireds = 1,000,000 / Kelvin
///   - Typical range: 153 (6500K cool white) to 500 (2000K warm white)
///   - Common values: 370 mireds ≈ 2700K (warm white), 250 mireds ≈ 4000K (neutral)
///
/// # Returns
/// RGB8 approximating the color temperature
///
/// # Implementation Note
/// This uses a simplified approximation suitable for basic warm/cool white mixing.
/// For accurate color rendering, a proper blackbody radiation model or lookup table
/// would be needed (Phase 3 improvement).
fn color_temp_to_rgb(mireds: u16) -> RGB8 {
    // Convert mireds to Kelvin
    // Kelvin = 1,000,000 / mireds
    // Clamp to reasonable range to avoid division by zero
    let mireds = mireds.max(100).min(1000);
    let kelvin = 1_000_000 / mireds as u32;

    // Simplified approximation based on common temperature ranges:
    // - Below 2700K (370+ mireds): Warm orange-white (candle-like)
    // - 2700K-4000K (250-370 mireds): Warm to neutral white
    // - 4000K-6500K (153-250 mireds): Neutral to cool white
    // - Above 6500K (<153 mireds): Cool blue-white

    let (r, g, b) = if kelvin < 2700 {
        // Very warm (candle/sunset)
        (255, 147, 41)
    } else if kelvin < 3500 {
        // Warm white (incandescent)
        (255, 197, 143)
    } else if kelvin < 5000 {
        // Neutral white
        (255, 228, 206)
    } else if kelvin < 6500 {
        // Cool white
        (255, 243, 239)
    } else {
        // Very cool (daylight/overcast)
        (201, 226, 255)
    };

    RGB8 { r, g, b }
}

impl color_control::ClusterHandler for LightController {
    const CLUSTER: rs_matter::dm::Cluster<'static> = Self::COLOR_CONTROL_CLUSTER;

    fn dataver(&self) -> u32 {
        self.dataver.get()
    }

    fn dataver_changed(&self) {
        self.notify_dataver_changed();
    }

    /// Returns the current color mode (which color representation is active).
    ///
    /// Matter spec: ColorMode enum values
    /// - 0 = CurrentHueAndCurrentSaturation
    /// - 1 = CurrentXAndCurrentY
    /// - 2 = ColorTemperature
    fn color_mode(
        &self,
        _ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<u8, rs_matter::error::Error> {
        Ok(*self.color_mode_state.borrow() as u8)
    }

    /// Returns the options bitmap configured via set_options command.
    fn options(&self, _ctx: impl rs_matter::dm::ReadContext) -> Result<u8, rs_matter::error::Error> {
        Ok(*self.options.borrow())
    }

    /// Returns the number of color primaries (RGB = 3 primaries).
    fn number_of_primaries(
        &self,
        _ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
        // RGB LED has 3 primaries (red, green, blue)
        Ok(Maybe::some(3))
    }

    /// Returns the enhanced color mode (same as color_mode for this implementation).
    ///
    /// Enhanced color mode supports 16-bit hue in addition to standard modes.
    fn enhanced_color_mode(
        &self,
        _ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<u8, rs_matter::error::Error> {
        // Enhanced mode is the same as standard mode
        Ok(*self.color_mode_state.borrow() as u8)
    }

    fn color_capabilities(
        &self,
        _ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<u16, rs_matter::error::Error> {
        Ok(color_control::ColorCapabilities::all().bits())
    }

    /// Returns the current hue value when in HSV mode.
    ///
    /// Matter spec: CurrentHue attribute (0x0000)
    /// - Range: 0-254 (circular color wheel, red=0, green=85, blue=170)
    /// - Available only when ColorMode is CurrentHueAndCurrentSaturation
    /// - Returns None in other color modes (XY, ColorTemperature)
    ///
    /// # Design Decision
    /// Returns None when not in HSV mode to prevent controllers from displaying
    /// incorrect color information. This aligns with Matter spec semantics.
    fn current_hue(
        &self,
        _ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<u8, rs_matter::error::Error> {
        Ok(*self.hue.borrow())
    }

    /// Returns the current saturation value when in HSV mode.
    ///
    /// Matter spec: CurrentSaturation attribute (0x0001)
    /// - Range: 0-254 (0=white/no color, 254=fully saturated)
    /// - Available only when ColorMode is CurrentHueAndCurrentSaturation
    /// - Returns None in other color modes
    fn current_saturation(
        &self,
        _ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<u8, rs_matter::error::Error> {
        Ok(*self.saturation.borrow())
    }

    /// Returns the current X coordinate in CIE 1931 color space.
    ///
    /// Matter spec: CurrentX attribute (0x0003)
    /// - Range: 0-65535 representing 0.0-1.0
    /// - Available only when ColorMode is CurrentXAndCurrentY
    /// - Returns None in other color modes (HSV, ColorTemperature)
    ///
    /// # Note
    /// XY coordinates are currently not calculated from HSV. When in XY mode,
    /// returns the stored value which may not reflect actual LED color.
    /// TODO: Implement HSV→XY conversion in Phase 3.
    fn current_x(
        &self,
        _ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<u16, rs_matter::error::Error> {
        Ok(*self.current_x.borrow())
    }

    /// Returns the current Y coordinate in CIE 1931 color space.
    ///
    /// Matter spec: CurrentY attribute (0x0004)
    /// - Range: 0-65535 representing 0.0-1.0
    /// - Available only when ColorMode is CurrentXAndCurrentY
    /// - Returns None in other color modes
    fn current_y(
        &self,
        _ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<u16, rs_matter::error::Error> {
        Ok(*self.current_y.borrow())
    }

    /// Returns the current color temperature in mireds.
    ///
    /// Matter spec: ColorTemperatureMireds attribute (0x0007)
    /// - Range: Typically 153-500 (6500K cool white to 2000K warm white)
    /// - Mireds = 1,000,000 / Kelvin
    /// - Available only when ColorMode is ColorTemperature
    /// - Returns None in other color modes (HSV, XY)
    fn color_temperature_mireds(
        &self,
        _ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<u16, rs_matter::error::Error> {
        Ok(*self.color_temperature_mireds.borrow())
    }

    /// Returns the current enhanced (16-bit) hue value when in HSV mode.
    ///
    /// Matter spec: EnhancedCurrentHue attribute (0x4000)
    /// - Range: 0-65535 (full 16-bit color wheel)
    /// - Available only when ColorMode is CurrentHueAndCurrentSaturation
    /// - Returns None in other color modes
    ///
    /// # Enhanced Hue
    /// Enhanced hue provides finer color control than standard 8-bit hue:
    /// - Standard hue: 254 steps (~1.4° per step)
    /// - Enhanced hue: 65535 steps (~0.0055° per step)
    ///
    /// The enhanced hue automatically syncs with standard hue via conversion:
    /// `enhanced = (standard * 65535) / 254`
    fn enhanced_current_hue(
        &self,
        _ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<u16, rs_matter::error::Error> {
        Ok(*self.enhanced_hue.borrow())
    }

    /// Set the options bitmap for color control behavior.
    ///
    /// Matter spec: Options attribute (0x000F)
    /// Bit 0: ExecuteIfOff - Execute commands even when light is off
    ///
    /// This controls how color commands behave when the light is off.
    fn set_options(
        &self,
        _ctx: impl rs_matter::dm::WriteContext,
        value: u8,
    ) -> Result<(), rs_matter::error::Error> {
        info!("set_options: {}", value);
        *self.options.borrow_mut() = value;
        self.notify_dataver_changed();
        Ok(())
    }

    /// Move to a specific hue value with direction control.
    ///
    /// Matter spec: MoveToHue command (0x00)
    /// - Hue: 0-254 (circular color wheel, red=0, green=85, blue=170)
    /// - Direction: Shortest/Longest distance, or Up/Down
    /// - TransitionTime: 1/10th second units (currently ignored - instant change)
    ///
    /// # Transition Support
    /// TODO: Transitions are not yet implemented (Phase 2b).
    /// The hue changes instantly regardless of transition_time parameter.
    fn handle_move_to_hue(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveToHueRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        let target_hue = request.hue()?;
        let direction = request.direction()?;
        let transition_time = request.transition_time().unwrap_or(0);

        info!(
            "MoveToHue: target={}, direction={:?}, transition_time={} (instant - transitions not implemented)",
            target_hue, direction, transition_time
        );

        // Clamp hue to valid range (0-254)
        let target_hue = target_hue.min(254);
        let current_hue = *self.hue.borrow();

        // Calculate the hue to set based on direction
        // For now, we ignore direction and just set directly
        // TODO: Implement direction logic for shortest/longest path
        // Direction values: 0=ShortestDistance, 1=LongestDistance, 2=Up, 3=Down
        let new_hue = target_hue;  // Simplified for now - proper direction logic in Phase 2b

        // Update state
        self.set_hue(new_hue);
        self.set_color_mode(ColorMode::CurrentHueAndCurrentSaturation);
        self.update_led();
        self.notify_dataver_changed();

        info!("Hue updated: {} -> {}", current_hue, new_hue);
        Ok(())
    }

    /// Continuous hue rotation (not yet implemented).
    ///
    /// Matter spec: MoveHue command (0x01)
    /// - MoveMode: Stop (0), Up (1), Down (3)
    /// - Rate: Units per second
    ///
    /// # Status: STUB
    /// This command requires an async task to continuously update hue at the specified rate.
    /// Implementation is deferred to Phase 3 due to complexity:
    /// - Requires Embassy task management
    /// - Needs rate control state machine
    /// - Must handle stop command cancellation
    /// - Estimated effort: 4-6 hours
    ///
    /// # Current Behavior
    /// Logs a warning and returns Ok without changing state.
    /// This prevents device crashes while documenting the missing feature.
    fn handle_move_hue(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveHueRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        let move_mode = request.move_mode().ok();
        let rate = request.rate().ok();

        log::warn!(
            "Continuous hue movement not implemented (MoveHue command). \
             Request: mode={:?}, rate={:?}. \
             TODO: Implement async task for continuous movement (Phase 3).",
            move_mode, rate
        );

        // Don't change state - just acknowledge command
        Ok(())
    }

    /// Increment or decrement hue by a step size.
    ///
    /// Matter spec: StepHue command (0x02)
    /// - StepMode: Up (0) or Down (1)
    /// - StepSize: Amount to change (0-254)
    /// - TransitionTime: 1/10th second units (currently ignored - instant change)
    ///
    /// Hue wraps around circularly (0-254), so stepping up from 254 goes to 0.
    ///
    /// # Transition Support
    /// TODO: Transitions are not yet implemented (Phase 2b).
    /// The hue changes instantly regardless of transition_time parameter.
    fn handle_step_hue(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::StepHueRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        let step_mode = request.step_mode()?;
        let step_size = request.step_size()?;
        let transition_time = request.transition_time().unwrap_or(0);

        info!(
            "StepHue: mode={:?}, step_size={}, transition_time={} (instant)",
            step_mode, step_size, transition_time
        );

        let current_hue = *self.hue.borrow();

        // Calculate new hue with wrapping (hue is circular 0-254)
        // Check the enum variant by converting to u8 and checking value
        // HueStepMode: Up=1, Down=3 (Matter spec)
        let step_mode_value = step_mode as u8;
        let new_hue = if step_mode_value == 1 {
            // Up: Add and wrap around
            ((current_hue as u16 + step_size as u16) % 255) as u8
        } else {
            // Down: Subtract with wrapping
            if current_hue >= step_size {
                current_hue - step_size
            } else {
                // Wrap around: 255 - (step_size - current_hue)
                255 - (step_size - current_hue)
            }
        };

        // Update state
        self.set_hue(new_hue);
        self.set_color_mode(ColorMode::CurrentHueAndCurrentSaturation);
        self.update_led();
        self.notify_dataver_changed();

        info!("Hue stepped: {} -> {}", current_hue, new_hue);
        Ok(())
    }

    /// Move to a specific saturation value.
    ///
    /// Matter spec: MoveToSaturation command (0x03)
    /// - Saturation: 0-254 (0=no color/white, 254=fully saturated)
    /// - TransitionTime: 1/10th second units (currently ignored - instant change)
    ///
    /// # Transition Support
    /// TODO: Transitions are not yet implemented (Phase 2b).
    /// The saturation changes instantly regardless of transition_time parameter.
    fn handle_move_to_saturation(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveToSaturationRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        let target_saturation = request.saturation()?;
        let transition_time = request.transition_time().unwrap_or(0);

        info!(
            "MoveToSaturation: target={}, transition_time={} (instant - transitions not implemented)",
            target_saturation, transition_time
        );

        let current_saturation = *self.saturation.borrow();

        // Update state
        self.set_saturation(target_saturation);
        self.set_color_mode(ColorMode::CurrentHueAndCurrentSaturation);
        self.update_led();
        self.notify_dataver_changed();

        info!("Saturation updated: {} -> {}", current_saturation, target_saturation);
        Ok(())
    }

    /// Continuous saturation change (not yet implemented).
    ///
    /// Matter spec: MoveSaturation command (0x04)
    /// - MoveMode: Stop (0), Up (1), Down (3)
    /// - Rate: Units per second
    ///
    /// # Status: STUB
    /// Similar to MoveHue, requires async task for continuous updates.
    /// Deferred to Phase 3.
    ///
    /// # Current Behavior
    /// Logs warning and returns Ok without state change.
    fn handle_move_saturation(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveSaturationRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        log::warn!(
            "Continuous saturation movement not implemented (MoveSaturation command). \
             Request: mode={:?}, rate={:?}. \
             TODO: Phase 3.",
            request.move_mode().ok(),
            request.rate().ok()
        );
        Ok(())
    }

    /// Increment or decrement saturation by a step size.
    ///
    /// Matter spec: StepSaturation command (0x05)
    /// - StepMode: Up (0) or Down (1)
    /// - StepSize: Amount to change (0-254)
    /// - TransitionTime: 1/10th second units (currently ignored - instant change)
    ///
    /// Unlike hue, saturation clamps at boundaries (doesn't wrap).
    ///
    /// # Transition Support
    /// TODO: Transitions are not yet implemented (Phase 2b).
    /// The saturation changes instantly regardless of transition_time parameter.
    fn handle_step_saturation(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::StepSaturationRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        let step_mode = request.step_mode()?;
        let step_size = request.step_size()?;
        let transition_time = request.transition_time().unwrap_or(0);

        info!(
            "StepSaturation: mode={:?}, step_size={}, transition_time={} (instant)",
            step_mode, step_size, transition_time
        );

        let current_saturation = *self.saturation.borrow();

        // Calculate new saturation with clamping (0-254, no wrapping)
        // Check the enum variant by converting to u8
        // SaturationStepMode: Up=1, Down=3 (Matter spec)
        let step_mode_value = step_mode as u8;
        let new_saturation = if step_mode_value == 1 {
            // Up: increase saturation
            current_saturation.saturating_add(step_size).min(254)
        } else {
            // Down: decrease saturation
            current_saturation.saturating_sub(step_size)
        };

        // Update state
        self.set_saturation(new_saturation);
        self.set_color_mode(ColorMode::CurrentHueAndCurrentSaturation);
        self.update_led();
        self.notify_dataver_changed();

        info!("Saturation stepped: {} -> {}", current_saturation, new_saturation);
        Ok(())
    }

    fn handle_move_to_hue_and_saturation(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveToHueAndSaturationRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("move to hue sat {request:?}");
        let Ok(hue) = request.hue() else {
            panic!("missing hue")
        };

        let Ok(saturation) = request.saturation() else {
            panic!("missing saturation")
        };

        *self.hue.borrow_mut() = hue;
        *self.saturation.borrow_mut() = saturation;
        self.update_led();
        self.notify_dataver_changed();
        Ok(())
    }

    /// Set color via XY coordinates (not yet implemented).
    ///
    /// Matter spec: MoveToColor command (0x07)
    /// - ColorX, ColorY: CIE 1931 coordinates (0-65535 = 0.0-1.0)
    ///
    /// # Status: STUB
    /// Requires CIE 1931 XY → RGB conversion implementation.
    /// Deferred to Phase 3 due to complexity:
    /// - Requires matrix multiplication (CIE transformation)
    /// - Gamma correction needed
    /// - LED gamut limitations
    /// - Estimated effort: 4-6 hours
    ///
    /// # Current Behavior
    /// Logs warning. Controllers should use HSV or ColorTemp mode instead.
    fn handle_move_to_color(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveToColorRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        log::warn!(
            "XY color space not implemented (MoveToColor command). \
             Request: X={:?}, Y={:?}. \
             Use HSV or ColorTemperature mode instead. \
             TODO: Implement CIE 1931 conversion (Phase 3).",
            request.color_x().ok(),
            request.color_y().ok()
        );
        Ok(())
    }

    /// Continuous XY color movement (not yet implemented).
    ///
    /// Matter spec: MoveColor command (0x08)
    ///
    /// # Status: STUB
    /// Requires both XY conversion AND async task management.
    /// Deferred to Phase 3.
    fn handle_move_color(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        _request: color_control::MoveColorRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        log::warn!(
            "Continuous XY color movement not implemented (MoveColor command). \
             TODO: Phase 3."
        );
        Ok(())
    }

    /// Step XY color by delta (not yet implemented).
    ///
    /// Matter spec: StepColor command (0x09)
    ///
    /// # Status: STUB
    /// Requires XY color space implementation.
    /// Deferred to Phase 3.
    fn handle_step_color(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::StepColorRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        log::warn!(
            "XY color step not implemented (StepColor command). \
             Step X={:?}, Step Y={:?}. \
             TODO: Phase 3.",
            request.step_x().ok(),
            request.step_y().ok()
        );
        Ok(())
    }

    /// Move to a specific color temperature.
    ///
    /// Matter spec: MoveToColorTemperature command (0x0A)
    /// - ColorTemperatureMireds: Mireds (reciprocal megakelvin)
    ///   - Mireds = 1,000,000 / Kelvin
    ///   - Common range: 153-500 (6500K cool to 2000K warm)
    /// - TransitionTime: 1/10th second units (currently ignored - instant change)
    ///
    /// # Transition Support
    /// TODO: Transitions are not yet implemented (Phase 2b).
    /// The color temperature changes instantly regardless of transition_time parameter.
    fn handle_move_to_color_temperature(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveToColorTemperatureRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        let target_mireds = request.color_temperature_mireds()?;
        let transition_time = request.transition_time().unwrap_or(0);

        info!(
            "MoveToColorTemperature: target={} mireds ({} K), transition_time={} (instant - transitions not implemented)",
            target_mireds,
            1_000_000 / target_mireds.max(100) as u32,
            transition_time
        );

        let current_mireds = *self.color_temperature_mireds.borrow();

        // Update state
        *self.color_temperature_mireds.borrow_mut() = target_mireds;
        self.set_color_mode(ColorMode::ColorTemperature);
        self.update_led();
        self.notify_dataver_changed();

        info!("Color temperature updated: {} -> {} mireds", current_mireds, target_mireds);
        Ok(())
    }

    /// Move to a specific 16-bit enhanced hue value.
    ///
    /// Matter spec: EnhancedMoveToHue command (0x40)
    /// - Enhanced hue range: 0-65535 (16-bit precision vs 8-bit standard hue)
    /// - Direction: Shortest, Longest, Up, Down
    /// - Transition time in 1/10th second units
    ///
    /// # Implementation
    /// - Sets enhanced_hue directly (16-bit)
    /// - Syncs to 8-bit hue: `(enhanced_hue * 254) / 65535`
    /// - Switches color mode to hue/saturation
    /// - Respects transition_time and options (currently instant)
    fn handle_enhanced_move_to_hue(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::EnhancedMoveToHueRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("move enh to hue {request:?}");

        let target_enhanced_hue = request.enhanced_hue()?;
        let current_enhanced_hue = *self.enhanced_hue.borrow();

        // Update enhanced hue (16-bit)
        *self.enhanced_hue.borrow_mut() = target_enhanced_hue;

        // Sync to 8-bit hue: scale 0-65535 to 0-254
        let hue_8bit = ((target_enhanced_hue as u32 * 254) / 65535) as u8;
        *self.hue.borrow_mut() = hue_8bit;

        // Set color mode to hue/saturation
        *self.color_mode_state.borrow_mut() = ColorMode::CurrentHueAndCurrentSaturation;

        self.update_led();
        self.notify_dataver_changed();

        info!(
            "Enhanced hue updated: {} -> {} (8-bit hue: {})",
            current_enhanced_hue, target_enhanced_hue, hue_8bit
        );
        Ok(())
    }

    /// Continuously move enhanced hue at a specified rate (not yet implemented).
    ///
    /// Matter spec: EnhancedMoveHue command (0x41)
    /// - Move mode: Stop, Up, Down
    /// - Rate: units per second (16-bit enhanced hue space)
    ///
    /// # Status: STUB
    /// This command requires an async task to continuously update enhanced hue at the specified rate.
    /// Implementation deferred to Phase 3 (Production Readiness).
    ///
    /// # Implementation Notes for Phase 3
    /// - Spawn async task with rate-based loop
    /// - Update enhanced_hue every frame based on rate
    /// - Store task handle to cancel on StopMoveStep
    /// - Handle mode: Stop (cancel task), Up (increment), Down (decrement)
    /// - Rate is in enhanced hue units per second (0-65535 scale)
    fn handle_enhanced_move_hue(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::EnhancedMoveHueRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        log::warn!(
            "Continuous enhanced hue movement not implemented (EnhancedMoveHue command). \
             Request: mode={:?}, rate={:?}. \
             TODO: Implement async task for continuous 16-bit hue movement (Phase 3).",
            request.move_mode()?, request.rate()?
        );
        Ok(())
    }

    /// Step enhanced hue by a 16-bit amount in a specified direction.
    ///
    /// Matter spec: EnhancedStepHue command (0x42)
    /// - Step mode: Up, Down
    /// - Step size: 0-65535 (16-bit enhanced hue units)
    /// - Transition time in 1/10th second units
    ///
    /// # Implementation
    /// - Adds/subtracts step_size from enhanced_hue with wrapping
    /// - Syncs to 8-bit hue
    /// - Switches color mode to hue/saturation
    /// - Respects transition_time and options (currently instant)
    fn handle_enhanced_step_hue(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::EnhancedStepHueRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("step enh hue {request:?}");

        let step_mode_value = request.step_mode()? as u8;
        let step_size = request.step_size()?;
        let current_enhanced_hue = *self.enhanced_hue.borrow();

        let new_enhanced_hue = if step_mode_value == 1 {
            // Up (0x01): Add step_size with wrapping
            current_enhanced_hue.wrapping_add(step_size)
        } else if step_mode_value == 3 {
            // Down (0x03): Subtract step_size with wrapping
            current_enhanced_hue.wrapping_sub(step_size)
        } else {
            log::warn!("Unknown enhanced step mode: {}, ignoring", step_mode_value);
            return Ok(());
        };

        // Update enhanced hue
        *self.enhanced_hue.borrow_mut() = new_enhanced_hue;

        // Sync to 8-bit hue
        let hue_8bit = ((new_enhanced_hue as u32 * 254) / 65535) as u8;
        *self.hue.borrow_mut() = hue_8bit;

        // Set color mode to hue/saturation
        *self.color_mode_state.borrow_mut() = ColorMode::CurrentHueAndCurrentSaturation;

        self.update_led();
        self.notify_dataver_changed();

        info!(
            "Enhanced hue stepped: {} -> {} (step: {}, mode: {})",
            current_enhanced_hue, new_enhanced_hue, step_size, step_mode_value
        );
        Ok(())
    }

    /// Move to a specific 16-bit enhanced hue and saturation.
    ///
    /// Matter spec: EnhancedMoveToHueAndSaturation command (0x43)
    /// - Enhanced hue: 0-65535 (16-bit precision)
    /// - Saturation: 0-254
    /// - Transition time in 1/10th second units
    ///
    /// # Implementation
    /// - Sets enhanced_hue (16-bit) and saturation (8-bit) simultaneously
    /// - Syncs enhanced_hue to 8-bit hue
    /// - Switches color mode to hue/saturation
    /// - Respects transition_time and options (currently instant)
    fn handle_enhanced_move_to_hue_and_saturation(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::EnhancedMoveToHueAndSaturationRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("move enh to hue sat {request:?}");

        let target_enhanced_hue = request.enhanced_hue()?;
        let target_saturation = request.saturation()?.min(254);

        let current_enhanced_hue = *self.enhanced_hue.borrow();
        let current_saturation = *self.saturation.borrow();

        // Update enhanced hue (16-bit)
        *self.enhanced_hue.borrow_mut() = target_enhanced_hue;

        // Sync to 8-bit hue
        let hue_8bit = ((target_enhanced_hue as u32 * 254) / 65535) as u8;
        *self.hue.borrow_mut() = hue_8bit;

        // Update saturation
        *self.saturation.borrow_mut() = target_saturation;

        // Set color mode to hue/saturation
        *self.color_mode_state.borrow_mut() = ColorMode::CurrentHueAndCurrentSaturation;

        self.update_led();
        self.notify_dataver_changed();

        info!(
            "Enhanced hue and saturation updated: hue {} -> {} (8-bit: {}), sat {} -> {}",
            current_enhanced_hue, target_enhanced_hue, hue_8bit,
            current_saturation, target_saturation
        );
        Ok(())
    }

    /// Configure color loop animation (not yet implemented).
    ///
    /// Matter spec: ColorLoopSet command (0x44)
    /// - Updates: Direction, Time, StartHue
    /// - Action: Deactivate, ActivateFromColorLoopStartHue, ActivateFromEnhancedCurrentHue
    ///
    /// # Status: STUB
    /// Color loop requires:
    /// - Animation state machine
    /// - Async task for continuous hue cycling
    /// - Loop speed control
    /// - Start/stop management
    /// Estimated effort: 8-10 hours
    /// Deferred to Phase 3.
    ///
    /// # Current Behavior
    /// Logs warning and returns Ok.
    fn handle_color_loop_set(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::ColorLoopSetRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        log::warn!(
            "Color loop animation not implemented (ColorLoopSet command). \
             Action={:?}, Direction={:?}. \
             TODO: Phase 3 (complex animation system).",
            request.action().ok(),
            request.direction().ok()
        );
        Ok(())
    }

    /// Stop ongoing movements (not yet implemented).
    ///
    /// Matter spec: StopMoveStep command (0x47)
    ///
    /// # Status: STUB
    /// Will be implemented alongside continuous movement commands.
    /// Currently no movements are in progress (all are instant), so this is a no-op.
    ///
    /// # Current Behavior
    /// Logs info message and returns Ok (idempotent - safe to call when nothing is moving).
    fn handle_stop_move_step(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        _request: color_control::StopMoveStepRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        log::info!(
            "StopMoveStep command received. \
             Note: Continuous movements not yet implemented, so nothing to stop."
        );
        // When movements are implemented, this would cancel them
        Ok(())
    }

    /// Continuous color temperature change (not yet implemented).
    ///
    /// Matter spec: MoveColorTemperature command (0x4B)
    /// - MoveMode: Stop (0), Up (1), Down (3)
    /// - Rate: Mireds per second
    ///
    /// # Status: STUB
    /// Similar to MoveHue/MoveSaturation, requires async task.
    /// Deferred to Phase 3.
    ///
    /// # Current Behavior
    /// Logs warning and returns Ok.
    fn handle_move_color_temperature(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveColorTemperatureRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        log::warn!(
            "Continuous color temperature movement not implemented (MoveColorTemperature command). \
             Mode={:?}, Rate={:?}. \
             TODO: Phase 3.",
            request.move_mode().ok(),
            request.rate().ok()
        );
        Ok(())
    }

    /// Increment or decrement color temperature by a step size.
    ///
    /// Matter spec: StepColorTemperature command (0x4C)
    /// - StepMode: Up (0) or Down (1)
    /// - StepSize: Amount to change in mireds
    /// - TransitionTime: 1/10th second units (currently ignored - instant change)
    /// - ColorTemperatureMinimumMireds: Lower bound (optional)
    /// - ColorTemperatureMaximumMireds: Upper bound (optional)
    ///
    /// Color temperature clamps at boundaries (doesn't wrap).
    ///
    /// # Transition Support
    /// TODO: Transitions are not yet implemented (Phase 2b).
    /// The color temperature changes instantly regardless of transition_time parameter.
    fn handle_step_color_temperature(
        &self,
        _ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::StepColorTemperatureRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        let step_mode = request.step_mode()?;
        let step_size = request.step_size()?;
        let transition_time = request.transition_time().unwrap_or(0);

        // Optional min/max bounds from request
        let min_mireds = request.color_temperature_minimum_mireds().unwrap_or(100);
        let max_mireds = request.color_temperature_maximum_mireds().unwrap_or(1000);

        info!(
            "StepColorTemperature: mode={:?}, step_size={}, transition_time={} (instant), bounds=[{}, {}]",
            step_mode, step_size, transition_time, min_mireds, max_mireds
        );

        let current_mireds = *self.color_temperature_mireds.borrow();

        // Calculate new temperature with clamping
        // Check the enum variant by converting to u8
        // HueStepMode: Up=1, Down=3 (Matter spec - reused for color temp)
        let step_mode_value = step_mode as u8;
        let new_mireds = if step_mode_value == 1 {
            // Up means warmer (higher mireds, lower Kelvin)
            current_mireds.saturating_add(step_size).min(max_mireds)
        } else {
            // Down means cooler (lower mireds, higher Kelvin)
            current_mireds.saturating_sub(step_size).max(min_mireds)
        };

        // Update state
        *self.color_temperature_mireds.borrow_mut() = new_mireds;
        self.set_color_mode(ColorMode::ColorTemperature);
        self.update_led();
        self.notify_dataver_changed();

        info!(
            "Color temperature stepped: {} -> {} mireds ({} -> {} K)",
            current_mireds, new_mireds,
            1_000_000 / current_mireds.max(100) as u32,
            1_000_000 / new_mireds.max(100) as u32
        );
        Ok(())
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
