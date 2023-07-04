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
include_directories(backends/config)
include(backends/CMakeLists.txt)

list(APPEND luatic_source
        main/lua/lua_vm.cc
        main/luatic/lexer.cc
        main/luatic/parser.cc
        main/luatic/tokens.cc
        main/luatic/ast.cc)

add_executable(luatic main/main.cc ${luatic_source})
target_link_libraries(luatic ${CMAKE_SOURCE_DIR}/bin/lib/libbackend.a ${CMAKE_DL_LIBS})

if (BUILD_TEST)
  add_executable(tests test/cpp/diff_tests.cc test/cpp/pretty_printer.cc ${luatic_source})
  target_link_libraries(tests ${GTEST_BOTH_LIBRARIES} ${CMAKE_THREAD_LIBS_INIT})
  target_link_libraries(tests ${CMAKE_SOURCE_DIR}/bin/lib/libbackend.a ${CMAKE_DL_LIBS})
endif()
