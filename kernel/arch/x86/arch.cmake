set(ARCH_C_FLAGS "-std=gnu99 -O2 -Wall -Wextra -ffreestanding -nostdlib")

set(ARCH_ASM_FLAGS "-f elf -F dwarf")

set(ARCH_LINKER_FLAGS "-nostdlib -lgcc")
