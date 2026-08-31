/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Macros and attributes for compiler-based static context analysis.
 * No-op stubs for tools.
 */

#[macro_export]
macro_rules! __guarded_by {
    ($($arg:tt)*) => {};
}

#[macro_export]
macro_rules! __pt_guarded_by {
    ($($arg:tt)*) => {};
}

#[macro_export]
macro_rules! context_lock_struct {
    ($name:ident, $($body:tt)*) => {
        struct $name $($body)*
    };
}

#[macro_export]
macro_rules! __no_context_analysis {
    () => {};
}

#[macro_export]
macro_rules! __context_unsafe {
    ($comment:expr) => {};
    ($($comment:tt)*) => {};
}

#[macro_export]
macro_rules! context_unsafe {
    ($($body:tt)*) => {{
        $($body)*
    }};
}

#[macro_export]
macro_rules! context_unsafe_alias {
    ($p:expr) => {};
    ($($p:tt)*) => {};
}

#[macro_export]
macro_rules! disable_context_analysis {
    () => {};
}

#[macro_export]
macro_rules! enable_context_analysis {
    () => {};
}

#[macro_export]
macro_rules! __must_hold {
    ($($arg:tt)*) => {};
}

#[macro_export]
macro_rules! __must_not_hold {
    ($($arg:tt)*) => {};
}

#[macro_export]
macro_rules! __acquires {
    ($($arg:tt)*) => {};
}

#[macro_export]
macro_rules! __cond_acquires {
    ($ret:expr, $x:expr) => {};
    ($($arg:tt)*) => {};
}

#[macro_export]
macro_rules! __releases {
    ($($arg:tt)*) => {};
}

#[macro_export]
macro_rules! __acquire {
    ($x:expr) => {
        ()
    };
    ($($x:tt)*) => {
        ()
    };
}

#[macro_export]
macro_rules! __release {
    ($x:expr) => {
        ()
    };
    ($($x:tt)*) => {
        ()
    };
}

#[macro_export]
macro_rules! __must_hold_shared {
    ($($arg:tt)*) => {};
}

#[macro_export]
macro_rules! __acquires_shared {
    ($($arg:tt)*) => {};
}

#[macro_export]
macro_rules! __cond_acquires_shared {
    ($ret:expr, $x:expr) => {};
    ($($arg:tt)*) => {};
}

#[macro_export]
macro_rules! __releases_shared {
    ($($arg:tt)*) => {};
}

#[macro_export]
macro_rules! __acquire_shared {
    ($x:expr) => {
        ()
    };
    ($($x:tt)*) => {
        ()
    };
}

#[macro_export]
macro_rules! __release_shared {
    ($x:expr) => {
        ()
    };
    ($($x:tt)*) => {
        ()
    };
}

#[macro_export]
macro_rules! __acquire_ret {
    ($call:expr, $expr:expr) => {
        $call
    };
    ($call:expr, $($expr:tt)*) => {
        $call
    };
}

#[macro_export]
macro_rules! __acquire_shared_ret {
    ($call:expr, $expr:expr) => {
        $call
    };
    ($call:expr, $($expr:tt)*) => {
        $call
    };
}

#[macro_export]
macro_rules! __acquires_ret {
    () => {};
}

#[macro_export]
macro_rules! __acquires_shared_ret {
    () => {};
}
