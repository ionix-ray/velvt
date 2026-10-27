//! Shared test utilities — compiled only under `cfg(test)`.
//!
//! Process-wide tracing capture: a single global fmt subscriber writes each
//! thread's events into that thread's buffer, so unit tests can assert on
//! the log output they produced.

use std::io::Write;

thread_local! {
    // `tracing`'s per-callsite `Interest` cache is a *process-wide* cache,
    // not per-thread. Swapping subscribers via `set_default` per test races
    // other test threads doing the same, intermittently dropping events.
    // A single global subscriber (installed once) sidesteps the race; each
    // test reads only the events its own thread produced.
    static CAPTURE_BUFFER: std::cell::RefCell<Vec<u8>> = const { std::cell::RefCell::new(Vec::new()) };
}

#[derive(Clone, Copy, Default)]
struct ThreadLocalWriter;

impl Write for ThreadLocalWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        CAPTURE_BUFFER.with(|b| b.borrow_mut().write(buf))
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl tracing_subscriber::fmt::MakeWriter<'_> for ThreadLocalWriter {
    type Writer = Self;
    fn make_writer(&self) -> Self::Writer {
        *self
    }
}

/// Install the capture subscriber once per test process.
pub(crate) fn install_capture_once() {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        let subscriber = tracing_subscriber::fmt()
            .with_writer(ThreadLocalWriter)
            .with_ansi(false)
            .finish();
        // Best-effort: a prior global default (set by another test binary
        // sharing this process) is not something we need to fail over.
        let _ = tracing::subscriber::set_global_default(subscriber);
    });
}

/// Drop this thread's captured log output.
pub(crate) fn clear_log() {
    CAPTURE_BUFFER.with(|b| b.borrow_mut().clear());
}

/// Copy this thread's captured log output.
pub(crate) fn take_log() -> String {
    CAPTURE_BUFFER.with(|b| String::from_utf8_lossy(&b.borrow()).into_owned())
}
