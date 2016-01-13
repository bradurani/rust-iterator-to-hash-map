extern crate iteratortohashmap;

use std::collections::HashMap;
use std::hash::Hash;
use iteratortohashmap::ToHashMap;

struct Person {
    id: i32,
    first_name: &'static str,
    last_name: &'static str,
}

#[test]
fn it_works() {

    let brad = Person {
        id: 1,
        first_name: "Brad",
        last_name: "Urani",
    };
    let barb = Person {
        id: 2,
        first_name: "Barb",
        last_name: "Hanover",
    };

    let a = vec![brad, barb];
    let key_func = |i: &Person| -> i32 { i.id };
    let value_func = |i: &Person| -> String {
        i.first_name.to_string() + &" " + &i.last_name.to_string()
    };

    let map = a.into_iter().to_hash_map(key_func, value_func);

    let mut hm: HashMap<i32, String> = HashMap::new();
    hm.insert(1, "Brad Urani".to_string());
    hm.insert(2, "Barb Hanover".to_string());

    assert_eq!(map.get(&1).unwrap(), hm.get(&1).unwrap());
    assert_eq!(map.get(&2).unwrap(), hm.get(&2).unwrap());
}
