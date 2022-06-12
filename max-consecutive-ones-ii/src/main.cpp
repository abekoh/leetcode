#include "gtest/gtest.h"

using namespace std;

class Solution {
public:
    static int findMaxConsecutiveOnes(vector<int> &nums) {
        int con = nums[0];
        int fcon = 1;
        int res = fcon;
        for (int i = 1; i < nums.size(); ++i) {
            if (nums[i] == 0) {
                res = max(res, fcon);
                fcon = con + 1;
                con = 0;
            } else {
                ++con;
                ++fcon;
            }
        }
        res = max(res, fcon);
        return res;
    }
};

TEST(Tests, Example01) {
    vector<int> nums = {1, 0, 1, 1, 0};
    EXPECT_EQ(Solution::findMaxConsecutiveOnes(nums), 4);
}

TEST(Tests, Example02) {
    vector<int> nums = {1, 0, 1, 1, 0, 1};
    EXPECT_EQ(Solution::findMaxConsecutiveOnes(nums), 4);
}

TEST(Tests, Example03) {
    vector<int> nums = {1, 1, 0, 1};
    EXPECT_EQ(Solution::findMaxConsecutiveOnes(nums), 4);
}

TEST(Tests, Example04) {
    vector<int> nums = {1, 0};
    EXPECT_EQ(Solution::findMaxConsecutiveOnes(nums), 2);
}

TEST(Tests, Example05) {
    vector<int> nums = {0};
    EXPECT_EQ(Solution::findMaxConsecutiveOnes(nums), 1);
}

TEST(Tests, Example06) {
    vector<int> nums = {0, 0};
    EXPECT_EQ(Solution::findMaxConsecutiveOnes(nums), 1);
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
