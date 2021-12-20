use std::fmt::{self, Display, Write};

enum BinaryTree<T> {
    Empty,
    Value(Box<Node<T>>),
}

impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match self {
            BinaryTree::Empty => {
                *self = BinaryTree::Value(Box::new(Node {
                    value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }))
            }
            BinaryTree::Value(node) => {
                if value < node.value {
                    node.left.add(value);
                } else if value > node.value {
                    node.right.add(value);
                }
            }
        }
    }
}

impl<T: Display> BinaryTree<T> {
    fn format(&self, out: &mut String, lvl: i32) -> fmt::Result {
        if let BinaryTree::Value(node) = self {
            node.left.format(out, lvl + 1)?;
            write!(out, "{} ", node.value)?;
            node.right.format(out, lvl + 1)
        } else {
            Ok(())
        }
    }
}

impl<T: Display> Display for BinaryTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        self.format(&mut out, 0)?;
        write!(f, "{}", out)
    }
}

struct Node<T> {
    value: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

fn main() {
    let mut tree = BinaryTree::Empty;
    tree.add(10);
    tree.add(20);
    tree.add(30);
    tree.add(5);

    println!("{}", tree);
}
