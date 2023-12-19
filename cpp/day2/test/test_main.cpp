#include "day2.h"
#include <gtest/gtest.h>

std::vector<std::string> TEST_SET = {
    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
    "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
    "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
    "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
    "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"};

// Write your test cases here
TEST(SampleTest, TrueIsTrue) { EXPECT_TRUE(true); }

TEST(TestExtractColours, BasicTest) {
  day2::COLOUR s;
  for (auto &test_string : TEST_SET) {
    int blue, red, green;
    day2::getColourValues(test_string, blue, red, green);
  }
  EXPECT_TRUE(true);
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
