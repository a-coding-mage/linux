// SPDX-License-Identifier: GPL-2.0
/*
 * build-id.c
 *
 * build-id support
 *
 * Copyright (C) 2009, 2010 Red Hat Inc.
 * Copyright (C) 2009, 2010 Arnaldo Carvalho de Melo <acme@redhat.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_ushort, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type pid_t = c_int;
type u8 = c_uchar;
type u16 = c_ushort;
type u64 = c_ulong;

const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const EEXIST: c_int = 17;
const F_OK: c_int = 0;
const R_OK: c_int = 4;
const X_OK: c_int = 1;
const PATH_MAX: usize = 4096;
const BUILD_ID_SIZE: usize = 20;
const SBUILD_ID_SIZE: usize = 41;
const SBUILD_ID_MIN_SIZE: usize = 3;
const PERF_MAX_STACK_DEPTH: c_int = 127;
const PERF_RECORD_MISC_BUILD_ID_SIZE: u16 = 0x2000;
const PERF_RECORD_MISC_KERNEL: u16 = 1;
const PERF_RECORD_MISC_USER: u16 = 2;
const PERF_RECORD_MISC_GUEST_KERNEL: u16 = 3;
const PERF_RECORD_MISC_GUEST_USER: u16 = 4;
const PERF_RECORD_MISC_CPUMODE_UNKNOWN: u16 = 0;

#[repr(C)]
pub struct build_id {
    pub data: [u8; BUILD_ID_SIZE],
    pub size: size_t,
}

#[repr(C)]
pub struct perf_event_header {
    pub type_: u32,
    pub misc: u16,
    pub size: u16,
}

#[repr(C)]
pub struct perf_record_header_build_id {
    pub header: perf_event_header,
    pub pid: pid_t,
    pub size: u8,
    pub data: [u8; BUILD_ID_SIZE],
}

#[repr(C)]
pub struct callchain_cursor_node {
    pub ms: map_symbol,
}

#[repr(C)]
pub struct map_symbol {
    pub map: *mut map,
}

#[repr(C)]
pub struct perf_sample {
    pub pid: pid_t,
    pub tid: pid_t,
    pub cpumode: u8,
    pub ip: u64,
    pub file_offset: u64,
}

#[repr(C)]
pub union perf_event {
    pub header: perf_event_header,
}

#[repr(C)]
pub struct addr_location {
    pub map: *mut map,
}

#[repr(C)]
pub struct machine {
    pub rb_node: rb_node,
    pub dsos: dsos,
    pub pid: pid_t,
    pub mmap_name: *const c_char,
    pub root_dir: [c_char; PATH_MAX],
}

#[repr(C)]
pub struct machines {
    pub host: machine,
    pub guests: rb_root_cached,
}

#[repr(C)]
pub struct perf_session {
    pub machines: machines,
}

#[repr(C)]
pub struct machine__write_buildid_table_cb_args {
    pub machine: *mut machine,
    pub fd: *mut feat_fd,
    pub kmisc: u16,
    pub umisc: u16,
}

#[repr(C)]
pub struct str_node {
    pub rb_node: rb_node,
    pub s: *mut c_char,
}

#[repr(C)]
pub struct dirent {
    pub d_name: [c_char; 256],
}

#[repr(C)]
pub struct stat {
    pub st_mode: c_uint,
}

#[repr(C)]
pub struct nscookie {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso_id {
    pub build_id: build_id,
}

#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_root_cached {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_tool {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dsos {
    _private: [u8; 0],
}

#[repr(C)]
pub struct feat_fd {
    _private: [u8; 0],
}

#[repr(C)]
pub struct strlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nsinfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct probe_cache {
    _private: [u8; 0],
}

type machine__dso_t = Option<unsafe extern "C" fn(*mut dso, *mut machine, *mut c_void) -> c_int>;

unsafe extern "C" {
    static mut buildid_dir: *const c_char;
    static mut errno: c_int;
    static DSO__NAME_KALLSYMS: *const c_char;
    static DSO__NAME_VDSO: *const c_char;

    fn map__dso(map: *mut map) -> *mut dso;
    fn dso__set_hit(dso: *mut dso);
    fn machine__findnew_thread(machine: *mut machine, pid: pid_t, tid: pid_t) -> *mut thread;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_debug4(fmt: *const c_char, ...);
    fn perf_event__name(type_: u32) -> *const c_char;
    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn thread__find_map(thread: *mut thread, cpumode: u8, addr: u64, al: *mut addr_location) -> bool;
    fn sample__for_each_callchain_node(
        thread: *mut thread,
        sample: *mut perf_sample,
        max_stack: c_int,
        symbols: bool,
        cb: Option<unsafe extern "C" fn(*mut callchain_cursor_node, *mut c_void) -> c_int>,
        data: *mut c_void,
    );
    fn thread__put(thread: *mut thread);
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn vsnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ap: *mut c_void) -> c_int;
    fn vasprintf(strp: *mut *mut c_char, fmt: *const c_char, ap: *mut c_void) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn access(path: *const c_char, mode: c_int) -> c_int;
    fn readlink(path: *const c_char, buf: *mut c_char, bufsiz: size_t) -> ssize_t;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memchr_inv(s: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    fn sysfs__read_build_id(filename: *const c_char, bid: *mut build_id) -> c_int;
    fn filename__read_build_id(filename: *const c_char, bid: *mut build_id) -> c_int;
    fn dso__is_vdso(dso: *mut dso) -> bool;
    fn dso__has_build_id(dso: *const dso) -> bool;
    fn dso__bid(dso: *const dso) -> *const build_id;
    fn is_regular_file(filename: *const c_char) -> bool;
    fn dso__is_kallsyms(dso: *mut dso) -> bool;
    fn do_write(fd: *mut feat_fd, buf: *const c_void, size: size_t) -> c_int;
    fn write_padded(fd: *mut feat_fd, buf: *const c_char, count: size_t, count_aligned: size_t) -> c_int;
    fn dso__hit(dso: *mut dso) -> bool;
    fn dso__is_kcore(dso: *mut dso) -> bool;
    fn dso__short_name(dso: *mut dso) -> *const c_char;
    fn dso__short_name_len(dso: *mut dso) -> size_t;
    fn dso__long_name(dso: *mut dso) -> *const c_char;
    fn dso__long_name_len(dso: *mut dso) -> size_t;
    fn dso__kernel(dso: *mut dso) -> bool;
    fn is_kernel_module(name: *const c_char, cpumode: u16) -> bool;
    fn dso__id(dso: *mut dso) -> *mut dso_id;
    fn machine__is_host(machine: *mut machine) -> bool;
    fn dsos__for_each_dso(
        dsos: *mut dsos,
        cb: Option<unsafe extern "C" fn(*mut dso, *mut c_void) -> c_int>,
        data: *mut c_void,
    ) -> c_int;
    fn rb_first_cached(root: *mut rb_root_cached) -> *mut rb_node;
    fn rb_next(node: *mut rb_node) -> *mut rb_node;
    fn isxdigit(c: c_int) -> c_int;
    fn symbol__init(arg: *mut c_void) -> c_int;
    fn strlist__new(a: *mut c_void, b: *mut c_void) -> *mut strlist;
    fn lsdir(name: *const c_char, filter: Option<unsafe extern "C" fn(*const c_char, *mut dirent) -> bool>) -> *mut strlist;
    fn strlist__delete(strlist: *mut strlist);
    fn strlist__add(strlist: *mut strlist, s: *const c_char) -> c_int;
    fn strlist__first(strlist: *mut strlist) -> *mut str_node;
    fn strlist__next(node: *mut str_node) -> *mut str_node;
    fn zfree(ptr: *mut *mut c_char);
    fn lsdir_no_dot_filter(name: *const c_char, d: *mut dirent) -> bool;
    fn nsinfo__realpath(name: *const c_char, nsi: *mut nsinfo) -> *mut c_char;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn path__join(buf: *mut c_char, size: size_t, a: *const c_char, b: *const c_char) -> c_int;
    fn __symbol__join_symfs(buf: *mut c_char, size: size_t, path: *const c_char) -> size_t;
    fn nsinfo__mountns_enter(nsi: *mut nsinfo, nsc: *mut nscookie);
    fn nsinfo__mountns_exit(nsc: *mut nscookie);
    fn realpath(path: *const c_char, resolved_path: *mut c_char) -> *mut c_char;
    fn close(fd: c_int) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn mkdir_p(path: *const c_char, mode: c_int) -> c_int;
    fn copyfile(from: *const c_char, to: *const c_char) -> c_int;
    fn copyfile_ns(from: *const c_char, to: *const c_char, nsi: *mut nsinfo) -> c_int;
    fn link(oldpath: *const c_char, newpath: *const c_char) -> c_int;
    fn stat(path: *const c_char, buf: *mut stat) -> c_int;
    fn copyfile_mode(from: *const c_char, to: *const c_char, mode: c_uint) -> c_int;
    fn nsinfo__need_setns(nsi: *mut nsinfo) -> bool;
    fn symlink(target: *const c_char, linkpath: *const c_char) -> c_int;
    fn rm_rf(path: *const c_char) -> c_int;
    fn dso__lock(dso: *mut dso) -> *mut c_void;
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn dso__nsinfo(dso: *mut dso) -> *mut nsinfo;
    fn dso__build_id_equal(dso: *mut dso, bid: *const build_id) -> bool;
    fn machine__for_each_dso(machine: *mut machine, fn_: machine__dso_t, priv_: *mut c_void) -> c_int;
    fn mkdir(pathname: *const c_char, mode: c_int) -> c_int;
    fn dsos__read_build_ids(dsos: *mut dsos, with_hits: bool) -> bool;
    fn probe_cache__new(sbuild_id: *const c_char, nsi: *mut nsinfo) -> *mut probe_cache;
    fn probe_cache__scan_sdt(cache: *mut probe_cache, realname: *const c_char) -> c_int;
    fn probe_cache__commit(cache: *mut probe_cache) -> c_int;
    fn probe_cache__delete(cache: *mut probe_cache);
}

static mut no_buildid_cache: bool = false;

unsafe fn ptr_add<T>(p: *mut T, n: usize) -> *mut T {
    p.add(n)
}

unsafe fn cstr_lit(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

fn PERF_ALIGN(x: size_t, a: size_t) -> size_t {
    (x + a - 1) & !(a - 1)
}

unsafe extern "C" fn mark_dso_hit_callback(node: *mut callchain_cursor_node, _data: *mut c_void) -> c_int {
    let map = (*node).ms.map;

    if !map.is_null() {
        dso__set_hit(map__dso(map));
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn build_id__mark_dso_hit(
    _tool: *const perf_tool,
    event: *mut perf_event,
    sample: *mut perf_sample,
    machine: *mut machine,
) -> c_int {
    let mut al: addr_location = mem::zeroed();
    let thread = machine__findnew_thread(machine, (*sample).pid, (*sample).tid);

    if thread.is_null() {
        pr_err(
            cstr_lit(b"problem processing %s event at offset %#lx, skipping it.\n\0"),
            perf_event__name((*event).header.type_),
            (*sample).file_offset,
        );
        return -1;
    }

    addr_location__init(&mut al);
    if thread__find_map(thread, (*sample).cpumode, (*sample).ip, &mut al) {
        dso__set_hit(map__dso(al.map));
    }

    addr_location__exit(&mut al);

    sample__for_each_callchain_node(
        thread,
        sample,
        PERF_MAX_STACK_DEPTH,
        false,
        Some(mark_dso_hit_callback),
        ptr::null_mut(),
    );

    thread__put(thread);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn build_id__snprintf(build_id: *const build_id, bf: *mut c_char, bf_size: size_t) -> c_int {
    let mut offs: size_t = 0;

    if (*build_id).size == 0 {
        /* Ensure bf is always \0 terminated. */
        if bf_size > 0 {
            *bf = 0;
        }
        return 0;
    }

    if bf_size > 0 {
        *bf = 0;
    }

    let mut i: size_t = 0;
    while i < (*build_id).size && offs + 1 < bf_size {
        offs += scnprintf(
            bf.add(offs),
            bf_size - offs,
            cstr_lit(b"%02x\0"),
            (*build_id).data[i] as c_int,
        ) as size_t;
        i += 1;
    }

    offs as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sysfs__snprintf_build_id(mut root_dir: *const c_char, sbuild_id: *mut c_char, sbuild_id_size: size_t) -> c_int {
    let mut notes = [0 as c_char; PATH_MAX];
    let mut bid: build_id = mem::zeroed();
    let ret: c_int;

    if root_dir.is_null() {
        root_dir = cstr_lit(b"\0");
    }

    scnprintf(notes.as_mut_ptr(), notes.len(), cstr_lit(b"%s/sys/kernel/notes\0"), root_dir);

    ret = sysfs__read_build_id(notes.as_ptr(), &mut bid);
    if ret < 0 {
        return ret;
    }

    build_id__snprintf(&bid, sbuild_id, sbuild_id_size)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn filename__snprintf_build_id(pathname: *const c_char, sbuild_id: *mut c_char, sbuild_id_size: size_t) -> c_int {
    let mut bid: build_id = mem::zeroed();
    let ret = filename__read_build_id(pathname, &mut bid);
    if ret < 0 {
        return ret;
    }

    build_id__snprintf(&bid, sbuild_id, sbuild_id_size)
}

/* asnprintf consolidates asprintf and snprintf */
unsafe fn asnprintf(strp: *mut *mut c_char, size: size_t, fmt: *const c_char, a1: *const c_char, a2: *const c_char, a3: *const c_char) -> c_int {
    if strp.is_null() {
        return -EINVAL;
    }

    if !(*strp).is_null() {
        snprintf(*strp, size, fmt, a1, a2, a3)
    } else {
        asprintf(strp, fmt, a1, a2, a3)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn build_id_cache__kallsyms_path(sbuild_id: *const c_char, bf: *mut c_char, size: size_t) -> *mut c_char {
    let mut retry_old = true;

    snprintf(bf, size, cstr_lit(b"%s/%s/%s/kallsyms\0"), buildid_dir, DSO__NAME_KALLSYMS, sbuild_id);
    loop {
        if access(bf, F_OK) == 0 {
            return bf;
        }
        if retry_old {
            /* Try old style kallsyms cache */
            snprintf(bf, size, cstr_lit(b"%s/%s/%s\0"), buildid_dir, DSO__NAME_KALLSYMS, sbuild_id);
            retry_old = false;
            continue;
        }
        break;
    }

    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn build_id_cache__linkname(sbuild_id: *const c_char, mut bf: *mut c_char, size: size_t) -> *mut c_char {
    let tmp = bf;
    let ret = asnprintf(
        &mut bf,
        size,
        cstr_lit(b"%s/.build-id/%.2s/%s\0"),
        buildid_dir,
        sbuild_id,
        sbuild_id.add(2),
    );
    if ret < 0 || (!tmp.is_null() && size < ret as c_uint as size_t) {
        return ptr::null_mut();
    }
    bf
}

/* The caller is responsible to free the returned buffer. */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn build_id_cache__origname(sbuild_id: *const c_char) -> *mut c_char {
    let linkname: *mut c_char;
    let mut buf = [0 as c_char; PATH_MAX];
    let mut ret: *mut c_char = ptr::null_mut();
    let mut offs: size_t = 5; /* == strlen("../..") */

    linkname = build_id_cache__linkname(sbuild_id, ptr::null_mut(), 0);
    if linkname.is_null() {
        return ptr::null_mut();
    }

    let len = readlink(linkname, buf.as_mut_ptr(), buf.len() - 1);
    if len <= 0 {
        free(linkname as *mut c_void);
        return ret;
    }
    buf[len as usize] = 0;

    /* The link should be "../..<origpath>/<sbuild_id>" */
    let p = strrchr(buf.as_ptr(), '/' as c_int); /* Cut off the "/<sbuild_id>" */
    if !p.is_null() && p > buf.as_mut_ptr().add(offs) {
        *p = 0;
        if buf[offs + 1] == '[' as c_char {
            offs += 1; /*
                        * This is a DSO name, like [kernel.kallsyms].
                        * Skip the first '/', since this is not the
                        * cache of a regular file.
                        */
        }
        ret = strdup(buf.as_mut_ptr().add(offs)); /* Skip "../..[/]" */
    }
    free(linkname as *mut c_void);
    ret
}

/* Check if the given build_id cache is valid on current running system */
unsafe fn build_id_cache__valid_id(sbuild_id: *mut c_char) -> bool {
    let mut real_sbuild_id = [0 as c_char; SBUILD_ID_SIZE];
    let pathname = build_id_cache__origname(sbuild_id);
    let mut ret: c_int = 0;
    let mut result = false;

    if pathname.is_null() {
        return false;
    }

    if strcmp(pathname, DSO__NAME_KALLSYMS) == 0 {
        ret = sysfs__snprintf_build_id(cstr_lit(b"/\0"), real_sbuild_id.as_mut_ptr(), real_sbuild_id.len());
    } else if *pathname == '/' as c_char {
        ret = filename__snprintf_build_id(pathname, real_sbuild_id.as_mut_ptr(), real_sbuild_id.len());
    } else {
        ret = -EINVAL; /* Should we support other special DSO cache? */
    }
    if ret >= 0 {
        result = strcmp(sbuild_id, real_sbuild_id.as_ptr()) == 0;
    }
    free(pathname as *mut c_void);

    result
}

unsafe fn build_id_cache__basename(is_kallsyms: bool, is_vdso: bool, is_debug: bool) -> *const c_char {
    if is_kallsyms {
        cstr_lit(b"kallsyms\0")
    } else if is_vdso {
        cstr_lit(b"vdso\0")
    } else if is_debug {
        cstr_lit(b"debug\0")
    } else {
        cstr_lit(b"elf\0")
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __dso__build_id_filename(dso: *const dso, mut bf: *mut c_char, size: size_t, is_debug: bool, is_kallsyms: bool) -> *mut c_char {
    let is_vdso = dso__is_vdso(dso as *mut dso);
    let mut sbuild_id = [0 as c_char; SBUILD_ID_SIZE];
    let alloc = bf.is_null();
    let ret: c_int;

    if !dso__has_build_id(dso) {
        return ptr::null_mut();
    }

    build_id__snprintf(dso__bid(dso), sbuild_id.as_mut_ptr(), sbuild_id.len());
    let linkname = build_id_cache__linkname(sbuild_id.as_ptr(), ptr::null_mut(), 0);
    if linkname.is_null() {
        return ptr::null_mut();
    }

    /* Check if old style build_id cache */
    if is_regular_file(linkname) {
        ret = asnprintf(&mut bf, size, cstr_lit(b"%s\0"), linkname, ptr::null(), ptr::null());
    } else {
        ret = asnprintf(&mut bf, size, cstr_lit(b"%s/%s\0"), linkname, build_id_cache__basename(is_kallsyms, is_vdso, is_debug), ptr::null());
    }
    if ret < 0 || (!alloc && size < ret as c_uint as size_t) {
        bf = ptr::null_mut();
    }
    free(linkname as *mut c_void);

    bf
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dso__build_id_filename(dso: *const dso, bf: *mut c_char, size: size_t, is_debug: bool) -> *mut c_char {
    let is_kallsyms = dso__is_kallsyms(dso as *mut dso);

    __dso__build_id_filename(dso, bf, size, is_debug, is_kallsyms)
}

unsafe fn write_buildid(name: *const c_char, name_len: size_t, bid: *mut build_id, pid: pid_t, mut misc: u16, fd: *mut feat_fd) -> c_int {
    let mut b: perf_record_header_build_id = mem::zeroed();
    let len = PERF_ALIGN(name_len + 1, mem::size_of::<u64>());

    memcpy(b.data.as_mut_ptr() as *mut c_void, (*bid).data.as_ptr() as *const c_void, (*bid).size);
    b.size = (*bid).size as u8;
    misc |= PERF_RECORD_MISC_BUILD_ID_SIZE;
    b.pid = pid;
    b.header.misc = misc;
    b.header.size = (mem::size_of::<perf_record_header_build_id>() + len) as u16;

    let err = do_write(fd, &b as *const _ as *const c_void, mem::size_of::<perf_record_header_build_id>());
    if err < 0 {
        return err;
    }

    write_padded(fd, name, name_len + 1, len)
}

unsafe extern "C" fn machine__write_buildid_table_cb(dso: *mut dso, data: *mut c_void) -> c_int {
    let args = data as *mut machine__write_buildid_table_cb_args;
    let name: *const c_char;
    let name_len: size_t;

    if !dso__has_build_id(dso) {
        return 0;
    }

    if !dso__hit(dso) && !dso__is_vdso(dso) {
        return 0;
    }

    if dso__is_vdso(dso) {
        name = dso__short_name(dso);
        name_len = dso__short_name_len(dso);
    } else if dso__is_kcore(dso) {
        name = (*(*args).machine).mmap_name;
        name_len = strlen(name);
    } else {
        name = dso__long_name(dso);
        name_len = dso__long_name_len(dso);
    }

    let in_kernel = dso__kernel(dso) || is_kernel_module(name, PERF_RECORD_MISC_CPUMODE_UNKNOWN);
    write_buildid(
        name,
        name_len,
        &mut (*dso__id(dso)).build_id,
        (*(*args).machine).pid,
        if in_kernel { (*args).kmisc } else { (*args).umisc },
        (*args).fd,
    )
}

unsafe fn machine__write_buildid_table(machine: *mut machine, fd: *mut feat_fd) -> c_int {
    let mut args = machine__write_buildid_table_cb_args {
        machine,
        fd,
        kmisc: PERF_RECORD_MISC_KERNEL,
        umisc: PERF_RECORD_MISC_USER,
    };

    if !machine__is_host(machine) {
        args.kmisc = PERF_RECORD_MISC_GUEST_KERNEL;
        args.umisc = PERF_RECORD_MISC_GUEST_USER;
    }

    dsos__for_each_dso(&mut (*machine).dsos, Some(machine__write_buildid_table_cb), &mut args as *mut _ as *mut c_void)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_session__write_buildid_table(session: *mut perf_session, fd: *mut feat_fd) -> c_int {
    let mut err = machine__write_buildid_table(&mut (*session).machines.host, fd);

    if err != 0 {
        return err;
    }

    let mut nd = rb_first_cached(&mut (*session).machines.guests);
    while !nd.is_null() {
        let pos = nd as *mut machine;
        err = machine__write_buildid_table(pos, fd);
        if err != 0 {
            break;
        }
        nd = rb_next(nd);
    }
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn disable_buildid_cache() {
    no_buildid_cache = true;
}

unsafe extern "C" fn lsdir_bid_head_filter(_name: *const c_char, d: *mut dirent) -> bool {
    strlen((*d).d_name.as_ptr()) == 2
        && isxdigit((*d).d_name[0] as c_int) != 0
        && isxdigit((*d).d_name[1] as c_int) != 0
}

unsafe extern "C" fn lsdir_bid_tail_filter(_name: *const c_char, d: *mut dirent) -> bool {
    let mut i: c_int = 0;
    while isxdigit((*d).d_name[i as usize] as c_int) != 0 && i < SBUILD_ID_SIZE as c_int - 3 {
        i += 1;
    }
    i >= SBUILD_ID_MIN_SIZE as c_int - 3
        && i <= SBUILD_ID_SIZE as c_int - 3
        && (*d).d_name[i as usize] == 0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn build_id_cache__list_all(validonly: bool) -> *mut strlist {
    let mut toplist: *mut strlist;
    let mut linklist: *mut strlist = ptr::null_mut();
    let mut bidlist: *mut strlist;
    let mut topdir: *mut c_char = ptr::null_mut();
    let mut linkdir: *mut c_char = ptr::null_mut();
    let mut sbuild_id = [0 as c_char; SBUILD_ID_SIZE];

    /* for filename__ functions */
    if validonly {
        symbol__init(ptr::null_mut());
    }

    /* Open the top-level directory */
    if asprintf(&mut topdir, cstr_lit(b"%s/.build-id/\0"), buildid_dir) < 0 {
        return ptr::null_mut();
    }

    bidlist = strlist__new(ptr::null_mut(), ptr::null_mut());
    if bidlist.is_null() {
        free(topdir as *mut c_void);
        return bidlist;
    }

    toplist = lsdir(topdir, Some(lsdir_bid_head_filter));
    if toplist.is_null() {
        pr_debug(cstr_lit(b"Error in lsdir(%s): %d\n\0"), topdir, errno);
        /* If there is no buildid cache, return an empty list */
        if errno == ENOENT {
            free(topdir as *mut c_void);
            return bidlist;
        }
        strlist__delete(bidlist);
        free(topdir as *mut c_void);
        return ptr::null_mut();
    }

    let mut nd = strlist__first(toplist);
    while !nd.is_null() {
        if asprintf(&mut linkdir, cstr_lit(b"%s/%s\0"), topdir, (*nd).s) < 0 {
            strlist__delete(linklist);
            zfree(&mut linkdir);
            strlist__delete(bidlist);
            strlist__delete(toplist);
            free(topdir as *mut c_void);
            return ptr::null_mut();
        }
        /* Open the lower-level directory */
        linklist = lsdir(linkdir, Some(lsdir_bid_tail_filter));
        if linklist.is_null() {
            pr_debug(cstr_lit(b"Error in lsdir(%s): %d\n\0"), linkdir, errno);
            zfree(&mut linkdir);
            strlist__delete(bidlist);
            strlist__delete(toplist);
            free(topdir as *mut c_void);
            return ptr::null_mut();
        }
        let mut nd2 = strlist__first(linklist);
        while !nd2.is_null() {
            if snprintf(sbuild_id.as_mut_ptr(), SBUILD_ID_SIZE, cstr_lit(b"%s%s\0"), (*nd).s, (*nd2).s) > SBUILD_ID_SIZE as c_int - 1 {
                strlist__delete(linklist);
                zfree(&mut linkdir);
                strlist__delete(bidlist);
                strlist__delete(toplist);
                free(topdir as *mut c_void);
                return ptr::null_mut();
            }
            if !(validonly && !build_id_cache__valid_id(sbuild_id.as_mut_ptr())) {
                if strlist__add(bidlist, sbuild_id.as_ptr()) < 0 {
                    strlist__delete(linklist);
                    zfree(&mut linkdir);
                    strlist__delete(bidlist);
                    strlist__delete(toplist);
                    free(topdir as *mut c_void);
                    return ptr::null_mut();
                }
            }
            nd2 = strlist__next(nd2);
        }
        strlist__delete(linklist);
        linklist = ptr::null_mut();
        zfree(&mut linkdir);
        nd = strlist__next(nd);
    }

    strlist__delete(toplist);
    free(topdir as *mut c_void);
    bidlist
}

unsafe fn str_is_build_id(maybe_sbuild_id: *const c_char, len: size_t) -> bool {
    let mut i: size_t = 0;

    while i < len {
        if isxdigit(*maybe_sbuild_id.add(i) as c_int) == 0 {
            return false;
        }
        i += 1;
    }
    true
}

/* Return the valid complete build-id */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn build_id_cache__complement(incomplete_sbuild_id: *const c_char) -> *mut c_char {
    let mut cand: *mut str_node = ptr::null_mut();
    let mut sbuild_id: *mut c_char = ptr::null_mut();
    let len = strlen(incomplete_sbuild_id);

    if len >= SBUILD_ID_SIZE || !str_is_build_id(incomplete_sbuild_id, len) {
        return ptr::null_mut();
    }

    let bidlist = build_id_cache__list_all(true);
    if bidlist.is_null() {
        return ptr::null_mut();
    }

    let mut nd = strlist__first(bidlist);
    while !nd.is_null() {
        if strncmp((*nd).s, incomplete_sbuild_id, len) != 0 {
            nd = strlist__next(nd);
            continue;
        }
        if !cand.is_null() {
            /* Error: There are more than 2 candidates. */
            cand = ptr::null_mut();
            break;
        }
        cand = nd;
        nd = strlist__next(nd);
    }
    if !cand.is_null() {
        sbuild_id = strdup((*cand).s);
    }
    strlist__delete(bidlist);

    sbuild_id
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn build_id_cache__cachedir(
    sbuild_id: *const c_char,
    name: *const c_char,
    nsi: *mut nsinfo,
    is_kallsyms: bool,
    is_vdso: bool,
) -> *mut c_char {
    let mut realname: *mut c_char = ptr::null_mut();
    let mut filename: *mut c_char = ptr::null_mut();
    let slash = is_kallsyms || is_vdso;

    if !slash {
        realname = nsinfo__realpath(name, nsi);
    }

    if asprintf(
        &mut filename,
        cstr_lit(b"%s%s%s%s%s\0"),
        buildid_dir,
        if slash { cstr_lit(b"/\0") } else { cstr_lit(b"\0") },
        if is_vdso { DSO__NAME_VDSO } else if !realname.is_null() { realname } else { name },
        if !sbuild_id.is_null() { cstr_lit(b"/\0") } else { cstr_lit(b"\0") },
        if !sbuild_id.is_null() { sbuild_id } else { cstr_lit(b"\0") },
    ) < 0 {
        filename = ptr::null_mut();
    }

    free(realname as *mut c_void);
    filename
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn build_id_cache__list_build_ids(pathname: *const c_char, nsi: *mut nsinfo, result: *mut *mut strlist) -> c_int {
    let dir_name = build_id_cache__cachedir(ptr::null(), pathname, nsi, false, false);
    let mut ret = 0;

    if dir_name.is_null() {
        return -ENOMEM;
    }

    *result = lsdir(dir_name, Some(lsdir_no_dot_filter));
    if (*result).is_null() {
        ret = -errno;
    }
    free(dir_name as *mut c_void);

    ret
}

/* HAVE_LIBELF_SUPPORT && HAVE_GELF_GETNOTE_SUPPORT: real SDT cache support. */
unsafe fn build_id_cache__add_sdt_cache(sbuild_id: *const c_char, realname: *const c_char, nsi: *mut nsinfo) -> c_int {
    let cache = probe_cache__new(sbuild_id, nsi);
    let mut nsc: nscookie = mem::zeroed();
    if cache.is_null() {
        return -1;
    }

    nsinfo__mountns_enter(nsi, &mut nsc);
    let mut ret = probe_cache__scan_sdt(cache, realname);
    nsinfo__mountns_exit(&mut nsc);
    if ret >= 0 {
        pr_debug4(cstr_lit(b"Found %d SDTs in %s\n\0"), ret, realname);
        if probe_cache__commit(cache) < 0 {
            ret = -1;
        }
    }
    probe_cache__delete(cache);
    ret
}

unsafe fn build_id_cache__find_debug(sbuild_id: *const c_char, nsi: *mut nsinfo, root_dir: *const c_char) -> *mut c_char {
    let mut dirname = cstr_lit(b"/usr/lib/debug/.build-id/\0");
    let mut realname: *mut c_char = ptr::null_mut();
    let mut dirbuf = [0 as c_char; PATH_MAX];
    let debugfile = calloc(1, PATH_MAX) as *mut c_char;
    let mut nsc: nscookie = mem::zeroed();

    if debugfile.is_null() {
        return realname;
    }

    if !root_dir.is_null() {
        path__join(dirbuf.as_mut_ptr(), PATH_MAX, root_dir, dirname);
        dirname = dirbuf.as_ptr();
    }

    let len = __symbol__join_symfs(debugfile, PATH_MAX, dirname);
    snprintf(debugfile.add(len), PATH_MAX - len, cstr_lit(b"%.2s/%s.debug\0"), sbuild_id, sbuild_id.add(2));

    nsinfo__mountns_enter(nsi, &mut nsc);
    realname = realpath(debugfile, ptr::null_mut());
    if !realname.is_null() && access(realname, R_OK) != 0 {
        zfree(&mut realname);
    }
    nsinfo__mountns_exit(&mut nsc);

    /* HAVE_DEBUGINFOD_SUPPORT: optionally download debuginfo by build-id here. */

    free(debugfile as *mut c_void);
    realname
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn build_id_cache__add(
    sbuild_id: *const c_char,
    name: *const c_char,
    realname: *const c_char,
    nsi: *mut nsinfo,
    is_kallsyms: bool,
    is_vdso: bool,
    mut proper_name: *const c_char,
    root_dir: *const c_char,
) -> c_int {
    let size = PATH_MAX;
    let mut filename: *mut c_char = ptr::null_mut();
    let mut dir_name: *mut c_char;
    let mut linkname = calloc(1, size) as *mut c_char;
    let mut debugfile: *mut c_char = ptr::null_mut();
    let mut err = -1;

    if proper_name.is_null() {
        proper_name = name;
    }

    dir_name = build_id_cache__cachedir(sbuild_id, proper_name, nsi, is_kallsyms, is_vdso);
    if dir_name.is_null() {
        free(linkname as *mut c_void);
        return err;
    }

    /* Remove old style build-id cache */
    if is_regular_file(dir_name) {
        if unlink(dir_name) != 0 {
            free(filename as *mut c_void);
            free(debugfile as *mut c_void);
            free(dir_name as *mut c_void);
            free(linkname as *mut c_void);
            return err;
        }
    }

    if mkdir_p(dir_name, 0o755) != 0 {
        free(dir_name as *mut c_void);
        free(linkname as *mut c_void);
        return err;
    }

    /* Save the allocated buildid dirname */
    if asprintf(&mut filename, cstr_lit(b"%s/%s\0"), dir_name, build_id_cache__basename(is_kallsyms, is_vdso, false)) < 0 {
        filename = ptr::null_mut();
        free(dir_name as *mut c_void);
        free(linkname as *mut c_void);
        return err;
    }

    if access(filename, F_OK) != 0 {
        if is_kallsyms {
            if copyfile(cstr_lit(b"/proc/kallsyms\0"), filename) != 0 {
                free(filename as *mut c_void);
                free(dir_name as *mut c_void);
                free(linkname as *mut c_void);
                return err;
            }
        } else if !nsi.is_null() && nsinfo__need_setns(nsi) {
            if copyfile_ns(name, filename, nsi) != 0 {
                free(filename as *mut c_void);
                free(dir_name as *mut c_void);
                free(linkname as *mut c_void);
                return err;
            }
        } else if link(realname, filename) != 0 && errno != EEXIST {
            let mut f_stat: stat = mem::zeroed();

            if !(stat(name, &mut f_stat) < 0) && copyfile_mode(name, filename, f_stat.st_mode) != 0 {
                free(filename as *mut c_void);
                free(dir_name as *mut c_void);
                free(linkname as *mut c_void);
                return err;
            }
        }
    }

    /* Some binaries are stripped, but have .debug files with their symbol
     * table.  Check to see if we can locate one of those, since the elf
     * file itself may not be very useful to users of our tools without a
     * symtab.
     */
    if !is_kallsyms && !is_vdso && strncmp(cstr_lit(b".ko\0"), name.add(strlen(name) - 3), 3) != 0 {
        debugfile = build_id_cache__find_debug(sbuild_id, nsi, root_dir);
        if !debugfile.is_null() {
            zfree(&mut filename);
            if asprintf(&mut filename, cstr_lit(b"%s/%s\0"), dir_name, build_id_cache__basename(false, false, true)) < 0 {
                filename = ptr::null_mut();
                free(debugfile as *mut c_void);
                free(dir_name as *mut c_void);
                free(linkname as *mut c_void);
                return err;
            }
            if access(filename, F_OK) != 0 {
                if !nsi.is_null() && nsinfo__need_setns(nsi) {
                    if copyfile_ns(debugfile, filename, nsi) != 0 {
                        free(filename as *mut c_void);
                        free(debugfile as *mut c_void);
                        free(dir_name as *mut c_void);
                        free(linkname as *mut c_void);
                        return err;
                    }
                } else if link(debugfile, filename) != 0 && errno != EEXIST && copyfile(debugfile, filename) != 0 {
                    free(filename as *mut c_void);
                    free(debugfile as *mut c_void);
                    free(dir_name as *mut c_void);
                    free(linkname as *mut c_void);
                    return err;
                }
            }
        }
    }

    if build_id_cache__linkname(sbuild_id, linkname, size).is_null() {
        free(filename as *mut c_void);
        free(debugfile as *mut c_void);
        free(dir_name as *mut c_void);
        free(linkname as *mut c_void);
        return err;
    }
    let mut tmp = strrchr(linkname, '/' as c_int);
    *tmp = 0;

    if access(linkname, X_OK) != 0 && mkdir_p(linkname, 0o755) != 0 {
        free(filename as *mut c_void);
        free(debugfile as *mut c_void);
        free(dir_name as *mut c_void);
        free(linkname as *mut c_void);
        return err;
    }

    *tmp = '/' as c_char;
    tmp = dir_name.add(strlen(buildid_dir) - 5);
    memcpy(tmp as *mut c_void, cstr_lit(b"../..\0") as *const c_void, 5);

    if symlink(tmp, linkname) == 0 {
        err = 0;
    } else if errno == EEXIST {
        let mut path = [0 as c_char; PATH_MAX];
        let len = readlink(linkname, path.as_mut_ptr(), path.len() - 1);
        if len <= 0 {
            pr_err(cstr_lit(b"Can't read link: %s\n\0"), linkname);
            free(filename as *mut c_void);
            free(debugfile as *mut c_void);
            free(dir_name as *mut c_void);
            free(linkname as *mut c_void);
            return err;
        }
        path[len as usize] = 0;

        if strcmp(tmp, path.as_ptr()) != 0 {
            pr_debug(cstr_lit(b"build <%s> already linked to %s\n\0"), sbuild_id, linkname);
        }
        err = 0;
    }

    /* Update SDT cache : error is just warned */
    if !realname.is_null() && build_id_cache__add_sdt_cache(sbuild_id, realname, nsi) < 0 {
        pr_debug4(cstr_lit(b"Failed to update/scan SDT cache for %s\n\0"), realname);
    }

    free(filename as *mut c_void);
    free(debugfile as *mut c_void);
    free(dir_name as *mut c_void);
    free(linkname as *mut c_void);
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __build_id_cache__add_s(
    sbuild_id: *const c_char,
    name: *const c_char,
    nsi: *mut nsinfo,
    is_kallsyms: bool,
    is_vdso: bool,
    proper_name: *const c_char,
    root_dir: *const c_char,
) -> c_int {
    let mut realname: *mut c_char = ptr::null_mut();
    let mut err = -1;

    if !is_kallsyms {
        if !is_vdso {
            realname = nsinfo__realpath(name, nsi);
        } else {
            realname = realpath(name, ptr::null_mut());
        }
        if realname.is_null() {
            return err;
        }
    }

    err = build_id_cache__add(sbuild_id, name, realname, nsi, is_kallsyms, is_vdso, proper_name, root_dir);
    if !is_kallsyms {
        free(realname as *mut c_void);
    }
    err
}

unsafe fn build_id_cache__add_b(
    bid: *const build_id,
    name: *const c_char,
    nsi: *mut nsinfo,
    is_kallsyms: bool,
    is_vdso: bool,
    proper_name: *const c_char,
    root_dir: *const c_char,
) -> c_int {
    let mut sbuild_id = [0 as c_char; SBUILD_ID_SIZE];

    build_id__snprintf(bid, sbuild_id.as_mut_ptr(), sbuild_id.len());

    __build_id_cache__add_s(sbuild_id.as_ptr(), name, nsi, is_kallsyms, is_vdso, proper_name, root_dir)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn build_id_cache__cached(sbuild_id: *const c_char) -> bool {
    let mut ret = false;
    let filename = build_id_cache__linkname(sbuild_id, ptr::null_mut(), 0);

    if !filename.is_null() && access(filename, F_OK) == 0 {
        ret = true;
    }
    free(filename as *mut c_void);

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn build_id_cache__remove_s(sbuild_id: *const c_char) -> c_int {
    let size = PATH_MAX;
    let filename = calloc(1, size) as *mut c_char;
    let linkname = calloc(1, size) as *mut c_char;
    let mut err = -1;

    if filename.is_null() || linkname.is_null() {
        free(filename as *mut c_void);
        free(linkname as *mut c_void);
        return err;
    }

    if build_id_cache__linkname(sbuild_id, linkname, size).is_null() {
        free(filename as *mut c_void);
        free(linkname as *mut c_void);
        return err;
    }

    if access(linkname, F_OK) != 0 {
        free(filename as *mut c_void);
        free(linkname as *mut c_void);
        return err;
    }

    if readlink(linkname, filename, size - 1) < 0 {
        free(filename as *mut c_void);
        free(linkname as *mut c_void);
        return err;
    }

    if unlink(linkname) != 0 {
        free(filename as *mut c_void);
        free(linkname as *mut c_void);
        return err;
    }

    /*
     * Since the link is relative, we must make it absolute:
     */
    let tmp = strrchr(linkname, '/' as c_int).add(1);
    snprintf(tmp, size - tmp.offset_from(linkname) as usize, cstr_lit(b"%s\0"), filename);

    if rm_rf(linkname) != 0 {
        free(filename as *mut c_void);
        free(linkname as *mut c_void);
        return err;
    }

    err = 0;
    free(filename as *mut c_void);
    free(linkname as *mut c_void);
    err
}

unsafe fn filename__read_build_id_ns(filename: *const c_char, bid: *mut build_id, nsi: *mut nsinfo) -> c_int {
    let mut nsc: nscookie = mem::zeroed();
    let ret: c_int;

    nsinfo__mountns_enter(nsi, &mut nsc);
    ret = filename__read_build_id(filename, bid);
    nsinfo__mountns_exit(&mut nsc);

    ret
}

unsafe fn dso__build_id_mismatch(dso: *mut dso, name: *const c_char) -> bool {
    let mut bid: build_id = mem::zeroed();
    let mut ret = false;

    mutex_lock(dso__lock(dso));
    if filename__read_build_id_ns(name, &mut bid, dso__nsinfo(dso)) >= 0 {
        ret = !dso__build_id_equal(dso, &bid);
    }

    mutex_unlock(dso__lock(dso));

    ret
}

unsafe extern "C" fn dso__cache_build_id(dso: *mut dso, machine: *mut machine, _priv: *mut c_void) -> c_int {
    let mut is_kallsyms = dso__is_kallsyms(dso);
    let is_vdso = dso__is_vdso(dso);
    let mut name = dso__long_name(dso);
    let mut proper_name: *const c_char = ptr::null();
    let mut root_dir: *const c_char = ptr::null();
    let mut allocated_name: *mut c_char = ptr::null_mut();
    let mut ret: c_int = 0;

    if !dso__has_build_id(dso) || !dso__hit(dso) {
        return 0;
    }

    if dso__is_kcore(dso) {
        is_kallsyms = true;
        name = (*machine).mmap_name;
    }

    if !machine__is_host(machine) {
        if (*machine).root_dir[0] != 0 {
            root_dir = (*machine).root_dir.as_ptr();
            ret = asprintf(&mut allocated_name, cstr_lit(b"%s/%s\0"), root_dir, name);
            if ret < 0 {
                return ret;
            }
            proper_name = name;
            name = allocated_name;
        } else if is_kallsyms {
            /* Cannot get guest kallsyms */
            return 0;
        }
    }

    if !is_kallsyms && dso__build_id_mismatch(dso, name) {
        free(allocated_name as *mut c_void);
        return ret;
    }

    mutex_lock(dso__lock(dso));
    ret = build_id_cache__add_b(dso__bid(dso), name, dso__nsinfo(dso), is_kallsyms, is_vdso, proper_name, root_dir);
    mutex_unlock(dso__lock(dso));
    free(allocated_name as *mut c_void);
    ret
}

unsafe fn machines__for_each_dso(machines: *mut machines, fn_: machine__dso_t, priv_: *mut c_void) -> c_int {
    let mut ret = machine__for_each_dso(&mut (*machines).host, fn_, priv_);

    let mut nd = rb_first_cached(&mut (*machines).guests);
    while !nd.is_null() {
        let pos = nd as *mut machine;

        ret |= machine__for_each_dso(pos, fn_, priv_);
        nd = rb_next(nd);
    }
    if ret != 0 { -1 } else { 0 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __perf_session__cache_build_ids(session: *mut perf_session, fn_: machine__dso_t, priv_: *mut c_void) -> c_int {
    if no_buildid_cache {
        return 0;
    }

    if mkdir(buildid_dir, 0o755) != 0 && errno != EEXIST {
        return -1;
    }

    if machines__for_each_dso(&mut (*session).machines, fn_, priv_) != 0 { -1 } else { 0 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_session__cache_build_ids(session: *mut perf_session) -> c_int {
    __perf_session__cache_build_ids(session, Some(dso__cache_build_id), ptr::null_mut())
}

unsafe fn machine__read_build_ids(machine: *mut machine, with_hits: bool) -> bool {
    dsos__read_build_ids(&mut (*machine).dsos, with_hits)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_session__read_build_ids(session: *mut perf_session, with_hits: bool) -> bool {
    let mut ret = machine__read_build_ids(&mut (*session).machines.host, with_hits);

    let mut nd = rb_first_cached(&mut (*session).machines.guests);
    while !nd.is_null() {
        let pos = nd as *mut machine;
        ret |= machine__read_build_ids(pos, with_hits);
        nd = rb_next(nd);
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn build_id__init(bid: *mut build_id, data: *const u8, mut size: size_t) {
    if size > BUILD_ID_SIZE {
        pr_debug(cstr_lit(b"Truncating build_id size from %zd\n\0"), size);
        size = BUILD_ID_SIZE;
    }
    memcpy((*bid).data.as_mut_ptr() as *mut c_void, data as *const c_void, size);
    (*bid).size = size;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn build_id__is_defined(bid: *const build_id) -> bool {
    if !bid.is_null() && (*bid).size != 0 {
        !memchr_inv((*bid).data.as_ptr() as *const c_void, 0, (*bid).size).is_null()
    } else {
        false
    }
}
