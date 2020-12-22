mod cio;
mod hashing;
mod parsing;

use std::env;
use hex;

fn main() 
{
	// get enviorment vars that the user input
	let mut args = env::args();

	// attempt to get the file name that the user input
	let filename = match args.nth(1) {
		Some(data) => data,
		// if the user did not input a file, panic for now (better soloution tbd)
		None => {
			println!("ERROR: cannot find file");
			cio::usage();

			panic!();
		}
	};

	println!("Calculating hash...");

	// get the file bytes
	let mut file_bytes = Box::new(cio::get_file_byte_array(&filename));

	// and then calc the final hash
	let final_hash = hashing::get_hash(&mut file_bytes);
	// print the hash to screen (hex formatting tbd)
	println!("{}{}{}{}{}{}{}{}", final_hash[0], final_hash[1], final_hash[2], final_hash[3], final_hash[4], final_hash[5], final_hash[6], final_hash[7]);
}