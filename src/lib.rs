//! # TES Core
//!
//! Topographic Execution Substrate - A coordination medium based on spatial topology.
//!
//! TES is **not** a runtime, programming language, or agent framework.
//! It is a passive space that determines where, how much, and for how long
//! entities can exist.
//!
//! ## Key Concepts
//!
//! - **Space**: Atemporal, passive topographic field
//! - **Shape**: Bounded-memory carrier that inhabits the space
//! - **Trace**: Scalar density field (side-effect of presence)
//! - **Isotope**: Multi-channel (RGB) trace for spectroscopic analysis
//!
//! ## Design Principles
//!
//! - Trace is a side-effect, not state (Source Amnesia)
//! - Space is immutable; observation projects through time
//! - No rollback, no replay - decay only
//! - Identity-free coordination

mod space;
mod shape;
mod grid;
mod substrate;
mod isotope;

pub use space::Space;
pub use shape::Shape;
pub use grid::DensityGrid;
pub use substrate::Substrate;
pub use isotope::{IsotopeGrid, ServiceColor};
