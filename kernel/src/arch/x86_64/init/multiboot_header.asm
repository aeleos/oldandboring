section .multiboot_header

extern LOADER_START
extern _bss_start
extern _kernel_end
extern start

multiboot2_start:
    dd 0xe85250d6                ; magic number (multiboot 2)
    dd 0                         ; architecture 0 (protected mode i386)
    dd multiboot2_end - multiboot2_start ; header length
    ; checksum
    dd 0x100000000 - (0xe85250d6 + 0 + (multiboot2_end - multiboot2_start))

    align 8
    address_tag_start:
        dw 2
        dw 0
        dd address_tag_end - address_tag_start   ;the size of this tag
        dd multiboot2_start                      ;the address of the beginning of the header
        dd LOADER_START                          ;the address at which the text segment should be loaded
        dd _bss_start                            ;the address at which the data segment ends
        dd _kernel_end                           ;the address at which the bss segment ends
    address_tag_end:

    entry_tag_start:
        dw 3
        dw 0
        dd entry_tag_end - entry_tag_start       ;the size of this tag
        dd start                                 ;the entry address for the kernel
    entry_tag_end:

    ; framebuffer_tag_start:
    ;     ; insert optional multiboot tags here
    ;     dw 5     ; type = 5
    ;     dw 0     ; flags
    ;     dd framebuffer_tag_end - framebuffer_tag_start
    ;     dd 0 ; width
    ;     dd 0  ; height
    ;     dd 0   ; depth
    ; framebuffer_tag_end:

    align 8

    ; required end tag
    end_tag_start:
        dw 0    ; type
        dw 0    ; flags
        dd end_tag_end - end_tag_start
    end_tag_end:

multiboot2_end:     ;end of multiboot header


;support for multiboot
%define MULTIBOOT_MAGIC 0x1badb002
%define MULTIBOOT_FLAGS 0x10003

multiboot_start:    ;start of multiboot header
    dd MULTIBOOT_MAGIC
    dd MULTIBOOT_FLAGS
    dd -(MULTIBOOT_MAGIC + MULTIBOOT_FLAGS)  ;checksum
    dd multiboot_start                       ;physical address of the header
    dd LOADER_START                          ;the address at which loading should start
    dd _bss_start                            ;the end address of the data segment
    dd _kernel_end                           ;the end address of the bss segment
    dd start                                 ;the entry point for the kernel
multiboot_end:
