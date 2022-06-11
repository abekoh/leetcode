#include "gtest/gtest.h"
#include <unordered_set>

using namespace std;

// https://leetcode.com/problems/number-of-equal-count-substrings/discuss/1570772/Sliding-Window-O(26n)
class Solution {
public:
    int equalCountSubstrings(string s, int count) {
        int res = 0;
        int max_unique = unordered_set(begin(s), end(s)).size();
        for (int unique = 1; unique <= max_unique; ++unique) {
            int cnt[26] = {};
            int len = count * unique;
            int has_count = 0;
            for (int i = 0; i < s.size(); ++i) {
                if (++cnt[s[i] - 'a'] == count) {
                    ++has_count;
                }
                if (i >= len && --cnt[s[i - len] - 'a'] == count - 1) {
                    --has_count;
                }
                res += has_count == unique;
            }
        }
        return res;
    }
};

TEST(Tests, Example01) {
    auto sol = new Solution;
    EXPECT_EQ(sol->equalCountSubstrings("aaabcbbcc", 3), 3);
}

TEST(Tests, Example02) {
    auto sol = new Solution;
    EXPECT_EQ(sol->equalCountSubstrings("abcd", 2), 0);
}

TEST(Tests, Example03) {
    auto sol = new Solution;
    EXPECT_EQ(sol->equalCountSubstrings("a", 5), 0);
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
