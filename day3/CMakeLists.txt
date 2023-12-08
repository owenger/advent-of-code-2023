add_library(day3_lib STATIC src/day3.cpp)
target_include_directories(day3_lib PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")

# Add an executable for the day's problem
add_executable(day3 src/main.cpp)
target_link_libraries(day3 day3_lib)

add_executable(test_day3 test/test_main.cpp)
target_include_directories(test_day3 PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")
target_link_libraries(test_day3 day3_lib GTest::GTest GTest::Main)
add_test(NAME test_day3 COMMAND test_day3)

