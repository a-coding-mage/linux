/* SPDX-License-Identifier: GPL-2.0 */

// Original C condition: #ifdef HAVE_ARCH_X86_64_SUPPORT
// When that configuration symbol is enabled, the C header defines MEMCPY_FN
// as a declaration macro, includes "mem-memcpy-x86-64-asm-def.h", then undefines
// the macro. The included file is an external dependency and supplies the macro
// invocations that name the architecture-specific memcpy functions.

#[cfg(HAVE_ARCH_X86_64_SUPPORT)]
macro_rules! MEMCPY_FN {
    ($fn:ident, $init:expr, $fini:expr, $name:expr, $desc:expr) => {
        unsafe extern "C" {
            pub fn $fn(
                arg0: *mut core::ffi::c_void,
                arg1: *const core::ffi::c_void,
                arg2: usize,
            ) -> *mut core::ffi::c_void;
        }
    };
}

// Original C dependency under HAVE_ARCH_X86_64_SUPPORT:
// #include "mem-memcpy-x86-64-asm-def.h"
