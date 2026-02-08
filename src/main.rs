/* @author Cameron Pinchin
 * 
 * @date Feb 06, 2026
 *
 * @brief Variant of fdtdump but built in Rust 
 */

use std::env; // returns an iterator of the command line arguments  

mod err_handling;
mod parser;

const TEST_PATH_EXTENSION: &str = "testfile.dtb";

fn main() {
	let args: Vec<String> = env::args().collect();
	let args_count = env::args().skip(1).count();
    //	dbg!(args); // quick print of args
	
	match args_count {
		1 => parser::parse_args(&args[1]),		
		0 => parser::show_help(),
		_ => println!("err, no input on args_count(?)"),
	}

	err_handling::ValidFileCheck(TEST_PATH_EXTENSION);

}

