use std::{
    cell::RefCell,
    collections::VecDeque,
    rc::Rc,
};

pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.is_empty() || inorder.is_empty() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(preorder[0])));
    let mut stack = VecDeque::new();
    stack.push_back(root.clone());

    let mut index = 0;
    for &val in preorder.iter().skip(1) {
        let current = Rc::new(RefCell::new(TreeNode::new(val)));
        let mut node = stack.back();

        if node.unwrap().borrow().val != inorder[index] {
            node.unwrap().borrow_mut().left = Some(current.clone());
        } else {
            while !stack.is_empty() && node.unwrap().borrow().val == inorder[index] {
                let poped = stack.pop_back();
                node = stack.back();
                index += 1;
                if stack.is_empty() || node.unwrap().borrow().val != inorder[index] {
                    poped.unwrap().borrow_mut().right = Some(current.clone());
                }
            }
        }

        stack.push_back(current);
    }

    Some(root)
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use crate::solu::s105_build_tree::TreeNode;

    use super::build_tree;

    #[test]
    fn test1() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let res = build_tree(preorder, inorder);

        print_inorder(&res);
    }

    fn print_inorder(node: &Option<Rc<RefCell<TreeNode>>>) {
        if let Some(n) = node {
            print_inorder(&n.borrow().left);
            print_inorder(&n.borrow().right);
            println!("hello: {} ", n.borrow().val);
        }
    }
}
