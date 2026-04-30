cmake_minimum_required(VERSION 3.31)

project(cfoxtk CXX)

add_compile_definitions(WIN32)

set(FOX_PATH "$ENV{FOX_PATH}")
file(GLOB FOX_SOURCE ${FOX_PATH}/lib/*)

add_library(fox OBJECT ${FOX_SOURCE})
target_include_directories(fox PRIVATE ${FOX_PATH}/include)

add_library(cfoxtk STATIC $<TARGET_OBJECTS:fox> foxtk.cpp)
target_include_directories(cfoxtk PRIVATE ${FOX_PATH}/include)

