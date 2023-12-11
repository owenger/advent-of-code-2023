#include "day7.h"
#include <gtest/gtest.h>

// Write your test cases here
TEST(SampleTest, TrueIsTrue) { EXPECT_TRUE(true); }

TEST(Day7Tests, Classs) {
  day7::Hand hand0{"AAAAA", 0};
  hand0.evaluateValue();
  EXPECT_EQ(hand0.hand_type_, day7::HandType::a5);
  day7::Hand hand1{"AA888", 0};
  hand1.evaluateValue();
  EXPECT_EQ(hand1.hand_type_, day7::HandType::fh);
  EXPECT_TRUE(hand0.isHigherThan(hand1));
}

TEST(Day7Tests, Reversed) {
  day7::Hand hand0{"AAAAA", 0, true};
  hand0.evaluateValue();
  EXPECT_EQ(hand0.hand_type_, day7::HandType::a5);
  day7::Hand hand1{"JAJAJ", 0, true};
  hand1.evaluateValue();
  EXPECT_EQ(hand1.hand_type_, day7::HandType::a5);
  EXPECT_TRUE(hand0.isHigherThan(hand1));
}

TEST(Day7Tests, SpecificHand) {
  day7::Hand hand0{"KTJJT", 0, true};
  hand0.evaluateValue();
  EXPECT_EQ(hand0.hand_type_, day7::HandType::a4);
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
