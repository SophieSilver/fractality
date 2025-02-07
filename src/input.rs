//! Allows panning and zooming the fractal using the left mouse button and the mouse wheel.

use bevy::{
    app::{Plugin, Update},
    input::{
        mouse::{AccumulatedMouseScroll, MouseScrollUnit},
        ButtonInput,
    },
    log::error,
    math::{dvec2, DVec2},
    prelude::*,
    window::{PrimaryWindow, Window},
};

use crate::{compositing::ViewportCamera, ui::UiSystemSet};

use crate::fractal::Fractal;

const PIXELS_PER_LINE: f64 = 12.0;
const PIXELS_PER_HALF_SCALE: f64 = 50.0;
const EPSILON: f64 = 0.0001;

pub struct FractalInputPlugin;

impl Plugin for FractalInputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<FractalInputState>();
        app.add_systems(Update, fractal_input_system.after(UiSystemSet));
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct DragState {
    start_cursor_pos: DVec2,
    // used to avoid editing the fractal when holding the mouse in place
    previous_cursor_pos: DVec2,
    start_offest: DVec2,
}

#[derive(Debug, Clone, Copy, Resource, Default)]
pub struct FractalInputState {
    drag_state: Option<DragState>,
}

pub fn fractal_input_system(
    mut camera: Query<&Camera, With<ViewportCamera>>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut fractal: Query<&mut Fractal>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_wheel: Res<AccumulatedMouseScroll>,
    mut state: ResMut<FractalInputState>,
) {
    let Ok(window) = window.get_single() else {
        return;
    };
    // we are assuming only one fractal and one fractal camera ever exists in the scene
    let camera = camera.single_mut();
    let mut fractal = fractal.single_mut();
    let Some(camera_rect) = camera.logical_viewport_rect() else {
        error!("Failed to get the camera rect");
        return;
    };
    let viewport_size = DVec2::from(camera_rect.size());
    let pixels_per_unit = f64::max(viewport_size.x, viewport_size.y) / 2.0;

    if mouse_buttons.just_released(MouseButton::Left) {
        state.drag_state = None;
    }

    let Some(cursor_pos_f32) = window.cursor_position() else {
        return;
    };

    let cursor_pos = DVec2::from(cursor_pos_f32);

    if mouse_buttons.just_pressed(MouseButton::Left) && camera_rect.contains(cursor_pos_f32) {
        state.drag_state = Some(DragState {
            start_cursor_pos: cursor_pos,
            previous_cursor_pos: cursor_pos,
            start_offest: fractal.offset,
        });
    }

    if mouse_buttons.pressed(MouseButton::Left) && state.drag_state.is_some() {
        // I don't want a nested if and let chains are unstable
        let drag_state = state.drag_state.as_mut().unwrap();

        // delta for checking if the mouse is actually moving
        // these are swapped because we're offsetting opposite the direction of the cursor
        let delta = drag_state.previous_cursor_pos - cursor_pos;

        // preventing updates while the mouse isn't moving
        if delta.x.abs() > EPSILON || delta.y.abs() > EPSILON {
            // delta from the start of the drag so that errors don't accumulate at small scales
            let total_delta = drag_state.start_cursor_pos - cursor_pos;

            // multiplying by (1.0, -1.0) because the cursor position uses different y axis
            let fractal = &mut *fractal;
            let scaled_delta = (total_delta * dvec2(1.0, -1.0) / pixels_per_unit) * fractal.scale;

            fractal.offset = drag_state.start_offest + scaled_delta;
            drag_state.previous_cursor_pos = cursor_pos;
        }
    }

    let scroll_amount = mouse_wheel.delta.y as f64;
    if scroll_amount.abs() > 0.001 {
        let pixels_scrolled = match mouse_wheel.unit {
            MouseScrollUnit::Line => lines_to_pixels(scroll_amount),
            MouseScrollUnit::Pixel => scroll_amount,
        };

        // preserve cursor world position
        let cursor_centered_pos = cursor_pos - viewport_size / 2.0;
        let cursor_normalized_pos = cursor_centered_pos * dvec2(1.0, -1.0) / pixels_per_unit;

        let cursor_world_pos = cursor_normalized_pos * fractal.scale + fractal.offset;
        fractal.scale *= f64::exp2(-pixels_scrolled / PIXELS_PER_HALF_SCALE);
        // if we rearrange the cursor_world pos equation we get this
        // offset cursor_world_pos - cursor_normalized_pos * scale
        fractal.offset = cursor_world_pos - cursor_normalized_pos * fractal.scale;
    }
}

fn lines_to_pixels(lines: f64) -> f64 {
    lines * PIXELS_PER_LINE
}
