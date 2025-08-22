use crate::models::StreamDetail;
use tracing::{debug, info, instrument};

#[instrument(skip(info), fields(stream_count = info.streams.len()))]
pub fn extract_streams(info: &ffprobe::FfProbe) -> Vec<StreamDetail> {
    let mut streams = Vec::new();
    let mut video_count = 0;
    let mut audio_count = 0;
    let mut subtitle_count = 0;

    info!(total_streams = info.streams.len(), "Extracting streams from media file");

    for (index, stream) in info.streams.iter().enumerate() {
        match stream.codec_type.as_deref() {
            Some("video") => {
                video_count += 1;
                let codec = stream
                    .codec_name
                    .clone()
                    .unwrap_or_else(|| "unknown".to_string());
                let width = stream.width.map(|w| w as u32);
                let height = stream.height.map(|h| h as u32);
                let bit_rate = stream.bit_rate.as_ref().map(|b| b.to_string());
                let frame_rate = Some(stream.r_frame_rate.clone());

                debug!(
                    stream_index = index,
                    codec = %codec,
                    width = ?width,
                    height = ?height,
                    bit_rate = ?bit_rate,
                    frame_rate = ?frame_rate,
                    "Extracted video stream"
                );

                streams.push(StreamDetail::Video {
                    codec,
                    width,
                    height,
                    bit_rate,
                    frame_rate,
                });
            }
            Some("audio") => {
                audio_count += 1;
                let codec = stream
                    .codec_name
                    .clone()
                    .unwrap_or_else(|| "unknown".to_string());
                let sample_rate = stream.sample_rate.clone();
                let channels = stream.channels.map(|c| c as u32);
                let bit_rate = stream.bit_rate.as_ref().map(|b| b.to_string());

                debug!(
                    stream_index = index,
                    codec = %codec,
                    sample_rate = ?sample_rate,
                    channels = ?channels,
                    bit_rate = ?bit_rate,
                    "Extracted audio stream"
                );

                streams.push(StreamDetail::Audio {
                    codec,
                    sample_rate,
                    channels,
                    bit_rate,
                });
            }
            Some("subtitle") => {
                subtitle_count += 1;
                let codec = stream
                    .codec_name
                    .clone()
                    .unwrap_or_else(|| "unknown".to_string());
                let language = stream.tags.as_ref().and_then(|tags| tags.language.clone());

                debug!(
                    stream_index = index,
                    codec = %codec,
                    language = ?language,
                    "Extracted subtitle stream"
                );

                streams.push(StreamDetail::Subtitle {
                    codec,
                    language,
                });
            }
            other => {
                debug!(
                    stream_index = index,
                    codec_type = ?other,
                    "Skipping unknown stream type"
                );
            }
        }
    }

    info!(
        video_streams = video_count,
        audio_streams = audio_count,
        subtitle_streams = subtitle_count,
        total_extracted = streams.len(),
        "Stream extraction completed"
    );

    streams
}
