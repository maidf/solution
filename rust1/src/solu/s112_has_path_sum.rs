use std::{
    cell::RefCell,
    collections::VecDeque,
    rc::Rc,
};

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

pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    if root.is_none() {
        return false;
    }
    let mut queue = VecDeque::new();
    let mut sum = VecDeque::<i32>::new();
    queue.push_back(root.clone().unwrap());
    sum.push_back(root.clone().unwrap().borrow().val);
    while !queue.is_empty() {
        let now = queue.pop_front();
        let tmp = sum.pop_front().unwrap();

        if now.clone().unwrap().borrow().left.clone() == None
            && now.clone().unwrap().borrow().right.clone() == None
        {
            if tmp == target_sum {
                return true;
            }
            continue;
        }
        if now.clone().unwrap().borrow().left.clone() != None {
            queue.push_back(now.clone().unwrap().borrow().left.clone().unwrap());
            sum.push_back(tmp + now.clone().unwrap().borrow().left.clone().unwrap().borrow().val);
        }
        if now.clone().unwrap().borrow().right.clone() != None {
            queue.push_back(now.clone().unwrap().borrow().right.clone().unwrap());
            sum.push_back(tmp + now.clone().unwrap().borrow().right.clone().unwrap().borrow().val);
        }
    }

    false
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use super::{TreeNode, has_path_sum};

    #[test]
    fn test() {
        let mut root = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        root.as_mut().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        root.as_mut().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(8))));
        let mut left = root.clone().unwrap().borrow_mut().left.clone();
        let mut right = root.clone().unwrap().borrow_mut().right.clone();

        left.as_mut().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(11))));

        right.as_mut().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(13))));
        right.as_mut().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));

        let mut left = left.clone().unwrap().borrow_mut().left.clone();
        left.as_mut().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        left.as_mut().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));

        let mut right = right.clone().unwrap().borrow_mut().right.clone();
        right.as_mut().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(1))));

        let res = has_path_sum(root.clone(), 22);
        assert_eq!(true, res);

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
