use std::collections::HashMap;

fn main() {
    print!("Hi");
    let l = LRU::with_capacity(2);
    l.put("1", "1");
    l.put("2", "2");
    l.get("1");
    l.put("3", "3");
    l.put("4", "4");
}
struct LRU {
    capacity: u16,
    size: u16,
    cache: HashMap<&'static str, &'static str>,
}
impl LRU {
    fn with_capacity(cap: u16) -> Self {
        return LRU {
            capacity: cap,
            size: 0,
            cache: HashMap::new(),
        };
    }
    fn get(&self, key: &str) -> &'static str {
        "1"
    }
    fn put(mut self, key: &'static str, value: &'static str) {
        if self.size >= self.capacity {
            //evict the lru element
        }
        self.cache.insert(key, value);
    }
}
