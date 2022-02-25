#!/usr/bin/env rust

// Return the name of the type as a String
// (`Any` provides similar functionality)
trait TypeName {
    fn type_name() -> String;
}

// Some implementations on types we want to be able to inspect.
// We could use macros to help generate these for many types.
impl TypeName for usize {
    fn type_name() -> String {
        "usize".to_owned()
    }
}
impl TypeName for String {
    fn type_name() -> String {
        "String".to_owned()
    }
}
impl <T: TypeName> TypeName for Vec<T> {
    fn type_name() -> String {
        format!("Vec<{}>", T::type_name())
    }
}
impl TypeName for () {
    fn type_name() -> String {
        "()".to_owned()
    }
}
impl <T1: TypeName, T2: TypeName> TypeName for (T1,T2) {
    fn type_name() -> String {
        format!("({},{})", T1::type_name(), T2::type_name())
    }
}

// Allow some basic introspection given the above:
fn inspect_function<F,T1,T2>(f: F)
where
  F: Fn(T1) -> T2,
  T1: TypeName,
  T2: TypeName
{
    println!("This function has the shape: {} -> {}",
        T1::type_name(),
        T2::type_name());
}
