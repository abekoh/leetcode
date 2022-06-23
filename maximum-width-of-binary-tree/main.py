from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def widthOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0
        queue = [(root, 1)]
        res = 0
        while len(queue) > 0:
            cnt = len(queue)
            begin, end = queue[0][1], queue[-1][1]
            res = max(res, end - begin + 1)
            for i in range(0, cnt):
                node, idx = queue.pop(0)
                if node.left is not None:
                    queue.append((node.left, idx * 2))
                if node.right is not None:
                    queue.append((node.right, idx * 2 + 1))
        return res


if __name__ == "__main__":
    sol = Solution()
    actual = sol.widthOfBinaryTree(
        TreeNode(
            1, TreeNode(3, TreeNode(5), TreeNode(3)), TreeNode(2, None, TreeNode(9))
        )
    )
    assert actual == 4
