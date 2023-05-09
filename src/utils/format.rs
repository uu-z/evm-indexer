use ethers::types::{Bytes, H160, H256, H64};

pub fn format_nonce(h: H64) -> String {
    format!("{:?}", h)
}

pub fn format_hash(h: H256) -> String {
    format!("{:?}", h)
}

pub fn format_address(h: H160) -> String {
    format!("{:?}", h)
}

pub fn format_bytes(b: &Bytes) -> String {
    serde_json::to_string(b).unwrap().replace('\"', "")
}

pub fn decode_bytes(s: String) -> Vec<u8> {
    let without_prefix = &s[2..];
    hex::decode(without_prefix).unwrap()
}

pub fn format_bytes_slice(b: &[u8]) -> String {
    format!("0x{}", hex::encode(b))
}

pub fn byte4_from_input(input: &str) -> [u8; 4] {
    let input_sanitized = input.strip_prefix("0x").unwrap();

    if input_sanitized.is_empty() {
        return [0x00, 0x00, 0x00, 0x00];
    }

    let input_bytes = hex::decode(input_sanitized).unwrap();

    if input_bytes.len() < 4 {
        return [0x00, 0x00, 0x00, 0x00];
    }

    let byte4: [u8; 4] =
        [input_bytes[0], input_bytes[1], input_bytes[2], input_bytes[3]];

    byte4
}

pub fn sanitize_string(str: String) -> String {
    let trim = str.trim_matches(char::from(0)).to_string();

    let remove_single_quotes: String = trim.replace('\'', "");

    let to_bytes = remove_single_quotes.as_bytes();

    let remove_non_utf8_chars = String::from_utf8_lossy(to_bytes);

    format!("'{}'", remove_non_utf8_chars)
}
