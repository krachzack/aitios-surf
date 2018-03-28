extern crate aitios_surf as surf;
extern crate aitios_asset as asset;
extern crate aitios_scene as scene;

use surf::{SurfaceBuilder, SurfelSampling};
use asset::obj;
use scene::Mesh;
use std::path::PathBuf;
use std::fs::File;

#[derive(Clone)]
struct SurfelData {
    prop: i32
}

#[test]
fn test_torus() {
    let torus = obj::load("tests/torus.obj")
        .expect("Could not load test geometry");

    let torus_triangles = torus.iter()
        .flat_map(|e| e.mesh.triangles());

    let prototype_surfel_data = SurfelData { prop: -1 };

    let surface = SurfaceBuilder::new()
        .sampling(SurfelSampling::MinimumDistance(0.1))
        .sample_triangles(torus_triangles, &prototype_surfel_data)
        .build();

    assert_eq!(prototype_surfel_data.prop, surface.samples[0].data().prop);
    assert_eq!(prototype_surfel_data.prop, surface.samples[10].data().prop);

    // Also save the results in an OBJ for examination in blender
    let target_path = PathBuf::from("tests/torus_surfels.obj");

    // Create the file
    let sink = &mut File::create(&target_path)
        .unwrap();

    // And finally dump the geometry to the OBJ at "circle_vertices.obj"
    surface.dump(sink)
        .unwrap();
}
