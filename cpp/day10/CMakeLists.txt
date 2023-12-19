add_library(day10_lib STATIC src/day10.cpp)
target_include_directories(day10_lib PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")

# Add an executable for the day's problem
add_executable(day10 src/main.cpp)
target_link_libraries(day10 day10_lib)

add_executable(test_day10 test/test_main.cpp)
target_include_directories(test_day10 PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")
target_link_libraries(test_day10 day10_lib GTest::GTest GTest::Main)
add_test(NAME test_day10 COMMAND test_day10)

