use objects_pool::{Pool as _, Simple, Unique, Variadic, Variant};

#[allow(non_camel_case_types)]
enum C {
    String(String),
    i32(i32),
}

impl Variant<C> for String {
    fn pack(self) -> C {
        C::String(self)
    }

    fn unpack(from: &C) -> &Self {
        match from {
            C::String(s) => s,
            _ => unreachable!(),
        }
    }
}
impl Variant<C> for i32 {
    fn pack(self) -> C {
        C::i32(self)
    }

    fn unpack(from: &C) -> &Self {
        match from {
            C::i32(i) => i,
            _ => unreachable!(),
        }
    }
}

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

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Hash)]
enum U {
    i32(i32),
    str(&'static str),
}

impl Variant<U> for &'static str {
    fn pack(self) -> U {
        U::str(self)
    }

    fn unpack(from: &U) -> &Self {
        match from {
            U::str(s) => s,
            _ => unreachable!(),
        }
    }
}
impl Variant<U> for i32 {
    fn pack(self) -> U {
        U::i32(self)
    }

    fn unpack(from: &U) -> &Self {
        match from {
            U::i32(i) => i,
            _ => unreachable!(),
        }
    }
}

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

    let id_c_cs = pool.insert(U::str("C"));
    let id_c_ci = pool.insert(U::i32(12));

    assert!(id_c_cs != id_c_ci);
    assert!(matches!(pool.get(id_c_cs), U::str(_)));
    assert!(matches!(pool.get(id_c_ci), U::i32(12)));
}
