use serde::{Deserialize, Serialize};

use super::super::{CoordinateTransform, CoordinateTransformInner};

/// Build a high dimensional transformation using lower dimensional transformations on subsets of dimensions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ByDimension {
    /// Transformations applied to subsets of axes.
    pub transformations: Vec<ByDimensionTransformation>
}

/// Transformations on subsets of axes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByDimensionTransformation {
    /// Which axes to apply this transformation to.
    pub input_axes: Vec<usize>,
    /// Which of the output axes this transformation produces.
    pub output_axes: Vec<usize>,
    /// Transformation to apply.
    #[serde(flatten)]
    pub inner: CoordinateTransform,
}

impl validatrix::Validate for ByDimension {
    fn validate_inner(&self, _accum: &mut validatrix::Accumulator) {
        todo!()
    }
}

impl super::TransformationType for ByDimension {
    fn invertible(&self) -> Option<bool> {
        for t in self.transformations.iter() {
            if !t.inner.invertible()? {
                return Some(false);
            }
        }
        Some(true)
    }

    fn input_ndim(&self) -> Option<usize> {
        Some(self.transformations.iter().map(|t| t.input_axes.len()).sum())
    }

    fn output_ndim(&self) -> Option<usize> {
        Some(self.transformations.iter().map(|t| t.output_axes.len()).sum())
    }
}

impl From<ByDimension> for super::CoordinateTransformInner {
    fn from(value: ByDimension) -> Self {
        Self::ByDimension(value)
    }
}
