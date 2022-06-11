#include "gtest/gtest.h"
#include <unordered_map>
#include <unordered_set>

using namespace std;

// https://leetcode.com/problems/two-out-of-three/discuss/1515973/C%2B%2B-Super-Simple-and-Easy-Solution-Explained
class Solution {
public:
    static vector<int> twoOutOfThree(vector<int> &nums1, vector<int> &nums2, vector<int> &nums3) {
        unordered_map<int, char> mp;
        for (int el: nums1) {
            mp[el] |= 1;
        }
        for (int el: nums2) {
            mp[el] |= 2;
        }
        for (int el: nums3) {
            mp[el] |= 4;
        }
        vector<int> res = {};
        for (auto [num, cnt]: mp) {
            if ((!!(cnt & 1) + !!(cnt & 2) + !!(cnt & 4)) >= 2) {
                res.push_back(num);
            }
        }
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
