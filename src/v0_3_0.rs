//! Structures for the [v0.3.0
//! version](https://github.com/singer-io/getting-started/blob/967232686a1c4ef3b23d59b35dc0ba2f5e92d0f9/docs/SPEC.md)
//! of the Singer specification.
//!
//! Meltano [provides a document](https://hub.meltano.com/singer/spec/) with the various bits of
//! the specification that are not in Singer's `SPEC.md` file.
use serde::{Deserialize, Serialize};

/// Messages sent over stdout or read from stdin.
///
/// Each message MUST be serialized to JSON on a single line.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Message {
    /// Record messages contain the actual data being extracted from a source system.
    #[serde(rename = "RECORD")]
    Record {
        stream: String,
        record: serde_json::Value,
        #[serde(with = "time::serde::rfc3339::option")]
        time_extracted: Option<time::OffsetDateTime>,
    },
    /// Schema messages define the structure of the data sent in a record message.
    #[serde(rename = "SCHEMA")]
    Schema {
        stream: String,
        schema: serde_json::Value,
        key_properties: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        bookmark_properties: Option<Vec<String>>,
    },
    /// State messages contain any information that a tap is designed to persist.
    /// These are used to inform the target of the current place in the
    /// extraction of a data stream.
    #[serde(rename = "STATE")]
    State { value: serde_json::Value },
}

/// A tap can periodically emit structured log messages containing metrics about read operations.
/// Consumers of the tap logs can parse these metrics out of the logs for monitoring or analysis.
///
/// Metrics should be emitted on `stdout` in the format `INFO METRIC: <metric-json>`.
///
/// # Example
///
/// ```
/// use singer_protocol::v0_3_0::{Metric, MetricType, MetricValue};
///
/// let example_metric = Metric {
///     metric_type: MetricType::Counter,
///     metric: "records".to_string(),
///     value: MetricValue::Integer(42),
///     tags: serde_json::json!({
///         "number_of_tags": 1,
///         "quality of tags": "great",
///     }),
/// };
/// eprintln!("INFO METRIC: {}", serde_json::to_string(&example_metric).unwrap());
/// ```
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Metric {
    #[serde(rename = "type")]
    pub metric_type: MetricType,
    /// The name of the metric. This should consist only of letters, numbers,
    /// underscore, and dash characters. For example, "http_request_duration".
    pub metric: String,
    /// The value of the data-point.
    pub value: MetricValue,
    /// Mapping of tags describing the data. The keys can be any strings
    /// consisting solely of letters, numbers, underscores, and dashes.
    pub tags: serde_json::Value,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MetricType {
    #[serde(rename = "counter")]
    Counter,
    #[serde(rename = "timer")]
    Timer,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MetricValue {
    Integer(i64),
    Float(f64),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Stream {
    /// The primary identifier of the stream as it will be passed to the target.
    pub stream: String,
    /// The unique identifier of the stream. This can differ from `stream` since some sources
    /// may have multiple available streams with the same name
    pub tap_stream_id: String,
    pub schema: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<Metadata>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Include {
    Available,
    Automatic,
    Unsupported,
}

impl Default for Include {
    fn default() -> Self {
        Self::Available
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ReplicationMethod {
    #[serde(rename = "FULL_TABLE")]
    FullTable,
    #[serde(rename = "INCREMENTAL")]
    Incremental,
    #[serde(rename = "LOG_BASED")]
    LogBased,
}

/// Metadata that is provided to the tap and is not discoverable from the source.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NonDiscoverableMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "replication-method")]
    pub replication_method: Option<ReplicationMethod>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "replication-key")]
    pub replication_key: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "view-key-properties"
    )]
    pub view_key_properties: Option<Vec<String>>,
}

/// Metadata about the source that is potentially discoverable by the tap.
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct DiscoverableMetadata {
    pub inclusion: Include,
    #[serde(rename = "selected-by-default")]
    pub selected_by_default: bool,
    #[serde(rename = "valid-replication-keys")]
    pub valid_replication_keys: Vec<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "forced-replication-method"
    )]
    pub forced_replication_method: Option<ReplicationMethod>,
    #[serde(rename = "table-key-properties")]
    pub table_key_properties: Vec<String>,
    #[serde(rename = "schema-name")]
    pub schema_name: Option<String>,
    #[serde(rename = "is-view")]
    pub is_view: Option<bool>,
    #[serde(rename = "row-count")]
    pub row_count: Option<u64>,
    #[serde(rename = "database-name")]
    pub database_name: Option<String>,
    #[serde(rename = "sql-datatype")]
    pub sql_database: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Metadata {
    /// A JSON object containing all of the metadata for either the stream or a
    /// property of the stream.
    ///
    /// The JSON object can contain any keys it wants, but a number of reserved
    /// keywords exist.
    pub metadata: serde_json::Value,
    /// This identifies whether the metadata applies to the entire stream or a
    /// property of the stream. An empty list means the metadata applies to the
    /// stream. For specific properties within the stream, the breadcrumb will
    /// have the properties key followed by the name of the property being
    /// described.
    pub breadcrumb: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Catalog {
    pub streams: Vec<Stream>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_serialization() {
        let result = serde_json::to_string(&Message::State {
            value: serde_json::json!({"key": "value"}),
        })
        .unwrap();
        assert_eq!(
            result,
            r#"{"type":"STATE","value":{"key":"value"}}"#.to_string()
        );
    }

    #[test]
    fn metric_serialization() {
        let example_metric = Metric {
            metric_type: MetricType::Counter,
            metric: "records".to_string(),
            value: MetricValue::Integer(42),
            tags: serde_json::json!({
                "number_of_tags": 1,
                "quality of tags": "great",
            }),
        };
        assert_eq!(
            r#"{"type":"counter","metric":"records","value":42,"tags":{"number_of_tags":1,"quality of tags":"great"}}"#,
            serde_json::to_string(&example_metric).unwrap()
        );
    }
}
