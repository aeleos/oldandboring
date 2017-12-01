# BoringOS
A boring description for a boring os


## Features
 - Has a github repository
 - Compiles (in theory)
 - Basic VGA output
 - Heap & Paging
 - Double fault protection
 - Simple allocator with new allocator API
 - Interrupts and PIC
 - Keybaord driver with PS2 Support


## TODO
 - Make shell actually usable
 - Multitasking
 - Get more information about host system
 - PCI driver
 - Networking?
 - Filesystem?

## Pictures

![Alt text](images/mandelbrot.png?raw=true "Mandelbrot")

![Alt text](images/terrain.png?raw=true "Terrain Rendering")


## Installation
This should install everything needed to get started from a clean ubuntu 16.04 installation


Install rustup

`curl https://sh.rustup.rs -sSf | sh`

Source the new rust environment variables

`source $HOME/.cargo/env`

Clone the repository and cd into the folder

`git clone https://github.com/aeleos/BoringOS && cd BoringOS`

Tell rustup to use the nightly

`rustup override add nightly`

Install xargo for compiling the toolchain

`cargo install xargo`

Add the rust-src component

`rustup component add rust-src`

Install some dependencies

`sudo apt install qemu xorriso nasm grub-pc-bin`

Build toolchain and BoringOS

`make run`
