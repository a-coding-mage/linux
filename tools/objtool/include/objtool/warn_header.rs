/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2015 Josh Poimboeuf <jpoimboe@redhat.com>
 */

// C header dependencies: stdlib.h, string.h, sys/types.h, sys/stat.h,
// fcntl.h, errno.h, objtool/builtin.h, objtool/elf.h.

use core::ffi::{c_char, c_int, c_ulong, c_ulonglong, c_void};

extern "C" {
    pub static objname: *const c_char;

    pub static mut debug: bool;
    pub static mut debug_correlate: bool;
    pub static mut debug_clone: bool;
    pub static mut indent: c_int;

    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strlen(s: *const c_char) -> usize;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;

    pub static mut stderr: *mut c_void;
    pub static mut errno: c_int;

    fn elf_errmsg(error: c_int) -> *const c_char;
    fn find_func_containing(sec: *mut section, offset: c_ulong) -> *mut symbol;
    fn find_symbol_containing(sec: *mut section, offset: c_ulong) -> *mut symbol;
    fn insn_sym(insn: *mut instruction) -> *mut symbol;
    fn objtool_disas_insn(insn: *mut instruction) -> *const c_char;
    fn unlikely(cond: bool) -> bool;
}

extern "C" {
    pub static mut opts: objtool_opts;
}

pub const SHF_EXECINSTR: c_ulong = 0x4;

#[repr(C)]
pub struct objtool_opts {
    pub werror: bool,
    pub sec_address: bool,
    pub verbose: bool,
    pub backtrace: bool,
}

#[repr(C)]
pub struct elf_shdr {
    pub sh_flags: c_ulong,
}

#[repr(C)]
pub struct section {
    pub sh: elf_shdr,
    pub name: *const c_char,
}

#[repr(C)]
pub struct symbol {
    pub name: *const c_char,
    pub offset: c_ulong,
    pub warned: bool,
    pub debug_checksum: bool,
}

#[repr(C)]
pub struct instruction {
    pub sec: *mut section,
    pub offset: c_ulong,
    pub trace: c_int,
}

pub unsafe fn offstr(sec: *mut section, offset: c_ulong) -> *mut c_char {
    let is_text: bool = ((*sec).sh.sh_flags & SHF_EXECINSTR) != 0;
    let mut sym: *mut symbol = core::ptr::null_mut();
    let str_: *mut c_char;
    let len: c_int;

    if is_text {
        sym = find_func_containing(sec, offset);
    }
    if sym.is_null() {
        sym = find_symbol_containing(sec, offset);
    }

    if !sym.is_null() {
        str_ = malloc(strlen((*sym).name) + strlen((*sec).name) + 40) as *mut c_char;
        len = sprintf(
            str_,
            b"%s+0x%lx\0".as_ptr() as *const c_char,
            (*sym).name,
            offset.wrapping_sub((*sym).offset),
        );
        if opts.sec_address {
            sprintf(
                str_.offset(len as isize),
                b" (%s+0x%lx)\0".as_ptr() as *const c_char,
                (*sec).name,
                offset,
            );
        }
    } else {
        str_ = malloc(strlen((*sec).name) + 20) as *mut c_char;
        sprintf(
            str_,
            b"%s+0x%lx\0".as_ptr() as *const c_char,
            (*sec).name,
            offset,
        );
    }

    str_
}

#[macro_export]
macro_rules! ___WARN {
    ($severity:expr, $extra:expr, $format:expr $(, $arg:expr)* $(,)?) => {{
        unsafe {
            fprintf(
                stderr,
                concat!("%s%s%s: objtool", $extra, ": ", $format, "\n\0").as_ptr() as *const c_char,
                if objname.is_null() { b"\0".as_ptr() as *const c_char } else { objname },
                if objname.is_null() { b"\0".as_ptr() as *const c_char } else { b": \0".as_ptr() as *const c_char },
                $severity,
                $($arg,)*
            )
        }
    }};
}

#[macro_export]
macro_rules! __WARN {
    ($severity:expr, $format:expr $(, $arg:expr)* $(,)?) => {
        ___WARN!($severity, "", $format $(, $arg)*)
    };
}

#[macro_export]
macro_rules! __WARN_LINE {
    ($severity:expr, $format:expr $(, $arg:expr)* $(,)?) => {
        ___WARN!($severity, " [%s:%d]", $format, file!().as_ptr() as *const c_char, line!() as c_int $(, $arg)*)
    };
}

#[macro_export]
macro_rules! __WARN_ELF {
    ($severity:expr, $format:expr $(, $arg:expr)* $(,)?) => {
        __WARN_LINE!($severity, concat!("%s: ", $format, " failed: %s"), module_path!().as_ptr() as *const c_char $(, $arg)*, unsafe { elf_errmsg(-1) })
    };
}

#[macro_export]
macro_rules! __WARN_GLIBC {
    ($severity:expr, $format:expr $(, $arg:expr)* $(,)?) => {
        __WARN_LINE!($severity, concat!("%s: ", $format, " failed: %s"), module_path!().as_ptr() as *const c_char $(, $arg)*, unsafe { strerror(errno) })
    };
}

#[macro_export]
macro_rules! __WARN_FUNC {
    ($severity:expr, $sec:expr, $offset:expr, $format:expr $(, $arg:expr)* $(,)?) => {{
        unsafe {
            let _str = offstr($sec, $offset);
            __WARN!($severity, concat!("%s: ", $format), _str $(, $arg)*);
            free(_str as *mut c_void);
        }
    }};
}

#[macro_export]
macro_rules! WARN_STR {
    () => {
        unsafe {
            if opts.werror {
                b"error\0".as_ptr() as *const c_char
            } else {
                b"warning\0".as_ptr() as *const c_char
            }
        }
    };
}

#[macro_export]
macro_rules! WARN {
    ($format:expr $(, $arg:expr)* $(,)?) => {
        __WARN!(WARN_STR!(), $format $(, $arg)*)
    };
}

#[macro_export]
macro_rules! WARN_FUNC {
    ($sec:expr, $offset:expr, $format:expr $(, $arg:expr)* $(,)?) => {
        __WARN_FUNC!(WARN_STR!(), $sec, $offset, $format $(, $arg)*)
    };
}

#[macro_export]
macro_rules! WARN_INSN {
    ($insn:expr, $format:expr $(, $arg:expr)* $(,)?) => {{
        unsafe {
            let _insn: *mut instruction = $insn;
            if insn_sym(_insn).is_null() || !(*insn_sym(_insn)).warned {
                WARN_FUNC!((*_insn).sec, (*_insn).offset, $format $(, $arg)*);
                BT_INSN!(_insn, "");
            }
            if !insn_sym(_insn).is_null() {
                (*insn_sym(_insn)).warned = true;
            }
        }
    }};
}

#[macro_export]
macro_rules! BT_INSN {
    ($insn:expr, $format:expr $(, $arg:expr)* $(,)?) => {{
        unsafe {
            if opts.verbose || opts.backtrace {
                let __insn: *mut instruction = $insn;
                let _str = offstr((*__insn).sec, (*__insn).offset);
                let _istr = objtool_disas_insn(__insn);
                let mut _len: c_int;
                _len = snprintf(core::ptr::null_mut(), 0, concat!("  %s: ", $format, "\0").as_ptr() as *const c_char, _str $(, $arg)*);
                _len = if _len < 50 { 50 - _len } else { 0 };
                WARN!(concat!("  %s: ", $format, "  %*s%s"), _str $(, $arg)*, _len, b"\0".as_ptr() as *const c_char, _istr);
                free(_str as *mut c_void);
                (*__insn).trace = 1;
            }
        }
    }};
}

pub const ERROR_STR: *const c_char = b"error\0".as_ptr() as *const c_char;

#[macro_export]
macro_rules! ERROR {
    ($format:expr $(, $arg:expr)* $(,)?) => {
        __WARN!(ERROR_STR, $format $(, $arg)*)
    };
}

#[macro_export]
macro_rules! ERROR_ELF {
    ($format:expr $(, $arg:expr)* $(,)?) => {
        __WARN_ELF!(ERROR_STR, $format $(, $arg)*)
    };
}

#[macro_export]
macro_rules! ERROR_GLIBC {
    ($format:expr $(, $arg:expr)* $(,)?) => {
        __WARN_GLIBC!(ERROR_STR, $format $(, $arg)*)
    };
}

#[macro_export]
macro_rules! ERROR_FUNC {
    ($sec:expr, $offset:expr, $format:expr $(, $arg:expr)* $(,)?) => {
        __WARN_FUNC!(ERROR_STR, $sec, $offset, $format $(, $arg)*)
    };
}

#[macro_export]
macro_rules! ERROR_INSN {
    ($insn:expr, $format:expr $(, $arg:expr)* $(,)?) => {
        ERROR_FUNC!((*$insn).sec, (*$insn).offset, $format $(, $arg)*)
    };
}

pub unsafe fn unindent(_unused: *mut c_int) {
    indent -= 1;
}

// C __cleanup(func) maps to compiler cleanup attributes and has no direct
// item-level Rust equivalent; dbg_clone! below preserves its scope-exit intent.

#[macro_export]
macro_rules! __dbg {
    ($format:expr $(, $arg:expr)* $(,)?) => {{
        unsafe {
            fprintf(
                stderr,
                concat!("DEBUG: %s%s", $format, "\n\0").as_ptr() as *const c_char,
                if objname.is_null() { b"\0".as_ptr() as *const c_char } else { objname },
                if objname.is_null() { b"\0".as_ptr() as *const c_char } else { b": \0".as_ptr() as *const c_char },
                $($arg,)*
            )
        }
    }};
}

#[macro_export]
macro_rules! dbg_checksum_insn {
    ($func:expr, $insn:expr, $checksum:expr $(,)?) => {{
        unsafe {
            if unlikely((*$func).debug_checksum) {
                let insn_off = offstr((*$insn).sec, (*$insn).offset);
                __dbg!(
                    "checksum: %s(): %s %016llx",
                    (*$func).name,
                    insn_off,
                    $checksum as c_ulonglong
                );
                free(insn_off as *mut c_void);
            }
        }
    }};
}

#[macro_export]
macro_rules! dbg_checksum_object {
    ($sym:expr, $offset:expr, $what:expr, $checksum:expr $(,)?) => {{
        unsafe {
            if unlikely((*$sym).debug_checksum) {
                __dbg!(
                    "checksum: %s+0x%lx: %s %016llx",
                    (*$sym).name,
                    $offset,
                    $what,
                    $checksum as c_ulonglong
                );
            }
        }
    }};
}

#[macro_export]
macro_rules! dbg_correlate {
    ($($args:tt)*) => {{
        unsafe {
            if unlikely(debug_correlate) {
                __dbg!($($args)*);
            }
        }
    }};
}

#[macro_export]
macro_rules! __dbg_clone {
    ($format:expr $(, $arg:expr)* $(,)?) => {{
        unsafe {
            if unlikely(debug_clone) {
                __dbg!(concat!("%*s", $format), indent * 8, b"\0".as_ptr() as *const c_char $(, $arg)*);
            }
        }
    }};
}

pub struct DbgCloneIndentGuard;

impl Drop for DbgCloneIndentGuard {
    fn drop(&mut self) {
        unsafe {
            unindent(core::ptr::null_mut());
        }
    }
}

#[macro_export]
macro_rules! dbg_clone {
    ($($args:tt)*) => {
        let __dummy = DbgCloneIndentGuard;
        __dbg_clone!($($args)*);
        unsafe {
            indent += 1;
        }
    };
}
