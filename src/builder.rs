use super::*;

use geom::prelude::*;
use geom::Position;
use nearest_kdtree::KdTree;
use sampling::into_poisson_disk_set;
use surfel::Surfel;

pub struct SurfaceBuilder<S: Position> {
    samples: Vec<S>,
    sampling: SurfelSampling,
}

#[derive(Copy, Clone)]
/// Enumerates surfel sampling strategies that differ in statistical properties and
/// performance characteristics.
pub enum SurfelSampling {
    /// Examines each triangle and randomly samples an amount of points proporitional to the given
    /// point density per square unit in world space. Clumps together on smaller scales, but crazy fast.
    /// Use `MinimumDistance` for better quality.
    PerSqrUnit(f32),
    /// Uses [dart throwing](https://www.researchgate.net/publication/230312465_Dart_Throwing_on_Surfaces)
    /// on surfaces as proposed for David Cline et. al. to generate a poisson disk set with given minimum distance
    /// for points in the resulting set.
    /// The strategy is slower than `PerSqrUnit`, but surfels are more evenly spaced.
    MinimumDistance(f32),
}

impl<V: Position, D> SurfaceBuilder<Surfel<V, D>> {
    /// Sets the surface sampling strategy for converting meshes to surfels.
    /// Defaults to `SurfelSampling::MinimumDistance(0.1)` if never called.
    pub fn sampling(mut self, sampling: SurfelSampling) -> Self {
        self.sampling = sampling;
        self
    }

    pub fn sample_triangles<I, T>(mut self, triangles: I, prototype_surfel_data: &D) -> Self
    where
        T: Clone + InterpolateVertex<Vertex = V> + FromVertices<Vertex = V>,
        V: Clone,
        I: IntoIterator<Item = T>,
        D: Clone,
    {
        self.samples.extend(match self.sampling {
            SurfelSampling::MinimumDistance(min_dist) => into_poisson_disk_set(triangles, min_dist)
                .map(|v| Surfel::new(v, prototype_surfel_data.clone())),
            _ => unimplemented!("Only SurfelSampling::MinimumDistance implemented at the moment"),
        });

        self
    }
}

impl<S: Position> SurfaceBuilder<S> {
    pub fn new() -> Self {
        SurfaceBuilder {
            samples: Vec::new(),
            sampling: SurfelSampling::MinimumDistance(0.1),
        }
    }

    /// Adds the given surface samples. Such samples can either be manually created
    /// or be the result of taking a subset of another surface.
    ///
    /// The method is especially useful for debugging, since Vec3 trivially implements Position
    /// and you can use it to dump points to files for further examination.
    ///
    /// This examples illustrates this by generating a file containing circle vertices:
    /// ```
    /// # extern crate aitios_geom;
    /// # extern crate aitios_surf;
    /// use std::f32::consts::PI;
    /// use std::path::PathBuf;
    /// use std::fs::{File, remove_file};
    /// use aitios_geom::Vec3;
    /// use aitios_surf::{Surface, SurfaceBuilder};
    ///
    /// # fn main() {
    /// // 100 Vertices of a circle on the X/Y plane with radius 5 will serve
    /// // as the surfels to dump into a file
    /// let point_count = 100;
    /// let radius = 5.0;
    /// let circle_vertices = (0..point_count)
    ///     .map(|i| 2.0 * PI * ((i as f32) / (point_count as f32))) // to angle in range 0..2π
    ///     .map(|a| a.sin_cos())
    ///     .map(|(y, x)| Vec3::new(x * radius, y * radius, 0.0));
    ///
    /// // Create a new surface from the circle vertices
    /// let circle_surface : Surface<Vec3> = SurfaceBuilder::new()
    ///     .add_samples(circle_vertices)
    ///     .build();
    ///
    /// // This OBJ file will contain the resulting positions, viewable with Blender
    /// let target_path = PathBuf::from("circle_vertices.obj");
    ///
    /// // Create the file
    /// let sink = &mut File::create(&target_path)
    ///     .unwrap();
    ///
    /// // And finally dump the geometry to the OBJ at "circle_vertices.obj"
    /// circle_surface.dump(sink)
    ///     .unwrap();
    /// #
    /// # let written = File::open("circle_vertices.obj");
    /// # assert!(written.is_ok());
    /// #
    /// # // Clean up
    /// # remove_file(target_path);
    /// # }
    /// ```
    pub fn add_samples<I>(mut self, samples: I) -> Self
    where
        I: IntoIterator<Item = S>,
    {
        self.samples.extend(samples.into_iter());
        self
    }

    /// Consumes the builder to create a new surface that is returned.
    pub fn build(self) -> Surface<S> {
        let spatial_idx = {
            let mut tree = KdTree::new(3);

            self.samples
                .iter()
                .map(S::position)
                .enumerate()
                .for_each(|(idx, s)| tree.add([s.x as f64, s.y as f64, s.z as f64], idx).unwrap());

            tree
        };

        Surface {
            samples: self.samples,
            spatial_idx,
        }
    }
}
