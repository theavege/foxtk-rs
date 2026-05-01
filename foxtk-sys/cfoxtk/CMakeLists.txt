cmake_minimum_required(VERSION 3.31)

project(cfoxtk CXX)

add_compile_definitions(WIN32)

set(FOX_PATH "$ENV{FOX_PATH}")
file(GLOB FOX_SOURCE ${FOX_PATH}/lib/*)
link_directories(fox  PRIVATE ../use/winsdk-7/v7.1A/Lib/x64)

add_library(fox OBJECT ${FOX_SOURCE})
#~ find_library (
    #~ WINSDK_SEVEN
    #~ NAMES AdvAPI32 ComCtl32 Gdi32 Imm32 WinSpool WSock32
    #~ PATHS ../use/winsdk-7/v7.1A/Lib/x64
    #~ REQUIRED
#~ )
#~ link_libraries(fox PRIVATE ${WINSDK_SEVEN})

add_library(AdvAPI32 STATIC IMPORTED)
set_property(TARGET AdvAPI32 PROPERTY IMPORTED_LOCATION ../use/winsdk-7/v7.1A/Lib/x64/AdvAPI32.lib)
link_libraries(fox AdvAPI32)

add_library(ComCtl32 STATIC IMPORTED)
set_property(TARGET ComCtl32 PROPERTY IMPORTED_LOCATION ../use/winsdk-7/v7.1A/Lib/x64/ComCtl32.lib)
link_libraries(fox ComCtl32)

add_library(Gdi32 STATIC IMPORTED)
set_property(TARGET Gdi32 PROPERTY IMPORTED_LOCATION ../use/winsdk-7/v7.1A/Lib/x64/Gdi32.lib)
link_libraries(fox Gdi32)

add_library(Imm32 STATIC IMPORTED)
set_property(TARGET Imm32 PROPERTY IMPORTED_LOCATION ../use/winsdk-7/v7.1A/Lib/x64/Imm32.lib)
link_libraries(fox Imm32)

add_library(WinSpool STATIC IMPORTED)
set_property(TARGET WinSpool PROPERTY IMPORTED_LOCATION ../use/winsdk-7/v7.1A/Lib/x64/WinSpool.lib)
link_libraries(fox WinSpool)

add_library(WSock32 STATIC IMPORTED)
set_property(TARGET WSock32 PROPERTY IMPORTED_LOCATION ../use/winsdk-7/v7.1A/Lib/x64/WSock32.lib)
link_libraries(fox WSock32)

add_library(cfoxtk STATIC $<TARGET_OBJECTS:fox> foxtk.cpp)
include_directories(cfoxtk PRIVATE ${FOX_PATH}/include)

