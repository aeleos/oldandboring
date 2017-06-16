cmake_minimum_required(VERSION 2.8.3)

INCLUDE(CMakeForceCompiler)

# this one is important
SET(CMAKE_SYSTEM_NAME Generic)

# specify the cross compiler
CMAKE_FORCE_C_COMPILER(i686-elf-gcc GNU)
CMAKE_FORCE_CXX_COMPILER(i686-elf-cpp GNU)

project(BoringOS)

enable_language(C ASM)

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

load_arch("x86")

# message("${ARCH_C_FLAGS}")
# message("${ARCH_ASM_FLAGS}")
# message("${ARCH_LINKER_FLAGS}")
# message("${ARCH_LINKER}")
# message("${ARCH_SRCS}")

# Now set up our environment
add_executable(boringos-kernel "${ARCH_SRCS}")

set(CMAKE_ASM-ATT_COMPILE_OBJECT "<CMAKE_ASM-ATT_COMPILER> ${ARCH_ASM_FLAGS} -o <OBJECT> <SOURCE>")
set(CMAKE_C_FLAGS ${ARCH_C_FLAGS})
#set_property(TARGET kernel PROPERTY LINK_FLAGS foo bar)

set_target_properties(boringos-kernel PROPERTIES LINK_FLAGS "-T ${ARCH_LINKER} -N ${ARCH_LINKER_FLAGS}")

INSTALL(TARGETS boringos-kernel DESTINATION ${CMAKE_CURRENT_SOURCE_DIR}/../util/boot)
