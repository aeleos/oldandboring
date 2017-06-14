#!/bin/bash
grub-mkrescue /usr/lib/grub/i386-pc -o boringos-disk.img util

qemu-system-i386 -cdrom boringos-disk.img
