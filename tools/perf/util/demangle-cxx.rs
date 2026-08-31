// SPDX-License-Identifier: GPL-2.0
// Translated from demangle-cxx.cpp.
// Original dependencies: "demangle-cxx.h", <stdlib.h>, <string.h>,
// <linux/compiler.h>, optionally <bfd.h> and <cxxabi.h>.

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

#[cfg(any(HAVE_LIBBFD_SUPPORT, HAVE_CPLUS_DEMANGLE_SUPPORT))]
const DMGL_PARAMS: c_int = 1 << 0; /* Include function args */
#[cfg(any(HAVE_LIBBFD_SUPPORT, HAVE_CPLUS_DEMANGLE_SUPPORT))]
const DMGL_ANSI: c_int = 1 << 1; /* Include const, volatile, etc */

#[cfg(HAVE_LIBBFD_SUPPORT)]
const PACKAGE: &str = "perf";

unsafe extern "C" {
    #[cfg(HAVE_LIBBFD_SUPPORT)]
    fn bfd_demangle(abfd: *mut c_void, name: *const c_char, options: c_int) -> *mut c_char;

    #[cfg(all(not(HAVE_LIBBFD_SUPPORT), HAVE_CPLUS_DEMANGLE_SUPPORT))]
    fn cplus_demangle(mangled: *const c_char, options: c_int) -> *mut c_char;

    #[cfg(all(
        not(HAVE_LIBBFD_SUPPORT),
        not(HAVE_CPLUS_DEMANGLE_SUPPORT),
        HAVE_CXA_DEMANGLE_SUPPORT
    ))]
    #[link_name = "__cxa_demangle"]
    fn abi___cxa_demangle(
        mangled_name: *const c_char,
        output_buffer: *mut c_char,
        length: *mut usize,
        status: *mut c_int,
    ) -> *mut c_char;
}

/*
 * Demangle C++ function signature
 *
 * Note: caller is responsible for freeing demangled string
 */
#[no_mangle]
pub unsafe extern "C" fn cxx_demangle_sym(
    str_: *const c_char,
    params: bool,
    modifiers: bool,
) -> *mut c_char {
    #[cfg(HAVE_LIBBFD_SUPPORT)]
    {
        let flags: c_int =
            (if params { DMGL_PARAMS } else { 0 }) | (if modifiers { DMGL_ANSI } else { 0 });

        return unsafe { bfd_demangle(ptr::null_mut(), str_, flags) };
    }

    #[cfg(all(not(HAVE_LIBBFD_SUPPORT), HAVE_CPLUS_DEMANGLE_SUPPORT))]
    {
        let flags: c_int =
            (if params { DMGL_PARAMS } else { 0 }) | (if modifiers { DMGL_ANSI } else { 0 });

        return unsafe { cplus_demangle(str_, flags) };
    }

    #[cfg(all(
        not(HAVE_LIBBFD_SUPPORT),
        not(HAVE_CPLUS_DEMANGLE_SUPPORT),
        HAVE_CXA_DEMANGLE_SUPPORT
    ))]
    {
        let mut status: c_int = 0;

        let output = unsafe {
            abi___cxa_demangle(
                str_,
                ptr::null_mut(), /*output_buffer=*/
                ptr::null_mut(), /*length=*/
                &mut status,
            )
        };
        return output;
    }

    #[cfg(all(
        not(HAVE_LIBBFD_SUPPORT),
        not(HAVE_CPLUS_DEMANGLE_SUPPORT),
        not(HAVE_CXA_DEMANGLE_SUPPORT)
    ))]
    {
        let _ = (str_, params, modifiers);
        return ptr::null_mut();
    }
}
