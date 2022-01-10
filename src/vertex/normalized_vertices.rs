use super::*;

impl Deref for NormalizedVertices {
    type Target = Vertices;

    fn deref(&self) -> &Self::Target {
        &self.vertices
    }
}

pub struct NormalizedVertices {
    scale: f64,
    vertices: Vertices,
}

impl NormalizedVertices {
    fn calculate_vertex_range(vertices: &Vertices) -> VertexRange {
        debug_assert!(
            !vertices.is_empty(),
            "Cannot calculate vertext range  because vertexes have not been parsed"
        );
        let mut range = VertexRange::default();

        for vertex in vertices {
            let Vertex { x, y, z } = vertex;
            for dimension in [x, y, z] {
                if dimension > &range.max {
                    range.max = *dimension;
                }
                if dimension < &range.min {
                    range.min = *dimension;
                }
            }
        }
        range
    }
    pub fn normalize_vertices(vertices: &Vertices, scale: f64) -> Vertices {
        let mut normalized_verticies = Vertices::new();
        for vertex in vertices {
            // let normalized_vertex = ((*vertex / scale) + 1.0) * 300.0;
            let normalized_vertex = Self::normalize(vertex, scale);

            normalized_verticies.push(normalized_vertex);
        }
        normalized_verticies
    }

    #[inline]
    fn normalize(v1: &Vertex, scale: f64) -> Vertex {
        let vertex = *v1 / scale;
        vertex
    }
}

impl From<Vertices> for NormalizedVertices {
    fn from(vertices: Vertices) -> NormalizedVertices {
        let vertex_range = NormalizedVertices::calculate_vertex_range(&vertices);
        let scale = vertex_range.magnitude();
        let normalized_vertices = NormalizedVertices::normalize_vertices(&vertices, scale);
        NormalizedVertices {
            vertices: normalized_vertices,
            scale,
        }
    }
}
#[derive(Debug, PartialEq)]
pub struct VertexRange {
    min: f64,
    max: f64,
}
impl VertexRange {
    pub fn magnitude(&self) -> f64 {
        if self.min.abs() < self.max.abs() {
            self.max.abs()
        } else {
            self.min.abs()
        }
    }
}
impl Default for VertexRange {
    fn default() -> Self {
        VertexRange {
            min: f64::MAX, //Note this is correct min should be max and max should be min so the first value can be lower than the large min and crater than the small max
            max: f64::MIN,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertex_range_cessna_test() {
        let f = ModelFile::open_file("assets/cessna.obj").unwrap();
        let verts = f.vertex_parse();

        let calculated_range = NormalizedVertices::calculate_vertex_range(&verts);
        assert_eq!(
            calculated_range,
            VertexRange {
                min: -22.152081,
                max: 22.152081
            }
        )
    }
    #[test]
    fn vertex_range_head_test() {
        let f = ModelFile::open_file("assets/head.obj").unwrap();
        let verts = f.vertex_parse();

        let calculated_range = NormalizedVertices::calculate_vertex_range(&verts);
        assert_eq!(
            calculated_range,
            VertexRange {
                min: -1.0,
                max: 1.0
            }
        )
    }
    #[test]
    fn calculate_scale_cessna_test() {
        let mut f = ModelFile::open_file("assets/cessna.obj").unwrap();
        f.load();
        assert_eq!(22.152081, f.vertices.unwrap().scale)
    }

    #[test]
    fn calculate_scale_head_test() {
        let mut f = ModelFile::open_file("assets/head.obj").unwrap();
        f.load();
        assert_eq!(1.0, f.vertices.unwrap().scale)
    }

    #[test]
    fn calculate_scale_airboat_test() {
        let mut f = ModelFile::open_file("assets/airboat.obj").unwrap();
        f.load();
        assert_eq!(8.114171, f.vertices.unwrap().scale)
    }

    #[test]
    fn vertex_range_magnitude_test() {
        let vr = VertexRange { min: -3., max: 9. };
        assert_eq!(9., vr.magnitude());
    }
    #[test]
    fn normalize_verticies_test() {
        let verts: Vertices = vec![
            Vertex {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            Vertex {
                x: 4.0,
                y: -10.0,
                z: 10.0,
            },
        ];
        let norm_verts = NormalizedVertices::normalize_vertices(&verts, 10.);
        assert_eq!(
            norm_verts[0],
            Vertex {
                x: 0.1,
                y: 0.2,
                z: 0.3
            }
        );
        assert_eq!(
            norm_verts[1],
            Vertex {
                x: 0.4,
                y: -1.0,
                z: 1.0
            }
        );
    }
    #[test]
    fn normalize_verticies_test_2() {
        let verts: Vertices = vec![
            Vertex {
                x: 1.0,
                y: -4.0,
                z: 2.0,
            },
            Vertex {
                x: 1.0,
                y: 2.0,
                z: -3.0,
            },
        ];
        let norm_verts = NormalizedVertices::normalize_vertices(&verts, 4.);

        assert_eq!(
            norm_verts[0],
            Vertex {
                x: 0.25,
                y: -1.0,
                z: 0.5
            }
        );
        assert_eq!(
            norm_verts[1],
            Vertex {
                x: 0.25,
                y: 0.5,
                z: -0.75
            }
        );
    }
}
