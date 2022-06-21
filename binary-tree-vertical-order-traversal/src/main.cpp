#include "gtest/gtest.h"

using namespace std;


struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;

    TreeNode() : val(0), left(nullptr), right(nullptr) {}

    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}

    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class Solution {
public:
    static vector<vector<int>> verticalOrder(TreeNode *root) {
        vector<vector<int>> result;
        return result;
    }
};

TEST(Tests, Example01) {
    auto input = new TreeNode(3, new TreeNode(9), new TreeNode(20, new TreeNode(15), new TreeNode(7)));
    vector<vector<int>> expected = {{9},
                                    {3, 15},
                                    {20},
                                    {7}};
    EXPECT_EQ(Solution::verticalOrder(input), expected);
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
