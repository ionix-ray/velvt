use std::sync::{Mutex, Once};

static INIT: Once = Once::new();
static LOG: Mutex<String> = Mutex::new(String::new());

pub fn install_capture_once() {
    INIT.call_once(|| {});
}

pub fn clear_log() {
    if let Ok(mut guard) = LOG.lock() {
        guard.clear();
    }
}

pub fn take_log() -> String {
    if let Ok(mut guard) = LOG.lock() {
        std::mem::take(&mut *guard)
    } else {
        String::new()
    }
}

#[macro_export]
macro_rules! capture_log {
    ($($arg:tt)*) => {{
        if let Ok(mut guard) = $crate::test_support::LOG.lock() {
            use std::io::Write;
            let _ = write!(guard, $($arg)*);
            let _ = writeln!(guard);
        }
    }};
}
