#include "gtest/gtest.h"

class Solution {
public:
    int sum(int a, int b) {
        return a + b;
    }
};

TEST(GameFrameworks, Placeholder) {
    auto sol = new Solution;
    EXPECT_EQ(sol->sum(2, 3), 5);
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
