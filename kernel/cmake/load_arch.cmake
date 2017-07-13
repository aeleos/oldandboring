FUNCTION(LOAD_ARCH ARCH)
  # Load flags associated with ISA and Profile
  INCLUDE("arch/${ARCH}/arch.cmake")

  FILE(GLOB ARCH_SRCS "arch/${ARCH}/*.c" "arch/${ARCH}/*.S")

  FILE(GLOB ARCH_LINKER "arch/${ARCH}/linker.ld")


  # Now export our output variables
  SET(ARCH_SRCS "${ARCH_SRCS}" PARENT_SCOPE)
  SET(ARCH_LINKER "${ARCH_LINKER}" PARENT_SCOPE)
  SET(ARCH_LINKER_FLAGS "${ARCH_LINKER_FLAGS}" PARENT_SCOPE)

  # And specific flags
  SET(ARCH_C_FLAGS ${ARCH_C_FLAGS} PARENT_SCOPE)
  SET(ARCH_ATT_FLAGS ${ARCH_ASM_ATT_FLAGS} PARENT_SCOPE)

  # ...
ENDFUNCTION(LOAD_ARCH)
