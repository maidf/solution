use std::{cell::RefCell, collections::VecDeque, i32, ops::Not, rc::Rc};

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

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut stack = VecDeque::new();
    let mut inorder = i64::MIN;
    let mut current = root.clone();

    while stack.is_empty().not() || current != None {
        while current != None {
            stack.push_back(current.clone());
            current = current.clone().unwrap().borrow().left.clone();
        }

        current = stack.pop_back().unwrap();

        if current.clone().unwrap().borrow().val as i64 <= inorder {
            return false;
        }

        inorder = current.clone().unwrap().borrow().val as i64;
        current = current.unwrap().borrow().right.clone();
    }

    true
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use crate::solu::s98_is_valid_bst::is_valid_bst;

    use super::TreeNode;

    #[test]
    fn test() {
        let mut root = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        root.as_mut().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.as_mut().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let mut right = root.clone().unwrap().borrow_mut().right.clone();

        right.as_mut().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        right.as_mut().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(6))));

        let res = is_valid_bst(root.clone());
        assert_eq!(false, res);

        print_inorder(&root);
    }

    fn print_inorder(node: &Option<Rc<RefCell<TreeNode>>>) {
        if let Some(n) = node {
            println!("hello: {} ", n.borrow().val);
            print_inorder(&n.borrow().left);
            print_inorder(&n.borrow().right);
        }
    }
}
