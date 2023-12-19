#include "day9.h"
#include <gtest/gtest.h>

// Write your test cases here
TEST(SampleTest, TrueIsTrue) { EXPECT_TRUE(true); }

TEST(Day9Tests, WalkTree) {
  std::vector<int64_t> test_vector = {10, 13, 16, 21, 30, 45};
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
