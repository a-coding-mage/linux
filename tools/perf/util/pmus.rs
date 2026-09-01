// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/util/pmus.c. C includes are intentionally not
// executable Rust; the declarations below name the external dependencies used
// by this implementation.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ulonglong, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type bool_t = bool;
type u32 = u32;
type __u64 = u64;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct perf_pmu {
    pub list: list_head,
    pub name: *const c_char,
    pub alias_name: *const c_char,
    pub is_core: bool_t,
    pub is_uncore: bool_t,
    pub type_: c_uint,
    pub id: *const c_char,
    pub format: list_head,
}

#[repr(C)]
pub struct io_dir {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_dirent64 {
    pub d_name: [c_char; 256],
}

#[repr(C)]
pub struct hwmon_type {
    _private: c_int,
}

#[repr(C)]
pub struct pmu_event_info {
    pub pmu: *const perf_pmu,
    pub name: *const c_char,
    pub alias: *const c_char,
    pub scale_unit: *const c_char,
    pub desc: *const c_char,
    pub long_desc: *const c_char,
    pub encoding_desc: *const c_char,
    pub topic: *const c_char,
    pub pmu_name: *const c_char,
    pub event_type_desc: *const c_char,
    pub deprecated: bool_t,
}

#[repr(C)]
pub struct strbuf {
    pub alloc: size_t,
    pub len: size_t,
    pub buf: *mut c_char,
}

#[repr(C)]
pub struct print_callbacks {
    pub skip_duplicate_pmus: unsafe extern "C" fn(*mut c_void) -> bool_t,
    pub print_event: unsafe extern "C" fn(
        *mut c_void,
        *const c_char,
        *const c_char,
        c_uint,
        *const c_char,
        *const c_char,
        *const c_char,
        bool_t,
        *const c_char,
        *const c_char,
        *const c_char,
        *const c_char,
    ),
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub config: u64,
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    pub pmu: *mut perf_pmu,
    pub core: evsel_core,
}

unsafe extern "C" {
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulonglong;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn qsort(base: *mut c_void, nmemb: size_t, size: size_t,
             compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int);
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn pthread_once(once_control: *mut pthread_once_t, init_routine: unsafe extern "C" fn()) -> c_int;

    fn list_empty(head: *const list_head) -> c_int;
    fn list_sort(priv_: *mut c_void, head: *mut list_head,
                 cmp: unsafe extern "C" fn(*mut c_void, *const list_head, *const list_head) -> c_int);
    fn list_del(entry: *mut list_head);
    fn list_add_tail(new_: *mut list_head, head: *mut list_head);

    fn perf_pmu__delete(pmu: *mut perf_pmu);
    fn perf_pmu__event_source_devices_fd() -> c_int;
    fn perf_pmu__lookup(head: *mut list_head, dirfd: c_int, name: *const c_char,
                        eager_load: bool_t) -> *mut perf_pmu;
    fn perf_pmu__create_placeholder_core_pmu(head: *mut list_head) -> bool_t;
    fn perf_pmu__num_events(pmu: *mut perf_pmu) -> c_int;
    fn perf_pmu__for_each_event(
        pmu: *mut perf_pmu,
        skip_duplicate_pmus: bool_t,
        state: *mut c_void,
        cb: unsafe extern "C" fn(*mut c_void, *mut pmu_event_info) -> c_int,
    ) -> c_int;
    fn perf_pmu__for_each_format(
        pmu: *mut perf_pmu,
        state: *mut c_void,
        cb: unsafe extern "C" fn(*mut c_void, *const c_char, c_int, *const c_ulong) -> c_int,
    ) -> c_int;
    fn perf_pmu__wildcard_match(pmu: *mut perf_pmu, wildcard: *const c_char) -> bool_t;
    fn perf_pmu__have_event(pmu: *mut perf_pmu, name: *const c_char) -> bool_t;

    fn is_pmu_core(name: *const c_char) -> bool_t;
    fn pmu_uncore_identifier_match(compat: *const c_char, id: *const c_char) -> bool_t;
    fn parse_hwmon_filename(filename: *const c_char, type_: *mut hwmon_type, number: *mut c_int,
                            item: *mut c_void, alarm: *mut c_void) -> bool_t;
    fn perf_pmus__read_hwmon_pmus(head: *mut list_head);
    fn perf_pmus__read_drm_pmus(head: *mut list_head);
    fn hwmon_pmu__new(head: *mut list_head, hwmon_dir: *const c_char,
                      sysfs_name: *const c_char, name: *const c_char) -> *mut perf_pmu;
    fn tool_pmu__new() -> *mut perf_pmu;

    fn io_dir__init(dir: *mut io_dir, fd: c_int);
    fn io_dir__readdir(dir: *mut io_dir) -> *mut io_dirent64;

    fn strisglob(str_: *const c_char) -> bool_t;
    fn bitmap_weight(bits: *const c_ulong, nbits: c_int) -> c_uint;
    fn strbuf_addch(sb: *mut strbuf, ch: c_int) -> c_int;
    fn strbuf_addf(sb: *mut strbuf, fmt: *const c_char, ...) -> c_int;
    fn strbuf_release(sb: *mut strbuf);
    fn zfree(ptr: *mut c_void);
    fn pager_in_use() -> bool_t;
    fn is_event_supported(type_: c_uint, config: u64) -> bool_t;
    fn pr_err(fmt: *const c_char, ...);
}

type pthread_once_t = c_int;

const PERF_TOOL_PMU_TYPE_PE_CORE: c_uint = 0;
const PERF_TOOL_PMU_TYPE_PE_OTHER: c_uint = 1;
const PERF_TOOL_PMU_TYPE_TOOL: c_uint = 2;
const PERF_TOOL_PMU_TYPE_HWMON: c_uint = 3;
const PERF_TOOL_PMU_TYPE_DRM: c_uint = 4;

const PERF_TOOL_PMU_TYPE_PE_CORE_MASK: c_uint = 1 << PERF_TOOL_PMU_TYPE_PE_CORE;
const PERF_TOOL_PMU_TYPE_PE_OTHER_MASK: c_uint = 1 << PERF_TOOL_PMU_TYPE_PE_OTHER;
const PERF_TOOL_PMU_TYPE_TOOL_MASK: c_uint = 1 << PERF_TOOL_PMU_TYPE_TOOL;
const PERF_TOOL_PMU_TYPE_HWMON_MASK: c_uint = 1 << PERF_TOOL_PMU_TYPE_HWMON;
const PERF_TOOL_PMU_TYPE_DRM_MASK: c_uint = 1 << PERF_TOOL_PMU_TYPE_DRM;
const PERF_TOOL_PMU_TYPE_ALL_MASK: c_uint = PERF_TOOL_PMU_TYPE_PE_CORE_MASK
    | PERF_TOOL_PMU_TYPE_PE_OTHER_MASK
    | PERF_TOOL_PMU_TYPE_TOOL_MASK
    | PERF_TOOL_PMU_TYPE_HWMON_MASK
    | PERF_TOOL_PMU_TYPE_DRM_MASK;

const PERF_PMU_TYPE_PE_START: c_uint = 0;
const PERF_PMU_TYPE_PE_END: c_uint = 0;
const PERF_PMU_TYPE_DRM_START: c_uint = 0;
const PERF_PMU_TYPE_DRM_END: c_uint = 0;
const PERF_PMU_TYPE_HWMON_START: c_uint = 0;
const PERF_PMU_TYPE_HWMON_END: c_uint = 0;
const PERF_PMU_TYPE_FAKE: c_uint = 0;
const PERF_PMU_FORMAT_BITS: c_int = 64;
const PERF_TYPE_HARDWARE: c_uint = 0;
const PERF_TYPE_HW_CACHE: c_uint = 3;
const PERF_TYPE_RAW: c_uint = 4;
const PERF_COUNT_HW_CPU_CYCLES: u64 = 0;
const PERF_PMU_TYPE_SHIFT: c_uint = 32;
const PTHREAD_ONCE_INIT: pthread_once_t = 0;
const ULLONG_MAX: c_ulonglong = c_ulonglong::MAX;

static mut core_pmus: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
static mut other_pmus: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
static mut read_pmu_types: c_uint = 0;
static mut perf_pmus__do_support_extended_type: bool_t = false;

unsafe fn cstr_lit(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

unsafe fn isxdigit_c(ch: c_char) -> bool {
    let c = ch as u8;
    (b'0'..=b'9').contains(&c) || (b'a'..=b'f').contains(&c) || (b'A'..=b'F').contains(&c)
}

unsafe fn isdigit_c(ch: c_char) -> bool {
    let c = ch as u8;
    (b'0'..=b'9').contains(&c)
}

#[no_mangle]
pub unsafe extern "C" fn pmu_name_len_no_suffix(str_: *const c_char) -> size_t {
    let orig_len: c_int;
    let mut len: c_int;
    let mut has_hex_digits = false;

    orig_len = strlen(str_) as c_int;
    len = orig_len;

    /* Count trailing digits. */
    while len > 0 && isxdigit_c(*str_.add((len - 1) as usize)) {
        if !isdigit_c(*str_.add((len - 1) as usize)) {
            has_hex_digits = true;
        }
        len -= 1;
    }

    if len > 0 && len != orig_len && *str_.add((len - 1) as usize) == b'_' as c_char {
        /*
         * There is a '_{num}' suffix. For decimal suffixes any length
         * will do, for hexadecimal ensure more than 2 hex digits so
         * that S390's cpum_cf PMU doesn't match.
         */
        if !has_hex_digits || (orig_len - len) > 2 {
            return (len - 1) as size_t;
        }
    }
    /* Use the full length. */
    orig_len as size_t
}

#[no_mangle]
pub unsafe extern "C" fn pmu_name_cmp(lhs_pmu_name: *const c_char, rhs_pmu_name: *const c_char) -> c_int {
    let mut lhs_num: c_ulonglong = 0;
    let mut rhs_num: c_ulonglong = 0;
    let lhs_pmu_name_len = pmu_name_len_no_suffix(lhs_pmu_name);
    let rhs_pmu_name_len = pmu_name_len_no_suffix(rhs_pmu_name);
    let n = if lhs_pmu_name_len < rhs_pmu_name_len { lhs_pmu_name_len } else { rhs_pmu_name_len };
    let ret = strncmp(lhs_pmu_name, rhs_pmu_name, n);

    if lhs_pmu_name_len != rhs_pmu_name_len || ret != 0 || lhs_pmu_name_len == 0 {
        return ret;
    }

    if lhs_pmu_name_len + 1 < strlen(lhs_pmu_name) {
        lhs_num = strtoull(lhs_pmu_name.add(lhs_pmu_name_len + 1), ptr::null_mut(), 16);
    }
    if rhs_pmu_name_len + 1 < strlen(rhs_pmu_name) {
        rhs_num = strtoull(rhs_pmu_name.add(rhs_pmu_name_len + 1), ptr::null_mut(), 16);
    }

    if lhs_num < rhs_num { -1 } else if lhs_num > rhs_num { 1 } else { 0 }
}

unsafe fn list_entry_perf_pmu(pos: *mut list_head) -> *mut perf_pmu {
    pos as *mut perf_pmu
}

unsafe fn list_first_entry_or_null_perf_pmu(head: *mut list_head) -> *mut perf_pmu {
    if list_empty(head) != 0 { ptr::null_mut() } else { list_entry_perf_pmu((*head).next) }
}

unsafe fn list_next_perf_pmu_or_null(head: *mut list_head, pmu: *mut perf_pmu) -> *mut perf_pmu {
    let next = if pmu.is_null() { (*head).next } else { (*pmu).list.next };
    if next == head { ptr::null_mut() } else { list_entry_perf_pmu(next) }
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmus__destroy() {
    let mut pmu = list_first_entry_or_null_perf_pmu(&mut core_pmus);
    while !pmu.is_null() {
        let tmp = list_next_perf_pmu_or_null(&mut core_pmus, pmu);
        list_del(&mut (*pmu).list);
        perf_pmu__delete(pmu);
        pmu = tmp;
    }
    pmu = list_first_entry_or_null_perf_pmu(&mut other_pmus);
    while !pmu.is_null() {
        let tmp = list_next_perf_pmu_or_null(&mut other_pmus, pmu);
        list_del(&mut (*pmu).list);
        perf_pmu__delete(pmu);
        pmu = tmp;
    }
    read_pmu_types = 0;
}

unsafe fn pmu_find(name: *const c_char) -> *mut perf_pmu {
    let mut pmu = list_first_entry_or_null_perf_pmu(&mut core_pmus);
    while !pmu.is_null() {
        if strcmp((*pmu).name, name) == 0 || (!(*pmu).alias_name.is_null() && strcmp((*pmu).alias_name, name) == 0) {
            return pmu;
        }
        pmu = list_next_perf_pmu_or_null(&mut core_pmus, pmu);
    }
    pmu = list_first_entry_or_null_perf_pmu(&mut other_pmus);
    while !pmu.is_null() {
        if strcmp((*pmu).name, name) == 0 || (!(*pmu).alias_name.is_null() && strcmp((*pmu).alias_name, name) == 0) {
            return pmu;
        }
        pmu = list_next_perf_pmu_or_null(&mut other_pmus, pmu);
    }
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmus__find(name: *const c_char) -> *mut perf_pmu {
    let mut pmu: *mut perf_pmu;
    let dirfd: c_int;
    let core_pmu: bool_t;
    let mut to_read_pmus: c_uint = 0;

    if strcmp(name, cstr_lit(b"default_core\0")) == 0 {
        return perf_pmus__find_core_pmu();
    }
    /*
     * Once PMU is loaded it stays in the list,
     * so we keep us from multiple reading/parsing
     * the pmu format definitions.
     */
    pmu = pmu_find(name);
    if !pmu.is_null() {
        return pmu;
    }

    if read_pmu_types == PERF_TOOL_PMU_TYPE_ALL_MASK {
        return ptr::null_mut();
    }

    core_pmu = is_pmu_core(name);
    if core_pmu && (read_pmu_types & PERF_TOOL_PMU_TYPE_PE_CORE_MASK) != 0 {
        return ptr::null_mut();
    }

    dirfd = perf_pmu__event_source_devices_fd();
    pmu = perf_pmu__lookup(if core_pmu { &mut core_pmus } else { &mut other_pmus }, dirfd, name, false);
    close(dirfd);

    if !pmu.is_null() {
        return pmu;
    }

    /* Looking up an individual perf event PMU failed, check if a tool PMU should be read. */
    if strncmp(name, cstr_lit(b"hwmon_\0"), 6) == 0 {
        to_read_pmus |= PERF_TOOL_PMU_TYPE_HWMON_MASK;
    } else if strncmp(name, cstr_lit(b"drm_\0"), 4) == 0 {
        to_read_pmus |= PERF_TOOL_PMU_TYPE_DRM_MASK;
    } else if strcmp(name, cstr_lit(b"tool\0")) == 0 {
        to_read_pmus |= PERF_TOOL_PMU_TYPE_TOOL_MASK;
    }

    if to_read_pmus != 0 {
        pmu_read_sysfs(to_read_pmus);
        pmu = pmu_find(name);
        if !pmu.is_null() {
            return pmu;
        }
    }
    /* Read all necessary PMUs from sysfs and see if the PMU is found. */
    to_read_pmus = PERF_TOOL_PMU_TYPE_PE_CORE_MASK;
    if !core_pmu {
        to_read_pmus |= PERF_TOOL_PMU_TYPE_PE_OTHER_MASK;
    }
    pmu_read_sysfs(to_read_pmus);
    pmu_find(name)
}

unsafe fn perf_pmu__find2(dirfd: c_int, name: *const c_char) -> *mut perf_pmu {
    let mut pmu: *mut perf_pmu;
    let core_pmu: bool_t;

    /*
     * Once PMU is loaded it stays in the list,
     * so we keep us from multiple reading/parsing
     * the pmu format definitions.
     */
    pmu = pmu_find(name);
    if !pmu.is_null() {
        return pmu;
    }

    if read_pmu_types == PERF_TOOL_PMU_TYPE_ALL_MASK {
        return ptr::null_mut();
    }

    core_pmu = is_pmu_core(name);
    if core_pmu && (read_pmu_types & PERF_TOOL_PMU_TYPE_PE_CORE_MASK) != 0 {
        return ptr::null_mut();
    }

    perf_pmu__lookup(if core_pmu { &mut core_pmus } else { &mut other_pmus }, dirfd, name, false)
}

unsafe extern "C" fn pmus_cmp(_priv: *mut c_void, lhs: *const list_head, rhs: *const list_head) -> c_int {
    let lhs_pmu = list_entry_perf_pmu(lhs as *mut list_head);
    let rhs_pmu = list_entry_perf_pmu(rhs as *mut list_head);
    let empty = cstr_lit(b"\0");
    pmu_name_cmp(if (*lhs_pmu).name.is_null() { empty } else { (*lhs_pmu).name },
                 if (*rhs_pmu).name.is_null() { empty } else { (*rhs_pmu).name })
}

/* Add all pmus in sysfs to pmu list: */
unsafe fn pmu_read_sysfs(to_read_types: c_uint) {
    let mut tool_pmu: *mut perf_pmu;

    if (read_pmu_types & to_read_types) == to_read_types {
        /* All requested PMU types have been read. */
        return;
    }

    if (to_read_types & (PERF_TOOL_PMU_TYPE_PE_CORE_MASK | PERF_TOOL_PMU_TYPE_PE_OTHER_MASK)) != 0 {
        let fd = perf_pmu__event_source_devices_fd();
        let mut dir: io_dir = mem::zeroed();
        let mut dent: *mut io_dirent64;
        let core_only = (to_read_types & PERF_TOOL_PMU_TYPE_PE_OTHER_MASK) == 0;

        if fd < 0 {
            goto_skip_pe_pmus(to_read_types);
            return;
        }

        io_dir__init(&mut dir, fd);

        loop {
            dent = io_dir__readdir(&mut dir);
            if dent.is_null() {
                break;
            }
            let d_name = (*dent).d_name.as_ptr();
            if strcmp(d_name, cstr_lit(b".\0")) == 0 || strcmp(d_name, cstr_lit(b"..\0")) == 0 {
                continue;
            }
            if core_only && !is_pmu_core(d_name) {
                continue;
            }
            /* add to static LIST_HEAD(core_pmus) or LIST_HEAD(other_pmus): */
            perf_pmu__find2(fd, d_name);
        }

        close(fd);
    }
    goto_skip_pe_pmus(to_read_types);
}

unsafe fn goto_skip_pe_pmus(to_read_types: c_uint) {
    let mut tool_pmu: *mut perf_pmu;

    if (to_read_types & PERF_TOOL_PMU_TYPE_PE_CORE_MASK) != 0 && list_empty(&mut core_pmus) != 0 {
        if !perf_pmu__create_placeholder_core_pmu(&mut core_pmus) {
            pr_err(cstr_lit(b"Failure to set up any core PMUs\n\0"));
        }
    }
    list_sort(ptr::null_mut(), &mut core_pmus, pmus_cmp);

    if (to_read_types & PERF_TOOL_PMU_TYPE_TOOL_MASK) != 0
        && (read_pmu_types & PERF_TOOL_PMU_TYPE_TOOL_MASK) == 0
    {
        tool_pmu = tool_pmu__new();
        if !tool_pmu.is_null() {
            list_add_tail(&mut (*tool_pmu).list, &mut other_pmus);
        }
    }
    if (to_read_types & PERF_TOOL_PMU_TYPE_HWMON_MASK) != 0
        && (read_pmu_types & PERF_TOOL_PMU_TYPE_HWMON_MASK) == 0
    {
        perf_pmus__read_hwmon_pmus(&mut other_pmus);
    }

    if (to_read_types & PERF_TOOL_PMU_TYPE_DRM_MASK) != 0
        && (read_pmu_types & PERF_TOOL_PMU_TYPE_DRM_MASK) == 0
    {
        perf_pmus__read_drm_pmus(&mut other_pmus);
    }

    list_sort(ptr::null_mut(), &mut other_pmus, pmus_cmp);

    read_pmu_types |= to_read_types;
}

unsafe fn __perf_pmus__find_by_type(type_: c_uint) -> *mut perf_pmu {
    let mut pmu = list_first_entry_or_null_perf_pmu(&mut core_pmus);
    while !pmu.is_null() {
        if (*pmu).type_ == type_ {
            return pmu;
        }
        pmu = list_next_perf_pmu_or_null(&mut core_pmus, pmu);
    }

    pmu = list_first_entry_or_null_perf_pmu(&mut other_pmus);
    while !pmu.is_null() {
        if (*pmu).type_ == type_ {
            return pmu;
        }
        pmu = list_next_perf_pmu_or_null(&mut other_pmus, pmu);
    }
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmus__find_by_type(type_: c_uint) -> *mut perf_pmu {
    let to_read_pmus: c_uint;
    let mut pmu = __perf_pmus__find_by_type(type_);

    if !pmu.is_null() || read_pmu_types == PERF_TOOL_PMU_TYPE_ALL_MASK {
        return pmu;
    }

    if type_ >= PERF_PMU_TYPE_PE_START && type_ <= PERF_PMU_TYPE_PE_END {
        to_read_pmus = PERF_TOOL_PMU_TYPE_PE_CORE_MASK | PERF_TOOL_PMU_TYPE_PE_OTHER_MASK;
    } else if type_ >= PERF_PMU_TYPE_DRM_START && type_ <= PERF_PMU_TYPE_DRM_END {
        to_read_pmus = PERF_TOOL_PMU_TYPE_DRM_MASK;
    } else if type_ >= PERF_PMU_TYPE_HWMON_START && type_ <= PERF_PMU_TYPE_HWMON_END {
        to_read_pmus = PERF_TOOL_PMU_TYPE_HWMON_MASK;
    } else {
        to_read_pmus = PERF_TOOL_PMU_TYPE_TOOL_MASK;
    }
    pmu_read_sysfs(to_read_pmus);
    pmu = __perf_pmus__find_by_type(type_);
    pmu
}

/*
 * pmu iterator: If pmu is NULL, we start at the begin, otherwise return the
 * next pmu. Returns NULL on end.
 */
#[no_mangle]
pub unsafe extern "C" fn perf_pmus__scan(mut pmu: *mut perf_pmu) -> *mut perf_pmu {
    let use_core_pmus = pmu.is_null() || (*pmu).is_core;

    if pmu.is_null() {
        pmu_read_sysfs(PERF_TOOL_PMU_TYPE_ALL_MASK);
        pmu = ptr::null_mut();
    }
    if use_core_pmus {
        let next = list_next_perf_pmu_or_null(&mut core_pmus, pmu);
        if !next.is_null() {
            return next;
        }
        pmu = ptr::null_mut();
    }
    list_next_perf_pmu_or_null(&mut other_pmus, pmu)
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmus__scan_core(mut pmu: *mut perf_pmu) -> *mut perf_pmu {
    if pmu.is_null() {
        pmu_read_sysfs(PERF_TOOL_PMU_TYPE_PE_CORE_MASK);
        return list_first_entry_or_null_perf_pmu(&mut core_pmus);
    }
    list_next_perf_pmu_or_null(&mut core_pmus, pmu)
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmus__scan_for_event(mut pmu: *mut perf_pmu, event: *const c_char) -> *mut perf_pmu {
    let use_core_pmus = pmu.is_null() || (*pmu).is_core;

    if pmu.is_null() {
        /* Hwmon filename values that aren't used. */
        let mut type_: hwmon_type = mem::zeroed();
        let mut number: c_int = 0;
        /*
         * Core PMUs, other sysfs PMUs and tool PMU can take all event
         * types or aren't wother optimizing for.
         */
        let mut to_read_pmus = PERF_TOOL_PMU_TYPE_PE_CORE_MASK
            | PERF_TOOL_PMU_TYPE_PE_OTHER_MASK
            | PERF_TOOL_PMU_TYPE_TOOL_MASK;

        /* Could the event be a hwmon event? */
        if parse_hwmon_filename(event, &mut type_, &mut number, ptr::null_mut(), ptr::null_mut()) {
            to_read_pmus |= PERF_TOOL_PMU_TYPE_HWMON_MASK;
        }

        /* Could the event be a DRM event? */
        if strlen(event) > 4 && strncmp(cstr_lit(b"drm-\0"), event, 4) == 0 {
            to_read_pmus |= PERF_TOOL_PMU_TYPE_DRM_MASK;
        }

        pmu_read_sysfs(to_read_pmus);
        pmu = ptr::null_mut();
    }
    if use_core_pmus {
        let next = list_next_perf_pmu_or_null(&mut core_pmus, pmu);
        if !next.is_null() {
            return next;
        }
        pmu = ptr::null_mut();
    }
    list_next_perf_pmu_or_null(&mut other_pmus, pmu)
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmus__scan_matching_wildcard(mut pmu: *mut perf_pmu, wildcard: *const c_char) -> *mut perf_pmu {
    let use_core_pmus = pmu.is_null() || (*pmu).is_core;

    if pmu.is_null() {
        /*
         * Core PMUs, other sysfs PMUs and tool PMU can have any name or
         * aren't worth optimizing for.
         */
        let mut to_read_pmus = PERF_TOOL_PMU_TYPE_PE_CORE_MASK
            | PERF_TOOL_PMU_TYPE_PE_OTHER_MASK
            | PERF_TOOL_PMU_TYPE_TOOL_MASK;

        /*
         * Hwmon PMUs have an alias from a sysfs name like hwmon0,
         * hwmon1, etc. or have a name of hwmon_<name>. They therefore
         * can only have a wildcard match if the wildcard begins with
         * "hwmon". Similarly drm PMUs must start "drm_", avoid reading
         * such events unless the PMU could match.
         */
        if strisglob(wildcard) {
            to_read_pmus |= PERF_TOOL_PMU_TYPE_HWMON_MASK | PERF_TOOL_PMU_TYPE_DRM_MASK;
        } else if strlen(wildcard) >= 4 && strncmp(cstr_lit(b"drm_\0"), wildcard, 4) == 0 {
            to_read_pmus |= PERF_TOOL_PMU_TYPE_DRM_MASK;
        } else if strlen(wildcard) >= 5 && strncmp(cstr_lit(b"hwmon\0"), wildcard, 5) == 0 {
            to_read_pmus |= PERF_TOOL_PMU_TYPE_HWMON_MASK;
        }

        pmu_read_sysfs(to_read_pmus);
        pmu = ptr::null_mut();
    }
    if use_core_pmus {
        loop {
            pmu = list_next_perf_pmu_or_null(&mut core_pmus, pmu);
            if pmu.is_null() {
                break;
            }
            if perf_pmu__wildcard_match(pmu, wildcard) {
                return pmu;
            }
        }
        pmu = ptr::null_mut();
    }
    loop {
        pmu = list_next_perf_pmu_or_null(&mut other_pmus, pmu);
        if pmu.is_null() {
            break;
        }
        if perf_pmu__wildcard_match(pmu, wildcard) {
            return pmu;
        }
    }
    ptr::null_mut()
}

unsafe fn perf_pmus__scan_skip_duplicates(mut pmu: *mut perf_pmu) -> *mut perf_pmu {
    let use_core_pmus = pmu.is_null() || (*pmu).is_core;
    let mut last_pmu_name_len: c_int = 0;
    let last_pmu_name = if !pmu.is_null() && !(*pmu).name.is_null() { (*pmu).name } else { cstr_lit(b"\0") };

    if pmu.is_null() {
        pmu_read_sysfs(PERF_TOOL_PMU_TYPE_ALL_MASK);
        pmu = ptr::null_mut();
    } else {
        last_pmu_name_len = pmu_name_len_no_suffix(if (*pmu).name.is_null() { cstr_lit(b"\0") } else { (*pmu).name }) as c_int;
    }

    if use_core_pmus {
        loop {
            pmu = list_next_perf_pmu_or_null(&mut core_pmus, pmu);
            if pmu.is_null() {
                break;
            }
            let pmu_name = if (*pmu).name.is_null() { cstr_lit(b"\0") } else { (*pmu).name };
            let pmu_name_len = pmu_name_len_no_suffix(pmu_name) as c_int;
            if last_pmu_name_len == pmu_name_len && strncmp(last_pmu_name, pmu_name, pmu_name_len as size_t) == 0 {
                continue;
            }
            return pmu;
        }
        pmu = ptr::null_mut();
    }
    loop {
        pmu = list_next_perf_pmu_or_null(&mut other_pmus, pmu);
        if pmu.is_null() {
            break;
        }
        let pmu_name = if (*pmu).name.is_null() { cstr_lit(b"\0") } else { (*pmu).name };
        let pmu_name_len = pmu_name_len_no_suffix(pmu_name) as c_int;
        if last_pmu_name_len == pmu_name_len && strncmp(last_pmu_name, pmu_name, pmu_name_len as size_t) == 0 {
            continue;
        }
        return pmu;
    }
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmus__scan_for_uncore_id(mut pmu: *mut perf_pmu, compat: *const c_char) -> *mut perf_pmu {
    if pmu.is_null() {
        /* Only uncore PMUs can have identifiers. */
        let to_read_pmus = PERF_TOOL_PMU_TYPE_PE_OTHER_MASK;

        pmu_read_sysfs(to_read_pmus);
        pmu = ptr::null_mut();
    }
    loop {
        pmu = list_next_perf_pmu_or_null(&mut other_pmus, pmu);
        if pmu.is_null() {
            break;
        }
        if !(*pmu).id.is_null() && pmu_uncore_identifier_match(compat, (*pmu).id) {
            return pmu;
        }
    }
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmus__pmu_for_pmu_filter(str_: *const c_char) -> *const perf_pmu {
    let mut pmu: *mut perf_pmu = ptr::null_mut();

    loop {
        pmu = perf_pmus__scan(pmu);
        if pmu.is_null() {
            break;
        }
        if strcmp((*pmu).name, str_) == 0 {
            return pmu;
        }
        /* Ignore "uncore_" prefix. */
        if strncmp((*pmu).name, cstr_lit(b"uncore_\0"), 7) == 0 {
            if strcmp((*pmu).name.add(7), str_) == 0 {
                return pmu;
            }
        }
        /* Ignore "cpu_" prefix on Intel hybrid PMUs. */
        if strncmp((*pmu).name, cstr_lit(b"cpu_\0"), 4) == 0 {
            if strcmp((*pmu).name.add(4), str_) == 0 {
                return pmu;
            }
        }
    }
    ptr::null()
}

/** Struct for ordering events as output in perf list. */
#[repr(C)]
pub struct sevent {
    /** PMU for event. */
    pub pmu: *const perf_pmu,
    pub name: *const c_char,
    pub alias: *const c_char,
    pub scale_unit: *const c_char,
    pub desc: *const c_char,
    pub long_desc: *const c_char,
    pub encoding_desc: *const c_char,
    pub topic: *const c_char,
    pub pmu_name: *const c_char,
    pub event_type_desc: *const c_char,
    pub deprecated: bool_t,
}

unsafe extern "C" fn cmp_sevent(a: *const c_void, b: *const c_void) -> c_int {
    let as_ = a as *const sevent;
    let bs = b as *const sevent;
    let a_iscpu: bool_t;
    let b_iscpu: bool_t;
    let mut ret: c_int;
    let empty = cstr_lit(b"\0");

    /* Put extra events last. */
    if (!(*as_).desc.is_null()) != (!(*bs).desc.is_null()) {
        return (!(*as_).desc.is_null()) as c_int - (!(*bs).desc.is_null()) as c_int;
    }

    /* Order by topics. */
    ret = strcmp(if (*as_).topic.is_null() { empty } else { (*as_).topic },
                 if (*bs).topic.is_null() { empty } else { (*bs).topic });
    if ret != 0 {
        return ret;
    }

    /* Order CPU core events to be first */
    a_iscpu = if !(*as_).pmu.is_null() { (*(*as_).pmu).is_core } else { true };
    b_iscpu = if !(*bs).pmu.is_null() { (*(*bs).pmu).is_core } else { true };
    if a_iscpu != b_iscpu {
        return if a_iscpu { -1 } else { 1 };
    }

    /* Order by PMU name. */
    if (*as_).pmu != (*bs).pmu {
        ret = strcmp(if (*as_).pmu_name.is_null() { empty } else { (*as_).pmu_name },
                     if (*bs).pmu_name.is_null() { empty } else { (*bs).pmu_name });
        if ret != 0 {
            return ret;
        }
    }

    /* Order by event name. */
    strcmp((*as_).name, (*bs).name)
}

unsafe fn pmu_alias_is_duplicate(a: *mut sevent, b: *mut sevent) -> bool_t {
    /* Different names -> never duplicates */
    if strcmp(if (*a).name.is_null() { cstr_lit(b"//\0") } else { (*a).name },
              if (*b).name.is_null() { cstr_lit(b"//\0") } else { (*b).name }) != 0 {
        return false;
    }

    /* Don't remove duplicates for different PMUs */
    strcmp((*a).pmu_name, (*b).pmu_name) == 0
}

#[repr(C)]
pub struct events_callback_state {
    pub aliases: *mut sevent,
    pub aliases_len: size_t,
    pub index: size_t,
}

unsafe extern "C" fn perf_pmus__print_pmu_events__callback(vstate: *mut c_void, info: *mut pmu_event_info) -> c_int {
    let state = vstate as *mut events_callback_state;
    let s: *mut sevent;

    if (*state).index >= (*state).aliases_len {
        pr_err(cstr_lit(b"Unexpected event %s/%s/\n\0"), (*(*info).pmu).name, (*info).name);
        return 1;
    }
    debug_assert!(!(*info).pmu.is_null() || !(*info).name.is_null());
    s = (*state).aliases.add((*state).index);
    (*s).pmu = (*info).pmu;
    (*s).name = if !(*info).name.is_null() { strdup((*info).name) } else { ptr::null_mut() };
    (*s).alias = if !(*info).alias.is_null() { strdup((*info).alias) } else { ptr::null_mut() };
    (*s).scale_unit = if !(*info).scale_unit.is_null() { strdup((*info).scale_unit) } else { ptr::null_mut() };
    (*s).desc = if !(*info).desc.is_null() { strdup((*info).desc) } else { ptr::null_mut() };
    (*s).long_desc = if !(*info).long_desc.is_null() { strdup((*info).long_desc) } else { ptr::null_mut() };
    (*s).encoding_desc = if !(*info).encoding_desc.is_null() { strdup((*info).encoding_desc) } else { ptr::null_mut() };
    (*s).topic = if !(*info).topic.is_null() { strdup((*info).topic) } else { ptr::null_mut() };
    (*s).pmu_name = if !(*info).pmu_name.is_null() { strdup((*info).pmu_name) } else { ptr::null_mut() };
    (*s).event_type_desc = if !(*info).event_type_desc.is_null() { strdup((*info).event_type_desc) } else { ptr::null_mut() };
    (*s).deprecated = (*info).deprecated;
    (*state).index += 1;
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmus__print_pmu_events(print_cb: *const print_callbacks, print_state: *mut c_void) {
    let mut pmu: *mut perf_pmu;
    let printed: c_int = 0;
    let mut len: c_int;
    let aliases: *mut sevent;
    let mut state: events_callback_state;
    let skip_duplicate_pmus = ((*print_cb).skip_duplicate_pmus)(print_state);
    let scan_fn: unsafe extern "C" fn(*mut perf_pmu) -> *mut perf_pmu =
        if skip_duplicate_pmus { perf_pmus__scan_skip_duplicates_shim } else { perf_pmus__scan };

    pmu = ptr::null_mut();
    len = 0;
    loop {
        pmu = scan_fn(pmu);
        if pmu.is_null() {
            break;
        }
        len += perf_pmu__num_events(pmu);
    }

    aliases = calloc(len as size_t, mem::size_of::<sevent>()) as *mut sevent;
    if aliases.is_null() {
        pr_err(cstr_lit(b"FATAL: not enough memory to print PMU events\n\0"));
        return;
    }
    pmu = ptr::null_mut();
    state = events_callback_state {
        aliases,
        aliases_len: len as size_t,
        index: 0,
    };
    loop {
        pmu = scan_fn(pmu);
        if pmu.is_null() {
            break;
        }
        perf_pmu__for_each_event(pmu, skip_duplicate_pmus, &mut state as *mut _ as *mut c_void,
                                 perf_pmus__print_pmu_events__callback);
    }
    qsort(aliases as *mut c_void, len as size_t, mem::size_of::<sevent>(), cmp_sevent);
    for j in 0..len {
        let sj = aliases.add(j as usize);
        /* Skip duplicates */
        if !(j < len - 1 && pmu_alias_is_duplicate(sj, aliases.add((j + 1) as usize))) {
            ((*print_cb).print_event)(
                print_state,
                (*sj).topic,
                (*sj).pmu_name,
                (*(*sj).pmu).type_,
                (*sj).name,
                (*sj).alias,
                (*sj).scale_unit,
                (*sj).deprecated,
                (*sj).event_type_desc,
                (*sj).desc,
                (*sj).long_desc,
                (*sj).encoding_desc,
            );
        }
        zfree(&mut (*sj).name as *mut _ as *mut c_void);
        zfree(&mut (*sj).alias as *mut _ as *mut c_void);
        zfree(&mut (*sj).scale_unit as *mut _ as *mut c_void);
        zfree(&mut (*sj).desc as *mut _ as *mut c_void);
        zfree(&mut (*sj).long_desc as *mut _ as *mut c_void);
        zfree(&mut (*sj).encoding_desc as *mut _ as *mut c_void);
        zfree(&mut (*sj).topic as *mut _ as *mut c_void);
        zfree(&mut (*sj).pmu_name as *mut _ as *mut c_void);
        zfree(&mut (*sj).event_type_desc as *mut _ as *mut c_void);
    }
    if printed != 0 && pager_in_use() {
        printf(cstr_lit(b"\n\0"));
    }

    zfree(&mut (aliases as *mut c_void) as *mut _ as *mut c_void);
}

unsafe extern "C" fn perf_pmus__scan_skip_duplicates_shim(pmu: *mut perf_pmu) -> *mut perf_pmu {
    perf_pmus__scan_skip_duplicates(pmu)
}

#[repr(C)]
pub struct build_format_string_args {
    pub short_string: strbuf,
    pub long_string: strbuf,
    pub num_formats: c_int,
}

unsafe extern "C" fn build_format_string(state: *mut c_void, name: *const c_char, config: c_int,
                                         bits: *const c_ulong) -> c_int {
    let args = state as *mut build_format_string_args;
    let num_bits: c_uint;
    let ret1: c_int;
    let mut ret2: c_int = 0;

    let _ = config;
    (*args).num_formats += 1;
    if (*args).num_formats > 1 {
        strbuf_addch(&mut (*args).long_string, b',' as c_int);
        if (*args).num_formats < 4 {
            strbuf_addch(&mut (*args).short_string, b',' as c_int);
        }
    }
    num_bits = if !bits.is_null() { bitmap_weight(bits, PERF_PMU_FORMAT_BITS) } else { 0 };
    if num_bits <= 1 {
        ret1 = strbuf_addf(&mut (*args).long_string, cstr_lit(b"%s\0"), name);
        if (*args).num_formats < 4 {
            ret2 = strbuf_addf(&mut (*args).short_string, cstr_lit(b"%s\0"), name);
        }
    } else if num_bits > 8 {
        ret1 = strbuf_addf(&mut (*args).long_string, cstr_lit(b"%s=0..0x%llx\0"), name,
                           ULLONG_MAX >> (64 - num_bits));
        if (*args).num_formats < 4 {
            ret2 = strbuf_addf(&mut (*args).short_string, cstr_lit(b"%s=0..0x%llx\0"), name,
                               ULLONG_MAX >> (64 - num_bits));
        }
    } else {
        ret1 = strbuf_addf(&mut (*args).long_string, cstr_lit(b"%s=0..%llu\0"), name,
                           ULLONG_MAX >> (64 - num_bits));
        if (*args).num_formats < 4 {
            ret2 = strbuf_addf(&mut (*args).short_string, cstr_lit(b"%s=0..%llu\0"), name,
                               ULLONG_MAX >> (64 - num_bits));
        }
    }
    if ret1 < 0 { ret1 } else if ret2 < 0 { ret2 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmus__print_raw_pmu_events(print_cb: *const print_callbacks, print_state: *mut c_void) {
    let skip_duplicate_pmus = ((*print_cb).skip_duplicate_pmus)(print_state);
    let scan_fn: unsafe extern "C" fn(*mut perf_pmu) -> *mut perf_pmu =
        if skip_duplicate_pmus { perf_pmus__scan_skip_duplicates_shim } else { perf_pmus__scan };
    let mut pmu: *mut perf_pmu = ptr::null_mut();

    loop {
        pmu = scan_fn(pmu);
        if pmu.is_null() {
            break;
        }
        let mut format_args = build_format_string_args {
            short_string: strbuf { alloc: 0, len: 0, buf: ptr::null_mut() },
            long_string: strbuf { alloc: 0, len: 0, buf: ptr::null_mut() },
            num_formats: 0,
        };
        let len = pmu_name_len_no_suffix((*pmu).name) as c_int;
        let mut desc = cstr_lit(b"(see 'man perf-list' or 'man perf-record' on how to encode it)\0");

        if !(*pmu).is_core {
            desc = ptr::null();
        }

        strbuf_addf(&mut format_args.short_string, cstr_lit(b"%.*s/\0"), len, (*pmu).name);
        strbuf_addf(&mut format_args.long_string, cstr_lit(b"%.*s/\0"), len, (*pmu).name);
        perf_pmu__for_each_format(pmu, &mut format_args as *mut _ as *mut c_void, build_format_string);

        if format_args.num_formats > 3 {
            strbuf_addf(&mut format_args.short_string, cstr_lit(b",.../modifier\0"));
        } else {
            strbuf_addf(&mut format_args.short_string, cstr_lit(b"/modifier\0"));
        }

        strbuf_addf(&mut format_args.long_string, cstr_lit(b"/modifier\0"));
        ((*print_cb).print_event)(
            print_state,
            ptr::null(),
            ptr::null(),
            (*pmu).type_,
            format_args.short_string.buf,
            ptr::null(),
            ptr::null(),
            false,
            cstr_lit(b"Raw event descriptor\0"),
            desc,
            ptr::null(),
            format_args.long_string.buf,
        );

        strbuf_release(&mut format_args.short_string);
        strbuf_release(&mut format_args.long_string);
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmus__have_event(pname: *const c_char, name: *const c_char) -> bool_t {
    let pmu = perf_pmus__find(pname);

    !pmu.is_null() && perf_pmu__have_event(pmu, name)
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmus__num_core_pmus() -> c_int {
    static mut count: c_int = 0;

    if count == 0 {
        let mut pmu: *mut perf_pmu = ptr::null_mut();

        loop {
            pmu = perf_pmus__scan_core(pmu);
            if pmu.is_null() {
                break;
            }
            count += 1;
        }
    }
    count
}

unsafe fn __perf_pmus__supports_extended_type() -> bool_t {
    let mut pmu: *mut perf_pmu = ptr::null_mut();

    if perf_pmus__num_core_pmus() <= 1 {
        return false;
    }

    loop {
        pmu = perf_pmus__scan_core(pmu);
        if pmu.is_null() {
            break;
        }
        if !is_event_supported(PERF_TYPE_HARDWARE,
                               PERF_COUNT_HW_CPU_CYCLES | (((*pmu).type_ as __u64) << PERF_PMU_TYPE_SHIFT)) {
            return false;
        }
    }

    true
}

unsafe extern "C" fn perf_pmus__init_supports_extended_type() {
    perf_pmus__do_support_extended_type = __perf_pmus__supports_extended_type();
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmus__supports_extended_type() -> bool_t {
    static mut extended_type_once: pthread_once_t = PTHREAD_ONCE_INIT;

    pthread_once(&mut extended_type_once, perf_pmus__init_supports_extended_type);

    perf_pmus__do_support_extended_type
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmus__find_by_attr(attr: *const perf_event_attr) -> *mut perf_pmu {
    let mut pmu = perf_pmus__find_by_type((*attr).type_);
    let mut type_ = (*attr).type_;
    let legacy_core_type = type_ == PERF_TYPE_HARDWARE || type_ == PERF_TYPE_HW_CACHE;

    if pmu.is_null() && legacy_core_type && perf_pmus__supports_extended_type() {
        type_ = ((*attr).config >> PERF_PMU_TYPE_SHIFT) as u32;

        pmu = perf_pmus__find_by_type(type_);
    }
    if pmu.is_null() && (legacy_core_type || type_ == PERF_TYPE_RAW) {
        /*
         * For legacy events, if there was no extended type info then
         * assume the PMU is the first core PMU.
         *
         * On architectures like ARM there is no sysfs PMU with type
         * PERF_TYPE_RAW, assume the RAW events are going to be handled
         * by the first core PMU.
         */
        pmu = perf_pmus__find_core_pmu();
    }
    pmu
}

#[no_mangle]
pub unsafe extern "C" fn evsel__find_pmu(evsel: *const evsel) -> *mut perf_pmu {
    let mut pmu = (*evsel).pmu;

    if !pmu.is_null() {
        return pmu;
    }

    pmu = perf_pmus__find_by_attr(&(*evsel).core.attr);
    (*(evsel as *mut evsel)).pmu = pmu;
    pmu
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmus__find_core_pmu() -> *mut perf_pmu {
    perf_pmus__scan_core(ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmus__add_test_pmu(test_sysfs_dirfd: c_int, name: *const c_char) -> *mut perf_pmu {
    /*
     * Some PMU functions read from the sysfs mount point, so care is
     * needed, hence passing the eager_load flag to load things like the
     * format files.
     */
    perf_pmu__lookup(&mut other_pmus, test_sysfs_dirfd, name, true)
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmus__add_test_hwmon_pmu(
    hwmon_dir: *const c_char,
    sysfs_name: *const c_char,
    name: *const c_char,
) -> *mut perf_pmu {
    hwmon_pmu__new(&mut other_pmus, hwmon_dir, sysfs_name, name)
}

#[no_mangle]
pub unsafe extern "C" fn perf_pmus__fake_pmu() -> *mut perf_pmu {
    static mut fake: perf_pmu = perf_pmu {
        name: b"fake\0".as_ptr() as *const c_char,
        type_: PERF_PMU_TYPE_FAKE,
        list: list_head { next: ptr::null_mut(), prev: ptr::null_mut() },
        alias_name: ptr::null(),
        is_core: false,
        is_uncore: false,
        id: ptr::null(),
        format: list_head { next: ptr::null_mut(), prev: ptr::null_mut() },
    };

    &mut fake
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
