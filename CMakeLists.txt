cmake_minimum_required(VERSION 3.25)
project(test)

set(CMAKE_CXX_STANDARD 17)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/bin")

if (UNIX AND NOT APPLE)
    set(LINUX ON)
endif ()

include_directories("third_party/font_kit/include")
link_directories("third_party/font_kit/target/debug")

add_executable(test main.cpp)
if (WIN32)
    target_link_libraries(test font_kit_wrapper.dll.lib)
#    target_link_libraries(test font_kit_wrapper)
elseif (LINUX)
    target_link_libraries(test font_kit_wrapper.so)
endif ()
