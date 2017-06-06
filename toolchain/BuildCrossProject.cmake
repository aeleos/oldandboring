function(BuildCrossProject)

		set(options PATCH)
    set(oneValueArgs
				DOWNLOAD_NAME
        PROJ
        PROJ_VERSION
        URL
    )
    set(multiValueArgs
			CONFIGURE_COMMAND
			BUILD_COMMAND
			INSTALL_COMMAND
		)

    cmake_parse_arguments(ARG "${options}" "${oneValueArgs}" "${multiValueArgs}" ${ARGN})

		set(PROJ_FULL "${ARG_PROJ}-${ARG_PROJ_VERSION}")

		if (ARG_PATCH)
			set(PATCH_CMD patch -p1 < ${CMAKE_CURRENT_SOURCE_DIR}/patches/${PROJ_FULL}.patch > /dev/null)
		else()
			set(PATCH_CMD "")
		endif()

		if (ARG_DOWNLOAD_NAME)
				set(DL_CMD "")
				set(SRC_DIR ${CMAKE_CURRENT_BINARY_DIR}/tarballs/${ARG_DOWNLOAD_NAME}-${ARG_PROJ_VERSION})
		else()
				set(DL_CMD wget ${ARG_URL}/${PROJ_FULL}.tar.gz -q --show-progress && tar -xzf ${PROJ_FULL}.tar.gz)
				set(SRC_DIR ${CMAKE_CURRENT_BINARY_DIR}/tarballs/${PROJ_FULL})
		endif()

	message("${ARG_PATCH}")


		ExternalProject_Add(${ARG_PROJ}
			STAMP_DIR ${ARG_PROJ}/stamps
			TMP_DIR ${ARG_PROJ}/tmp
	    DOWNLOAD_DIR ${CMAKE_CURRENT_BINARY_DIR}/tarballs
			DOWNLOAD_COMMAND ${DL_CMD}
			PATCH_COMMAND ${PATCH_CMD}
	    SOURCE_DIR ${SRC_DIR}
	    INSTALL_DIR ${CMAKE_CURRENT_BINARY_DIR}/local
	    BINARY_DIR ${CMAKE_CURRENT_BINARY_DIR}/${ARG_PROJ}
	    CONFIGURE_COMMAND <SOURCE_DIR>/${ARG_CONFIGURE_COMMAND}
	    BUILD_COMMAND ${ARG_BUILD_COMMAND}
	    INSTALL_COMMAND ${ARG_INSTALL_COMMAND}
		)


		#message("${ARG_URL} ${ARG_PROJ_VERSION} ${ARG_CONFIGURE_COMMAND} ${ARG_BUILD_COMMAND} ${ARG_INSTALL_COMMAND}")

endfunction()
