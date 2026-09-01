// SPDX-License-Identifier: GPL-2.0
/*
 * VPA support
 */

use core::ffi::{c_char, c_int, c_void};

/* C includes removed; the following declarations are supplied by the
 * surrounding perf sources in the original repository:
 * errno.h, linux/kernel.h, linux/types.h, linux/string.h, linux/zalloc.h,
 * util/evlist.h, util/debug.h, util/auxtrace.h, util/powerpc-vpadtl.h,
 * util/record.h, and internal/lib.h for page_size.
 */

type size_t = usize;
type u64 = u64;

const ENOMEM: c_int = 12;
const UINT_MAX: u32 = u32::MAX;
const PERF_AUXTRACE_VPA_DTL: u32 = 5;
const VPADTL_AUXTRACE_PRIV_SIZE: size_t = 0;

const fn KiB(x: usize) -> usize {
    x * 1024
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_session {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    pub name: *const c_char,
    pub needs_auxtrace_mmap: bool,
    pub evlist: *mut evlist,
}

#[repr(C)]
pub struct record_opts {
    pub full_auxtrace: bool,
    pub auxtrace_mmap_pages: u32,
    pub mmap_pages: u32,
}

#[repr(C)]
pub struct perf_record_auxtrace_info {
    pub type_: u32,
}

#[repr(C)]
pub struct auxtrace_record {
    pub recording_options: Option<
        unsafe extern "C" fn(*mut auxtrace_record, *mut evlist, *mut record_opts) -> c_int,
    >,
    pub info_priv_size:
        Option<unsafe extern "C" fn(*mut auxtrace_record, *mut evlist) -> size_t>,
    pub info_fill: Option<
        unsafe extern "C" fn(
            *mut auxtrace_record,
            *mut perf_session,
            *mut perf_record_auxtrace_info,
            size_t,
        ) -> c_int,
    >,
    pub free: Option<unsafe extern "C" fn(*mut auxtrace_record)>,
    pub reference: Option<unsafe extern "C" fn(*mut auxtrace_record) -> u64>,
}

unsafe extern "C" {
    static page_size: size_t;

    fn free(ptr: *mut c_void);
    fn zalloc(size: size_t) -> *mut c_void;
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool;
    fn evlist__to_front(evlist: *mut evlist, evsel: *mut evsel);
    fn pr_debug(fmt: *const c_char, ...);
}

unsafe extern "C" fn powerpc_vpadtl_recording_options(
    _ar: *mut auxtrace_record,
    _evlist: *mut evlist,
    opts: *mut record_opts,
) -> c_int {
    unsafe {
        (*opts).full_auxtrace = true;

        /*
         * Set auxtrace_mmap_pages to minimum
         * two pages
         */
        if (*opts).auxtrace_mmap_pages == 0 {
            (*opts).auxtrace_mmap_pages = (KiB(128) / page_size) as u32;
            if (*opts).mmap_pages == UINT_MAX {
                (*opts).mmap_pages = (KiB(256) / page_size) as u32;
            }
        }
    }

    0
}

unsafe extern "C" fn powerpc_vpadtl_info_priv_size(
    _itr: *mut auxtrace_record,
    _evlist: *mut evlist,
) -> size_t {
    VPADTL_AUXTRACE_PRIV_SIZE
}

unsafe extern "C" fn powerpc_vpadtl_info_fill(
    _itr: *mut auxtrace_record,
    _session: *mut perf_session,
    auxtrace_info: *mut perf_record_auxtrace_info,
    _priv_size: size_t,
) -> c_int {
    unsafe {
        (*auxtrace_info).type_ = PERF_AUXTRACE_VPA_DTL;
    }

    0
}

unsafe extern "C" fn powerpc_vpadtl_free(itr: *mut auxtrace_record) {
    unsafe {
        free(itr as *mut c_void);
    }
}

unsafe extern "C" fn powerpc_vpadtl_reference(_itr: *mut auxtrace_record) -> u64 {
    0
}

unsafe extern "C" {
    /* Rust translation placeholder for the original evlist__for_each_entry()
     * macro iteration over evlist entries.
     */
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__next(evlist: *mut evlist, pos: *mut evsel) -> *mut evsel;
}

#[no_mangle]
pub unsafe extern "C" fn auxtrace_record__init(
    evlist: *mut evlist,
    err: *mut c_int,
) -> *mut auxtrace_record {
    let mut aux: *mut auxtrace_record;
    let mut pos: *mut evsel;
    let mut found: c_int = 0;

    /*
     * Set err value to zero here. Any fail later
     * will set appropriate return code to err.
     */
    unsafe {
        *err = 0;

        pos = evlist__first(evlist);
        while !pos.is_null() {
            if strstarts((*pos).name, c"vpa_dtl".as_ptr()) {
                found = 1;
                (*pos).needs_auxtrace_mmap = true;
                break;
            }
            pos = evlist__next(evlist, pos);
        }

        if found == 0 {
            return core::ptr::null_mut();
        }

        /*
         * To obtain the auxtrace buffer file descriptor, the auxtrace event
         * must come first.
         */
        evlist__to_front((*pos).evlist, pos);

        aux = zalloc(core::mem::size_of::<auxtrace_record>()) as *mut auxtrace_record;
        if aux.is_null() {
            pr_debug(c"aux record is NULL\n".as_ptr());
            *err = -ENOMEM;
            return core::ptr::null_mut();
        }

        (*aux).recording_options = Some(powerpc_vpadtl_recording_options);
        (*aux).info_priv_size = Some(powerpc_vpadtl_info_priv_size);
        (*aux).info_fill = Some(powerpc_vpadtl_info_fill);
        (*aux).free = Some(powerpc_vpadtl_free);
        (*aux).reference = Some(powerpc_vpadtl_reference);
    }

    aux
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
