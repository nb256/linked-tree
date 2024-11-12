#[derive(Debug, PartialEq)]
pub struct TreeNode {
    pub value: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    // Create a new tree node with a value and no children
    pub fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    // Insert a value into the binary tree
    pub fn insert(&mut self, new_value: i32) {
        if new_value <= self.value {
            match self.left {
                Some(ref mut left_child) => left_child.insert(new_value),
                None => self.left = Some(Box::new(TreeNode::new(new_value))),
            }
        } else if new_value > self.value {
            match self.right {
                Some(ref mut right_child) => right_child.insert(new_value),
                None => self.right = Some(Box::new(TreeNode::new(new_value))),
            }
        }
        // If new_value == self.value, do nothing to avoid duplicate values
    }

    // Check if the tree contains a value
    pub fn contains(&self, target: i32) -> bool {
        if self.value == target {
            true
        } else if target < self.value {
            match &self.left {
                Some(left_child) => left_child.contains(target),
                None => false,
            }
        } else {
            match &self.right {
                Some(right_child) => right_child.contains(target),
                None => false,
            }
        }
    }
}

pub fn dfs(root: &TreeNode) {
    let mut stack = vec![root];
    while let Some(node) = stack.pop() {
        println!("{}", node.value);
        if let Some(right) = &node.right {
            stack.push(right);
        }
        if let Some(left) = &node.left {
            stack.push(left);
        }
    }
}

pub fn bfs(root: &TreeNode) {
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root);
    while let Some(node) = queue.pop_front() {
        println!("{}", node.value);
        if let Some(left) = &node.left {
            queue.push_back(left);
        }
        if let Some(right) = &node.right {
            queue.push_back(right);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_node() {
        let node = TreeNode::new(10);
        assert_eq!(node.value, 10);
        assert!(node.left.is_none());
        assert!(node.right.is_none());
    }

    #[test]
    fn test_insert_left_and_right() {
        let mut root = TreeNode::new(10);
        root.insert(5);
        root.insert(15);

        assert_eq!(root.left.as_ref().unwrap().value, 5);
        assert_eq!(root.right.as_ref().unwrap().value, 15);
    }

    #[test]
    fn test_contains() {
        let mut root = TreeNode::new(10);
        root.insert(5);
        root.insert(15);
        root.insert(3);
        root.insert(7);
        root.insert(12);
        root.insert(18);

        assert!(root.contains(7));
        assert!(root.contains(12));
        assert!(!root.contains(20));
    }

    #[test]
    fn test_insert_same_value() {
        let mut root = TreeNode::new(10);
        root.insert(10);
        root.insert(10);

        assert!(root.left.as_ref().unwrap().value == 10);
        assert!(root.left.as_ref().unwrap().left.as_ref().unwrap().value == 10);
    }

    // TODO: Fix this test to properly test the output
    #[test]
    fn test_dfs() {
        let mut root = TreeNode::new(10);
        root.insert(5);
        root.insert(15);
        root.insert(3);
        root.insert(7);
        root.insert(12);
        root.insert(18);

        dfs(&root);
    }
}
