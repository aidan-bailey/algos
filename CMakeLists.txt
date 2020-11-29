cmake_minimum_required( VERSION 3.2 )

project( algos )

set( CMAKE_EXPORT_COMPILE_COMMANDS ON )

add_executable( test test/test.cpp )

target_include_directories( test PUBLIC test include)
