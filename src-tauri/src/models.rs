use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
pub enum MediaType {
    Audio,
    Video,
    Image,
    Document,
    Executable,
    Archive,
    Library,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
pub struct StreamResult {
    pub path: String,
    pub filename: String,
    pub media_type: MediaType,
    pub duration: Option<f64>,
    pub size: u64,
    pub streams: Vec<StreamDetail>,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
pub enum StreamDetail {
    Video {
        codec: String,
        width: Option<u32>,
        height: Option<u32>,
        bit_rate: Option<String>,
        frame_rate: Option<String>,
    },
    Audio {
        codec: String,
        sample_rate: Option<String>,
        channels: Option<u32>,
        bit_rate: Option<String>,
    },
    Subtitle {
        codec: String,
        language: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
pub struct StreamResultError {
    pub filename: Option<String>,
    pub reason: String,
    pub error_type: String,
}
