use rand::Rng;
use std::collections::HashSet;

#[derive(Debug)]
struct RandomizedSet (HashSet<i32>);

impl RandomizedSet {

    fn new() -> Self {
        RandomizedSet(HashSet::new())
    }
    
    fn insert(&mut self, val: i32) -> bool {
        if self.0.contains(&val) {
            true
        } else {
            self.0.insert(val);

            false
        }
    }
    
    fn remove(&mut self, val: i32) -> bool {
        if self.0.contains(&val) {
            self.0.remove(&val);

            true
        } else {
            false
        }
        
    }
    
    fn get_random(&self) -> i32 {
        // self.0.get(rand::thread_rng().gen_range(0..self.0.len())).copied()
        let vec_collection = self.0.iter().collect::<Vec<&i32>>();
        *self.0.get(&vec_collection[rand::thread_rng().gen_range(0..vec_collection.len())]).unwrap()
        
    }
}

fn main() {
    let mut a = RandomizedSet::new();
    a.insert(1);
    a.insert(2);
    a.insert(3);
    a.insert(4);

    a.remove(2);
    dbg!(&a);

    dbg!(&a.get_random());
    dbg!(&a.get_random());
    dbg!(&a.get_random());
    dbg!(&a.get_random());
}
