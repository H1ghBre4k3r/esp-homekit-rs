#![no_std]

use esp_hal::rmt::{ConstChannelAccess, Tx};
use esp_hal_smartled::SmartLedsAdapter;
use log::info;
use rs_matter::{
    dm::Dataver,
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

pub struct LightController {
    dataver: Dataver,
    led: RefCell<Led>,
}

impl LightController {
    pub fn new(dataver: Dataver, led: Led) -> Self {
        LightController {
            dataver,
            led: RefCell::new(led),
        }
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
    const CLUSTER: rs_matter::dm::Cluster<'static> = color_control::FULL_CLUSTER
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

    fn dataver(&self) -> u32 {
        self.dataver.get()
    }

    fn dataver_changed(&self) {
        self.dataver.changed();
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
        let RGB8 { r, g, b } = hsv_to_rgb(hue, saturation, 100);
        let mut led = self.led.borrow_mut();
        led.write([RGB8 { r: g, g: r, b }]).unwrap();
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
