// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
// Translated from perf/util/drm_pmu.c.
// C include dependencies intentionally remain external to this isolated file:
// drm_pmu.h, counts.h, cpumap.h, debug.h, evsel.h, pmu.h, perf/threadmap.h,
// api/fs/fs.h, api/io.h, and the referenced libc/Linux APIs.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type uint64_t = u64;
type __u32 = u32;
type mode_t = c_uint;
type dev_t = u64;

const DRM_PMU_UNIT_BYTES: drm_pmu_unit = 0;
const DRM_PMU_UNIT_CAPACITY: drm_pmu_unit = 1;
const DRM_PMU_UNIT_CYCLES: drm_pmu_unit = 2;
const DRM_PMU_UNIT_HZ: drm_pmu_unit = 3;
const DRM_PMU_UNIT_NS: drm_pmu_unit = 4;
const DRM_PMU_UNIT_MAX: usize = 5;
type drm_pmu_unit = c_uint;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct perf_pmu {
    pub list: list_head,
    pub name: *const c_char,
    pub type_: __u32,
    pub cpus: *mut c_void,
}

#[repr(C)]
pub struct perf_event_attr {
    pub config: uint64_t,
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
    pub threads: *mut perf_thread_map,
}

#[repr(C)]
pub struct evsel {
    pub pmu: *mut perf_pmu,
    pub core: evsel_core,
    pub prev_raw_counts: *mut c_void,
    pub counts: *mut c_void,
}

#[repr(C)]
pub struct perf_thread_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_counts_values {
    pub val: uint64_t,
    pub ena: uint64_t,
    pub run: uint64_t,
}

#[repr(C)]
pub struct parse_events_term {
    pub list: list_head,
    pub type_term: c_int,
    pub config: *const c_char,
    pub err_val: c_int,
}

#[repr(C)]
pub struct parse_events_terms {
    pub terms: list_head,
}

#[repr(C)]
pub struct parse_events_error {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_pmu_info {
    pub unit: *const c_char,
    pub scale: c_int,
}

#[repr(C)]
pub struct pmu_event_info {
    pub pmu: *const perf_pmu,
    pub name: *const c_char,
    pub alias: *const c_char,
    pub scale_unit: *const c_char,
    pub desc: *const c_char,
    pub long_desc: *const c_char,
    pub encoding_desc: *mut c_char,
    pub topic: *const c_char,
    pub pmu_name: *const c_char,
    pub event_type_desc: *const c_char,
}

type pmu_event_callback =
    Option<unsafe extern "C" fn(state: *mut c_void, info: *mut pmu_event_info) -> c_int>;

#[repr(C)]
pub struct io {
    pub fd: c_int,
}

#[repr(C)]
pub struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dirent {
    pub d_ino: u64,
    pub d_off: i64,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [c_char; 256],
}

#[repr(C)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: mode_t,
    pub st_uid: c_uint,
    pub st_gid: c_uint,
    pub __pad0: c_int,
    pub st_rdev: dev_t,
}

#[repr(C)]
pub struct drm_pmu_event {
    pub name: *const c_char,
    pub desc: *const c_char,
    pub unit: drm_pmu_unit,
}

#[repr(C)]
pub struct drm_pmu {
    pub pmu: perf_pmu,
    pub events: *mut drm_pmu_event,
    pub num_events: c_int,
}

#[repr(C)]
pub struct minor_info {
    pub minors: *mut c_uint,
    pub minors_num: c_int,
    pub minors_len: c_int,
}

#[repr(C)]
pub struct read_drm_event_cb_args {
    pub match_: *const c_char,
    pub count: uint64_t,
    pub unit: drm_pmu_unit,
}

const PERF_PMU_TYPE_DRM_START: __u32 = 0x4000_0000;
const PERF_PMU_TYPE_DRM_END: __u32 = 0x4000_00ff;
const PARSE_EVENTS__TERM_TYPE_USER: c_int = 1;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const O_RDONLY: c_int = 0;
const O_DIRECTORY: c_int = 0o200000;
const DT_DIR: u8 = 4;
const DT_LNK: u8 = 10;
const S_IFMT: mode_t = 0o170000;
const S_IFCHR: mode_t = 0o020000;

const BYTES: &[u8] = b"bytes\0";
const CAPACITY: &[u8] = b"capacity\0";
const CYCLES: &[u8] = b"cycles\0";
const HZ: &[u8] = b"hz\0";
const NS: &[u8] = b"ns\0";
const SCALE_BYTES: &[u8] = b"1bytes\0";
const SCALE_CAPACITY: &[u8] = b"1capacity\0";
const SCALE_CYCLES: &[u8] = b"1cycles\0";
const SCALE_HZ: &[u8] = b"1hz\0";
const SCALE_NS: &[u8] = b"1ns\0";

static drm_pmu_unit_strs: [*const c_char; DRM_PMU_UNIT_MAX] = [
    BYTES.as_ptr() as *const c_char,
    CAPACITY.as_ptr() as *const c_char,
    CYCLES.as_ptr() as *const c_char,
    HZ.as_ptr() as *const c_char,
    NS.as_ptr() as *const c_char,
];

static drm_pmu_scale_unit_strs: [*const c_char; DRM_PMU_UNIT_MAX] = [
    SCALE_BYTES.as_ptr() as *const c_char,
    SCALE_CAPACITY.as_ptr() as *const c_char,
    SCALE_CYCLES.as_ptr() as *const c_char,
    SCALE_HZ.as_ptr() as *const c_char,
    SCALE_NS.as_ptr() as *const c_char,
];

unsafe extern "C" {
    static verbose: c_int;

    fn perf_pmu__init(pmu: *mut perf_pmu, type_: __u32, name: *const c_char) -> c_int;
    fn perf_pmu__delete(pmu: *mut perf_pmu);
    fn perf_cpu_map__new_int(cpu: c_int) -> *mut c_void;
    fn perf_counts(counts: *mut c_void, cpu_map_idx: c_int, thread: c_int) -> *mut perf_counts_values;
    fn perf_thread_map__pid(threads: *mut perf_thread_map, thread: c_int) -> c_int;

    fn io__init(io: *mut io, fd: c_int, buf: *mut c_char, size: size_t);
    fn io__getline(io: *mut io, line: *mut *mut c_char, line_len: *mut size_t) -> isize;
    fn procfs__mountpoint() -> *const c_char;

    fn parse_events_error__handle(
        err: *mut parse_events_error,
        idx: c_int,
        str_: *mut c_char,
        help: *mut c_char,
    );
    fn parse_events__term_type_str(type_term: c_int) -> *const c_char;

    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fdopendir(fd: c_int) -> *mut DIR;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn dirfd(dirp: *mut DIR) -> c_int;
    fn fstatat(dirfd: c_int, pathname: *const c_char, statbuf: *mut stat, flags: c_int) -> c_int;

    fn free(ptr: *mut c_void);
    fn zalloc(size: size_t) -> *mut c_void;
    fn reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void;
    fn strndup(s: *const c_char, n: size_t) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn scnprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;

    fn isspace(c: c_int) -> c_int;
    fn isdigit(c: c_int) -> c_int;
    fn isblank(c: c_int) -> c_int;
    fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: c_uint,
        function: *const c_char,
    ) -> !;
}

unsafe fn container_of_perf_pmu(pmu: *const perf_pmu) -> *mut drm_pmu {
    pmu as *mut drm_pmu
}

unsafe fn list_add_tail(new_: *mut list_head, head: *mut list_head) {
    (*new_).prev = (*head).prev;
    (*new_).next = head;
    (*(*head).prev).next = new_;
    (*head).prev = new_;
}

unsafe fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool {
    strncmp(str_, prefix, strlen(prefix)) == 0
}

unsafe fn major(dev: dev_t) -> c_uint {
    ((dev >> 8) & 0xfff) as c_uint
}

unsafe fn minor(dev: dev_t) -> c_uint {
    ((dev & 0xff) | ((dev >> 12) & 0xfffff00)) as c_uint
}

unsafe fn pr_err(fmt: *const c_char, arg: *const c_char) {
    unsafe extern "C" {
        fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
        static mut stderr: *mut c_void;
    }
    fprintf(stderr, fmt, arg);
}

unsafe fn pr_debug(fmt: *const c_char, arg: *const c_char) {
    unsafe extern "C" {
        fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
        static mut stderr: *mut c_void;
    }
    fprintf(stderr, fmt, arg);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_pmu__is_drm(pmu: *const perf_pmu) -> bool {
    !pmu.is_null()
        && (*pmu).type_ >= PERF_PMU_TYPE_DRM_START
        && (*pmu).type_ <= PERF_PMU_TYPE_DRM_END
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn evsel__is_drm(evsel: *const evsel) -> bool {
    perf_pmu__is_drm((*evsel).pmu)
}

unsafe fn add_drm_pmu(pmus: *mut list_head, line: *mut c_char, line_len: size_t) -> *mut drm_pmu {
    let mut drm: *mut drm_pmu;
    let mut pmu: *mut perf_pmu;
    let name: *const c_char;
    let mut max_drm_pmu_type: __u32 = 0;
    let type_: __u32;
    let mut i: c_int = 12;

    if *line.add(line_len - 1) == b'\n' as c_char {
        *line.add(line_len - 1) = 0;
    }
    while isspace(*line.add(i as usize) as c_int) != 0 {
        i += 1;
    }

    i -= 1;
    *line.add(i as usize) = b'_' as c_char;
    i -= 1;
    *line.add(i as usize) = b'm' as c_char;
    i -= 1;
    *line.add(i as usize) = b'r' as c_char;
    i -= 1;
    *line.add(i as usize) = b'd' as c_char;
    name = line.add(i as usize);

    pmu = (*pmus).next as *mut perf_pmu;
    while &mut (*pmu).list as *mut list_head != pmus {
        if !perf_pmu__is_drm(pmu) {
            pmu = (*(*pmu).list.next).next as *mut perf_pmu;
            continue;
        }
        if (*pmu).type_ > max_drm_pmu_type {
            max_drm_pmu_type = (*pmu).type_;
        }
        if strcmp((*pmu).name, name) == 0 {
            /* PMU already exists. */
            return ptr::null_mut();
        }
        pmu = (*(*pmu).list.next).next as *mut perf_pmu;
    }

    if max_drm_pmu_type != 0 {
        type_ = max_drm_pmu_type + 1;
    } else {
        type_ = PERF_PMU_TYPE_DRM_START;
    }

    if type_ > PERF_PMU_TYPE_DRM_END {
        drm = ptr::null_mut();
        free((&mut drm as *mut *mut drm_pmu).cast::<c_void>());
        pr_err(b"Unable to encode DRM PMU type for %s\n\0".as_ptr().cast(), name);
        return ptr::null_mut();
    }

    drm = zalloc(size_of::<drm_pmu>()).cast();
    if drm.is_null() {
        return ptr::null_mut();
    }

    if perf_pmu__init(&mut (*drm).pmu, type_, name) != 0 {
        perf_pmu__delete(&mut (*drm).pmu);
        return ptr::null_mut();
    }

    (*drm).pmu.cpus = perf_cpu_map__new_int(0);
    if (*drm).pmu.cpus.is_null() {
        perf_pmu__delete(&mut (*drm).pmu);
        return ptr::null_mut();
    }
    drm
}

unsafe fn add_event(
    events: *mut *mut drm_pmu_event,
    num_events: *mut c_int,
    line: *const c_char,
    unit: drm_pmu_unit,
    desc: *const c_char,
) -> c_int {
    let colon = strchr(line, b':' as c_int);
    let tmp: *mut drm_pmu_event;

    if colon.is_null() {
        return -EINVAL;
    }

    tmp = reallocarray(
        (*events).cast(),
        (*num_events + 1) as size_t,
        size_of::<drm_pmu_event>(),
    )
    .cast();
    if tmp.is_null() {
        return -ENOMEM;
    }
    (*tmp.add(*num_events as usize)).unit = unit;
    (*tmp.add(*num_events as usize)).desc = desc;
    (*tmp.add(*num_events as usize)).name = strndup(line, colon.offset_from(line) as size_t);
    if (*tmp.add(*num_events as usize)).name.is_null() {
        return -ENOMEM;
    }
    *num_events += 1;
    *events = tmp;
    0
}

unsafe extern "C" fn read_drm_pmus_cb(
    args: *mut c_void,
    fdinfo_dir_fd: c_int,
    fd_name: *const c_char,
) -> c_int {
    let pmus = args.cast::<list_head>();
    let mut buf = [0 as c_char; 640];
    let mut io: io = core::mem::zeroed();
    let mut line: *mut c_char = ptr::null_mut();
    let mut line_len: size_t = 0;
    let mut drm: *mut drm_pmu = ptr::null_mut();
    let mut events: *mut drm_pmu_event = ptr::null_mut();
    let mut num_events: c_int = 0;

    io__init(
        &mut io,
        openat(fdinfo_dir_fd, fd_name, O_RDONLY),
        buf.as_mut_ptr(),
        size_of::<[c_char; 640]>(),
    );
    if io.fd == -1 {
        /* Failed to open file, ignore. */
        return 0;
    }

    while io__getline(&mut io, &mut line, &mut line_len) > 0 {
        if strstarts(line, b"drm-driver:\0".as_ptr().cast()) {
            drm = add_drm_pmu(pmus, line, line_len);
            if drm.is_null() {
                break;
            }
            continue;
        }
        /*
         * Note the string matching below is alphabetical, with more
         * specific matches appearing before less specific.
         */
        if strstarts(line, b"drm-active-\0".as_ptr().cast()) {
            add_event(
                &mut events,
                &mut num_events,
                line,
                DRM_PMU_UNIT_BYTES,
                b"Total memory active in one or more engines\0".as_ptr().cast(),
            );
            continue;
        }
        if strstarts(line, b"drm-cycles-\0".as_ptr().cast()) {
            add_event(&mut events, &mut num_events, line, DRM_PMU_UNIT_CYCLES, b"Busy cycles\0".as_ptr().cast());
            continue;
        }
        if strstarts(line, b"drm-engine-capacity-\0".as_ptr().cast()) {
            add_event(&mut events, &mut num_events, line, DRM_PMU_UNIT_CAPACITY, b"Engine capacity\0".as_ptr().cast());
            continue;
        }
        if strstarts(line, b"drm-engine-\0".as_ptr().cast()) {
            add_event(&mut events, &mut num_events, line, DRM_PMU_UNIT_NS, b"Utilization in ns\0".as_ptr().cast());
            continue;
        }
        if strstarts(line, b"drm-maxfreq-\0".as_ptr().cast()) {
            add_event(&mut events, &mut num_events, line, DRM_PMU_UNIT_HZ, b"Maximum frequency\0".as_ptr().cast());
            continue;
        }
        if strstarts(line, b"drm-purgeable-\0".as_ptr().cast()) {
            add_event(&mut events, &mut num_events, line, DRM_PMU_UNIT_BYTES, b"Size of resident and purgeable memory buffers\0".as_ptr().cast());
            continue;
        }
        if strstarts(line, b"drm-resident-\0".as_ptr().cast()) {
            add_event(&mut events, &mut num_events, line, DRM_PMU_UNIT_BYTES, b"Size of resident memory buffers\0".as_ptr().cast());
            continue;
        }
        if strstarts(line, b"drm-shared-\0".as_ptr().cast()) {
            add_event(&mut events, &mut num_events, line, DRM_PMU_UNIT_BYTES, b"Size of shared memory buffers\0".as_ptr().cast());
            continue;
        }
        if strstarts(line, b"drm-total-cycles-\0".as_ptr().cast()) {
            add_event(&mut events, &mut num_events, line, DRM_PMU_UNIT_BYTES, b"Total busy cycles\0".as_ptr().cast());
            continue;
        }
        if strstarts(line, b"drm-total-\0".as_ptr().cast()) {
            add_event(&mut events, &mut num_events, line, DRM_PMU_UNIT_BYTES, b"Size of shared and private memory\0".as_ptr().cast());
            continue;
        }
        if verbose > 1
            && strstarts(line, b"drm-\0".as_ptr().cast())
            && !strstarts(line, b"drm-client-id:\0".as_ptr().cast())
            && !strstarts(line, b"drm-pdev:\0".as_ptr().cast())
        {
            pr_debug(b"Unhandled DRM PMU fdinfo line match '%s'\n\0".as_ptr().cast(), line);
        }
    }
    if !drm.is_null() {
        (*drm).events = events;
        (*drm).num_events = num_events;
        list_add_tail(&mut (*drm).pmu.list, pmus);
    }
    free(line.cast());
    if io.fd != -1 {
        close(io.fd);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn drm_pmu__exit(pmu: *mut perf_pmu) {
    let drm = container_of_perf_pmu(pmu);
    free((*drm).events.cast());
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn drm_pmu__have_event(pmu: *const perf_pmu, name: *const c_char) -> bool {
    let drm = container_of_perf_pmu(pmu);

    if !strstarts(name, b"drm-\0".as_ptr().cast()) {
        return false;
    }

    for i in 0..(*drm).num_events {
        if strcasecmp((*(*drm).events.add(i as usize)).name, name) == 0 {
            return true;
        }
    }
    false
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn drm_pmu__for_each_event(
    pmu: *const perf_pmu,
    state: *mut c_void,
    cb: pmu_event_callback,
) -> c_int {
    let drm = container_of_perf_pmu(pmu);

    for i in 0..(*drm).num_events {
        let mut encoding_buf = [0 as c_char; 128];
        let event = &*(*drm).events.add(i as usize);
        let mut info = pmu_event_info {
            pmu,
            name: event.name,
            alias: ptr::null(),
            scale_unit: drm_pmu_scale_unit_strs[event.unit as usize],
            desc: event.desc,
            long_desc: ptr::null(),
            encoding_desc: encoding_buf.as_mut_ptr(),
            topic: b"drm\0".as_ptr().cast(),
            pmu_name: (*pmu).name,
            event_type_desc: b"DRM event\0".as_ptr().cast(),
        };
        let ret: c_int;

        snprintf(
            encoding_buf.as_mut_ptr(),
            size_of::<[c_char; 128]>(),
            b"%s/config=0x%x/\0".as_ptr().cast(),
            (*pmu).name,
            i,
        );

        ret = cb.unwrap()(state, &mut info);
        if ret != 0 {
            return ret;
        }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn drm_pmu__num_events(pmu: *const perf_pmu) -> size_t {
    let drm = container_of_perf_pmu(pmu);
    (*drm).num_events as size_t
}

unsafe fn drm_pmu__index_for_event(drm: *const drm_pmu, name: *const c_char) -> c_int {
    for i in 0..(*drm).num_events {
        if strcmp((*(*drm).events.add(i as usize)).name, name) == 0 {
            return i;
        }
    }
    -1
}

unsafe fn drm_pmu__config_term(
    drm: *const drm_pmu,
    attr: *mut perf_event_attr,
    term: *mut parse_events_term,
    err: *mut parse_events_error,
) -> c_int {
    if (*term).type_term == PARSE_EVENTS__TERM_TYPE_USER {
        let i = drm_pmu__index_for_event(drm, (*term).config);

        if i >= 0 {
            (*attr).config = i as uint64_t;
            return 0;
        }
    }
    if !err.is_null() {
        let mut err_str: *mut c_char = ptr::null_mut();

        parse_events_error__handle(
            err,
            (*term).err_val,
            if asprintf(
                &mut err_str,
                b"unexpected drm event term (%s) %s\0".as_ptr().cast(),
                parse_events__term_type_str((*term).type_term),
                (*term).config,
            ) < 0
            {
                strdup(b"unexpected drm event term\0".as_ptr().cast())
            } else {
                err_str
            },
            ptr::null_mut(),
        );
    }
    -EINVAL
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn drm_pmu__config_terms(
    pmu: *const perf_pmu,
    attr: *mut perf_event_attr,
    terms: *mut parse_events_terms,
    err: *mut parse_events_error,
) -> c_int {
    let drm = container_of_perf_pmu(pmu);
    let mut pos = (*terms).terms.next as *mut parse_events_term;

    while &mut (*pos).list as *mut list_head != &mut (*terms).terms {
        if drm_pmu__config_term(drm, attr, pos, err) != 0 {
            return -EINVAL;
        }
        pos = (*(*pos).list.next).next as *mut parse_events_term;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn drm_pmu__check_alias(
    pmu: *const perf_pmu,
    terms: *mut parse_events_terms,
    info: *mut perf_pmu_info,
    err: *mut parse_events_error,
) -> c_int {
    let drm = container_of_perf_pmu(pmu);
    let term = (*terms).terms.next as *mut parse_events_term;

    if (*term).type_term == PARSE_EVENTS__TERM_TYPE_USER {
        let i = drm_pmu__index_for_event(drm, (*term).config);

        if i >= 0 {
            (*info).unit = drm_pmu_unit_strs[(*(*drm).events.add(i as usize)).unit as usize];
            (*info).scale = 1;
            return 0;
        }
    }
    if !err.is_null() {
        let mut err_str: *mut c_char = ptr::null_mut();

        parse_events_error__handle(
            err,
            (*term).err_val,
            if asprintf(
                &mut err_str,
                b"unexpected drm event term (%s) %s\0".as_ptr().cast(),
                parse_events__term_type_str((*term).type_term),
                (*term).config,
            ) < 0
            {
                strdup(b"unexpected drm event term\0".as_ptr().cast())
            } else {
                err_str
            },
            ptr::null_mut(),
        );
    }
    -EINVAL
}

unsafe fn for_each_drm_fdinfo_in_dir(
    cb: Option<unsafe extern "C" fn(*mut c_void, c_int, *const c_char) -> c_int>,
    args: *mut c_void,
    proc_dir: c_int,
    pid_name: *const c_char,
    minors: *mut minor_info,
) -> c_int {
    let mut buf = [0 as c_char; 256];
    let fd_dir: *mut DIR;
    let mut fd_entry: *mut dirent;
    let fd_dir_fd: c_int;
    let mut fdinfo_dir_fd: c_int = -1;

    scnprintf(buf.as_mut_ptr(), size_of::<[c_char; 256]>(), b"%s/fd\0".as_ptr().cast(), pid_name);
    fd_dir_fd = openat(proc_dir, buf.as_ptr(), O_DIRECTORY);
    if fd_dir_fd == -1 {
        return 0; /* Presumably lost race to open. */
    }
    fd_dir = fdopendir(fd_dir_fd);
    if fd_dir.is_null() {
        close(fd_dir_fd);
        return -ENOMEM;
    }
    loop {
        fd_entry = readdir(fd_dir);
        if fd_entry.is_null() {
            break;
        }
        let mut stat_buf: stat = core::mem::zeroed();
        let minor_: c_uint;
        let mut is_dup = false;
        let ret: c_int;

        if (*fd_entry).d_type != DT_LNK {
            continue;
        }

        if fstatat(fd_dir_fd, (*fd_entry).d_name.as_ptr(), &mut stat_buf, 0) != 0 {
            continue;
        }

        if (stat_buf.st_mode & S_IFMT) != S_IFCHR || major(stat_buf.st_rdev) != 226 {
            continue;
        }

        minor_ = minor(stat_buf.st_rdev);
        for i in 0..(*minors).minors_num {
            if minor(stat_buf.st_rdev) == *(*minors).minors.add(i as usize) {
                is_dup = true;
                break;
            }
        }
        if is_dup {
            continue;
        }

        if (*minors).minors_num == (*minors).minors_len {
            let tmp = reallocarray(
                (*minors).minors.cast(),
                ((*minors).minors_len + 4) as size_t,
                size_of::<c_uint>(),
            )
            .cast::<c_uint>();

            if !tmp.is_null() {
                (*minors).minors = tmp;
                (*minors).minors_len += 4;
            }
        }
        *(*minors).minors.add((*minors).minors_num as usize) = minor_;
        (*minors).minors_num += 1;
        if fdinfo_dir_fd == -1 {
            /* Open fdinfo dir if we have a DRM fd. */
            scnprintf(buf.as_mut_ptr(), size_of::<[c_char; 256]>(), b"%s/fdinfo\0".as_ptr().cast(), pid_name);
            fdinfo_dir_fd = openat(proc_dir, buf.as_ptr(), O_DIRECTORY);
            if fdinfo_dir_fd == -1 {
                continue;
            }
        }
        ret = cb.unwrap()(args, fdinfo_dir_fd, (*fd_entry).d_name.as_ptr());
        if ret != 0 {
            break;
        }
    }

    if fdinfo_dir_fd != -1 {
        close(fdinfo_dir_fd);
    }
    closedir(fd_dir);
    0
}

unsafe fn for_each_drm_fdinfo(
    skip_all_duplicates: bool,
    cb: Option<unsafe extern "C" fn(*mut c_void, c_int, *const c_char) -> c_int>,
    args: *mut c_void,
) -> c_int {
    let proc_dir: *mut DIR;
    let mut proc_entry: *mut dirent;
    let mut ret: c_int = 0;
    /*
     * minors maintains an array of DRM minor device numbers seen for a pid,
     * or for all pids if skip_all_duplicates is true, so that duplicates
     * are ignored.
     */
    let mut minors = minor_info {
        minors: ptr::null_mut(),
        minors_num: 0,
        minors_len: 0,
    };

    proc_dir = opendir(procfs__mountpoint());
    if proc_dir.is_null() {
        return 0;
    }

    /* Walk through the /proc directory. */
    loop {
        proc_entry = readdir(proc_dir);
        if proc_entry.is_null() {
            break;
        }
        if (*proc_entry).d_type != DT_DIR || isdigit((*proc_entry).d_name[0] as c_int) == 0 {
            continue;
        }
        if !skip_all_duplicates {
            /* Reset the seen minor numbers for each pid. */
            minors.minors_num = 0;
        }
        ret = for_each_drm_fdinfo_in_dir(
            cb,
            args,
            dirfd(proc_dir),
            (*proc_entry).d_name.as_ptr(),
            &mut minors,
        );
        if ret != 0 {
            break;
        }
    }
    free(minors.minors.cast());
    closedir(proc_dir);
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_pmus__read_drm_pmus(pmus: *mut list_head) -> c_int {
    for_each_drm_fdinfo(true, Some(read_drm_pmus_cb), pmus.cast())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn evsel__drm_pmu_open(
    _evsel: *mut evsel,
    _threads: *mut perf_thread_map,
    _start_cpu_map_idx: c_int,
    _end_cpu_map_idx: c_int,
) -> c_int {
    0
}

unsafe fn read_count_and_apply_unit(count_and_unit: *const c_char, unit: drm_pmu_unit) -> uint64_t {
    let mut unit_ptr: *mut c_char = ptr::null_mut();
    let mut count: uint64_t = strtoul(count_and_unit, &mut unit_ptr, 10) as uint64_t;

    if unit_ptr.is_null() {
        return 0;
    }

    while isblank(*unit_ptr as c_int) != 0 {
        unit_ptr = unit_ptr.add(1);
    }

    match unit {
        DRM_PMU_UNIT_BYTES => {
            if *unit_ptr == 0 {
                if count != 0 {
                    __assert_fail(
                        b"count == 0\0".as_ptr().cast(),
                        b"drm_pmu.c\0".as_ptr().cast(),
                        534,
                        b"read_count_and_apply_unit\0".as_ptr().cast(),
                    );
                }
            } else if strcmp(unit_ptr, b"KiB\0".as_ptr().cast()) == 0 {
                count = count.wrapping_mul(1024);
            } else if strcmp(unit_ptr, b"MiB\0".as_ptr().cast()) == 0 {
                count = count.wrapping_mul(1024 * 1024);
            } else {
                pr_err(b"Unexpected bytes unit '%s'\n\0".as_ptr().cast(), unit_ptr);
            }
        }
        DRM_PMU_UNIT_CAPACITY => {
            /* No units expected. */
        }
        DRM_PMU_UNIT_CYCLES => {
            /* No units expected. */
        }
        DRM_PMU_UNIT_HZ => {
            if strcmp(unit_ptr, b"Hz\0".as_ptr().cast()) == 0 {
                count = count.wrapping_mul(1);
            } else if strcmp(unit_ptr, b"KHz\0".as_ptr().cast()) == 0 {
                count = count.wrapping_mul(1000);
            } else if strcmp(unit_ptr, b"MHz\0".as_ptr().cast()) == 0 {
                count = count.wrapping_mul(1000000);
            } else {
                pr_err(b"Unexpected hz unit '%s'\n\0".as_ptr().cast(), unit_ptr);
            }
        }
        DRM_PMU_UNIT_NS => {
            /* Only unit ns expected. */
        }
        DRM_PMU_UNIT_MAX | _ => {}
    }
    count
}

unsafe fn read_drm_event(
    fdinfo_dir_fd: c_int,
    fd_name: *const c_char,
    match_: *const c_char,
    unit: drm_pmu_unit,
) -> uint64_t {
    let mut buf = [0 as c_char; 640];
    let mut io: io = core::mem::zeroed();
    let mut line: *mut c_char = ptr::null_mut();
    let mut line_len: size_t = 0;
    let mut count: uint64_t = 0;

    io__init(
        &mut io,
        openat(fdinfo_dir_fd, fd_name, O_RDONLY),
        buf.as_mut_ptr(),
        size_of::<[c_char; 640]>(),
    );
    if io.fd == -1 {
        /* Failed to open file, ignore. */
        return 0;
    }
    while io__getline(&mut io, &mut line, &mut line_len) > 0 {
        let mut i = strlen(match_);

        if strncmp(line, match_, i) != 0 {
            continue;
        }
        if *line.add(i) != b':' as c_char {
            continue;
        }
        loop {
            i += 1;
            if isblank(*line.add(i) as c_int) == 0 {
                break;
            }
        }
        if *line.add(line_len - 1) == b'\n' as c_char {
            *line.add(line_len - 1) = 0;
        }
        count = read_count_and_apply_unit(line.add(i), unit);
        break;
    }
    free(line.cast());
    close(io.fd);
    count
}

unsafe extern "C" fn read_drm_event_cb(
    vargs: *mut c_void,
    fdinfo_dir_fd: c_int,
    fd_name: *const c_char,
) -> c_int {
    let args = vargs.cast::<read_drm_event_cb_args>();

    (*args).count = (*args)
        .count
        .wrapping_add(read_drm_event(fdinfo_dir_fd, fd_name, (*args).match_, (*args).unit));
    0
}

unsafe fn drm_pmu__read_system_wide(drm: *mut drm_pmu, evsel: *mut evsel) -> uint64_t {
    let config = (*evsel).core.attr.config as usize;
    let mut args = read_drm_event_cb_args {
        count: 0,
        match_: (*(*drm).events.add(config)).name,
        unit: (*(*drm).events.add(config)).unit,
    };

    for_each_drm_fdinfo(false, Some(read_drm_event_cb), (&mut args as *mut read_drm_event_cb_args).cast());
    args.count
}

unsafe fn drm_pmu__read_for_pid(drm: *mut drm_pmu, evsel: *mut evsel, pid: c_int) -> uint64_t {
    let config = (*evsel).core.attr.config as usize;
    let mut args = read_drm_event_cb_args {
        count: 0,
        match_: (*(*drm).events.add(config)).name,
        unit: (*(*drm).events.add(config)).unit,
    };
    let mut minors = minor_info {
        minors: ptr::null_mut(),
        minors_num: 0,
        minors_len: 0,
    };
    let proc_dir = open(procfs__mountpoint(), O_DIRECTORY);
    let mut pid_name = [0 as c_char; 12];
    let ret: c_int;

    if proc_dir < 0 {
        return 0;
    }

    snprintf(pid_name.as_mut_ptr(), size_of::<[c_char; 12]>(), b"%d\0".as_ptr().cast(), pid);
    ret = for_each_drm_fdinfo_in_dir(
        Some(read_drm_event_cb),
        (&mut args as *mut read_drm_event_cb_args).cast(),
        proc_dir,
        pid_name.as_ptr(),
        &mut minors,
    );
    free(minors.minors.cast());
    close(proc_dir);
    if ret == 0 { args.count } else { 0 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn evsel__drm_pmu_read(
    evsel: *mut evsel,
    cpu_map_idx: c_int,
    thread: c_int,
) -> c_int {
    let drm = container_of_perf_pmu((*evsel).pmu);
    let mut old_count: *mut perf_counts_values = ptr::null_mut();
    let count: *mut perf_counts_values;
    let pid = perf_thread_map__pid((*evsel).core.threads, thread);
    let counter: uint64_t;

    if pid != -1 {
        counter = drm_pmu__read_for_pid(drm, evsel, pid);
    } else {
        counter = drm_pmu__read_system_wide(drm, evsel);
    }

    if !(*evsel).prev_raw_counts.is_null() {
        old_count = perf_counts((*evsel).prev_raw_counts, cpu_map_idx, thread);
    }

    count = perf_counts((*evsel).counts, cpu_map_idx, thread);
    if !old_count.is_null() {
        (*count).val = (*old_count).val.wrapping_add(counter);
        (*count).run = (*old_count).run.wrapping_add(1);
        (*count).ena = (*old_count).ena.wrapping_add(1);
    } else {
        (*count).val = counter;
        (*count).run = (*count).run.wrapping_add(1);
        (*count).ena = (*count).ena.wrapping_add(1);
    }
    0
}
