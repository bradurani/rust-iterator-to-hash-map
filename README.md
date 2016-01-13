# Iterator to HashMap

A Rust crate that adds a method to any `Iterator` or `IntoIterator` (such as `Vec`) that converts it to a `HashMap` using the trait `ToHashMap`.

```rust
extern crate iterator_to_hash_map;

use std::collections::HashMap;
use iterator_to_hash_map::ToHashMap;

struct Person {
    id: i32,
    first_name: &'static str,
    last_name: &'static str,
}

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

let map = a.to_hash_map(key_func, value_func);
# { 1: "Brad Urani", 2: "Barb Hanover" }
```

## Contributing
Open a pull request!
