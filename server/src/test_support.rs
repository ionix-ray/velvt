use std::sync::{Mutex, Once};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

static INIT: Once = Once::new();
static LOG: Mutex<String> = Mutex::new(String::new());

pub fn install_capture_once() {
    INIT.call_once(|| {
        let layer = tracing_subscriber::fmt::layer()
            .without_time()
            .with_target(false)
            .with_ansi(false)
            .with_writer(|| CaptureWriter);
        let _ = tracing_subscriber::registry().with(layer).try_init();
    });
}

struct CaptureWriter;
impl std::io::Write for CaptureWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Ok(mut guard) = LOG.lock() {
            let s = String::from_utf8_lossy(buf);
            guard.push_str(&s);
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
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
