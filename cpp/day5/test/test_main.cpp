#include "day5.h"
#include <gtest/gtest.h>

// Write your test cases here
TEST(SampleTest, TrueIsTrue) { EXPECT_TRUE(true); }

TEST(Day5Tests, Map) {
  day5::RecipeMap rm;
  rm.addMap({10, 0, 5});
  rm.addMap({80, 21, 20});
  int res0 = rm.map(1);
  EXPECT_EQ(res0, 11);
  int res1 = rm.map(32);
  EXPECT_EQ(res1, 91);
  int res2 = rm.map(100);
  EXPECT_EQ(res2, 100);
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
