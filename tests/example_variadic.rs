use objects_pool::{variadic, Pool as _, Simple, Unique, Variadic};

variadic!(C: String, i32);

#[test]
fn variadic_simple() {
    let mut pool: Variadic<C, Simple<C>> = Default::default();

    let id_abc = pool.insert_s("abc".to_string());
    let id_abc_2 = pool.insert_s("abc".to_string());
    let id_123 = pool.insert_s(123);
    let id_234 = pool.insert_s(234);
    let id_123_2 = pool.insert_s(123);
    let id_123_c = id_123;

    assert!(id_abc != id_abc_2); // !
    assert!(id_123 == id_123_c);
    assert!(id_123 != id_123_2); // !

    // assert!(id_abc != id_123);

    assert!(pool.get_s(id_abc) == "abc");
    assert!(*pool.get_s(id_234) == 234);

    let id_c_cs = pool.insert(C::String("C".to_string()));
    let id_c_ci = pool.insert(C::i32(12));

    assert!(id_c_cs != id_c_ci);
    assert!(matches!(pool.get(id_c_cs), C::String(_)));
    assert!(matches!(pool.get(id_c_ci), C::i32(12)));
}

type StaticStr = &'static str;
variadic!(U: StaticStr, i32; derive(Hash, PartialEq, Eq));

#[test]
fn variadic_unique() {
    let mut pool: Variadic<U, Unique<U>> = Default::default();

    let id_abc = pool.insert_s("abc");
    let id_abc_2 = pool.insert_s("abc");
    let id_123 = pool.insert_s(123);
    let id_234 = pool.insert_s(234);
    let id_123_2 = pool.insert_s(123);
    let id_123_c = id_123;

    assert!(id_abc == id_abc_2); // !
    assert!(id_123 == id_123_c);
    assert!(id_123 == id_123_2); // !
                                 // assert!(id_abc != id_123);

    assert!(*pool.get_s(id_abc) == "abc");
    assert!(*pool.get_s(id_234) == 234);

    let id_c_cs = pool.insert(U::StaticStr("C"));
    let id_c_ci = pool.insert(U::i32(12));

    assert!(id_c_cs != id_c_ci);
    assert!(matches!(pool.get(id_c_cs), U::StaticStr(_)));
    assert!(matches!(pool.get(id_c_ci), U::i32(12)));
}
