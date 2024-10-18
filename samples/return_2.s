	.globl _main
_main:
	push	%rbp
	mov	%rsp, %rbp
	subq $4, %rsp
	movl	$5, -4(%rbp)
	addl	$2, -4(%rbp)
	movl	-4(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
