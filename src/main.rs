/* @author Cameron Pinchin
 * 
 * @date Feb 06, 2026
 *
 * @brief Variant of fdtdump but built in Rust 
 */

use std::env; // returns an iterator of the command line arguments  

const DBT_EXTENSION: ".dbt";

/* @brief Input a file path string and determines if it  
 * is a valid .dtb file.
 *
 * @param str&: file_path is a file path input.
 *
 * @return Returns a boolean depending on if the file 
 * ends in .dtb or not.
 */
fn is_dtb_file( str&: file_path ) -> bool {
	let target = '.';
    let last_dot_pos = file_path.rfind(&target);
	
    if last_dot_pos == None { return false; }
		
	let file_path_extension = safe_slice(file_path, last_dot_pos - 1); // incude the '.'

	return file_path_extension == DBT_EXTENSION; 	

}


/* @brief Input a file_path string and nth index 
 * to splice the first n characters off a string.
 * 
 * @params str&: file_path is a file path input.
 * usize: n is an index input.
 *
 * @return str& A reference to the modified string. 
 */
fn safe_slice( str&: file_path, usize: n ) -> &str {
	match file_path.char_indices().nth(n) {
		Some((byte_index, _)) => {
			&file_path[byte_index..]
		},
		None => ""
	} 
}


fn main() {
	let args: Vec<String> = env::args().collect();
	dbg!(args) // quick print of args 

}

