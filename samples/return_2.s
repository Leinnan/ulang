	.globl _main
._main
	pushq	%rbp
	movq	%rsp, %rbp
	subq	12, %rsp
	movl	0, -4(%rbp)
	notl	-4(%rbp)
	movl	-4(%rbp), %r10d
	movl	%r10d, -8(%rbp)
	notl	-8(%rbp)
	movl	-8(%rbp), %r10d
	movl	%r10d, -12(%rbp)
	negl	-12(%rbp)
	movl	-12(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
