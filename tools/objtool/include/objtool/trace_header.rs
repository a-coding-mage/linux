/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2025, Oracle and/or its affiliates.
 */

/* Translated from objtool/include/objtool/trace.h. */
/* Depends on declarations supplied by objtool/check.h and objtool/disas.h. */

#[cfg(DISAS)]
extern "C" {
    pub static mut trace: bool;
    pub static mut trace_depth: ::std::os::raw::c_int;

    pub static mut stderr: *mut FILE;
    pub static mut objtool_disas_ctx: *mut ::std::ffi::c_void;

    pub fn fprintf(
        stream: *mut FILE,
        format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
    pub fn free(ptr: *mut ::std::ffi::c_void);

    pub fn disas_print_info(
        stream: *mut FILE,
        insn: *mut instruction,
        depth: ::std::os::raw::c_int,
        fmt: *const ::std::os::raw::c_char,
        ...
    );
    pub fn disas_print_insn(
        stream: *mut FILE,
        ctx: *mut ::std::ffi::c_void,
        insn: *mut instruction,
        depth: ::std::os::raw::c_int,
        fmt: *const ::std::os::raw::c_char,
        ...
    );
    pub fn disas_alt_type_name(insn: *mut instruction) -> *const ::std::os::raw::c_char;
    pub fn disas_alt_name(alt: *mut alternative) -> *mut ::std::os::raw::c_char;

    pub fn trace_insn_state(
        insn: *mut instruction,
        sprev: *mut insn_state,
        snext: *mut insn_state,
    );
    pub fn trace_alt_begin(
        orig_insn: *mut instruction,
        alt: *mut alternative,
        alt_name: *mut ::std::os::raw::c_char,
    );
    pub fn trace_alt_end(
        orig_insn: *mut instruction,
        alt: *mut alternative,
        alt_name: *mut ::std::os::raw::c_char,
    );
}

#[cfg(DISAS)]
#[repr(C)]
pub struct FILE {
    _unused: [u8; 0],
}

#[cfg(DISAS)]
macro_rules! TRACE {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {{
        unsafe {
            if trace {
                fprintf(stderr, $fmt $(, $arg)*);
            }
        }
    }};
}

/*
 * Print the instruction address and a message. The instruction
 * itself is not printed.
 */
#[cfg(DISAS)]
macro_rules! TRACE_ADDR {
    ($insn:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {{
        unsafe {
            if trace {
                disas_print_info(
                    stderr,
                    $insn,
                    trace_depth - 1,
                    concat!($fmt, "\n\0").as_ptr() as *const ::std::os::raw::c_char
                    $(, $arg)*,
                );
            }
        }
    }};
}

/*
 * Print the instruction address, the instruction and a message.
 */
#[cfg(DISAS)]
macro_rules! TRACE_INSN {
    ($insn:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {{
        unsafe {
            if trace {
                disas_print_insn(
                    stderr,
                    objtool_disas_ctx,
                    $insn,
                    trace_depth - 1,
                    $fmt
                    $(, $arg)*,
                );
                fprintf(stderr, b"\n\0".as_ptr() as *const ::std::os::raw::c_char);
                (*$insn).trace = 1;
            }
        }
    }};
}

#[cfg(DISAS)]
macro_rules! TRACE_INSN_STATE {
    ($insn:expr, $sprev:expr, $snext:expr $(,)?) => {{
        unsafe {
            if trace {
                trace_insn_state($insn, $sprev, $snext);
            }
        }
    }};
}

#[cfg(DISAS)]
macro_rules! TRACE_ALT_FMT {
    ($pfx:expr, $fmt:expr) => {
        concat!($pfx, "<%s.%lx> ", $fmt)
    };
}

#[cfg(DISAS)]
macro_rules! TRACE_ALT_ARG {
    ($insn:expr) => {
        disas_alt_type_name($insn), (*$insn).offset
    };
}

#[cfg(DISAS)]
macro_rules! TRACE_ALT {
    ($insn:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {
        TRACE_INSN!(
            $insn,
            TRACE_ALT_FMT!("", $fmt).as_ptr() as *const ::std::os::raw::c_char,
            TRACE_ALT_ARG!($insn)
            $(, $arg)*
        )
    };
}

#[cfg(DISAS)]
macro_rules! TRACE_ALT_INFO {
    ($insn:expr, $pfx:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {
        TRACE_ADDR!(
            $insn,
            TRACE_ALT_FMT!($pfx, $fmt),
            TRACE_ALT_ARG!($insn)
            $(, $arg)*
        )
    };
}

#[cfg(DISAS)]
macro_rules! TRACE_ALT_INFO_NOADDR {
    ($insn:expr, $pfx:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {
        TRACE_ADDR!(
            ::std::ptr::null_mut(),
            TRACE_ALT_FMT!($pfx, $fmt),
            TRACE_ALT_ARG!($insn)
            $(, $arg)*
        )
    };
}

#[cfg(DISAS)]
macro_rules! TRACE_ALT_BEGIN {
    ($insn:expr, $alt:expr, $alt_name:ident $(,)?) => {{
        unsafe {
            if trace {
                $alt_name = disas_alt_name($alt);
                trace_alt_begin($insn, $alt, $alt_name);
            }
        }
    }};
}

#[cfg(DISAS)]
macro_rules! TRACE_ALT_END {
    ($insn:expr, $alt:expr, $alt_name:expr $(,)?) => {{
        unsafe {
            if trace {
                trace_alt_end($insn, $alt, $alt_name);
                free($alt_name as *mut ::std::ffi::c_void);
            }
        }
    }};
}

#[cfg(DISAS)]
#[inline]
pub unsafe fn trace_enable() {
    trace = true;
    trace_depth = 0;
}

#[cfg(DISAS)]
#[inline]
pub unsafe fn trace_disable() {
    trace = false;
}

#[cfg(DISAS)]
#[inline]
pub unsafe fn trace_depth_inc() {
    if trace {
        trace_depth += 1;
    }
}

#[cfg(DISAS)]
#[inline]
pub unsafe fn trace_depth_dec() {
    if trace {
        trace_depth -= 1;
    }
}

#[cfg(not(DISAS))]
macro_rules! TRACE {
    ($($arg:tt)*) => {{}};
}

#[cfg(not(DISAS))]
macro_rules! TRACE_ADDR {
    ($($arg:tt)*) => {{}};
}

#[cfg(not(DISAS))]
macro_rules! TRACE_INSN {
    ($($arg:tt)*) => {{}};
}

#[cfg(not(DISAS))]
macro_rules! TRACE_INSN_STATE {
    ($($arg:tt)*) => {{}};
}

#[cfg(not(DISAS))]
macro_rules! TRACE_ALT {
    ($($arg:tt)*) => {{}};
}

#[cfg(not(DISAS))]
macro_rules! TRACE_ALT_INFO {
    ($($arg:tt)*) => {{}};
}

#[cfg(not(DISAS))]
macro_rules! TRACE_ALT_INFO_NOADDR {
    ($($arg:tt)*) => {{}};
}

#[cfg(not(DISAS))]
macro_rules! TRACE_ALT_BEGIN {
    ($($arg:tt)*) => {{}};
}

#[cfg(not(DISAS))]
macro_rules! TRACE_ALT_END {
    ($($arg:tt)*) => {{}};
}

#[cfg(not(DISAS))]
#[inline]
pub fn trace_enable() {}

#[cfg(not(DISAS))]
#[inline]
pub fn trace_disable() {}

#[cfg(not(DISAS))]
#[inline]
pub fn trace_depth_inc() {}

#[cfg(not(DISAS))]
#[inline]
pub fn trace_depth_dec() {}

#[cfg(not(DISAS))]
#[inline]
pub unsafe fn trace_alt_begin(
    _orig_insn: *mut instruction,
    _alt: *mut alternative,
    _alt_name: *mut ::std::os::raw::c_char,
) {
}

#[cfg(not(DISAS))]
#[inline]
pub unsafe fn trace_alt_end(
    _orig_insn: *mut instruction,
    _alt: *mut alternative,
    _alt_name: *mut ::std::os::raw::c_char,
) {
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
