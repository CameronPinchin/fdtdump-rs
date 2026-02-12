/* @author Cameron Pinchin 
 * @date Feb 11 2026 
 * @brief This handles debug functionality when '-d, --debug' is passed. 
 * May include helper methods.
 */

use fdt::Fdt;

use crate::error;

pub fn dump_blob( file: &str ) {
   let test_file = error::TEST_PATH_EXTENSION.to_string();
   static MY_FDT: &[u8] = include_bytes!(test_file);

    let fdt = Fdt::new(MY_FDT).unwrap();
    println!("This is a device tree representation of a {}:", fdt.root().model());
    println!("...which is compatible with atleast: {}", fdt.root().compatible().first());
    println!("...which has {} CPUs", fdt.cpus().count());
 }