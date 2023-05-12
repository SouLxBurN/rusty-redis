#![allow(dead_code)]
use std::cmp;
use std::fmt::Display;

#[derive(Debug)]
struct AVLTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T> AVLTree<T> where T: Ord + Display {
    fn new() -> Self {
        AVLTree { root: None }
    }

    fn insert(&mut self, data: T) {
        println!("Insert {data}");
        if let Some(mut root_node) = self.root.take() {
            root_node.insert(data);
            let balance = root_node.balance_factor();
            self.root = match balance {
                x if x > 1 => Some(root_node.right_rotation(0)),
                x if x < -1 => Some(root_node.left_rotation(0)),
                _ => Some(root_node)
            };
        } else {
            self.root = Some(Box::new(Node {
                data,
                left: None,
                right: None,
            }));
        }
    }

}

impl<T> AVLTree<T> where T: Display + Ord {
    fn print_data(&self) {
        if let Some(n) = &self.root {
            n.print_tree()
        }
    }
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> where T: Ord + Display {
    fn new(data: T) -> Self {
        Node {
            data,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, data: T) {
        if data < self.data {
            self.left = if let Some(mut node) = self.left.take() {
                node.insert(data);
                let balance = node.balance_factor();
                match balance {
                    x if x > 1 => Some(node.right_rotation(0)),
                    x if x < -1 => Some(node.left_rotation(0)),
                    _ => Some(node)
                }
            } else {
                Some(Box::new(Node::new(data)))
            }
        } else {
            self.right = if let Some(mut node) = self.right.take() {
                node.insert(data);
                let balance = node.balance_factor();
                match balance {
                    x if x > 1 => Some(node.right_rotation(0)),
                    x if x < -1 => Some(node.left_rotation(0)),
                    _ => Some(node)
                }
            } else {
                Some(Box::new(Node::new(data)))
            }
        }
    }

    fn right_rotation(mut self, depth: u8) -> Box<Node<T>> {
        let mut l = self.left.take().unwrap();
        if l.balance_factor() < 0 && depth < 2 {
                l = l.left_rotation(depth+1);
        }
        if l.right.is_some() {
            let _ = self.left.insert(l.right.take().unwrap());
        }
        let _ = l.right.insert(Box::new(self));
        l
    }

    fn left_rotation(mut self, depth: u8) -> Box<Node<T>> {
        let mut r = self.right.take().unwrap();
        if r.balance_factor() > 0 && depth < 2 {
            r = r.right_rotation(depth+1);
        }
        if r.left.is_some() {
            let _ = self.right.insert(r.left.take().unwrap());
        }
        let _ = r.left.insert(Box::new(self));
        r
    }

    fn balance_factor(&self) -> i32 {
        self.left_height() - self.right_height()
    }

    fn height(&self) -> i32 {
        cmp::max(self.left_height(), self.right_height()) + 1
    }

    fn left_height(&self) -> i32  {
        match &self.left {
            Some(n) => n.height(),
            None => -1,
        }
    }

    fn right_height(&self) -> i32  {
        match &self.right {
            Some(n) => n.height(),
            None => -1,
        }
    }
}

impl<T> Node<T> where T: Display + Ord {
    fn print_tree(&self) {
        if let Some(l) = &self.left {
            l.print_tree();
        }
        let l = if let Some(t) = self.left.as_deref() {
            t.data.to_string()
        } else {
            String::from("None")
        };

        let r = if let Some(t) = self.right.as_deref() {
            t.data.to_string()
        } else {
            String::from("None")
        };

        println!("{}, {}, {}, l:{}, r:{}",
            self.data,
            self.height(),
            self.balance_factor(),
            l,
            r
        );

        if let Some(r) = &self.right {
            r.print_tree();
        }
    }
}

#[cfg(test)]
mod test {
    use super::AVLTree;

    #[test]
    fn test_insert() {
        let mut tree = AVLTree::new();
        tree.insert(5);
        tree.insert(2);
        tree.insert(9);
        tree.insert(6);
        tree.insert(1);
        tree.insert(8);
        tree.insert(4);
        tree.insert(3);
        tree.insert(7);
        tree.print_data();
    }

    #[test]
    fn test_left_rotation() {
        let mut tree = AVLTree::new();
        tree.insert(1);
        tree.insert(2);
        tree.insert(3);
        tree.insert(5);
        tree.insert(4);
        tree.print_data();
    }

    #[test]
    fn test_single_node_height() {
        let mut tree = AVLTree::new();
        tree.insert(5);
        tree.insert(7);
        tree.print_data();
    }

    #[test]
    fn test_with_strings() {
        let mut tree = AVLTree::new();
        tree.insert("Hello");
        tree.insert("Stream");
        tree.print_data();
    }
}
