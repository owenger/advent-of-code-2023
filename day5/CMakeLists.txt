add_library(day5_lib STATIC src/day5.cpp)
target_include_directories(day5_lib PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")

# Add an executable for the day's problem
add_executable(day5 src/main.cpp)
target_link_libraries(day5 day5_lib)

add_executable(test_day5 test/test_main.cpp)
target_include_directories(test_day5 PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")
target_link_libraries(test_day5 day5_lib GTest::GTest GTest::Main)
add_test(NAME test_day5 COMMAND test_day5)

