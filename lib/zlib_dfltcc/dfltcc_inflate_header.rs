// SPDX-License-Identifier: Zlib

// Dependency supplied by dfltcc.h in the original C header.

/* External functions */
unsafe extern "C" {
    pub fn dfltcc_reset_inflate_state(strm: z_streamp);
    pub fn dfltcc_can_inflate(strm: z_streamp) -> ::core::ffi::c_int;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dfltcc_inflate_action {
    DFLTCC_INFLATE_CONTINUE,
    DFLTCC_INFLATE_BREAK,
    DFLTCC_INFLATE_SOFTWARE,
}

unsafe extern "C" {
    pub fn dfltcc_inflate(
        strm: z_streamp,
        flush: ::core::ffi::c_int,
        ret: *mut ::core::ffi::c_int,
    ) -> dfltcc_inflate_action;
}

// `z_streamp` is provided by the translated dfltcc/zlib declarations.

#[macro_export]
macro_rules! INFLATE_RESET_HOOK {
    ($strm:expr) => {{
        unsafe { $crate::dfltcc_reset_inflate_state($strm) }
    }};
}

#[macro_export]
macro_rules! INFLATE_TYPEDO_HOOK {
    ($strm:expr, $flush:expr) => {
        if unsafe { $crate::dfltcc_can_inflate($strm) } != 0 {
            let mut action;

            // RESTORE() and LOAD() are macros supplied by the including inflate
            // implementation and are intentionally resolved at the call site.
            RESTORE!();
            action = unsafe {
                $crate::dfltcc_inflate($strm, $flush, &mut ret)
            };
            LOAD!();
            if action == $crate::dfltcc_inflate_action::DFLTCC_INFLATE_CONTINUE {
                break;
            } else if action == $crate::dfltcc_inflate_action::DFLTCC_INFLATE_BREAK {
                // The C macro jumps to the including function's `inf_leave`
                // label; Rust has no direct equivalent for a foreign label.
                break;
            }
        }
    };
}

#[macro_export]
macro_rules! INFLATE_NEED_CHECKSUM {
    ($strm:expr) => {{
        unsafe { $crate::dfltcc_can_inflate($strm) } == 0
    }};
}

#[macro_export]
macro_rules! INFLATE_NEED_UPDATEWINDOW {
    ($strm:expr) => {{
        unsafe { $crate::dfltcc_can_inflate($strm) } == 0
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
