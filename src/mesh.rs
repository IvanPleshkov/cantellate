use crate::vec3::Vec3;
use num_traits::{float::Float, FromPrimitive, ToPrimitive};
use std::{path::Path, vec};

pub type SmallVec<T> = smallvec::SmallVec<[T; 4]>;

#[derive(Debug, Default, Clone)]
pub struct Mesh<N>
where
    N: Float + ToPrimitive + FromPrimitive + Default,
{
    pub vertices: Vec<Vec3<N>>,
    pub faces: Vec<SmallVec<usize>>,
}

impl<N> Mesh<N>
where
    N: Float + ToPrimitive + FromPrimitive + Default,
{
    pub fn load_obj(path: impl AsRef<Path>) -> Result<Self, String> {
        let obj_data = obj::Obj::load(path).map_err(|e| format!("Failed to load OBJ file: {e}"))?;
        let vertices: Vec<Vec3<N>> = obj_data
            .data
            .position
            .iter()
            .cloned()
            .map(Into::into)
            .collect();

        // unite all objects and groups into a single mesh
        let faces: Vec<SmallVec<_>> = obj_data
            .data
            .objects
            .iter()
            .flat_map(|object| object.groups.iter())
            .flat_map(|group| group.polys.iter())
            .map(|poly| poly.0.iter().map(|index_tuple| index_tuple.0).collect())
            .collect();

        Ok(Self { vertices, faces })
    }

    pub fn save_obj(&self, path: impl AsRef<Path>) -> Result<(), String> {
        let path = path.as_ref();
        let obj_group = obj::Group {
            name: "default".to_owned(),
            polys: self
                .faces
                .iter()
                .map(|face| {
                    obj::SimplePolygon(
                        face.iter()
                            .map(|&index| obj::IndexTuple(index, None, None))
                            .collect(),
                    )
                })
                .collect(),
            index: 0,
            material: None,
        };
        let obj_data = obj::Obj {
            data: obj::ObjData {
                position: self.vertices.iter().cloned().map(Into::into).collect(),
                objects: vec![obj::Object {
                    name: "mesh".to_owned(),
                    groups: vec![obj_group],
                }],
                ..Default::default()
            },
            path: path.to_owned(),
        };
        obj_data
            .save(path)
            .map_err(|e| format!("Failed to save OBJ file: {e}"))
    }
}
