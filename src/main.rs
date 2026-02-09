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

fn two_argument_parse( arg_0: &str, arg_1: &str ) -> &str  {
	let mut arg = "";
	if parser::is_valid_arg(&args[1]) && parser::is_valid_arg(&args[2]) {
		arg = &args[1] + &args[2];
	} else {
		parser::show_error();
		return None;
	}

	return arg;
}

/* === TO - DO ===
 *  - 2-argument case: 
 *       - arg_1: match one of options defined in parser
 *       - arg_2: pass dtb file verification checks, be a valid dtb file
 *  - filler 'show_error()' for the moment 
 */
fn main() {
	let args: Vec<String> = env::args().collect();
	let args_count = env::args().skip(1).count();
    //	dbg!(args); // quick print of args
    	
	match args_count {
		2 => two_argument_parse( &args[1], &args[2] ),
		1 => parser::parse_args(&args[1]),		
		0 => parser::show_help(),
		_ => println!("err, no input on args_count(?)"),
	}

	error::ValidFileCheck(TEST_PATH_EXTENSION);

}

