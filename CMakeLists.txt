cmake_minimum_required(VERSION 3.21)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_OSX_ARCHITECTURES "x86_64")
set(CMAKE_CXX_VISIBILITY_PRESET hidden)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON) # generate compile_commands.json for clangd

# Set target platform based on the current system
if(WIN32)
    set(GEODE_TARGET_PLATFORM "Win64")
elseif(APPLE)
    set(GEODE_TARGET_PLATFORM "MacOS")
else()
    set(GEODE_TARGET_PLATFORM "Android64")
endif()

project(zcblive VERSION 1.0.0)

add_library(${PROJECT_NAME} SHARED src/main.cpp)

add_subdirectory(${CMAKE_SOURCE_DIR}/live)
add_library(zcblive_rust STATIC IMPORTED)

# Set the correct library file name based on platform
if(WIN32)
    set(RUST_LIB_NAME "zcblive.lib")
else()
    set(RUST_LIB_NAME "libzcblive.a")
endif()

set_target_properties(zcblive_rust PROPERTIES IMPORTED_LOCATION ${CMAKE_CURRENT_SOURCE_DIR}/live/target/release/${RUST_LIB_NAME})
target_link_libraries(${PROJECT_NAME} zcblive_rust)

if (NOT DEFINED ENV{GEODE_SDK})
    message(FATAL_ERROR "Unable to find Geode SDK! Please define GEODE_SDK environment variable to point to Geode")
else()
    message(STATUS "Found Geode: $ENV{GEODE_SDK}")
endif()

add_subdirectory($ENV{GEODE_SDK} ${CMAKE_CURRENT_BINARY_DIR}/geode)

setup_geode_mod(${PROJECT_NAME})
