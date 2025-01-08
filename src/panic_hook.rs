use backtrace::Backtrace;
use bevy::prelude::*;
use rfd::{MessageButtons, MessageLevel};
use smallstr::SmallString;
use std::{
    any::Any,
    fmt::Write,
    panic::{self, PanicHookInfo},
    thread,
};

mod format_backtrace;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PanicHookPlugin;

impl Plugin for PanicHookPlugin {
    fn build(&self, _app: &mut App) {
        let previous_hook = panic::take_hook();

        std::panic::set_hook(Box::new(move |info| {
            panic_hook(info);
            previous_hook(info);
        }));
    }
}

fn panic_hook(info: &PanicHookInfo<'_>) {
    let current_thread = thread::current();
    let thread_name = current_thread.name().unwrap_or("<unnamed>");
    let backtrace = format_backtrace::BacktraceFormatter(Backtrace::new());
    let payload = payload_as_str(info.payload());

    let mut location_str = SmallString::<[u8; 1024]>::new();
    if let Some(l) = info.location() {
        let res = write!(location_str, " at {}:{}:{}", l.file(), l.line(), l.column());

        if res.is_err() {
            location_str.clear();
        }
    }

    let description = format!(
        "thread {thread_name} panicked{location_str}\n{payload}\nstack backtrace:\n{backtrace}",
    );

    rfd::MessageDialog::new()
        .set_level(MessageLevel::Error)
        .set_title("Critical Error")
        .set_buttons(MessageButtons::Ok)
        .set_description(description)
        .show();
}

fn payload_as_str(payload: &dyn Any) -> &str {
    payload
        .downcast_ref::<&str>()
        .map(|&s| s)
        .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
        .unwrap_or_else(|| "<NO PAYLOAD>")
}
