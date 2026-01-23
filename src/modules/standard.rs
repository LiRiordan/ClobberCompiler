use crate::modules::data::{DataBlock};

pub struct StandardLib {
}

impl StandardLib {
	/* Print for strings. Can only be printed from memory */
	pub fn prints(val: &str) -> String {
		format!("    mov rax, 1\n    mov rdi, 1\n    lea rsi, [rip + {}]\n    mov rdx, {}_len\n", val, val)
	}
	/* Print for ints. Can be printed by input or memory */
	pub fn printi(val: &str) -> String {	
		match val.parse::<i32>() {
			Ok(x) => {format!("    lea rdi, [rip+intprint]\n    mov rsi, {}\n    xor rax, rax\n    call printcall\n", val)},
			_ => {format!("    lea rdi, [rip+intprint]\n    mov rsi, [rip+{}]\n    xor rax, rax\n    call printcall\n", val)}
		}
	}
	/* Print for floats. These also have to be done from memory */
	pub fn printfl(val: &str) -> String {
		format!("    lea rdi, [rip+flprint]\n    movsd xmm0, [rip+{}]\n    mov rax, 1\n    call printcall\n", val)
	}
	/* Now do a global print function which splits the cases */ 
	pub fn printgl(val:&str, datablock: &DataBlock) -> String {
		match datablock.varquery(val) {
			".byte" => {Self::printi(val)},
			".long" => {Self::printi(val)},
			".quad" => {Self::printi(val)},
			".float" => {Self::printfl(val)},
			".double" => {Self::printfl(val)},
			".asciz" => {Self::prints(val)},
			_ => {panic!("Cannot print unknown type");
				String::from("")}
		}
	}
	pub fn assignvar(buf: &str, datablock: &DataBlock) -> String {
		match datablock.varquery(buf) {
			".float" => {format!("    mov [rip + {}], xmm0\n", buf)},
			".double" => {format!("    mov [rip + {}], xmm0\n", buf)},
			_ => format!("    push rax\n    lea rsi, [rsp]\n    lea rdi, [rip + {}]\n    mov rcx, {}_end - {}\n    rep movsb\n    pop rax\n", buf, buf, buf)
	}
	}
	/* Unlike the others this is a libfunction rather than an inlined function */
	pub fn printcall() -> String {
		String::from("printcall:\n    push rbp\n    mov rbp, rsp    call printf\n    xor rax, rax\n    leave\n    ret\n")
	}
	pub fn stdin(buf: &str) -> String {
		format!("    mov rax, 0\n    mov rdi, 0\n    lea rsi, [rip+{}]\n    mov rdx, {}_end - {}\n    syscall\n", buf, buf, buf)
	}
	/* stdin will be different for ints and floats */
//	pub fn stdini(buf: &str) -> String {
	/* Don't forget to follow this by an assignvar for the file fd. Use {filename}_fd as name */
	pub fn createfile(filename: &str) -> String {
		format!("    mov rax, 2\n    mov rdx, 0644\n    lea rdi, [rip+{}]\n    mov rsi, 1 | 64\n    syscall\n", filename)
	}    
	pub fn writefile(msg: &str, filename: &str) -> String {
		format!("    mov rax, 1\n    mov rdi, [rip+{}_fd]\n    lea rsi, [rip+{}]\n    mov rdx, {}_end - {}\n    syscall\n", filename, msg, msg, msg)
	}
	pub fn sysclose() -> String {
		String::from("    mov rax, 3\n    syscall\n")
	}
//	pub fn add(args: (Vec<&str>, usize)) -> String {
//		match args {
//			/* Probably split into several cases for type and no of args.
//			   use xmm if float and rax, rcx if not. Do the same for mul */
}

