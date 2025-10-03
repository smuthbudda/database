use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
pub struct Node {
    pub keys: Vec<u16>,
    pub children: Vec<Box<Node>>,
    pub n: usize,           // Current number of keys
    pub leaf: bool,
    pub t: usize,           // Minimum degree
}

impl Node {
    pub fn new(t: usize, leaf: bool) -> Self {
        Node {
            keys: vec![0; 2 * t - 1],
            children: Vec::with_capacity(2 * t),
            n: 0,
            leaf,
            t,
        }
    }

    // Insert a key in a non-full node
    pub fn insert_non_full(&mut self, key: u16) {
        let mut i = self.n as i32 - 1;

        if self.leaf {
            // If this is a leaf node, insert the key in sorted order
            while i >= 0 && self.keys[i as usize] > key {
                self.keys[(i + 1) as usize] = self.keys[i as usize];
                i -= 1;
            }
            self.keys[(i + 1) as usize] = key;
            self.n += 1;
        } else {
            // If this is not a leaf, find the child to insert into
            while i >= 0 && self.keys[i as usize] > key {
                i -= 1;
            }
            i += 1;

            // Check if the found child is full
            if self.children[i as usize].n == 2 * self.t - 1 {
                // If full, split it
                self.split_child(i as usize);

                // After split, decide which child to go to
                if self.keys[i as usize] < key {
                    i += 1;
                }
            }
            self.children[i as usize].insert_non_full(key);
        }
    }

    // Split the child at index i
    pub fn split_child(&mut self, i: usize) {
        let t = self.t;

        // Extract values we need before borrowing mutably
        let y_leaf = self.children[i].leaf;
        let middle_key = self.children[i].keys[t - 1];

        // Create a new node to store (t-1) keys of y
        let mut z = Box::new(Node::new(t, y_leaf));
        z.n = t - 1;

        // Copy the last (t-1) keys of y to z
        for j in 0..(t - 1) {
            z.keys[j] = self.children[i].keys[j + t];
        }

        // Copy the last t children of y to z (if not leaf)
        if !y_leaf {
            // Move the last t children from y to z
            let mut children_to_move = Vec::new();
            for _ in 0..t {
                if self.children[i].children.len() > t {
                    children_to_move.push(self.children[i].children.pop().unwrap());
                }
            }
            children_to_move.reverse();
            z.children = children_to_move;
        }

        // Reduce the number of keys in y
        self.children[i].n = t - 1;

        // Create space for new child
        self.children.insert(i + 1, z);

        // Move a key from y to this node
        self.keys.insert(i, middle_key);
        self.n += 1;
    }

    // Traverse all nodes
    pub fn traverse(&self) {
        let mut i = 0;
        while i < self.n {
            if !self.leaf {
                self.children[i].traverse();
            }
            print!("{} ", self.keys[i]);
            i += 1;
        }

        if !self.leaf {
            self.children[i].traverse();
        }
    }

    // Search for a key
    pub fn search(&self, key: u16) -> Option<&Node> {
        let mut i = 0;
        while i < self.n && key > self.keys[i] {
            i += 1;
        }

        if i < self.n && key == self.keys[i] {
            return Some(self);
        }

        if self.leaf {
            return None;
        }

        self.children[i].search(key)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
pub struct BTree {
    pub root: Option<Box<Node>>,
    pub t: usize, // Minimum degree
}

impl BTree {
    pub fn new(t: usize) -> Self {
        BTree { root: None, t }
    }

    pub fn traverse(&self) {
        if let Some(ref root) = self.root {
            root.traverse();
        }
    }

    pub fn search(&self, key: u16) -> bool {
        match &self.root {
            None => false,
            Some(root) => root.search(key).is_some(),
        }
    }

    // Main insert function
    pub fn insert(&mut self, key: u16) {
        if self.root.is_none() {
            // Tree is empty, create root
            let mut root = Box::new(Node::new(self.t, true));
            root.keys[0] = key;
            root.n = 1;
            self.root = Some(root);
        } else {
            // If root is full, tree grows in height
            if self.root.as_ref().unwrap().n == 2 * self.t - 1 {
                let mut s = Box::new(Node::new(self.t, false));

                // Make old root as child of new root
                let old_root = self.root.take().unwrap();
                s.children.push(old_root);

                // Split the old root and move 1 key to new root
                s.split_child(0);

                // New root has two children now
                let i = if s.keys[0] < key { 1 } else { 0 };
                s.children[i].insert_non_full(key);

                self.root = Some(s);
            } else {
                // If root is not full, call insert_non_full
                self.root.as_mut().unwrap().insert_non_full(key);
            }
        }
    }
}
