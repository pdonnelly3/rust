use std::boxed::Box;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::vec::Vec;

struct BST {
    root: Link,
    _marker: PhantomData<Box<Node>>,
}

type Link = Option<NonNull<Node>>;

#[derive(Debug)]
struct Node {
    data: i32,
    left: Link,
    right: Link,
}

impl Node {
    fn new(data: i32) -> Self {
        Node {
            data,
            left: None,
            right: None,
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new(-1)
    }
}

impl Default for BST {
    fn default() -> Self {
        Self::new()
    }
}

impl BST {
    pub fn new() -> Self {
        BST {
            root: None,
            _marker: PhantomData,
        }
    }

    /*
    fn insert_rec(&self, root: &mut Option<NonNull<Node>>, data: i32) {
        match root {
            None => {
                *root = NonNull::<Node>::new(&mut Node::new(data));
                println!("{:?}", root);
            },
            Some(r) => unsafe {
                let ptr = &mut r.as_ref();
                if data < ptr.data {
                    self.insert_rec(&mut ptr.left, data);
                }
                self.insert_rec(&mut ptr.right, data);
            },
        }
    } */

    pub fn insert(&mut self, data: i32) {
        let mut root: &mut Link = &mut self.root;

        while let Some(mut r) = root {
            unsafe {
                let ptr = r.as_mut();
                if data < ptr.data {
                    root = &mut ptr.left;
                } else {
                    root = &mut ptr.right;
                }
            }
        }
        let mut node = Box::new(Node::new(data));
        *root = Some(NonNull::from(Box::leak(node)));
    }

    fn print_rec(&self, root: &Link) {
        match root {
            None => return,
            Some(r) => unsafe {
                let ptr = r.as_ref();
                self.print_rec(&ptr.left);
                println!("{:?}", ptr);
                self.print_rec(&ptr.right);
            },
        }
    }

    pub fn print(&self) {
        self.print_rec(&self.root);
    }

    pub fn print_branch(&self, root: &Link) {
        match root {
            None => return,
            Some(r) => unsafe {
                let ptr = r.as_ref();

                self.print_rec(&ptr.left);
                self.print_rec(&ptr.right);
            },
        }
    }

    pub fn print_tree(&self) {
        let mut q: VecDeque<Vec<&Node>> = VecDeque::<Vec<&Node>>::new();
        let mut v = Vec::new();
        let mut row_count = 1;

        if let Some(node) = self.root {
            unsafe {
                v.push(node.as_ref());
                q.push_front(v);
            }
        }

        while !q.is_empty() {
            let row: Vec<&Node> = q.pop_front().unwrap();

            let mut next_row: Vec<&Node> = Vec::new();

            for item in row.iter() {
                if let Some(node) = item.left {
                    unsafe {
                        let ptr = node.as_ref();
                        next_row.push(node.as_ref());
                        println!("{:>>width$}", ptr.data, width=row_count*3);
                    }
                }

                if let Some(node) = item.right {
                    unsafe {
                        let ptr = node.as_ref();
                        next_row.push(node.as_ref());
                        println!("{:>>width$}", ptr.data, width=row_count*3);
                    }
                }
            }

            if !next_row.is_empty() {
                q.push_front(next_row);
                row_count += 1;
            }
        }
    }
}

fn main() {
    let mut list: BST = BST::new();

    list.insert(10);
    list.insert(0);
    list.insert(40);
    list.insert(100);
    list.insert(-1);
    list.print_tree();
}
