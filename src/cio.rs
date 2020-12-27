use std::fs;
use std::path::Path;

// print the use instructions
pub fn usage()
{
	println!("Use instructions:");
	println!("\trsha <filename/path>");
}

// open a file and 
pub fn get_file_byte_array(file: &String) -> Result<Vec<u8>, String>
{
	// try to open file
	let data = match fs::read(Path::new(file)) {
		Ok(data) => data,
		Err(e) => {
			// if the file could nto be read for some reason return the error
			return Err(e.to_string())
		}
	};
	
	// return ownership the data
	Ok(data)
}