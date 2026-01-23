mod modules;
use crate::modules::data::{DataBlock, data_from_cont};
use crate::modules::bss::{Bss, bss_from_cont};
use std::env;
use std::fs;
use std::io::Write;
use std::fs::File;

fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();
	
	/* Check to see if correct number of arguments were passed to the compiler */
	if args.len() != 2 
		{ eprintln!("Compiler expects one argument e.g. './clob foo.clob' but {} arguments passed\n", args.len()-1);
		  std::process::exit(1);
	}
	
	let filename: &str = &args[1];
	/* Check correct file extension */
	let extension = &filename[filename.len()-5..filename.len()]; 
	if extension != ".clob" 
		{ eprintln!("Compiler expects a clobber file e.g. 'foo.clob'");
		  std::process::exit(1);
		}
	let outname = &filename[..filename.len()-5];
	/* If previous checks were passed we can now read the file contents */
	let contents: String = fs::read_to_string(filename)?;
	let mut initdata: DataBlock = data_from_cont(contents.clone()).unwrap();
	let datastring = initdata.DataString.clone();

	let bssdata: Bss = bss_from_cont(contents.clone(), &mut initdata).unwrap();
	let bssstring = bssdata.BssString;
 
	/* We should do this at the end to avoid holding up more memory. */
	/* Also pass extern functions to prestring */ 
	let mut prestring: String = ".intel_syntax noprefix\n\n".to_owned();
	let teststring: &str = "\n.global _start\n_start:\n";     
	let closerstring: &str = "	mov rax, 60\n	xor rdi, rdi\n	syscall\n";

	prestring += &datastring;
	prestring += &bssstring;
	prestring += teststring;
	let assembly = prestring + closerstring;

	let assemblyname: &str = &format!("{}.s", outname);
	let mut file = File::create(assemblyname)?;
	file.write_all(assembly.as_bytes())?;
	
	for (key, value) in &initdata.DataLookup {
		println!("{}, {}", key, value);
	}	
	file.flush()?;

	Ok(())
}
