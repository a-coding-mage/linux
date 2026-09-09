/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from asm-generic/error-injection.h. */

#[cfg(all(feature = "kernel", not(feature = "assembly")))]
#[repr(C)]
pub enum ErrorInjectionType {
    /// Return NULL if failure.
    EI_ETYPE_NULL,
    /// Return -ERRNO if failure.
    EI_ETYPE_ERRNO,
    /// Return -ERRNO or NULL if failure.
    EI_ETYPE_ERRNO_NULL,
    /// Return true if failure.
    EI_ETYPE_TRUE,
}

#[cfg(all(feature = "kernel", not(feature = "assembly")))]
#[repr(C)]
pub struct error_injection_entry {
    pub addr: libc::c_ulong,
    pub etype: libc::c_int,
}

#[cfg(all(feature = "kernel", not(feature = "assembly")))]
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[cfg(all(feature = "kernel", not(feature = "assembly"), feature = "function-error-injection"))]
/*
 * Whitelist generating macro. Specify functions which can be error-injectable
 * using this macro. If you unsure what is required for the error-injectable
 * functions, please read Documentation/fault-injection/fault-injection.rst
 * 'Error Injectable Functions' section.
 *
 * Rust cannot concatenate identifiers in macro_rules!; the generated entry is
 * therefore named by the caller while preserving the C entry's fields and
 * section placement intent.
 */
#[macro_export]
macro_rules! ALLOW_ERROR_INJECTION {
    ($entry:ident, $fname:path, $etype:ident) => {
        #[used]
        #[link_section = "_error_injection_whitelist"]
        static $entry: $crate::error_injection_entry = $crate::error_injection_entry {
            addr: $fname as usize as libc::c_ulong,
            etype: $crate::ErrorInjectionType::$etype as libc::c_int,
        };
    };
}

#[cfg(all(feature = "kernel", not(feature = "assembly"), feature = "function-error-injection"))]
unsafe extern "C" {
    pub fn override_function_with_return(regs: *mut pt_regs);
}

#[cfg(all(feature = "kernel", not(feature = "assembly"), not(feature = "function-error-injection")))]
#[inline]
pub unsafe fn override_function_with_return(_regs: *mut pt_regs) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
