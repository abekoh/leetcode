from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def countOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0
        queue = [root]
        res = 0
        while len(queue) > 0:
            res = max(res, len(queue))
            new_queue = []
            for node in queue:
                if node.left is not None:
                    new_queue.append(node.left)
                if node.right is not None:
                    new_queue.append(node.right)
            queue = new_queue
        return res


if __name__ == "__main__":
    sol = Solution()
    actual = sol.countOfBinaryTree(
        TreeNode(
            1, TreeNode(3, TreeNode(5), TreeNode(3)), TreeNode(2, None, TreeNode(9))
        )
    )
    assert actual == 3

    sol = Solution()
    actual = sol.countOfBinaryTree(
        TreeNode(
            1,
            TreeNode(3, TreeNode(5), TreeNode(3)),
            TreeNode(2, TreeNode(1), TreeNode(9)),
        )
    )
    assert actual == 4

    actual = sol.countOfBinaryTree(
        TreeNode(
            1,
            TreeNode(3, None, TreeNode(3)),
            TreeNode(2, None, TreeNode(9)),
        )
    )
    assert actual == 2

    actual = sol.countOfBinaryTree(
        TreeNode(
            1,
            TreeNode(3, TreeNode(9), TreeNode(3, TreeNode(1), TreeNode(2))),
            TreeNode(2, TreeNode(1), TreeNode(9, TreeNode(8))),
        )
    )
    assert actual == 4
