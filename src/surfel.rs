use geom::{Normal, Position, Texcoords, Vec2, Vec3};

pub struct Surfel<V, D> {
    /// An interpolated vertex at the surfel position
    vertex: V,
    /// Additional associated data of the surfel
    data: D,
}

impl<V: Position, D> Surfel<V, D> {
    pub fn new(vertex: V, data: D) -> Self {
        Surfel { vertex, data }
    }

    pub fn vertex(&self) -> &V {
        &self.vertex
    }

    pub fn data(&self) -> &D {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut D {
        &mut self.data
    }
}

impl<V: Position, D> Position for Surfel<V, D> {
    fn position(&self) -> Vec3 {
        self.vertex.position()
    }
}

impl<V: Normal, D> Normal for Surfel<V, D> {
    fn normal(&self) -> Vec3 {
        self.vertex.normal()
    }
}

impl<V: Texcoords, D> Texcoords for Surfel<V, D> {
    fn texcoords(&self) -> Vec2 {
        self.vertex.texcoords()
    }
}
