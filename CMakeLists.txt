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


add_subdirectory(./rust_foo)


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

	-fvisibility=hidden)

target_include_directories(${PROJECT_NAME} PUBLIC
	"${CMAKE_CURRENT_SOURCE_DIR}/include")

target_link_libraries(${PROJECT_NAME} PRIVATE PkgConfig::GTK rust_foo)

target_link_options(${PROJECT_NAME} PRIVATE -Wl,--gc-sections)
