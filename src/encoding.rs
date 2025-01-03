use base64::{prelude::BASE64_STANDARD, Engine};

/**
 * Convert a vector of u32 to a vector of u8.
 * The data is in RGBA format, so the bytes are in the order of RGBA.
 *
 * There is probably a faster/better way.
 */
pub fn convert_data_format(data: Vec<u32>) -> Vec<u8> {
    let result = data
        .into_iter()
        .map(|x| x.to_be_bytes())
        .collect::<Vec<[u8; 4]>>()
        .as_flattened()
        .to_vec();

    result
}

pub fn base64_encode(data: Vec<u8>) -> String {
    let str = BASE64_STANDARD.encode(data);

    str
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_data_format() {
        let data = vec![0x11223344, 0x55667788, 0x0000FFFF];

        let result = convert_data_format(data);

        assert_eq!(result.len(), 12);
        assert_eq!(result[0], 0x11);
        assert_eq!(result[1], 0x22);
        assert_eq!(result[2], 0x33);
        assert_eq!(result[3], 0x44);
        assert_eq!(result[4], 0x55);
    }
}
