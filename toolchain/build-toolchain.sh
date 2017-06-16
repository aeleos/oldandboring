#!/bin/bash

#Parts taken from toaruOS
#https://github.com/klange/toaruos
#

if [ -d "build" ]; then
  echo "Warning, build directory already exists, will rebuild from there. For a fresh build, please delete the directory."
else
  mkdir build
fi


echo "I am going to install some system packages. I will probably need you to provide a password."
echo "If you don't want to do this and you're sure you have all of the required system packages, then interrupt the password prompt and run this script again with -q."

if [ -f /etc/debian_version ]; then
    sudo apt-get install yasm genext2fs build-essential wget libmpfr-dev libmpc-dev libgmp3-dev qemu autoconf automake texinfo pkg-config git ctags gperf grub-pc-bin xorriso
elif [ -f /etc/fedora-release ]; then
    sudo dnf groupinstall 'Development Tools'
    sudo dnf groupinstall 'Development Libraries'
    sudo dnf install yasm mpfr-devel libmpc-devel gmp-devel gperf
    echo "Warning: Fedora is unsupported in this script. Be careful!"
    echo "For best results, follow the steps in the script manually."
    echo "(Script will continue in 5 seconds)"
    sleep 5
else
    echo "You are on an entirely unsupported system, please ensure you have the following packages:"
    echo "  - essential development packages for your platform (C headers, etc.)"
    echo "  - development headers for mpfr, libmpc, and gmp"
    echo "  - gcc"
    echo "  - YASM"
    echo "  - genext2fs"
    echo "  - autoconf/automake"
    echo "  - wget"
    echo "  - qemu"
    echo "  - texinfo"
    echo "  - pkg-config"
    echo "  - git"
    echo "  - ctags"
    echo "(If you are on Arch, install: gcc yasm genext2fs base-devel wget mpfr libmpc gmp qemu autoconf automake texinfo pkg-config git ctags)"
    echo ""
    echo "... then run this script (toolchain/toolchain-build.sh) again with the -q flag."
    exit 1
fi


source set-env-vars.sh


pushd build
  cmake ..
  make
popd
