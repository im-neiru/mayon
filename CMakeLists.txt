cmake_minimum_required(VERSION 3.22)
project(MayonLibrary VERSION 0.1.0 LANGUAGES C)

include(FetchContent)

FetchContent_Declare(
        Corrosion
        GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
        GIT_TAG v0.6.0
)

FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(MANIFEST_PATH "${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml" CRATES mayon_c_api)

target_include_directories(mayon INTERFACE
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/crates/c_api/include>
        $<INSTALL_INTERFACE:include>
)

target_sources(mayon
        INTERFACE
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/crates/c_api/include/Mayon.h>
        $<INSTALL_INTERFACE:include/Mayon.h>
)

if (NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Debug CACHE STRING "Build type" FORCE)
endif ()
set(RUST_BUILD_DIR "${CMAKE_CURRENT_BINARY_DIR}/_deps/corrosion-src/build/cargo_target/${CMAKE_BUILD_TYPE}")

set(RUST_LIB_NAME "mayon")

if (WIN32)
    set(LIB_FILE "${RUST_BUILD_DIR}/${RUST_LIB_NAME}.dll")
elseif (APPLE)
    set(LIB_FILE "${RUST_BUILD_DIR}/lib${RUST_LIB_NAME}.dylib")
else ()
    set(LIB_FILE "${RUST_BUILD_DIR}/lib${RUST_LIB_NAME}.so")
endif ()

install(FILES "${LIB_FILE}" DESTINATION lib)

set(MAYON_INCLUDE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/crates/c_api/include")
install(DIRECTORY "${MAYON_INCLUDE_DIR}/" DESTINATION include)

corrosion_install(TARGETS mayon EXPORT MayonTargets)
install(
        EXPORT MayonTargets
        NAMESPACE Mayon::
        DESTINATION lib/cmake/Mayon
)
