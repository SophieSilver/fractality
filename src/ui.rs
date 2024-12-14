use bevy::{
    input::mouse::AccumulatedMouseScroll,
    math::{dvec2, uvec2, vec2},
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_egui::{
    egui::{self, DragValue, Frame, Grid, Margin, RichText, ScrollArea},
    EguiContext, EguiContexts, EguiPlugin, EguiSettings,
};
use parameter::ComplexParameterInput;
use num_input::show_num_input;

use crate::fractal::Fractal;
pub mod parameter;
pub mod num_input;

const UI_SCALE: f32 = 1.25;
const DRAG_SENSITIVITY: f32 = 0.0025;
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
    mut windows: Query<(&mut EguiContext, &mut EguiSettings, &Window), With<PrimaryWindow>>,
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
) {
    let ctx = contexts.ctx_mut();
    let mut fractal = fractal.single_mut();

    egui::SidePanel::right("UiPanel")
        .resizable(false)
        .frame(Frame::side_top_panel(&ctx.style()).inner_margin(Margin::symmetric(10.0, 10.0)))
        .show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.label(RichText::new("Parameters").strong().size(18.0));
                ui.separator();

                ui.horizontal(|ui| {
                    let current_iter_count = fractal.iteration_count;
                    ui.label("Iteration Count:");
                    show_num_input(
                        ui,
                        fractal.reborrow().map_unchanged(|f| &mut f.iteration_count),
                        (current_iter_count + 1) as f32 * ITER_COUNT_SENSITIVITY_COEF,
                    );
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
    let ctx = contexts.ctx_mut();

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
