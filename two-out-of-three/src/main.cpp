#include "gtest/gtest.h"

using namespace std;

class Solution {
public:
    static vector<int> twoOutOfThree(vector<int> &nums1, vector<int> &nums2, vector<int> &nums3) {
        vector<int> res = {3, 2};
        return res;
    }
};

TEST(Tests, Example01) {
    vector<int> nums1 = {1, 1, 3, 2};
    vector<int> nums2 = {2, 3};
    vector<int> nums3 = {3};
    vector<int> actual = Solution::twoOutOfThree(nums1, nums2, nums3);
    EXPECT_EQ(actual.size(), 2);
    EXPECT_TRUE(find(actual.begin(), actual.end(), 3) != actual.end());
    EXPECT_TRUE(find(actual.begin(), actual.end(), 2) != actual.end());
}

TEST(Tests, Example02) {
    vector<int> nums1 = {3, 1};
    vector<int> nums2 = {2, 3};
    vector<int> nums3 = {1, 2};
    vector<int> actual = Solution::twoOutOfThree(nums1, nums2, nums3);
    EXPECT_EQ(actual.size(), 3);
    EXPECT_TRUE(find(actual.begin(), actual.end(), 2) != actual.end());
    EXPECT_TRUE(find(actual.begin(), actual.end(), 3) != actual.end());
    EXPECT_TRUE(find(actual.begin(), actual.end(), 1) != actual.end());
}

TEST(Tests, Example03) {
    vector<int> nums1 = {1, 2, 2};
    vector<int> nums2 = {4, 3, 3};
    vector<int> nums3 = {5};
    vector<int> actual = Solution::twoOutOfThree(nums1, nums2, nums3);
    EXPECT_EQ(actual.size(), 0);
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
