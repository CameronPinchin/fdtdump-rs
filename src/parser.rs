/* @author Cameron Pinchin 
 * @date Feb 07 2026 
 * @brief This handles parsing the arguments inputted to the binary. 
 * It is just a front end that will link other modules in a central area.
 *
 */

use crate::error;

const VERSION: &str = "0.01";

const _ARG_VERSION_LONG:  &str = "--version";
const _ARG_HELP_LONG:     &str = "--help";
const _ARG_SCAN_LONG: 	  &str = "--scan";
const _ARG_DEBUG_LONG:    &str = "--debug";

const _ARG_VERSION_SHORT: &str = "-V"; 
const _ARG_HELP_SHORT:    &str = "-h";
const _ARG_SCAN_SHORT:    &str = "-s";
const _ARG_DEBUG_SHORT:   &str = "-d";


/* @brief Show error log to user when too many arguments are inputted.  
 *
 * @return Returns () 
 */
pub fn show_error(){
	println!("ERROR: Invalid input, use one argument at a time.\n");
	show_help();
}

/* @brief Shows help message output when user inputs any of '-h, --help'
 * 
 * @return Returns () 
 */
pub fn show_help(){
    println!("***** fdtdump-rs is a low-level debugging tool, based on fdtdump, but ported for rust.");
	println!("***** If you want to decompile a dtb, you probably want to use dtc");
	println!("****      dtc -I dtb -O dts <filename>");
	println!("\nUsage:fdtdump [options..[in progres]] <filename>\n");
	println!("Options: -[dshV]");
	println!("  -d, --debug     Dump debug information while decoding the file [to-be-implemented]");
	println!("  -s, --scan      Scan for an embedded fdt in file");
	println!("  -h, --help      Print this help and exit");
	println!(   "-V, --version  Print version and exit");
}

/* @brief Shows scan message output when user inputs any of '-s, --scan'.
 * To be implemented further. 
 *
 * @return Returns () 
 */
pub fn show_scan(){
    println!("***** fdtdump-rs is a low-level debugging tool, based on fdtdump, but ported for rust.");
	println!("***** If you want to decompile a dtb, you probably want to use dtc");
	println!("****      dtc -I dtb -O dts <filename>");
    println!("\n\nTO-BE-IMPLEMENTED:\n");
    println!("  -s, --scan      Scan for an embedded fdt in file [in-progress...]");
}

/* @brief Show debug message output when user inputs any of '-d, --debug'.
 * To be implemented further. 
 *
 * @return Returns () 
 */
pub fn show_debug(){
    println!("***** fdtdump-rs is a low-level debugging tool, based on fdtdump, but ported for rust.");
	println!("***** If you want to decompile a dtb, you probably want to use dtc");
	println!("****      dtc -I dtb -O dts <filename>");
    println!("\n\nTO-BE-IMPLEMENTED:\n");
    println!("  -d, --debug     Dump debug information while decoding the file [to-be-implemented]");
}

/* @brief Show version message output when user inputs any of '-v, --version'.  
 *
 * @return Returns () 
 */
pub fn show_version(){
	println!("***** fdtdump-rs is a low-level debugging tool, based on fdtdump, but ported for rust.");
	println!("***** If you want to decompile a dtb, you probably want to use dtc");
	println!("****      dtc -I dtb -O dts <filename>");
   	println!("Version: {} ", VERSION);
}

/* @brief Parses the argument inpuuted by the user.  
 *
 * @param str&: arg is the user argument provided in main.rs.
 *
 * @return Returns cases when arg can be matched.  
 */
pub fn parse_args( arg: &str ) {	
	match arg {
		_ARG_VERSION_SHORT  => show_version(),
		_ARG_VERSION_LONG   => show_version(),
		_ARG_HELP_SHORT     => show_help(),
		_ARG_HELP_LONG      => show_help(),
        _ARG_DEBUG_SHORT    => show_debug(),
        _ARG_DEBUG_LONG     => show_debug(),
        _ARG_SCAN_SHORT     => show_scan(),
        _ARG_SCAN_LONG      => show_scan(),
		_ 				    => { if error::valid_file_check(arg) == true {
									println!("TRUE: valid-dbt-file");
								} else {
									println!("FALSE: valid-dbt-file");
								}
								},
	}
}

fn is_valid_arg( arg: &str ) -> bool {
	match arg {
		_ARG_VERSION_SHORT  => true,
		_ARG_VERSION_LONG   => true,
		_ARG_HELP_SHORT     => true,
		_ARG_HELP_LONG      => true,
        _ARG_DEBUG_SHORT    => true,
        _ARG_DEBUG_LONG     => true,
        _ARG_SCAN_SHORT     => true,
        _ARG_SCAN_LONG      => true,
		_					=> error::valid_file_check(arg),
	}
}

fn two_arg_parse( arg_0: &str, arg_1: &str ) -> String {
	if is_valid_arg(arg_0) && is_valid_arg(arg_1) {
		let arg = arg_0.to_string() + "," + arg_1;
        return arg;
	} else {
		show_error();
        String::new()
	}
}

pub fn two_argument_parse( arg_0: &str, arg_1: &str ) {
    let arg = two_arg_parse( arg_0, arg_1 );
    println!("argument: {}", arg);
}







