pub fn parse_i32_to_vecu8(value: i32) -> Vec<u8> {
    let mut result = vec![];
    if value > 16_777_215 {
        let byte1 = (value & 0xFF) as u8; // Lowest 8 bits
        let byte2 = ((value >> 8) & 0xFF) as u8; // Next 8 bits
        let byte3 = ((value >> 16) & 0xFF) as u8; // Next 8 bits
        let byte4 = ((value >> 24) & 0xFF) as u8; // Highest 8 bits
        result.push(byte4);
        result.push(byte3);
        result.push(byte2);
        result.push(byte1);
    } else if value > 65_535 {
        let byte1 = (value & 0xFF) as u8; // Lowest 8 bits
        let byte2 = ((value >> 8) & 0xFF) as u8; // Next 8 bits
        let byte3 = ((value >> 16) & 0xFF) as u8; // Highest 8 bits
        result.push(0); // 1 padding byte for 4-byte alignment
        result.push(byte3);
        result.push(byte2);
        result.push(byte1);
    } else if value > 255 {
        let low_byte = (value & 0xFF) as u8; // Lowest 8 bits
        let high_byte = ((value >> 8) & 0xFF) as u8; // Highest 8 bits
        result.push(0); // 2 padding bytes for 4-byte alignment
        result.push(0);
        result.push(high_byte);
        result.push(low_byte);
    } else if value > -16_777_215 {
        let byte1 = (value & 0xFF) as u8; // Lowest 8 bits
        let byte2 = ((value >> 8) & 0xFF) as u8; // Next 8 bits
        let byte3 = ((value >> 16) & 0xFF) as u8; // Next 8 bits
        let byte4 = ((value >> 24) & 0xFF) as u8; // Highest 8 bits
        result.push(byte4);
        result.push(byte3);
        result.push(byte2);
        result.push(byte1);
    } else if value > -65_535 {
        // 3 bytes
        let byte1 = (value & 0xFF) as u8; // Lowest 8 bits
        let byte2 = ((value >> 8) & 0xFF) as u8; // Next 8 bits
        let byte3 = ((value >> 16) & 0xFF) as u8; // Highest 8 bits
        result.push(1);
        result.push(byte3);
        result.push(byte2);
        result.push(byte1);
    } else if value > -255 {
        let low_byte = (value & 0xFF) as u8; // Lowest 8 bits
        let high_byte = ((value >> 8) & 0xFF) as u8; // Highest 8 bits
        result.push(1); // 2 padding bytes for 4-byte alignment
        result.push(1);
        result.push(high_byte);
        result.push(low_byte);
    } else if value <= -1 {
        result.push(1);
        result.push(1);
        result.push(1);
        result.push(value as u8);
    } else {
        result.push(0); // 3 padding bytes for 4-byte alignment
        result.push(0);
        result.push(0);
        result.push(value as u8);
    }
    return result;
}
