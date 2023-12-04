#!/bin/bash

# Check if an argument is provided
if [ $# -eq 0 ]; then
    echo "No arguments provided. Usage: ./run_tests.sh <test_name>"
    exit 1
fi

# Navigate to the build directory
# Update this path if your build directory is located elsewhere
cd build

# Run CTest with the provided test name
ctest -R $1

