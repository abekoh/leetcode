#include "gtest/gtest.h"
#include <cmath>

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
        if (root->left == nullptr || root->right == nullptr) {
            return 1;
        }
        auto l = root->left;
        auto r = root->right;
        int res = 2;
        int l_count = -1;
        int r_count = -1;
        while (true) {
            if (l_count >= 0) {
                res += (int) pow(2, l_count);
            }
            if (r_count >= 0) {
                res += (int) pow(2, r_count);
            }
            if (l->left != nullptr) {
                l_count++;
                l = l->left;
            } else if (l->right != nullptr) {
                l = l->right;
            } else {
                break;
            }
            if (r->right != nullptr) {
                r_count++;
                r = r->right;
            } else if (r->left != nullptr) {
                r = r->left;
            } else {
                break;
            }
        }
        return res;
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

TEST(Tests, Example04) {
    auto input = new TreeNode(1);
    EXPECT_EQ(Solution::widthOfBinaryTree(input), 1);
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
