// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

pub type u64 = u64;

const ENOSPC: c_int = 28;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

const TRACE_TYPE_REG: c_int = 1;
const TRACE_TYPE_COUNTER: c_int = 2;
const TRACE_TYPE_STRING: c_int = 3;
const TRACE_TYPE_INDENT: c_int = 4;
const TRACE_TYPE_OUTDENT: c_int = 5;

#[repr(C)]
pub struct trace_buffer {
    pub size: u64,
    pub tail: *mut c_void,
    pub overflow: bool,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct trace_entry {
    pub length: c_int,
    pub type_: c_int,
    pub data: [u8; 0],
}

unsafe extern "C" {
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;
    fn perror(s: *const c_char);
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn puts(s: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;

    static mut stderr: *mut c_void;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_buffer_allocate(size: u64) -> *mut trace_buffer {
    let tb: *mut trace_buffer;

    if size < size_of::<trace_buffer>() as u64 {
        unsafe {
            fprintf(
                stderr,
                c"Error: trace buffer too small\n".as_ptr(),
            );
        }
        return ptr::null_mut();
    }

    tb = unsafe {
        mmap(
            ptr::null_mut(),
            size as usize,
            PROT_READ | PROT_WRITE,
            MAP_ANONYMOUS | MAP_PRIVATE,
            -1,
            0,
        ) as *mut trace_buffer
    };
    if tb as *mut c_void == MAP_FAILED {
        unsafe {
            perror(c"mmap".as_ptr());
        }
        return ptr::null_mut();
    }

    unsafe {
        (*tb).size = size;
        (*tb).tail = (*tb).data.as_mut_ptr() as *mut c_void;
        (*tb).overflow = false;
    }

    tb
}

unsafe fn trace_check_bounds(tb: *mut trace_buffer, p: *mut c_void) -> bool {
    (p as usize) < unsafe { (tb as *mut u8).add((*tb).size as usize) as usize }
}

unsafe fn trace_check_alloc(tb: *mut trace_buffer, p: *mut c_void) -> bool {
    /*
     * If we ever overflowed don't allow any more input. This prevents us
     * from dropping a large item and then later logging a small one. The
     * buffer should just stop when overflow happened, not be patchy. If
     * you're overflowing, make your buffer bigger.
     */
    if unsafe { (*tb).overflow } {
        return false;
    }

    if !unsafe { trace_check_bounds(tb, p) } {
        unsafe {
            (*tb).overflow = true;
        }
        return false;
    }

    true
}

unsafe fn trace_alloc(tb: *mut trace_buffer, bytes: c_int) -> *mut c_void {
    let p: *mut c_void;
    let newtail: *mut c_void;

    unsafe {
        p = (*tb).tail;
        newtail = ((*tb).tail as *mut u8).add(bytes as usize) as *mut c_void;
    }
    if !unsafe { trace_check_alloc(tb, newtail) } {
        return ptr::null_mut();
    }

    unsafe {
        (*tb).tail = newtail;
    }

    p
}

unsafe fn trace_alloc_entry(tb: *mut trace_buffer, payload_size: c_int) -> *mut trace_entry {
    let e: *mut trace_entry;

    e = unsafe { trace_alloc(tb, (size_of::<trace_entry>() as c_int) + payload_size) as *mut trace_entry };
    if !e.is_null() {
        unsafe {
            (*e).length = payload_size;
        }
    }

    e
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_log_reg(tb: *mut trace_buffer, reg: u64, value: u64) -> c_int {
    let e: *mut trace_entry;
    let mut p: *mut u64;

    e = unsafe { trace_alloc_entry(tb, (size_of::<u64>() + size_of::<u64>()) as c_int) };
    if e.is_null() {
        return -ENOSPC;
    }

    unsafe {
        (*e).type_ = TRACE_TYPE_REG;
        p = (*e).data.as_mut_ptr() as *mut u64;
        *p = reg;
        p = p.add(1);
        *p = value;
        p = p.add(1);
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_log_counter(tb: *mut trace_buffer, value: u64) -> c_int {
    let e: *mut trace_entry;
    let mut p: *mut u64;

    e = unsafe { trace_alloc_entry(tb, size_of::<u64>() as c_int) };
    if e.is_null() {
        return -ENOSPC;
    }

    unsafe {
        (*e).type_ = TRACE_TYPE_COUNTER;
        p = (*e).data.as_mut_ptr() as *mut u64;
        *p = value;
        p = p.add(1);
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_log_string(tb: *mut trace_buffer, str_: *mut c_char) -> c_int {
    let e: *mut trace_entry;
    let mut p: *mut c_char;
    let len: c_int;

    len = unsafe { strlen(str_) as c_int };

    /* We NULL terminate to make printing easier */
    e = unsafe { trace_alloc_entry(tb, len + 1) };
    if e.is_null() {
        return -ENOSPC;
    }

    unsafe {
        (*e).type_ = TRACE_TYPE_STRING;
        p = (*e).data.as_mut_ptr() as *mut c_char;
        memcpy(p as *mut c_void, str_ as *const c_void, len as usize);
        p = p.add(len as usize);
        *p = 0;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_log_indent(tb: *mut trace_buffer) -> c_int {
    let e: *mut trace_entry;

    e = unsafe { trace_alloc_entry(tb, 0) };
    if e.is_null() {
        return -ENOSPC;
    }

    unsafe {
        (*e).type_ = TRACE_TYPE_INDENT;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_log_outdent(tb: *mut trace_buffer) -> c_int {
    let e: *mut trace_entry;

    e = unsafe { trace_alloc_entry(tb, 0) };
    if e.is_null() {
        return -ENOSPC;
    }

    unsafe {
        (*e).type_ = TRACE_TYPE_OUTDENT;
    }

    0
}

unsafe fn trace_print_header(seq: c_int, prefix: c_int) {
    unsafe {
        printf(c"%*s[%d]: ".as_ptr(), prefix, c"".as_ptr(), seq);
    }
}

unsafe fn trace_decode_reg(reg: c_int) -> *mut c_char {
    match reg {
        769 => return c"SPRN_MMCR2".as_ptr() as *mut c_char,
        770 => return c"SPRN_MMCRA".as_ptr() as *mut c_char,
        779 => return c"SPRN_MMCR0".as_ptr() as *mut c_char,
        804 => return c"SPRN_EBBHR".as_ptr() as *mut c_char,
        805 => return c"SPRN_EBBRR".as_ptr() as *mut c_char,
        806 => return c"SPRN_BESCR".as_ptr() as *mut c_char,
        800 => return c"SPRN_BESCRS".as_ptr() as *mut c_char,
        801 => return c"SPRN_BESCRSU".as_ptr() as *mut c_char,
        802 => return c"SPRN_BESCRR".as_ptr() as *mut c_char,
        803 => return c"SPRN_BESCRRU".as_ptr() as *mut c_char,
        771 => return c"SPRN_PMC1".as_ptr() as *mut c_char,
        772 => return c"SPRN_PMC2".as_ptr() as *mut c_char,
        773 => return c"SPRN_PMC3".as_ptr() as *mut c_char,
        774 => return c"SPRN_PMC4".as_ptr() as *mut c_char,
        775 => return c"SPRN_PMC5".as_ptr() as *mut c_char,
        776 => return c"SPRN_PMC6".as_ptr() as *mut c_char,
        780 => return c"SPRN_SIAR".as_ptr() as *mut c_char,
        781 => return c"SPRN_SDAR".as_ptr() as *mut c_char,
        768 => return c"SPRN_SIER".as_ptr() as *mut c_char,
        _ => {}
    }

    ptr::null_mut()
}

unsafe fn trace_print_reg(e: *mut trace_entry) {
    let mut p: *mut u64;
    let reg: *mut u64;
    let value: *mut u64;
    let name: *mut c_char;

    unsafe {
        p = (*e).data.as_mut_ptr() as *mut u64;
        reg = p;
        p = p.add(1);
        value = p;

        name = trace_decode_reg(*reg as c_int);
        if !name.is_null() {
            printf(c"register %-10s = 0x%016llx\n".as_ptr(), name, *value);
        } else {
            printf(c"register %lld = 0x%016llx\n".as_ptr(), *reg, *value);
        }
    }
}

unsafe fn trace_print_counter(e: *mut trace_entry) {
    let value: *mut u64;

    unsafe {
        value = (*e).data.as_mut_ptr() as *mut u64;
        printf(c"counter = %lld\n".as_ptr(), *value);
    }
}

unsafe fn trace_print_string(e: *mut trace_entry) {
    let str_: *mut c_char;

    unsafe {
        str_ = (*e).data.as_mut_ptr() as *mut c_char;
        puts(str_);
    }
}

const BASE_PREFIX: c_int = 2;
const PREFIX_DELTA: c_int = 8;

unsafe fn trace_print_entry(e: *mut trace_entry, seq: c_int, prefix: *mut c_int) {
    unsafe {
        match (*e).type_ {
            TRACE_TYPE_REG => {
                trace_print_header(seq, *prefix);
                trace_print_reg(e);
            }
            TRACE_TYPE_COUNTER => {
                trace_print_header(seq, *prefix);
                trace_print_counter(e);
            }
            TRACE_TYPE_STRING => {
                trace_print_header(seq, *prefix);
                trace_print_string(e);
            }
            TRACE_TYPE_INDENT => {
                trace_print_header(seq, *prefix);
                puts(c"{".as_ptr());
                *prefix += PREFIX_DELTA;
            }
            TRACE_TYPE_OUTDENT => {
                *prefix -= PREFIX_DELTA;
                if *prefix < BASE_PREFIX {
                    *prefix = BASE_PREFIX;
                }
                trace_print_header(seq, *prefix);
                puts(c"}".as_ptr());
            }
            _ => {
                trace_print_header(seq, *prefix);
                printf(c"entry @ %p type %d\n".as_ptr(), e, (*e).type_);
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_buffer_print(tb: *mut trace_buffer) {
    let mut e: *mut trace_entry;
    let mut i: c_int;
    let mut prefix: c_int;
    let mut p: *mut c_void;

    unsafe {
        printf(c"Trace buffer dump:\n".as_ptr());
        printf(c"  address  %p \n".as_ptr(), tb);
        printf(c"  tail     %p\n".as_ptr(), (*tb).tail);
        printf(c"  size     %llu\n".as_ptr(), (*tb).size);
        printf(
            c"  overflow %s\n".as_ptr(),
            if (*tb).overflow {
                c"TRUE".as_ptr()
            } else {
                c"false".as_ptr()
            },
        );
        printf(c"  Content:\n".as_ptr());

        p = (*tb).data.as_mut_ptr() as *mut c_void;

        i = 0;
        prefix = BASE_PREFIX;

        while trace_check_bounds(tb, p) && (p as usize) < ((*tb).tail as usize) {
            e = p as *mut trace_entry;

            trace_print_entry(e, i, &mut prefix);

            i += 1;
            p = (e as *mut u8)
                .add(size_of::<trace_entry>() + (*e).length as usize)
                as *mut c_void;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_print_location(tb: *mut trace_buffer) {
    unsafe {
        printf(c"Trace buffer 0x%llx bytes @ %p\n".as_ptr(), (*tb).size, tb);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
