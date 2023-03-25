use objects_pool::{id_cast, variadic, Id, Pool as _, Simple, Unique, Variadic};

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

    let id_abc_gen: Id<U> = id_cast(id_abc);
    let id_123_gen: Id<U> = id_cast(id_123);
    assert!(id_123_gen != id_abc_gen);

    assert!(*pool.get_s(id_abc) == "abc");
    assert!(*pool.get_s(id_234) == 234);

    let id_c_cs = pool.insert(U::StaticStr("C"));
    let id_c_ci = pool.insert(U::i32(12));

    assert!(id_c_cs != id_c_ci);
    assert!(matches!(pool.get(id_c_cs), U::StaticStr(_)));
    assert!(matches!(pool.get(id_c_ci), U::i32(12)));
}

#[cfg(feature = "fn_overload")]
mod test_nightly {
    use objects_pool::{id_cast, variadic, Id, Pool as _, Unique, Variadic};

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

        let id_a = pool.insert("a".to_string());
        let id_a_2 = pool.insert("a".to_string());
        let id_1 = pool.insert(1);
        assert_eq!(id_a, id_a_2);
        // assert_ne!(id_a, id_1); // Different types.
        let id_a_gen: Id<Object> = id_cast(id_a);
        let id_1_gen: Id<Object> = id_cast(id_1);
        assert_ne!(id_a_gen, id_1_gen);

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

        assert!(matches!(pool.get(v[4]), Object::i32(2)));
        assert!(matches!(pool.get(v[3]), Object::String(_)));
    }
}
