#include "gtest/gtest.h"

using namespace std;

class Solution {
public:
    static int add(int a, int b) {
        return a + b;
    }
};

TEST(Tests, Example01) {
    EXPECT_EQ(Solution::add(1, 2), 3);
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
