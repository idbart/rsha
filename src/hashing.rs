mod preprocessing;
mod functions;

use crate::parsing;
use std::num::Wrapping;

pub fn get_hash(bytes: &mut Vec<u8>) -> [u32; 8]
{
	// init the hash
	let mut hash: Box<[u32; 8]> = Box::new([0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19]); 

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

	// define the 64 constants (the first thirty-two bits of the fractional parts of the cube roots of the first sixty-four prime numbers)
	let k: Box<[u32; 64]> = Box::new([0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be,  0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2]);

	// start the main loop
	let mut i = 0;
	while i < words.len()
	{
		// load the current hash value into temps
		let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *hash; 

		// prep the message schedule for this block
		let w: Box<[u32; 64]> = {

			let mut w = Box::new([0; 64]);
			for j in 0..64
			{
				if j < 16
				{
					w[j] = words[i + j];
				}
				else
				{
					w[j] = (Wrapping(functions::sigma_lower_1(w[j - 1])) + Wrapping(w[j - 7]) + Wrapping(functions::sigma_lower_0(w[j - 15])) + Wrapping(w[j - 16])).0;
				}
			} 
			w
		};

		for t in 0..64
		{
			// make the temps
			let temp_1 = Wrapping(h) + Wrapping(functions::sigma_upper_1(e)) + Wrapping(functions::Ch(e, f, g)) + Wrapping(k[t]) + Wrapping(w[t]);
			let temp_2 = Wrapping(functions::sigma_upper_0(a)) + Wrapping(functions::Maj(a, b, c));

			// shift the thing?
			h = g;
			g = f;
			f = e;
			e = (Wrapping(d) + temp_1).0;
			d = c;
			c = b;
			b = a;
			a = (temp_1 + temp_2).0; 
		}

		// set the new hash and do it all over again
		*hash = [(Wrapping(a) + Wrapping(hash[0])).0, (Wrapping(b) + Wrapping(hash[1])).0, (Wrapping(c) + Wrapping(hash[2])).0, (Wrapping(d) + Wrapping(hash[3])).0, (Wrapping(e) + Wrapping(hash[4])).0, (Wrapping(f) + Wrapping(hash[5])).0, (Wrapping(g) + Wrapping(hash[6])).0, (Wrapping(h) + Wrapping(hash[7])).0];
		i += 16;
	}

	// return hash after the main loop has finished
	*hash
}