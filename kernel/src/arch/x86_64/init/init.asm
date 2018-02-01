global long_mode_start

extern main

section .init

bits 32
init_fpu: ;enable fpu
    mov eax, cr0
    and al, 11110011b
    or al, 00100010b
    mov cr0, eax
    mov eax, cr4
    or eax, 0x200
    mov cr4, eax
    fninit
    ret

init_sse: ;enable sse
    mov eax, cr4
    or ax, 0000011000000000b
    mov cr4, eax
    ret

bits 64
long_mode_start: ;first 64-bit code to be executed

    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    mov rsp, 0xfffffe8000000000 ;make the stack pointer point to the virtual stack top

    call init_fpu
    call init_sse

    mov rax, main
    jmp rax

    ;in case the rust code ever returns, halt the CPU indefinitely
.endlessLoop:
    hlt
    jmp .endlessLoop
