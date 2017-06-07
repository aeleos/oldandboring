cmake_minimum_required (VERSION 2.8)

include(ExternalProject)
include(CMakeParseArguments)
include(BuildCrossProject.cmake)

if(${CMAKE_CURRENT_SOURCE_DIR} STREQUAL ${CMAKE_CURRENT_BINARY_DIR} AND NOT WIN32)
	message(FATAL_ERROR "You cannot build the toolchain in source. Please create a build directory and run cmake ../")
endif(${CMAKE_CURRENT_SOURCE_DIR} STREQUAL ${CMAKE_CURRENT_BINARY_DIR} AND NOT WIN32)


set(SYS_TARGET "i686-pc-boring")

BuildCrossProject(
	PATCH
	PROJ binutils
	PROJ_VERSION 2.27
	URL http://ftp.gnu.org/gnu/binutils/
	CONFIGURE_COMMAND configure --target=${SYS_TARGET} --prefix=<INSTALL_DIR> --with-sysroot=${CMAKE_CURRENT_SOURCE_DIR}/hdd --disable-werror > /dev/null 2>&1
	BUILD_COMMAND make -j4  > /dev/null 2>&1
	INSTALL_COMMAND make install > /dev/null 2>&1
)

BuildCrossProject(
	PROJ binutils-elf
	DEPENDS binutils
	DOWNLOAD_NAME binutils
	PROJ_VERSION 2.27
	CONFIGURE_COMMAND configure --target=i686-elf --prefix=<INSTALL_DIR> --disable-werror > /dev/null 2>&1
	BUILD_COMMAND make -j4 > /dev/null 2>&1
	INSTALL_COMMAND make install > /dev/null 2>&1
)

BuildCrossProject(
	PATCH
	PROJ gcc
	DEPENDS binutils binutils-elf
	PROJ_VERSION 6.3.0
	URL http://www.netgull.com/gcc/releases/gcc-6.3.0
	CONFIGURE_COMMAND configure --target=${SYS_TARGET} --prefix=<INSTALL_DIR> --with-sysroot=${CMAKE_CURRENT_SOURCE_DIR}/hdd --disable-nls --enable-languages=c,c++,go --disable-libssp --with-newlib > /dev/null 2>&1
	BUILD_COMMAND make -j4 all-gcc all-target-libgcc > /dev/null 2>&1
	INSTALL_COMMAND make install-gcc install-target-libgcc > /dev/null 2>&1
)

BuildCrossProject(
	PROJ gcc-elf
	DEPENDS binutils binutils-elf gcc
	DOWNLOAD_NAME gcc
	PROJ_VERSION 6.3.0
	CONFIGURE_COMMAND configure --target=i686-elf --prefix=<INSTALL_DIR> --disable-nls --enable-languages=c --disable-libssp --without-headers > /dev/null 2>&1
	BUILD_COMMAND make -j4 all-gcc all-target-libgcc > /dev/null 2>&1
	INSTALL_COMMAND make install-gcc install-target-libgcc > /dev/null 2>&1
)
