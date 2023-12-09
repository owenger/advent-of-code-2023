cmake_minimum_required(VERSION 3.10)
project(AdventOfCode)

# Specify the C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

find_package(GTest REQUIRED)
enable_testing()

# Add subdirectories for each day
add_subdirectory(day1)
add_subdirectory(day2)
add_subdirectory(day3)
add_subdirectory(day4)


# ... add other days as needed

