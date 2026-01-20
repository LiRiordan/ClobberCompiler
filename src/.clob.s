.intel_syntax no_prefix

.section .data
x: .byte 38
y: .asciz "run"
y_len = . - y 

.global _start
_start:
	mov rax, 60
	xor rdi, rdi
	syscall