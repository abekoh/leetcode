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
        if (root == nullptr) {
            return true;
        }
        return helper(LONG_MIN, root->val, root->left) && helper(root->val, LONG_MAX, root->right);
    }

    static bool helper(long min, long max, TreeNode *tree) {
        if (tree == nullptr) {
            return true;
        }
        if (tree->val <= min || tree->val >= max) {
            return false;
        }
        return helper(min, (long) tree->val, tree->left) && helper((long) tree->val, max, tree->right);
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

TEST(Tests, Example03) {
    auto input = new TreeNode(2, new TreeNode(2), new TreeNode(2));
    EXPECT_FALSE(Solution::isValidBST(input));
}

TEST(Tests, Example04) {
    auto input = new TreeNode(5, new TreeNode(4), new TreeNode(6, new TreeNode(3), new TreeNode(7)));
    EXPECT_FALSE(Solution::isValidBST(input));
}

TEST(Tests, Example05) {
    auto input = new TreeNode(3, new TreeNode(1, new TreeNode(0), new TreeNode(2)),
                              new TreeNode(5, new TreeNode(4), new TreeNode(6)));
    EXPECT_TRUE(Solution::isValidBST(input));
}

TEST(Tests, Example06) {
    EXPECT_TRUE(Solution::isValidBST(new TreeNode(INT_MIN, nullptr, new TreeNode(INT_MAX))));
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
