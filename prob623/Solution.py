from typing import List, Optional
from collections import deque

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def addOneRow(self, root: Optional[TreeNode], val: int, depth: int) -> Optional[TreeNode]:
        if depth == 1:
            return TreeNode(val, root)
        
        bfs_queue = [root]
        cd = 2

        while cd < depth:
            cd += 1
            tmp = []
            for el in bfs_queue:
                if el.left is not None:
                    tmp.append(el.left)
                if el.right is not None:
                    tmp.append(el.right)
            bfs_queue = tmp
        
        for el in bfs_queue:
            el.left = TreeNode(val, el.left)
            el.right = TreeNode(val, None, el.right)

        return root


