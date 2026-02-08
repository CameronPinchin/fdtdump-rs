/* @author Cameron Pinchin 
 * @date Feb 07 2026 
 * @brief This handles parsing the arguments inputted to the binary. 
 * It is just a front end that will link other modules in a central area.
 *
 */


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


