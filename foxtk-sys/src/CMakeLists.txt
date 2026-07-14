cmake_minimum_required(VERSION 3.20)
project(cfoxtk CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(NOT DEFINED ENV{FOX_PATH} OR "$ENV{FOX_PATH}" STREQUAL "")
    message(FATAL_ERROR "FOX_PATH environment variable is not set")
endif()

cmake_path(SET FOX_PATH NORMALIZE "$ENV{FOX_PATH}")

file(GLOB FOX_SOURCE CONFIGURE_DEPENDS ${FOX_PATH}/lib/*.cpp)
add_library(fox OBJECT ${FOX_SOURCE})
target_include_directories(fox PRIVATE ${FOX_PATH}/include)

add_library(cfoxtk STATIC foxtk.cpp $<TARGET_OBJECTS:fox>)

target_include_directories(cfoxtk PRIVATE
    ${FOX_PATH}/include
    ${CMAKE_CURRENT_SOURCE_DIR}/../use/winsdk-7/v7.1A/Include
)

target_compile_definitions(cfoxtk PRIVATE WIN32)

set(SDK_LIB_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../use/winsdk-7/v7.1A/Lib/x64)
target_link_libraries(cfoxtk PRIVATE
    ${SDK_LIB_DIR}/AdvAPI32.lib
    ${SDK_LIB_DIR}/ComCtl32.lib
    ${SDK_LIB_DIR}/Gdi32.lib
    ${SDK_LIB_DIR}/Imm32.lib
    ${SDK_LIB_DIR}/Mpr.lib
    ${SDK_LIB_DIR}/MSImg32.lib
    ${SDK_LIB_DIR}/User32.lib
    ${SDK_LIB_DIR}/WinSpool.lib
    ${SDK_LIB_DIR}/WSock32.lib
)