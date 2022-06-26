#include "gtest/gtest.h"

using namespace std;

// https://leetcode.com/problems/count-number-of-texts/discuss/2021877/C%2B%2B-DP-Solution-O(N)
class Solution {
public:
    static int countTexts(string pressedKeys) {
        int MOD = 1000000007;
        int len = pressedKeys.length();
        vector<vector<int>> preDP = {{0, 1, 2, 4},
                                     {0, 1, 2, 4, 8}};
        int maxFreq[10] = {};
        vector<int> freq[10];

        int curr = -1;
        int count = 0;

        for (int i = 0; i < len; i++) {
            if (pressedKeys[i] - '0' == curr) {
                count++;
                maxFreq[curr] = max(maxFreq[curr], count);
                freq[curr].back()++;
            } else {
                curr = pressedKeys[i] - '0';
                count = 1;
                maxFreq[curr] = max(maxFreq[curr], count);
                freq[curr].push_back(count);
            }
        }

        vector<long long> dp(len + 1);
        unsigned long long totalCount = 1L;
        int type;
        for (int i = 0; i < 10; i++) {
            if (maxFreq[i] == 0) continue;

            type = (i == 7 || i == 9) ? 1 : 0;

            for (int j = 0; j <= maxFreq[i] && j <= 3 + type; j++) {
                dp[j] = preDP[type][j];
            }

            for (int j = 4 + type; j <= maxFreq[i]; j++) {
                dp[j] = (dp[j - 1] + dp[j - 2] + dp[j - 3] + (type ? dp[j - 4] : 0) + MOD) % MOD;
            }

            for (int j = 0; j < freq[i].size(); j++) {
                totalCount = (totalCount % MOD * dp[freq[i][j]] % MOD) % MOD;
            }
        }

        return (int) totalCount;
    }
};


TEST(Tests, Example01) {
    EXPECT_EQ(Solution::countTexts("22233"), 8);
}

TEST(Tests, Example02) {
    EXPECT_EQ(Solution::countTexts("222222222222222222222222222222222222"), 82876089);
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
