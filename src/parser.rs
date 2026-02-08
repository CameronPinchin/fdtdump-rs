/* @author Cameron Pinchin 
 * @date Feb 07 2026 
 * @brief This handles parsing the arguments inputted to the binary. 
 * It is just a front end that will link other modules in a central area.
 *
 */

const VERSION: &str = "0.10";

const _ARG_VERSION_LONG:  &str = "--version";
const _ARG_HELP_LONG:     &str = "--help";
const _ARG_SCAN_LONG: 	  &str = "--scan";
const _ARG_DEBUG_LONG:    &str = "--debug";

const _ARG_VERSION_SHORT: &str = "-V"; 
const _ARG_HELP_SHORT:    &str = "-h";
const _ARG_SCAN_SHORT:    &str = "-s";
const _ARG_DEBUG_SHORT:   &str = "-d";

pub fn show_help(){
    println!("***** fdtdump-rs is a low-ldevel debugging tool, based on fdtdump, but ported for rust.");
	println!("***** If you want to decompile a dtb, you probably want to use dtc");
	println!("****      dtc -I dtb -O dts <filename>");
	println!("\nUsage:fdtdump [options..[in progres]] <filename>\n");
	println!("Options: -[dshV]");
	println!("  -d, --debug     Dump debug information while decoding the file [to-be-implemented]");
	println!("  -s, --scan      Scan for an embedded fdt in file");
	println!("  -h, --help      Print this help and exit");
	println!(   "-V, --version  Print version and exit");
}


pub fn show_version(){
	println!("***** fdtdump-rs is a low-ldevel debugging tool, based on fdtdump, but ported for rust.");
	println!("***** If you want to decompile a dtb, you probably want to use dtc");
	println!("****      dtc -I dtb -O dts <filename>");
	
   	println!("Version: {} ", VERSION);
}

pub fn parse_args( arg: &str ) {	
	match arg {
		_ARG_VERSION_SHORT => show_version(),
		_ARG_VERSION_LONG  => show_version(),
		_ARG_HELP_SHORT    => show_help(),
		_ARG_HELP_LONG     => show_help(),
		_ 				   => {
								println!("Error, invalid input. ");
								show_help();
								},
	}

}









