// SPDX-License-Identifier: GPL-2.0-only
/*
 * lib/parser.c - simple parser for mount, etc. options.
 */
//! Mount-option parser with the original C capture and conversion semantics.

#[path = "../include/linux/parser_header.rs"]
pub mod declarations;
use core::ptr::addr_of_mut;
use declarations::{match_token as token_entry, substring_t, MAX_OPT_ARGS};
use kernel::bindings::{
    kmemdup_nul, kstrtouint, kstrtoull, memcpy, simple_strtol, simple_strtoul, strchr, strcmp,
    strlen, strncmp, GFP_KERNEL,
};
use kernel::ffi::{c_char, c_int, c_uint};

const NUMBER_BUF_LEN: usize = 24;
const EINVAL: c_int = 22;
const ERANGE: c_int = 34;

unsafe fn match_one(mut s: *mut c_char, mut p: *const c_char, args: *mut substring_t) -> bool {
    // SAFETY: Caller supplies valid storage for accesses actually reached,
    // including partial writes by failed alternatives; a null pattern reads
    // neither s nor args. Capture fields need not be initialized before writes.
    // Keep raw pointers: captures may alias input/table. Failed alternatives
    // still publish from/to fields in precisely the original C write order.
    unsafe {
        let mut argc = 0;
        if p.is_null() {
            return true;
        }
        loop {
            let mut len: c_int = -1;
            let meta = strchr(p, b'%' as c_int);
            if meta.is_null() {
                return strcmp(p, s) == 0;
            }
            let prefix = meta.offset_from(p);
            if strncmp(p, s, prefix as usize) != 0 {
                return false;
            }
            s = s.offset(prefix);
            p = meta.add(1);
            if (*p as u8).is_ascii_digit() {
                let mut end = core::ptr::null_mut();
                // C narrows unsigned long to int, including long field widths.
                len = simple_strtoul(p, &mut end, 10) as c_int;
                p = end;
            } else if *p as u8 == b'%' {
                if *s as u8 != b'%' {
                    return false;
                }
                s = s.add(1);
                p = p.add(1);
                continue;
            }
            if argc >= MAX_OPT_ARGS as usize {
                return false;
            }
            let arg = args.add(argc);
            (*arg).from = s;
            let kind = *p as u8;
            p = p.add(1);
            match kind {
                b's' => {
                    let str_len = strlen(s);
                    if str_len == 0 {
                        return false;
                    }
                    // Usual C conversions promote int to size_t here.
                    if len == -1 || len as usize > str_len {
                        len = str_len as c_int;
                    }
                    (*arg).to = s.offset(len as isize);
                }
                b'd' => {
                    simple_strtol(s, addr_of_mut!((*arg).to), 0);
                }
                b'u' => {
                    simple_strtoul(s, addr_of_mut!((*arg).to), 0);
                }
                b'o' => {
                    simple_strtoul(s, addr_of_mut!((*arg).to), 8);
                }
                b'x' => {
                    simple_strtoul(s, addr_of_mut!((*arg).to), 16);
                }
                _ => return false,
            }
            if kind != b's' && (*arg).to == (*arg).from {
                return false;
            }
            s = (*arg).to;
            argc += 1;
        }
    }
}

/// Finds the first matching entry, retaining partial captures on failure.
///
/// # Safety
/// Table storage must permit traversal through the first match or null-pattern
/// sentinel. Table fields and string bytes actually read must be initialized
/// and readable, with NUL termination where the C string operations require it.
/// Capture fields actually written must be writable, including partial writes
/// from failed alternatives; they need not be initialized before those writes.
/// A null pattern accesses neither s nor args; literal matches need no captures,
/// and a table reaching only one capture needs only one slot. Unaccessed pointers
/// may be null. All reached pointer arithmetic and accesses obey C API bounds.
#[no_mangle]
pub unsafe extern "C" fn match_token(
    s: *mut c_char,
    table: *const token_entry,
    args: *mut substring_t,
) -> c_int {
    // SAFETY: Caller guarantees the table and conditionally accessed storage.
    unsafe {
        let mut entry = table;
        while !match_one(s, (*entry).pattern, args) {
            entry = entry.add(1);
        }
        (*entry).token
    }
}

unsafe fn match_number(s: *mut substring_t, result: *mut c_int, base: c_uint) -> c_int {
    // SAFETY: Caller provides a readable substring and a result writable only
    // on success. Errors do not access result, which may then be null.
    unsafe {
        let mut buf = [0; NUMBER_BUF_LEN];
        if match_strlcpy(buf.as_mut_ptr(), s, NUMBER_BUF_LEN) >= NUMBER_BUF_LEN {
            return -ERANGE;
        }
        let mut end = core::ptr::null_mut();
        let value = simple_strtol(buf.as_ptr(), &mut end, base);
        if end == buf.as_mut_ptr() {
            -EINVAL
        } else if value < c_int::MIN as kernel::ffi::c_long
            || value > c_int::MAX as kernel::ffi::c_long
        {
            -ERANGE
        } else {
            *result = value as c_int;
            0
        }
    }
}

macro_rules! signed_conversion {
    ($name:ident, $base:expr) => {
        /// Parses with the original permissive signed conversion.
        ///
        /// # Safety
        /// s describes a readable substring; result is writable on success.
        #[no_mangle]
        pub unsafe extern "C" fn $name(s: *mut substring_t, result: *mut c_int) -> c_int {
            // SAFETY: Caller guarantees the substring and output storage.
            unsafe { match_number(s, result, $base) }
        }
    };
}
signed_conversion!(match_int, 0);
signed_conversion!(match_octal, 8);
signed_conversion!(match_hex, 16);

/// Parses an unsigned decimal substring with strict kernel conversion rules.
///
/// # Safety
/// s describes a readable substring; result is writable on success.
#[no_mangle]
pub unsafe extern "C" fn match_uint(s: *mut substring_t, result: *mut c_uint) -> c_int {
    // SAFETY: Copy before conversion preserves C behavior if result aliases s.
    unsafe {
        let mut buf = [0; NUMBER_BUF_LEN];
        if match_strlcpy(buf.as_mut_ptr(), s, NUMBER_BUF_LEN) >= NUMBER_BUF_LEN {
            return -ERANGE;
        }
        kstrtouint(buf.as_ptr(), 10, result)
    }
}

/// Parses a u64 with kernel radix detection and strict end checking.
///
/// # Safety
/// s describes a readable substring; result is writable on success.
#[no_mangle]
pub unsafe extern "C" fn match_u64(s: *mut substring_t, result: *mut u64) -> c_int {
    // SAFETY: Stack copy and temporary value preserve error output semantics.
    unsafe {
        let mut buf = [0; NUMBER_BUF_LEN];
        if match_strlcpy(buf.as_mut_ptr(), s, NUMBER_BUF_LEN) >= NUMBER_BUF_LEN {
            return -ERANGE;
        }
        let mut value = 0;
        let ret = kstrtoull(buf.as_ptr(), 0, &mut value);
        if ret == 0 {
            *result = value;
        }
        ret
    }
}

/// Matches the C parser's byte-oriented '*' and '?' wildcard language.
///
/// # Safety
/// Both pointers refer to readable NUL-terminated strings.
#[no_mangle]
pub unsafe extern "C" fn match_wildcard(pattern: *const c_char, string: *const c_char) -> bool {
    // SAFETY: Each advance follows a non-NUL byte in a caller-owned string.
    unsafe {
        let (mut s, mut p) = (string, pattern);
        let mut star = false;
        let (mut saved_s, mut saved_p) = (string, pattern);
        while *s != 0 {
            match *p as u8 {
                b'?' => {
                    s = s.add(1);
                    p = p.add(1);
                }
                b'*' => {
                    star = true;
                    saved_s = s;
                    p = p.add(1);
                    if *p == 0 {
                        return true;
                    }
                    saved_p = p;
                }
                _ => {
                    if *s == *p {
                        s = s.add(1);
                        p = p.add(1);
                    } else if !star {
                        return false;
                    } else {
                        saved_s = saved_s.add(1);
                        s = saved_s;
                        p = saved_p;
                    }
                }
            }
        }
        while *p as u8 == b'*' {
            p = p.add(1);
        }
        *p == 0
    }
}

/// Copies at most size-1 bytes and returns the full substring length.
///
/// # Safety
/// src bounds belong to one allocation. For nonzero size dest permits the
/// copied bytes and NUL write; copied ranges satisfy C memcpy's rules.
#[no_mangle]
pub unsafe extern "C" fn match_strlcpy(
    dest: *mut c_char,
    src: *const substring_t,
    size: usize,
) -> usize {
    // SAFETY: Raw field loads allow dest to overlap the descriptor, as in C.
    unsafe {
        let ret = (*src).to.offset_from((*src).from) as usize;
        if size != 0 {
            let len = ret.min(size - 1);
            memcpy(dest.cast(), (*src).from.cast(), len);
            *dest.add(len) = 0;
        }
        ret
    }
}

/// Allocates a NUL-terminated substring; caller frees it with kfree().
///
/// # Safety
/// s describes a readable substring in one allocation. Caller must allow
/// GFP_KERNEL allocation (which may sleep). Allocation failure returns null.
#[no_mangle]
pub unsafe extern "C" fn match_strdup(s: *const substring_t) -> *mut c_char {
    // SAFETY: Delegate allocation, overflow and failure behavior to the kernel.
    unsafe {
        kmemdup_nul(
            (*s).from,
            (*s).to.offset_from((*s).from) as usize,
            GFP_KERNEL,
        )
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
