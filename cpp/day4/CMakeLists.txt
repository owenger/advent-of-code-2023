add_library(day4_lib STATIC src/day4.cpp)
target_include_directories(day4_lib PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")

# Add an executable for the day's problem
add_executable(day4 src/main.cpp)
target_link_libraries(day4 day4_lib)

add_executable(test_day4 test/test_main.cpp)
target_include_directories(test_day4 PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")
target_link_libraries(test_day4 day4_lib GTest::GTest GTest::Main)
add_test(NAME test_day4 COMMAND test_day4)

