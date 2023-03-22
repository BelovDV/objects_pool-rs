use objects_pool::PoolUnique;

#[test]
fn simple_str() {
    let mut pool = PoolUnique::default();
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
fn simple_string() {
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
        "bcd" => {}
        "abc" => panic!(),
        _ => panic!(),
    }
}

#[test]
fn simple_t() {
    #[derive(Debug, Hash, PartialEq, Eq)]
    struct T(i32);

    let mut pool = PoolUnique::default();
    let k1 = pool.insert(T(1));
    let k2 = pool.insert(T(2));
    let k1_2 = pool.insert(T(1));

    assert!(k1 != k2);
    assert!(k1 == k1_2);
}
