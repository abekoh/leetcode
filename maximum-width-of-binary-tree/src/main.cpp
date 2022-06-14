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
    static int widthOfBinaryTree(TreeNode *root) {
        return 0;
    }
};

TEST(Tests, Example01) {
    auto input = new TreeNode(1, new TreeNode(3, new TreeNode(5), new TreeNode(3)),
                              new TreeNode(2, nullptr, new TreeNode(9)));
    EXPECT_EQ(Solution::widthOfBinaryTree(input), 4);
}

TEST(Tests, Example02) {
    auto input = new TreeNode(1, new TreeNode(3, new TreeNode(5, new TreeNode(6), nullptr), nullptr),
                              new TreeNode(2, nullptr, new TreeNode(9, new TreeNode(7),
                                                                    nullptr)));
    EXPECT_EQ(Solution::widthOfBinaryTree(input), 7);
}

TEST(Tests, Example03) {
    auto input = new TreeNode(1, new TreeNode(3, new TreeNode(5), nullptr), new TreeNode(2));
    EXPECT_EQ(Solution::widthOfBinaryTree(input), 2);
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
