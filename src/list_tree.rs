use std::ptr;
use std::rc::Rc;

// Complete these:
pub fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
    let mut new_vec: Vec<i32> = Vec::new();
    for item in v { new_vec.push(item + n); }
    new_vec
}

pub fn add_n_in_place(v: &mut Vec<i32>, n: i32) {
    v.iter_mut().map(|item| *item += n);
}

pub fn reverse<T>(v: &mut Vec<T>) {
    for i in 0..(v.len() as f32 / 2f32).floor() as u32 {
        let x = v.len() - i as usize;
        unsafe { ptr::swap(v.get_mut(i as usize).unwrap(), v.get_mut(x - 1).unwrap()); }
    }
}


// Tree

#[test]
fn tree_test() {
    let t1 = wrap(Tree::Node(
        wrap(Tree::Leaf),
        100,
        wrap(Tree::Leaf)));
    let t2 = Tree::Node(wrap(Tree::Leaf), 30, t1.clone());
    let t3 = Tree::Node(wrap(t2), 40, t1.clone());

    // Should produce the tree:
    //    40
    //   /  \
    //  30   \
    //   \   /
    //    100
}


pub enum Tree<T> {
    Node(Rc<Tree<T>>, T, Rc<Tree<T>>),
    Leaf
}


fn wrap<T>(t: Tree<T>) -> Rc<Tree<T>> {
    Rc::new(t)
}

