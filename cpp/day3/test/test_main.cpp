#include "day3.h"
#include <gtest/gtest.h>

std::vector<std::string> TEST_SET = {
    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
    "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
    "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
    "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
    "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"};

// Write your test cases here
TEST(SampleTest, TrueIsTrue) { EXPECT_TRUE(true); }

TEST(BitmapTest, BasicTest) {
  std::vector<std::string> test_input = {"...#4", ".+34.."};
  std::vector<std::vector<bool>> output = day3::getBitMap(test_input);
  std::vector<std::vector<bool>> check_output = {
      {false, false, false, true, false}, {false, true, false, false, false}};

  for (int i = 0; i < output.size(); i++) {
    for (int j = 0; j < output[i].size(); j++) {
      EXPECT_EQ(output[i][j], check_output[i][j]);
    }
  }
}

TEST(BitmapTest, IsPart) {
  day3::BoolBitmap test_input = {
      {true, false, false, false, false, true, false},
      {false, false, false, false, false, false, false},
      {false, false, false, false, false, false, false}};

  bool test = day3::checkIfPart(test_input, 1, 2, 3);
  EXPECT_FALSE(test);
  test = day3::checkIfPart(test_input, 1, 1, 3);
  EXPECT_TRUE(test);
}

TEST(BitmapTest, WalkString) {
  day3::BoolBitmap test_input = {
      {true, false, false, false, false, true, false},
      {false, false, false, false, false, false, false},
      {false, false, false, false, false, false, false}};

  std::string test = "..12...";
  int value = day3::walkString(test, 1, test_input);
  EXPECT_EQ(value, 0);
  test = ".12....";
  value = day3::walkString(test, 1, test_input);
  EXPECT_EQ(value, 12);
}

TEST(BitmapTest, IsPairPart) {
  std::vector<std::string> test_input = {"...*4", "*+34.."};
  day3::PairBitmap bitmap = day3::getPairBitMap(test_input);
  EXPECT_EQ(0, bitmap[0][0].first);
  EXPECT_EQ(1, bitmap[0][3].first);
  EXPECT_EQ(1, bitmap[1][0].first);
}

TEST(BitmapTest, CheckIfPairPart) {
  day3::PairBitmap test_input = {
      {{1, 1}, {0, 0}, {0, 0}, {0, 0}, {0, 0}, {1, 1}, {0, 0}},
      {{0, 0}, {0, 0}, {0, 0}, {0, 0}, {0, 0}, {0, 0}, {0, 0}},
      {{0, 0}, {0, 0}, {0, 0}, {0, 0}, {0, 0}, {0, 0}, {0, 0}}};

  day3::checkIfPairPart(test_input, 12, 1, 1, 3);
  EXPECT_EQ(test_input[0][0].first, 2);
  EXPECT_EQ(test_input[0][0].second, 12);
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
