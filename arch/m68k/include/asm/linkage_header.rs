/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __ASM_LINKAGE_H

// C assembler alignment directives, retained as source-level constants.
pub const __ALIGN: &str = ".align 4";
pub const __ALIGN_STR: &str = ".align 4";

/*
 * Make sure the compiler doesn't do anything stupid with the
 * arguments on the stack - they are owned by the *caller*, not
 * the callee. This just fools gcc into not spilling into them,
 * and keeps it from doing tailcall recursion and/or using the
 * stack slots for temporaries, since they are live and "used"
 * all the way to the end of the function.
 *
 * The C implementation uses an empty volatile asm statement with
 * register and memory constraints. `black_box` expresses the same
 * compiler-observable use in Rust.
 */
#[macro_export]
macro_rules! __asmlinkage_protect_n {
    ($ret:expr $(, $arg:expr)*) => {{
        let _ = core::hint::black_box(&$ret);
        $(let _ = core::hint::black_box(&$arg);)*
    }};
}

#[macro_export]
macro_rules! __asmlinkage_protect0 {
    ($ret:expr) => { $crate::__asmlinkage_protect_n!($ret) };
}

#[macro_export]
macro_rules! __asmlinkage_protect1 {
    ($ret:expr, $arg1:expr) => { $crate::__asmlinkage_protect_n!($ret, $arg1) };
}

#[macro_export]
macro_rules! __asmlinkage_protect2 {
    ($ret:expr, $arg1:expr, $arg2:expr) => {
        $crate::__asmlinkage_protect_n!($ret, $arg1, $arg2)
    };
}

#[macro_export]
macro_rules! __asmlinkage_protect3 {
    ($ret:expr, $arg1:expr, $arg2:expr, $arg3:expr) => {
        $crate::__asmlinkage_protect_n!($ret, $arg1, $arg2, $arg3)
    };
}

#[macro_export]
macro_rules! __asmlinkage_protect4 {
    ($ret:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {
        $crate::__asmlinkage_protect_n!($ret, $arg1, $arg2, $arg3, $arg4)
    };
}

#[macro_export]
macro_rules! __asmlinkage_protect5 {
    ($ret:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {
        $crate::__asmlinkage_protect_n!($ret, $arg1, $arg2, $arg3, $arg4, $arg5)
    };
}

#[macro_export]
macro_rules! __asmlinkage_protect6 {
    ($ret:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr) => {
        $crate::__asmlinkage_protect_n!($ret, $arg1, $arg2, $arg3, $arg4, $arg5, $arg6)
    };
}

#[macro_export]
macro_rules! asmlinkage_protect {
    (0, $ret:expr) => { $crate::__asmlinkage_protect0!($ret) };
    (1, $ret:expr, $arg1:expr) => { $crate::__asmlinkage_protect1!($ret, $arg1) };
    (2, $ret:expr, $arg1:expr, $arg2:expr) => {
        $crate::__asmlinkage_protect2!($ret, $arg1, $arg2)
    };
    (3, $ret:expr, $arg1:expr, $arg2:expr, $arg3:expr) => {
        $crate::__asmlinkage_protect3!($ret, $arg1, $arg2, $arg3)
    };
    (4, $ret:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {
        $crate::__asmlinkage_protect4!($ret, $arg1, $arg2, $arg3, $arg4)
    };
    (5, $ret:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {
        $crate::__asmlinkage_protect5!($ret, $arg1, $arg2, $arg3, $arg4, $arg5)
    };
    (6, $ret:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr) => {
        $crate::__asmlinkage_protect6!($ret, $arg1, $arg2, $arg3, $arg4, $arg5, $arg6)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
