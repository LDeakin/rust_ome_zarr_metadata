//! "plate" metadata
//!
//! <https://ngff--242.org.readthedocs.build/latest/index.html#plate-md>.

use std::num::NonZeroU64;

use serde::{Deserialize, Serialize};

use super::{PlateAcquisition, PlateColumn, PlateRow, PlateWell};

/// `plate` metadata. For high-content screening datasets.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Plate {
    /// A list of JSON objects defining the acquisitions for a given plate to which wells can refer to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquisitions: Option<Vec<PlateAcquisition>>,
    /// A list of JSON objects defining the columns of the plate
    pub columns: Vec<PlateColumn>,
    /// The field count defining the maximum number of fields per view across all wells (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_count: Option<NonZeroU64>,
    /// The name of the plate (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Defines the rows of the plate.
    pub rows: Vec<PlateRow>,
    /// Defines the wells of the plate.
    pub wells: Vec<PlateWell>,
}

#[cfg(test)]
mod tests {
    use crate::v0_5_rfc2::get_ome_attribute_from_zarr_group_metadata;

    use super::*;

    #[test]
    fn plate_0_5_rfc2_spec() {
        let json = r#"
{
  "zarr_format": 3,
  "node_type": "group",
  "attributes": {
    "ome": {
      "version": "0.5",
      "plate": {
        "acquisitions": [
          {
            "id": 1,
            "maximumfieldcount": 2,
            "name": "Meas_01(2012-07-31_10-41-12)",
            "starttime": 1343731272000
          },
          {
            "id": 2,
            "maximumfieldcount": 2,
            "name": "Meas_02(201207-31_11-56-41)",
            "starttime": 1343735801000
          }
        ],
        "columns": [
          {
            "name": "1"
          },
          {
            "name": "2"
          },
          {
            "name": "3"
          }
        ],
        "field_count": 4,
        "name": "test",
        "rows": [
          {
            "name": "A"
          },
          {
            "name": "B"
          }
        ],
        "wells": [
          {
            "path": "A/1",
            "rowIndex": 0,
            "columnIndex": 0
          },
          {
            "path": "A/2",
            "rowIndex": 0,
            "columnIndex": 1
          },
          {
            "path": "A/3",
            "rowIndex": 0,
            "columnIndex": 2
          },
          {
            "path": "B/1",
            "rowIndex": 1,
            "columnIndex": 0
          },
          {
            "path": "B/2",
            "rowIndex": 1,
            "columnIndex": 1
          },
          {
            "path": "B/3",
            "rowIndex": 1,
            "columnIndex": 2
          }
        ]
      }
    }
  }
}
"#;
        let group_metadata: serde_json::Map<String, serde_json::Value> =
            serde_json::from_str(json).unwrap();
        let ome_metadata = get_ome_attribute_from_zarr_group_metadata(&group_metadata).unwrap();
        let plate = ome_metadata.get("plate").unwrap();
        let _plate: Plate = serde_json::from_value(plate.clone()).unwrap();
    }

    #[test]
    fn plate_0_5_rfc2_minimal() {
        let json = r#"
{
  "zarr_format": 3,
  "node_type": "group",
  "attributes": {
    "ome": {
      "version": "0.5",
      "plate": {
        "columns": [
          {
            "name": "1"
          },
          {
            "name": "2"
          },
          {
            "name": "3"
          }
        ],
        "rows": [
          {
            "name": "A"
          },
          {
            "name": "B"
          }
        ],
        "wells": [
          {
            "path": "A/1",
            "rowIndex": 0,
            "columnIndex": 0
          },
          {
            "path": "A/2",
            "rowIndex": 0,
            "columnIndex": 1
          },
          {
            "path": "A/3",
            "rowIndex": 0,
            "columnIndex": 2
          },
          {
            "path": "B/1",
            "rowIndex": 1,
            "columnIndex": 0
          },
          {
            "path": "B/2",
            "rowIndex": 1,
            "columnIndex": 1
          },
          {
            "path": "B/3",
            "rowIndex": 1,
            "columnIndex": 2
          }
        ]
      }
    }
  }
}
"#;
        let group_metadata: serde_json::Map<String, serde_json::Value> =
            serde_json::from_str(json).unwrap();
        let ome_metadata = get_ome_attribute_from_zarr_group_metadata(&group_metadata).unwrap();
        let plate = ome_metadata.get("plate").unwrap();
        let _plate: Plate = serde_json::from_value(plate.clone()).unwrap();
    }
}
