use crate::modules::data::DataBlock;
use crate::modules::standard;
use crate::modules::format::{extargs, extfunc, extout};


pub struct Text {
	pub TextString: String
}

impl Text {
	pub fn init() -> Text {
		Text{TextString: String::from("")}
	}
	pub fn parse_from_string(contents: &str, datablock: &mut DataBlock) -> String {
		let lines: Vec<&str> = contents.split("\n").collect();
		for line in lines {
				
		}	
	}	
}

pub fn text_from_cont(contents: String, datablock: &mut DataBlock) -> std::io::Result<Text> {
        let textsec: Vec<&str> = contents.split("<Text>").collect();
        match textsec.len() {
                1 => {Ok(Text::init())},
                3 => {Ok(Text::parse_from_string(textsec[1], datablock))}
                _ => {panic!("The compiler expects one text block and the wrapper must be closed afterwards\n E.g. '\n<Text>\n x-print->\n3-assign->j\n<Text>'");}
        }
}
