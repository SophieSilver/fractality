use crate::utils::EitherIterator;
use backtrace::Backtrace;
use backtrace::BacktraceFrame;
use backtrace::SymbolName;
use smallstr::SmallString;
use std::fmt::Formatter;
use std::fmt::{self, Display, Write};
use std::iter;

pub struct BacktraceFormatter(pub Backtrace);

impl Display for BacktraceFormatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for (i, item) in short_backtrace_iterator(&self.0).enumerate() {
            if first {
                first = false;
            } else {
                write!(f, "\n")?;
            }

            write!(f, "{i:>4} {item}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BacktraceItem<'a> {
    Name(&'a [u8]),
    Addr(usize),
}

impl Display for BacktraceItem<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BacktraceItem::Name(name) => write!(f, "{:#}", SymbolName::new(name)),
            BacktraceItem::Addr(addr) => write!(f, "{addr:#x}"),
        }
    }
}

pub(crate) fn short_backtrace_iterator(bt: &Backtrace) -> impl Iterator<Item = BacktraceItem> {
    let mut items = bt.frames().into_iter().flat_map(frame_item_iterator);
    // backup in case we can't find the end
    let initial_items = items.clone();
    // skip all elements before a trace end
    loop {
        match items.next() {
            Some(item) => {
                if item_is_end(item) {
                    break;
                }
            }
            None => {
                // no end found, restore items
                items = initial_items;
                break;
            }
        }
    }

    items
        .take_while(|&item| !item_is_start(item))
        .filter(|&item| {
            !item_name_contains(item, "rust_begin_unwind")
                && !item_name_contains(item, "core::panicking::panic_fmt")
        })
}

pub(crate) fn frame_item_iterator(
    frame: &BacktraceFrame,
) -> impl Iterator<Item = BacktraceItem> + Clone {
    let symbols = frame.symbols();

    let items = symbols
        .into_iter()
        .flat_map(|sym| sym.name().map(|name| name.as_bytes()))
        .map(BacktraceItem::Name);

    // if no names were found
    if items.clone().next().is_none() {
        return EitherIterator::A(iter::once(BacktraceItem::Addr(
            frame.symbol_address() as usize
        )));
    }

    EitherIterator::B(items)
}

pub(crate) fn item_is_end(item: BacktraceItem) -> bool {
    item_name_contains(item, "__rust_end_short_backtrace")
}

pub(crate) fn item_is_start(item: BacktraceItem) -> bool {
    item_name_contains(item, "__rust_begin_short_backtrace")
}

pub(crate) fn item_name_contains(item: BacktraceItem, s: &str) -> bool {
    let BacktraceItem::Name(name) = item else {
        return false;
    };
    let name = SymbolName::new(name);
    let mut demangled = SmallString::<[_; 1024]>::new();
    let demangle_result = write!(&mut demangled, "{:#}", name);
    if let Err(_) = demangle_result {
        false;
    };

    demangled.contains(&s)
}
