include(FetchContent)

if (CMAKE_CROSSCOMPILING)
	set(Rust_CARGO_TARGET "arm-unknown-linux-gnueabi")
endif()

FetchContent_Declare(
	Corrosion
	GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
	GIT_TAG v0.5.1
)
FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(
	MANIFEST_PATH "${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml"
)

# cbindgen in build.rs generates into ./generated/cbindgen/rust_foo/rust_foo.h
target_include_directories(rust_foo INTERFACE
	"${CMAKE_CURRENT_SOURCE_DIR}/generated/cbindgen/")
