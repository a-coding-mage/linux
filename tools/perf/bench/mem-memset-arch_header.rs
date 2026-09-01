/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C source condition:
 *   #ifdef HAVE_ARCH_X86_64_SUPPORT
 *
 * The original header defines MEMSET_FN for use by
 * "mem-memset-x86-64-asm-def.h", which expands architecture-specific memset
 * declarations, then undefines MEMSET_FN.
 */

#[cfg(HAVE_ARCH_X86_64_SUPPORT)]
macro_rules! MEMSET_FN {
    ($fn:ident, $init:expr, $fini:expr, $name:expr, $desc:expr) => {
        unsafe extern "C" {
            pub fn $fn(arg1: *mut ::core::ffi::c_void, arg2: ::core::ffi::c_int, arg3: usize)
                -> *mut ::core::ffi::c_void;
        }
    };
}

#[cfg(HAVE_ARCH_X86_64_SUPPORT)]
/* include dependency in C source: "mem-memset-x86-64-asm-def.h" */
const _: () = ();


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
