// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/util/print-events.c.
// C include dependencies are represented here as external declarations and
// opaque C-compatible types supplied by the surrounding translated repository.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

type bool_ = bool;
type u8 = u8;
type u64 = u64;

const MAX_NAME_LEN: usize = 100;
const ENOMEM: c_int = 12;

const PERF_TYPE_HARDWARE: u32 = 0;
const PERF_TYPE_SOFTWARE: u32 = 1;
const PERF_TYPE_TRACEPOINT: u32 = 2;
const PERF_TYPE_HW_CACHE: u32 = 3;
const PERF_TYPE_RAW: u32 = 4;
const PERF_TYPE_BREAKPOINT: u32 = 5;

#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_root_cached {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rblist {
    pub entries: rb_root_cached,
    pub node_cmp: Option<unsafe extern "C" fn(*mut rb_node, *const c_void) -> c_int>,
    pub node_new: Option<unsafe extern "C" fn(*mut rblist, *const c_void) -> *mut rb_node>,
    pub node_delete: Option<unsafe extern "C" fn(*mut rblist, *mut rb_node)>,
}

#[repr(C)]
pub struct strlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct str_node {
    pub nd: rb_node,
    pub s: *mut c_char,
}

#[repr(C)]
pub struct probe_trace_event {
    pub event: *const c_char,
    pub group: *const c_char,
}

#[repr(C)]
pub struct probe_cache_entry {
    pub node: list_head,
    pub pev: probe_trace_event,
}

#[repr(C)]
pub struct probe_cache {
    pub entries: list_head,
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub config: u64,
    pub disabled: u64,
    pub exclude_kernel: u64,
    pub exclude_guest: u64,
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
}

#[repr(C)]
pub struct perf_thread_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pmu_metric {
    pub metric_group: *const c_char,
    pub metric_name: *const c_char,
    pub desc: *const c_char,
    pub long_desc: *const c_char,
    pub metric_expr: *const c_char,
    pub metric_threshold: *const c_char,
    pub unit: *const c_char,
    pub pmu: *const c_char,
}

#[repr(C)]
pub struct pmu_metrics_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct print_callbacks {
    pub print_event: unsafe extern "C" fn(
        *mut c_void,
        *const c_char,
        *const c_char,
        u32,
        *const c_char,
        *const c_char,
        usize,
        usize,
        *const c_char,
        *const c_char,
        *const c_char,
        *const c_char,
    ),
    pub print_metric: unsafe extern "C" fn(
        *mut c_void,
        *const c_char,
        *const c_char,
        *const c_char,
        *const c_char,
        *const c_char,
        *const c_char,
        *const c_char,
        *const c_char,
    ),
}

/** Strings corresponding to enum perf_type_id. */
static event_type_descriptors: [*const c_char; 6] = [
    b"Hardware event\0".as_ptr() as *const c_char,
    b"Software event\0".as_ptr() as *const c_char,
    b"Tracepoint event\0".as_ptr() as *const c_char,
    b"Hardware cache event\0".as_ptr() as *const c_char,
    b"Raw event descriptor\0".as_ptr() as *const c_char,
    b"Hardware breakpoint\0".as_ptr() as *const c_char,
];

extern "C" {
    static mut errno: c_int;

    fn pr_debug(fmt: *const c_char, ...);
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strchrnul(s: *const c_char, c: c_int) -> *mut c_char;
    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn skip_spaces(str_: *const c_char) -> *const c_char;

    fn strlist__new(a: *const c_void, b: *const c_void) -> *mut strlist;
    fn strlist__add(slist: *mut strlist, s: *const c_char) -> c_int;
    fn strlist__delete(slist: *mut strlist);
    fn strlist__next(node: *mut str_node) -> *mut str_node;

    fn build_id_cache__list_all(validonly: bool_) -> *mut strlist;
    fn build_id_cache__origname(sbuild_id: *const c_char) -> *mut c_char;
    fn probe_cache__new(target: *const c_char, nsi: *const c_void) -> *mut probe_cache;
    fn probe_cache__delete(pcache: *mut probe_cache);

    fn thread_map__new_by_tid(tid: c_int) -> *mut perf_thread_map;
    fn perf_thread_map__put(map: *mut perf_thread_map);
    fn evsel__new(attr: *mut perf_event_attr) -> *mut evsel;
    fn evsel__open(evsel: *mut evsel, cpus: *const c_void, threads: *mut perf_thread_map) -> c_int;
    fn evsel__close(evsel: *mut evsel);
    fn evsel__put(evsel: *mut evsel);

    fn rblist__init(rl: *mut rblist);
    fn rblist__find(rl: *mut rblist, entry: *const c_void) -> *mut rb_node;
    fn rblist__add_node(rl: *mut rblist, entry: *const c_void) -> c_int;
    fn rblist__remove_node(rl: *mut rblist, rb_node: *mut rb_node);
    fn rb_first_cached(root: *mut rb_root_cached) -> *mut rb_node;
    fn rb_next(node: *mut rb_node) -> *mut rb_node;

    fn pmu_metrics_table__find() -> *const pmu_metrics_table;
    fn metricgroup__for_each_metric(
        table: *const pmu_metrics_table,
        cb: unsafe extern "C" fn(*const pmu_metric, *const pmu_metrics_table, *mut c_void) -> c_int,
        data: *mut c_void,
    ) -> c_int;

    fn perf_pmus__print_pmu_events(print_cb: *const print_callbacks, print_state: *mut c_void);
    fn perf_pmus__print_raw_pmu_events(print_cb: *const print_callbacks, print_state: *mut c_void);
    fn print_libpfm_events(print_cb: *const print_callbacks, print_state: *mut c_void);

    /* External iteration helpers corresponding to strlist__for_each_entry and list_for_each_entry. */
    fn strlist__first(slist: *mut strlist) -> *mut str_node;
    fn probe_cache_entry__first(head: *mut list_head) -> *mut probe_cache_entry;
    fn probe_cache_entry__next(ent: *mut probe_cache_entry, head: *mut list_head) -> *mut probe_cache_entry;
}

unsafe fn zfree_char(pptr: *mut *mut c_char) {
    if !(*pptr).is_null() {
        free(*pptr as *mut c_void);
        *pptr = ptr::null_mut();
    }
}

pub unsafe extern "C" fn print_sdt_events(print_cb: *const print_callbacks, print_state: *mut c_void) {
    let mut bidlist: *mut strlist;
    let mut sdtlist: *mut strlist;
    let mut bid_nd: *mut str_node;
    let mut sdt_name: *mut str_node;
    let mut next_sdt_name: *mut str_node;
    let mut last_sdt_name: *const c_char = ptr::null();

    /*
     * The implicitly sorted sdtlist will hold the tracepoint name followed
     * by @<buildid>. If the tracepoint name is unique (determined by
     * looking at the adjacent nodes) the @<buildid> is dropped otherwise
     * the executable path and buildid are added to the name.
     */
    sdtlist = strlist__new(ptr::null(), ptr::null());
    if sdtlist.is_null() {
        pr_debug(b"Failed to allocate new strlist for SDT\n\0".as_ptr() as *const c_char);
        return;
    }
    bidlist = build_id_cache__list_all(true);
    if bidlist.is_null() {
        pr_debug(b"Failed to get buildids: %d\n\0".as_ptr() as *const c_char, errno);
        return;
    }
    bid_nd = strlist__first(bidlist);
    while !bid_nd.is_null() {
        let mut pcache: *mut probe_cache;
        let mut ent: *mut probe_cache_entry;

        pcache = probe_cache__new((*bid_nd).s, ptr::null());
        if !pcache.is_null() {
            ent = probe_cache_entry__first(&mut (*pcache).entries);
            while !ent.is_null() {
                let mut buf = [0 as c_char; 1024];

                snprintf(
                    buf.as_mut_ptr(),
                    buf.len(),
                    b"%s:%s@%s\0".as_ptr() as *const c_char,
                    (*ent).pev.group,
                    (*ent).pev.event,
                    (*bid_nd).s,
                );
                strlist__add(sdtlist, buf.as_ptr());
                ent = probe_cache_entry__next(ent, &mut (*pcache).entries);
            }
            probe_cache__delete(pcache);
        }
        bid_nd = strlist__next(bid_nd);
    }
    strlist__delete(bidlist);

    sdt_name = strlist__first(sdtlist);
    while !sdt_name.is_null() {
        let mut show_detail = false;
        let mut bid = strchr((*sdt_name).s, b'@' as c_int);
        let mut evt_name: *mut c_char = ptr::null_mut();

        if !bid.is_null() {
            *bid = 0;
            bid = bid.add(1);
        }

        if !last_sdt_name.is_null() && strcmp(last_sdt_name, (*sdt_name).s) == 0 {
            show_detail = true;
        } else {
            next_sdt_name = strlist__next(sdt_name);
            if !next_sdt_name.is_null() {
                let bid2 = strchrnul((*next_sdt_name).s, b'@' as c_int);

                show_detail = strncmp(
                    (*sdt_name).s,
                    (*next_sdt_name).s,
                    bid2.offset_from((*next_sdt_name).s) as usize,
                ) == 0;
            }
        }
        last_sdt_name = (*sdt_name).s;

        if show_detail {
            let path = build_id_cache__origname(bid);

            if !path.is_null() {
                if asprintf(
                    &mut evt_name,
                    b"%s@%s(%.12s)\0".as_ptr() as *const c_char,
                    (*sdt_name).s,
                    path,
                    bid,
                ) < 0
                {
                    evt_name = ptr::null_mut();
                }
                free(path as *mut c_void);
            }
        }
        ((*print_cb).print_event)(
            print_state,
            ptr::null(),
            ptr::null(),
            PERF_TYPE_TRACEPOINT,
            if !evt_name.is_null() { evt_name } else { (*sdt_name).s },
            ptr::null(),
            false as usize,
            ptr::null::<c_char>() as usize,
            b"SDT event\0".as_ptr() as *const c_char,
            ptr::null(),
            ptr::null(),
            ptr::null(),
        );

        free(evt_name as *mut c_void);
        sdt_name = strlist__next(sdt_name);
    }
    strlist__delete(sdtlist);
}

pub unsafe extern "C" fn is_event_supported(type_: u8, config: u64) -> bool_ {
    let mut ret = true;
    let mut attr = perf_event_attr {
        type_: type_ as u32,
        config,
        disabled: 1,
        exclude_kernel: 0,
        exclude_guest: 0,
    };
    let tmap = thread_map__new_by_tid(0);

    if tmap.is_null() {
        return false;
    }

    let evsel = evsel__new(&mut attr);
    if !evsel.is_null() {
        ret = evsel__open(evsel, ptr::null(), tmap) >= 0;

        if !ret {
            /*
             * The event may fail to open if the paranoid value
             * /proc/sys/kernel/perf_event_paranoid is set to 2
             * Re-run with exclude_kernel set; we don't do that by
             * default as some ARM machines do not support it.
             */
            (*evsel).core.attr.exclude_kernel = 1;
            ret = evsel__open(evsel, ptr::null(), tmap) >= 0;
        }

        if !ret {
            /*
             * The event may fail to open if the PMU requires
             * exclude_guest to be set (e.g. as the Apple M1 PMU
             * requires).
             * Re-run with exclude_guest set; we don't do that by
             * default as it's equally legitimate for another PMU
             * driver to require that exclude_guest is clear.
             */
            (*evsel).core.attr.exclude_guest = 1;
            ret = evsel__open(evsel, ptr::null(), tmap) >= 0;
        }

        evsel__close(evsel);
        evsel__put(evsel);
    }

    perf_thread_map__put(tmap);
    ret
}

/** struct mep - RB-tree node for building printing information. */
#[repr(C)]
pub struct mep {
    /** nd - RB-tree element. */
    pub nd: rb_node,
    /** @metric_group: Owned metric group name, separated others with ';'. */
    pub metric_group: *mut c_char,
    pub metric_name: *const c_char,
    pub metric_desc: *const c_char,
    pub metric_long_desc: *const c_char,
    pub metric_expr: *const c_char,
    pub metric_threshold: *const c_char,
    pub metric_unit: *const c_char,
    pub pmu_name: *const c_char,
}

unsafe fn mep_from_rb_node(rb_node: *mut rb_node) -> *mut mep {
    rb_node as *mut mep
}

unsafe extern "C" fn mep_cmp(rb_node: *mut rb_node, entry: *const c_void) -> c_int {
    let a = mep_from_rb_node(rb_node);
    let b = entry as *mut mep;
    let ret: c_int;

    ret = strcmp((*a).metric_group, (*b).metric_group);
    if ret != 0 {
        return ret;
    }

    strcmp((*a).metric_name, (*b).metric_name)
}

unsafe extern "C" fn mep_new(_rl: *mut rblist, entry: *const c_void) -> *mut rb_node {
    let me = malloc(mem::size_of::<mep>()) as *mut mep;

    if me.is_null() {
        return ptr::null_mut();
    }

    memcpy(me as *mut c_void, entry, mem::size_of::<mep>());
    &mut (*me).nd
}

unsafe extern "C" fn mep_delete(_rl: *mut rblist, nd: *mut rb_node) {
    let me = mep_from_rb_node(nd);

    zfree_char(&mut (*me).metric_group);
    free(me as *mut c_void);
}

unsafe fn mep_lookup(groups: *mut rblist, metric_group: *const c_char, metric_name: *const c_char) -> *mut mep {
    let mut nd: *mut rb_node;
    let mut me = mep {
        nd: mem::zeroed(),
        metric_group: strdup(metric_group),
        metric_name,
        metric_desc: ptr::null(),
        metric_long_desc: ptr::null(),
        metric_expr: ptr::null(),
        metric_threshold: ptr::null(),
        metric_unit: ptr::null(),
        pmu_name: ptr::null(),
    };

    nd = rblist__find(groups, &mut me as *mut mep as *const c_void);
    if !nd.is_null() {
        free(me.metric_group as *mut c_void);
        return mep_from_rb_node(nd);
    }
    rblist__add_node(groups, &mut me as *mut mep as *const c_void);
    nd = rblist__find(groups, &mut me as *mut mep as *const c_void);
    if !nd.is_null() {
        return mep_from_rb_node(nd);
    }
    ptr::null_mut()
}

unsafe extern "C" fn metricgroup__add_to_mep_groups_callback(
    pm: *const pmu_metric,
    _table: *const pmu_metrics_table,
    vdata: *mut c_void,
) -> c_int {
    let groups = vdata as *mut rblist;
    let mut g: *const c_char;
    let omg: *mut c_char;
    let mut mg: *mut c_char;

    mg = strdup(if !(*pm).metric_group.is_null() {
        (*pm).metric_group
    } else {
        (*pm).metric_name
    });
    if mg.is_null() {
        return -ENOMEM;
    }
    omg = mg;
    loop {
        g = strsep(&mut mg, b";\0".as_ptr() as *const c_char);
        if g.is_null() {
            break;
        }

        let me: *mut mep;

        g = skip_spaces(g);
        if strlen(g) != 0 {
            me = mep_lookup(groups, g, (*pm).metric_name);
        } else {
            me = mep_lookup(groups, (*pm).metric_name, (*pm).metric_name);
        }

        if !me.is_null() {
            (*me).metric_desc = (*pm).desc;
            (*me).metric_long_desc = (*pm).long_desc;
            (*me).metric_expr = (*pm).metric_expr;
            (*me).metric_threshold = (*pm).metric_threshold;
            (*me).metric_unit = (*pm).unit;
            (*me).pmu_name = (*pm).pmu;
        }
    }
    free(omg as *mut c_void);

    0
}

pub unsafe extern "C" fn metricgroup__print(print_cb: *const print_callbacks, print_state: *mut c_void) {
    let mut groups: rblist = mem::zeroed();
    let mut node: *mut rb_node;
    let mut next: *mut rb_node;
    let table = pmu_metrics_table__find();

    rblist__init(&mut groups);
    groups.node_new = Some(mep_new);
    groups.node_cmp = Some(mep_cmp);
    groups.node_delete = Some(mep_delete);

    metricgroup__for_each_metric(table, metricgroup__add_to_mep_groups_callback, &mut groups as *mut rblist as *mut c_void);

    node = rb_first_cached(&mut groups.entries);
    while !node.is_null() {
        let me = mep_from_rb_node(node);

        ((*print_cb).print_metric)(
            print_state,
            (*me).metric_group,
            (*me).metric_name,
            (*me).metric_desc,
            (*me).metric_long_desc,
            (*me).metric_expr,
            (*me).metric_threshold,
            (*me).metric_unit,
            (*me).pmu_name,
        );
        next = rb_next(node);
        rblist__remove_node(&mut groups, node);
        node = next;
    }
}

/*
 * Print the help text for the event symbols:
 */
pub unsafe extern "C" fn print_events(print_cb: *const print_callbacks, print_state: *mut c_void) {
    perf_pmus__print_pmu_events(print_cb, print_state);

    ((*print_cb).print_event)(
        print_state,
        ptr::null(),
        ptr::null(),
        PERF_TYPE_RAW,
        b"rNNN\0".as_ptr() as *const c_char,
        ptr::null(),
        ptr::null::<c_char>() as usize,
        false as usize,
        event_type_descriptors[PERF_TYPE_RAW as usize],
        ptr::null(),
        ptr::null(),
        ptr::null(),
    );

    perf_pmus__print_raw_pmu_events(print_cb, print_state);

    ((*print_cb).print_event)(
        print_state,
        ptr::null(),
        ptr::null(),
        PERF_TYPE_BREAKPOINT,
        b"mem:<addr>[/len][:access]\0".as_ptr() as *const c_char,
        ptr::null(),
        ptr::null::<c_char>() as usize,
        false as usize,
        event_type_descriptors[PERF_TYPE_BREAKPOINT as usize],
        ptr::null(),
        ptr::null(),
        ptr::null(),
    );

    print_sdt_events(print_cb, print_state);

    metricgroup__print(print_cb, print_state);

    print_libpfm_events(print_cb, print_state);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
