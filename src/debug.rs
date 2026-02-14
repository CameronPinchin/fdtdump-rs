/* @author Cameron Pinchin 
 * @date Feb 11 2026 
 * @brief This handles debug functionality when '-d, --debug' is passed. 
 * May include helper methods.
 */

use fdt::Fdt;
use std::env; 

use crate::error;

fn grab_file_directory() {
   match env::current_dir() {
      Ok(path) => println!("current working path: {}", path.display()),
      Err(e) => eprintln!("Error getting working path: {}", e),

   }
}

/* @brief Input a file path string to parse the dtb file  
 *
 * @param str&: file is a file path input leading to a dtb file
 *
 * @return No return, on success prints out DTB file contents to console.
 */
pub fn dump_blob( file: &str ) {
   grab_file_directory();
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

   println!("\nFind all nodes for '/':");
   for node in fdt.find_all_nodes("/") {
      println!("{}", node.name);
   }

   println!("Find all nodes for '/soc/virtio':");
   for node in fdt.find_all_nodes("/soc/virtio") {
      println!("{}", node.name);
   }

 }