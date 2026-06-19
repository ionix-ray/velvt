//! Prelude — common re-exports for vaelvet-ui components.
//!
//! Import with `use crate::prelude::*;` to reduce boilerplate in every
//! component file. Intentionally small — only what every component needs.

pub use crate::config::Site;
pub use dioxus::prelude::*;

/// Convenience accessor for the browser window — returns `None` outside WASM.
#[inline]
pub fn window() -> Option<web_sys::Window> {
    web_sys::window()
}
