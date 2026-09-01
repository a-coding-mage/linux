// SPDX-License-Identifier: GPL-2.0
/*
 * Support for libpfm4 event encoding.
 *
 * Copyright 2020 Google LLC.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

// Dependencies originally included from:
// "util/cpumap.h", "util/debug.h", "util/event.h", "util/evlist.h",
// "util/evsel.h", "util/parse-events.h", "util/pmus.h", "util/pfm.h",
// "util/strbuf.h", "util/thread_map.h", <errno.h>, <string.h>,
// <linux/kernel.h>, and <perfmon/pfmlib_perf_event.h>.

#[repr(C)]
pub struct option {
    pub value: *mut c_void,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_pmu {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_thread_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub exclude_kernel: u64,
}

#[repr(C)]
pub struct evsel_core {
    pub nr_members: c_int,
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
    pub is_libpfm_event: bool,
}

#[repr(C)]
pub struct strbuf {
    pub buf: *mut c_char,
}

#[repr(C)]
pub struct print_callbacks {
    pub print_event: Option<
        unsafe extern "C" fn(
            print_state: *mut c_void,
            topic: *const c_char,
            pmu_name: *const c_char,
            pmu_type: u32,
            event_name: *const c_char,
            alias: *const c_char,
            scale_unit: *const c_char,
            deprecated: *const c_char,
            event_type: *const c_char,
            desc: *const c_char,
            long_desc: *const c_char,
            encoding_desc: *const c_char,
        ),
    >,
}

#[repr(C)]
pub struct pfm_event_attr_info_t {
    pub size: usize,
    pub is_dfl: bool,
    pub is_precise: bool,
    pub ctrl: c_int,
    pub type_: c_int,
    pub name: *const c_char,
    pub desc: *const c_char,
    pub code: u64,
}

#[repr(C)]
pub struct pfm_event_info_t {
    pub size: usize,
    pub idx: c_int,
    pub name: *const c_char,
    pub equiv: *const c_char,
    pub desc: *const c_char,
    pub code: u64,
    pub nattrs: c_int,
}

#[repr(C)]
pub struct pfm_pmu_info_t {
    pub size: usize,
    pub name: *const c_char,
    pub is_present: bool,
    pub pmu: c_int,
    pub first_event: c_int,
}

unsafe extern "C" {
    static PFM_ATTR_CTRL_MAX: c_int;
    static PFM_ATTR_CTRL_UNKNOWN: c_int;
    static PFM_ATTR_CTRL_PMU: c_int;
    static PFM_ATTR_CTRL_PERF_EVENT: c_int;
    static PFM_ATTR_UMASK: c_int;
    static PFM_ATTR_MOD_BOOL: c_int;
    static PFM_ATTR_MOD_INTEGER: c_int;
    static PFM_ATTR_NONE: c_int;
    static PFM_ATTR_RAW_UMASK: c_int;
    static PFM_ATTR_MAX: c_int;
    static PFM_OS_PERF_EVENT_EXT: c_int;
    static PFM_PLM0: c_int;
    static PFM_PLM3: c_int;
    static PFM_PMU_PERF_EVENT: c_int;
    static PFM_SUCCESS: c_int;
    static PERF_TYPE_RAW: u32;

    fn event_attr_init(attr: *mut perf_event_attr);
    fn evlist__add(evlist: *mut evlist, evsel: *mut evsel);
    fn evlist__nr_entries(evlist: *mut evlist) -> c_int;
    fn evsel__close(evsel: *mut evsel);
    fn evsel__open(
        evsel: *mut evsel,
        cpus: *mut perf_cpu_map,
        threads: *mut perf_thread_map,
    ) -> c_int;
    fn evsel__put(evsel: *mut evsel);
    fn evsel__set_leader(evsel: *mut evsel, leader: *mut evsel);
    fn free(ptr: *mut c_void);
    fn perf_cpu_map__empty_new(nr: c_int) -> *mut perf_cpu_map;
    fn perf_cpu_map__put(cpus: *mut perf_cpu_map);
    fn perf_pmus__find_by_type(type_: c_uint) -> *mut perf_pmu;
    fn perf_thread_map__put(threads: *mut perf_thread_map);
    fn pfm_get_event_attr_info(
        idx: c_int,
        attr: c_int,
        os: c_int,
        info: *mut pfm_event_attr_info_t,
    ) -> c_int;
    fn pfm_get_event_info(idx: c_int, os: c_int, info: *mut pfm_event_info_t) -> c_int;
    fn pfm_get_event_next(idx: c_int) -> c_int;
    fn pfm_get_perf_event_encoding(
        event: *const c_char,
        plm: c_int,
        attr: *mut perf_event_attr,
        fstr: *mut c_void,
        idx: *mut c_void,
    ) -> c_int;
    fn pfm_get_pmu_info(pmu: c_int, info: *mut pfm_pmu_info_t) -> c_int;
    fn pfm_initialize() -> c_int;
    fn pfm_strerror(code: c_int) -> *const c_char;
    fn parse_events__add_event(
        idx: c_int,
        attr: *mut perf_event_attr,
        name: *const c_char,
        metric_id: *const c_char,
        pmu: *mut perf_pmu,
    ) -> *mut evsel;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strbuf_addf(buf: *mut strbuf, fmt: *const c_char, ...);
    fn strbuf_init(buf: *mut strbuf, hint: usize);
    fn strbuf_release(buf: *mut strbuf);
    fn strbuf_setlen(buf: *mut strbuf, len: usize);
    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn thread_map__new_by_tid(tid: c_int) -> *mut perf_thread_map;
    fn ui__error(fmt: *const c_char, ...);
    fn ui__warning(fmt: *const c_char, ...);
}

const EACCES: c_int = 13;

static mut SRCS: [*const c_char; 3] = [
    c"???".as_ptr(),
    c"PMU".as_ptr(),
    c"perf_event".as_ptr(),
];

unsafe fn libpfm_initialize() {
    let ret: c_int;

    ret = pfm_initialize();
    if ret != PFM_SUCCESS {
        ui__warning(
            c"libpfm failed to initialize: %s\n".as_ptr(),
            pfm_strerror(ret),
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_libpfm_events_option(
    opt: *const option,
    str_: *const c_char,
    _unset: c_int,
) -> c_int {
    let evlist = *((*opt).value as *mut *mut evlist);
    let mut attr: perf_event_attr = mem::zeroed();
    let mut pmu: *mut perf_pmu;
    let mut evsel: *mut evsel;
    let mut grp_leader: *mut evsel = ptr::null_mut();
    let mut p: *mut c_char;
    let mut q: *mut c_char;
    let mut p_orig: *mut c_char;
    let mut sep: *const c_char;
    let mut grp_evt: c_int = -1;
    let mut ret: c_int;

    libpfm_initialize();

    p_orig = strdup(str_);
    p = p_orig;
    if p.is_null() {
        return -1;
    }

    q = p;
    while !strsep(&mut p, c",{}".as_ptr()).is_null() {
        sep = if !p.is_null() {
            str_.add(p.offset_from(p_orig) as usize - 1)
        } else {
            c"".as_ptr()
        };
        if *sep == b'{' as c_char {
            if grp_evt > -1 {
                ui__error(c"nested event groups not supported\n".as_ptr());
                goto_error(p_orig);
                return -1;
            }
            grp_evt += 1;
        }

        /* no event */
        if *q == b'\0' as c_char {
            if *sep == b'}' as c_char {
                if grp_evt < 0 {
                    ui__error(c"cannot close a non-existing event group\n".as_ptr());
                    goto_error(p_orig);
                    return -1;
                }
                grp_evt -= 1;
            }
            q = p;
            continue;
        }

        ptr::write_bytes(&mut attr as *mut perf_event_attr, 0, 1);
        event_attr_init(&mut attr);

        ret = pfm_get_perf_event_encoding(
            q,
            PFM_PLM0 | PFM_PLM3,
            &mut attr,
            ptr::null_mut(),
            ptr::null_mut(),
        );

        if ret != PFM_SUCCESS {
            ui__error(
                c"failed to parse event %s : %s\n".as_ptr(),
                str_,
                pfm_strerror(ret),
            );
            goto_error(p_orig);
            return -1;
        }

        pmu = perf_pmus__find_by_type(attr.type_ as c_uint);
        evsel = parse_events__add_event(
            evlist__nr_entries(evlist),
            &mut attr,
            q,
            ptr::null(),
            pmu,
        );
        if evsel.is_null() {
            goto_error(p_orig);
            return -1;
        }

        (*evsel).is_libpfm_event = true;

        evlist__add(evlist, evsel);

        if grp_evt == 0 {
            grp_leader = evsel;
        }

        if grp_evt > -1 {
            evsel__set_leader(evsel, grp_leader);
            (*grp_leader).core.nr_members += 1;
            grp_evt += 1;
        }

        if *sep == b'}' as c_char {
            if grp_evt < 0 {
                ui__error(c"cannot close a non-existing event group\n".as_ptr());
                goto_error(p_orig);
                return -1;
            }
            grp_leader = ptr::null_mut();
            grp_evt = -1;
        }

        q = p;
    }
    free(p_orig as *mut c_void);
    0
}

unsafe fn goto_error(p_orig: *mut c_char) {
    free(p_orig as *mut c_void);
}

unsafe fn is_libpfm_event_supported(
    name: *const c_char,
    cpus: *mut perf_cpu_map,
    threads: *mut perf_thread_map,
) -> bool {
    let mut pmu: *mut perf_pmu;
    let mut evsel: *mut evsel;
    let mut attr: perf_event_attr = mem::zeroed();
    let mut result = true;
    let mut ret: c_int;

    ret = pfm_get_perf_event_encoding(
        name,
        PFM_PLM0 | PFM_PLM3,
        &mut attr,
        ptr::null_mut(),
        ptr::null_mut(),
    );
    if ret != PFM_SUCCESS {
        return false;
    }

    pmu = perf_pmus__find_by_type(attr.type_ as c_uint);
    evsel = parse_events__add_event(0, &mut attr, name, ptr::null(), pmu);
    if evsel.is_null() {
        return false;
    }

    (*evsel).is_libpfm_event = true;

    ret = evsel__open(evsel, cpus, threads);
    if ret == -EACCES {
        /*
         * This happens if the paranoid value
         * /proc/sys/kernel/perf_event_paranoid is set to 2
         * Re-run with exclude_kernel set; we don't do that
         * by default as some ARM machines do not support it.
         *
         */
        (*evsel).core.attr.exclude_kernel = 1;
        ret = evsel__open(evsel, cpus, threads);
    }
    if ret < 0 {
        result = false;
    }

    evsel__close(evsel);
    evsel__put(evsel);

    result
}

unsafe fn src_for_ctrl(ctrl: c_int) -> *const c_char {
    if ctrl == PFM_ATTR_CTRL_PMU {
        SRCS[1]
    } else if ctrl == PFM_ATTR_CTRL_PERF_EVENT {
        SRCS[2]
    } else {
        SRCS[0]
    }
}

unsafe fn print_attr_flags(buf: *mut strbuf, info: *const pfm_event_attr_info_t) {
    if (*info).is_dfl {
        strbuf_addf(buf, c"[default] ".as_ptr());
    }

    if (*info).is_precise {
        strbuf_addf(buf, c"[precise] ".as_ptr());
    }
}

unsafe fn print_libpfm_event(
    print_cb: *const print_callbacks,
    print_state: *mut c_void,
    pinfo: *const pfm_pmu_info_t,
    info: *const pfm_event_info_t,
    buf: *mut strbuf,
) {
    let mut j: c_int;
    let mut ret: c_int;
    let mut topic = [0 as c_char; 80];
    let mut name = [0 as c_char; 80];
    let cpus = perf_cpu_map__empty_new(1);
    let threads = thread_map__new_by_tid(0);

    strbuf_setlen(buf, 0);
    snprintf(
        topic.as_mut_ptr(),
        topic.len(),
        c"pfm %s".as_ptr(),
        (*pinfo).name,
    );

    snprintf(
        name.as_mut_ptr(),
        name.len(),
        c"%s::%s".as_ptr(),
        (*pinfo).name,
        (*info).name,
    );
    strbuf_addf(buf, c"Code: 0x%lx\n".as_ptr(), (*info).code);

    j = 0;
    while j < (*info).nattrs {
        let mut ainfo: pfm_event_attr_info_t = mem::zeroed();
        let src: *const c_char;

        ainfo.size = mem::size_of::<pfm_event_attr_info_t>();
        ret = pfm_get_event_attr_info((*info).idx, j, PFM_OS_PERF_EVENT_EXT, &mut ainfo);
        if ret != PFM_SUCCESS {
            j += 1;
            continue;
        }

        if ainfo.ctrl >= PFM_ATTR_CTRL_MAX {
            ainfo.ctrl = PFM_ATTR_CTRL_UNKNOWN;
        }

        src = src_for_ctrl(ainfo.ctrl);
        if ainfo.type_ == PFM_ATTR_UMASK {
            /* Ignore for now */
        } else if ainfo.type_ == PFM_ATTR_MOD_BOOL {
            strbuf_addf(
                buf,
                c" Modif: %s: [%s] : %s (boolean)\n".as_ptr(),
                src,
                ainfo.name,
                ainfo.desc,
            );
        } else if ainfo.type_ == PFM_ATTR_MOD_INTEGER {
            strbuf_addf(
                buf,
                c" Modif: %s: [%s] : %s (integer)\n".as_ptr(),
                src,
                ainfo.name,
                ainfo.desc,
            );
        } else if ainfo.type_ == PFM_ATTR_NONE
            || ainfo.type_ == PFM_ATTR_RAW_UMASK
            || ainfo.type_ == PFM_ATTR_MAX
            || true
        {
            strbuf_addf(
                buf,
                c" Attr: %s: [%s] : %s\n".as_ptr(),
                src,
                ainfo.name,
                ainfo.desc,
            );
        }
        j += 1;
    }

    if is_libpfm_event_supported(name.as_ptr(), cpus, threads) {
        if let Some(print_event) = (*print_cb).print_event {
            print_event(
                print_state,
                topic.as_ptr(),
                (*pinfo).name,
                PERF_TYPE_RAW,
                name.as_ptr(),
                (*info).equiv,
                ptr::null(),
                ptr::null(),
                c"PFM event".as_ptr(),
                (*info).desc,
                ptr::null(),
                (*buf).buf,
            );
        }
    }

    j = 0;
    while j < (*info).nattrs {
        let mut ainfo: pfm_event_attr_info_t = mem::zeroed();
        let src: *const c_char;

        strbuf_setlen(buf, 0);

        ainfo.size = mem::size_of::<pfm_event_attr_info_t>();
        ret = pfm_get_event_attr_info((*info).idx, j, PFM_OS_PERF_EVENT_EXT, &mut ainfo);
        if ret != PFM_SUCCESS {
            j += 1;
            continue;
        }

        if ainfo.ctrl >= PFM_ATTR_CTRL_MAX {
            ainfo.ctrl = PFM_ATTR_CTRL_UNKNOWN;
        }

        src = src_for_ctrl(ainfo.ctrl);
        if ainfo.type_ == PFM_ATTR_UMASK {
            strbuf_addf(
                buf,
                c"Umask: 0x%02lx : %s: ".as_ptr(),
                ainfo.code,
                src,
            );
            print_attr_flags(buf, &ainfo);
            snprintf(
                name.as_mut_ptr(),
                name.len(),
                c"%s::%s:%s".as_ptr(),
                (*pinfo).name,
                (*info).name,
                ainfo.name,
            );

            if !is_libpfm_event_supported(name.as_ptr(), cpus, threads) {
                j += 1;
                continue;
            }

            if let Some(print_event) = (*print_cb).print_event {
                print_event(
                    print_state,
                    topic.as_ptr(),
                    (*pinfo).name,
                    PERF_TYPE_RAW,
                    name.as_ptr(),
                    ptr::null(),
                    ptr::null(),
                    ptr::null(),
                    c"PFM event".as_ptr(),
                    ainfo.desc,
                    ptr::null(),
                    (*buf).buf,
                );
            }
        }
        j += 1;
    }

    perf_cpu_map__put(cpus);
    perf_thread_map__put(threads);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn print_libpfm_events(
    print_cb: *const print_callbacks,
    print_state: *mut c_void,
) {
    let mut info: pfm_event_info_t = mem::zeroed();
    let mut pinfo: pfm_pmu_info_t = mem::zeroed();
    let mut p: c_int;
    let mut ret: c_int;
    let mut storage: strbuf = mem::zeroed();

    libpfm_initialize();

    /* initialize to zero to indicate ABI version */
    info.size = mem::size_of::<pfm_event_info_t>();
    pinfo.size = mem::size_of::<pfm_pmu_info_t>();

    strbuf_init(&mut storage, 2048);

    // pfm_for_all_pmus(p)
    p = 0;
    while p < PFM_ATTR_CTRL_MAX {
        ret = pfm_get_pmu_info(p, &mut pinfo);
        if ret != PFM_SUCCESS {
            p += 1;
            continue;
        }

        /* only print events that are supported by host HW */
        if !pinfo.is_present {
            p += 1;
            continue;
        }

        /* handled by perf directly */
        if pinfo.pmu == PFM_PMU_PERF_EVENT {
            p += 1;
            continue;
        }

        let mut i = pinfo.first_event;
        while i != -1 {
            ret = pfm_get_event_info(i, PFM_OS_PERF_EVENT_EXT, &mut info);
            if ret == PFM_SUCCESS {
                print_libpfm_event(print_cb, print_state, &pinfo, &info, &mut storage);
            }

            i = pfm_get_event_next(i);
        }

        p += 1;
    }
    strbuf_release(&mut storage);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
