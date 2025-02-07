use bevy::{input::mouse::AccumulatedMouseScroll, math::uvec2, prelude::*, window::PrimaryWindow};
use bevy_egui::{
    egui::{self, Checkbox, Color32, Frame, Grid, Margin, RichText, ScrollArea, Ui},
    EguiContext, EguiContextSettings, EguiContexts, EguiPlugin,
};
use num_input::show_num_input;
use parameter::ComplexParameterInput;

use crate::fractal::{render::DoublePrecisionSupported, Fractal};
pub mod num_input;
pub mod parameter;

const UI_SCALE: f32 = 1.25;
const DRAG_SENSITIVITY: f64 = 0.0025;
const ITER_COUNT_SENSITIVITY_COEF: f32 = 0.0075;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, SystemSet, Hash)]
pub struct UiSystemSet;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .init_resource::<NonUiArea>()
            .add_systems(Startup, setup_ui)
            .add_systems(
                Update,
                (ui_system, consume_inputs_over_ui)
                    .chain()
                    .in_set(UiSystemSet),
            );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource, Deref, DerefMut, Default)]
pub struct NonUiArea(pub URect);

pub fn setup_ui(
    mut windows: Query<(&mut EguiContext, &mut EguiContextSettings, &Window), With<PrimaryWindow>>,
) {
    for (mut context, mut settings, window) in windows.iter_mut() {
        settings.scale_factor = UI_SCALE / window.scale_factor();
        context.get_mut().style_mut(|s| {
            s.interaction.selectable_labels = false;
        });
    }
}

pub fn ui_system(
    mut contexts: EguiContexts,
    mut fractal: Query<&mut Fractal>,
    mut non_ui_area: ResMut<NonUiArea>,
    f64_supported: Res<DoublePrecisionSupported>,
) {
    let Some(ctx) = contexts.try_ctx_mut() else {
        return;
    };
    let mut fractal = fractal.single_mut();

    egui::SidePanel::right("UiPanel")
        .resizable(false)
        .frame(Frame::side_top_panel(&ctx.style()).inner_margin(Margin::symmetric(10.0, 10.0)))
        .show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.label(RichText::new("Parameters").strong().size(18.0));
                ui.separator();

                Grid::new(ui.next_auto_id()).show(ui, |ui| {
                    ui.add_enabled_ui(f64_supported.0, |ui| {
                        ui.label("Double Precision:");
                        show_checkbox(ui, fractal.reborrow().map_unchanged(|f| &mut f.use_f64));
                    })
                    .response
                    .on_disabled_hover_text(
                        RichText::new("Unsupported on your device").color(Color32::LIGHT_RED),
                    );
                    ui.end_row();

                    let current_iter_count = fractal.iteration_count;
                    // make iteration count more sensitive the larger it is
                    let iter_count_sensitivity =
                        (current_iter_count + 1) as f32 * ITER_COUNT_SENSITIVITY_COEF;

                    ui.label("Iteration Count:");
                    show_num_input(
                        ui,
                        fractal.reborrow().map_unchanged(|f| &mut f.iteration_count),
                        iter_count_sensitivity,
                    );
                    ui.end_row();

                    let r = fractal.reborrow().map_unchanged(|f| &mut f.escape_radius);
                    let r_sensitivity = (r.abs() + 1.0) * DRAG_SENSITIVITY;
                    ui.label("Escape Radius:");
                    show_num_input(ui, r, r_sensitivity);
                    ui.end_row();
                });
                ui.add_space(5.0);

                let initial_z = fractal.reborrow().map_unchanged(|f| &mut f.initial_z);
                ui.label("Initial Z:");
                ui.indent(ui.next_auto_id(), |ui| {
                    ui.add(ComplexParameterInput(initial_z))
                });
                ui.add_space(5.0);

                let c = fractal.reborrow().map_unchanged(|f| &mut f.c);
                ui.label("C:");
                ui.indent(ui.next_auto_id(), |ui| ui.add(ComplexParameterInput(c)));
                ui.add_space(5.0);

                let p = fractal.reborrow().map_unchanged(|f| &mut f.p);
                ui.label("Exponent:");
                ui.indent(ui.next_auto_id(), |ui| ui.add(ComplexParameterInput(p)));
                ui.add_space(5.0);
            });
        });

    let new_non_ui_area = egui_rect_to_urect(ctx.available_rect() * UI_SCALE);

    if new_non_ui_area != non_ui_area.0 {
        debug!(area=?new_non_ui_area, "Setting non UI area");
        non_ui_area.0 = new_non_ui_area;
    }
}

pub fn consume_inputs_over_ui(
    mut contexts: EguiContexts,
    mut buttons: ResMut<ButtonInput<MouseButton>>,
    mut wheel: ResMut<AccumulatedMouseScroll>,
) {
    let Some(ctx) = contexts.try_ctx_mut() else {
        return;
    };

    if ctx.is_using_pointer() || ctx.is_pointer_over_area() {
        buttons.clear();
        wheel.delta = Vec2::ZERO;
    }
}

fn egui_rect_to_urect(egui_rect: egui::Rect) -> bevy::math::URect {
    let min = egui_rect.min;
    let max = egui_rect.max;

    let min = uvec2(min.x as u32, min.y as u32);
    let max = uvec2(max.x as u32, max.y as u32);
    URect { min, max }
}

fn show_checkbox(ui: &mut Ui, mut value: Mut<bool>) {
    let mut temp_value = *value;
    ui.add(Checkbox::without_text(&mut temp_value));
    if temp_value != *value {
        *value = temp_value;
    }
}
