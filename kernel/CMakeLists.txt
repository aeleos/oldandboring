cmake_minimum_required(VERSION 2.8.3)

INCLUDE(CMakeForceCompiler)

# this one is important
SET(CMAKE_SYSTEM_NAME Generic)

# specify the cross compiler
CMAKE_FORCE_C_COMPILER(i686-elf-gcc GNU)
CMAKE_FORCE_CXX_COMPILER(i686-elf-cpp GNU)
# CMAKE_ASM_COMPILER(nasm)

project(BoringOS)

enable_language(C ASM_NASM)


if(NOT DEFINED ENV{TOOLCHAIN})
	message(FATAL_ERROR "You have either not actived or not build the toolchain")
endif()
# Force external build
if(${CMAKE_CURRENT_SOURCE_DIR} STREQUAL ${CMAKE_CURRENT_BINARY_DIR} AND NOT WIN32)
	message(FATAL_ERROR "You can not use CMake to build from the root of it's source tree! Remove the CMakeCache.txt file from this directory, then create a separate directory (either below this directory or elsewhere), and then re-run CMake from there.")
endif(${CMAKE_CURRENT_SOURCE_DIR} STREQUAL ${CMAKE_CURRENT_BINARY_DIR} AND NOT WIN32)


# Cmake module path (if exists)
set(CMAKE_MODULE_PATH ${CMAKE_SOURCE_DIR}/cmake)


include(cmake/load_arch.cmake)

load_arch("x86_64")

# message("${ARCH_C_FLAGS}")
# message("${ARCH_ASM_FLAGS}")
# message("${ARCH_LINKER_FLAGS}")
# message("${ARCH_LINKER}")
# message("${ARCH_SRCS}")
message("${ARCH_PATH}")

set(CMAKE_ASM_NASM_COMPILE_OBJECT "<CMAKE_ASM_NASM_COMPILER> ${ARCH_ASM_FLAGS} -o <OBJECT> -i${ARCH_PATH} <SOURCE>")
set(CMAKE_ASM_NASM_LINK_EXECUTABLE "ld <FLAGS> ${ARCH_LINKER_FLAGS} -T ${ARCH_LINKER} <OBJECTS>  -o <TARGET> <LINK_LIBRARIES>")
# message(${CMAKE_ASM_NASM_LINK_EXECUTABLE})
set(CMAKE_C_FLAGS ${ARCH_C_FLAGS})


# Now set up our environment
file(GLOB KERNEL_SRCS "*.c" "*.asm")

add_executable(boringos-kernel "${ARCH_SRCS}" "${KERNEL_SRCS}")

# set_target_properties(boringos-kernel PROPERTIES LINK_FLAGS "-T ${ARCH_LINKER} -N ${ARCH_LINKER_FLAGS}")

INSTALL(TARGETS boringos-kernel DESTINATION ${CMAKE_CURRENT_SOURCE_DIR}/../util/boot)
