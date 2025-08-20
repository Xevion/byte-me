use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
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
pub struct StreamResult {
    pub path: String,
    pub filename: String,
    pub media_type: MediaType,
    pub duration: Option<f64>,
    pub size: u64,
    pub streams: Vec<StreamDetail>,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
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
pub struct StreamResultError {
    pub filename: Option<String>,
    pub reason: String,
    pub error_type: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn export_bindings() {
        // This will generate TypeScript bindings when you run `cargo test export_bindings`
        use super::*;

        StreamDetail::export_all_to("../src/bindings").expect("Failed to export bindings");
        StreamResult::export_all_to("../src/bindings").expect("Failed to export bindings");
        StreamResultError::export_all_to("../src/bindings").expect("Failed to export bindings");
        MediaType::export_all_to("../src/bindings").expect("Failed to export bindings");
    }
}
