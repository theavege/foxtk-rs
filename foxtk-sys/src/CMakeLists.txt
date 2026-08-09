cmake_minimum_required(VERSION 3.20)
project(cfoxtk CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(WIN32)
    set(SDK_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../use/winsdk-7/v7.1A)

    if(NOT DEFINED ENV{FOX_PATH} OR "$ENV{FOX_PATH}" STREQUAL "")
        message(FATAL_ERROR "FOX_PATH environment variable is not set")
    endif()

    cmake_path(SET FOX_PATH NORMALIZE "$ENV{FOX_PATH}")
    file(GLOB FOX_SOURCE CONFIGURE_DEPENDS ${FOX_PATH}/src/*.cpp)
    set(OBJS foxtk.cpp $<TARGET_OBJECTS:fox>)
    foreach(tar IN ITEMS AdvAPI32 ComCtl32 Gdi32 Imm32 Mpr MSImg32 User32 WinSpool WSock32)
        add_library(${tar} STATIC IMPORTED)
        set_property(TARGET ${tar} PROPERTY IMPORTED_LOCATION ${SDK_DIR}/Lib/x64/${tar}.lib)
        list(APPEND OBJS $<TARGET_OBJECTS:${tar}>)
    endforeach()

    add_compile_definitions(WIN32)
    add_compile_definitions(HAVE_VSSCANF)

    add_library(fox OBJECT ${FOX_SOURCE} )
    target_include_directories(fox PRIVATE ${FOX_PATH}/include)

    add_library(cfoxtk STATIC ${OBJS} )
    target_compile_options(cfoxtk PRIVATE
        $<$<CXX_COMPILER_ID:MSVC>:/W4 /permissive->
        $<$<NOT:$<CXX_COMPILER_ID:MSVC>>:-Wall -Wextra -Wpedantic>
    )
    target_include_directories(cfoxtk PRIVATE ${FOX_PATH}/include ${SDK_DIR}/Include)
    find_program(CLANG_TIDY "clang-tidy")
    if(CLANG_TIDY)
        message(STATUS "clang-tidy found: ${CLANG_TIDY}")
        set(CMAKE_CXX_CLANG_TIDY ${CLANG_TIDY} --warnings-as-errors=*)
    else()
        message(WARNING "clang-tidy not found!")
    endif()
endif()
