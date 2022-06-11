#include "gtest/gtest.h"
#include <unordered_map>
#include <unordered_set>

using namespace std;

class Solution {
public:
    static vector<int> twoOutOfThree(vector<int> &nums1, vector<int> &nums2, vector<int> &nums3) {
        unordered_map<int, int> mp = {};
        unordered_set<int> s1(nums1.begin(), nums1.end());
        for (int i: s1) {
            mp[i] += 1;
        }
        unordered_set<int> s2(nums2.begin(), nums2.end());
        for (int i: s2) {
            mp[i] += 1;
        }
        unordered_set<int> s3(nums3.begin(), nums3.end());
        for (int i: s3) {
            mp[i] += 1;
        }
        vector<int> res = {};
        for (pair<int, int> el: mp) {
            if (el.second >= 2) {
                res.push_back(el.first);
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
