cmake_minimum_required(VERSION 3.23)

project(kindle_personal_dashboard)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(CMAKE_INTERPROCEDURAL_OPTIMIZATION_RELEASE ON)

set(CMAKE_C_FLAGS_DEBUG "-O0 -g3")
set(CMAKE_CXX_FLAGS_DEBUG "-O0 -g3")

set(CMAKE_C_FLAGS_RELEASE "-O2 -DNDEBUG")
set(CMAKE_CXX_FLAGS_RELEASE "-O2 -DNDEBUG")


# GTK+-2.0
find_package(PkgConfig REQUIRED)
pkg_check_modules(GTK IMPORTED_TARGET REQUIRED gtk+-2.0)


# rust kindle_personal_dashboard_core library
add_subdirectory(./core)


# kindle_personal_dashboard
add_executable(${PROJECT_NAME} "./src/main.cpp")

target_compile_features(${PROJECT_NAME} PUBLIC cxx_std_17)
set_target_properties(${PROJECT_NAME} PROPERTIES CXX_STANDARD_REQUIRED ON)
set_target_properties(${PROJECT_NAME} PROPERTIES CXX_EXTENSIONS OFF)

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

	-ffunction-sections
	-fdata-sections

	-fstack-protector-strong

	-fvisibility=hidden
)

target_include_directories(${PROJECT_NAME} PUBLIC
	"${CMAKE_CURRENT_SOURCE_DIR}/include"
)

target_link_libraries(${PROJECT_NAME} PRIVATE
	kindle_personal_dashboard_core
	PkgConfig::GTK
)

target_link_options(${PROJECT_NAME} PRIVATE -Wl,--gc-sections)


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
