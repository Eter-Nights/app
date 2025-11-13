file(GLOB_RECURSE SOURCES "*.cpp")

add_library(src SHARED ${SOURCES})

target_include_directories(src PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR}
)

target_link_libraries(src gtest_main)

set_target_properties(src
        PROPERTIES
        ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
        RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
)


