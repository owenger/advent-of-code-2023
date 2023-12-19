add_library(day9_lib STATIC src/day9.cpp)
target_include_directories(day9_lib PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")

# Add an executable for the day's problem
add_executable(day9 src/main.cpp)
target_link_libraries(day9 day9_lib)

add_executable(test_day9 test/test_main.cpp)
target_include_directories(test_day9 PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")
target_link_libraries(test_day9 day9_lib GTest::GTest GTest::Main)
add_test(NAME test_day9 COMMAND test_day9)

