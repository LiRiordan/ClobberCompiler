pub fn arrowsplit(line: &'static str) -> Vec<&'static str> {
	let splitvec: Vec<&'static str> = line.split("-").map(|i| i.trim()).collect();
	splitvec
}

pub fn extfunc(line: &'static str) -> &'static str {
	arrowsplit(line)[1]
}
pub fn extargs(line: &'static str) -> (Vec<&'static str>, usize) {
	let args = arrowsplit(line)[0];
	let argvec: Vec<&'static str> = args.split(",").map(|i| i.trim()).collect();
	/* Need to compute arglen first as if we do (argvec, argvec.len()) then the first value uses up argvec which 
	doesn't implement copy so we compute arglen first. We could also have done (argvec.clone(), argvec.len())
	but this just creates an unnecessary copy */
	let arglen: usize = argvec.len();
	(argvec, arglen)
}

pub fn extout(line: &'static str) -> &'static str {
	match arrowsplit(line)[2].len() {
		0 => {""},
		_ => {&arrowsplit(line)[2][1..]}
	}
}
