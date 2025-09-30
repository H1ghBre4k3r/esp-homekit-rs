#![no_std]

use crate::color_control::StepColorTemperatureRequest;

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

struct MyController;

impl color_control::ClusterHandler for MyController {
    const CLUSTER: rs_matter::dm::Cluster<'static> = color_control::FULL_CLUSTER;

    fn dataver(&self) -> u32 {
        todo!()
    }

    fn dataver_changed(&self) {
        todo!()
    }

    fn color_mode(
        &self,
        ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<u8, rs_matter::error::Error> {
        todo!()
    }

    fn options(&self, ctx: impl rs_matter::dm::ReadContext) -> Result<u8, rs_matter::error::Error> {
        todo!()
    }

    fn number_of_primaries(
        &self,
        ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<rs_matter::tlv::Nullable<u8>, rs_matter::error::Error> {
        todo!()
    }

    fn enhanced_color_mode(
        &self,
        ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<u8, rs_matter::error::Error> {
        todo!()
    }

    fn color_capabilities(
        &self,
        ctx: impl rs_matter::dm::ReadContext,
    ) -> Result<u16, rs_matter::error::Error> {
        todo!()
    }

    fn set_options(
        &self,
        ctx: impl rs_matter::dm::WriteContext,
        value: u8,
    ) -> Result<(), rs_matter::error::Error> {
        todo!()
    }

    fn handle_move_to_hue(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveToHueRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        todo!()
    }

    fn handle_move_hue(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveHueRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        todo!()
    }

    fn handle_step_hue(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::StepHueRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        todo!()
    }

    fn handle_move_to_saturation(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveToSaturationRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        todo!()
    }

    fn handle_move_saturation(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveSaturationRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        todo!()
    }

    fn handle_step_saturation(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::StepSaturationRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        todo!()
    }

    fn handle_move_to_hue_and_saturation(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveToHueAndSaturationRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        todo!()
    }

    fn handle_move_to_color(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveToColorRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        todo!()
    }

    fn handle_move_color(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveColorRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        todo!()
    }

    fn handle_step_color(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::StepColorRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        todo!()
    }

    fn handle_move_to_color_temperature(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveToColorTemperatureRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        todo!()
    }

    fn handle_enhanced_move_to_hue(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::EnhancedMoveToHueRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        todo!()
    }

    fn handle_enhanced_move_hue(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::EnhancedMoveHueRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        todo!()
    }

    fn handle_enhanced_step_hue(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::EnhancedStepHueRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        todo!()
    }

    fn handle_enhanced_move_to_hue_and_saturation(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::EnhancedMoveToHueAndSaturationRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        todo!()
    }

    fn handle_color_loop_set(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::ColorLoopSetRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        todo!()
    }

    fn handle_stop_move_step(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::StopMoveStepRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        todo!()
    }

    fn handle_move_color_temperature(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::MoveColorTemperatureRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        todo!()
    }

    fn handle_step_color_temperature(
        &self,
        ctx: impl rs_matter::dm::InvokeContext,
        request: color_control::StepColorTemperatureRequest<'_>,
    ) -> Result<(), rs_matter::error::Error> {
        todo!()
    }
}
