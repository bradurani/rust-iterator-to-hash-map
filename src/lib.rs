use std::collections::HashMap;
use std::hash::Hash;

pub trait ToHashMap<T,K,V, FK, FV>
    where K:Hash,
          K:Eq,
          FK:Fn(&T)->K,
          FV:Fn(&T)->V {

    fn to_hash_map(self, key_func: FK, value_func: FV) -> HashMap<K, V>;
}

impl<T, K, V, FK, FV, I> ToHashMap<T, K, V, FK, FV> for I
    where K: Hash,
          K: Eq,
          FK: Fn(&T) -> K,
          FV: Fn(&T) -> V,
          I: IntoIterator<Item = T>
{
    fn to_hash_map(self, key_func: FK, value_func: FV) -> HashMap<K, V> {
        let mut hm: HashMap<K, V> = HashMap::new();
        for x in self {
            hm.insert(key_func(&x), value_func(&x));
        }
        hm
    }
}
