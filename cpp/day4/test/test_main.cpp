#include "day4.h"
#include <gtest/gtest.h>

std::string TEST_INPUT = "Card   1: 99 46 62 | 83 99 62";

day4::StringVector TEST_VECTOR = {
    "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
    "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
    "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
    "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
    "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
    "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"};

// Write your test cases here
TEST(SampleTest, TrueIsTrue) { EXPECT_TRUE(true); }

TEST(Day4Tests, ParseTest) {
  std::vector<int> ticket_numbers;
  std::set<int> winning_numbers;
  day4::parseInputString(TEST_INPUT, ticket_numbers, winning_numbers);
  std::cout << "\nVector: ";
  for (auto &el : ticket_numbers) {
    std::cout << " " << std::to_string(el) << ",";
  }
  EXPECT_EQ(ticket_numbers[0], 99);
  EXPECT_EQ(winning_numbers.count(83), 1);
  EXPECT_EQ(winning_numbers.count(99), 1);
}

TEST(Day4Tests, NextTest) {
  std::vector<int> ticket_numbers;
  std::set<int> winning_numbers;
  day4::parseInputString(TEST_INPUT, ticket_numbers, winning_numbers);
  int sum = day4::getScore(ticket_numbers, winning_numbers);
  std::cout << "\n" << std::to_string(sum) << "\n";
  EXPECT_EQ(sum, 2);
}

TEST(Day4Tests, Part2Test) {
  int result = day4::walkBackVector(TEST_VECTOR);
  EXPECT_EQ(result, 30);
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
