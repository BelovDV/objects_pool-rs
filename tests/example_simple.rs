use objects_pool::PoolSimple;

#[test]
fn simple_str() {
    let mut pool = PoolSimple::default();
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
    let mut pool = PoolSimple::default();
    let abc = pool.insert("abc".to_string());
    let bcd = pool.insert("bcd".to_string());

    let abc2 = abc;
    assert!(abc == abc2);
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
    struct T;

    let mut pool = PoolSimple::default();
    let k1 = pool.insert(T);
    let k2 = pool.insert(T);

    assert!(k1 != k2);
}
