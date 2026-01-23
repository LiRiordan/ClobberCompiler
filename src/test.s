.intel_syntax noprefix

.section .data
intprint:    .asciz "%i"
intlen=    . - intprint
flprint:    .asciz "%f"
fllen=     . - flprint
x: .byte 38
y: .asciz "run"
y_len = . - y

.section .bss
j: .space 1
j_end:
h: .space 30
h_end:
.align 8
s: .space 8
s_end:

.global _start
_start:
	mov rax, 60
	xor rdi, rdi
	syscall
