# Objects pool

Store objects in pool and manipulate their ids.

Is there common implementation of this (on [crates.io](crates.io))?

## Purpose

It was written for two use cases:

- switch from `String` identficators to `Id` - just integer,
- simple work with DAGs.

## Example

```rust
use objects_pool::PoolUnique;

fn test_with_string() {
    let mut pool = PoolUnique::default();
    let abc = pool.insert("abc".to_string());
    let bcd = pool.insert("bcd".to_string());
    let abc_2 = pool.insert("abc".to_string());

    let abc2 = abc;
    assert!(abc == abc2);
    assert!(abc == abc_2);
    assert!(abc != bcd);
    assert!(pool.get(abc) == pool.get(abc2));

    match pool.get(bcd).as_str() {
        "abc" => panic!(),
        "bcd" => {}
        _ => panic!(),
    }
}
```
