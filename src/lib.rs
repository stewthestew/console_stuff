//! # Usage
//!
//! Namespaced (recommended):
//! ```rust
//! use console_stuff::{console, dialoguer, indicatif, clap};
//! console::style("hello").red();
//! ```
//!
//! Or import everything:
//! ```rust  
//! use console_stuff::prelude::*; // Bad idea, really bad idea, just use the namespaced way.
//! style("hello").red();
//! ```

pub use clap;
pub use console;
pub use dialoguer;
pub use indicatif;

#[allow(ambiguous_glob_reexports)]
pub mod prelude {
    pub use clap::*;
    pub use console::*;
    pub use dialoguer::*;
    pub use indicatif::*;
}
