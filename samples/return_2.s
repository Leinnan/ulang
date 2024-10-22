	.globl _main
_main:
	push	%rbp
	mov	%rsp, %rbp
	subq $8, %rsp
	movl	$2, -4(%rbp)
	movl	-4(%rbp), %r11d
	imull	$3, %r11d
	movl	%r11d, -4(%rbp)
	movl	$5, -8(%rbp)
	movl	-4(%rbp), %r10d
	addl	%r10d, -8(%rbp)
	movl	-8(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
