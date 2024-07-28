use std::collections::HashMap;

use crate::{
    mesh::{Mesh, SmallVec},
    vec3::Vec3,
};
use num_traits::{float::Float, FromPrimitive, ToPrimitive};
use parking_lot::Mutex;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

/// Cantellate the mesh.
/// The cantellation factor is an absolute distance from the original face to the cantellated face.
/// The epsilon is a small value to compare floating point numbers.
pub fn cantellate<N>(
    mesh: &Mesh<N>,
    factor: N,
    epsilon: N,
) -> Mesh<N>
where
    N: Float + ToPrimitive + FromPrimitive + Default + Send + Sync,
{
    let mut result_mesh = Mutex::new(Mesh::default());

    // calculate the normal of each face.
    let faces_normal = get_faces_normal(mesh, epsilon);

    let timer = std::time::Instant::now();
    let mut cantellated_vertices: Vec<_> = (0..mesh.vertices.len())
        .map(|vertex_index| CantellatedVertex::new(vertex_index))
        .collect();
    println!("Faces normal calculation took {:?}", timer.elapsed());

    // fill the map from the vertex to their faces
    let timer = std::time::Instant::now();
    fill_vertices_fo_faces(mesh, &mut cantellated_vertices);
    println!("Fill vertices for faces took {:?}", timer.elapsed());

    // cantellate the vertices
    // this operation expands all vertices by their faces
    // all vertices of result mesh are create here, expanded edges and faces are created later
    // using the information about the cantellated vertices
    let timer = std::time::Instant::now();
    cantellate_vertices(
        mesh,
        &mut result_mesh,
        &mut cantellated_vertices,
        &faces_normal,
        factor,
        epsilon,
    );
    println!("Cantellate vertices took {:?}", timer.elapsed());

    let timer = std::time::Instant::now();
    cantellate_edges(mesh, &mut result_mesh, &cantellated_vertices);
    println!("Cantellate edges took {:?}", timer.elapsed());

    let timer = std::time::Instant::now();
    cantellate_faces(mesh, &mut result_mesh, &cantellated_vertices);
    println!("Cantellate faces took {:?}", timer.elapsed());

    result_mesh.into_inner()
}

fn cantellate_vertices<N>(
    mesh: &Mesh<N>,
    result_mesh: &Mutex<Mesh<N>>,
    cantellated_vertices: &mut [CantellatedVertex],
    faces_normal: &[Option<Vec3<N>>],
    factor: N,
    epsilon: N,
) where
    N: Float + ToPrimitive + FromPrimitive + Default + Send + Sync,
{
    cantellated_vertices
        .into_par_iter()
        .for_each(|cantellated_vertex| {
            cantellated_vertex.cantellate(&faces_normal, factor, epsilon, mesh, result_mesh);
        });
}

fn cantellate_faces<N>(
    mesh: &Mesh<N>,
    result_mesh: &Mutex<Mesh<N>>,
    cantellated_vertices: &[CantellatedVertex],
) where
    N: Float + ToPrimitive + FromPrimitive + Default + Send + Sync,
{
    mesh.faces.par_iter().enumerate().for_each(|(face_index, face)| {
        let mut cantellated_face = SmallVec::<usize>::new();
        for &vertex_index in face {
            let new_vertex_index =
                cantellated_vertices
                    .get(vertex_index)
                    .and_then(|cantellated_vertex| {
                        cantellated_vertex.get_cantellated_vertex_by_face(face_index)
                    });
            if let Some(new_vertex_index) = new_vertex_index {
                cantellated_face.push(new_vertex_index);
            }
        }
        if cantellated_face.len() > 2 {
            result_mesh.lock().faces.push(cantellated_face);
        }
    });
}

fn cantellate_edges<N>(
    mesh: &Mesh<N>,
    result_mesh: &Mutex<Mesh<N>>,
    cantellated_vertices: &[CantellatedVertex],
) where
    N: Float + ToPrimitive + FromPrimitive + Default + Send + Sync,
{
    // First pass. Store all edges in forward direction.
    // Do it in a single thread mode because we need to store all edges in a single collection.
    let mut edges: HashMap<(usize, usize), SmallVec<usize>> = HashMap::new();
    for (face_index, face) in mesh.faces.iter().enumerate() {
        for i in 0..face.len() {
            let v1 = face[i];
            let v2 = face[(i + 1) % face.len()];
            edges
                .entry((v1, v2))
                .or_insert_with(SmallVec::new)
                .push(face_index);
        }
    }

    // Second pass. Cantellate the edges in parallel.
    mesh.faces.par_iter().enumerate().for_each(|(face_index, face)| {
        if !is_face_valid(face) {
            return;
        }

        for i in 0..face.len() {
            let v1 = face[i];
            let v2 = face[(i + 1) % face.len()];
            let another_faces = edges
                .get(&(v2, v1))
                .map(|faces| faces.as_slice())
                .unwrap_or_default();
            for &another_face_index in another_faces {
                // avoid duplicate edges
                if another_face_index <= face_index {
                    continue;
                }

                let edge_face = get_cantellated_edge_face(
                    cantellated_vertices,
                    v1,
                    v2,
                    face_index,
                    another_face_index,
                );

                if let Some(edge_face) = edge_face {
                    result_mesh.lock().faces.push(edge_face);
                }
            }
        }
    });
}

fn get_cantellated_edge_face(
    cantellated_vertices: &[CantellatedVertex],
    v1: usize,
    v2: usize,
    face1: usize,
    face2: usize,
) -> Option<SmallVec<usize>> {
    let e1 = cantellated_vertices
        .get(v1)?
        .get_cantellated_vertex_by_face(face2)?;
    let e2 = cantellated_vertices
        .get(v2)?
        .get_cantellated_vertex_by_face(face2)?;
    let e3 = cantellated_vertices
        .get(v2)?
        .get_cantellated_vertex_by_face(face1)?;
    let e4 = cantellated_vertices
        .get(v1)?
        .get_cantellated_vertex_by_face(face1)?;

    let mut edge_face = SmallVec::<usize>::new();
    edge_face.push(e1);
    edge_face.push(e2);
    // e3 and e2 were expanded by the same vertex, check if the vertex is expanded.
    if e3 != e2 {
        edge_face.push(e3);
    }
    // e1 and e4 were expanded by the same vertex, check if the vertex is expanded.
    if e4 != e1 {
        edge_face.push(e4);
    }

    if edge_face.len() > 2 {
        Some(edge_face)
    } else {
        None
    }
}

fn fill_vertices_fo_faces<N>(mesh: &Mesh<N>, cantellated_vertices: &mut [CantellatedVertex])
where
    N: Float + ToPrimitive + FromPrimitive + Default,
{
    // Fill the vertices_fo_faces as is
    mesh.faces
        .iter()
        .enumerate()
        .for_each(|(face_index, face)| {
            if !is_face_valid(face) {
                return;
            }
            face.iter().for_each(|&vertex_index| {
                // do safe access because we don't know if the mesh is valid
                if let Some(cantellated_vertex) = cantellated_vertices.get_mut(vertex_index) {
                    cantellated_vertex.faces.push(face_index);
                }
            });
        });
}

/// Calculate the normal of each face.
fn get_faces_normal<N>(
    mesh: &Mesh<N>,
    epsilon: N,
) -> Vec<Option<Vec3<N>>>
where
    N: Float + ToPrimitive + FromPrimitive + Default + Send + Sync,
{
    (0..mesh.faces.len())
        .into_par_iter()
        .map(|face_index| {
            let face = &mesh.faces[face_index];
            let points: Vec<_> = face
                .iter()
                .map(|&vertex_index| mesh.vertices[vertex_index])
                .collect();
            find_face_normal(points, epsilon)
        })
        .collect()
}

fn find_face_normal<N>(points: Vec<Vec3<N>>, epsilon: N) -> Option<Vec3<N>>
where
    N: Float + ToPrimitive + FromPrimitive + Default,
{
    if points.len() < 3 {
        // not enough vertices to form a face
        return None;
    } else if points.len() == 3 {
        // face is a triangle, just do cross product
        let v1 = points[1] - points[0];
        let v2 = points[2] - points[0];
        return v1.cross(v2).normalize(epsilon);
    }

    let mut normal = Vec3::zero();
    for i in 0..points.len() {
        let a = points[i];
        let b = points[(i + 1) % points.len()];
        let c = points[(i + 2) % points.len()];
        normal = normal + (c - b).cross(a - b);
    }

    normal.normalize(epsilon)
}

// face is valid if it has at least 3 vertices and all vertices are different
fn is_face_valid(face: &[usize]) -> bool {
    face.len() > 2
        && (1..face.len()).all(|i| {
            let vertex = face[i - 1];
            face.iter().skip(i).all(|&v| v != vertex)
        })
}

/// Help structure to store the information of the cantellated vertex.
struct CantellatedVertex {
    /// Index of the vertex in the original mesh.
    index: usize,

    /// Faces in the original mesh that the vertex is part of.
    faces: SmallVec<usize>,

    /// Vetices in the result mesh that are produced by the vertex cantellation.
    /// The order of the vertices is the same as the order of the faces.
    /// Indices may be equal to each other in case when the cantellated vertex position is the same by different faces.
    cantellated: SmallVec<usize>,
}

impl CantellatedVertex {
    fn new(index: usize) -> Self {
        Self {
            index,
            faces: SmallVec::new(),
            cantellated: SmallVec::new(),
        }
    }

    /// Expand the vertex and create a face of the expanded vertex.
    fn cantellate<N>(
        &mut self,
        faces_normal: &[Option<Vec3<N>>],
        factor: N,
        epsilon: N,
        mesh: &Mesh<N>,
        result_mesh: &Mutex<Mesh<N>>,
    ) where
        N: Float + ToPrimitive + FromPrimitive + Default,
    {
        let vertex = mesh.vertices[self.index];

        // If the vertex is not part of any face, then it is a single point.
        if self.faces.is_empty() {
            result_mesh.lock().vertices.push(vertex);
            return;
        }

        let is_watertight = self.sort_faces(mesh);

        let mut face = SmallVec::<usize>::new();
        for &face_index in &self.faces {
            let cantellated_vertex = if let Some(face_normal) = faces_normal[face_index] {
                vertex + face_normal * factor
            } else {
                vertex
            };

            // it's needed to decide if the vertex is the same as a neighbour vertex
            // it is needed to avoid duplicate vertices and avoid zero-length edges
            let same_neighbour = if let Some(&prev_index) = self.cantellated.last() {
                // check with the previous vertex as a constructed neightbour
                let prev_vertex = result_mesh.lock().vertices[prev_index];
                let diff = cantellated_vertex - prev_vertex;
                let same_neighbour = if diff.length() < epsilon {
                    Some(prev_index)
                } else {
                    None
                };

                // special case. if vertex is the last, compare also with the first
                if same_neighbour.is_none() && self.cantellated.len() + 1 == self.faces.len() {
                    let first_index = self.cantellated[0];
                    let first_vertex = result_mesh.lock().vertices[first_index];
                    let diff = cantellated_vertex - first_vertex;
                    if diff.length() < epsilon {
                        Some(first_index)
                    } else {
                        None
                    }
                } else {
                    same_neighbour
                }
            } else {
                // first vertex has no any constructed neighbours to check
                None
            };

            if let Some(same_neighbour) = same_neighbour {
                // if the vertex is the same as a neighbour vertex,
                // then use the neighbour vertex as a cantellated vertex by face
                self.cantellated.push(same_neighbour);
            } else {
                // create a new vertex in the result mesh
                let mut result_mesh = result_mesh.lock();
                let new_vertex_index = result_mesh.vertices.len();
                self.cantellated.push(new_vertex_index);
                face.push(new_vertex_index);
                result_mesh.vertices.push(cantellated_vertex);
            }
        }

        // non-watertight vertex has no cantellated face
        if is_watertight && face.len() > 2 {
            result_mesh.lock().faces.push(face);
        }
    }

    /// sort vertex faces by correct clockwise order
    /// return true if the vertex is watertight
    /// non-watertight vertex has no cantellated face and sort is not needed
    fn sort_faces<N>(&mut self, mesh: &Mesh<N>) -> bool
    where
        N: Float + ToPrimitive + FromPrimitive + Default,
    {
        if self.faces.len() < 2 {
            return false;
        }

        // All edges from the vertex.
        let mut vertex_forward_edges: SmallVec<_> = self
            .faces
            .iter()
            .filter_map(|&face_index| {
                let face = &mesh.faces[face_index];
                let vertex_index_in_face = face.iter().position(|&v| v == self.index)?;
                Some(face[(vertex_index_in_face + 1) % face.len()])
            })
            .collect();

        // All edges to the vertex.
        let mut vertex_backward_edges: SmallVec<_> = self
            .faces
            .iter()
            .filter_map(|&face_index| {
                let face = &mesh.faces[face_index];
                let vertex_index_in_face = face.iter().position(|&v| v == self.index)?;
                Some(face[(vertex_index_in_face + face.len() - 1) % face.len()])
            })
            .collect();

        // Something went wrong, just skip expanded polygon.
        if vertex_forward_edges.len() != self.faces.len()
            || vertex_backward_edges.len() != self.faces.len()
        {
            return false;
        }

        let mut is_watertight = true;
        // Do in-place sort of the faces.
        for i in 1..self.faces.len() - 1 {
            let edge_next = vertex_backward_edges[i - 1];

            let next_face = vertex_forward_edges
                .iter()
                .skip(i)
                .position(|&edge| edge == edge_next)
                .map(|j| i + j);

            if let Some(next_face) = next_face {
                self.faces.swap(i, next_face);
                vertex_forward_edges.swap(i, next_face);
                vertex_backward_edges.swap(i, next_face);
            } else {
                is_watertight = false;
                break;
            }
        }

        if is_watertight {
            let last_edge = vertex_backward_edges[vertex_backward_edges.len() - 1];
            let first_edge = vertex_forward_edges[0];
            last_edge == first_edge
        } else {
            false
        }
    }

    /// find the cantellated vertex by face.
    fn get_cantellated_vertex_by_face(&self, face_index: usize) -> Option<usize> {
        self.faces.iter().zip(self.cantellated.iter()).find_map(
            |(&vertex_face, &cantellated_vertex)| {
                if vertex_face == face_index {
                    Some(cantellated_vertex)
                } else {
                    None
                }
            },
        )
    }
}
