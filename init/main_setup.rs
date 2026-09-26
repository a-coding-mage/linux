// SPDX-License-Identifier: GPL-2.0-only
//! Built-in boot-option records with the layout of the original init header.
//!
//! The caller supplies a nullable C callback and a unique identifier.
//! Records and their strings are reclaimed with init memory. The callback must
//! satisfy the boot-option parser's C interface and remain live during parsing.

/// Registers an original `__setup_param` record without a C metadata shim.
///
/// `early` is zero for a setup callback and one for an early parameter callback;
/// the two parsers interpret the callback's result differently. The unique
/// identifier allows several records to share a callback. Modules emit no
/// records, matching the original built-in-only registration mechanism.
/// `None` retains the original obsolete-option record without a callback.
macro_rules! setup_param {
    ($text:literal, $id:ident, $callback:expr, $early:expr) => {
        #[cfg(not(MODULE))]
        #[allow(dead_code, non_upper_case_globals)]
        const $id: () = {
            #[used]
            #[link_section = ".init.rodata"]
            static TEXT: [u8; concat!($text, "\0").len()] = {
                let source = concat!($text, "\0").as_bytes();
                let mut bytes = [0; concat!($text, "\0").len()];
                let mut index = 0;
                while index < bytes.len() {
                    bytes[index] = source[index];
                    index += 1;
                }
                bytes
            };

            // The original struct is writable and naturally aligned. Using
            // its canonical generated type preserves linker-array stride;
            // no wrapper adds padding or requires a fabricated Sync contract.
            #[used]
            #[link_section = ".init.setup"]
            static mut RECORD: $crate::bindings::obs_kernel_param =
                $crate::bindings::obs_kernel_param {
                    str_: TEXT.as_ptr().cast(),
                    setup_func: $callback,
                    early: $early,
                };
        };
    };
}

pub(crate) use setup_param;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
