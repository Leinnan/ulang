	.globl _main
._main
	pushq	%rbp
	movq	%rsp, %rbp
	subq	12, %rsp
	movl	5, -4(%rbp)
	imull	7, -4(%rbp)
	movl	2, -8(%rbp)
	addl	-4(%rbp), -8(%rbp)
	movl	-8(%rbp), %r10d
	movl	%r10d, -12(%rbp)
	subl	3, -12(%rbp)
	movl	-12(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
