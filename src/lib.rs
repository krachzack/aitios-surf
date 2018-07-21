//! Manages collections of points that represent a surface.
//!
//! Provides functionality for building and searching surfaces, as well as
//! functionality to sample points on triangle meshes.

extern crate aitios_geom as geom;
extern crate aitios_sampling as sampling;
extern crate aitios_scene as scene;
extern crate nearest_kdtree;

mod builder;
mod surface;
mod surfel;

pub use builder::{SurfaceBuilder, SurfelSampling};
pub use surface::Surface;
pub use surfel::Surfel;
