
if (CMAKE_CROSSCOMPILING AND (NOT DEFINED Rust_CARGO_TARGET))
	message(FATAL_ERROR
		"Rust_CARGO_TARGET not defined while crosscompiling,"
		"which is necessary to crosscompile the rust code!"
	)
endif()

set(CMAKE_POLICY_DEFAULT_CMP0116 NEW)

include(FetchContent)

FetchContent_Declare(
	Corrosion
	GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
	GIT_TAG v0.5.1
)
FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(
	MANIFEST_PATH "${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml"
)

corrosion_experimental_cbindgen(
	TARGET kindle_personal_dashboard_core
	HEADER_NAME "kindle_personal_dashboard_core.h"
	FLAGS --lang c++
)

