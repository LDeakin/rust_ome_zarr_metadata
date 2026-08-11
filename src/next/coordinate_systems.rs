use std::collections::BTreeSet;

use crate::v0_4::AxisUnit;
use serde::{Deserialize, Serialize};
use validatrix::{Accumulator, Validate};

/// A named set of axes representing a known space.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoordinateSystem {
    /// Name of the coordinate system.
    pub name: String,
    /// Ordered axes of the coordinate system.
    pub axes: Vec<Axis>,
}

impl Validate for CoordinateSystem {
    fn validate_inner(&self, accum: &mut validatrix::Accumulator) {
        accum.with_key("axes", |a| valid_axes(a, &self.axes));
    }
}

pub(crate) fn unique_axis_names(accum: &mut Accumulator, axes: &[Axis]) {
    let mut names = BTreeSet::default();
    for (idx, a) in axes.iter().enumerate() {
        if !names.insert(a.name.as_str()) {
            accum.with_keys(&[idx.into(), "name".into()], |ac| ac.add_failure(format!("duplicate axis name '{}'", a.name)));
        }
    }
}

pub(crate) fn valid_axes(accum: &mut Accumulator, axes: &[Axis]) {
    accum.validate_iter(axes);
    unique_axis_names(accum, axes);
    // TODO: RFC-5 (coordinate transformations) might implicitly include RFC-3 (loosening dimensionality constraints).
    // Here it's assumed we don't need to validate t?c?z?yx
}

/// [`Axis`] `type` metadata. Represents the type of an axis.
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum AxisType {
    /// The `array` axis type. Always discrete.
    Array,
    /// The `space` axis type.
    Space,
    /// The `time` axis type.
    Time,
    /// The `channel` axis type.
    Channel,
    /// The `coordinate` axis type.
    Coordinate,
    /// The `displacement` axis type.
    Displacement,
    #[serde(untagged)]
    /// A custom axis type.
    Custom(String),
}

/// `axis` element metadata. Represents a dimension (axis) of a physical coordinate space.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Axis {
    /// The name for this dimension.
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A longer, more descriptive name for the axis.
    pub long_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether this axis is discrete (i.e. may not be interpolated).
    pub discrete: Option<bool>,
    /// The optional type of this dimension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AxisType>,
    /// The optional physical unit of this dimension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<AxisUnit>,
}

impl Validate for Axis {
    fn validate_inner(&self, accum: &mut validatrix::Accumulator) {
        if matches!(self.r#type, Some(AxisType::Array)) && matches!(self.discrete, Some(false)) {
            accum.add_failure_at("discrete", "array axes must be discrete");
        }
    }
}

impl Axis {
    /// Whether this axis is discrete.
    /// This checks for [AxisType::Array] axes, and so should be preferred over directly accessing [Axis::discrete].
    pub fn is_discrete(&self) -> bool {
        matches!(self.r#type, Some(AxisType::Array)) || self.discrete.unwrap_or(false)
    }
}
