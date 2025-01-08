use bevy::{log::debug, prelude::Mut};
use bevy_egui::egui::{self, ComboBox, DragValue, Grid, Ui, Widget};

use crate::fractal::parameters::{ComplexParameter, Parameter};

use super::DRAG_SENSITIVITY;

#[derive(Debug)]
pub struct ComplexParameterInput<'a>(pub Mut<'a, ComplexParameter>);

impl Widget for ComplexParameterInput<'_> {
    fn ui(mut self, ui: &mut Ui) -> egui::Response {
        // ui.allocate_space(egui::vec2(300.0, 0.0));
        Grid::new("Complex parameter input")
            .min_row_height(45.0)
            .show(ui, |ui| {
                ui.label("real:");
                show_parameter_input(ui, self.0.reborrow().map_unchanged(|v| &mut v.real));
                ui.end_row();
                ui.label("imaginary:");
                show_parameter_input(ui, self.0.reborrow().map_unchanged(|v| &mut v.imaginary));
            })
            .response
    }
}

fn show_parameter_input(ui: &mut Ui, mut value: Mut<Parameter>) {
    let mut modifiable_param = *value;
    let initial_param = modifiable_param;

    ui.vertical(|ui| {
        ComboBox::new(ui.next_auto_id(), "")
            .selected_text(modifiable_param.variant_str())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut modifiable_param,
                    Parameter::Value(0.0),
                    Parameter::Value(0.0).variant_str(),
                );
                ui.selectable_value(
                    &mut modifiable_param,
                    Parameter::PixelX,
                    Parameter::PixelX.variant_str(),
                );
                ui.selectable_value(
                    &mut modifiable_param,
                    Parameter::PixelY,
                    Parameter::PixelY.variant_str(),
                );
            });

        if let Parameter::Value(ref mut inner_value) = modifiable_param {
            ui.add(DragValue::new(inner_value).speed(DRAG_SENSITIVITY));
        }
    });

    if initial_param != modifiable_param {
        debug!("Complex parameter modified");
        *value = modifiable_param;
    }
}
