/* This code generates a cryptographic hash of a specified file for verification
*
* Author: Josh McIntyre
*/
use std::env;
use std::io::{Error, ErrorKind, Read};
use std::fs::File;
use sha2::{Sha256, Digest};

/* The main entry point for the program */
fn main() -> Result<(), Error>
{
	// Get command line arguments
	let args: Vec<String> = env::args().collect();
	
	if args.len() != 2
	{
		println!("Usage ./hashhelper [filename]");
		return Err(Error::new(ErrorKind::Other, "filename required"));
	}
	let filename: &String = &args[1];
	
	// Get the hash of the file in hex format
	let digest = hash_file(filename.to_string());
	
	// Output the final hash to stdout
	println!("{digest}");
	
	return Ok(());
}

/* Hash the contents of the file and return a formatted digest */
fn hash_file(filename: String) -> String
{
	// Open the file 
	let hash_file = File::open(filename);
	let mut hasher = Sha256::new();

	for byte_result in hash_file.expect("Error reading file").bytes()
	{
		let byte = byte_result.unwrap();
		hasher.update(&[byte]);
	}

	// Finalize and format the hash
	let digest_raw = hasher.finalize();
	let digest = format!("{:#02x}", digest_raw);
	
	return digest;
}