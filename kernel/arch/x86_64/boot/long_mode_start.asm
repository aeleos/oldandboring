global long_mode_start

section .text

bits 16

initialize:
.fpu: ;enable fpu
    mov eax, cr0
    and al, 11110011b
    or al, 00100010b
    mov cr0, eax
    mov eax, cr4
    or eax, 0x200
    mov cr4, eax
    fninit
    ret

.sse: ;enable sse
    mov eax, cr4
    or ax, 0000011000000000b
    mov cr4, eax
    ret


bits 64
long_mode_start:

    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax


    call initialize.fpu
    call initialize.sse

    extern rust_main     ; new
    call rust_main       ; new


    ; print `OKAY` to screen
    mov rax, 0x2f592f412f4b2f4f
    mov qword [0xb8000], rax
    hlt
