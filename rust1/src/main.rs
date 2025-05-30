use std::{cell::RefCell, rc::Rc};

use solu::s105_build_tree::{build_tree, TreeNode};

mod slib;
mod solu;

fn main() {
    // let nums1 = vec![20, 0, 18, 11, 0, 0, 0, 0, 0, 0, 17, 28, 0, 11, 10, 0, 0, 15, 29];
    // let nums2 = vec![16, 9, 25, 16, 1, 9, 20, 28, 8, 0, 1, 0, 1, 27];
    // let res = min_sum(nums1, nums2);
    // println!("res: {res}");

    // let nums1 = vec![3, 2, 0, 1, 0];
    // let nums2 = vec![6, 5, 0];
    // let res = min_sum(nums1, nums2);
    // println!("res: {res}");

    // let nums1 = vec![8, 13, 15, 18, 0, 18, 0, 0, 5, 20, 12, 27, 3, 14, 22, 0];
    // let nums2 = vec![29, 1, 6, 0, 10, 24, 27, 17, 14, 13, 2, 19, 2, 11];
    // let res = min_sum(nums1, nums2);
    // println!("res: {res}");

    // let mut nums1 = vec![2, 0, 3, 0, 0, 0];
    // let m = 1;
    // let mut nums2 = vec![1, 5, 6];
    // let n = 1;
    // merge(&mut nums1, m, &mut nums2, n);
    // println!("nums1: {:?}", nums1);

    let preorder = vec![3, 9, 20, 15, 7];
    let inorder = vec![9, 3, 15, 20, 7];
    let res = build_tree(preorder, inorder);

    print_inorder(&res);
}

fn print_inorder(node: &Option<Rc<RefCell<TreeNode>>>) {
    if let Some(n) = node {
        print_inorder(&n.borrow().left);
        println!("val: {} ", n.borrow().val);
        print_inorder(&n.borrow().right);
    }
}

pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
    let sum1 = nums1.iter().map(|&n| n as i64).sum::<i64>();
    let sum2 = nums2.iter().map(|&n| n as i64).sum::<i64>();

    let len1 = nums1.iter().filter(|&&n| n == 0).count() as i64;
    let len2 = nums2.iter().filter(|&&n| n == 0).count() as i64;

    let r1 = sum1 + len1;
    let r2 = sum2 + len2;

    if (len1 == 0 && sum1 < sum2) || (len2 == 0 && sum1 > sum2) {
        return -1;
    }
    std::cmp::max(r1, r2)
}

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    if n == 0 {
        return;
    }

    if m == 0 {
        for i in 0..n as usize {
            nums1[i] = nums2[i];
        }
        return;
    }

    let end = (m + n) as usize;
    let mut m = (m - 1) as usize;
    let mut n = (n - 1) as usize;
    for i in (0..end).rev() {
        println!("i: {i}");
        nums1[i] = if nums1[m] > nums2[n] {
            let r = nums1[m];
            if m == 0 {
                nums1[m] = i32::MIN
            }
            if m >= 1 {
                m -= 1
            }
            r
        } else {
            let r = nums2[n];
            if n == 0 {
                nums2[n] = i32::MAX
            }
            if n >= 1 {
                n -= 1
            }
            r
        }
    }
}