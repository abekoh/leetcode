#include "gtest/gtest.h"

using namespace std;

class Solution {
public:
    static int fib(int n) {
        if (n <= 1) {
            return n;
        }
        int dp[n + 1];
        dp[0] = 0;
        dp[1] = 1;
        for (int i = 2; i <= n; ++i) {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        return dp[n];
    }
};

TEST(Tests, Example01) {
    EXPECT_EQ(Solution::fib(2), 1);
}

TEST(Tests, Example02) {
    EXPECT_EQ(Solution::fib(3), 2);
}

TEST(Tests, Example03) {
    EXPECT_EQ(Solution::fib(4), 3);
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
