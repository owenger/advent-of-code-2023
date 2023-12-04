add_library(day2_lib STATIC src/day2.cpp)
target_include_directories(day2_lib PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")

# Add an executable for the day's problem
add_executable(day2 src/main.cpp)
target_link_libraries(day2 day2_lib)

add_executable(test_day2 test/test_main.cpp)
target_include_directories(test_day2 PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")
target_link_libraries(test_day2 day2_lib GTest::GTest GTest::Main)
add_test(NAME test_day2 COMMAND test_day2)

