cmake_minimum_required(VERSION 4.0)
project(app)

set(CMAKE_CXX_STANDARD 20)

add_subdirectory(build/googletest)
add_subdirectory(src)

add_executable(test_cpp main.cpp)

target_link_libraries(test_cpp gtest_main src)

set_target_properties(test_cpp
        PROPERTIES
        ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
        RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
)