#include "gtest/gtest.h"
#include "unordered_map"

using namespace std;

class Solution {
public:
    static int maxOperations(vector<int> &nums, int k) {
        unordered_map<int, int> mp;
        for (int n: nums) {
            mp[n]++;
        }
        unordered_map<int, bool> searched;
        int res = 0;
        for (int i = 0; i < nums.size(); i++) {
            int cur = nums[i];
            if (cur > k / 2) {
                continue;
            }
            if (searched[cur]) {
                continue;
            }
            searched[cur] = true;
            int comp = k - nums[i];
            if (k % 2 == 0 && (cur == comp)) {
                res += mp[cur] / 2;
            } else {
                res += min(mp[cur], mp[comp]);
            }
        }
        return res;
    }
};

TEST(Tests, Example01) {
    vector<int> inp = {1, 2, 3, 4};
    EXPECT_EQ(Solution::maxOperations(inp, 5), 2);
}

TEST(Tests, Example02) {
    vector<int> inp = {3, 1, 3, 4, 3};
    EXPECT_EQ(Solution::maxOperations(inp, 6), 1);
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
