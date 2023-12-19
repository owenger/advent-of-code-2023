add_library(day6_lib STATIC src/day6.cpp)
target_include_directories(day6_lib PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")

# Add an executable for the day's problem
add_executable(day6 src/main.cpp)
target_link_libraries(day6 day6_lib)

add_executable(test_day6 test/test_main.cpp)
target_include_directories(test_day6 PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")
target_link_libraries(test_day6 day6_lib GTest::GTest GTest::Main)
add_test(NAME test_day6 COMMAND test_day6)

