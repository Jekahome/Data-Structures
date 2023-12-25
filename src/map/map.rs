// https://github.com/PacktPublishing/Hands-On-Data-Structures-and-Algorithms-with-Rust/blob/master/Chapter06/src/map.rs

#[derive(Clone, Debug, PartialEq)]
pub struct LocationInformation {
    name: String,
    opened: String,
    address: String,
    security_group_name: String,
}
const MOD_ADLER: u32 = 65521;

pub fn adler32(bytes: &[u8]) -> u32 {
    let mut a = 1_u32;
    let mut b = 0_u32;

    for byte in bytes {
        a = (a + *byte as u32) % MOD_ADLER;
        b = (b + a) % MOD_ADLER;
    }

    (b << 16) | a
}

pub fn hashcode(bytes: &[u8]) -> u32 {
    let mut a = 0_u32;
    for (i, b) in bytes.iter().enumerate() {
        a ^= *b as u32;
        a <<= i % 4;
    }
    a
}

type Entry<K, V> = Vec<(K, V)>;

pub type LocationCache = HashMap<String, LocationInformation>;

pub struct HashMap<K, V>
where
    K: PartialEq + Clone,
    V: Clone,
{
    hash_fn: Box<dyn (Fn(&K) -> usize)>,
    store: Box<[Entry<K, V>]>,
    pub length: usize,
}

impl<K, V> HashMap<K, V>
where
    K: PartialEq + Clone,
    V: Clone,
{
    pub fn new(hash_fn: Box<dyn (Fn(&K) -> usize)>, length: usize) -> HashMap<K, V> {
        HashMap {
            hash_fn: hash_fn,
            length: 0,
            store: vec![vec![]; length].into_boxed_slice(),
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let h = (self.hash_fn)(key);
        let idx = h & (self.store.len() - 1);
        self.store[idx]
            .iter()
            .find(|e| e.0 == *key)
            .map(|e| e.1.clone())
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        let h = (self.hash_fn)(&key);
        let idx = h & (self.store.len() - 1);
        match self.store[idx].iter().position(|e| e.0 == key) {
            Some(pos) => {
                self.length -= 1;
                Some(self.store[idx].remove(pos).1)
            }
            _ => None,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let h = (self.hash_fn)(&key);
        let idx = h & (self.store.len() - 1);
        match self.store[idx].iter().position(|e| e.0 == key) {
            Some(pos) => self.store[idx][pos] = (key, value),
            None => {
                self.store[idx].push((key, value));
                self.length += 1
            }
        }
    }
}