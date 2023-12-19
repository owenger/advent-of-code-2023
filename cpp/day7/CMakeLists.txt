add_library(day7_lib STATIC src/day7.cpp)
target_include_directories(day7_lib PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")

# Add an executable for the day's problem
add_executable(day7 src/main.cpp)
target_link_libraries(day7 day7_lib)

add_executable(test_day7 test/test_main.cpp)
target_include_directories(test_day7 PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")
target_link_libraries(test_day7 day7_lib GTest::GTest GTest::Main)
add_test(NAME test_day5 COMMAND test_day7)

