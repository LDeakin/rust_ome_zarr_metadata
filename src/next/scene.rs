use super::CoordinateSystem;
use super::CoordinateTransform;
use serde::{Deserialize, Serialize};

/// Description of spatial relationship between images.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scene {
    /// List of coordinate transformations.
    pub coordinate_transformations: Vec<CoordinateTransform>,
    /// List of coordinate systems.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coordinate_systems: Vec<CoordinateSystem>,
}

/// Reference to another coordinate system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CoordinateSystemRef {
    /// Path to a zarr array whose intrinsic coordinate system is referenced.
    ///
    /// https://github.com/ome/ngff/pull/389/changes#r2627747020
    Path(String),
    /// Name of a coordinate system in the coordinateSystems list of the containing [Scene].
    Name(String),
    /// Path to a group containing multiscales data and the name of a coordinate system referenced therein.
    #[serde(untagged)]
    Both {
        /// Path to a zarr group containing multiscales metadata.
        path: String,
        /// Name of a coordinate system referenced in one of the multiscales objects in that group's metadata.
        name: String,
    },
}

impl CoordinateSystemRef {
    /// Get the path and name of the referenced coordinate system.
    /// At least one is guaranteed to be Some.
    pub fn path_name(&self) -> (Option<&str>, Option<&str>) {
        match self {
            CoordinateSystemRef::Path(p) => (Some(p.as_str()), None),
            CoordinateSystemRef::Name(n) => (None, Some(n.as_str())),
            CoordinateSystemRef::Both { path, name } => (Some(path.as_str()), Some(name.as_str())),
        }
    }
}
