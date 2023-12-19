# Add an executable for the day's problem
add_executable(day1 main.cpp)

# If there are any include directories, specify them
target_include_directories(day1 PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")

