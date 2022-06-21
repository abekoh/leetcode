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
    static bool isValidBST(TreeNode *root) {

    }
};

TEST(Tests, Example01) {
    auto input = new TreeNode(2, new TreeNode(1), new TreeNode(3));
    EXPECT_TRUE(Solution::isValidBST(input));
}

TEST(Tests, Example02) {
    auto input = new TreeNode(5, new TreeNode(1), new TreeNode(4, new TreeNode(3), new TreeNode(6)));
    EXPECT_FALSE(Solution::isValidBST(input));
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
