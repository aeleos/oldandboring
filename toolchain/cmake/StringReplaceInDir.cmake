include(CMakeParseArguments)

set(REPLACE_DIR "" CACHE STRING "Directory to search in")
set(MATCH_STRING "" CACHE STRING "String to look for")
set(REPLACE_STRING " " CACHE STRING "String to replace with")


function(StringReplaceInDir)

  #message("${REPLACE_DIR}")
  #message("${MATCH_STRING}")
  #message("${REPLACE_STRING}")

  file(GLOB_RECURSE files ${REPLACE_DIR} ${REPLACE_DIR}/*)
  #message("${files}")
  foreach(file_dir ${files})
    file(READ ${file_dir} file)

    #message("${file}")
    if (file)
      STRING(REPLACE ${MATCH_STRING} ${REPLACE_STRING} file_replaced "${file}")
    endif()

    if (file_replaced EQUAL file)
    else()
    #  file(REMOVE ${file_dir})
      file(WRITE ${file_dir} "${file_replaced}")
      #message("updated file: ${file_dir}")
    endif()
    #message("${file}")
    #file(WRITE ${file_dir} ${file})
    # ... calculate ${i} to get the test name
    # add_test(validate_${i}, "validator", ${file})
  endforeach()


endfunction()

StringReplaceInDir()
