cmake_minimum_required (VERSION 2.8)

include(ExternalProject)
include(CMakeParseArguments)

set(CMAKE_MODULE_PATH ${CMAKE_SOURCE_DIR}/cmake)

include(cmake/BuildCrossProject.cmake)
include(cmake/GetDate.cmake)

if(${CMAKE_CURRENT_SOURCE_DIR} STREQUAL ${CMAKE_CURRENT_BINARY_DIR} AND NOT WIN32)
	message(FATAL_ERROR "You cannot build the toolchain in source. Please create a build directory and run cmake ../")
endif(${CMAKE_CURRENT_SOURCE_DIR} STREQUAL ${CMAKE_CURRENT_BINARY_DIR} AND NOT WIN32)


set(SYS_TARGET "i686-pc-boring")

GET_DATE()


# set(TIME_STAMP "${GD_YEAR}-${GD_MONTH}-${GD_DAY}_${GD_HOUR}-${GD_MINUTE}-${GD_SECOND}")
#
#
# file(MAKE_DIRECTORY "${CMAKE_BINARY_DIR}/log/build_log-${TIME_STAMP}")
#
# set(LOG_DIR "${CMAKE_BINARY_DIR}/log/build_log-${TIME_STAMP}")
#
# set(BINUTILS_CONFIGURE_LOG_FILE ${LOG_DIR}/binutils-configure.log)
# set(BINUTILS_BUILD_LOG_FILE ${LOG_DIR}/binutils-build.log)
# set(BINUTILS_INSTALL_LOG_FILE ${LOG_DIR}/binutils-install.log)
# set(BINUTILS_ELF_CONFIGURE_LOG_FILE ${LOG_DIR}/binutils-elf-configure.log)
# set(BINUTILS_ELF_BUILD_LOG_FILE ${LOG_DIR}/binutils-elf-build.log)
# set(BINUTILS_ELF_INSTALL_LOG_FILE ${LOG_DIR}/binutils-elf-install.log)
# set(GCC_CONFIGURE_LOG_FILE ${LOG_DIR}/gcc-configure.log)
# set(GCC_BUILD_LOG_FILE ${LOG_DIR}/gcc-build.log)
# set(GCC_INSTALL_LOG_FILE ${LOG_DIR}/gcc-install.log)
# set(GCC_ELF_CONFIGURE_LOG_FILE ${LOG_DIR}/gcc-elf-configure.log)
# set(GCC_ELF_BUILD_LOG_FILE ${LOG_DIR}/gcc-elf-build.log)
# set(GCC_ELF_INSTALL_LOG_FILE ${LOG_DIR}/gcc-elf-install.log)
# set(NEWLIB_CONFIGURE_LOG_FILE ${LOG_DIR}/newlib-configure.log)
# set(NEWLIB_BUILD_LOG_FILE ${LOG_DIR}/newlib-build.log)
# set(NEWLIB_INSTALL_LOG_FILE ${LOG_DIR}/newlib-install.log)
# set(LIBSTDCPP_CONFIGURE_LOG_FILE ${LOG_DIR}/libstdcpp-configure.log)
# set(LIBSTDCPP_BUILD_LOG_FILE ${LOG_DIR}/libstdcpp-build.log)
# set(LIBSTDCPP_INSTALL_LOG_FILE ${LOG_DIR}/libstdcpp-install.log)

message($ENV{PATH})

BuildCrossProject(
	PATCH
	PROJ binutils
	PROJ_VERSION 2.27
	URL http://ftp.gnu.org/gnu/binutils/
	CONFIGURE_COMMAND configure --target=${SYS_TARGET} --prefix=<INSTALL_DIR> --with-sysroot=${CMAKE_CURRENT_SOURCE_DIR}/hdd --disable-werror > /dev/null 2>&1
	BUILD_COMMAND make -j8 > /dev/null 2>&1
	INSTALL_COMMAND make install > /dev/null 2>&1
)

BuildCrossProject(
	PROJ binutils-elf
	DEPENDS binutils
	DOWNLOAD_NAME binutils
	PROJ_VERSION 2.27
	CONFIGURE_COMMAND configure --target=i686-elf --prefix=<INSTALL_DIR> --disable-werror > /dev/null 2>&1
	BUILD_COMMAND make -j8 > /dev/null 2>&1
	INSTALL_COMMAND make install > /dev/null 2>&1
)

BuildCrossProject(
	PATCH
	PROJ gcc
	DEPENDS binutils binutils-elf
	PROJ_VERSION 6.3.0
	URL http://www.netgull.com/gcc/releases/gcc-6.3.0
	CONFIGURE_COMMAND configure --target=${SYS_TARGET} --prefix=<INSTALL_DIR> --with-sysroot=${CMAKE_CURRENT_SOURCE_DIR}/hdd --disable-nls --enable-languages=c,c++,go --disable-libssp --with-newlib > /dev/null 2>&1
	BUILD_COMMAND make -j8 all-gcc all-target-libgcc > /dev/null 2>&1
	INSTALL_COMMAND make install-gcc install-target-libgcc > /dev/null 2>&1
)

BuildCrossProject(
	PROJ gcc-elf
	DEPENDS binutils binutils-elf gcc
	DOWNLOAD_NAME gcc
	PROJ_VERSION 6.3.0
	CONFIGURE_COMMAND configure --target=i686-elf --prefix=<INSTALL_DIR> --disable-nls --enable-languages=c --disable-libssp --without-headers > /dev/null 2>&1
	BUILD_COMMAND make -j8 all-gcc all-target-libgcc > /dev/null 2>&1
	INSTALL_COMMAND make install-gcc install-target-libgcc > /dev/null 2>&1
)

BuildCrossProject(
	PATCH
	PROJ newlib
	DEPENDS gcc-elf
	PROJ_VERSION 1.19.0
	URL http://b.dakko.us/~klange/mirrors
	CONFIGURE_COMMAND configure --target=${SYS_TARGET} --prefix=/usr --enable-newlib-io-c99-formats > /dev/null 2>&1
	BUILD_COMMAND make -j8 > /dev/null 2>&1
	INSTALL_COMMAND make DESTDIR=${CMAKE_CURRENT_SOURCE_DIR}/hdd install > /dev/null 2>&1
)


set(NEWLIB_LIBC newlib/libc)
set(NEWLIB_PATCH_DIR ${CMAKE_CURRENT_SOURCE_DIR}/patches/newlib)

ExternalProject_Add_Step(newlib patch-extras
	WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/tarballs/newlib-1.19.0
  COMMAND cp -r ${NEWLIB_PATCH_DIR}/boring ${NEWLIB_LIBC}/sys/boring
	COMMAND cp -r ${NEWLIB_PATCH_DIR}/include/pwd.h ${NEWLIB_LIBC}/sys/boring/pwd.h
	COMMAND cp -r ${NEWLIB_PATCH_DIR}/include/syscall.h ${NEWLIB_LIBC}/sys/boring/syscall.h
	COMMAND cp -r ${NEWLIB_PATCH_DIR}/include/syscall_nums.h ${NEWLIB_LIBC}/sys/boring/syscall_nums.h
	COMMAND cp -r ${NEWLIB_PATCH_DIR}/setjmp.S ${NEWLIB_LIBC}/machine/i386/setjmp.S
	COMMAND cp -r ${NEWLIB_PATCH_DIR}/wcwidth.c ${NEWLIB_LIBC}/string/wcwidth.c
	COMMAND cp -r ${NEWLIB_PATCH_DIR}/wcswidth.c ${NEWLIB_LIBC}/string/wcswidth.c
	#COMMAND find -type f -print0 | xargs -0 sed 's|--cygnus||g;s|cygnus||g' -i {} +
	#COMMAND "sed" -i "" -e 's|--cygnus||g;s|cygnus||g' $(find -type f)
	COMMAND cmake -DREPLACE_DIR=${CMAKE_CURRENT_BINARY_DIR}/tarballs/newlib-1.19.0/newlib -DMATCH_STRING=--cygnus -P ${CMAKE_CURRENT_SOURCE_DIR}/cmake/StringReplaceInDir.cmake
	COMMAND cmake -DREPLACE_DIR=${CMAKE_CURRENT_BINARY_DIR}/tarballs/newlib-1.19.0/newlib -DMATCH_STRING=cygnus -P ${CMAKE_CURRENT_SOURCE_DIR}/cmake/StringReplaceInDir.cmake
	DEPENDEES patch
	DEPENDERS configure)

ExternalProject_Add_Step(newlib run-autoreconf
	WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/tarballs/newlib-1.19.0/newlib/libc/sys/boring
	COMMAND cmake -E chdir ../. autoconf > /dev/null 2>&1
	COMMAND touch INSTALL NEWS README AUTHORS ChangeLog COPYING
	COMMAND autoreconf > /dev/null 2>&1
	COMMAND yasm -f elf -o crt0.o crt0.s
	COMMAND yasm -f elf -o crti.o crti.s
	COMMAND yasm -f elf -o crtn.o crtn.s
	COMMAND cp crt0.o ../
	COMMAND cp crt0.o /tmp/__boring_crt0.o
	COMMAND cp crti.o ../
	COMMAND cp crti.o /tmp/__boring_crti.o
	COMMAND cp crtn.o ../
	COMMAND cp crtn.o /tmp/__boring_crtn.o
  DEPENDEES patch-extras
	DEPENDERS configure)


ExternalProject_Add_Step(newlib cleanup
	WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/newlib
	COMMAND mkdir -p ${SYS_TARGET}/newlib/libc/sys
	COMMAND cp /tmp/__boring_crt0.o ${SYS_TARGET}/newlib/libc/sys/crt0.o
	COMMAND rm /tmp/__boring_crt0.o
	COMMAND cp /tmp/__boring_crti.o ${SYS_TARGET}/newlib/libc/sys/crti.o
	COMMAND rm /tmp/__boring_crti.o
	COMMAND cp /tmp/__boring_crtn.o ${SYS_TARGET}/newlib/libc/sys/crtn.o
	COMMAND rm /tmp/__boring_crtn.o
	COMMAND echo "" > ${CMAKE_CURRENT_BINARY_DIR}/tarballs/newlib-1.19.0/newlib/libc/stdlib/malign.c

	DEPENDEES run-autoreconf
	DEPENDERS configure)

ExternalProject_Add_Step(newlib makefile-fix
	WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/newlib
	COMMAND cmake -DREPLACE_FILE=Makefile -DMATCH_STRING=prefix}/i686-pc-boring -DREPLACE_STRING=prefix} -P ${CMAKE_CURRENT_SOURCE_DIR}/cmake/StringReplaceInFile.cmake
	COMMAND cmake -DREPLACE_FILE=Makefile -DMATCH_STRING=i686-pc-boring-cc -DREPLACE_STRING=i686-pc-boring-gcc -P ${CMAKE_CURRENT_SOURCE_DIR}/cmake/StringReplaceInFile.cmake
	DEPENDEES configure
	DEPENDERS build)

ExternalProject_Add_Step(newlib post-install
	WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/newlib
	COMMAND cp -r ${NEWLIB_PATCH_DIR}/include/pwd.h ${CMAKE_CURRENT_SOURCE_DIR}/hdd/usr/include/pwd.h
	COMMAND cp -r ${NEWLIB_PATCH_DIR}/include/syscall.h ${CMAKE_CURRENT_SOURCE_DIR}/hdd/usr/include/syscall.h
	COMMAND cp -r ${NEWLIB_PATCH_DIR}/include/syscall_nums.h ${CMAKE_CURRENT_SOURCE_DIR}/hdd/usr/include/syscall_nums.h
	COMMAND cp ${SYS_TARGET}/newlib/libc/sys/crt0.o ${CMAKE_CURRENT_SOURCE_DIR}/hdd/usr/lib/
	COMMAND cp ${SYS_TARGET}/newlib/libc/sys/crti.o ${CMAKE_CURRENT_SOURCE_DIR}/hdd/usr/lib/
	COMMAND cp ${SYS_TARGET}/newlib/libc/sys/crtn.o ${CMAKE_CURRENT_SOURCE_DIR}/hdd/usr/lib/
	DEPENDEES install)


ExternalProject_Add_Step(newlib build-libstdc++
	WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/newlib
	COMMAND make all-target-libstdc++-v3 > /dev/null 2>&1
	COMMAND make install-target-libstdc++-v3 > /dev/null 2>&1
	DEPENDEES post-install)
