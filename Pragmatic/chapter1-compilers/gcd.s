	.file	"gcd.c"
	.section	.rodata
.LC0:
	.string	"%d"
	.text
	.globl	getint
	.type	getint, @function
getint:
.LFB0:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$16, %rsp
	movq	%fs:40, %rax
	movq	%rax, -8(%rbp)
	xorl	%eax, %eax
	leaq	-12(%rbp), %rax
	movq	%rax, %rsi
	movl	$.LC0, %edi
	movl	$0, %eax
	call	__isoc99_scanf
	movl	-12(%rbp), %eax
	movq	-8(%rbp), %rdx
	xorq	%fs:40, %rdx
	je	.L3
	call	__stack_chk_fail
.L3:
	leave
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE0:
	.size	getint, .-getint
	.globl	putint
	.type	putint, @function
putint:
.LFB1:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$16, %rsp
	movl	%edi, -4(%rbp)
	movl	-4(%rbp), %eax
	movl	%eax, %esi
	movl	$.LC0, %edi
	movl	$0, %eax
	call	printf
	nop
	leave
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE1:
	.size	putint, .-putint
	.globl	main
	.type	main, @function
main:
.LFB2:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$16, %rsp
	movl	$0, %eax
	call	getint
	movl	%eax, -8(%rbp)
	movl	$0, %eax
	call	getint
	movl	%eax, -4(%rbp)
	jmp	.L6
.L8:
	movl	-8(%rbp), %eax
	cmpl	-4(%rbp), %eax
	jle	.L7
	movl	-4(%rbp), %eax
	subl	%eax, -8(%rbp)
	jmp	.L6
.L7:
	movl	-8(%rbp), %eax
	subl	%eax, -4(%rbp)
.L6:
	movl	-8(%rbp), %eax
	cmpl	-4(%rbp), %eax
	jne	.L8
	movl	-8(%rbp), %eax
	movl	%eax, %edi
	call	putint
	movl	$0, %eax
	leave
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE2:
	.size	main, .-main
	.ident	"GCC: (Ubuntu 5.4.0-6ubuntu1~16.04.9) 5.4.0 20160609"
	.section	.note.GNU-stack,"",@progbits
