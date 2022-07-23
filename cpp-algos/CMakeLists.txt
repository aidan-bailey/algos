cmake_minimum_required( VERSION 3.2 )

project( algos )

set( CMAKE_EXPORT_COMPILE_COMMANDS ON )

include_directories( test include )

add_executable( test test/test.cpp )
