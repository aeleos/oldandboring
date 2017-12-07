arch ?= x86_64
kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso
target ?= $(arch)-boring_os
rust_os := target/$(target)/debug/libboring_os.a

linker_script := kernel/arch/$(arch)/linker.ld
grub_cfg := kernel/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard kernel/arch/$(arch)/boot/*.asm)
assembly_object_files := $(patsubst kernel/arch/$(arch)/boot/%.asm, \
	build/arch/$(arch)/boot/%.o, $(assembly_source_files))

.PHONY: all clean run iso kernel gdb

all: $(kernel)

clean:
	@rm -r build

run: $(iso)
	@qemu-system-x86_64 -cdrom $(iso) -s -serial stdio -vga std

debug: $(iso)
	@qemu-system-x86_64 -cdrom $(iso) -s -S -d int -no-reboot

gdb: $(kernel)
	exec rust-gdb "$(kernel)" -ex "target remote :1234"

iso: $(iso)



$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@grub-mkrescue -o $(iso) build/isofiles 2> /dev/null
	@rm -rf build/isofiles

$(kernel): kernel $(rust_os) $(assembly_object_files) $(linker_script)
	@ld -n --gc-sections -T $(linker_script) -o $(kernel) \
		$(assembly_object_files) $(rust_os)

kernel:
	export CARGO_TARGET_DIR=build
	CARGO_INCREMENTAL=1 time xargo build --target $(target)

# compile assembly files
build/arch/$(arch)/boot/%.o: kernel/arch/$(arch)/boot/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@
