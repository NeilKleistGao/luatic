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
include_directories(libraries)
include(backends/CMakeLists.txt)

add_executable(luatic main/luatic/main.cc)
target_link_libraries(luatic ${CMAKE_SOURCE_DIR}/bin/lib/libbackend.a)

if (BUILD_TEST)
  add_executable(tests test/cpp/diff_tests.cc)
  target_link_libraries(tests ${GTEST_BOTH_LIBRARIES})
  target_link_libraries(tests ${CMAKE_THREAD_LIBS_INIT})
  target_link_libraries(tests ${CMAKE_SOURCE_DIR}/bin/lib/libbackend.a)
endif()
