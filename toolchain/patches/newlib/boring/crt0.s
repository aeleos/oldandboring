BITS 32

global _start
_start:
	pop    eax
	extern pre_main
	extern main
	push main
	call   pre_main
