cmake_minimum_required(VERSION 3.20)
project(cfoxtk CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
if(NOT DEFINED FOX_PATH OR FOX_PATH STREQUAL "")
    message(FATAL_ERROR "FOX_PATH CMake variable is not set")
else()
    cmake_path(SET FOX_PATH NORMALIZE "${FOX_PATH}")
    file(GLOB FOX_SOURCE CONFIGURE_DEPENDS ${FOX_PATH}/src/*.cpp)
    add_library(fox OBJECT ${FOX_SOURCE} )
    target_include_directories(fox PRIVATE ${FOX_PATH}/include ${FOX_INCLUDE_DIRS})
    set(OBJS foxtk.cpp $<TARGET_OBJECTS:fox>)
    if(WIN32)
        add_compile_definitions(WIN32)
        add_compile_definitions(HAVE_VSSCANF)
        add_library(cfoxtk STATIC ${OBJS} )
        target_include_directories(cfoxtk PRIVATE ${FOX_PATH}/include ${FOX_INCLUDE_DIRS})
    else()
        add_library(cfoxtk STATIC ${OBJS} )
        if(UNIX)
            set(FOX_UNIX_DEFINITIONS
                HAVE_DIRENT_H
                HAVE_UNISTD_H
                HAVE_SYS_WAIT_H
                HAVE_SYS_TIME_H
                HAVE_SYS_SELECT_H
                HAVE_VSNPRINTF
                HAVE_VSSCANF
                HAVE_STRTOLL
                HAVE_STRTOULL
            )
            target_compile_definitions(fox PRIVATE ${FOX_UNIX_DEFINITIONS})
            target_compile_definitions(cfoxtk PRIVATE ${FOX_UNIX_DEFINITIONS})
        endif()
        target_compile_options(cfoxtk PRIVATE
            $<$<CXX_COMPILER_ID:MSVC>:/W4 /permissive->
            $<$<NOT:$<CXX_COMPILER_ID:MSVC>>:-Wall -Wextra -Wpedantic -O2>
        )
        target_include_directories(cfoxtk PRIVATE ${FOX_PATH}/include ${FOX_INCLUDE_DIRS})
    endif()
endif()
