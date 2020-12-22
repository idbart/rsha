use std::fs;
use std::path::Path;

// print the use instructions
pub fn usage()
{
	println!("Use instructions:");
	println!("\trsha <filename/path>");
}

// open a file and 
pub fn get_file_byte_array(file: &String) -> Vec<u8>
{
	// try to open file
	let data = match fs::read(Path::new(file)) {
		Ok(data) => data,
		Err(e) => {
			// if the file could nto be read for some reason print it and panic for now
			println!("ERROR: {}", e.to_string());
			panic!();
		}
	};
	
	// return ownership the data
	data
}