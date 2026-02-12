/* @author Cameron Pinchin 
 * @date Feb 11 2026 
 * @brief This handles debug functionality when '-d, --debug' is passed. 
 * May include helper methods.
 */

use fdt::Fdt;
use std::env; 

use crate::error;

pub fn dump_blob( file: &str ) {

   let test_file = error::TEST_PATH_EXTENSION.to_string();
   static MY_FDT: &[u8] = include_bytes!( concat!(env!("CARGO_MANIFEST_DIR"), "/_test/testfile.dtb") );

    let fdt = Fdt::new(MY_FDT).unwrap();
    println!("This is a device tree representation of a {}:", fdt.root().model());
    println!("...which is compatible with atleast: {}", fdt.root().compatible().first());
    println!("...which has {} CPUs", fdt.cpus().count());
    println!(
      "...and has atleast one memory location at: {:#X}\n",
      fdt.memory().regions().next().unwrap().starting_address as usize
   );

   let chosen = fdt.chosen();
   if let Some(bootargs) = chosen.bootargs() {
      println!("The bootargs are: {:?}", bootargs);
   }

   if let Some(stdout) = chosen.stdout() {
      println!("It would write stdout to: {}", stdout.name);
   }

   let soc = fdt.find_node("/soc");
   println!("Does it have a '/soc' node? {}", if soc.is_some() { "yes" } else { "no" });
   if let Some(soc) = soc {
      println!("...and it has the following children:");
      for child in soc.children() {
         println!("  {}", child.name);
      }
   }

 }