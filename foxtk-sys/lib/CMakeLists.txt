cmake_minimum_required(VERSION 3.31)

project(cfoxtk CXX)

add_compile_definitions(WIN32)

set(FOX_PATH "$ENV{FOX_PATH}")
file(GLOB FOX_SOURCE ${FOX_PATH}/lib/*)

add_library(fox OBJECT ${FOX_SOURCE} )

set(OBJS foxtk.cpp $<TARGET_OBJECTS:fox>)
foreach(tar IN ITEMS AdvAPI32 ComCtl32 Gdi32 Imm32 Mpr MSImg32 User32 WinSpool WSock32)
    add_library(${tar} STATIC IMPORTED)
    set_property(TARGET ${tar} PROPERTY IMPORTED_LOCATION ../use/winsdk-7/v7.1A/Lib/x64/${tar}.lib)
    list(APPEND OBJS $<TARGET_OBJECTS:${tar}>)
endforeach()

add_library(cfoxtk STATIC ${OBJS} )
include_directories(cfoxtk PRIVATE ${FOX_PATH}/include ../use/winsdk-7/v7.1A/Include)
