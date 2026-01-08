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
        MANIFEST_PATH "${CMAKE_CURRENT_SOURCE_DIR}/crates/c-api/Cargo.toml"
)

set(MAYON_INCLUDE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/crates/c-api/include")

target_include_directories(mayon_c_api PUBLIC ${MAYON_INCLUDE_DIR})

install(TARGETS mayon_c_api
        LIBRARY DESTINATION lib
        ARCHIVE DESTINATION lib
        RUNTIME DESTINATION bin
)

install(DIRECTORY "${MAYON_INCLUDE_DIR}/" DESTINATION include)
