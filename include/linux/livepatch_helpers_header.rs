/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Interfaces for use by livepatch patches
 *
 * C includes are intentionally omitted; the referenced kernel symbols are
 * supplied by the surrounding translation unit.
 */

/* MODULE selects __KBUILD_MODNAME in the C header; otherwise vmlinux. */
#[cfg(module)]
pub const KLP_OBJNAME: &str = "__KBUILD_MODNAME";
#[cfg(not(module))]
pub const KLP_OBJNAME: &str = "vmlinux";

/* Livepatch callback registration */

pub const KLP_CALLBACK_PTRS: &str = ".discard.klp_callback_ptrs";

/*
 * These macros describe linker-section callback registrations in C. Rust
 * declarations retain the callback and object-name arguments; section and
 * used attributes remain build-system responsibilities.
 */
#[macro_export]
macro_rules! KLP_PRE_PATCH_CALLBACK {
    ($func:expr) => { $func };
}

#[macro_export]
macro_rules! KLP_POST_PATCH_CALLBACK {
    ($func:expr) => { $func };
}

#[macro_export]
macro_rules! KLP_PRE_UNPATCH_CALLBACK {
    ($func:expr) => { $func };
}

#[macro_export]
macro_rules! KLP_POST_UNPATCH_CALLBACK {
    ($func:expr) => { $func };
}

/*
 * Replace static_call() usage with this macro when create-diff-object
 * recommends it due to the original static call key living in a module.
 *
 * This converts the static call to a regular indirect call.
 */
#[macro_export]
macro_rules! KLP_STATIC_CALL {
    ($name:ident) => {
        (STATIC_CALL_KEY!($name).func)
    };
}

/* Syscall patching */

#[macro_export]
macro_rules! KLP_SYSCALL_DEFINE1 { ($name:ident, $($args:tt)*) => { KLP_SYSCALL_DEFINEx!(1, $name, $($args)*); }; }
#[macro_export]
macro_rules! KLP_SYSCALL_DEFINE2 { ($name:ident, $($args:tt)*) => { KLP_SYSCALL_DEFINEx!(2, $name, $($args)*); }; }
#[macro_export]
macro_rules! KLP_SYSCALL_DEFINE3 { ($name:ident, $($args:tt)*) => { KLP_SYSCALL_DEFINEx!(3, $name, $($args)*); }; }
#[macro_export]
macro_rules! KLP_SYSCALL_DEFINE4 { ($name:ident, $($args:tt)*) => { KLP_SYSCALL_DEFINEx!(4, $name, $($args)*); }; }
#[macro_export]
macro_rules! KLP_SYSCALL_DEFINE5 { ($name:ident, $($args:tt)*) => { KLP_SYSCALL_DEFINEx!(5, $name, $($args)*); }; }
#[macro_export]
macro_rules! KLP_SYSCALL_DEFINE6 { ($name:ident, $($args:tt)*) => { KLP_SYSCALL_DEFINEx!(6, $name, $($args)*); }; }

#[macro_export]
macro_rules! KLP_SYSCALL_DEFINEx {
    ($x:expr, $sname:ident, $($args:tt)*) => {
        __KLP_SYSCALL_DEFINEx!($x, $sname, $($args)*);
    };
}

/* CONFIG_X86_64: the implementation expands architecture syscall stubs. */
#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! __KLP_SYSCALL_DEFINEx {
    ($x:expr, $name:ident, $($args:tt)*) => {
        /*
         * C equivalent:
         * static long __se_sys$name(...);
         * static inline long __klp_do_sys$name(...);
         * __X64_SYS_STUBx; __IA32_SYS_STUBx;
         * static long __se_sys$name(...) { let ret = __klp_do_sys$name(...);
         *   __MAP(...__SC_TEST...); __PROTECT(...); return ret; }
         * static inline long __klp_do_sys$name(...)
         *
         * The architecture-provided syscall wrapper machinery supplies these
         * declarations and expansions in the translated kernel environment.
         */
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
