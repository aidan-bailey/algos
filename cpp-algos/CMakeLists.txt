cmake_minimum_required(VERSION 3.11)
project(ALGOS_LIB)

option(DEV "Generate compiler commands and set logging to debug." OFF)

set(ALGOS_VERSION_MAJOR 0)
set(ALGOS_VERSION_MINOR 1)
set(ALGOS_VERSION ${ALGOS_VERSION_MAJOR}.${ALGOS_VERSION_MINOR})
set(CMAKE_INCLUDE_CURRENT_DIR ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)
if(DEV)
  set(CMAKE_EXPORT_COMPILE_COMMANDS True)
  set(CMAKE_BUILD_TYPE Debug)
endif()

if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif()

set(CMAKE_CXX_FLAGS "-Wall -Wextra")
set(CMAKE_CXX_FLAGS_DEBUG "-g")
set(CMAKE_CXX_FLAGS_RELEASE "-O2")

include(GenerateExportHeader)

###################
# TEST EXECUTABLE #
###################

include(FetchContent)

fetchcontent_declare(
  Catch2
  GIT_REPOSITORY https://github.com/catchorg/Catch2.git
  GIT_TAG v3.0.1
)

fetchcontent_makeavailable(Catch2)

add_executable(tests test/test.cpp)
target_include_directories(
  tests PUBLIC
  include
)

target_link_libraries(tests PUBLIC Catch2::Catch2WithMain)
