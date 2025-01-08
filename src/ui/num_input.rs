use bevy::prelude::Mut;
use bevy_egui::egui::{emath::Numeric, DragValue, Ui};

pub fn show_num_input<Num: Numeric>(ui: &mut Ui, mut value: Mut<Num>, sensitivity: impl Into<f64>) {
    let mut modifiable_value = *value;

    let changed = ui
        .add(DragValue::new(&mut modifiable_value).speed(sensitivity))
        .changed();

    if changed {
        *value = modifiable_value;
    }
}
