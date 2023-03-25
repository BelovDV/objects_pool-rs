use objects_pool::{Pool as _, Unique};

#[test]
fn unique_str() {
    let mut pool = Unique::default();
    let abc = pool.insert("abc");
    let bcd = pool.insert("bcd");

    let abc2 = abc;
    assert!(abc == abc2);
    assert!(abc != bcd);
    assert!(pool.get(abc) == pool.get(abc2));

    match *pool.get(bcd) {
        "bcd" => {}
        "abc" => panic!(),
        _ => panic!(),
    }
}

#[test]
fn unique_string() {
    let mut pool = Unique::default();
    let abc = pool.insert("abc".to_string());
    let bcd = pool.insert("bcd".to_string());
    let abc_2 = pool.insert("abc".to_string());

    let abc2 = abc;
    assert!(abc == abc2);
    assert!(abc == abc_2);
    assert!(abc != bcd);
    assert!(pool.get(abc) == pool.get(abc2));

    match pool.get(bcd).as_str() {
        "bcd" => {}
        "abc" => panic!(),
        _ => panic!(),
    }
}

#[test]
fn unique_t() {
    #[derive(Debug, Hash, PartialEq, Eq)]
    struct T(i32);

    let mut pool = Unique::default();
    let k1 = pool.insert(T(1));
    let k2 = pool.insert(T(2));
    let k1_2 = pool.insert(T(1));

    assert!(k1 != k2);
    assert!(k1 == k1_2);
}

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
                (0, 3) | (1, 4) => assert_eq!(words[i], words[j]),
                _ => assert_ne!(words[i], words[j]),
            }
        }
    }
}
