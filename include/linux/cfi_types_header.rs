/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Clang Control Flow Integrity (CFI) type definitions.
 *
 * This file preserves the header's assembly/preprocessor declarations as
 * Rust-facing conditional declarations and macro equivalents.
 */

/* The following declarations apply only when compiling assembly. */
#[cfg(all(target_arch = "asm", feature = "config_cfi"))]
macro_rules! __CFI_TYPE {
    ($name:ident) => {
        /* .4byte __kcfi_typeid_$name */
    };
}

#[cfg(all(target_arch = "asm", feature = "config_cfi"))]
macro_rules! SYM_TYPED_ENTRY {
    ($name:ident, $linkage:ident, $($align:tt)*) => {
        /* linkage($name), $($align)*, __CFI_TYPE($name), $name: */
    };
}

#[cfg(all(target_arch = "asm", feature = "config_cfi"))]
macro_rules! SYM_TYPED_START {
    ($name:ident, $linkage:ident, $($align:tt)*) => {
        SYM_TYPED_ENTRY!($name, $linkage, $($align)*);
    };
}

#[cfg(all(target_arch = "asm", not(feature = "config_cfi")))]
macro_rules! SYM_TYPED_START {
    ($name:ident, $linkage:ident, $($align:tt)*) => {
        /* SYM_START($name, $linkage, $($align)*) */
    };
}

#[cfg(target_arch = "asm")]
macro_rules! SYM_TYPED_FUNC_START {
    ($name:ident) => {
        SYM_TYPED_START!($name, SYM_L_GLOBAL, SYM_A_ALIGN);
    };
}

/*
 * In the C compilation branch, CONFIG_CFI's DEFINE_CFI_TYPE macro forces a
 * reference to a function and emits a read-only-after-init u32 containing its
 * __kcfi_typeid symbol. Rust has no direct file-local equivalent for the
 * compiler-specific inline assembly and __ADDRESSABLE mechanism; preserve the
 * externally visible declaration and emission intent here.
 */
#[cfg(feature = "config_cfi")]
macro_rules! DEFINE_CFI_TYPE {
    ($name:ident, $func:ident) => {
        extern "C" {
            static mut $name: u32;
        }
        /*
         * Intended layout: .data..ro_after_init, global object `$name`,
         * 4-byte alignment, and one .4byte __kcfi_typeid_$func.
         */
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
