pub struct DataTypes {
}

impl DataTypes {
	fn typesend(datafn:&str) -> &str {
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
	pub fn typelist() {
		println!("Datatypes:\n8 bit integers: int8, 32 bit integers: int32, 64 bit integers: int64, 32 bit floats: float, 64 bit floats: float64, Strings: string");
	}
}

pub struct DataBlock {
	DataString: String
}

impl DataBlock {
	fn init() -> DataBlock {
		DataBlock{ DataString: ".section .data\n".to_owned()}
	}
	fn parse_from_string(contents: &str) -> DataBlock {
		let mut startdata: DataBlock = DataBlock::init();
		for line in contents.lines() {
			let assign_string: Vec<&str> = line.split("-").collect();
			if assign_string.len() == 3 {
			let val: &str =  assign_string[0].trim();		
			let datafn: &str = DataTypes::typesend(assign_string[1].trim());
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
									Ok(par_val) => &format!("{}: {} {:?}\n", var_name, datafn, par_val),
									Err(_) => panic!("Error has occured converting {} to type i64", val),
									}
							},
							".float" => {match val.parse::<f32>() {
									Ok(par_val) => &format!("{}: {} {:?}\n", var_name, datafn, par_val),
									Err(_) => panic!("Error has occured converting {} to type f32", val),
									}
							},
							".double" => {match val.parse::<f64>() {
									Ok(par_val) => &format!("{}: {} {:?}\n", var_name, datafn, par_val),
									Err(_) => panic!("Error has occured converting {} to type f64", val),
									}
							},
							".asciz" => {&format!("{}: {} {}\n{}_len = . - {} \n", var_name, datafn, val, var_name, var_name)}
							_ => {panic!("The compiler does not recognise the type {}", datafn)},
						};
					startdata.DataString += datastring;
					};
			}
		startdata
	}
}

pub fn data_from_cont(contents: String) -> std::io::Result<String> {
	let presec: Vec<&str> = contents.split("<Preload>").collect();
	match presec.len() {
		1 => {Ok("".to_owned())},
		3 => {Ok(DataBlock::parse_from_string(presec[1]).DataString)}
		_ => {panic!("The compiler expects one preload block and the preload wrapper must be closed afterwards\n E.g. \n<Preload> \n3 -int-> x\nTrue -bool-> y\n<Preload>");}
	}	
}
