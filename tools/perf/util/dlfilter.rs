// SPDX-License-Identifier: GPL-2.0
/*
 * dlfilter.rs: Interface to perf script --dlfilter shared object
 * Copyright (c) 2021, Intel Corporation.
 *
 * Rust translation of perf/util/dlfilter.c.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type bool_ = bool;
type size_t = usize;
type __s32 = i32;
type __u8 = u8;
type __u32 = u32;
type __u64 = u64;
type u64_ = u64;

const PATH_MAX: usize = 4096;
const R_OK: c_int = 4;
const RTLD_NOW: c_int = 2;
const DT_DIR: u8 = 4;
const PERF_SAMPLE_ADDR: __u64 = 1 << 3;

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_session {
    _private: [u8; 0],
}

#[repr(C)]
pub union perf_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    pub sample_type: __u64,
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
pub struct dso_id {
    pub size: __u32,
    pub data: *const __u8,
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct maps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    pub name: *const c_char,
    pub start: __u64,
    pub end: __u64,
}

#[repr(C)]
pub struct addr_location {
    pub thread: *mut thread,
    pub map: *mut map,
    pub sym: *mut symbol,
    pub addr: __u64,
    pub filtered: bool_,
}

#[repr(C)]
pub struct branch_stack {
    pub nr: __u64,
}

#[repr(C)]
pub struct callchain {
    pub nr: __u64,
    pub ips: *mut __u64,
}

#[repr(C)]
pub struct perf_branch_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    pub ip: __u64,
    pub pid: __u32,
    pub tid: __u32,
    pub time: __u64,
    pub addr: __u64,
    pub id: __u64,
    pub stream_id: __u64,
    pub period: __u64,
    pub weight: __u64,
    pub ins_lat: __u16,
    pub transaction: __u64,
    pub insn_cnt: __u64,
    pub cyc_cnt: __u64,
    pub cpu: __u32,
    pub flags: __u64,
    pub data_src: __u64,
    pub phys_addr: __u64,
    pub data_page_size: __u64,
    pub code_page_size: __u64,
    pub cgroup: __u64,
    pub cpumode: __u8,
    pub misc: __u16,
    pub raw_size: __u32,
    pub raw_data: *mut c_void,
    pub machine_pid: __u32,
    pub vcpu: __u32,
    pub weight3: __u64,
    pub branch_stack: *mut branch_stack,
    pub callchain: *mut callchain,
    pub insn_len: __u32,
    pub insn: *mut __u8,
}

type __u16 = u16;

#[repr(C)]
pub struct perf_dlfilter_al {
    pub size: __u32,
    pub dso: *const c_char,
    pub sym: *const c_char,
    pub sym_start: __u64,
    pub sym_end: __u64,
    pub symoff: __u64,
    pub addr: __u64,
    pub comm: *const c_char,
    pub is_64_bit: bool_,
    pub is_kernel_ip: bool_,
    pub buildid_size: __u32,
    pub buildid: *const __u8,
    pub sym_binding: __u8,
    pub filtered: bool_,
    pub priv_: *mut c_void,
}

#[repr(C)]
pub struct perf_dlfilter_sample {
    pub size: __u32,
    pub ip: __u64,
    pub pid: __u32,
    pub tid: __u32,
    pub time: __u64,
    pub addr: __u64,
    pub id: __u64,
    pub stream_id: __u64,
    pub period: __u64,
    pub weight: __u64,
    pub ins_lat: __u16,
    pub transaction: __u64,
    pub insn_cnt: __u64,
    pub cyc_cnt: __u64,
    pub cpu: __u32,
    pub flags: __u64,
    pub data_src: __u64,
    pub phys_addr: __u64,
    pub data_page_size: __u64,
    pub code_page_size: __u64,
    pub cgroup: __u64,
    pub cpumode: __u8,
    pub misc: __u16,
    pub raw_size: __u32,
    pub raw_data: *mut c_void,
    pub machine_pid: __u32,
    pub vcpu: __u32,
    pub p_stage_cyc: __u64,
    pub brstack_nr: __u64,
    pub brstack: *mut perf_branch_entry,
    pub raw_callchain_nr: __u64,
    pub raw_callchain: *mut __u64,
    pub addr_correlates_sym: bool_,
    pub event: *const c_char,
}

#[repr(C)]
pub struct perf_dlfilter_fns {
    pub resolve_ip: Option<unsafe extern "C" fn(*mut c_void) -> *const perf_dlfilter_al>,
    pub resolve_addr: Option<unsafe extern "C" fn(*mut c_void) -> *const perf_dlfilter_al>,
    pub args: Option<unsafe extern "C" fn(*mut c_void, *mut c_int) -> *mut *mut c_char>,
    pub resolve_address: Option<unsafe extern "C" fn(*mut c_void, __u64, *mut perf_dlfilter_al) -> __s32>,
    pub al_cleanup: Option<unsafe extern "C" fn(*mut c_void, *mut perf_dlfilter_al)>,
    pub insn: Option<unsafe extern "C" fn(*mut c_void, *mut __u32) -> *const __u8>,
    pub srcline: Option<unsafe extern "C" fn(*mut c_void, *mut __u32) -> *const c_char>,
    pub attr: Option<unsafe extern "C" fn(*mut c_void) -> *mut perf_event_attr>,
    pub object_code: Option<unsafe extern "C" fn(*mut c_void, __u64, *mut c_void, __u32) -> __s32>,
}

type start_fn = unsafe extern "C" fn(*mut *mut c_void, *mut dlfilter) -> c_int;
type filter_event_fn = unsafe extern "C" fn(*mut c_void, *mut perf_dlfilter_sample, *mut dlfilter) -> c_int;
type stop_fn = unsafe extern "C" fn(*mut c_void, *mut dlfilter) -> c_int;

#[repr(C)]
pub struct dlfilter {
    pub file: *mut c_char,
    pub handle: *mut c_void,
    pub start: Option<start_fn>,
    pub filter_event: Option<filter_event_fn>,
    pub filter_event_early: Option<filter_event_fn>,
    pub stop: Option<stop_fn>,
    pub fns: *mut perf_dlfilter_fns,
    pub data: *mut c_void,
    pub session: *mut perf_session,
    pub event: *mut perf_event,
    pub sample: *mut perf_sample,
    pub evsel: *mut evsel,
    pub machine: *mut machine,
    pub al: *mut addr_location,
    pub addr_al: *mut addr_location,
    pub d_sample: *mut perf_dlfilter_sample,
    pub d_ip_al: *mut perf_dlfilter_al,
    pub d_addr_al: *mut perf_dlfilter_al,
    pub dlargc: c_int,
    pub dlargv: *mut *mut c_char,
    pub ctx_valid: bool_,
    pub in_start: bool_,
    pub in_stop: bool_,
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
pub struct DIR {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut symbol_conf: symbol_conf_t;
    static mut verbose: c_int;

    fn map__dso(map: *mut map) -> *mut dso;
    fn dso__long_name(dso: *mut dso) -> *const c_char;
    fn dso__name(dso: *mut dso) -> *const c_char;
    fn dso__is_64_bit(dso: *mut dso) -> bool_;
    fn dso__bid(dso: *mut dso) -> *mut dso_id;
    fn symbol__binding(sym: *mut symbol) -> __u8;
    fn machine__resolve(machine: *mut machine, al: *mut addr_location, sample: *mut perf_sample) -> c_int;
    fn machine__kernel_ip(machine: *mut machine, ip: __u64) -> bool_;
    fn thread__comm_str(thread: *mut thread) -> *const c_char;
    fn thread__resolve(thread: *mut thread, al: *mut addr_location, sample: *mut perf_sample);
    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn thread__find_symbol_fb(thread: *mut thread, cpumode: __u8, address: __u64, al: *mut addr_location);
    fn memdup(src: *const c_void, len: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn thread__maps(thread: *mut thread) -> *mut maps;
    fn maps__machine(maps: *mut maps) -> *mut machine;
    fn perf_sample__fetch_insn(sample: *mut perf_sample, thread: *mut thread, machine: *mut machine);
    fn get_srcline_split(dso: *mut dso, addr: __u64, line: *mut c_uint) -> *mut c_char;
    fn map__rip_2objdump(map: *mut map, addr: u64_) -> u64_;
    fn map__map_ip(map: *mut map, ip: __u64) -> u64_;
    fn map__end(map: *mut map) -> __u64;
    fn map__start(map: *mut map) -> __u64;
    fn dso__data_read_offset(dso: *mut dso, machine: *mut machine, offset: u64_, buf: *mut c_void, len: __u32) -> __s32;
    fn thread__find_map_fb(thread: *mut thread, cpumode: __u8, ip: __u64, al: *mut addr_location);
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn access(path: *const c_char, mode: c_int) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn get_argv_exec_path() -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> c_int;
    fn pr_err(format: *const c_char, ...);
    fn malloc(size: size_t) -> *mut c_void;
    fn perf_sample__branch_entries(sample: *mut perf_sample) -> *mut perf_branch_entry;
    fn sample_addr_correlates_sym(attr: *mut perf_event_attr) -> bool_;
    fn evsel__name(evsel: *mut evsel) -> *const c_char;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn exit(status: c_int) -> !;
}

#[repr(C)]
pub struct symbol_conf_t {
    pub show_kernel_path: bool_,
}

unsafe fn zfree(pptr: *mut *mut c_char) {
    if !(*pptr).is_null() {
        free(*pptr as *mut c_void);
        *pptr = ptr::null_mut();
    }
}

unsafe fn al_to_d_al(al: *mut addr_location, d_al: *mut perf_dlfilter_al) {
    let sym = (*al).sym;

    (*d_al).size = size_of::<perf_dlfilter_al>() as __u32;
    if !(*al).map.is_null() {
        let dso = map__dso((*al).map);

        if symbol_conf.show_kernel_path && !dso__long_name(dso).is_null() {
            (*d_al).dso = dso__long_name(dso);
        } else {
            (*d_al).dso = dso__name(dso);
        }
        (*d_al).is_64_bit = dso__is_64_bit(dso);
        (*d_al).buildid_size = (*dso__bid(dso)).size;
        (*d_al).buildid = (*dso__bid(dso)).data;
    } else {
        (*d_al).dso = ptr::null();
        (*d_al).is_64_bit = false;
        (*d_al).buildid_size = 0;
        (*d_al).buildid = ptr::null();
    }
    if !sym.is_null() {
        (*d_al).sym = (*sym).name;
        (*d_al).sym_start = (*sym).start;
        (*d_al).sym_end = (*sym).end;
        if (*al).addr < (*sym).end {
            (*d_al).symoff = (*al).addr.wrapping_sub((*sym).start);
        } else if !(*al).map.is_null() {
            (*d_al).symoff = (*al).addr.wrapping_sub(map__start((*al).map)).wrapping_sub((*sym).start);
        } else {
            (*d_al).symoff = 0;
        }
        (*d_al).sym_binding = symbol__binding(sym);
    } else {
        (*d_al).sym = ptr::null();
        (*d_al).sym_start = 0;
        (*d_al).sym_end = 0;
        (*d_al).symoff = 0;
        (*d_al).sym_binding = 0;
    }
    (*d_al).addr = (*al).addr;
    (*d_al).comm = ptr::null();
    (*d_al).filtered = false;
    (*d_al).priv_ = ptr::null_mut();
}

unsafe fn get_al(d: *mut dlfilter) -> *mut addr_location {
    let al = (*d).al;

    if (*al).thread.is_null() && machine__resolve((*d).machine, al, (*d).sample) < 0 {
        return ptr::null_mut();
    }
    al
}

unsafe fn get_thread(d: *mut dlfilter) -> *mut thread {
    let al = get_al(d);

    if !al.is_null() { (*al).thread } else { ptr::null_mut() }
}

unsafe extern "C" fn dlfilter__resolve_ip(ctx: *mut c_void) -> *const perf_dlfilter_al {
    let d = ctx as *mut dlfilter;
    let d_al = (*d).d_ip_al;
    let al: *mut addr_location;

    if !(*d).ctx_valid {
        return ptr::null();
    }

    /* 'size' is also used to indicate already initialized */
    if (*d_al).size != 0 {
        return d_al;
    }

    al = get_al(d);
    if al.is_null() {
        return ptr::null();
    }

    al_to_d_al(al, d_al);

    (*d_al).is_kernel_ip = machine__kernel_ip((*d).machine, (*(*d).sample).ip);
    (*d_al).comm = if !(*al).thread.is_null() {
        thread__comm_str((*al).thread)
    } else {
        c":-1".as_ptr()
    };
    (*d_al).filtered = (*al).filtered;

    d_al
}

unsafe extern "C" fn dlfilter__resolve_addr(ctx: *mut c_void) -> *const perf_dlfilter_al {
    let d = ctx as *mut dlfilter;
    let d_addr_al = (*d).d_addr_al;
    let addr_al = (*d).addr_al;

    if !(*d).ctx_valid || !(*(*d).d_sample).addr_correlates_sym {
        return ptr::null();
    }

    /* 'size' is also used to indicate already initialized */
    if (*d_addr_al).size != 0 {
        return d_addr_al;
    }

    if (*addr_al).thread.is_null() {
        let thread = get_thread(d);

        if thread.is_null() {
            return ptr::null();
        }
        thread__resolve(thread, addr_al, (*d).sample);
    }

    al_to_d_al(addr_al, d_addr_al);

    (*d_addr_al).is_kernel_ip = machine__kernel_ip((*d).machine, (*(*d).sample).addr);

    d_addr_al
}

unsafe extern "C" fn dlfilter__args(ctx: *mut c_void, dlargc: *mut c_int) -> *mut *mut c_char {
    let d = ctx as *mut dlfilter;

    if !dlargc.is_null() {
        *dlargc = 0;
    } else {
        return ptr::null_mut();
    }

    if !(*d).ctx_valid && !(*d).in_start && !(*d).in_stop {
        return ptr::null_mut();
    }

    *dlargc = (*d).dlargc;
    (*d).dlargv
}

unsafe fn has_priv(d_al_p: *mut perf_dlfilter_al) -> bool_ {
    (*d_al_p).size as usize >= offset_of!(perf_dlfilter_al, priv_) + size_of::<*mut c_void>()
}

unsafe extern "C" fn dlfilter__resolve_address(
    ctx: *mut c_void,
    address: __u64,
    d_al_p: *mut perf_dlfilter_al,
) -> __s32 {
    let d = ctx as *mut dlfilter;
    let mut d_al: perf_dlfilter_al = core::mem::zeroed();
    let mut al: addr_location = core::mem::zeroed();
    let thread: *mut thread;
    let sz: __u32;

    if !(*d).ctx_valid || d_al_p.is_null() {
        return -1;
    }

    thread = get_thread(d);
    if thread.is_null() {
        return -1;
    }

    addr_location__init(&mut al);
    thread__find_symbol_fb(thread, (*(*d).sample).cpumode, address, &mut al);

    al_to_d_al(&mut al, &mut d_al);

    d_al.is_kernel_ip = machine__kernel_ip((*d).machine, address);

    sz = (*d_al_p).size;
    memcpy(
        d_al_p as *mut c_void,
        &d_al as *const _ as *const c_void,
        core::cmp::min(sz as usize, size_of::<perf_dlfilter_al>()),
    );
    (*d_al_p).size = sz;

    if has_priv(d_al_p) {
        (*d_al_p).priv_ = memdup(&al as *const _ as *const c_void, size_of::<addr_location>());
    } else {
        /* Avoid leak for v0 API */
        addr_location__exit(&mut al);
    }

    0
}

unsafe extern "C" fn dlfilter__al_cleanup(_ctx: *mut c_void, d_al_p: *mut perf_dlfilter_al) {
    let al: *mut addr_location;

    /* Ensure backward compatibility */
    if !has_priv(d_al_p) || (*d_al_p).priv_.is_null() {
        return;
    }

    al = (*d_al_p).priv_ as *mut addr_location;

    (*d_al_p).priv_ = ptr::null_mut();

    addr_location__exit(al);

    free(al as *mut c_void);
}

unsafe extern "C" fn dlfilter__insn(ctx: *mut c_void, len: *mut __u32) -> *const __u8 {
    let d = ctx as *mut dlfilter;

    if len.is_null() {
        return ptr::null();
    }

    *len = 0;

    if !(*d).ctx_valid {
        return ptr::null();
    }

    if (*(*d).sample).ip != 0 && (*(*d).sample).insn_len == 0 {
        let al = (*d).al;

        if (*al).thread.is_null() && machine__resolve((*d).machine, al, (*d).sample) < 0 {
            return ptr::null();
        }

        if !thread__maps((*al).thread).is_null() {
            let machine = maps__machine(thread__maps((*al).thread));

            if !machine.is_null() {
                perf_sample__fetch_insn((*d).sample, (*al).thread, machine);
            }
        }
    }

    if (*(*d).sample).insn_len == 0 {
        return ptr::null();
    }

    *len = (*(*d).sample).insn_len;

    (*(*d).sample).insn as *const __u8
}

unsafe extern "C" fn dlfilter__srcline(ctx: *mut c_void, line_no: *mut __u32) -> *const c_char {
    let d = ctx as *mut dlfilter;
    let al: *mut addr_location;
    let mut line: c_uint = 0;
    let mut srcfile: *mut c_char = ptr::null_mut();
    let map: *mut map;
    let dso: *mut dso;
    let addr: u64_;

    if !(*d).ctx_valid || line_no.is_null() {
        return ptr::null();
    }

    al = get_al(d);
    if al.is_null() {
        return ptr::null();
    }

    map = (*al).map;
    addr = (*al).addr;
    dso = if !map.is_null() { map__dso(map) } else { ptr::null_mut() };

    if !dso.is_null() {
        srcfile = get_srcline_split(dso, map__rip_2objdump(map, addr), &mut line);
    }

    *line_no = line;
    srcfile
}

unsafe extern "C" fn dlfilter__attr(ctx: *mut c_void) -> *mut perf_event_attr {
    let d = ctx as *mut dlfilter;

    if !(*d).ctx_valid {
        return ptr::null_mut();
    }

    &mut (*(*d).evsel).core.attr
}

unsafe fn code_read(ip: __u64, map: *mut map, machine: *mut machine, buf: *mut c_void, mut len: __u32) -> __s32 {
    let offset: u64_ = map__map_ip(map, ip);

    if ip.wrapping_add(len as __u64) >= map__end(map) {
        len = map__end(map).wrapping_sub(ip) as __u32;
    }

    dso__data_read_offset(map__dso(map), machine, offset, buf, len)
}

unsafe extern "C" fn dlfilter__object_code(ctx: *mut c_void, ip: __u64, buf: *mut c_void, len: __u32) -> __s32 {
    let d = ctx as *mut dlfilter;
    let al: *mut addr_location;
    let mut a: addr_location = core::mem::zeroed();
    let ret: __s32;

    if !(*d).ctx_valid {
        return -1;
    }

    al = get_al(d);
    if al.is_null() {
        return -1;
    }

    if !(*al).map.is_null()
        && ip >= map__start((*al).map)
        && ip < map__end((*al).map)
        && machine__kernel_ip((*d).machine, ip) == machine__kernel_ip((*d).machine, (*(*d).sample).ip)
    {
        return code_read(ip, (*al).map, (*d).machine, buf, len);
    }

    addr_location__init(&mut a);

    thread__find_map_fb((*al).thread, (*(*d).sample).cpumode, ip, &mut a);
    ret = if !a.map.is_null() {
        code_read(ip, a.map, (*d).machine, buf, len)
    } else {
        -1
    };

    addr_location__exit(&mut a);

    ret
}

static perf_dlfilter_fns: perf_dlfilter_fns = perf_dlfilter_fns {
    resolve_ip: Some(dlfilter__resolve_ip),
    resolve_addr: Some(dlfilter__resolve_addr),
    args: Some(dlfilter__args),
    resolve_address: Some(dlfilter__resolve_address),
    al_cleanup: Some(dlfilter__al_cleanup),
    insn: Some(dlfilter__insn),
    srcline: Some(dlfilter__srcline),
    attr: Some(dlfilter__attr),
    object_code: Some(dlfilter__object_code),
};

unsafe fn find_dlfilter(mut file: *const c_char) -> *mut c_char {
    let mut path: [c_char; PATH_MAX] = [0; PATH_MAX];
    let exec_path: *mut c_char;

    if !strchr(file, '/' as c_int).is_null() {
        return strdup(file);
    }

    if access(file, R_OK) == 0 {
        /*
         * Prepend "./" so that dlopen will find the file in the
         * current directory.
         */
        snprintf(path.as_mut_ptr(), path.len(), c"./%s".as_ptr(), file);
        file = path.as_ptr();
        return strdup(file);
    }

    exec_path = get_argv_exec_path();
    if exec_path.is_null() {
        return strdup(file);
    }
    snprintf(path.as_mut_ptr(), path.len(), c"%s/dlfilters/%s".as_ptr(), exec_path, file);
    free(exec_path as *mut c_void);
    if access(path.as_ptr(), R_OK) == 0 {
        file = path.as_ptr();
    }

    strdup(file)
}

/* CHECK_FLAG(x) asserted C PERF_DLFILTER_FLAG_* values match PERF_IP_FLAG_* values. */

unsafe fn dlfilter__init(d: *mut dlfilter, file: *const c_char, dlargc: c_int, dlargv: *mut *mut c_char) -> c_int {
    memset(d as *mut c_void, 0, size_of::<dlfilter>());
    (*d).file = find_dlfilter(file);
    if (*d).file.is_null() {
        return -1;
    }
    (*d).dlargc = dlargc;
    (*d).dlargv = dlargv;
    0
}

unsafe fn dlfilter__exit(d: *mut dlfilter) {
    zfree(&mut (*d).file);
}

unsafe fn dlfilter__open(d: *mut dlfilter) -> c_int {
    (*d).handle = dlopen((*d).file, RTLD_NOW);
    if (*d).handle.is_null() {
        pr_err(c"dlopen failed for: '%s'\n".as_ptr(), (*d).file);
        return -1;
    }
    (*d).start = core::mem::transmute(dlsym((*d).handle, c"start".as_ptr()));
    (*d).filter_event = core::mem::transmute(dlsym((*d).handle, c"filter_event".as_ptr()));
    (*d).filter_event_early = core::mem::transmute(dlsym((*d).handle, c"filter_event_early".as_ptr()));
    (*d).stop = core::mem::transmute(dlsym((*d).handle, c"stop".as_ptr()));
    (*d).fns = dlsym((*d).handle, c"perf_dlfilter_fns".as_ptr()) as *mut perf_dlfilter_fns;
    if !(*d).fns.is_null() {
        memcpy(
            (*d).fns as *mut c_void,
            &perf_dlfilter_fns as *const _ as *const c_void,
            size_of::<perf_dlfilter_fns>(),
        );
    }
    0
}

unsafe fn dlfilter__close(d: *mut dlfilter) -> c_int {
    dlclose((*d).handle)
}

#[no_mangle]
pub unsafe extern "C" fn dlfilter__new(file: *const c_char, dlargc: c_int, dlargv: *mut *mut c_char) -> *mut dlfilter {
    let d = malloc(size_of::<dlfilter>()) as *mut dlfilter;

    if d.is_null() {
        return ptr::null_mut();
    }

    if dlfilter__init(d, file, dlargc, dlargv) != 0 {
        free(d as *mut c_void);
        return ptr::null_mut();
    }

    if dlfilter__open(d) != 0 {
        dlfilter__exit(d);
        free(d as *mut c_void);
        return ptr::null_mut();
    }

    d
}

unsafe fn dlfilter__free(d: *mut dlfilter) {
    if !d.is_null() {
        dlfilter__exit(d);
        free(d as *mut c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn dlfilter__start(d: *mut dlfilter, session: *mut perf_session) -> c_int {
    if !d.is_null() {
        (*d).session = session;
        if let Some(start) = (*d).start {
            let ret: c_int;

            (*d).in_start = true;
            ret = start(&mut (*d).data, d);
            (*d).in_start = false;
            return ret;
        }
    }
    0
}

unsafe fn dlfilter__stop(d: *mut dlfilter) -> c_int {
    if !d.is_null() {
        if let Some(stop) = (*d).stop {
            let ret: c_int;

            (*d).in_stop = true;
            ret = stop((*d).data, d);
            (*d).in_stop = false;
            return ret;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn dlfilter__cleanup(d: *mut dlfilter) {
    if !d.is_null() {
        dlfilter__stop(d);
        dlfilter__close(d);
        dlfilter__free(d);
    }
}

#[no_mangle]
pub unsafe extern "C" fn dlfilter__do_filter_event(
    d: *mut dlfilter,
    event: *mut perf_event,
    sample: *mut perf_sample,
    evsel: *mut evsel,
    machine: *mut machine,
    al: *mut addr_location,
    addr_al: *mut addr_location,
    early: bool_,
) -> c_int {
    let mut d_sample: perf_dlfilter_sample = core::mem::zeroed();
    let mut d_ip_al: perf_dlfilter_al = core::mem::zeroed();
    let mut d_addr_al: perf_dlfilter_al = core::mem::zeroed();
    let ret: c_int;

    (*d).event = event;
    (*d).sample = sample;
    (*d).evsel = evsel;
    (*d).machine = machine;
    (*d).al = al;
    (*d).addr_al = addr_al;
    (*d).d_sample = &mut d_sample;
    (*d).d_ip_al = &mut d_ip_al;
    (*d).d_addr_al = &mut d_addr_al;

    d_sample.size = size_of::<perf_dlfilter_sample>() as __u32;
    d_sample.p_stage_cyc = (*sample).weight3;
    d_ip_al.size = 0; /* To indicate d_ip_al is not initialized */
    d_addr_al.size = 0; /* To indicate d_addr_al is not initialized */

    d_sample.ip = (*sample).ip;
    d_sample.pid = (*sample).pid;
    d_sample.tid = (*sample).tid;
    d_sample.time = (*sample).time;
    d_sample.addr = (*sample).addr;
    d_sample.id = (*sample).id;
    d_sample.stream_id = (*sample).stream_id;
    d_sample.period = (*sample).period;
    d_sample.weight = (*sample).weight;
    d_sample.ins_lat = (*sample).ins_lat;
    d_sample.transaction = (*sample).transaction;
    d_sample.insn_cnt = (*sample).insn_cnt;
    d_sample.cyc_cnt = (*sample).cyc_cnt;
    d_sample.cpu = (*sample).cpu;
    d_sample.flags = (*sample).flags;
    d_sample.data_src = (*sample).data_src;
    d_sample.phys_addr = (*sample).phys_addr;
    d_sample.data_page_size = (*sample).data_page_size;
    d_sample.code_page_size = (*sample).code_page_size;
    d_sample.cgroup = (*sample).cgroup;
    d_sample.cpumode = (*sample).cpumode;
    d_sample.misc = (*sample).misc;
    d_sample.raw_size = (*sample).raw_size;
    d_sample.raw_data = (*sample).raw_data;
    d_sample.machine_pid = (*sample).machine_pid;
    d_sample.vcpu = (*sample).vcpu;

    if !(*sample).branch_stack.is_null() {
        d_sample.brstack_nr = (*(*sample).branch_stack).nr;
        d_sample.brstack = perf_sample__branch_entries(sample);
    } else {
        d_sample.brstack_nr = 0;
        d_sample.brstack = ptr::null_mut();
    }

    if !(*sample).callchain.is_null() {
        d_sample.raw_callchain_nr = (*(*sample).callchain).nr;
        d_sample.raw_callchain = (*(*sample).callchain).ips;
    } else {
        d_sample.raw_callchain_nr = 0;
        d_sample.raw_callchain = ptr::null_mut();
    }

    d_sample.addr_correlates_sym =
        ((*evsel).core.attr.sample_type & PERF_SAMPLE_ADDR) != 0
            && sample_addr_correlates_sym(&mut (*evsel).core.attr);

    d_sample.event = evsel__name(evsel);

    (*d).ctx_valid = true;

    if early {
        ret = (*d).filter_event_early.unwrap()((*d).data, &mut d_sample, d);
    } else {
        ret = (*d).filter_event.unwrap()((*d).data, &mut d_sample, d);
    }

    (*d).ctx_valid = false;

    ret
}

#[no_mangle]
pub unsafe extern "C" fn get_filter_desc(
    dirname: *const c_char,
    name: *const c_char,
    desc: *mut *mut c_char,
    long_desc: *mut *mut c_char,
) -> bool_ {
    let mut path: [c_char; PATH_MAX] = [0; PATH_MAX];
    let handle: *mut c_void;
    let desc_fn: Option<unsafe extern "C" fn(*mut *const c_char) -> *const c_char>;

    snprintf(path.as_mut_ptr(), path.len(), c"%s/%s".as_ptr(), dirname, name);
    handle = dlopen(path.as_ptr(), RTLD_NOW);
    if handle.is_null()
        || (dlsym(handle, c"filter_event".as_ptr()).is_null()
            && dlsym(handle, c"filter_event_early".as_ptr()).is_null())
    {
        return false;
    }
    desc_fn = core::mem::transmute(dlsym(handle, c"filter_description".as_ptr()));
    if let Some(desc_fn) = desc_fn {
        let dsc: *const c_char;
        let mut long_dsc: *const c_char = ptr::null();

        dsc = desc_fn(&mut long_dsc);
        if !dsc.is_null() {
            *desc = strdup(dsc);
        }
        if !long_dsc.is_null() {
            *long_desc = strdup(long_dsc);
        }
    }
    dlclose(handle);
    true
}

unsafe fn list_filters(dirname: *const c_char) {
    let mut entry: *mut dirent;
    let dir: *mut DIR;

    dir = opendir(dirname);
    if dir.is_null() {
        return;
    }

    loop {
        entry = readdir(dir);
        if entry.is_null() {
            break;
        }

        let n = strlen((*entry).d_name.as_ptr());
        let mut long_desc: *mut c_char = ptr::null_mut();
        let mut desc: *mut c_char = ptr::null_mut();

        if (*entry).d_type == DT_DIR
            || n < 4
            || strcmp(c".so".as_ptr(), (*entry).d_name.as_ptr().add(n - 3)) != 0
        {
            continue;
        }
        if !get_filter_desc(dirname, (*entry).d_name.as_ptr(), &mut desc, &mut long_desc) {
            continue;
        }
        printf(c"  %-36s %s\n".as_ptr(), (*entry).d_name.as_ptr(), if !desc.is_null() { desc } else { c"".as_ptr() as *mut c_char });
        if verbose > 0 {
            let mut p = long_desc;
            let mut line: *mut c_char;

            loop {
                line = strsep(&mut p, c"\n".as_ptr());
                if line.is_null() {
                    break;
                }
                printf(c"%39s%s\n".as_ptr(), c"".as_ptr(), line);
            }
        }
        free(long_desc as *mut c_void);
        free(desc as *mut c_void);
    }

    closedir(dir);
}

#[no_mangle]
pub unsafe extern "C" fn list_available_dlfilters(
    _opt: *const option,
    _s: *const c_char,
    _unset: c_int,
) -> c_int {
    let mut path: [c_char; PATH_MAX] = [0; PATH_MAX];
    let exec_path: *mut c_char;

    printf(c"List of available dlfilters:\n".as_ptr());

    list_filters(c".".as_ptr());

    exec_path = get_argv_exec_path();
    if exec_path.is_null() {
        exit(0);
    }
    snprintf(path.as_mut_ptr(), path.len(), c"%s/dlfilters".as_ptr(), exec_path);

    list_filters(path.as_ptr());

    free(exec_path as *mut c_void);
    exit(0);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
