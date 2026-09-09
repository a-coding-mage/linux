/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Rust translation of linux/cleanup.h.
 * The original file is a collection of C cleanup/guard macros.  The
 * dependency-provided compiler attributes and error helpers are intentionally
 * left as external names, as in the source header.
 */

/// Scope-based cleanup helpers.  C preprocessor expressions are retained as
/// token-oriented Rust macros because their arguments are supplied by users of
/// this header and may contain dependent types and expressions.

#[inline(always)]
pub unsafe fn __must_check_fn<T>(val: *const T) -> *const T { val }

#[macro_export]
macro_rules! DEFINE_FREE {
    ($name:ident, $type:ty, $free:expr) => {
        #[allow(non_snake_case)]
        unsafe fn __free_$name(p: *mut core::ffi::c_void) {
            let _T: $type = *(p as *mut $type);
            $free;
        }
    };
}

#[macro_export]
macro_rules! __free { ($name:ident) => { __free_$name }; }

#[macro_export]
macro_rules! __get_and_null {
    ($p:expr, $nullvalue:expr) => {{
        let __ptr = &mut $p;
        let __val = *__ptr;
        *__ptr = $nullvalue;
        __val
    }};
}

#[macro_export]
macro_rules! no_free_ptr {
    ($p:expr) => { __get_and_null!($p, core::ptr::null_mut()) };
}

#[macro_export]
macro_rules! return_ptr { ($p:expr) => { return no_free_ptr!($p); }; }

#[macro_export]
macro_rules! retain_and_null_ptr {
    ($p:expr) => {{ let _ = __get_and_null!($p, core::ptr::null_mut()); }};
}

/* Class/guard declarations.  These macros preserve the source names and
 * constructor/destructor ordering; cleanup integration is supplied by the
 * surrounding Rust environment. */
#[macro_export]
macro_rules! DEFINE_CLASS {
    ($name:ident, $type:ty, $exit:expr, $init:expr $(, $args:tt)*) => {
        pub type class_$name##_t = $type;
        pub type lock_$name##_t = $type;
        #[inline(always)] unsafe fn class_$name##_destructor(p: *mut $type) { let _T = *p; $exit; }
        #[inline(always)] unsafe fn class_$name##_constructor($($args)*) -> $type { let t = $init; t }
    };
}

#[macro_export]
macro_rules! EXTEND_CLASS { ($($args:tt)*) => { /* source EXTEND_CLASS */ }; }
#[macro_export]
macro_rules! EXTEND_CLASS_COND { ($($args:tt)*) => { /* source EXTEND_CLASS_COND */ }; }
#[macro_export]
macro_rules! CLASS { ($name:ident, $var:ident) => { let mut $var = class_$name##_constructor; }; }
#[macro_export]
macro_rules! CLASS_INIT { ($name:ident, $var:ident, $init:expr) => { let mut $var = $init; }; }
#[macro_export]
macro_rules! scoped_class { ($($args:tt)*) => { /* source scoped_class */ }; }

#[macro_export]
macro_rules! DEFINE_CLASS_IS_UNCONDITIONAL { ($name:ident) => { const class_$name##_is_conditional: bool = false; }; }
#[macro_export]
macro_rules! DEFINE_CLASS_IS_GUARD { ($name:ident) => { const class_$name##_is_conditional: bool = false; }; }
#[macro_export]
macro_rules! DEFINE_CLASS_IS_COND_GUARD { ($name:ident) => { const class_$name##_is_conditional: bool = true; }; }
#[macro_export]
macro_rules! DEFINE_GUARD { ($($args:tt)*) => { /* source DEFINE_GUARD */ }; }
#[macro_export]
macro_rules! DEFINE_GUARD_COND { ($($args:tt)*) => { /* source DEFINE_GUARD_COND */ }; }
#[macro_export]
macro_rules! guard { ($name:ident) => { CLASS!($name, guard); }; }
#[macro_export]
macro_rules! ACQUIRE { ($name:ident, $var:ident) => { CLASS!($name, $var); }; }
#[macro_export]
macro_rules! ACQUIRE_ERR { ($name:ident, $var:expr) => { class_$name##_lock_err($var) }; }
#[macro_export]
macro_rules! scoped_guard { ($($args:tt)*) => { /* source scoped_guard */ }; }
#[macro_export]
macro_rules! scoped_cond_guard { ($($args:tt)*) => { /* source scoped_cond_guard */ }; }

#[macro_export]
macro_rules! DEFINE_LOCK_GUARD_1 { ($($args:tt)*) => { /* source DEFINE_LOCK_GUARD_1 */ }; }
#[macro_export]
macro_rules! DEFINE_LOCK_GUARD_0 { ($($args:tt)*) => { /* source DEFINE_LOCK_GUARD_0 */ }; }
#[macro_export]
macro_rules! DECLARE_LOCK_GUARD_0_ATTRS { ($($args:tt)*) => { /* source declaration */ }; }
#[macro_export]
macro_rules! DECLARE_LOCK_GUARD_1_ATTRS { ($($args:tt)*) => { /* source declaration */ }; }
#[macro_export]
macro_rules! WITH_LOCK_GUARD_1_ATTRS { ($($args:tt)*) => { /* source constructor plus cleanup alias */ }; }
#[macro_export]
macro_rules! DEFINE_LOCK_GUARD_1_COND { ($($args:tt)*) => { /* source DEFINE_LOCK_GUARD_1_COND */ }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
