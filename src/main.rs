use linked_tree::tree_node::{bfs, dfs, TreeNode};

fn main() {
    let mut root = TreeNode::new(10);
    root.insert(5);
    root.insert(15);
    root.insert(3);

    println!("Tree contains 5: {}", root.contains(5));
    println!("Tree contains 20: {}", root.contains(20));

    println!("DFS: ");

    dfs(&root);

    println!("BFS: ");

    bfs(&root);
}
