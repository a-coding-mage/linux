// Translated from perf/arch/s390/util/auxtrace.c
// C includes removed; referenced types/constants/functions are expected from
// surrounding perf/Linux bindings.

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;

const PERF_EVENT_CPUM_SF: u64 = 0xB0000; /* Event: Basic-sampling */
const PERF_EVENT_CPUM_SF_DIAG: u64 = 0xBD000; /* Event: Combined-sampling */
const DEFAULT_AUX_PAGES: u32 = 128;
const DEFAULT_FREQ: u32 = 4000;
const UINT_MAX: u32 = u32::MAX;
const ENOMEM: c_int = 12;

extern "C" {
    static PERF_AUXTRACE_S390_CPUMSF: u32;

    fn free(ptr: *mut c_void);
    fn zalloc(size: usize) -> *mut c_void;
    fn roundup_pow_of_two(n: u32) -> u32;
}

#[repr(C)]
pub struct perf_session {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_record_auxtrace_info {
    pub type_: u32,
}

#[repr(C)]
pub struct record_opts {
    pub full_auxtrace: bool,
    pub auxtrace_mmap_pages: u32,
    pub user_freq: u32,
}

#[repr(C)]
pub struct evlist_core {
    pub nr_entries: c_int,
}

#[repr(C)]
pub struct evlist {
    pub core: evlist_core,
}

#[repr(C)]
pub struct perf_event_attr {
    pub config: u64,
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
    pub needs_auxtrace_mmap: bool,
}

#[repr(C)]
pub struct auxtrace_record {
    pub parse_snapshot_options: Option<
        unsafe extern "C" fn(
            itr: *mut auxtrace_record,
            opts: *mut record_opts,
            str_: *const c_char,
        ) -> c_int,
    >,
    pub recording_options: Option<
        unsafe extern "C" fn(
            ar: *mut auxtrace_record,
            evlist: *mut evlist,
            opts: *mut record_opts,
        ) -> c_int,
    >,
    pub info_priv_size:
        Option<unsafe extern "C" fn(itr: *mut auxtrace_record, evlist: *mut evlist) -> usize>,
    pub info_fill: Option<
        unsafe extern "C" fn(
            itr: *mut auxtrace_record,
            session: *mut perf_session,
            auxtrace_info: *mut perf_record_auxtrace_info,
            priv_size: usize,
        ) -> c_int,
    >,
    pub free: Option<unsafe extern "C" fn(itr: *mut auxtrace_record)>,
    pub reference: Option<unsafe extern "C" fn(itr: *mut auxtrace_record) -> c_ulong>,
}

// Rust translation note: the C source iterates with evlist__for_each_entry().
// The surrounding binding is expected to provide equivalent iteration support.
extern "C" {
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__next(evlist: *mut evlist, pos: *mut evsel) -> *mut evsel;
}

unsafe extern "C" fn cpumsf_free(itr: *mut auxtrace_record) {
    unsafe {
        free(itr as *mut c_void);
    }
}

unsafe extern "C" fn cpumsf_info_priv_size(
    _itr: *mut auxtrace_record,
    _evlist: *mut evlist,
) -> usize {
    0
}

unsafe extern "C" fn cpumsf_info_fill(
    _itr: *mut auxtrace_record,
    _session: *mut perf_session,
    auxtrace_info: *mut perf_record_auxtrace_info,
    _priv_size: usize,
) -> c_int {
    unsafe {
        (*auxtrace_info).type_ = PERF_AUXTRACE_S390_CPUMSF;
    }
    0
}

unsafe extern "C" fn cpumsf_reference(_itr: *mut auxtrace_record) -> c_ulong {
    0
}

unsafe extern "C" fn cpumsf_recording_options(
    _ar: *mut auxtrace_record,
    _evlist: *mut evlist,
    opts: *mut record_opts,
) -> c_int {
    let mut factor: u32 = 1;
    let pages: u32;

    unsafe {
        (*opts).full_auxtrace = true;

        /*
         * The AUX buffer size should be set properly to avoid
         * overflow of samples if it is not set explicitly.
         * DEFAULT_AUX_PAGES is an proper size when sampling frequency
         * is DEFAULT_FREQ. It is expected to hold about 1/2 second
         * of sampling data. The size used for AUX buffer will scale
         * according to the specified frequency and DEFAULT_FREQ.
         */
        if (*opts).auxtrace_mmap_pages == 0 {
            if (*opts).user_freq != UINT_MAX {
                factor = ((*opts).user_freq + DEFAULT_FREQ - 1) / DEFAULT_FREQ;
            }
            pages = DEFAULT_AUX_PAGES * factor;
            (*opts).auxtrace_mmap_pages = roundup_pow_of_two(pages);
        }
    }

    0
}

unsafe extern "C" fn cpumsf_parse_snapshot_options(
    _itr: *mut auxtrace_record,
    _opts: *mut record_opts,
    _str: *const c_char,
) -> c_int {
    0
}

/*
 * auxtrace_record__init is called when perf record
 * check if the event really need auxtrace
 */
#[no_mangle]
pub unsafe extern "C" fn auxtrace_record__init(
    evlist: *mut evlist,
    err: *mut c_int,
) -> *mut auxtrace_record {
    let mut aux: *mut auxtrace_record;
    let mut pos: *mut evsel;
    let mut diagnose: c_int = 0;

    unsafe {
        *err = 0;
        if (*evlist).core.nr_entries == 0 {
            return core::ptr::null_mut();
        }

        pos = evlist__first(evlist);
        while !pos.is_null() {
            if (*pos).core.attr.config == PERF_EVENT_CPUM_SF_DIAG {
                diagnose = 1;
                (*pos).needs_auxtrace_mmap = true;
                break;
            }
            pos = evlist__next(evlist, pos);
        }

        if diagnose == 0 {
            return core::ptr::null_mut();
        }

        /* sampling in diagnose mode. alloc aux buffer */
        aux = zalloc(size_of::<auxtrace_record>()) as *mut auxtrace_record;
        if aux.is_null() {
            *err = -ENOMEM;
            return core::ptr::null_mut();
        }

        (*aux).parse_snapshot_options = Some(cpumsf_parse_snapshot_options);
        (*aux).recording_options = Some(cpumsf_recording_options);
        (*aux).info_priv_size = Some(cpumsf_info_priv_size);
        (*aux).info_fill = Some(cpumsf_info_fill);
        (*aux).free = Some(cpumsf_free);
        (*aux).reference = Some(cpumsf_reference);

        aux
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
