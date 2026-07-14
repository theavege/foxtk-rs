cmake_minimum_required(VERSION 3.20)
project(cfoxtk CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(SDK_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../use/winsdk-7/v7.1A)

if(NOT DEFINED ENV{FOX_PATH} OR "$ENV{FOX_PATH}" STREQUAL "")
    message(FATAL_ERROR "FOX_PATH environment variable is not set")
endif()

target_compile_definitions(cfoxtk PRIVATE WIN32)
target_compile_definitions(fox PRIVATE WIN32)

cmake_path(SET FOX_PATH NORMALIZE "$ENV{FOX_PATH}")
file(GLOB FOX_SOURCE ${FOX_PATH}/lib/*.cpp)

set(OBJS foxtk.cpp $<TARGET_OBJECTS:fox>)
foreach(tar IN ITEMS AdvAPI32 ComCtl32 Gdi32 Imm32 Mpr MSImg32 User32 WinSpool WSock32)
    add_library(${tar} STATIC IMPORTED)
    set_property(TARGET ${tar} PROPERTY IMPORTED_LOCATION ${SDK_DIR}/Lib/x64/${tar}.lib)
    list(APPEND OBJS $<TARGET_OBJECTS:${tar}>)
endforeach()

add_library(fox OBJECT ${FOX_SOURCE} )
add_library(cfoxtk STATIC ${OBJS} )

target_include_directories(fox PRIVATE ${FOX_PATH}/include)
target_include_directories(cfoxtk PRIVATE ${FOX_PATH}/include ${SDK_DIR}/Include)