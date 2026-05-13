# ==================== CONFIGRATION =======================

# Variables that NEED be set (see ./kindlepw2_toolchain.cmake for how to use):
# - CMAKE_SYSTEM_NAME
# - CMAKE_SYSTEM_PROCESSOR
# - KINDLE_TARGET
# - Rust_CARGO_TARGET

if (NOT DEFINED CMAKE_SYSTEM_NAME)
	message(FATAL_ERROR "CMAKE_SYSTEM_NAME is not yet defined!")
endif()
if (NOT DEFINED CMAKE_SYSTEM_PROCESSOR)
	message(FATAL_ERROR "CMAKE_SYSTEM_PROCESSOR is not yet defined!")
endif()
if (NOT DEFINED KINDLE_TARGET)
	message(FATAL_ERROR "KINDLE_TARGET is not yet defined!")
endif()
if (NOT DEFINED Rust_CARGO_TARGET)
	message(FATAL_ERROR "Rust_CARGO_TARGET is not yet defined!")
endif()

if (NOT KINDLE_TOOLCHAIN_MESSAGES_LOGGED)
	message(STATUS "Kindle Toolchain configuration:")
	message(STATUS "  CMAKE_SYSTEM_NAME:       ${CMAKE_SYSTEM_NAME}")
	message(STATUS "  CMAKE_SYSTEM_PROCESSOR:  ${CMAKE_SYSTEM_PROCESSOR}")
	message(STATUS "  KINDLE_TARGET:           ${KINDLE_TARGET}")
	message(STATUS "  Rust_CARGO_TARGET:       ${Rust_CARGO_TARGET}")

	set(KINDLE_TOOLCHAIN_MESSAGES_LOGGED TRUE)
endif()

# Variables that can be set (depending on the build environment):
# - KINDLE_XTOOLS_DIR




# ==================== IMPLEMENTATION =====================

# by default at ~/x-tools
set(KINDLE_XTOOLS_DIR "$ENV{HOME}/x-tools" CACHE PATH
	"Base directory of x-tools toolchains"
)

# Compilers
set(KINDLE_TOOLCHAIN	"${KINDLE_XTOOLS_DIR}/${KINDLE_TARGET}")
set(KINDLE_SYSROOT		"${KINDLE_TOOLCHAIN}/${KINDLE_TARGET}/sysroot")

set(CMAKE_C_COMPILER	"${KINDLE_TOOLCHAIN}/bin/${KINDLE_TARGET}-gcc")
set(CMAKE_CXX_COMPILER	"${KINDLE_TOOLCHAIN}/bin/${KINDLE_TARGET}-g++")
set(CMAKE_ASM_COMPILER	"${KINDLE_TOOLCHAIN}/bin/${KINDLE_TARGET}-gcc")

set(CMAKE_STRIP "${KINDLE_TOOLCHAIN}/bin/${KINDLE_TARGET}-strip"
	CACHE FILEPATH "Strip"
)
set(CMAKE_AR "${KINDLE_TOOLCHAIN}/bin/${KINDLE_TARGET}-gcc-ar"
	CACHE FILEPATH "Archive"
)
set(CMAKE_RANLIB "${KINDLE_TOOLCHAIN}/bin/${KINDLE_TARGET}-gcc-ranlib"
	CACHE FILEPATH "RanLib"
)
set(CMAKE_NM "${KINDLE_TOOLCHAIN}/bin/${KINDLE_TARGET}-gcc-nm"
	CACHE FILEPATH "NM"
)

# Sysroot + search
set(ONLY_CMAKE_FIND_ROOT_PATH TRUE) # forces use of crosscompile-usable dirs
set(CMAKE_SYSROOT ${KINDLE_SYSROOT})
set(CMAKE_FIND_ROOT_PATH ${KINDLE_SYSROOT})
set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NEVER)
set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_PACKAGE ONLY)

# Sensible flags
set(CMAKE_C_FLAGS_INIT   "--sysroot=${KINDLE_SYSROOT}")
set(CMAKE_CXX_FLAGS_INIT "--sysroot=${KINDLE_SYSROOT}")
set(CMAKE_EXE_LINKER_FLAGS_INIT   "--sysroot=${KINDLE_SYSROOT}")
set(CMAKE_SHARED_LINKER_FLAGS_INIT "--sysroot=${KINDLE_SYSROOT}")

include_directories(BEFORE "${CMAKE_FIND_ROOT_PATH}/usr/include")

# Make pkg-config discover target .pc files inside the sysroot
set(ENV{PKG_CONFIG_PATH} "")
if (DEFINED ENV{PKG_CONFIG})
	set(PKG_CONFIG_EXECUTABLE "$ENV{PKG_CONFIG}" CACHE FILEPATH "")
endif()
set(ENV{PKG_CONFIG_SYSROOT_DIR} "${KINDLE_SYSROOT}")
set(ENV{PKG_CONFIG_LIBDIR}
	"${KINDLE_SYSROOT}/usr/lib/pkgconfig:${KINDLE_SYSROOT}/usr/share/pkgconfig"
)

