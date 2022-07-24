// use std::hash::Hasher;
use std::{
    collections::{hash_map::DefaultHasher, LinkedList},
    fmt::Debug,
    hash::{Hash, Hasher},
};

#[derive(Eq, PartialEq, Hash, Debug)]
enum Node<K, V> {
    HasCollision(LinkedList<(K, V)>),
    Perfect(K, V),
    Empty,
}

impl<'a, K, V> Default for Node<K, V> {
    fn default() -> Self {
        Node::Empty
    }
}

impl<K: Clone, V: Clone> Clone for Node<K, V> {
    fn clone(&self) -> Self {
        match &self {
            &Node::Empty => Node::Empty,
            &Node::Perfect(k, v) => Node::Perfect(k.clone(), v.clone()),
            &Node::HasCollision(ll) => {
                let mut new_ll = LinkedList::new();
                new_ll.clone_from(ll);
                Node::HasCollision(new_ll)
            }
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct HashMap<K, V> {
    // every resize
    max_size: usize,
    keys: Vec<K>,

    /// In separate chaining, the process involves building a linked list with key–value pair for each search array index.
    /// This supposed to be a append-fast doubly-linked list with non-empty groups. But implement doubly-linked list is hard in rust.
    buckets: Vec<Node<K, V>>,
}

impl<K: Clone, V: Clone> Default for HashMap<K, V> {
    fn default() -> Self {
        let mut buckets = Vec::with_capacity(8);
        buckets.fill(Node::Empty);
        HashMap {
            max_size: 8,
            keys: vec![],
            buckets: buckets,
        }
    }
}

pub fn determine_size(s: usize) -> usize {
    if s < 8 {
        8
    } else {
        let m = (s as f32).log2().ceil();
        2_u32.pow(m as u32).try_into().unwrap()
    }
}

impl<K, V> HashMap<K, V>
where
    K: Copy + Clone + PartialEq + Hash + Debug,
    V: Copy + Clone + Debug,
{
    pub fn keys(&self) -> Vec<K> {
        self.keys.clone()
    }

    pub fn size_of(&self) -> usize {
        self.keys.len()
    }

    pub fn new() -> Self {
        HashMap::default()
    }

    pub fn from_vec(xs: Vec<(K, V)>) -> Self {
        let keys: Vec<K> = xs.iter().map(|x| x.0).collect();
        let bucket_size = determine_size(xs.len());

        println!("bucket size is {:?}", bucket_size);
        let mut bucket_group = vec![Node::Empty; bucket_size];
        // bucket_group.fill(Node::Empty);

        println!("bucket group is now {:?}", bucket_group);
        let mut s = DefaultHasher::new();
        for &(key, value) in xs.iter() {
            key.hash(&mut s);
            let x = s.finish();
            let i = (x as usize) % bucket_size;
            match &bucket_group[i] {
                Node::Empty => bucket_group[i] = Node::Perfect(key, value),
                Node::HasCollision(ll) => {
                    let mut a = ll.clone();
                    a.push_back((key, value));
                    bucket_group[i] = Node::HasCollision(ll.to_owned())
                }
                Node::Perfect(k, v) => {
                    let ll = LinkedList::from([(*k, *v), (key, value)]);
                    bucket_group[i] = Node::HasCollision(ll.to_owned())
                }
                _ => unreachable!(),
            }
        }
        HashMap {
            max_size: bucket_size,
            keys,
            buckets: bucket_group,
        }
    }

    pub fn contains_key(&self, k: &K) -> bool {
        self.keys.iter().find(|&x| x == k).is_some()
    }

    // follow what the https://doc.rust-lang.org/stable/src/std/collections/hash/map.rs.html#938-940
    fn insert(&mut self, key: &K, value: &V) -> Option<V> {
        self.keys.push(key.to_owned()); // update the keys

        // insert the bucket
        None
    }
}

fn main() {
    println!(
        "hashmap is {:?}",
        HashMap::from_vec(vec![
            ("Zheng Yan", 25),
            ("Chee Yung", 23),
            ("William", 21),
            ("XXX", 20),
            ("xxy", 32)
        ])
    )
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_determine_size() {
        assert_eq!(determine_size(8), 8);
        assert_eq!(determine_size(9), 16);
        assert_eq!(determine_size(15), 16);
        assert_eq!(determine_size(16), 16);
        assert_eq!(determine_size(17), 32);
        assert_eq!(determine_size(18), 32);
    }

    #[test]
    fn test_hashmap_from_vec() {
        println!("{:?}", HashMap::from_vec(vec![("Zheng Yan", 25)]))
    }
}
