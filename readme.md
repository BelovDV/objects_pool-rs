# Objects pool

Store objects in pool and manipulate their ids.

Is there common implementation of this (on [crates.io](crates.io))?

How should it be named?

## Purpose

It was written for two use cases:

- switch from `String` identficators to `Id` - just integer,
- simple work with DAGs.

## Example

Implement dictionary. Use `Copy`able identifiers to work with instead of strings.

```rust
use objects_pool::{Pool as _, Unique};

#[test]
fn example_dict() {
    let mut pool = Unique::default();

    let words: Vec<_> = ["a", "b", "c", "a", "b", "d"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let ids: Vec<_> = words.iter().map(|w| pool.insert(w)).collect();

    for i in 0..ids.len() {
        for j in i + 1..ids.len() {
            match (i, j) {
                // "a" == "a" | "b" == "b".
                (0, 3) | (1, 4) => assert_eq!(words[i], words[j]),
                _ => assert_ne!(words[i], words[j]),
            }
        }
    }
}
```

Implement storage for `Variant` types. `Id` may be both general and specific.

This can be done without *unstable*, but with a bit less convenient syntax.

```rust
use objects_pool::{id_cast, variadic, Id, Pool as _, Unique, Variadic};

// Object is `enum` of listed types. Traits to be `derive`d can be passed.
variadic!(Object: String, i32; derive(Hash, PartialEq, Eq));

type Pool = Variadic<Object, Unique<Object>>;

fn insert_s_gen(pool: &mut Pool, value: &str) -> Id<Object> {
    id_cast(pool.insert(value.to_string()))
}
fn insert_i_gen(pool: &mut Pool, value: i32) -> Id<Object> {
    id_cast(pool.insert(value))
}

#[test]
fn variadic_example_nightly() {
    let mut pool: Pool = Default::default();

    // Work with specific ids.
    let id_a = pool.insert("a".to_string());
    let id_a_2 = pool.insert("a".to_string());
    let id_1 = pool.insert(1);
    assert_eq!(id_a, id_a_2);
    // assert_ne!(id_a, id_1); // Different types.
    let id_a_gen: Id<Object> = id_cast(id_a);
    let id_1_gen: Id<Object> = id_cast(id_1);
    assert_ne!(id_a_gen, id_1_gen);

    // Work with general ids.
    let v = vec![
        insert_s_gen(&mut pool, "b"),
        insert_s_gen(&mut pool, "c"),
        insert_i_gen(&mut pool, 1),
        insert_s_gen(&mut pool, "a"),
        insert_i_gen(&mut pool, 2),
    ];
    assert!(v[0] != v[1]);
    assert!(v[2] == id_1_gen);
    assert!(v[3] == id_a_gen);

    // General objects can be processed with `match`.
    assert!(matches!(pool.get(v[4]), Object::i32(2)));
    assert!(matches!(pool.get(v[3]), Object::String(_)));
}
```
