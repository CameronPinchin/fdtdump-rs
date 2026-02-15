/* @author Cameron Pinchin
 * 
 * @date Feb 06, 2026
 *
 * @brief Variant of fdtdump but built in Rust 
 */

use std::env; 

mod error;
mod parser;
mod debug;

const TEST_PATH_EXTENSION: &str = "testfile.dtb";

fn main() {
	let args: Vec<String> = env::args().collect();
	let args_count = env::args().skip(1).count();
	match args_count {
		2 => parser::two_argument_parse( &args[1], &args[2] ),
		1 => parser::parse_args(&args[1]),		
		0 => parser::show_help(),
		_ => println!("err, no input on args_count(?)"),
	}
}

