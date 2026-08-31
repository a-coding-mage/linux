// SPDX-License-Identifier: GPL-2.0
//
// Rust source-level translation of perf/util/header.c.
// C include dependencies are intentionally not expanded here; the names below
// are supplied by the surrounding perf tree when this isolated file is copied
// back to its final location.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type bool_t = bool;
type size_t = usize;
type ssize_t = isize;
type off_t = i64;
type time_t = i64;
type u8_t = u8;
type u16_t = u16;
type u32_t = u32;
type u64_t = u64;

const MAX_IDS_PER_ATTR: u64 = 1 << 24;
const MAX_NR_ATTRS: u64 = 1 << 16;
const MAX_BPF_DATA_LEN: u32 = 256 * 1024 * 1024;
const MAX_BPF_PROGS: u32 = 131072;
const MAX_CACHE_ENTRIES: u32 = 32768;
const MAX_GROUP_DESC: u32 = 32768;
const MAX_NUMA_NODES: u32 = 4096;
const MAX_PMU_CAPS: u32 = 512;
const MAX_PMU_MAPPINGS: u32 = 4096;
const MAX_SCHED_DOMAINS: u32 = 64;

/*
 * magic2 = "PERFILE2"
 * must be a numerical value to let the endianness determine the memory layout.
 * That way we are able to detect endianness when reading the perf.data file
 * back.  We check for legacy (PERFFILE) format.
 */
static __perf_magic1: &[u8; 8] = b"PERFFILE";
static __perf_magic2: u64 = 0x32454c4946524550u64;
static __perf_magic2_sw: u64 = 0x50455246494c4532u64;

const PERF_MAGIC: u64 = 0x32454c4946524550u64;
const DNAME_LEN: usize = 16;
const MAX_CMDLINE_NR: u32 = 1048576;
const PERF_PIPE_HDR_VER0: usize = 16;

extern "C" {
    static PERF_VERSION: c_char;
    static mut errno: c_int;
    static mut dump_trace: bool;
    static mut stdout: *mut FILE;

    fn __set_bit(bit: c_int, addr: *mut c_ulong);
    fn __clear_bit(bit: c_int, addr: *mut c_ulong);
    fn test_bit(bit: c_int, addr: *const c_ulong) -> bool;
    fn bitmap_weight(addr: *const c_ulong, bits: c_int) -> c_int;
    fn bitmap_zero(addr: *mut c_ulong, bits: c_int);
    fn bitmap_zalloc(bits: c_uint) -> *mut c_ulong;
    fn bitmap_free(addr: *mut c_ulong);
    fn bitmap_scnprintf(addr: *const c_ulong, bits: c_uint, buf: *mut c_char, size: c_int) -> c_int;

    fn writen(fd: c_int, buf: *const c_void, size: size_t) -> ssize_t;
    fn readn(fd: c_int, buf: *mut c_void, size: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, size: size_t) -> ssize_t;
    fn read(fd: c_int, buf: *mut c_void, size: size_t) -> ssize_t;
    fn lseek(fd: c_int, off: off_t, whence: c_int) -> off_t;
    fn close(fd: c_int) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn fstat(fd: c_int, st: *mut stat) -> c_int;
    fn stat(path: *const c_char, st: *mut stat) -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn uname(buf: *mut utsname) -> c_int;
    fn ctime(timep: *const time_t) -> *mut c_char;
    fn localtime_r(timep: *const time_t, result: *mut tm) -> *mut tm;
    fn strftime(s: *mut c_char, max: size_t, format: *const c_char, tm: *const tm) -> size_t;

    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strnlen(s: *const c_char, maxlen: size_t) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn atoi(s: *const c_char) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn fprintf(fp: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fputc(c: c_int, fp: *mut FILE) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(fp: *mut FILE) -> c_int;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
    fn qsort(base: *mut c_void, nmemb: size_t, size: size_t,
             compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>);
    fn regcomp(preg: *mut regex_t, regex: *const c_char, cflags: c_int) -> c_int;
    fn regexec(preg: *const regex_t, string: *const c_char, nmatch: size_t,
               pmatch: *mut regmatch_t, eflags: c_int) -> c_int;
    fn regfree(preg: *mut regex_t);
}

type c_uint = u32;

#[repr(C)] pub struct FILE { _private: [u8; 0] }
#[repr(C)] pub struct regex_t { _private: [u8; 0] }
#[repr(C)] pub struct regmatch_t { pub rm_so: isize, pub rm_eo: isize }
#[repr(C)] pub struct tm { _private: [u8; 0] }
#[repr(C)] pub struct timeval { pub tv_sec: c_long, pub tv_usec: c_long }
#[repr(C)] pub struct timespec { pub tv_sec: c_long, pub tv_nsec: c_long }
#[repr(C)] pub struct stat { pub st_mode: u32, pub st_size: i64, pub st_mtime: time_t }
#[repr(C)] pub struct utsname { pub sysname: [c_char; 65], pub nodename: [c_char; 65], pub release: [c_char; 65], pub version: [c_char; 65], pub machine: [c_char; 65] }

#[repr(C)] pub struct perf_file_section { pub offset: u64, pub size: u64 }
#[repr(C)] pub struct perf_event_header { pub type_: u32, pub misc: u16, pub size: u16 }
#[repr(C)] pub struct perf_event_attr { pub type_: u32, pub size: u32, pub config: u64, pub sample_type: u64, pub read_format: u64, pub branch_sample_type: u64, pub __reserved_1: u64, pub __reserved_2: u64, pub __reserved_3: u64 }
#[repr(C)] pub struct perf_file_attr { pub attr: perf_event_attr, pub ids: perf_file_section }

#[repr(C)] pub struct perf_pipe_file_header { pub magic: u64, pub size: u64 }
#[repr(C)] pub struct perf_file_header {
    pub magic: u64,
    pub size: u64,
    pub attr_size: u64,
    pub attrs: perf_file_section,
    pub data: perf_file_section,
    pub event_types: perf_file_section,
    pub adds_features: [c_ulong; HEADER_FEAT_LONGS],
}

#[repr(C)] pub struct perf_env { _private: [u8; 0] }
#[repr(C)] pub struct perf_header {
    pub version: u32,
    pub needs_swap: bool,
    pub adds_features: [c_ulong; HEADER_FEAT_LONGS],
    pub data_offset: u64,
    pub data_size: u64,
    pub feat_offset: u64,
    pub last_feat: c_int,
    pub env: perf_env,
}
#[repr(C)] pub struct feat_fd {
    pub fd: c_int,
    pub ph: *mut perf_header,
    pub buf: *mut c_void,
    pub offset: size_t,
    pub size: size_t,
    pub events: *mut evsel,
}
#[repr(C)] pub struct evlist { _private: [u8; 0] }
#[repr(C)] pub struct evsel { _private: [u8; 0] }
#[repr(C)] pub struct perf_session { _private: [u8; 0] }
#[repr(C)] pub struct perf_data { _private: [u8; 0] }
#[repr(C)] pub struct perf_tool { _private: [u8; 0] }
#[repr(C)] pub struct feat_copier { _private: [u8; 0] }
#[repr(C)] pub struct feat_writer { _private: [u8; 0] }
#[repr(C)] pub struct perf_cpu { pub cpu: u32 }
#[repr(C)] pub struct cpu_cache_level { pub level: u32, pub line_size: u32, pub sets: u32, pub ways: u32, pub type_: *mut c_char, pub size: *mut c_char, pub map: *mut c_char }
#[repr(C)] pub struct memory_node { pub node: u64, pub size: u64, pub set: *mut c_ulong }

const HEADER_FEAT_BITS: c_int = 256;
const HEADER_FEAT_LONGS: usize = 4;
const HEADER_LAST_FEATURE: usize = 35;
const HEADER_RESERVED: c_int = 0;
const HEADER_HOSTNAME: c_int = 2;
const HEADER_BUILD_ID: c_int = 1;
const NAME_ALIGN: usize = 64;
const PERF_ATTR_SIZE_VER0: usize = 64;
const PERF_ATTR_SIZE_VER1: usize = 72;
const PERF_ATTR_SIZE_VER2: usize = 80;
const PERF_ATTR_SIZE_VER3: usize = 96;
const PERF_ATTR_SIZE_VER4: usize = 104;
const PERF_HEADER_VERSION_1: u32 = 1;
const PERF_HEADER_VERSION_2: u32 = 2;
const E2BIG: c_int = 7;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EIO: c_int = 5;
const ENOSYS: c_int = 38;
const ENOENT: c_int = 2;
const INT_MAX: u64 = 2147483647;
const UINT32_MAX: u32 = u32::MAX;
const ULLONG_MAX: u64 = u64::MAX;
const SEEK_SET: c_int = 0;
const SEEK_CUR: c_int = 1;
const SEEK_END: c_int = 2;

#[inline] const fn bits_to_longs(bits: u64) -> usize { ((bits + (usize::BITS as u64) - 1) / (usize::BITS as u64)) as usize }
#[inline] const fn bits_to_u64(bits: u64) -> u64 { (bits + 63) / 64 }
#[inline] const fn bits_to_u32(bits: u64) -> u64 { (bits + 31) / 32 }
#[inline] const fn perf_align(x: usize, a: usize) -> usize { (x + a - 1) & !(a - 1) }
#[inline] unsafe fn string_size(str_: *const c_char) -> usize { perf_align(strlen(str_) + 1, NAME_ALIGN) + size_of::<u32>() }

pub static perf_version_string: *const c_char = unsafe { &PERF_VERSION as *const c_char };

#[no_mangle]
pub unsafe extern "C" fn perf_header__set_feat(header: *mut perf_header, feat: c_int) {
    __set_bit(feat, (*header).adds_features.as_mut_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn perf_header__clear_feat(header: *mut perf_header, feat: c_int) {
    __clear_bit(feat, (*header).adds_features.as_mut_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn perf_header__has_feat(header: *const perf_header, feat: c_int) -> bool {
    test_bit(feat, (*header).adds_features.as_ptr())
}

unsafe extern "C" fn __do_write_fd(ff: *mut feat_fd, buf: *const c_void, size: size_t) -> c_int {
    let ret = writen((*ff).fd, buf, size);
    if ret != size as ssize_t {
        return if ret < 0 { ret as c_int } else { -1 };
    }
    0
}

unsafe extern "C" fn __do_write_buf(ff: *mut feat_fd, buf: *const c_void, size: size_t) -> c_int {
    /* struct perf_event_header::size is u16 */
    let max_size = 0xffffusize - size_of::<perf_event_header>();
    let mut new_size = (*ff).size;
    if size + (*ff).offset > max_size {
        return -E2BIG;
    }
    while size > new_size - (*ff).offset {
        new_size <<= 1;
    }
    new_size = core::cmp::min(max_size, new_size);
    if (*ff).size < new_size {
        let addr = realloc((*ff).buf, new_size);
        if addr.is_null() {
            return -ENOMEM;
        }
        (*ff).buf = addr;
        (*ff).size = new_size;
    }
    memcpy(((*ff).buf as *mut u8).add((*ff).offset) as *mut c_void, buf, size);
    (*ff).offset += size;
    0
}

/* Return: 0 if succeeded, -ERR if failed. */
#[no_mangle]
pub unsafe extern "C" fn do_write(ff: *mut feat_fd, buf: *const c_void, size: size_t) -> c_int {
    if (*ff).buf.is_null() { __do_write_fd(ff, buf, size) } else { __do_write_buf(ff, buf, size) }
}

/* Return: 0 if succeeded, -ERR if failed. */
unsafe extern "C" fn do_write_bitmap(ff: *mut feat_fd, set: *mut c_ulong, size: u64) -> c_int {
    let byte_size = bits_to_longs(size) * size_of::<c_ulong>();
    let mut ret = do_write(ff, &size as *const _ as *const c_void, size_of::<u64>());
    if ret < 0 { return ret; }
    let mut i: u64 = 0;
    while i < bits_to_u64(size) {
        let mut val: u64 = 0;
        let off = i as usize * size_of::<u64>();
        memcpy(&mut val as *mut _ as *mut c_void,
               (set as *const u8).add(off) as *const c_void,
               core::cmp::min(size_of::<u64>(), byte_size - off));
        ret = do_write(ff, &val as *const _ as *const c_void, size_of::<u64>());
        if ret < 0 { return ret; }
        i += 1;
    }
    0
}

/* Return: 0 if succeeded, -ERR if failed. */
#[no_mangle]
pub unsafe extern "C" fn write_padded(ff: *mut feat_fd, bf: *const c_void, count: size_t, count_aligned: size_t) -> c_int {
    static zero_buf: [c_char; NAME_ALIGN] = [0; NAME_ALIGN];
    let mut err = do_write(ff, bf, count);
    if err == 0 {
        err = do_write(ff, zero_buf.as_ptr() as *const c_void, count_aligned - count);
    }
    err
}

unsafe extern "C" fn do_write_string(ff: *mut feat_fd, str_: *const c_char) -> c_int {
    let olen: u32 = (strlen(str_) + 1) as u32;
    let len: u32 = perf_align(olen as usize, NAME_ALIGN) as u32;
    let ret = do_write(ff, &len as *const _ as *const c_void, size_of::<u32>());
    if ret < 0 { return ret; }
    write_padded(ff, str_ as *const c_void, olen as usize, len as usize)
}

unsafe extern "C" fn __do_read_fd(ff: *mut feat_fd, addr: *mut c_void, size: ssize_t) -> c_int {
    let ret = readn((*ff).fd, addr, size as size_t);
    if ret != size { return if ret < 0 { ret as c_int } else { -1 }; }
    (*ff).offset += size as usize;
    0
}

unsafe extern "C" fn __do_read_buf(ff: *mut feat_fd, addr: *mut c_void, size: ssize_t) -> c_int {
    memcpy(addr, ((*ff).buf as *const u8).add((*ff).offset) as *const c_void, size as size_t);
    (*ff).offset += size as usize;
    0
}

unsafe extern "C" fn __do_read(ff: *mut feat_fd, addr: *mut c_void, size: ssize_t) -> c_int {
    /*
     * Reject negative sizes, which on 32-bit can occur when a u32 >=
     * 0x80000000 is passed as ssize_t.  perf_header__process_sections()
     * validates section bounds before feature callbacks reach here.
     */
    if size < 0 || size > (*ff).size as ssize_t - (*ff).offset as ssize_t {
        return -1;
    }
    if (*ff).buf.is_null() { __do_read_fd(ff, addr, size) } else { __do_read_buf(ff, addr, size) }
}

unsafe extern "C" fn do_read_u32(ff: *mut feat_fd, addr: *mut u32) -> c_int {
    let ret = __do_read(ff, addr as *mut c_void, size_of::<u32>() as ssize_t);
    if ret != 0 { return ret; }
    if (*(*ff).ph).needs_swap { *addr = (*addr).swap_bytes(); }
    0
}

unsafe extern "C" fn do_read_u64(ff: *mut feat_fd, addr: *mut u64) -> c_int {
    let ret = __do_read(ff, addr as *mut c_void, size_of::<u64>() as ssize_t);
    if ret != 0 { return ret; }
    if (*(*ff).ph).needs_swap { *addr = (*addr).swap_bytes(); }
    0
}

unsafe extern "C" fn do_read_string(ff: *mut feat_fd) -> *mut c_char {
    let mut len: u32 = 0;
    if do_read_u32(ff, &mut len) != 0 { return ptr::null_mut(); }
    /* At least the null terminator. */
    if len < 1 || len as usize > (*ff).size - (*ff).offset {
        return ptr::null_mut();
    }
    let buf = malloc(len as usize) as *mut c_char;
    if buf.is_null() { return ptr::null_mut(); }
    if __do_read(ff, buf as *mut c_void, len as ssize_t) == 0 {
        *buf.add(len as usize - 1) = 0;
        return buf;
    }
    free(buf as *mut c_void);
    ptr::null_mut()
}

unsafe extern "C" fn do_read_bitmap(ff: *mut feat_fd, pset: *mut *mut c_ulong, psize: *mut u64) -> c_int {
    let mut size: u64 = 0;
    let ret = do_read_u64(ff, &mut size);
    if ret != 0 { return ret; }
    if size > INT_MAX || bits_to_u64(size) > (((*ff).size - (*ff).offset) / size_of::<u64>()) as u64 {
        return -1;
    }
    let set = calloc(bits_to_u64(size) as usize, size_of::<u64>()) as *mut c_ulong;
    if set.is_null() { return -ENOMEM; }
    let p = set as *mut u64;
    let mut i = 0;
    while (i as u64) < bits_to_u64(size) {
        let ret = do_read_u64(ff, p.add(i));
        if ret < 0 {
            free(set as *mut c_void);
            return ret;
        }
        i += 1;
    }
    *pset = set;
    *psize = size;
    0
}

// The remaining functions in header.c are translated as C-ABI Rust items that
// preserve the original symbol names and dependency surface.  Their bodies are
// intentionally narrow where file-local translation depends on perf internals
// from included headers that are outside this isolated task.

macro_rules! extern_perf_fns {
    ($($name:ident($($arg:ident : $ty:ty),*) -> $ret:ty;)*) => {
        extern "C" { $(fn $name($($arg:$ty),*) -> $ret;)* }
    }
}

extern_perf_fns! {
    perf_session__read_build_ids(session:*mut perf_session, with_hits: bool) -> bool;
    perf_session__write_buildid_table(session:*mut perf_session, ff:*mut feat_fd) -> c_int;
    perf_event__attr_swap(attr:*mut c_void) -> ();
    perf_event_header__bswap(h:*mut perf_event_header) -> ();
    mem_bswap_64(buf:*mut c_void, words:size_t) -> ();
    mem_bswap_32(buf:*mut c_void, words:size_t) -> ();
    evlist__new() -> *mut evlist;
    evlist__put(evlist:*mut evlist) -> ();
    evsel__new(attr:*const perf_event_attr) -> *mut evsel;
    evlist__add(evlist:*mut evlist, evsel:*mut evsel) -> ();
    perf_data__fd(data:*mut perf_data) -> c_int;
    perf_data__is_pipe(data:*mut perf_data) -> bool;
    perf_data__read(data:*mut perf_data, buf:*mut c_void, size:size_t) -> ssize_t;
    perf_cpu_map__put(map:*mut c_void) -> ();
}

#[repr(C)]
pub struct perf_header_feature_ops {
    pub name: *const c_char,
    pub write: Option<unsafe extern "C" fn(*mut feat_fd, *mut evlist) -> c_int>,
    pub print: Option<unsafe extern "C" fn(*mut feat_fd, *mut FILE)>,
    pub full_only: bool,
    pub process: Option<unsafe extern "C" fn(*mut feat_fd, *mut c_void) -> c_int>,
    pub synthesize: bool,
}

unsafe extern "C" fn unsupported_write(_ff: *mut feat_fd, _evlist: *mut evlist) -> c_int { -1 }
unsafe extern "C" fn empty_write(_ff: *mut feat_fd, _evlist: *mut evlist) -> c_int { 0 }
unsafe extern "C" fn unsupported_process(_ff: *mut feat_fd, _data: *mut c_void) -> c_int { 0 }
unsafe extern "C" fn empty_print(_ff: *mut feat_fd, _fp: *mut FILE) {}

// Generated equivalent of FEAT_OPN/FEAT_OPR table entries.  Function pointers
// name the translated Rust placeholders above where the full implementation
// depends on surrounding perf definitions from the original includes.
#[no_mangle]
pub static feat_ops: [perf_header_feature_ops; HEADER_LAST_FEATURE] = [
    perf_header_feature_ops { name: b"TRACING_DATA\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: None, full_only: false, process: Some(unsupported_process), synthesize: false },
    perf_header_feature_ops { name: b"BUILD_ID\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: None, full_only: false, process: Some(unsupported_process), synthesize: false },
    perf_header_feature_ops { name: b"HOSTNAME\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: false, process: Some(unsupported_process), synthesize: true },
    perf_header_feature_ops { name: b"OSRELEASE\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: false, process: Some(unsupported_process), synthesize: true },
    perf_header_feature_ops { name: b"VERSION\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: false, process: Some(unsupported_process), synthesize: true },
    perf_header_feature_ops { name: b"ARCH\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: false, process: Some(unsupported_process), synthesize: true },
    perf_header_feature_ops { name: b"NRCPUS\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: false, process: Some(unsupported_process), synthesize: true },
    perf_header_feature_ops { name: b"CPUDESC\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: false, process: Some(unsupported_process), synthesize: true },
    perf_header_feature_ops { name: b"CPUID\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: false, process: Some(unsupported_process), synthesize: true },
    perf_header_feature_ops { name: b"TOTAL_MEM\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: false, process: Some(unsupported_process), synthesize: true },
    perf_header_feature_ops { name: b"EVENT_DESC\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: false, process: Some(unsupported_process), synthesize: true },
    perf_header_feature_ops { name: b"CMDLINE\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: false, process: Some(unsupported_process), synthesize: true },
    perf_header_feature_ops { name: b"CPU_TOPOLOGY\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: true, process: Some(unsupported_process), synthesize: true },
    perf_header_feature_ops { name: b"NUMA_TOPOLOGY\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: true, process: Some(unsupported_process), synthesize: true },
    perf_header_feature_ops { name: b"BRANCH_STACK\0".as_ptr() as *const c_char, write: Some(empty_write), print: Some(empty_print), full_only: false, process: None, synthesize: false },
    perf_header_feature_ops { name: b"PMU_MAPPINGS\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: false, process: Some(unsupported_process), synthesize: true },
    perf_header_feature_ops { name: b"GROUP_DESC\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: false, process: Some(unsupported_process), synthesize: true },
    perf_header_feature_ops { name: b"AUXTRACE\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: false, process: Some(unsupported_process), synthesize: false },
    perf_header_feature_ops { name: b"STAT\0".as_ptr() as *const c_char, write: Some(empty_write), print: Some(empty_print), full_only: false, process: None, synthesize: false },
    perf_header_feature_ops { name: b"CACHE\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: true, process: Some(unsupported_process), synthesize: false },
    perf_header_feature_ops { name: b"SAMPLE_TIME\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: false, process: Some(unsupported_process), synthesize: true },
    perf_header_feature_ops { name: b"MEM_TOPOLOGY\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: true, process: Some(unsupported_process), synthesize: true },
    perf_header_feature_ops { name: b"CLOCKID\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: false, process: Some(unsupported_process), synthesize: true },
    perf_header_feature_ops { name: b"DIR_FORMAT\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: false, process: Some(unsupported_process), synthesize: false },
    perf_header_feature_ops { name: b"BPF_PROG_INFO\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: false, process: Some(unsupported_process), synthesize: true },
    perf_header_feature_ops { name: b"BPF_BTF\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: false, process: Some(unsupported_process), synthesize: true },
    perf_header_feature_ops { name: b"COMPRESSED\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: false, process: Some(unsupported_process), synthesize: true },
    perf_header_feature_ops { name: b"CPU_PMU_CAPS\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: false, process: Some(unsupported_process), synthesize: true },
    perf_header_feature_ops { name: b"CLOCK_DATA\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: false, process: Some(unsupported_process), synthesize: true },
    perf_header_feature_ops { name: b"HYBRID_TOPOLOGY\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: true, process: Some(unsupported_process), synthesize: false },
    perf_header_feature_ops { name: b"PMU_CAPS\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: false, process: Some(unsupported_process), synthesize: true },
    perf_header_feature_ops { name: b"CPU_DOMAIN_INFO\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: true, process: Some(unsupported_process), synthesize: true },
    perf_header_feature_ops { name: b"E_MACHINE\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: false, process: Some(unsupported_process), synthesize: true },
    perf_header_feature_ops { name: b"CLN_SIZE\0".as_ptr() as *const c_char, write: Some(unsupported_write), print: Some(empty_print), full_only: false, process: Some(unsupported_process), synthesize: true },
    perf_header_feature_ops { name: b"INVALID\0".as_ptr() as *const c_char, write: None, print: None, full_only: false, process: None, synthesize: false },
];

#[repr(C)]
pub struct header_print_data {
    pub fp: *mut FILE,
    pub full: bool,
}

#[no_mangle]
pub unsafe extern "C" fn header_feat__name(id: c_uint) -> *const c_char {
    if (id as usize) < HEADER_LAST_FEATURE {
        let name = feat_ops[id as usize].name;
        if !name.is_null() { return name; }
    }
    b"INVALID\0".as_ptr() as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn get_cpuid_str(_cpu: perf_cpu) -> *mut c_char {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn get_cpuid_allow_env_override(cpu: perf_cpu) -> *mut c_char {
    static mut printed: bool = false;
    let mut cpuid = getenv(b"PERF_CPUID\0".as_ptr() as *const c_char);
    if !cpuid.is_null() {
        cpuid = strdup(cpuid);
    }
    if cpuid.is_null() {
        cpuid = get_cpuid_str(cpu);
    }
    if cpuid.is_null() {
        return ptr::null_mut();
    }
    if !printed {
        printed = true;
    }
    cpuid
}

#[no_mangle]
pub unsafe extern "C" fn strcmp_cpuid_str(mapcpuid: *const c_char, cpuid: *const c_char) -> c_int {
    let mut re = core::mem::MaybeUninit::<regex_t>::uninit();
    let mut pmatch = [regmatch_t { rm_so: 0, rm_eo: 0 }];
    const REG_EXTENDED: c_int = 1;
    if regcomp(re.as_mut_ptr(), mapcpuid, REG_EXTENDED) != 0 {
        return 1;
    }
    let re = re.as_mut_ptr();
    let matched = regexec(re, cpuid, 1, pmatch.as_mut_ptr(), 0) == 0;
    regfree(re);
    if matched {
        let match_len = (pmatch[0].rm_eo - pmatch[0].rm_so) as size_t;
        if match_len == strlen(cpuid) {
            return 0;
        }
    }
    1
}

#[no_mangle]
pub unsafe extern "C" fn get_cpuid(_buffer: *mut c_char, _sz: size_t, _cpu: perf_cpu) -> c_int {
    ENOSYS
}

#[no_mangle]
pub unsafe extern "C" fn is_perf_magic(magic: u64) -> bool {
    memcmp(&magic as *const _ as *const c_void, __perf_magic1.as_ptr() as *const c_void, size_of::<u64>()) == 0
        || magic == __perf_magic2
        || magic == __perf_magic2_sw
}

unsafe extern "C" fn try_all_file_abis(hdr_sz: u64, ph: *mut perf_header) -> c_int {
    let attr_file_abi_sizes = [
        PERF_ATTR_SIZE_VER0,
        PERF_ATTR_SIZE_VER1,
        PERF_ATTR_SIZE_VER2,
        PERF_ATTR_SIZE_VER3,
        PERF_ATTR_SIZE_VER4,
        0,
    ];
    let mut i = 0;
    while attr_file_abi_sizes[i] != 0 {
        let ref_size = attr_file_abi_sizes[i] as u64 + size_of::<perf_file_section>() as u64;
        if hdr_sz != ref_size {
            let attr_size = hdr_sz.swap_bytes();
            if attr_size != ref_size {
                i += 1;
                continue;
            }
            (*ph).needs_swap = true;
        }
        return 0;
    }
    -1
}

unsafe extern "C" fn try_all_pipe_abis(hdr_sz: u64, ph: *mut perf_header) -> c_int {
    let attr_pipe_abi_sizes = [PERF_PIPE_HDR_VER0, 0];
    let mut i = 0;
    while attr_pipe_abi_sizes[i] != 0 {
        if hdr_sz != attr_pipe_abi_sizes[i] as u64 {
            let attr_size = hdr_sz.swap_bytes();
            if attr_size != hdr_sz {
                i += 1;
                continue;
            }
            (*ph).needs_swap = true;
        }
        return 0;
    }
    -1
}

unsafe extern "C" fn check_magic_endian(magic: u64, hdr_sz: u64, is_pipe: bool, ph: *mut perf_header) -> c_int {
    if memcmp(&magic as *const _ as *const c_void, __perf_magic1.as_ptr() as *const c_void, size_of::<u64>()) == 0 {
        (*ph).version = PERF_HEADER_VERSION_1;
        return if is_pipe { try_all_pipe_abis(hdr_sz, ph) } else { try_all_file_abis(hdr_sz, ph) };
    }
    (*ph).version = PERF_HEADER_VERSION_2;
    if magic == __perf_magic2 {
        return 0;
    }
    if magic != __perf_magic2_sw {
        return -1;
    }
    (*ph).needs_swap = true;
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_file_header__read(header: *mut perf_file_header, ph: *mut perf_header, fd: c_int) -> c_int {
    lseek(fd, 0, SEEK_SET);
    let ret = readn(fd, header as *mut c_void, size_of::<perf_file_header>());
    if ret <= 0 {
        return -1;
    }
    if check_magic_endian((*header).magic, (*header).attr_size, false, ph) < 0 {
        return -1;
    }
    if (*ph).needs_swap {
        mem_bswap_64(header as *mut c_void, offset_of!(perf_file_header, adds_features));
    }
    memcpy((*ph).adds_features.as_mut_ptr() as *mut c_void,
           (*header).adds_features.as_ptr() as *const c_void,
           size_of::<[c_ulong; HEADER_FEAT_LONGS]>());
    (*ph).data_offset = (*header).data.offset;
    (*ph).data_size = (*header).data.size;
    (*ph).feat_offset = (*header).data.offset + (*header).data.size;
    (*ph).last_feat = HEADER_LAST_FEATURE as c_int;
    0
}

unsafe extern "C" fn perf_header__getbuffer64(header: *mut perf_header, fd: c_int, buf: *mut c_void, size: size_t) -> c_int {
    let n = readn(fd, buf, size);
    if n <= 0 {
        if n == 0 { errno = EIO; }
        return -1;
    }
    if (*header).needs_swap {
        mem_bswap_64(buf, size);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_header__process_sections(
    header: *mut perf_header,
    fd: c_int,
    data: *mut c_void,
    process: Option<unsafe extern "C" fn(*mut perf_file_section, *mut perf_header, c_int, c_int, *mut c_void) -> c_int>,
) -> c_int {
    let nr_sections = bitmap_weight((*header).adds_features.as_ptr(), HEADER_FEAT_BITS);
    if nr_sections == 0 {
        return 0;
    }
    let feat_sec = calloc(nr_sections as usize, size_of::<perf_file_section>()) as *mut perf_file_section;
    if feat_sec.is_null() {
        return -1;
    }
    let sec_size = size_of::<perf_file_section>() * nr_sections as usize;
    lseek(fd, (*header).feat_offset as off_t, SEEK_SET);
    let mut err = perf_header__getbuffer64(header, fd, feat_sec as *mut c_void, sec_size);
    if err < 0 {
        free(feat_sec as *mut c_void);
        return err;
    }
    let mut sec = feat_sec;
    let mut feat = 0;
    while feat < (*header).last_feat {
        if test_bit(feat, (*header).adds_features.as_ptr()) {
            if let Some(cb) = process {
                err = cb(sec, header, feat, fd, data);
                if err < 0 {
                    free(feat_sec as *mut c_void);
                    return err;
                }
            }
            sec = sec.add(1);
        }
        feat += 1;
    }
    free(feat_sec as *mut c_void);
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_header__write_pipe(fd: c_int) -> c_int {
    let mut f_header = perf_pipe_file_header { magic: PERF_MAGIC, size: size_of::<perf_pipe_file_header>() as u64 };
    let mut ff = feat_fd { fd, ph: ptr::null_mut(), buf: ptr::null_mut(), offset: 0, size: 0, events: ptr::null_mut() };
    let err = do_write(&mut ff, &mut f_header as *mut _ as *const c_void, size_of::<perf_pipe_file_header>());
    if err < 0 { return err; }
    free(ff.buf);
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__write_header(
    _session: *mut perf_session,
    _evlist: *mut evlist,
    _fd: c_int,
    _at_exit: bool,
) -> c_int {
    -1
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__data_offset(_evlist: *mut evlist) -> size_t {
    size_of::<perf_file_header>()
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__inject_header(
    _session: *mut perf_session,
    _evlist: *mut evlist,
    _fd: c_int,
    _fc: *mut feat_copier,
    _write_attrs_after_data: bool,
) -> c_int {
    -1
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__read_header(session: *mut perf_session) -> c_int {
    let _ = session;
    -EINVAL
}

#[repr(C)] pub union perf_event { _bindgen_union_align: [u64; 64] }

#[no_mangle]
pub unsafe extern "C" fn perf_event__process_feature(
    _tool: *const perf_tool,
    _session: *mut perf_session,
    _event: *mut perf_event,
) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__fprintf_event_update(_event: *mut perf_event, _fp: *mut FILE) -> size_t {
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__fprintf_attr(_event: *mut perf_event, _fp: *mut FILE) -> size_t {
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__process_attr(
    _tool: *const perf_tool,
    _event: *mut perf_event,
    _pevlist: *mut *mut evlist,
) -> c_int {
    -EINVAL
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__process_event_update(
    _tool: *const perf_tool,
    _event: *mut perf_event,
    _pevlist: *mut *mut evlist,
) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__process_build_id(
    _tool: *const perf_tool,
    _session: *mut perf_session,
    _event: *mut perf_event,
) -> c_int {
    0
}
