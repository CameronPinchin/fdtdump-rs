/* @author Cameron Pinchin
 * 
 * @date Feb 06, 2026
 *
 * @brief Variant of fdtdump but built in Rust 
 */

use std::env; // returns an iterator of the command line arguments  

mod err_handling;

const TEST_PATH_EXTENSION: &str = "testfile.dtb";

fn main() {
	let args: Vec<String> = env::args().collect();
	dbg!(args); // quick print of args
	
	err_handling::ValidFileCheck(TEST_PATH_EXTENSION);
}

