#!/bin/bash

# Directory for out-of-source build
BUILD_DIR="build"

# Check if build directory exists, if not create it
if [ ! -d "$BUILD_DIR" ]; then
    echo "Creating build directory..."
    mkdir "$BUILD_DIR"
fi

# Navigate to the build directory
cd "$BUILD_DIR"

# Run CMake with the option to export compile commands
echo "Running CMake to generate compile_commands.json..."
cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=ON ..

# Check if compile_commands.json was created
if [ ! -f "compile_commands.json" ]; then
    echo "Error: compile_commands.json was not generated."
    exit 1
fi

# Copy compile_commands.json to the project root
echo "Copying compile_commands.json to the project root..."
cp compile_commands.json ../

# Return to the original directory
cd ..

echo "Update complete."


