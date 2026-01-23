use std::collections::HashMap;

#[derive(Debug)]
pub enum DataTypes {
	Int8(i8),
	Int32(i32),
	Int64(i64),
	Float(f32),
	Float64(f64),
	Str(String),
	Var(String)
}

impl DataTypes {
	pub fn varorval(data: &str) -> DataTypes {
		if let Ok(x) = data.parse::<i8>() {
			return DataTypes::Int8(x);
		}
		if let Ok(x) = data.parse::<i32>() {
			return DataTypes::Int32(x);
		}
		if let Ok(x) = data.parse::<i64>() {
			return DataTypes::Int64(x);
		}
		if let Ok(x) = data.parse::<f32>() {
			return DataTypes::Float(x);
		}
		if let Ok(x) = data.parse::<f64>() {
			return DataTypes::Float64(x);
		}
		if (data.starts_with("\'") && data.ends_with("\'") && data.len() > 1) ||
			(data.starts_with("\"") && data.ends_with("\"") && data.len() > 1) {
			return DataTypes::Str(String::from(&data[1..data.len()-1]));
		}
		DataTypes::Var(String::from(data))
	}
}



pub struct DataConvert {
}

impl DataConvert {
	pub fn typesend(datafn:&str) -> &str {
		match datafn {
			"int8" => {".byte"},
			"int32" => {".long"},
			"int64" => {".quad"},
			"float" => {".float"},
			"float64" => {".double"},
			"string" => {".asciz"},
			_ => {panic!("The assignment {} is not a valid datatype\n", datafn);}
		}
	}	
}

pub struct DataBlock {
	pub DataString: String,
	pub DataLookup: HashMap<String, String>
}

impl DataBlock {
	fn init() -> DataBlock {
		let datahash: HashMap<String, String> = HashMap::new();
		DataBlock{ DataString: ".section .data\nintprint:    .asciz \"%i\"\nintlen=    . - intprint\nflprint:    .asciz \"%f\"\nfllen=     . - flprint\n".to_owned(), DataLookup: datahash}
	}
	fn parse_from_string(contents: &str) -> DataBlock {
		let mut startdata: DataBlock = DataBlock::init();
		for line in contents.lines() {
			let assign_string: Vec<&str> = line.split("-").collect();
			if assign_string.len() == 3 {
			let val: &str =  assign_string[0].trim();		
			let datafn: &str = DataConvert::typesend(assign_string[1].trim());
			let var_name: &str = assign_string[2][1..].trim();
			let datastring: &str = match datafn {
							".byte" => {match val.parse::<i8>() {
									Ok(par_val) => &format!("{}: {} {:?}\n", var_name, datafn, par_val),
									Err(_) => panic!("Error has occured converting {} to type i8", val),
									}
							},
							".long" => {match val.parse::<i32>() {
									Ok(par_val) => &format!("{}: {} {:?}\n", var_name, datafn, par_val),
									Err(_) => panic!("Error has occured converting {} to type i32", val),
									}
							},
							".quad" => {match val.parse::<i64>() {
									Ok(par_val) => &format!(".align 8\n{}: {} {:?}\n", var_name, datafn, par_val),
									Err(_) => panic!("Error has occured converting {} to type i64", val),
									}
							},
							".float" => {match val.parse::<f32>() {
									Ok(par_val) => &format!("{}: {} {:?}\n", var_name, datafn, par_val),
									Err(_) => panic!("Error has occured converting {} to type f32", val),
									}
							},
							".double" => {match val.parse::<f64>() {
									Ok(par_val) => &format!(".align8\n{}: {} {:?}\n", var_name, datafn, par_val),
									Err(_) => panic!("Error has occured converting {} to type f64", val),
									}
							},
							".asciz" => {&format!("{}: {} {}\n{}_len = . - {}\n", var_name, datafn, val, var_name, var_name)}
							_ => {panic!("The compiler does not recognise the type {}", datafn)},
						};
					startdata.DataString += datastring;
					startdata.DataLookup.insert(String::from(var_name), String::from(datafn));
					};
			}
		startdata
	}
	pub fn varquery(&self, val: &str) -> &str {
                match DataTypes::varorval(val) {
                        DataTypes::Int8(_) => {".byte"},
                        DataTypes::Int32(_) => {".long"},
                        DataTypes::Int64(_) => {".quad"},
                        DataTypes::Float(_) => {".float"},
                        DataTypes::Float64(_) => {".double"},
                        DataTypes::Str(_) => {".asciz"},
                        DataTypes::Var(_) => {match self.DataLookup.get(val) {
						Some(x) => {&x},
						None => {panic!("No variable named {}", val); ""}
						}
						}
		} 
        }
}

pub fn data_from_cont(contents: String) -> std::io::Result<DataBlock> {
	let presec: Vec<&str> = contents.split("<Data>").collect();
	match presec.len() {
		1 => {Ok(DataBlock::init())},
		3 => {Ok(DataBlock::parse_from_string(presec[1]))}
		_ => {panic!("The compiler expects one data block and the wrapper must be closed afterwards\n E.g. \n<Data> \n3 -int-> x\nTrue -bool-> y\n<Data>");}
	}	
}

