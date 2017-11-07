//! A macro for defining #[cfg] if-else statements.
//!
//! The macro provided by this crate, `cfg_if`, is similar to the `if/elif` C
//! preprocessor macro by allowing definition of a cascade of `#[cfg]` cases,
//! emitting the implementation which matches first.
//!
//! This allows you to conveniently provide a long list #[cfg]'d blocks of code
//! without having to rewrite each clause multiple times.
//!
//! # Example
//!
//! ```
//! #[macro_use]
//! extern crate cfg_if;
//!
//! cfg_if! {
//!     if #[cfg(unix)] {
//!         fn foo() { /* unix specific functionality */ }
//!     } else if #[cfg(target_pointer_width = "32")] {
//!         fn foo() { /* non-unix, 32-bit functionality */ }
//!     } else {
//!         fn foo() { /* fallback implementation */ }
//!     }
//!
//!     if #[cfg(windows)] {
//!
//!         /// windows specific module
//!         mod windows_only;
//!
//!         if #[test] {
//!             /// test for `windows_only`
//!             mod test;
//!         }
//!     }
//! }
//!
//! # fn main() {}
//! ```

#![no_std]

#![doc(html_root_url = "http://alexcrichton.com/cfg-if")]
#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]

#[macro_export]
macro_rules! cfg_if {
    (
        if #[$($cfg:tt)*] { $($inner:tt)* }
        $($rest:tt)*
    ) => {
        cfg_if! {
            @IF(if #[$($cfg)*] { cfg_if! { $($inner)* } })
            $($rest)*
        }
    };
    (
        @IF($($if:tt)*)
        else if #[$($e_cfg:tt)*] { $($e_inner:tt)* }
        $($rest:tt)*
    ) => {
        cfg_if! {
            @IF(
                $($if)* 
                else if #[$($e_cfg)*] { cfg_if! { $($e_inner)* } }
            )
            $($rest)*
        }
    };
    (
        @IF($($if:tt)*)
        else { $($e_inner:tt)* }
        $($rest:tt)*
    ) => {
        __flat_cfg_if! {
            $($if)* 
            else { cfg_if! { $($e_inner)* } }
        }
        cfg_if! {
            $($rest)*
        }
    };
    (
        @IF($($if:tt)*)
        $($rest:tt)*
    ) => {
        __flat_cfg_if! { $($if)* }
        cfg_if! { $($rest)* }
    };
    ($it:item $($rest:tt)*) => {
        $it
        cfg_if! { $($rest)* }
    };
    () => {}
}

#[macro_export]
#[doc(hidden)]
macro_rules! __flat_cfg_if {
    ($(
        if #[cfg($($meta:meta),*)] { $($it:item)* }
    ) else * else {
        $($it2:item)*
    }) => {
        __cfg_if_items! {
            () ;
            $( ( ($($meta),*) ($($it)*) ), )*
            ( () ($($it2)*) ),
        }
    };
    (
        if #[cfg($($i_met:meta),*)] { $($i_it:item)* }
        $(
            else if #[cfg($($e_met:meta),*)] { $($e_it:item)* }
        )*
    ) => {
        __cfg_if_items! {
            () ;
            ( ($($i_met),*) ($($i_it)*) ),
            $( ( ($($e_met),*) ($($e_it)*) ), )*
            ( () () ),
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __cfg_if_items {
    (($($not:meta,)*) ; ) => {};
    (($($not:meta,)*) ; ( ($($m:meta),*) ($($it:item)*) ), $($rest:tt)*) => {
        __cfg_if_apply! { cfg(all($($m,)* not(any($($not),*)))), $($it)* }
        __cfg_if_items! { ($($not,)* $($m,)*) ; $($rest)* }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! __cfg_if_apply {
    ($m:meta, $($it:item)*) => {
        $(#[$m] $it)*
    }
}

#[cfg(test)]
mod tests {
    cfg_if! {
        if #[cfg(test)] {
            use core::option::Option as Option2;
            fn works1() -> Option2<u32> { Some(1) }
        } else {
            fn works1() -> Option<u32> { None }
        }
    }

    cfg_if! {
        if #[cfg(foo)] {
            fn works2() -> bool { false }
        } else if #[cfg(test)] {
            fn works2() -> bool { true }
        } else {
            fn works2() -> bool { false }
        }
    }

    cfg_if! {
        if #[cfg(foo)] {
            fn works3() -> bool { false }
        } else {
            fn works3() -> bool { true }
        }
    }

    cfg_if! {
        if #[cfg(test)] {
            use core::option::Option as Option3;
            fn works4() -> Option3<u32> { Some(1) }
        }
    }

    cfg_if! {
        if #[cfg(foo)] {
            fn works5() -> bool { false }
        } else if #[cfg(test)] {
            fn works5() -> bool { true }
        }
    }

    cfg_if! {
        fn works8() -> bool { true }
        if #[cfg(test)] {
            fn works6() -> bool { true }

            if #[cfg(foo)] {
                fn works7() -> bool { false }
            } else {
                fn works7() -> bool { true }
            }
        }
        fn works9() -> bool { true }
    }

    cfg_if! {
        if #[cfg(not(test))] {
            fn works10() -> bool { false }
        } else if #[cfg(foo)] {
            fn works10() -> bool { false }
        } else {
            if #[cfg(test)] {
                fn works10() -> bool { true }
            }
        }
    }

    #[test]
    fn it_works() {
        assert!(works1().is_some());
        assert!(works2());
        assert!(works3());
        assert!(works4().is_some());
        assert!(works5());
        assert!(works6());
        assert!(works7());
        assert!(works8());
        assert!(works9());
        assert!(works10());
    }
}
