/* @author Cameron Pinchin
 * 
 * @date Feb 06, 2026
 *
 * @brief Variant of fdtdump but built in Rust 
 */

use std::env; // returns an iterator of the command line arguments  

mod error;
mod parser;

const TEST_PATH_EXTENSION: &str = "testfile.dtb";

/* === TO - DO ===
 *  - 2-argument case: 
 *       - arg_1: match one of options defined in parser
 *       - arg_2: pass dtb file verification checks, be a valid dtb file
 *  - filler 'show_error()' for the moment 
 */
fn main() {
	let args: Vec<String> = env::args().collect();
	let args_count = env::args().skip(1).count();
    //	dbg!(args); 
	match args_count {
		2 => parser::two_argument_parse( &args[1], &args[2] ),
		1 => parser::parse_args(&args[1]),		
		0 => parser::show_help(),
		_ => println!("err, no input on args_count(?)"),
	}
}

