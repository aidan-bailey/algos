#!/usr/bin/env bash
cmake --version > /dev/null 2> /dev/null && flag=true || flag=false
if [ "$flag" = false ]; then
    echo "Cmake install not found."
    exit 1
fi 
echo "Entering cmake dir..."
cd build
echo "Running cmake command..."
cmake .. > /dev/null
echo "Building project with make..."
make -j > /dev/null
echo "Running tests..."
./tests -s
cd ..
