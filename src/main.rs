/* @author Cameron Pinchin
 * 
 * @date Feb 06, 2026
 *
 * @brief Variant of fdtdump but built in Rust 
 */

use std::env; // returns an iterator of the command line arguments  

const DTB_EXTENSION: &str =  ".dtb";
const TEST_PATH_EXTENSION: &str = "testfile.dtb";

/* @brief Input a file path string and determines if it  
 * is a valid .dtb file.
 *
 * @param str&: file_path is a file path input.
 *
 * @return Returns a boolean depending on if the file 
 * ends in .dtb or not.
 */
fn is_dtb_file( file_path: &str ) -> bool {
	
	let target = '.';
    let mut last_dot_pos = 0;

	match file_path.rfind(target) {
		Some(index) => {
			last_dot_pos = index;
		},
		None => {
			return false;
		}
	}
		
	let file_path_extension = safe_slice(file_path, last_dot_pos);
    
	if file_path_extension == DTB_EXTENSION {
		println!("TRUE");
	} else {
		println!("FALSE: {}, {}", file_path_extension, DTB_EXTENSION);
	}

	return file_path_extension == DTB_EXTENSION; 	
}


/* @brief Input a file_path string and nth index 
 * to splice the first n characters off a string.
 * 
 * @params str&: file_path is a file path input.
 * usize: n is an index input.
 *
 * @return str& A reference to the modified string. 
 */
fn safe_slice( file_path: &str, n: usize ) -> &str {
	match file_path.char_indices().nth(n) {
		Some((byte_index, _)) => {
			&file_path[byte_index..]
		},
		None => ""
	} 
}


fn main() {
	let args: Vec<String> = env::args().collect();
	dbg!(args); // quick print of args
	
	is_dtb_file(TEST_PATH_EXTENSION);
}

