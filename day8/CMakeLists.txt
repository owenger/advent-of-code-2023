add_library(day8_lib STATIC src/day8.cpp)
target_include_directories(day8_lib PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")

# Add an executable for the day's problem
add_executable(day8 src/main.cpp)
target_link_libraries(day8 day8_lib)

add_executable(test_day8 test/test_main.cpp)
target_include_directories(test_day8 PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")
target_link_libraries(test_day8 day8_lib GTest::GTest GTest::Main)
add_test(NAME test_day8 COMMAND test_day8)

