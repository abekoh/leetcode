#include "gtest/gtest.h"

struct ListNode {
    int val;
    ListNode *next;

    ListNode() : val(0), next(nullptr) {}

    ListNode(int x) : val(x), next(nullptr) {}

    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

bool operator==(const ListNode &lh, const ListNode &rh) {
    if (lh.val != rh.val) {
        return false;
    }
    if (lh.next != nullptr && rh.next != nullptr) {
        return operator==(*lh.next, *rh.next);
    }
    if (lh.next == nullptr && rh.next == nullptr) {
        return true;
    }
    return false;
}

class Solution {
public:
    ListNode *mergeTwoLists(ListNode *list1, ListNode *list2) {
        auto res = new ListNode();
        auto cur = res;
        while (true) {
            if (list1 == nullptr) {
                cur->next = list2;
                break;
            }
            if (list2 == nullptr) {
                cur->next = list1;
                break;
            }
            if (list1->val < list2->val) {
                cur->next = new ListNode(list1->val);
                cur = cur->next;
                list1 = list1->next;
            } else {
                cur->next = new ListNode(list2->val);
                cur = cur->next;
                list2 = list2->next;
            }
        }
        return res->next;
    }
};

TEST(GameFrameworks, Placeholder) {
    auto sol = new Solution;
    auto actual = sol->mergeTwoLists(new ListNode(1, new ListNode(2, new ListNode(4))),
                                     new ListNode(1, new ListNode(3, new ListNode(4))));
    auto expected = new ListNode(1,
                                 new ListNode(1, new ListNode(2, new ListNode(3, new ListNode(4, new ListNode(4))))));
    EXPECT_EQ(*actual, *expected);
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
