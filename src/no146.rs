#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Node {
    key: i32,
    value: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}


impl Node {
    fn new(key: i32, value: i32) -> Self {
        Self {
            key,
            value,
            prev: None,
            next: None,
        }
    }
}

struct LRUCache {
    capacity: usize,
    dummy: Rc<RefCell<Node>>,
    cache: HashMap<i32, Rc<RefCell<Node>>>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let dummy = Rc::new(RefCell::new(Node::new(-1, -1)));
        dummy.borrow_mut().prev = Some(dummy.clone());
        dummy.borrow_mut().next = Some(dummy.clone());

        Self {
            capacity: capacity as usize,
            dummy,
            cache: HashMap::new(),
        }
    }

    fn get(&self, key: i32) -> i32 {
        if let Some(node) = self.cache.get(&key) {
            self.remove(node.clone());
            self.push_front(node.clone());
            node.borrow().value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.cache.get(&key) {
            node.borrow_mut().value = value;
            self.remove(node.clone());
            self.push_front(node.clone());
        } else {
            let node = Rc::new(RefCell::new(Node::new(key, value)));
            self.cache.insert(key, node.clone());
            self.push_front(node);
            if self.cache.len() > self.capacity {
                let back_node = self.dummy.borrow().prev.clone().unwrap();
                self.cache.remove(&back_node.borrow().key);
                self.remove(back_node);
            }
        }
    }

    fn push_front(&self, node: Rc<RefCell<Node>>) {
        let cur = self.dummy.borrow().next.clone();

        node.borrow_mut().prev = Some(self.dummy.clone());
        node.borrow_mut().next = cur.clone();

        self.dummy.borrow_mut().next = Some(node.clone());
        cur.unwrap().borrow_mut().prev = Some(node);
    }

    fn remove(&self, node: Rc<RefCell<Node>>) {
        let prev = node.borrow().prev.clone().unwrap();
        let next = node.borrow().next.clone().unwrap();

        prev.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(prev);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 1);
        lru_cache.put(2, 2);
        assert_eq!(lru_cache.get(1), 1);
        lru_cache.put(3, 3);
        assert_eq!(lru_cache.get(2), -1);
        lru_cache.put(4, 4);
        assert_eq!(lru_cache.get(1), -1);
        assert_eq!(lru_cache.get(3), 3);
        assert_eq!(lru_cache.get(4), 4);
    }
}
