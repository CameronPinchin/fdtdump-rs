/* @author Cameron Pinchin 
 * @date Feb 11 2026 
 * @brief This handles debug functionality when '-d, --debug' is passed. 
 * May include helper methods.
 */

use::fdt;

mod error;

 fn dump_blob( &str file ) {
    let fdt = Fdt::new(error::TEST_PATH_EXTENSION).unwrap();
    println!("This is a device tree representation of a {}:", fdt.root().model());
    println!("...which is compatible with atleast: {}", fdt.root().compatible().first());
    println!("...which has {} CPUs", fdt.cpus().count());
 }