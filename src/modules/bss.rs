use crate::modules::data::{DataBlock, DataConvert};
pub struct Bss {
	pub BssString: String,
}

impl Bss {
	pub fn init() -> Bss {
		Bss{BssString: String::from("\n.section .bss\n")}
	}
	pub fn parse_from_string(contents: &str,datablock: &mut DataBlock) -> Bss {
                let mut startdata: Bss = Bss::init();
                for line in contents.lines() {
                        let assign_string: Vec<&str> = line.split("-").collect();
                        if assign_string.len() == 3 {
                        let var_name: &str = assign_string[2][1..].trim();
                        let datafn: &str = assign_string[1].trim();
                        if let Ok(j) = datafn.parse::<i32>() {
                                startdata.BssString += &format!("{}: .space {:?}\n{}_end:\n", var_name, j, var_name);
				datablock.DataLookup.insert(String::from(var_name), ".asciz".to_owned());
                        }
                        else {
                                let datastring: &str = match DataConvert::typesend(datafn) {
                                                        ".byte" => {&format!("{}: .space 1\n{}_end:\n", var_name, var_name)},
                                                        ".long" => {&format!("{}: .space 4\n{}_end:\n", var_name, var_name)},
                                                        ".quad" => {&format!(".align 8\n{}: .space 8\n{}_end:\n", var_name, var_name)},
                                                        ".float" => {&format!(".align 8\n{}: .space 8\n{}_end:\n", var_name, var_name)},
                                                        ".double" => {&format!(".align 8\n{}: .space 16\n{}_end:\n", var_name, var_name)},
                                                        _ => {panic!("The compiler does not recognise the type {}.\nIf you were trying to assign a string then this is done by its length: -5-> msg\n can later be assigned to 'Hello'.", datafn)},
                                                };
                                        startdata.BssString += datastring;
					datablock.DataLookup.insert(String::from(var_name), String::from(DataConvert::typesend(datafn)));
                                        };
                        }
			}
                startdata
        }
}

pub fn bss_from_cont(contents: String, datablock: &mut DataBlock) -> std::io::Result<Bss> {
        let initsec: Vec<&str> = contents.split("<Bss>").collect();
        match initsec.len() {
                1 => {Ok(Bss::init())},
                3 => {Ok(Bss::parse_from_string(initsec[1], datablock))}
                _ => {panic!("The compiler expects one initialising block and the wrapper must be closed afterwards\n E.g. \n<Bss> \n -int8-> x\n 30 -string-> j\n<Bss>");}

        }
}
