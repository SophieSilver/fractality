use bevy::{input::mouse::AccumulatedMouseScroll, math::uvec2, prelude::*, window::PrimaryWindow};
use bevy_egui::{
    egui::{self, DragValue, Grid, Label, RichText, Visuals},
    EguiContext, EguiContexts, EguiPlugin, EguiSettings,
};

use crate::fractal::Fractal;

const UI_SCALE: f32 = 1.5;

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
    // contexts.ctx_mut().set_zoom_factor(zoom_factor);
    let ctx = contexts.ctx_mut();
    // ctx.style_mut(|s| {
    let mut fractal = fractal.single_mut();
    // });

    egui::SidePanel::right("UiPanel")
        .resizable(false)
        .show(ctx, |ui| {
            ui.allocate_space(egui::vec2(200.0, 0.0));
            ui.label("Initial Z:");
            ui.indent(123, |ui| {
                Grid::new("Complex input grid")
                    .min_col_width(50.0)
                    .show(ui, |ui| {
                        let mut new_z = fractal.initial_z;
                        ui.label("real:");
                        let mut changed =
                            ui.add(DragValue::new(&mut new_z.x).speed(0.005)).changed();
                        ui.end_row();
                        ui.label("imaginary:");
                        changed |= ui.add(DragValue::new(&mut new_z.y).speed(0.005)).changed();
                        ui.end_row();

                        if changed {
                            fractal.initial_z = new_z;
                        }
                    });
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