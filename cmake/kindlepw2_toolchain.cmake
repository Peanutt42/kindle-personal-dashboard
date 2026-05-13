cmake_minimum_required(VERSION 3.23)

set(CMAKE_SYSTEM_NAME		"Linux")
set(CMAKE_SYSTEM_PROCESSOR	"arm")
set(KINDLE_TARGET			"arm-kindlepw2-linux-gnueabi")
set(Rust_CARGO_TARGET		"arm-unknown-linux-gnueabi")

include("${CMAKE_CURRENT_LIST_DIR}/generic_kindle_toolchain.cmake")

