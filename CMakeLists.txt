cmake_minimum_required(VERSION 3.23)

project(kindle_personal_dashboard)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(CMAKE_C_FLAGS_DEBUG
	"${CMAKE_C_FLAGS_DEBUG} -Og -g3 -fno-omit-frame-pointer"
)
set(CMAKE_CXX_FLAGS_DEBUG
	"${CMAKE_CXX_FLAGS_DEBUG} -Og -g3 -fno-omit-frame-pointer"
)

set(CMAKE_C_FLAGS_RELEASE
	"${CMAKE_C_FLAGS_RELEASE} -O3 -DNDEBUG -ffast-math -ftree-vectorize"
)
set(CMAKE_CXX_FLAGS_RELEASE
	"${CMAKE_CXX_FLAGS_RELEASE} -O3 -DNDEBUG -ffast-math -ftree-vectorize"
)
set(CMAKE_INTERPROCEDURAL_OPTIMIZATION_RELEASE ON)

find_program(CCACHE_EXE "ccache")
if (CCACHE_EXE)
	message(STATUS "Using ccache: ${CCACHE_EXE}")
	set(CMAKE_C_COMPILER_LAUNCHER ${CCACHE_EXE})
	set(CMAKE_CXX_COMPILER_LAUNCHER ${CCACHE_EXE})
endif()

# GTK+-2.0
find_package(PkgConfig REQUIRED)
pkg_check_modules(GTK IMPORTED_TARGET REQUIRED gtk+-2.0)


# rust kindle_personal_dashboard_core library
add_subdirectory(./core)


# kindle_personal_dashboard
add_executable(${PROJECT_NAME}
	"${CMAKE_CURRENT_SOURCE_DIR}/src/main.cpp"
	"${CMAKE_CURRENT_SOURCE_DIR}/src/window.cpp"
	"${CMAKE_CURRENT_SOURCE_DIR}/src/gh_heatmap.cpp"
	"${CMAKE_CURRENT_SOURCE_DIR}/src/getauxval_fix.cpp"
)

target_compile_features(${PROJECT_NAME} PUBLIC cxx_std_17)
set_target_properties(${PROJECT_NAME} PROPERTIES CXX_STANDARD_REQUIRED ON)
set_target_properties(${PROJECT_NAME} PROPERTIES CXX_EXTENSIONS OFF)

option(KPD_LOCAL_DEV "Enable local development of KPD")
if (KPD_LOCAL_DEV)
	target_compile_definitions(${PROJECT_NAME} PRIVATE KPD_LOCAL_DEV)
endif()

target_compile_options(${PROJECT_NAME} PRIVATE
	-Wall
	-Wextra
	-Wpedantic
	-Wshadow
	-Wconversion
	-Wsign-conversion
	-Wformat=2
	-Wundef
	-Wdouble-promotion
	-Wimplicit-fallthrough

	-Wnon-virtual-dtor
	-Wold-style-cast
	-Woverloaded-virtual

	-fno-rtti
	-fno-exceptions

	-ffunction-sections
	-fdata-sections

	-fstack-protector-strong

	-fvisibility=hidden
)

target_include_directories(${PROJECT_NAME} PUBLIC
	"${CMAKE_CURRENT_SOURCE_DIR}/include/kpd/"
)

target_link_libraries(${PROJECT_NAME} PRIVATE
	kindle_personal_dashboard_core
	PkgConfig::GTK
)

target_link_options(${PROJECT_NAME} PRIVATE
	-Wl,--gc-sections
	-static-libstdc++
	-static-libgcc
)

# Linting: clang-tidy
option(KPD_ENABLE_CLANG_TIDY "Enable clang-tidy for kindle-personal-dashboard")
if (KPD_ENABLE_CLANG_TIDY)
	if (CMAKE_CROSSCOMPILING)
		message(WARNING
			"KPD_ENABLE_CLANG_TIDY enabled but ignored while cross-compiling"
		)
	else()
		find_program(CLANG_TIDY_EXE "clang-tidy")
		if (CLANG_TIDY_EXE)
			if (CMAKE_CROSSCOMPILING)
				message(WARNING
				"KPD_ENABLE_CLANG_TIDY enabled but ignored while cross-compiling"
			)
			else()
				message(STATUS "Using clang-tidy: ${CLANG_TIDY_EXE}")
				set_target_properties(${PROJECT_NAME} PROPERTIES
				CXX_CLANG_TIDY ${CLANG_TIDY_EXE}
			)
			endif()
		else()
			message(WARNING
			"KPD_ENABLE_CLANG_TIDY enabled but could not find clang-tidy!"
		)
		endif()
	endif()
endif()

# Strip
if (CMAKE_STRIP AND CMAKE_BUILD_TYPE STREQUAL "Release")
	add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
		COMMAND ${CMAKE_STRIP} $<TARGET_FILE:kindle_personal_dashboard>
		COMMENT "Stripping $<TARGET_FILE:kindle_personal_dashboard>"
	)
endif()

# Address Sanitization
option(KPD_ENABLE_ASAN "Enable Address Sanitizer for KPD")
if (KPD_ENABLE_ASAN)
	target_compile_options(${PROJECT_NAME} PRIVATE
		-fsanitize=address,undefined
	)
	target_link_options(${PROJECT_NAME} PRIVATE
		-fsanitize=address,undefined
	)
endif()


# Bundling
set(KPD_BUNDLE_DIR "${CMAKE_BINARY_DIR}/bundle")
add_custom_target(bundle_kindle_personal_dashboard
	WORKING_DIRECTORY ${CMAKE_BINARY_DIR}

	COMMAND ${CMAKE_COMMAND} -E make_directory ${KPD_BUNDLE_DIR}

	COMMAND ${CMAKE_COMMAND} -E make_directory "${KPD_BUNDLE_DIR}/bin/"

	COMMAND ${CMAKE_COMMAND} -E copy
		$<TARGET_FILE:kindle_personal_dashboard>
		"${KPD_BUNDLE_DIR}/bin/"

	COMMAND ${CMAKE_COMMAND} -E copy
		"${CMAKE_CURRENT_SOURCE_DIR}/assets/config.xml"
		"${KPD_BUNDLE_DIR}/"

	COMMAND ${CMAKE_COMMAND} -E copy
		"${CMAKE_CURRENT_SOURCE_DIR}/assets/menu.json"
		"${KPD_BUNDLE_DIR}/"

	COMMAND ${CMAKE_COMMAND} -E
		echo "Bundled kindle-personal-dashboard into ${KPD_BUNDLE_DIR}/"
)
