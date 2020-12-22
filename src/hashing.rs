mod preprocessing;

use crate::parsing;

pub fn get_hash(bytes: &mut Vec<u8>) -> [u32; 8]
{
	// init the hash
	let mut hash: [u32; 8] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19]; 

	// add padding to the data (so that the length is a multiple of 512)
	preprocessing::pad(bytes);
	
	// convert the vector of bytes to a vector of 32 bit words
	let words = match parsing::convert_byte_vec_to_u32_vec(bytes) {
		Ok(data) => Box::new(data),
		Err(e) => {
			println!("{}", e);
			panic!();
		}
	};

	let mut i = 0;
	while i < words.len()
	{
		println!("the {} word is: {}", i, words[i]);
		i += 16;
	}

	hash
}