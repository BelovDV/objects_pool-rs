# Objects pool

Store objects (several different types) in pool and manipulate their ids. Main feature - allow creating ids for one object, some of them know about actual type, some not.

Is there common implementation of this (on [crates.io](crates.io))? There are allocators (references - not id), there are arenas for single type or which id doesn't know actual type - this crate exists for another case.

How should it be named?

## State

This is in **early development state** - for now only interface matters. Performance is mostly (except asymptotics) ignored.

## Purpose

It was written for two use cases. One is simple (and there are better crates to do this) - switch from `String` identficators to `Id` - just integer.

Other - simple work with persistent 'DAG' which stores several complicated types. Features:
- don't bother about lifetimes,
- don't adjust types to work with DAG, just use `Id` field in them,
- one-line macro (list desired types to be in DAG) to use pool,
- don't clutter up code with checking `get` function - all `Id` are always valid,
- use generalized `Id` for object of any type from DAG (and `match` to work with actual value),
- use specific `Id` to enable compile-time type checking if reference to specific type required,
- don't clutter up code with unnecessary casting - if `Id` know object type then `get` will return reference with required type.

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
