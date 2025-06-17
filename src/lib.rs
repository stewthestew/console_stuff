//! # Usage
//!
//! Namespaced (recommended):
//! ```rust
//! use console_stuff::{console, dialoguer, indicatif};
//! console::style("hello").red();
//! ```
//!
//! Or import everything:
//! ```rust  
//! use console_stuff::prelude::*;
//! style("hello").red();
//! ```

pub use console;
pub use dialoguer;
pub use indicatif;

pub mod prelude {
    pub use console::*;
    pub use dialoguer::*;
    pub use indicatif::*;
}
