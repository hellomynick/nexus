use std::sync::{LazyLock, RwLock};

pub const M3U_TAG: &str = "#EXTM3U";
pub const VER_TAG_V3: &str = "#EXT-X-VERSION:3";
pub const VER_TAG_V6: &str = "#EXT-X-VERSION:6";
pub const DURATION_TAG: &str = "#EXT-X-TARGETDURATION:";
pub const SEQ_TAG: &str = "#EXT-X-MEDIA-SEQUENCE:";
pub const SEGMENT_TAG: &str = "#EXT-X-INDEPENDENT-SEGMENTS";
pub const CHUNK: &str = "Chunk_";
pub const INF_TAG: &str = "#EXTINF:";
pub const URI_TAG: &str = "URI";

pub const DOWN_LINE_SYMBOL: &str = "\n";
pub const COMMA: &str = ",";
pub const TS_EXT: &str = ".ts";

pub static GLOBAL_M3U8_BUFFER: LazyLock<RwLock<String>> =
    LazyLock::new(|| RwLock::new(String::with_capacity(1024)));

pub fn generate_hls_response_m3u8_v3(duration: u8, seq: u32, chunk_duration: f32, chunk_id: u32) {
    let mut num_buf = itoa::Buffer::new();
    let mut float_buf = ryu::Buffer::new();

    let mut buffer = match GLOBAL_M3U8_BUFFER.write() {
        Ok(lock) => lock,
        Err(poisoned) => {
            println!("Failed to acquire write lock: {poisoned:?}");
            poisoned.into_inner()
        }
    };

    buffer.clear();

    // M3U tag
    buffer.push_str(M3U_TAG);
    buffer.push_str(DOWN_LINE_SYMBOL);

    // Version
    buffer.push_str(VER_TAG_V3);
    buffer.push_str(DOWN_LINE_SYMBOL);

    // Duration
    buffer.push_str(DURATION_TAG);
    buffer.push_str(num_buf.format(duration));
    buffer.push_str(DOWN_LINE_SYMBOL);

    // Media sequence
    buffer.push_str(SEQ_TAG);
    buffer.push_str(num_buf.format(seq));
    buffer.push_str(DOWN_LINE_SYMBOL);

    // Segment
    buffer.push_str(SEGMENT_TAG);
    buffer.push_str(DOWN_LINE_SYMBOL);

    // Inf
    buffer.push_str(INF_TAG);
    buffer.push_str(float_buf.format(chunk_duration));
    buffer.push_str(COMMA);
    buffer.push_str(DOWN_LINE_SYMBOL);

    // Chunk video
    buffer.push_str(CHUNK);
    buffer.push_str(num_buf.format(chunk_id));
    buffer.push_str(TS_EXT);
    buffer.push_str(DOWN_LINE_SYMBOL);
}

pub fn read_m3u8() -> Vec<u8> {
    let buffer = match GLOBAL_M3U8_BUFFER.read() {
        Ok(lock) => lock,
        Err(poisoned) => {
            panic!("Failed to acquire read lock: {poisoned:?}");
        }
    };

    return buffer.as_bytes().to_vec();
}
