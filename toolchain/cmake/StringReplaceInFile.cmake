include(CMakeParseArguments)

set(REPLACE_FILE "" CACHE STRING "Directory to search in")
set(MATCH_STRING "" CACHE STRING "String to look for")
set(REPLACE_STRING " " CACHE STRING "String to replace with")


function(StringReplaceInFile)


  # message("${REPLACE_FILE}")
  # message("${MATCH_STRING}")
  # message("${REPLACE_STRING}")


  file(READ ${REPLACE_FILE} file)

  #message("${file}")
  if (file)
    STRING(REPLACE ${MATCH_STRING} ${REPLACE_STRING} file_replaced "${file}")
  endif()

  if (file_replaced EQUAL file)
  else()
  #  file(REMOVE ${file_dir})
    file(WRITE ${REPLACE_FILE} "${file_replaced}")
    #message("updated file: ${file_dir}")
  endif()


endfunction()

StringReplaceInFile()
