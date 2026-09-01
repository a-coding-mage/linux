// SPDX-License-Identifier: GPL-2.0-only
/*
 * intel_pt_log.c: Intel Processor Trace support
 * Copyright (c) 2013-2014, Intel Corporation.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type SizeT = usize;
type SSizeT = isize;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct intel_pt_pkt {
    _private: [u8; 0],
}

#[repr(C)]
pub struct intel_pt_insn {
    pub buf: [u8; INTEL_PT_INSN_BUF_SZ],
    pub length: SizeT,
}

#[repr(C)]
pub struct cookie_io_functions_t {
    pub read: Option<unsafe extern "C" fn(*mut c_void, *mut c_char, SizeT) -> SSizeT>,
    pub write: Option<unsafe extern "C" fn(*mut c_void, *const c_char, SizeT) -> SSizeT>,
    pub seek: Option<unsafe extern "C" fn(*mut c_void, *mut i64, c_int) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
}

unsafe extern "C" {
    static mut stdout: *mut FILE;

    fn fflush(stream: *mut FILE) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fwrite(ptr: *const c_void, size: SizeT, nmemb: SizeT, stream: *mut FILE) -> SizeT;
    fn fopencookie(
        cookie: *mut c_void,
        mode: *const c_char,
        io_funcs: cookie_io_functions_t,
    ) -> *mut FILE;
    fn malloc(size: SizeT) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: SizeT) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: SizeT) -> *mut c_void;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: SizeT) -> *mut c_char;
    fn free(ptr: *mut c_void);

    fn intel_pt_pkt_desc(packet: *const intel_pt_pkt, desc: *mut c_char, len: SizeT);
    fn intel_pt_insn_desc(insn: *mut intel_pt_insn, desc: *mut c_char, len: SizeT) -> c_int;
}

const MAX_LOG_NAME: usize = 256;

const DFLT_BUF_SZ: usize = 16 * 1024;

const INTEL_PT_PKT_DESC_MAX: usize = 256;
const INTEL_PT_INSN_DESC_MAX: usize = 256;
const INTEL_PT_INSN_BUF_SZ: usize = 16;

#[repr(C)]
struct log_buf {
    buf: *mut c_char,
    buf_sz: SizeT,
    head: SizeT,
    wrapped: bool,
    backend: *mut FILE,
}

static mut f: *mut FILE = ptr::null_mut();
static mut log_name: [c_char; MAX_LOG_NAME] = [0; MAX_LOG_NAME];
#[unsafe(no_mangle)]
pub static mut intel_pt_enable_logging: bool = false;
static mut intel_pt_dump_log_on_error: bool = false;
static mut intel_pt_log_on_error_size: c_uint = 0;
static mut log_buf: log_buf = log_buf {
    buf: ptr::null_mut(),
    buf_sz: 0,
    head: 0,
    wrapped: false,
    backend: ptr::null_mut(),
};

unsafe fn zfree(ptrp: *mut *mut c_char) {
    if !(*ptrp).is_null() {
        free(*ptrp as *mut c_void);
        *ptrp = ptr::null_mut();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intel_pt_log_fp() -> *mut c_void {
    f as *mut c_void
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intel_pt_log_enable(dump_log_on_error: bool, log_on_error_size: c_uint) {
    intel_pt_enable_logging = true;
    intel_pt_dump_log_on_error = dump_log_on_error;
    intel_pt_log_on_error_size = log_on_error_size;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intel_pt_log_disable() {
    if !f.is_null() {
        fflush(f);
    }
    intel_pt_enable_logging = false;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intel_pt_log_set_name(name: *const c_char) {
    strncpy(log_name.as_mut_ptr(), name, MAX_LOG_NAME - 5);
    strcat(log_name.as_mut_ptr(), c".log".as_ptr());
}

unsafe fn intel_pt_print_data(buf: *const u8, len: c_int, pos: u64, indent: c_int) {
    let mut i: c_int;

    i = 0;
    while i < indent {
        fprintf(f, c" ".as_ptr());
        i += 1;
    }

    fprintf(f, c"  %08lx: ".as_ptr(), pos);
    i = 0;
    while i < len {
        fprintf(f, c" %02x".as_ptr(), *buf.add(i as usize) as c_int);
        i += 1;
    }
    while i < 16 {
        fprintf(f, c"   ".as_ptr());
        i += 1;
    }
    fprintf(f, c" ".as_ptr());
}

unsafe fn intel_pt_print_no_data(pos: u64, indent: c_int) {
    let mut i: c_int;

    i = 0;
    while i < indent {
        fprintf(f, c" ".as_ptr());
        i += 1;
    }

    fprintf(f, c"  %08lx: ".as_ptr(), pos);
    i = 0;
    while i < 16 {
        fprintf(f, c"   ".as_ptr());
        i += 1;
    }
    fprintf(f, c" ".as_ptr());
}

unsafe extern "C" fn log_buf__write(cookie: *mut c_void, mut buf: *const c_char, size: SizeT) -> SSizeT {
    let b = cookie as *mut log_buf;
    let mut sz = size;

    if (*b).buf.is_null() {
        return size as SSizeT;
    }

    while sz != 0 {
        let space = (*b).buf_sz - (*b).head;
        let n = if space < sz { space } else { sz };

        memcpy((*b).buf.add((*b).head) as *mut c_void, buf as *const c_void, n);
        sz -= n;
        buf = buf.add(n);
        (*b).head += n;
        if sz != 0 && (*b).head >= (*b).buf_sz {
            (*b).head = 0;
            (*b).wrapped = true;
        }
    }
    size as SSizeT
}

unsafe extern "C" fn log_buf__close(cookie: *mut c_void) -> c_int {
    let b = cookie as *mut log_buf;

    zfree(&mut (*b).buf);
    0
}

unsafe fn log_buf__open(b: *mut log_buf, backend: *mut FILE, sz: c_uint) -> *mut FILE {
    let fns = cookie_io_functions_t {
        read: None,
        write: Some(log_buf__write),
        seek: None,
        close: Some(log_buf__close),
    };
    let file: *mut FILE;

    memset(b as *mut c_void, 0, size_of::<log_buf>());
    (*b).buf_sz = sz as SizeT;
    (*b).buf = malloc((*b).buf_sz) as *mut c_char;
    (*b).backend = backend;
    file = fopencookie(b as *mut c_void, c"a".as_ptr(), fns);
    if file.is_null() {
        zfree(&mut (*b).buf);
    }
    file
}

unsafe fn remove_first_line(p: *mut *const c_char, n: *mut SizeT) -> bool {
    while *n != 0 && **p != b'\n' as c_char {
        *p = (*p).add(1);
        *n -= 1;
    }
    if *n != 0 {
        *p = (*p).add(1);
        *n -= 1;
        return true;
    }
    false
}

unsafe fn write_lines(mut p: *const c_char, mut n: SizeT, fp: *mut FILE, remove_first: *mut bool) {
    if *remove_first {
        *remove_first = !remove_first_line(&mut p, &mut n);
    }
    fwrite(p as *const c_void, n, 1, fp);
}

unsafe fn log_buf__dump(b: *mut log_buf) {
    let mut remove_first = false;

    if (*b).buf.is_null() {
        return;
    }

    fflush(f); /* Could update b->head and b->wrapped */
    fprintf((*b).backend, c"Dumping debug log buffer\n".as_ptr());
    if (*b).wrapped {
        remove_first = true;
        write_lines(
            (*b).buf.add((*b).head),
            (*b).buf_sz - (*b).head,
            (*b).backend,
            &mut remove_first,
        );
    }
    write_lines((*b).buf, (*b).head, (*b).backend, &mut remove_first);
    fprintf((*b).backend, c"End of debug log buffer dump\n".as_ptr());

    (*b).head = 0;
    (*b).wrapped = false;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn intel_pt_log_dump_buf() {
    log_buf__dump(&mut log_buf);
}

unsafe fn intel_pt_log_open() -> c_int {
    if !intel_pt_enable_logging {
        return -1;
    }

    if !f.is_null() {
        return 0;
    }

    if log_name[0] != 0 {
        f = fopen(log_name.as_ptr(), c"w+".as_ptr());
    } else {
        f = stdout;
    }
    if !f.is_null() && intel_pt_dump_log_on_error {
        f = log_buf__open(&mut log_buf, f, intel_pt_log_on_error_size);
    }
    if f.is_null() {
        intel_pt_enable_logging = false;
        return -1;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __intel_pt_log_packet(
    packet: *const intel_pt_pkt,
    pkt_len: c_int,
    pos: u64,
    buf: *const u8,
) {
    let mut desc: [c_char; INTEL_PT_PKT_DESC_MAX] = [0; INTEL_PT_PKT_DESC_MAX];

    if intel_pt_log_open() != 0 {
        return;
    }

    intel_pt_print_data(buf, pkt_len, pos, 0);
    intel_pt_pkt_desc(packet, desc.as_mut_ptr(), INTEL_PT_PKT_DESC_MAX);
    fprintf(f, c"%s\n".as_ptr(), desc.as_ptr());
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __intel_pt_log_insn(intel_pt_insn: *mut intel_pt_insn, ip: u64) {
    let mut desc: [c_char; INTEL_PT_INSN_DESC_MAX] = [0; INTEL_PT_INSN_DESC_MAX];
    let mut len = (*intel_pt_insn).length;

    if intel_pt_log_open() != 0 {
        return;
    }

    if len > INTEL_PT_INSN_BUF_SZ {
        len = INTEL_PT_INSN_BUF_SZ;
    }
    intel_pt_print_data((*intel_pt_insn).buf.as_ptr(), len as c_int, ip, 8);
    if intel_pt_insn_desc(intel_pt_insn, desc.as_mut_ptr(), INTEL_PT_INSN_DESC_MAX) > 0 {
        fprintf(f, c"%s\n".as_ptr(), desc.as_ptr());
    } else {
        fprintf(f, c"Bad instruction!\n".as_ptr());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
