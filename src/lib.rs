//! SigmaOS-popd — Directory stack pop model.
//!
//! Models popping directories from a stack.
//! Deterministic and std-only.

pub mod popd;
pub mod options;

pub use popd::{Popd, DirectoryStack};
pub use options::{PopdOptions, RotationMode};
