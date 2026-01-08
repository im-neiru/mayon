cmake_minimum_required(VERSION 3.22)
project(MayonLibrary VERSION 0.1.0 LANGUAGES C)

include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.6.0
)

FetchContent_MakeAvailable(Corrosion)

set(CORROSION_RUST_TOOLCHAIN "nightly-x86_64-pc-windows-msvc")

corrosion_import_crate(
    MANIFEST_PATH "${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml"
    CRATES mayon_c_api
)


set(MAYON_INCLUDE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/crates/c_api/include")
install(DIRECTORY "${MAYON_INCLUDE_DIR}/" DESTINATION include)

set(RUST_BUILD_DIR "${CMAKE_CURRENT_BINARY_DIR}/_deps/corrosion-src/build/cargo_target/debug")
set(RUST_LIB_NAME "mayon_c_api")

if(WIN32) # WindowOS
    set(LIB_FILE "${RUST_BUILD_DIR}/${RUST_LIB_NAME}.dll")
elseif(APPLE) # MacOS
    set(LIB_FILE "${RUST_BUILD_DIR}/lib${RUST_LIB_NAME}.dylib")
else() # LinuxOS
    set(LIB_FILE "${RUST_BUILD_DIR}/lib${RUST_LIB_NAME}.so")
endif()

install(FILES "${LIB_FILE}" DESTINATION lib)

if(NOT DEFINED CMAKE_INSTALL_PREFIX)
    if(WIN32)
        set(CMAKE_INSTALL_PREFIX "${CMAKE_CURRENT_BINARY_DIR}/install" CACHE PATH "Install path prefix" FORCE)
    else()
        set(CMAKE_INSTALL_PREFIX "/usr/local" CACHE PATH "Install path prefix" FORCE)
    endif()
endif()

message(STATUS "Install prefix: ${CMAKE_INSTALL_PREFIX}")
