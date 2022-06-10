#include "gtest/gtest.h"

struct ListNode {
    int val;
    ListNode *next;

    ListNode() : val(0), next(nullptr) {}

    ListNode(int x) : val(x), next(nullptr) {}

    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
public:
    ListNode *mergeTwoLists(ListNode *list1, ListNode *list2) {
        auto node = new ListNode();
        return node;
    }
};

TEST(GameFrameworks, Placeholder) {
    auto sol = new Solution;
    EXPECT_EQ(sol->mergeTwoLists(new ListNode(1, new ListNode(2, new ListNode(4))),
                                 new ListNode(1, new ListNode(3, new ListNode(4)))),
              new ListNode(1, new ListNode(1, new ListNode(2, new ListNode(3, new ListNode(4, new ListNode(4))))))
    );
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
