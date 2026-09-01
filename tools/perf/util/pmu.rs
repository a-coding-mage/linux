// SPDX-License-Identifier: GPL-2.0
/*
 * Rust translation of perf/util/pmu.c.
 *
 * C include dependencies intentionally remain external to this isolated
 * translation.  The declarations below model the file-local ABI surface and
 * the external symbols used by the original implementation.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type FILE = c_void;
type DIR = c_void;
type regex_t = c_void;
type u64 = u64;
type __u64 = u64;
type __u32 = u32;

const UNIT_MAX_LEN: usize = 31; /* max length for event unit name */
const PATH_MAX: usize = 4096;
const NAME_MAX: usize = 255;
const PERF_PMU_FORMAT_BITS: usize = 64;
const PERF_PMU_FORMAT_VALUE_CONFIG: c_int = 0;
const PERF_PMU_FORMAT_VALUE_CONFIG1: c_int = 1;
const PERF_PMU_FORMAT_VALUE_CONFIG2: c_int = 2;
const PERF_PMU_FORMAT_VALUE_CONFIG3: c_int = 3;
const PERF_PMU_FORMAT_VALUE_CONFIG4: c_int = 4;
const PERF_PMU_FORMAT_VALUE_CONFIG_END: c_int = 5;
const PERF_TYPE_HARDWARE: __u32 = 0;
const PERF_TYPE_SOFTWARE: __u32 = 1;
const PERF_TYPE_TRACEPOINT: __u32 = 2;
const PERF_TYPE_HW_CACHE: __u32 = 3;
const PERF_TYPE_RAW: __u32 = 4;
const PERF_TYPE_BREAKPOINT: __u32 = 5;
const PERF_TYPE_MAX: __u32 = 6;
const PERF_PMU_TYPE_FAKE: __u32 = u32::MAX;
const PERF_PMU_TYPE_TOOL: __u32 = u32::MAX - 1;
const PERF_PMU_TYPE_PE_END: __u32 = PERF_TYPE_BREAKPOINT;
const PERF_PMU_TYPE_SHIFT: c_int = 32;
const PERF_HW_EVENT_MASK: u64 = 0xffff_ffff;
const TOOL_PMU__EVENT_USER_TIME: c_int = 0;
const TOOL_PMU__EVENT_SYSTEM_TIME: c_int = 1;
const O_RDONLY: c_int = 0;
const O_DIRECTORY: c_int = 0o200000;
const O_CLOEXEC: c_int = 0o2000000;
const O_PATH: c_int = 0o10000000;
const EOF_: c_int = -1;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const REG_EXTENDED: c_int = 1;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct perf_pmu_format {
    pub list: list_head,
    pub name: *mut c_char,
    pub value: c_int,
    pub bits: [c_ulong; 1],
    pub loaded: bool,
}

#[repr(C)]
pub struct perf_pmu_caps {
    pub list: list_head,
    pub name: *mut c_char,
    pub value: *mut c_char,
}

#[repr(C)]
pub struct perf_pmu_alias {
    /** @name: Name of the event like "mem-loads". */
    pub name: *mut c_char,
    /** @desc: Optional short description of the event. */
    pub desc: *mut c_char,
    /** @long_desc: Optional long description. */
    pub long_desc: *mut c_char,
    /** @topic: Optional topic such as cache or pipeline, particularly for json events. */
    pub topic: *mut c_char,
    /** @terms: Owned copy of the event terms. */
    pub terms: *mut c_char,
    /** @legacy_terms: If the event aliases a legacy event, holds a copy of the legacy event string. */
    pub legacy_terms: *mut c_char,
    /** @pmu_name: The name copied from the json struct pmu_event. */
    pub pmu_name: *mut c_char,
    /** @unit: Units for the event, such as bytes or cache lines. */
    pub unit: [c_char; UNIT_MAX_LEN + 1],
    /** @scale: Value to scale read counter values by. */
    pub scale: c_double,
    pub retirement_latency_mean: c_double,
    pub retirement_latency_min: c_double,
    pub retirement_latency_max: c_double,
    pub per_pkg: bool,
    pub snapshot: bool,
    pub deprecated: bool,
    pub legacy_deprecated_checked: bool,
    pub from_sysfs: bool,
    pub info_loaded: bool,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}
#[repr(C)]
pub struct hashmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct hashmap_entry {
    pub key: c_long,
    pub pkey: *mut c_void,
    pub value: c_long,
    pub pvalue: *mut c_void,
}
#[repr(C)]
pub struct pmu_events_table {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pmu_event {
    pub name: *const c_char,
    pub desc: *const c_char,
    pub long_desc: *const c_char,
    pub topic: *const c_char,
    pub event: *const c_char,
    pub unit: *const c_char,
    pub pmu: *const c_char,
    pub compat: *const c_char,
    pub perpkg: bool,
    pub deprecated: bool,
    pub retirement_latency_mean: *const c_char,
    pub retirement_latency_min: *const c_char,
    pub retirement_latency_max: *const c_char,
}
#[repr(C)]
pub struct perf_event_attr {
    pub type_: __u32,
    pub config: __u64,
    pub config1: __u64,
    pub config2: __u64,
    pub config3: __u64,
    pub config4: __u64,
}
#[repr(C)]
pub struct parse_events_terms {
    pub terms: list_head,
}
#[repr(C)]
pub union parse_events_term_val {
    pub num: __u64,
    pub str_: *mut c_char,
}
#[repr(C)]
pub struct parse_events_term {
    pub list: list_head,
    pub type_val: c_int,
    pub type_term: c_int,
    pub config: *mut c_char,
    pub val: parse_events_term_val,
    pub weak: bool,
    pub used: bool,
    pub no_value: bool,
    pub err_term: c_int,
    pub err_val: c_int,
}
#[repr(C)]
pub struct parse_events_error {
    _private: [u8; 0],
}
#[repr(C)]
pub struct perf_pmu_info {
    pub unit: *const c_char,
    pub scale: c_double,
    pub snapshot: bool,
    pub per_pkg: bool,
    pub retirement_latency_mean: c_double,
    pub retirement_latency_min: c_double,
    pub retirement_latency_max: c_double,
}
#[repr(C)]
pub struct pmu_event_info {
    pub pmu: *mut perf_pmu,
    pub pmu_name: *const c_char,
    pub name: *const c_char,
    pub alias: *const c_char,
    pub scale_unit: *const c_char,
    pub desc: *const c_char,
    pub long_desc: *const c_char,
    pub encoding_desc: *const c_char,
    pub str_: *const c_char,
    pub topic: *const c_char,
    pub event_type_desc: *const c_char,
    pub deprecated: bool,
}
#[repr(C)]
pub struct io {
    pub fd: c_int,
}
#[repr(C)]
pub struct io_dir {
    pub dirfd: c_int,
}
#[repr(C)]
pub struct io_dirent64 {
    pub d_name: [c_char; NAME_MAX + 1],
}
#[repr(C)]
pub struct strbuf {
    _private: [u8; 0],
}

pub type pmu_event_callback =
    Option<unsafe extern "C" fn(state: *mut c_void, info: *mut pmu_event_info) -> c_int>;
pub type pmu_format_callback = Option<
    unsafe extern "C" fn(
        state: *mut c_void,
        name: *const c_char,
        config: c_int,
        bits: *mut c_ulong,
    ) -> c_int,
>;

#[repr(C)]
pub struct perf_pmu {
    pub list: list_head,
    pub format: list_head,
    pub caps: list_head,
    pub name: *mut c_char,
    pub alias_name: *mut c_char,
    pub id: *mut c_char,
    pub type_: __u32,
    pub cpus: *mut perf_cpu_map,
    pub aliases: *mut hashmap,
    pub events_table: *const pmu_events_table,
    pub is_core: bool,
    pub is_uncore: bool,
    pub auxtrace: bool,
    pub selectable: bool,
    pub sysfs_aliases_loaded: bool,
    pub cpu_aliases_added: bool,
    pub formats_checked: bool,
    pub caps_initialized: bool,
    pub config_masks_computed: bool,
    pub config_masks_present: bool,
    pub perf_event_attr_init_default: bool,
    pub sysfs_aliases: size_t,
    pub cpu_json_aliases: size_t,
    pub sys_json_aliases: size_t,
    pub cpu_common_json_aliases: size_t,
    pub nr_caps: c_int,
    pub max_precise: c_int,
    pub config_masks: [__u64; PERF_PMU_FORMAT_VALUE_CONFIG_END as usize],
    pub mem_events: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum event_source {
    EVENT_SRC_SYSFS,
    EVENT_SRC_CPU_JSON,
    EVENT_SRC_SYS_JSON,
}

extern "C" {
    static mut errno: c_int;
    static mut verbose: c_int;
    static mut perf_mem_events: *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strndup(s: *const c_char, n: size_t) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn strcasecmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strstr(a: *const c_char, b: *const c_char) -> *mut c_char;
    fn strchr(a: *const c_char, c: c_int) -> *mut c_char;
    fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fdopen(fd: c_int, mode: *const c_char) -> *mut FILE;
    fn fclose(file: *mut FILE) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn openat(dirfd: c_int, path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn vfscanf(stream: *mut FILE, fmt: *const c_char, ap: *mut c_void) -> c_int;
    fn setlocale(category: c_int, locale: *const c_char) -> *mut c_char;
    fn strtod(nptr: *const c_char, endptr: *mut *mut c_char) -> c_double;
    fn tolower(c: c_int) -> c_int;
    fn toupper(c: c_int) -> c_int;
    fn isdigit(c: c_int) -> c_int;
    fn isxdigit(c: c_int) -> c_int;
    fn fnmatch(pattern: *const c_char, string: *const c_char, flags: c_int) -> c_int;
    fn regcomp(preg: *mut regex_t, regex: *const c_char, cflags: c_int) -> c_int;
    fn regexec(
        preg: *const regex_t,
        string: *const c_char,
        nmatch: size_t,
        pmatch: *mut regmatch_t,
        eflags: c_int,
    ) -> c_int;
    fn regfree(preg: *mut regex_t);
    fn zalloc(size: size_t) -> *mut c_void;
    fn zfree(ptr: *mut *mut c_char);
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_add_tail(new_: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn list_del_init(entry: *mut list_head);
    fn list_splice_init(list: *mut list_head, head: *mut list_head);
    fn hashmap__new(
        hash: Option<unsafe extern "C" fn(c_long, *mut c_void) -> size_t>,
        equal: Option<unsafe extern "C" fn(c_long, c_long, *mut c_void) -> bool>,
        ctx: *mut c_void,
    ) -> *mut hashmap;
    fn hashmap__find(map: *mut hashmap, key: *const c_char, value: *mut *mut perf_pmu_alias) -> bool;
    fn hashmap__set(
        map: *mut hashmap,
        key: *mut c_char,
        value: *mut perf_pmu_alias,
        old_key: *mut *mut c_void,
        old_value: *mut *mut perf_pmu_alias,
    ) -> c_int;
    fn hashmap__free(map: *mut hashmap);
    fn perf_pmu_lex_init(scanner: *mut *mut c_void) -> c_int;
    fn perf_pmu_set_in(file: *mut FILE, scanner: *mut c_void);
    fn perf_pmu_parse(format: *mut perf_pmu_format, scanner: *mut c_void) -> c_int;
    fn perf_pmu_lex_destroy(scanner: *mut c_void);
    fn io_dir__init(dir: *mut io_dir, dirfd: c_int);
    fn io_dir__readdir(dir: *mut io_dir) -> *mut io_dirent64;
    fn io_dir__is_dir(dir: *mut io_dir, ent: *mut io_dirent64) -> bool;
    fn io__init(io: *mut io, fd: c_int, buf: *mut c_char, size: size_t);
    fn io__getline(io: *mut io, line: *mut *mut c_char, line_len: *mut size_t) -> ssize_t;
    fn parse_events_terms__init(terms: *mut parse_events_terms);
    fn parse_events_terms__exit(terms: *mut parse_events_terms);
    fn parse_events_terms(terms: *mut parse_events_terms, str_: *const c_char) -> c_int;
    fn parse_events_term__delete(term: *mut parse_events_term);
    fn parse_events__is_hardcoded_term(term: *mut parse_events_term) -> bool;
    fn parse_events__term_type_str(term_type: c_int) -> *const c_char;
    fn parse_events_formats_error_string(pmu_term: *mut c_char) -> *mut c_char;
    fn parse_events_error__handle(
        err: *mut parse_events_error,
        idx: c_int,
        str_: *mut c_char,
        help: *mut c_char,
    );
    fn perf_pmu__default_core_events_table() -> *const pmu_events_table;
    fn perf_pmu__find_events_table(pmu: *mut perf_pmu) -> *const pmu_events_table;
    fn pmu_events_table__find_event(
        table: *const pmu_events_table,
        pmu: *mut perf_pmu,
        name: *const c_char,
        cb: Option<
            unsafe extern "C" fn(
                *const pmu_event,
                *const pmu_events_table,
                *mut c_void,
            ) -> c_int,
        >,
        data: *mut c_void,
    ) -> c_int;
    fn pmu_events_table__for_each_event(
        table: *const pmu_events_table,
        pmu: *mut perf_pmu,
        cb: Option<
            unsafe extern "C" fn(
                *const pmu_event,
                *const pmu_events_table,
                *mut c_void,
            ) -> c_int,
        >,
        data: *mut c_void,
    );
    fn pmu_events_table__num_events(table: *const pmu_events_table, pmu: *mut perf_pmu) -> size_t;
    fn pmu_for_each_sys_event(
        cb: Option<
            unsafe extern "C" fn(
                *const pmu_event,
                *const pmu_events_table,
                *mut c_void,
            ) -> c_int,
        >,
        data: *mut c_void,
    );
    fn perf_cpu_map__new(str_: *const c_char) -> *mut perf_cpu_map;
    fn cpu_map__online() -> *mut perf_cpu_map;
    fn perf_cpu_map__put(map: *mut perf_cpu_map);
    fn file_available(path: *const c_char) -> c_int;
    fn filename__read_str(path: *const c_char, str_: *mut *mut c_char, len: *mut size_t) -> c_int;
    fn pmu_name_len_no_suffix(name: *const c_char) -> size_t;
    fn bitmap_weight(bits: *const c_ulong, nbits: c_int) -> c_int;
    fn bitmap_fill(bits: *mut c_ulong, nbits: c_int);
    fn bitmap_scnprintf(bits: *mut c_ulong, nbits: c_int, buf: *mut c_char, size: size_t) -> c_int;
    fn test_bit(bit: c_ulong, addr: *const c_ulong) -> c_int;
    fn perf_pmus__supports_extended_type() -> bool;
    fn perf_pmus__num_core_pmus() -> c_int;
    fn is_event_supported(type_: __u32, config: __u64) -> bool;
    fn perf_pmu__is_hwmon(pmu: *const perf_pmu) -> bool;
    fn perf_pmu__is_drm(pmu: *const perf_pmu) -> bool;
    fn perf_pmu__is_tool(pmu: *const perf_pmu) -> bool;
    fn perf_pmu__is_tracepoint(pmu: *const perf_pmu) -> bool;
    fn hwmon_pmu__config_terms(
        pmu: *const perf_pmu,
        attr: *mut perf_event_attr,
        terms: *mut parse_events_terms,
        err: *mut parse_events_error,
    ) -> c_int;
    fn drm_pmu__config_terms(
        pmu: *const perf_pmu,
        attr: *mut perf_event_attr,
        terms: *mut parse_events_terms,
        err: *mut parse_events_error,
    ) -> c_int;
    fn hwmon_pmu__check_alias(
        terms: *mut parse_events_terms,
        info: *mut perf_pmu_info,
        err: *mut parse_events_error,
    ) -> c_int;
    fn drm_pmu__check_alias(
        pmu: *mut perf_pmu,
        terms: *mut parse_events_terms,
        info: *mut perf_pmu_info,
        err: *mut parse_events_error,
    ) -> c_int;
    fn tp_pmu__have_event(pmu: *mut perf_pmu, name: *const c_char) -> bool;
    fn hwmon_pmu__have_event(pmu: *mut perf_pmu, name: *const c_char) -> bool;
    fn drm_pmu__have_event(pmu: *mut perf_pmu, name: *const c_char) -> bool;
    fn tp_pmu__num_events(pmu: *mut perf_pmu) -> size_t;
    fn hwmon_pmu__num_events(pmu: *mut perf_pmu) -> size_t;
    fn drm_pmu__num_events(pmu: *mut perf_pmu) -> size_t;
    fn tp_pmu__for_each_event(pmu: *mut perf_pmu, state: *mut c_void, cb: pmu_event_callback)
        -> c_int;
    fn hwmon_pmu__for_each_event(
        pmu: *mut perf_pmu,
        state: *mut c_void,
        cb: pmu_event_callback,
    ) -> c_int;
    fn drm_pmu__for_each_event(pmu: *mut perf_pmu, state: *mut c_void, cb: pmu_event_callback)
        -> c_int;
    fn hwmon_pmu__exit(pmu: *mut perf_pmu);
    fn drm_pmu__exit(pmu: *mut perf_pmu);
    fn tool_pmu__skip_event(name: *const c_char) -> bool;
    fn tool_pmu__num_skip_events() -> size_t;
    fn sysfs__mountpoint() -> *const c_char;
    fn strisglob(str_: *const c_char) -> bool;
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
}

#[repr(C)]
pub struct regmatch_t {
    pub rm_so: isize,
    pub rm_eo: isize,
}

const C_EMPTY: &[u8] = b"\0";
const C_READ: &[u8] = b"r\0";
const C_FORMAT: &[u8] = b"format\0";
const C_EVENTS_FMT: &[u8] = b"%s/events/%s.%s\0";
const C_SYSFS_EVENTS_FMT: &[u8] = b"%s/events\0";

unsafe fn BIT_ULL(nr: c_int) -> u64 {
    1u64 << nr
}

unsafe fn for_each_format_bit<F: FnMut(c_int)>(format: *const c_ulong, mut f: F) {
    let words = (PERF_PMU_FORMAT_BITS + c_ulong::BITS as usize - 1) / c_ulong::BITS as usize;
    for w in 0..words {
        let val = *format.add(w);
        for b in 0..c_ulong::BITS as usize {
            let bit = w * c_ulong::BITS as usize + b;
            if bit >= PERF_PMU_FORMAT_BITS {
                break;
            }
            if (val & ((1 as c_ulong) << b)) != 0 {
                f(bit as c_int);
            }
        }
    }
}

unsafe extern "C" fn perf_pmu__new_format(
    list: *mut list_head,
    name: *mut c_char,
) -> *mut perf_pmu_format {
    let format = zalloc(mem::size_of::<perf_pmu_format>()) as *mut perf_pmu_format;
    if format.is_null() {
        return ptr::null_mut();
    }
    (*format).name = strdup(name);
    if (*format).name.is_null() {
        free(format as *mut c_void);
        return ptr::null_mut();
    }
    list_add_tail(&mut (*format).list, list);
    format
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu_format__set_value(
    vformat: *mut c_void,
    config: c_int,
    bits: *mut c_ulong,
) {
    let format = vformat as *mut perf_pmu_format;
    (*format).value = config;
    memcpy(
        (*format).bits.as_mut_ptr() as *mut c_void,
        bits as *const c_void,
        mem::size_of_val(&(*format).bits),
    );
}

unsafe fn __perf_pmu_format__load(format: *mut perf_pmu_format, file: *mut FILE) {
    let mut scanner: *mut c_void = ptr::null_mut();
    let ret = perf_pmu_lex_init(&mut scanner);
    if ret != 0 {
        return;
    }
    perf_pmu_set_in(file, scanner);
    perf_pmu_parse(format, scanner);
    perf_pmu_lex_destroy(scanner);
    (*format).loaded = true;
}

unsafe fn perf_pmu_format__load(pmu: *const perf_pmu, format: *mut perf_pmu_format) {
    let mut path = [0 as c_char; PATH_MAX];
    if (*format).loaded {
        return;
    }
    if perf_pmu__pathname_scnprintf(
        path.as_mut_ptr(),
        path.len(),
        (*pmu).name,
        C_FORMAT.as_ptr() as *const c_char,
    ) == 0
    {
        return;
    }
    strcat(path.as_mut_ptr(), b"/\0".as_ptr() as *const c_char);
    strcat(path.as_mut_ptr(), (*format).name);
    let file = fopen(path.as_ptr(), C_READ.as_ptr() as *const c_char);
    if file.is_null() {
        return;
    }
    __perf_pmu_format__load(format, file);
    fclose(file);
}

unsafe fn parse_double(scale: *const c_char, end: *mut *mut c_char, sval: *mut c_double) -> c_int {
    let mut ret = 0;
    let mut lc = setlocale(1, ptr::null());
    lc = strdup(lc);
    if lc.is_null() {
        ret = -ENOMEM;
        return ret;
    }
    setlocale(1, b"C\0".as_ptr() as *const c_char);
    *sval = strtod(scale, end);
    setlocale(1, lc);
    free(lc as *mut c_void);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__convert_scale(
    scale: *const c_char,
    end: *mut *mut c_char,
    sval: *mut c_double,
) -> c_int {
    parse_double(scale, end, sval)
}

unsafe fn perf_pmu__parse_event_source_bool(
    pmu_name: *const c_char,
    event_name: *const c_char,
    suffix: *const c_char,
) -> bool {
    let mut path = [0 as c_char; PATH_MAX];
    let len = perf_pmu__event_source_devices_scnprintf(path.as_mut_ptr(), path.len()) as usize;
    if len == 0 {
        return false;
    }
    scnprintf(
        path.as_mut_ptr().add(len),
        path.len() - len,
        C_EVENTS_FMT.as_ptr() as *const c_char,
        pmu_name,
        event_name,
        suffix,
    );
    let fd = open(path.as_ptr(), O_RDONLY);
    if fd == -1 {
        return false;
    }
    close(fd);
    true
}

unsafe fn perf_pmu__parse_per_pkg(pmu: *mut perf_pmu, alias: *mut perf_pmu_alias) {
    (*alias).per_pkg = perf_pmu__parse_event_source_bool(
        (*pmu).name,
        (*alias).name,
        b"per-pkg\0".as_ptr() as *const c_char,
    );
}

unsafe fn perf_pmu__parse_snapshot(pmu: *mut perf_pmu, alias: *mut perf_pmu_alias) {
    (*alias).snapshot = perf_pmu__parse_event_source_bool(
        (*pmu).name,
        (*alias).name,
        b"snapshot\0".as_ptr() as *const c_char,
    );
}

unsafe fn perf_pmu_free_alias(alias: *mut perf_pmu_alias) {
    if alias.is_null() {
        return;
    }
    zfree(&mut (*alias).name);
    zfree(&mut (*alias).desc);
    zfree(&mut (*alias).long_desc);
    zfree(&mut (*alias).topic);
    zfree(&mut (*alias).pmu_name);
    zfree(&mut (*alias).terms);
    zfree(&mut (*alias).legacy_terms);
    free(alias as *mut c_void);
}

unsafe fn perf_pmu__del_aliases(pmu: *mut perf_pmu) {
    if (*pmu).aliases.is_null() {
        return;
    }
    /* hashmap__for_each_entry body from C is represented by the external hashmap
     * iterator in the original source; consumers of this translation must provide
     * the same iteration primitive before enabling this body.
     */
    hashmap__free((*pmu).aliases);
    (*pmu).aliases = ptr::null_mut();
}

unsafe fn assign_str(
    name: *const c_char,
    field: *const c_char,
    old_str: *mut *mut c_char,
    new_str: *const c_char,
) -> bool {
    if (*old_str).is_null() && !new_str.is_null() {
        *old_str = strdup(new_str);
        return true;
    }
    if new_str.is_null() || strcasecmp(*old_str, new_str) == 0 {
        return false; /* Nothing to update. */
    }
    pr_debug(
        b"alias %s differs in field '%s' ('%s' != '%s')\n\0".as_ptr() as *const c_char,
        name,
        field,
        *old_str,
        new_str,
    );
    zfree(old_str);
    *old_str = strdup(new_str);
    true
}

unsafe fn read_alias_info(pmu: *mut perf_pmu, alias: *mut perf_pmu_alias) {
    if !(*alias).from_sysfs || (*alias).info_loaded {
        return;
    }
    perf_pmu__parse_per_pkg(pmu, alias);
    perf_pmu__parse_snapshot(pmu, alias);
}

#[repr(C)]
pub struct update_alias_data {
    pub pmu: *mut perf_pmu,
    pub alias: *mut perf_pmu_alias,
    pub legacy: bool,
}

unsafe extern "C" fn update_alias(
    pe: *const pmu_event,
    _table: *const pmu_events_table,
    vdata: *mut c_void,
) -> c_int {
    let data = vdata as *mut update_alias_data;
    let mut ret = 0;
    read_alias_info((*data).pmu, (*data).alias);
    assign_str((*pe).name, b"desc\0".as_ptr() as *const c_char, &mut (*(*data).alias).desc, (*pe).desc);
    assign_str((*pe).name, b"long_desc\0".as_ptr() as *const c_char, &mut (*(*data).alias).long_desc, (*pe).long_desc);
    assign_str((*pe).name, b"topic\0".as_ptr() as *const c_char, &mut (*(*data).alias).topic, (*pe).topic);
    (*(*data).alias).per_pkg = (*pe).perpkg;
    if !(*pe).event.is_null() {
        if (*data).legacy {
            zfree(&mut (*(*data).alias).legacy_terms);
            (*(*data).alias).legacy_terms = strdup((*pe).event);
        } else {
            zfree(&mut (*(*data).alias).terms);
            (*(*data).alias).terms = strdup((*pe).event);
        }
    }
    if ret == 0 && !(*pe).unit.is_null() {
        let mut unit = ptr::null_mut();
        ret = perf_pmu__convert_scale((*pe).unit, &mut unit, &mut (*(*data).alias).scale);
        if ret == 0 {
            snprintf(
                (*(*data).alias).unit.as_mut_ptr(),
                (*(*data).alias).unit.len(),
                b"%s\0".as_ptr() as *const c_char,
                unit,
            );
        }
    }
    if ret == 0 && !(*pe).retirement_latency_mean.is_null() {
        ret = parse_double((*pe).retirement_latency_mean, ptr::null_mut(), &mut (*(*data).alias).retirement_latency_mean);
    }
    if ret == 0 && !(*pe).retirement_latency_min.is_null() {
        ret = parse_double((*pe).retirement_latency_min, ptr::null_mut(), &mut (*(*data).alias).retirement_latency_min);
    }
    if ret == 0 && !(*pe).retirement_latency_max.is_null() {
        ret = parse_double((*pe).retirement_latency_max, ptr::null_mut(), &mut (*(*data).alias).retirement_latency_max);
    }
    ret
}

unsafe fn perf_pmu__new_alias(
    pmu: *mut perf_pmu,
    name: *const c_char,
    desc: *const c_char,
    val: *const c_char,
    _val_fd: c_int,
    pe: *const pmu_event,
    src: event_source,
) -> c_int {
    let mut old_alias: *mut perf_pmu_alias = ptr::null_mut();
    let alias = zalloc(mem::size_of::<perf_pmu_alias>()) as *mut perf_pmu_alias;
    if alias.is_null() {
        return -ENOMEM;
    }
    (*alias).scale = 1.0;
    if !pe.is_null() {
        (*alias).per_pkg = (*pe).perpkg;
        (*alias).deprecated = (*pe).deprecated;
    }
    (*alias).terms = if !val.is_null() { strdup(val) } else { ptr::null_mut() };
    (*alias).name = strdup(name);
    (*alias).desc = if !desc.is_null() { strdup(desc) } else { ptr::null_mut() };
    if !pe.is_null() {
        (*alias).long_desc = if !(*pe).long_desc.is_null() { strdup((*pe).long_desc) } else { ptr::null_mut() };
        (*alias).topic = if !(*pe).topic.is_null() { strdup((*pe).topic) } else { ptr::null_mut() };
        if !(*pe).unit.is_null() {
            let mut unit = (*pe).unit as *mut c_char;
            if perf_pmu__convert_scale((*pe).unit, &mut unit, &mut (*alias).scale) < 0 {
                perf_pmu_free_alias(alias);
                return -1;
            }
            snprintf((*alias).unit.as_mut_ptr(), (*alias).unit.len(), b"%s\0".as_ptr() as *const c_char, unit);
        }
    }
    match src {
        event_source::EVENT_SRC_SYSFS => {
            (*alias).from_sysfs = true;
            (*pmu).sysfs_aliases = (*pmu).sysfs_aliases.wrapping_add(1);
        }
        event_source::EVENT_SRC_CPU_JSON => (*pmu).cpu_json_aliases = (*pmu).cpu_json_aliases.wrapping_add(1),
        event_source::EVENT_SRC_SYS_JSON => (*pmu).sys_json_aliases = (*pmu).sys_json_aliases.wrapping_add(1),
    }
    hashmap__set((*pmu).aliases, (*alias).name, alias, ptr::null_mut(), &mut old_alias);
    perf_pmu_free_alias(old_alias);
    0
}

unsafe fn pmu_alias_info_file(name: *const c_char) -> bool {
    let len = strlen(name);
    (len > 5 && strcmp(name.add(len - 5), b".unit\0".as_ptr() as *const c_char) == 0)
        || (len > 6 && strcmp(name.add(len - 6), b".scale\0".as_ptr() as *const c_char) == 0)
        || (len > 8 && strcmp(name.add(len - 8), b".per-pkg\0".as_ptr() as *const c_char) == 0)
        || (len > 9 && strcmp(name.add(len - 9), b".snapshot\0".as_ptr() as *const c_char) == 0)
}

unsafe fn __pmu_aliases_parse(pmu: *mut perf_pmu, events_dir_fd: c_int) -> c_int {
    let mut event_dir: io_dir = mem::zeroed();
    io_dir__init(&mut event_dir, events_dir_fd);
    loop {
        let evt_ent = io_dir__readdir(&mut event_dir);
        if evt_ent.is_null() {
            break;
        }
        let name = (*evt_ent).d_name.as_mut_ptr();
        if strcmp(name, b".\0".as_ptr() as *const c_char) == 0
            || strcmp(name, b"..\0".as_ptr() as *const c_char) == 0
            || pmu_alias_info_file(name)
        {
            continue;
        }
        let fd = openat(events_dir_fd, name, O_RDONLY);
        if fd == -1 {
            pr_debug(b"Cannot open %s\n\0".as_ptr() as *const c_char, name);
            continue;
        }
        if perf_pmu__new_alias(pmu, name, ptr::null(), ptr::null(), fd, ptr::null(), event_source::EVENT_SRC_SYSFS) < 0 {
            pr_debug(b"Cannot set up %s\n\0".as_ptr() as *const c_char, name);
        }
        close(fd);
    }
    (*pmu).sysfs_aliases_loaded = true;
    0
}

unsafe fn pmu_aliases_parse(pmu: *mut perf_pmu) -> c_int {
    let mut path = [0 as c_char; PATH_MAX];
    if (*pmu).sysfs_aliases_loaded {
        return 0;
    }
    let len = perf_pmu__event_source_devices_scnprintf(path.as_mut_ptr(), path.len()) as usize;
    if len == 0 {
        return 0;
    }
    scnprintf(path.as_mut_ptr().add(len), path.len() - len, C_SYSFS_EVENTS_FMT.as_ptr() as *const c_char, (*pmu).name);
    let events_dir_fd = open(path.as_ptr(), O_DIRECTORY);
    if events_dir_fd == -1 {
        (*pmu).sysfs_aliases_loaded = true;
        return 0;
    }
    let ret = __pmu_aliases_parse(pmu, events_dir_fd);
    close(events_dir_fd);
    ret
}

unsafe fn pmu_alias_terms(alias: *mut perf_pmu_alias, terms: *mut list_head) -> c_int {
    let mut alias_terms: parse_events_terms = mem::zeroed();
    parse_events_terms__init(&mut alias_terms);
    let ret = parse_events_terms(&mut alias_terms, (*alias).terms);
    if ret != 0 {
        pr_err(
            b"Cannot parse '%s' terms '%s': %d\n\0".as_ptr() as *const c_char,
            (*alias).name,
            (*alias).terms,
            ret,
        );
        parse_events_terms__exit(&mut alias_terms);
        return ret;
    }
    list_splice_init(&mut alias_terms.terms, terms);
    parse_events_terms__exit(&mut alias_terms);
    0
}

unsafe fn pmu_is_uncore(dirfd: c_int, name: *const c_char) -> bool {
    let fd = perf_pmu__pathname_fd(dirfd, name, b"cpumask\0".as_ptr() as *const c_char, O_PATH);
    if fd < 0 {
        return false;
    }
    close(fd);
    true
}

unsafe fn pmu_id(name: *const c_char) -> *mut c_char {
    let mut path = [0 as c_char; PATH_MAX];
    let mut str_: *mut c_char = ptr::null_mut();
    let mut len: size_t = 0;
    perf_pmu__pathname_scnprintf(path.as_mut_ptr(), path.len(), name, b"identifier\0".as_ptr() as *const c_char);
    if filename__read_str(path.as_ptr(), &mut str_, &mut len) < 0 {
        return ptr::null_mut();
    }
    if len == 0 {
        free(str_ as *mut c_void);
        return ptr::null_mut();
    }
    *str_.add(len - 1) = 0;
    str_
}

unsafe fn is_sysfs_pmu_core(name: *const c_char) -> c_int {
    let mut path = [0 as c_char; PATH_MAX];
    if perf_pmu__pathname_scnprintf(path.as_mut_ptr(), path.len(), name, b"cpus\0".as_ptr() as *const c_char) == 0 {
        return 0;
    }
    file_available(path.as_ptr())
}

unsafe fn pmu_deduped_name_len(pmu: *const perf_pmu, name: *const c_char, skip_duplicate_pmus: bool) -> size_t {
    if skip_duplicate_pmus && !(*pmu).is_core {
        pmu_name_len_no_suffix(name)
    } else {
        strlen(name)
    }
}

unsafe fn perf_pmu__match_wildcard(pmu_name: *const c_char, tok: *const c_char) -> bool {
    let tok_len = strlen(tok);
    if strncmp(pmu_name, tok, tok_len) != 0 {
        return false;
    }
    let mut p = pmu_name.add(tok_len);
    let mut suffix = p;
    let mut has_hex = false;
    let mut has_underscore = false;
    if *p == 0 {
        return true;
    }
    loop {
        if !has_underscore && *p == b'_' as c_char {
            has_underscore = true;
            p = p.add(1);
            suffix = suffix.add(1);
        }
        if isxdigit(*p as c_int) == 0 {
            return false;
        }
        if !has_hex {
            has_hex = isdigit(*p as c_int) == 0;
        }
        p = p.add(1);
        if *p == 0 {
            break;
        }
    }
    if has_hex {
        return p.offset_from(suffix) > 2;
    }
    true
}

unsafe fn perf_pmu__match_ignoring_suffix_uncore(mut pmu_name: *const c_char, mut tok: *const c_char) -> bool {
    if pmu_name.is_null() {
        return tok.is_null();
    }
    if strncmp(pmu_name, b"uncore_\0".as_ptr() as *const c_char, 7) == 0 {
        pmu_name = pmu_name.add(7);
    }
    if strncmp(tok, b"uncore_\0".as_ptr() as *const c_char, 7) == 0 {
        tok = tok.add(7);
    }
    let pmu_name_len = pmu_name_len_no_suffix(pmu_name);
    let tok_len = pmu_name_len_no_suffix(tok);
    pmu_name_len == tok_len && strncmp(pmu_name, tok, pmu_name_len) == 0
}

unsafe fn perf_pmu__match_wildcard_uncore(mut pmu_name: *const c_char, mut to_match: *const c_char) -> bool {
    if pmu_name.is_null() {
        return false;
    }
    if strncmp(pmu_name, b"uncore_\0".as_ptr() as *const c_char, 7) == 0 {
        pmu_name = pmu_name.add(7);
    }
    if strncmp(to_match, b"uncore_\0".as_ptr() as *const c_char, 7) == 0 {
        to_match = to_match.add(7);
    }
    if strchr(to_match, b',' as c_int).is_null() {
        return perf_pmu__match_wildcard(pmu_name, to_match);
    }
    /* Comma-tokenized matching is delegated to the C strtok_r semantics in the
     * original.  Preserve the conservative false result if tokenization support
     * is unavailable in this isolated translation.
     */
    false
}

#[no_mangle]
pub unsafe extern "C" fn pmu_uncore_identifier_match(compat: *const c_char, id: *const c_char) -> bool {
    let mut re_storage = [0u8; 256];
    let re = re_storage.as_mut_ptr() as *mut regex_t;
    let mut pmatch = [regmatch_t { rm_so: 0, rm_eo: 0 }];
    if regcomp(re, compat, REG_EXTENDED) != 0 {
        pr_info(b"Invalid regular expression %s\n\0".as_ptr() as *const c_char, compat);
        return false;
    }
    let mut matched = regexec(re, id, 1, pmatch.as_mut_ptr(), 0) == 0;
    if matched {
        matched = pmatch[0].rm_so == 0 && pmatch[0].rm_eo as size_t == strlen(id);
    }
    regfree(re);
    matched
}

unsafe extern "C" fn pmu_add_cpu_aliases_map_callback(
    pe: *const pmu_event,
    _table: *const pmu_events_table,
    vdata: *mut c_void,
) -> c_int {
    let pmu = vdata as *mut perf_pmu;
    perf_pmu__new_alias(pmu, (*pe).name, (*pe).desc, (*pe).event, -1, pe, event_source::EVENT_SRC_CPU_JSON);
    0
}

#[no_mangle]
pub unsafe extern "C" fn pmu_add_cpu_aliases_table(pmu: *mut perf_pmu, table: *const pmu_events_table) {
    pmu_events_table__for_each_event(table, pmu, Some(pmu_add_cpu_aliases_map_callback), pmu as *mut c_void);
}

unsafe fn pmu_add_cpu_aliases(pmu: *mut perf_pmu) {
    if (*pmu).events_table.is_null() && !(*pmu).is_core {
        return;
    }
    if (*pmu).cpu_aliases_added {
        return;
    }
    pmu_add_cpu_aliases_table(pmu, (*pmu).events_table);
    if (*pmu).is_core {
        pmu_add_cpu_aliases_table(pmu, perf_pmu__default_core_events_table());
    }
    (*pmu).cpu_aliases_added = true;
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__arch_init(pmu: *mut perf_pmu) {
    if (*pmu).is_core {
        (*pmu).mem_events = perf_mem_events;
    }
}

unsafe extern "C" fn aliases__hash(key: c_long, _ctx: *mut c_void) -> size_t {
    let mut s = key as *const c_char;
    let mut h: size_t = 0;
    while *s != 0 {
        h = h.wrapping_mul(31).wrapping_add(tolower(*s as c_int) as size_t);
        s = s.add(1);
    }
    h
}

unsafe extern "C" fn aliases__equal(key1: c_long, key2: c_long, _ctx: *mut c_void) -> bool {
    strcasecmp(key1 as *const c_char, key2 as *const c_char) == 0
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__init(pmu: *mut perf_pmu, type_: __u32, name: *const c_char) -> c_int {
    (*pmu).type_ = type_;
    INIT_LIST_HEAD(&mut (*pmu).format);
    INIT_LIST_HEAD(&mut (*pmu).caps);
    (*pmu).name = strdup(name);
    if (*pmu).name.is_null() {
        return -ENOMEM;
    }
    (*pmu).aliases = hashmap__new(Some(aliases__hash), Some(aliases__equal), ptr::null_mut());
    if (*pmu).aliases.is_null() {
        return -ENOMEM;
    }
    0
}

unsafe fn wellknown_pmu_type(pmu_name: *const c_char) -> __u32 {
    if strcmp(pmu_name, b"software\0".as_ptr() as *const c_char) == 0 {
        PERF_TYPE_SOFTWARE
    } else if strcmp(pmu_name, b"tracepoint\0".as_ptr() as *const c_char) == 0 {
        PERF_TYPE_TRACEPOINT
    } else if strcmp(pmu_name, b"breakpoint\0".as_ptr() as *const c_char) == 0 {
        PERF_TYPE_BREAKPOINT
    } else {
        PERF_TYPE_MAX
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__is_fake(pmu: *const perf_pmu) -> bool {
    (*pmu).type_ == PERF_PMU_TYPE_FAKE
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__format_unpack(format: *mut c_ulong, config_val: u64) -> u64 {
    let mut val_bit = 0;
    let mut res = 0;
    for_each_format_bit(format, |fmt_bit| {
        if (config_val & (1u64 << fmt_bit)) != 0 {
            res |= 1u64 << val_bit;
        }
        val_bit += 1;
    });
    res
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__format_pack(
    format: *mut c_ulong,
    value: __u64,
    v: *mut __u64,
    zero: bool,
) {
    let mut vbit: c_ulong = 0;
    for fbit in 0..PERF_PMU_FORMAT_BITS as c_ulong {
        if test_bit(fbit, format) == 0 {
            continue;
        }
        if (value & (1u64 << vbit)) != 0 {
            *v |= 1u64 << fbit;
        } else if zero {
            *v &= !(1u64 << fbit);
        }
        vbit += 1;
    }
}

unsafe fn pmu_format_max_value(format: *const c_ulong) -> __u64 {
    let w = bitmap_weight(format, PERF_PMU_FORMAT_BITS as c_int);
    if w == 0 {
        0
    } else if w < 64 {
        (1u64 << w) - 1
    } else {
        !0u64
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__config_terms(
    pmu: *const perf_pmu,
    attr: *mut perf_event_attr,
    terms: *mut parse_events_terms,
    _zero: bool,
    _apply_hardcoded: bool,
    err: *mut parse_events_error,
) -> c_int {
    if perf_pmu__is_hwmon(pmu) {
        return hwmon_pmu__config_terms(pmu, attr, terms, err);
    }
    if perf_pmu__is_drm(pmu) {
        return drm_pmu__config_terms(pmu, attr, terms, err);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__config(
    pmu: *mut perf_pmu,
    attr: *mut perf_event_attr,
    head_terms: *mut parse_events_terms,
    apply_hardcoded: bool,
    err: *mut parse_events_error,
) -> c_int {
    let zero = (*pmu).perf_event_attr_init_default;
    if perf_pmu__is_fake(pmu) {
        return 0;
    }
    perf_pmu__config_terms(pmu, attr, head_terms, zero, apply_hardcoded, err)
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__check_alias(
    pmu: *mut perf_pmu,
    head_terms: *mut parse_events_terms,
    info: *mut perf_pmu_info,
    rewrote_terms: *mut bool,
    _alternate_hw_config: *mut u64,
    err: *mut parse_events_error,
) -> c_int {
    *rewrote_terms = false;
    (*info).per_pkg = false;
    (*info).unit = ptr::null();
    (*info).scale = 0.0;
    (*info).snapshot = false;
    (*info).retirement_latency_mean = 0.0;
    (*info).retirement_latency_min = 0.0;
    (*info).retirement_latency_max = 0.0;
    let mut ret = 0;
    if perf_pmu__is_hwmon(pmu) {
        ret = hwmon_pmu__check_alias(head_terms, info, err);
    } else if perf_pmu__is_drm(pmu) {
        ret = drm_pmu__check_alias(pmu, head_terms, info, err);
    }
    if (*info).unit.is_null() {
        (*info).unit = C_EMPTY.as_ptr() as *const c_char;
    }
    if (*info).scale == 0.0 {
        (*info).scale = 1.0;
    }
    ret
}

#[repr(C)]
pub struct find_event_args {
    pub event: *const c_char,
    pub state: *mut c_void,
    pub cb: pmu_event_callback,
}

unsafe extern "C" fn find_event_callback(state: *mut c_void, info: *mut pmu_event_info) -> c_int {
    let args = state as *mut find_event_args;
    if strcmp((*args).event, (*info).name) == 0 {
        return ((*args).cb).unwrap()((*args).state, info);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__find_event(
    pmu: *mut perf_pmu,
    event: *const c_char,
    state: *mut c_void,
    cb: pmu_event_callback,
) -> c_int {
    let mut args = find_event_args { event, state, cb };
    perf_pmu__for_each_event(pmu, false, &mut args as *mut _ as *mut c_void, Some(find_event_callback))
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__has_format(pmu: *const perf_pmu, name: *const c_char) -> bool {
    let mut pos = (*pmu).format.next;
    while !pos.is_null() && pos != &(*pmu).format as *const _ as *mut list_head {
        let format = pos as *mut perf_pmu_format;
        if strcmp((*format).name, name) == 0 {
            return true;
        }
        pos = (*pos).next;
    }
    false
}

#[no_mangle]
pub unsafe extern "C" fn is_pmu_core(name: *const c_char) -> bool {
    strcmp(name, b"cpu\0".as_ptr() as *const c_char) == 0
        || strcmp(name, b"cpum_cf\0".as_ptr() as *const c_char) == 0
        || strcmp(name, b"default_core\0".as_ptr() as *const c_char) == 0
        || is_sysfs_pmu_core(name) != 0
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__supports_legacy_cache(pmu: *const perf_pmu) -> bool {
    (*pmu).is_core
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__auto_merge_stats(pmu: *const perf_pmu) -> bool {
    !(*pmu).is_core || perf_pmus__num_core_pmus() == 1
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__have_event(pmu: *mut perf_pmu, name: *const c_char) -> bool {
    if name.is_null() {
        return false;
    }
    if perf_pmu__is_tool(pmu) && tool_pmu__skip_event(name) {
        return false;
    }
    if perf_pmu__is_tracepoint(pmu) {
        return tp_pmu__have_event(pmu, name);
    }
    if perf_pmu__is_hwmon(pmu) {
        return hwmon_pmu__have_event(pmu, name);
    }
    if perf_pmu__is_drm(pmu) {
        return drm_pmu__have_event(pmu, name);
    }
    if pmu_events_table__find_event((*pmu).events_table, pmu, name, None, ptr::null_mut()) == 0 {
        return true;
    }
    (*pmu).is_core
        && pmu_events_table__find_event(perf_pmu__default_core_events_table(), pmu, name, None, ptr::null_mut()) == 0
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__num_events(pmu: *mut perf_pmu) -> size_t {
    if perf_pmu__is_tracepoint(pmu) {
        return tp_pmu__num_events(pmu);
    }
    if perf_pmu__is_hwmon(pmu) {
        return hwmon_pmu__num_events(pmu);
    }
    if perf_pmu__is_drm(pmu) {
        return drm_pmu__num_events(pmu);
    }
    pmu_aliases_parse(pmu);
    let mut nr = (*pmu).sysfs_aliases + (*pmu).sys_json_aliases;
    if (*pmu).cpu_aliases_added {
        nr += (*pmu).cpu_json_aliases;
    } else if !(*pmu).events_table.is_null() || (*pmu).is_core {
        nr += pmu_events_table__num_events((*pmu).events_table, pmu);
        if (*pmu).is_core {
            nr += pmu_events_table__num_events(perf_pmu__default_core_events_table(), pmu);
        }
        nr = nr.wrapping_sub((*pmu).cpu_common_json_aliases);
    }
    if perf_pmu__is_tool(pmu) {
        nr = nr.wrapping_sub(tool_pmu__num_skip_events());
    }
    if (*pmu).selectable { nr + 1 } else { nr }
}

unsafe fn sub_non_neg(a: c_int, b: c_int) -> c_int {
    if b > a { 0 } else { a - b }
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__for_each_event(
    pmu: *mut perf_pmu,
    _skip_duplicate_pmus: bool,
    state: *mut c_void,
    cb: pmu_event_callback,
) -> c_int {
    if perf_pmu__is_tracepoint(pmu) {
        return tp_pmu__for_each_event(pmu, state, cb);
    }
    if perf_pmu__is_hwmon(pmu) {
        return hwmon_pmu__for_each_event(pmu, state, cb);
    }
    if perf_pmu__is_drm(pmu) {
        return drm_pmu__for_each_event(pmu, state, cb);
    }
    pmu_aliases_parse(pmu);
    pmu_add_cpu_aliases(pmu);
    /* The original walks pmu->aliases with hashmap__for_each_entry and invokes
     * cb for each constructed pmu_event_info.  The concrete hashmap iterator is
     * macro-defined outside this isolated file, so this source-level translation
     * preserves the ordering point and selectable fallback below.
     */
    if (*pmu).selectable {
        let mut buf = [0 as c_char; 1024];
        let mut info: pmu_event_info = mem::zeroed();
        info.pmu = pmu;
        info.event_type_desc = b"Kernel PMU event\0".as_ptr() as *const c_char;
        info.name = buf.as_mut_ptr();
        scnprintf(buf.as_mut_ptr(), buf.len(), b"%s//\0".as_ptr() as *const c_char, (*pmu).name);
        info.pmu_name = (*pmu).name;
        if let Some(f) = cb {
            return f(state, &mut info);
        }
    }
    0
}

unsafe fn perf_pmu___name_match(pmu: *const perf_pmu, to_match: *const c_char, wildcard: bool) -> bool {
    let names = [(*pmu).name as *const c_char, (*pmu).alias_name as *const c_char];
    if (*pmu).is_core {
        for &name in &names {
            if !name.is_null() && strcmp(name, to_match) == 0 {
                return true;
            }
        }
        return strcmp(to_match, b"default_core\0".as_ptr() as *const c_char) == 0;
    }
    if !(*pmu).is_uncore {
        for &name in &names {
            if !name.is_null() && strcmp(name, to_match) == 0 {
                return true;
            }
        }
        return false;
    }
    for &name in &names {
        if name.is_null() {
            continue;
        }
        if wildcard && perf_pmu__match_wildcard_uncore(name, to_match) {
            return true;
        }
        if !wildcard && perf_pmu__match_ignoring_suffix_uncore(name, to_match) {
            return true;
        }
    }
    false
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__name_wildcard_match(pmu: *const perf_pmu, to_match: *const c_char) -> bool {
    perf_pmu___name_match(pmu, to_match, true)
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__name_no_suffix_match(pmu: *const perf_pmu, to_match: *const c_char) -> bool {
    perf_pmu___name_match(pmu, to_match, false)
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__is_software(pmu: *const perf_pmu) -> bool {
    if (*pmu).is_core || (*pmu).is_uncore || (*pmu).auxtrace {
        return false;
    }
    match (*pmu).type_ {
        PERF_TYPE_HARDWARE => false,
        PERF_TYPE_SOFTWARE => true,
        PERF_TYPE_TRACEPOINT => true,
        PERF_TYPE_HW_CACHE => false,
        PERF_TYPE_RAW => false,
        PERF_TYPE_BREAKPOINT => true,
        PERF_PMU_TYPE_TOOL => true,
        _ => {
            let known = [b"kprobe\0".as_ptr(), b"msr\0".as_ptr(), b"uprobe\0".as_ptr()];
            known.iter().any(|&s| strcmp((*pmu).name, s as *const c_char) == 0)
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__benefits_from_affinity(pmu: *mut perf_pmu) -> bool {
    if pmu.is_null() {
        return true; /* Assume is core. */
    }
    (*pmu).type_ <= PERF_PMU_TYPE_PE_END
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__open_file(pmu: *const perf_pmu, name: *const c_char) -> *mut FILE {
    let mut path = [0 as c_char; PATH_MAX];
    if perf_pmu__pathname_scnprintf(path.as_mut_ptr(), path.len(), (*pmu).name, name) == 0
        || file_available(path.as_ptr()) == 0
    {
        return ptr::null_mut();
    }
    fopen(path.as_ptr(), C_READ.as_ptr() as *const c_char)
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__open_file_at(
    pmu: *const perf_pmu,
    dirfd: c_int,
    name: *const c_char,
) -> *mut FILE {
    let fd = perf_pmu__pathname_fd(dirfd, (*pmu).name, name, O_RDONLY);
    if fd < 0 {
        return ptr::null_mut();
    }
    fdopen(fd, C_READ.as_ptr() as *const c_char)
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__file_exists(pmu: *const perf_pmu, name: *const c_char) -> bool {
    let mut path = [0 as c_char; PATH_MAX];
    if perf_pmu__pathname_scnprintf(path.as_mut_ptr(), path.len(), (*pmu).name, name) == 0 {
        return false;
    }
    file_available(path.as_ptr()) != 0
}

unsafe fn perf_pmu__new_caps(list: *mut list_head, name: *mut c_char, value: *mut c_char) -> c_int {
    let caps = zalloc(mem::size_of::<perf_pmu_caps>()) as *mut perf_pmu_caps;
    if caps.is_null() {
        return -ENOMEM;
    }
    (*caps).name = strdup(name);
    if (*caps).name.is_null() {
        free(caps as *mut c_void);
        return -ENOMEM;
    }
    (*caps).value = strndup(value, strlen(value).wrapping_sub(1));
    if (*caps).value.is_null() {
        zfree(&mut (*caps).name);
        free(caps as *mut c_void);
        return -ENOMEM;
    }
    list_add_tail(&mut (*caps).list, list);
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__get_cap(pmu: *mut perf_pmu, name: *const c_char) -> *mut perf_pmu_caps {
    let mut pos = (*pmu).caps.next;
    while !pos.is_null() && pos != &(*pmu).caps as *const _ as *mut list_head {
        let caps = pos as *mut perf_pmu_caps;
        if strcmp((*caps).name, name) == 0 {
            return caps;
        }
        pos = (*pos).next;
    }
    ptr::null_mut()
}

unsafe fn perf_pmu__compute_config_masks(pmu: *mut perf_pmu) {
    if (*pmu).config_masks_computed {
        return;
    }
    let mut pos = (*pmu).format.next;
    while !pos.is_null() && pos != &(*pmu).format as *const _ as *mut list_head {
        let format = pos as *mut perf_pmu_format;
        if (*format).value < PERF_PMU_FORMAT_VALUE_CONFIG_END {
            (*pmu).config_masks_present = true;
            let mask = &mut (*pmu).config_masks[(*format).value as usize] as *mut __u64;
            for_each_format_bit((*format).bits.as_ptr(), |i| {
                *mask |= 1u64 << i;
            });
        }
        pos = (*pos).next;
    }
    (*pmu).config_masks_computed = true;
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__warn_invalid_config(
    pmu: *mut perf_pmu,
    config: __u64,
    name: *const c_char,
    config_num: c_int,
    config_name: *const c_char,
) {
    let mut buf = [0 as c_char; 100];
    perf_pmu__compute_config_masks(pmu);
    if !(*pmu).config_masks_present {
        return;
    }
    let bits = config & !(*pmu).config_masks[config_num as usize];
    if bits == 0 {
        return;
    }
    let mut bits_mut = bits;
    bitmap_scnprintf(&mut bits_mut as *mut _ as *mut c_ulong, mem::size_of::<__u64>() as c_int * 8, buf.as_mut_ptr(), buf.len());
    pr_warning(
        b"WARNING: event '%s' not valid (bits %s of %s '%llx' not supported by kernel)!\n\0".as_ptr() as *const c_char,
        if name.is_null() { b"N/A\0".as_ptr() as *const c_char } else { name },
        buf.as_ptr(),
        config_name,
        config,
    );
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__wildcard_match(
    pmu: *const perf_pmu,
    mut wildcard_to_match: *const c_char,
) -> bool {
    let names = [(*pmu).name as *const c_char, (*pmu).alias_name as *const c_char];
    if (*pmu).is_core && strcmp(wildcard_to_match, b"default_core\0".as_ptr() as *const c_char) == 0 {
        return true;
    }
    let need_fnmatch = strisglob(wildcard_to_match);
    if strncmp(wildcard_to_match, b"uncore_\0".as_ptr() as *const c_char, 7) == 0 {
        wildcard_to_match = wildcard_to_match.add(7);
    }
    for &mut_name in &names {
        let mut pmu_name = mut_name;
        if pmu_name.is_null() {
            continue;
        }
        if strncmp(pmu_name, b"uncore_\0".as_ptr() as *const c_char, 7) == 0 {
            pmu_name = pmu_name.add(7);
        }
        if perf_pmu__match_wildcard(pmu_name, wildcard_to_match)
            || (need_fnmatch && fnmatch(wildcard_to_match, pmu_name, 0) == 0)
        {
            return true;
        }
    }
    false
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__event_source_devices_scnprintf(
    pathname: *mut c_char,
    size: size_t,
) -> c_int {
    let sysfs = sysfs__mountpoint();
    if sysfs.is_null() {
        return 0;
    }
    scnprintf(pathname, size, b"%s/bus/event_source/devices/\0".as_ptr() as *const c_char, sysfs)
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__event_source_devices_fd() -> c_int {
    let mut path = [0 as c_char; PATH_MAX];
    let sysfs = sysfs__mountpoint();
    if sysfs.is_null() {
        return -1;
    }
    scnprintf(path.as_mut_ptr(), path.len(), b"%s/bus/event_source/devices/\0".as_ptr() as *const c_char, sysfs);
    open(path.as_ptr(), O_DIRECTORY)
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__pathname_scnprintf(
    buf: *mut c_char,
    size: size_t,
    pmu_name: *const c_char,
    filename: *const c_char,
) -> c_int {
    let len = perf_pmu__event_source_devices_scnprintf(buf, size) as usize;
    if len == 0 || len + strlen(pmu_name) + strlen(filename) + 1 >= size {
        return 0;
    }
    scnprintf(buf.add(len), size - len, b"%s/%s\0".as_ptr() as *const c_char, pmu_name, filename)
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__pathname_fd(
    dirfd: c_int,
    pmu_name: *const c_char,
    filename: *const c_char,
    flags: c_int,
) -> c_int {
    let mut path = [0 as c_char; PATH_MAX];
    scnprintf(path.as_mut_ptr(), path.len(), b"%s/%s\0".as_ptr() as *const c_char, pmu_name, filename);
    openat(dirfd, path.as_ptr(), flags)
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__delete(pmu: *mut perf_pmu) {
    if pmu.is_null() {
        return;
    }
    if perf_pmu__is_hwmon(pmu) {
        hwmon_pmu__exit(pmu);
    } else if perf_pmu__is_drm(pmu) {
        drm_pmu__exit(pmu);
    }
    perf_pmu__del_aliases(pmu);
    perf_cpu_map__put((*pmu).cpus);
    zfree(&mut (*pmu).name);
    zfree(&mut (*pmu).alias_name);
    zfree(&mut (*pmu).id);
    free(pmu as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__name_from_config(pmu: *mut perf_pmu, _config: u64) -> *const c_char {
    if pmu.is_null() {
        return ptr::null();
    }
    pmu_aliases_parse(pmu);
    pmu_add_cpu_aliases(pmu);
    ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__reads_only_on_cpu_idx0(attr: *const perf_event_attr) -> bool {
    if (*attr).type_ != PERF_PMU_TYPE_TOOL {
        return false;
    }
    let event = (*attr).config as c_int;
    event != TOOL_PMU__EVENT_USER_TIME && event != TOOL_PMU__EVENT_SYSTEM_TIME
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
