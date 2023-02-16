cmake_minimum_required(VERSION 3.16)
project(luatic)

set(CMAKE_CXX_STANDARD 17)
set(EXECUTABLE_OUTPUT_PATH "${CMAKE_SOURCE_DIR}/bin")
set(LIBRARY_OUTPUT_PATH "${CMAKE_SOURCE_DIR}/bin/lib")

option(BUILD_TEST "build test modules" ON)

if (BUILD_TEST)
  find_package(GTest REQUIRED)
  find_package(Threads REQUIRED)
  include_directories(${GTEST_INCLUDE_DIRS})
endif()

include_directories(main)

add_library(chunk STATIC main/shared/chunk/binary_chunk.cc main/shared/chunk/binary_chunk.h main/shared/chunk/prototype.cc main/shared/chunk/prototype.h main/shared/chunk/literal.cc main/shared/chunk/literal.h main/shared/chunk/helper.cc main/shared/chunk/helper.h main/shared/instructions/instructions.cc main/shared/instructions/instructions.h)

add_executable(luna main/luna/main.cc)
add_executable(lunac main/lunac/main.cc)

target_link_libraries(luna ${LIBRARY_OUTPUT_PATH}/libchunk.a)
target_link_libraries(lunac ${LIBRARY_OUTPUT_PATH}/libchunk.a)

if (BUILD_TEST)
  add_executable(tests test/cpp/diff_tests.cc test/cpp/binary_chunk_tests.cc test/cpp/binary_pretty_printer.cc test/cpp/binary_pretty_printer.h)
  target_link_libraries(tests ${GTEST_BOTH_LIBRARIES})
  target_link_libraries(tests ${CMAKE_THREAD_LIBS_INIT})
  target_link_libraries(tests ${LIBRARY_OUTPUT_PATH}/libchunk.a)
endif()
