/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Trivial I/O routine definitions, intentionally meant to be included
 * multiple times. Ugly I/O routine concatenation helpers taken from
 * alpha. Must be included _before_ io.h to avoid preprocessor-induced
 * routine mismatch.
 *
 * The C header requires __IO_PREFIX to be defined by each includer. Rust
 * cannot concatenate an identifier with a macro argument on stable syntax,
 * so the equivalent declarations are emitted in a module named by the
 * supplied prefix.
 */

#[macro_export]
macro_rules! declare_io_generic {
    ($prefix:ident) => {
        pub mod $prefix {
            unsafe extern "C" {
                pub fn ioport_map(addr: ::core::ffi::c_ulong, size: ::core::ffi::c_uint)
                    -> *mut ::core::ffi::c_void;
                pub fn ioport_unmap(addr: *mut ::core::ffi::c_void);
                pub fn mem_init();
            }
        }
    };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
