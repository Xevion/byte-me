use crate::models::StreamDetail;

pub fn extract_streams(info: &ffprobe::FfProbe) -> Vec<StreamDetail> {
    let mut streams = Vec::new();

    for stream in &info.streams {
        match stream.codec_type.as_deref() {
            Some("video") => {
                streams.push(StreamDetail::Video {
                    codec: stream
                        .codec_name
                        .clone()
                        .unwrap_or_else(|| "unknown".to_string()),
                    width: stream.width.map(|w| w as u32),
                    height: stream.height.map(|h| h as u32),
                    bit_rate: stream.bit_rate.as_ref().map(|b| b.to_string()),
                    frame_rate: Some(stream.r_frame_rate.clone()),
                });
            }
            Some("audio") => {
                streams.push(StreamDetail::Audio {
                    codec: stream
                        .codec_name
                        .clone()
                        .unwrap_or_else(|| "unknown".to_string()),
                    sample_rate: stream.sample_rate.clone(),
                    channels: stream.channels.map(|c| c as u32),
                    bit_rate: stream.bit_rate.as_ref().map(|b| b.to_string()),
                });
            }
            Some("subtitle") => {
                streams.push(StreamDetail::Subtitle {
                    codec: stream
                        .codec_name
                        .clone()
                        .unwrap_or_else(|| "unknown".to_string()),
                    language: stream.tags.as_ref().and_then(|tags| tags.language.clone()),
                });
            }
            _ => {}
        }
    }

    streams
}


