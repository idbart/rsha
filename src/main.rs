mod cio;
mod hashing;

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

	// get the bytes of the file and then calc the final hash
	let file = cio::get_file_byte_array(&filename);
	let final_hash = hashing::get_hash(&file);
	// print the hash to screen (hex formatting tbd)
	println!("{}{}{}{}{}{}{}{}", final_hash[0], final_hash[1], final_hash[2], final_hash[3], final_hash[4], final_hash[5], final_hash[6], final_hash[7]);
}