#!/bin/bash


function bail () {
    echo -e "\033[1;31mBuild failed. Please check the logs above to see what went wrong.\033[0m"
    exit 1
}

function print_build_help(){
    echo "options:"
    echo "-h, --help                show brief help"
    echo "-r, --rebuild             full rebuild of kernel"
    echo "-f, --force               force full rebuild of entire kernel, including toolchain. Requires the -r option"

}
