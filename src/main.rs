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
	match args.nth(1) {
		Some(filename) => {

			// get the file bytes
			match cio::get_file_byte_array(&filename)
			{
				Ok(mut file_bytes) => {

					// and then calc the final hash
					let final_hash = hashing::get_hash(&mut file_bytes);

					// print the hash to screen
					println!("{}{}{}{}{}{}{}{}", hex::encode(final_hash[0].to_ne_bytes()), hex::encode(final_hash[1].to_ne_bytes()), hex::encode(final_hash[2].to_ne_bytes()), hex::encode(final_hash[3].to_ne_bytes()), hex::encode(final_hash[4].to_ne_bytes()), hex::encode(final_hash[5].to_ne_bytes()), hex::encode(final_hash[6].to_ne_bytes()), hex::encode(final_hash[7].to_ne_bytes()));
				},
				Err(e) => {
					println!("ERROR: {}", e);
				}
			}
		},
		// if the user did not input a file, panic for now (better soloution tbd)
		None => {
			println!("ERROR: cannot find file");
			cio::usage();
		}
	};
}