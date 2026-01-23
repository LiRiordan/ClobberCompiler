use crate::modules::standard::{StandardLib};
use crate::modules::text::{Text};

pub fn lib_from_cont(contents: String) -> std::io::Result<String> {
        let libsec: Vec<&str> = contents.split("<Lib>").collect();
        match libsec.len() {
                1 => {Ok(LibString::init())},
                3 => {Ok(LibString::parse_from_string(libsec[1]))}
                _ => {panic!("The compiler expects one lib block and the wrapper must be closed afterwards\n E.g. \n<Lib>printf\n<Lib>")}
        }
}
pub struct Lib {
	LibString: String
}

impl Lib {
	pub fn init() -> Lib {
		Lib{LibString: "".to_owned()}
	}
	/* We will also need to update .text to add .extern foo */
	pub fn parse_from_string(contents: &str, textblock: &mut Text) -> Lib {
		let mut lib: Lib = Lib::init();
		let libs: Vec<&str> = contents.split("\n").map(|i| i.trim()).collect();
		if "printf" in libs {textblock.TextString += ".extern printf\n"; lib.LibString += &StandardLib::printcall();}
	lib
}
