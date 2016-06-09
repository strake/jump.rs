.intel_syntax noprefix

.text

.global __mkjump
__mkjump:
	pop qword ptr [rdi]  /* return location */
	mov [rdi+0x08], rsp
	mov [rdi+0x10], rbp
	mov [rdi+0x18], rbx
	mov [rdi+0x20], r12
	mov [rdi+0x28], r13
	mov [rdi+0x30], r14
	mov [rdi+0x38], r15
	jmp [rdi]

.global __jump
__jump:
	mov rsp, [rdi+0x08]
	mov rbp, [rdi+0x10]
	mov rbx, [rdi+0x18]
	mov r12, [rdi+0x20]
	mov r13, [rdi+0x28]
	mov r14, [rdi+0x30]
	mov r15, [rdi+0x38]
	jmp [rdi]
