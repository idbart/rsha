

pub fn convert_byte_vec_to_u32_vec(bytes: &Vec<u8>) -> Result<Vec<u32>, String>
{
    if bytes.len() % 4 != 0
    {
        return Result::Err(String::from(format!("byte array not correct length: {}", bytes.len())))
    }

    let mut file_words = Vec::new();

    let mut i = 0;
    while i < bytes.len()
    {
        file_words.push(u32::from_ne_bytes([bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]));
        i += 4;
    }


    Result::Ok(file_words)
}