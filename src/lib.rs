#![no_std]

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
pub mod level_control;
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

pub const LED_COUNT: usize = 1;

pub const LED_SIZE: usize = 1 + LED_COUNT * 32;

type Led = SmartLedsAdapter<ConstChannelAccess<Tx, 0>, LED_SIZE, RGBA<u8>>;

pub struct LightController {
    dataver: Dataver,
    led: RefCell<Led>,
    on_off: RefCell<bool>,
    hue: RefCell<u8>,
    saturation: RefCell<u8>,
    // LevelControl cluster state
    current_level: RefCell<u8>, // 0-254 (1=minimum light, 254=maximum)
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

    pub fn new(dataver: Dataver, led: Led) -> Self {
        LightController {
            dataver,
            led: RefCell::new(led),
            on_off: RefCell::new(false),
            hue: RefCell::new(0),
            saturation: RefCell::new(0),
            current_level: RefCell::new(254), // Start at maximum brightness
        }
    }

    fn update_led(&self) {
        let on = *self.on_off.borrow();
        let mut led = self.led.borrow_mut();

        if on {
            let hue = *self.hue.borrow();
            let sat = *self.saturation.borrow();
            let level = *self.current_level.borrow();

            // Convert level (0-254) to value (0-100) for HSV
            // Use saturating math to avoid overflow
            let value = ((level as u16 * 100) / 254) as u8;

            let RGB8 { r, g, b } = hsv_to_rgb(hue, sat, value);
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
        ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<u8, rs_matter::error::Error> {
        Ok(ColorMode::CurrentXAndCurrentY as _)
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
        ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<u8, rs_matter::error::Error> {
        Ok(ColorMode::CurrentXAndCurrentY as _)
    }

    fn color_capabilities(
        &self,
        ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<u16, rs_matter::error::Error> {
        Ok(color_control::ColorCapabilities::all().bits())
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

    fn handle_move_to_color(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveToColorRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("move to color {request:?}");
        todo!()
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
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveToColorTemperatureRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        info!("move to temp {request:?}");
        todo!()
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

        // For now, implement instant change (no transition)
        // TODO: Implement smooth transition based on request.transition_time
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
