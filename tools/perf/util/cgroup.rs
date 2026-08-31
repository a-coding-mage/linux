// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/cgroup.c.
// C includes translated as external declarations and dependency notes:
// <subcmd/parse-options.h>, "evsel.h", "cgroup.h", "evlist.h", "rblist.h",
// "metricgroup.h", "stat.h", <linux/zalloc.h>, <sys/types.h>,
// <sys/stat.h>, <sys/statfs.h>, <errno.h>, <fcntl.h>, <stdlib.h>,
// <string.h>, <api/fs/fs.h>, <ftw.h>, <regex.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type bool_ = bool;
type u64 = u64;
type uint64_t = u64;
type size_t = usize;

const PATH_MAX: usize = 4096;
const O_RDONLY: c_int = 0;
const AT_FDCWD: c_int = -100;
const FTW_D: c_int = 1;
const REG_NOSUB: c_int = 1;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

// #ifndef CGROUP2_SUPER_MAGIC
const CGROUP2_SUPER_MAGIC: c_long = 0x63677270;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct rb_node {
    pub __rb_parent_color: c_ulong,
    pub rb_right: *mut rb_node,
    pub rb_left: *mut rb_node,
}

#[repr(C)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}

#[repr(C)]
pub struct refcount_t {
    pub refs: c_int,
}

#[repr(C)]
pub struct cgroup {
    pub node: rb_node,
    pub refcnt: refcount_t,
    pub name: *mut c_char,
    pub fd: c_int,
    pub id: u64,
}

#[repr(C)]
pub struct evsel {
    pub core: list_head,
    pub cgrp: *mut cgroup,
    pub priv_: *mut c_void,
    pub metric_leader: *mut evsel,
    pub first_wildcard_match: *mut evsel,
}

#[repr(C)]
pub struct evlist_core {
    pub entries: list_head,
    pub nr_entries: c_int,
}

#[repr(C)]
pub struct evlist {
    pub core: evlist_core,
}

#[repr(C)]
pub struct rblist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct option {
    pub value: *mut c_void,
}

#[repr(C)]
pub struct rw_semaphore {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_env_cgroups {
    pub lock: rw_semaphore,
    pub tree: rb_root,
}

#[repr(C)]
pub struct perf_env {
    pub cgroups: perf_env_cgroups,
}

#[repr(C)]
pub struct stat {
    _private: [u8; 0],
}

#[repr(C)]
pub struct statfs {
    pub f_type: c_long,
}

#[repr(C)]
pub struct FTW {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regex_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file_handle {
    pub handle_bytes: c_uint,
    pub handle_type: c_int,
    pub f_handle: [c_uchar; 0],
}

type c_uchar = u8;

#[repr(C)]
struct cgroup_name {
    list: list_head,
    used: bool_,
    name: [c_char; 0],
}

#[repr(C)]
struct cgroup_file_handle {
    fh: file_handle,
    cgroup_id: uint64_t,
}

unsafe extern "C" {
    static mut stderr: *mut c_void;

    fn cgroupfs_find_mountpoint(buf: *mut c_char, maxlen: size_t, subsys: *const c_char) -> c_int;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn statfs(path: *const c_char, buf: *mut statfs) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn zalloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strndup(s: *const c_char, n: size_t) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strpbrk(s: *const c_char, accept: *const c_char) -> *mut c_char;
    fn nftw(
        dirpath: *const c_char,
        fn_: Option<
            unsafe extern "C" fn(*const c_char, *const stat, c_int, *mut FTW) -> c_int,
        >,
        nopenfd: c_int,
        flags: c_int,
    ) -> c_int;
    fn regcomp(preg: *mut regex_t, regex: *const c_char, cflags: c_int) -> c_int;
    fn regexec(
        preg: *const regex_t,
        string: *const c_char,
        nmatch: size_t,
        pmatch: *mut c_void,
        eflags: c_int,
    ) -> c_int;
    fn regfree(preg: *mut regex_t);
    fn name_to_handle_at(
        dirfd: c_int,
        pathname: *const c_char,
        handle: *mut file_handle,
        mount_id: *mut c_int,
        flags: c_int,
    ) -> c_int;

    fn refcount_set(r: *mut refcount_t, n: c_int);
    fn refcount_inc(r: *mut refcount_t);
    fn refcount_dec_and_test(r: *mut refcount_t) -> bool_;

    fn evlist__new() -> *mut evlist;
    fn evlist__put(evlist: *mut evlist);
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evlist__nr_entries(evlist: *mut evlist) -> c_int;
    fn evlist__splice_list_tail(evlist: *mut evlist, list: *mut list_head);
    fn evlist__add(evlist: *mut evlist, evsel: *mut evsel);
    fn evlist__metric_events(evlist: *mut evlist) -> *mut rblist;

    fn evsel__clone(evsel: *mut evsel) -> *mut evsel;
    fn evsel__leader(evsel: *mut evsel) -> *mut evsel;
    fn evsel__set_leader(evsel: *mut evsel, leader: *mut evsel);

    fn metricgroup__rblist_init(metric_events: *mut rblist);
    fn metricgroup__rblist_exit(metric_events: *mut rblist);
    fn metricgroup__copy_metric_events(
        tmp_list: *mut evlist,
        cgrp: *mut cgroup,
        dst: *mut rblist,
        src: *mut rblist,
    ) -> c_int;

    fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, rb_link: *mut *mut rb_node);
    fn rb_insert_color(node: *mut rb_node, root: *mut rb_root);
    fn rb_first(root: *mut rb_root) -> *mut rb_node;
    fn rb_erase(node: *mut rb_node, root: *mut rb_root);

    fn down_write(sem: *mut rw_semaphore);
    fn up_write(sem: *mut rw_semaphore);
    fn down_read(sem: *mut rw_semaphore);
    fn up_read(sem: *mut rw_semaphore);
}

#[no_mangle]
pub static mut nr_cgroups: c_int = 0;
#[no_mangle]
pub static mut cgrp_event_expanded: bool_ = false;

/* used to match cgroup name with patterns */
static mut cgroup_list: list_head = list_head {
    next: ptr::null_mut(),
    prev: ptr::null_mut(),
};

#[inline]
unsafe fn list_init_once(head: *mut list_head) {
    if (*head).next.is_null() {
        (*head).next = head;
        (*head).prev = head;
    }
}

#[inline]
unsafe fn list_empty(head: *const list_head) -> bool {
    (*(head as *mut list_head)).next == head as *mut list_head
}

#[inline]
unsafe fn __list_add(new: *mut list_head, prev: *mut list_head, next: *mut list_head) {
    (*next).prev = new;
    (*new).next = next;
    (*new).prev = prev;
    (*prev).next = new;
}

#[inline]
unsafe fn list_add_tail(new: *mut list_head, head: *mut list_head) {
    __list_add(new, (*head).prev, head);
}

#[inline]
unsafe fn __list_del(prev: *mut list_head, next: *mut list_head) {
    (*next).prev = prev;
    (*prev).next = next;
}

#[inline]
unsafe fn list_del(entry: *mut list_head) {
    __list_del((*entry).prev, (*entry).next);
    (*entry).next = ptr::null_mut();
    (*entry).prev = ptr::null_mut();
}

#[inline]
unsafe fn zfree_char(pp: *mut *mut c_char) {
    if !(*pp).is_null() {
        free(*pp as *mut c_void);
        *pp = ptr::null_mut();
    }
}

#[inline]
unsafe fn cgroup_name_from_list(ptr: *mut list_head) -> *mut cgroup_name {
    ptr as *mut cgroup_name
}

#[inline]
unsafe fn cgroup_from_node(ptr: *mut rb_node) -> *mut cgroup {
    ptr as *mut cgroup
}

#[inline]
unsafe fn evsel_from_list(ptr: *mut list_head) -> *mut evsel {
    ptr as *mut evsel
}

#[inline]
unsafe fn evlist_for_each_entry<F: FnMut(*mut evsel)>(evlist: *mut evlist, mut f: F) {
    let head = &mut (*evlist__core(evlist)).entries as *mut list_head;
    let mut pos = (*head).next;
    while pos != head {
        let next = (*pos).next;
        f(evsel_from_list(pos));
        pos = next;
    }
}

#[inline]
unsafe fn rb_empty_root(root: *mut rb_root) -> bool {
    (*root).rb_node.is_null()
}

unsafe fn open_cgroup(name: *const c_char) -> c_int {
    let mut path = [0 as c_char; PATH_MAX + 1];
    let mut mnt = [0 as c_char; PATH_MAX + 1];
    let fd: c_int;

    if cgroupfs_find_mountpoint(
        mnt.as_mut_ptr(),
        PATH_MAX + 1,
        c"perf_event".as_ptr(),
    ) != 0
    {
        return -1;
    }

    scnprintf(
        path.as_mut_ptr(),
        PATH_MAX,
        c"%s/%s".as_ptr(),
        mnt.as_mut_ptr(),
        name,
    );

    fd = open(path.as_ptr(), O_RDONLY);
    if fd == -1 {
        fprintf(stderr, c"no access to cgroup %s\n".as_ptr(), path.as_ptr());
    }

    fd
}

// #ifdef HAVE_FILE_HANDLE
unsafe fn __read_cgroup_id(path: *const c_char) -> u64 {
    let mut handle: cgroup_file_handle = mem::zeroed();
    let mut mount_id: c_int = 0;

    handle.fh.handle_bytes = mem::size_of_val(&handle.cgroup_id) as c_uint;
    if name_to_handle_at(
        AT_FDCWD,
        path,
        &mut handle.fh,
        &mut mount_id,
        0,
    ) < 0
    {
        return !0u64;
    }

    handle.cgroup_id
}

#[no_mangle]
pub unsafe extern "C" fn read_cgroup_id(cgrp: *mut cgroup) -> c_int {
    let mut path = [0 as c_char; PATH_MAX + 1];
    let mut mnt = [0 as c_char; PATH_MAX + 1];

    if cgroupfs_find_mountpoint(
        mnt.as_mut_ptr(),
        PATH_MAX + 1,
        c"perf_event".as_ptr(),
    ) != 0
    {
        return -1;
    }

    scnprintf(
        path.as_mut_ptr(),
        PATH_MAX,
        c"%s/%s".as_ptr(),
        mnt.as_mut_ptr(),
        (*cgrp).name,
    );

    (*cgrp).id = __read_cgroup_id(path.as_ptr());
    0
}
// #else
// static inline u64 __read_cgroup_id(const char *path __maybe_unused) { return -1ULL; }
// #endif  /* HAVE_FILE_HANDLE */

#[no_mangle]
pub unsafe extern "C" fn cgroup_is_v2(subsys: *const c_char) -> c_int {
    let mut mnt = [0 as c_char; PATH_MAX + 1];
    let mut stbuf: statfs = mem::zeroed();

    if cgroupfs_find_mountpoint(mnt.as_mut_ptr(), PATH_MAX + 1, subsys) != 0 {
        return -1;
    }

    if statfs(mnt.as_ptr(), &mut stbuf) < 0 {
        return -1;
    }

    (stbuf.f_type == CGROUP2_SUPER_MAGIC) as c_int
}

unsafe fn evlist__find_cgroup(evlist: *mut evlist, str_: *const c_char) -> *mut cgroup {
    let mut found: *mut cgroup = ptr::null_mut();

    /*
     * check if cgrp is already defined, if so we reuse it
     */
    evlist_for_each_entry(evlist, |counter| unsafe {
        if found.is_null() {
            if (*counter).cgrp.is_null() {
                return;
            }
            if strcmp((*(*counter).cgrp).name, str_) == 0 {
                found = cgroup__get((*counter).cgrp);
            }
        }
    });

    found
}

#[no_mangle]
pub unsafe extern "C" fn cgroup__new(name: *const c_char, do_open: bool_) -> *mut cgroup {
    let cgroup = zalloc(mem::size_of::<cgroup>()) as *mut cgroup;

    if !cgroup.is_null() {
        refcount_set(&mut (*cgroup).refcnt, 1);

        (*cgroup).name = strdup(name);
        if (*cgroup).name.is_null() {
            free(cgroup as *mut c_void);
            return ptr::null_mut();
        }

        if do_open {
            (*cgroup).fd = open_cgroup(name);
            if (*cgroup).fd == -1 {
                zfree_char(&mut (*cgroup).name);
                free(cgroup as *mut c_void);
                return ptr::null_mut();
            }
        } else {
            (*cgroup).fd = -1;
        }
    }

    cgroup
}

#[no_mangle]
pub unsafe extern "C" fn evlist__findnew_cgroup(
    evlist: *mut evlist,
    name: *const c_char,
) -> *mut cgroup {
    let cgroup = evlist__find_cgroup(evlist, name);

    if !cgroup.is_null() {
        cgroup
    } else {
        cgroup__new(name, true)
    }
}

unsafe fn add_cgroup(evlist: *mut evlist, str_: *const c_char) -> c_int {
    let cgrp = evlist__findnew_cgroup(evlist, str_);
    let mut n: c_int;
    let mut found_counter: *mut evsel = ptr::null_mut();

    if cgrp.is_null() {
        return -1;
    }
    /*
     * find corresponding event
     * if add cgroup N, then need to find event N
     */
    n = 0;
    evlist_for_each_entry(evlist, |counter| unsafe {
        if found_counter.is_null() {
            if n == nr_cgroups {
                found_counter = counter;
            }
            n += 1;
        }
    });

    if found_counter.is_null() {
        cgroup__put(cgrp);
        return -1;
    }
    (*found_counter).cgrp = cgrp;
    0
}

unsafe fn cgroup__delete(cgroup: *mut cgroup) {
    if (*cgroup).fd >= 0 {
        close((*cgroup).fd);
    }
    zfree_char(&mut (*cgroup).name);
    free(cgroup as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn cgroup__put(cgrp: *mut cgroup) {
    if !cgrp.is_null() && refcount_dec_and_test(&mut (*cgrp).refcnt) {
        cgroup__delete(cgrp);
    }
}

#[no_mangle]
pub unsafe extern "C" fn cgroup__get(cgroup: *mut cgroup) -> *mut cgroup {
    if !cgroup.is_null() {
        refcount_inc(&mut (*cgroup).refcnt);
    }
    cgroup
}

unsafe fn evsel__set_default_cgroup(evsel: *mut evsel, cgroup: *mut cgroup) {
    if (*evsel).cgrp.is_null() {
        (*evsel).cgrp = cgroup__get(cgroup);
    }
}

#[no_mangle]
pub unsafe extern "C" fn evlist__set_default_cgroup(evlist: *mut evlist, cgroup: *mut cgroup) {
    evlist_for_each_entry(evlist, |evsel| unsafe {
        evsel__set_default_cgroup(evsel, cgroup);
    });
}

/* helper function for ftw() in match_cgroups and list_cgroups */
unsafe extern "C" fn add_cgroup_name(
    fpath: *const c_char,
    _sb: *const stat,
    typeflag: c_int,
    _ftwbuf: *mut FTW,
) -> c_int {
    let cn: *mut cgroup_name;

    if typeflag != FTW_D {
        return 0;
    }

    cn = malloc(mem::size_of::<cgroup_name>() + strlen(fpath) + 1) as *mut cgroup_name;
    if cn.is_null() {
        return -1;
    }

    (*cn).used = false;
    strcpy((*cn).name.as_mut_ptr(), fpath);

    list_init_once(&mut cgroup_list);
    list_add_tail(&mut (*cn).list, &mut cgroup_list);
    0
}

unsafe fn check_and_add_cgroup_name(fpath: *const c_char) -> c_int {
    let mut pos: *mut list_head;

    list_init_once(&mut cgroup_list);
    pos = cgroup_list.next;
    while pos != &mut cgroup_list {
        let cn = cgroup_name_from_list(pos);
        if strcmp((*cn).name.as_ptr(), fpath) == 0 {
            return 0;
        }
        pos = (*pos).next;
    }

    /* pretend if it's added by ftw() */
    add_cgroup_name(fpath, ptr::null(), FTW_D, ptr::null_mut())
}

unsafe fn release_cgroup_list() {
    list_init_once(&mut cgroup_list);
    while !list_empty(&cgroup_list) {
        let cn = cgroup_name_from_list(cgroup_list.next);
        list_del(&mut (*cn).list);
        free(cn as *mut c_void);
    }
}

/* collect given cgroups only */
unsafe fn list_cgroups(mut str_: *const c_char) -> c_int {
    let mut p: *const c_char;
    let mut e: *const c_char;
    let eos = str_.add(strlen(str_));
    let mut s: *mut c_char;

    /* use given name as is when no regex is given */
    loop {
        p = strchr(str_, ',' as c_int) as *const c_char;
        e = if !p.is_null() { p } else { eos };

        if e.offset_from(str_) != 0 {
            let ret: c_int;

            s = strndup(str_, e.offset_from(str_) as size_t);
            if s.is_null() {
                return -1;
            }

            ret = check_and_add_cgroup_name(s);
            free(s as *mut c_void);
            if ret < 0 {
                return -1;
            }
        } else if check_and_add_cgroup_name(c"/".as_ptr()) < 0 {
            return -1;
        }

        if p.is_null() {
            break;
        }
        str_ = p.add(1);
    }

    /* these groups will be used */
    list_init_once(&mut cgroup_list);
    let mut pos = cgroup_list.next;
    while pos != &mut cgroup_list {
        let cn = cgroup_name_from_list(pos);
        (*cn).used = true;
        pos = (*pos).next;
    }

    0
}

/* collect all cgroups first and then match with the pattern */
unsafe fn match_cgroups(mut str_: *const c_char) -> c_int {
    let mut mnt = [0 as c_char; PATH_MAX];
    let mut p: *const c_char;
    let mut e: *const c_char;
    let eos = str_.add(strlen(str_));
    let mut reg: regex_t = mem::zeroed();
    let prefix_len: c_int;
    let mut s: *mut c_char;

    if cgroupfs_find_mountpoint(mnt.as_mut_ptr(), mem::size_of_val(&mnt), c"perf_event".as_ptr())
        != 0
    {
        return -1;
    }

    /* cgroup_name will have a full path, skip the root directory */
    prefix_len = strlen(mnt.as_ptr()) as c_int;

    /* collect all cgroups in the cgroup_list */
    if nftw(mnt.as_ptr(), Some(add_cgroup_name), 20, 0) < 0 {
        return -1;
    }

    loop {
        p = strchr(str_, ',' as c_int) as *const c_char;
        e = if !p.is_null() { p } else { eos };

        /* allow empty cgroups, i.e., skip */
        if e.offset_from(str_) != 0 {
            /* termination added */
            s = strndup(str_, e.offset_from(str_) as size_t);
            if s.is_null() {
                return -1;
            }
            if regcomp(&mut reg, s, REG_NOSUB) != 0 {
                free(s as *mut c_void);
                return -1;
            }

            /* check cgroup name with the pattern */
            list_init_once(&mut cgroup_list);
            let mut pos = cgroup_list.next;
            while pos != &mut cgroup_list {
                let cn = cgroup_name_from_list(pos);
                let mut name = (*cn).name.as_mut_ptr().add(prefix_len as usize);

                if *name == '/' as c_char && *name.add(1) != 0 {
                    name = name.add(1);
                }
                if regexec(&reg, name, 0, ptr::null_mut(), 0) == 0 {
                    (*cn).used = true;
                }
                pos = (*pos).next;
            }
            regfree(&mut reg);
            free(s as *mut c_void);
        } else {
            /* first entry to root cgroup */
            list_init_once(&mut cgroup_list);
            let cn = cgroup_name_from_list(cgroup_list.next);
            (*cn).used = true;
        }

        if p.is_null() {
            break;
        }
        str_ = p.add(1);
    }
    prefix_len
}

#[no_mangle]
pub unsafe extern "C" fn parse_cgroups(
    opt: *const option,
    mut str_: *const c_char,
    _unset: c_int,
) -> c_int {
    let evlist = *((*opt).value as *mut *mut evlist);
    let mut cgrp: *mut cgroup = ptr::null_mut();
    let mut p: *const c_char;
    let mut e: *const c_char;
    let eos = str_.add(strlen(str_));
    let mut s: *mut c_char;
    let mut ret: c_int;
    let mut i: c_int;

    if list_empty(&(*evlist__core(evlist)).entries) {
        fprintf(stderr, c"must define events before cgroups\n".as_ptr());
        return -1;
    }

    loop {
        p = strchr(str_, ',' as c_int) as *const c_char;
        e = if !p.is_null() { p } else { eos };

        /* allow empty cgroups, i.e., skip */
        if e.offset_from(str_) != 0 {
            /* termination added */
            s = strndup(str_, e.offset_from(str_) as size_t);
            if s.is_null() {
                return -1;
            }
            ret = add_cgroup(evlist, s);
            free(s as *mut c_void);
            if ret != 0 {
                return -1;
            }
        }
        /* nr_cgroups is increased een for empty cgroups */
        nr_cgroups += 1;
        if p.is_null() {
            break;
        }
        str_ = p.add(1);
    }
    /* for the case one cgroup combine to multiple events */
    i = 0;
    if nr_cgroups == 1 {
        evlist_for_each_entry(evlist, |counter| unsafe {
            if i == 0 {
                cgrp = (*counter).cgrp;
            } else {
                (*counter).cgrp = cgrp;
                refcount_inc(&mut (*cgrp).refcnt);
            }
            i += 1;
        });
    }
    0
}

unsafe fn has_pattern_string(str_: *const c_char) -> bool_ {
    !strpbrk(str_, c"{}[]()|*+?^$".as_ptr()).is_null()
}

#[no_mangle]
pub unsafe extern "C" fn evlist__expand_cgroup(
    evlist: *mut evlist,
    str_: *const c_char,
    open_cgroup_: bool_,
) -> c_int {
    let mut orig_list: *mut evlist;
    let mut tmp_list: *mut evlist;
    let mut orig_metric_events: rblist;
    let mut cgrp: *mut cgroup = ptr::null_mut();
    let mut ret: c_int = -1;
    let prefix_len: c_int;

    if evlist__nr_entries(evlist) == 0 {
        fprintf(stderr, c"must define events before cgroups\n".as_ptr());
        return -EINVAL;
    }

    orig_list = evlist__new();
    tmp_list = evlist__new();
    if orig_list.is_null() || tmp_list.is_null() {
        fprintf(stderr, c"memory allocation failed\n".as_ptr());
        return -ENOMEM;
    }

    /* save original events and init evlist */
    evlist__splice_list_tail(orig_list, &mut (*evlist__core(evlist)).entries);
    (*evlist__core(evlist)).nr_entries = 0;

    orig_metric_events = ptr::read(evlist__metric_events(evlist));
    metricgroup__rblist_init(evlist__metric_events(evlist));

    if has_pattern_string(str_) {
        prefix_len = match_cgroups(str_);
    } else {
        prefix_len = list_cgroups(str_);
    }

    if prefix_len < 0 {
        goto_out_err(
            ret,
            orig_list,
            tmp_list,
            &mut orig_metric_events,
        );
        return ret;
    }

    list_init_once(&mut cgroup_list);
    let mut cn_pos = cgroup_list.next;
    while cn_pos != &mut cgroup_list {
        let cn = cgroup_name_from_list(cn_pos);
        let next_cn = (*cn_pos).next;
        let mut name: *mut c_char;

        if !(*cn).used {
            cn_pos = next_cn;
            continue;
        }

        /* cgroup_name might have a full path, skip the prefix */
        name = (*cn).name.as_mut_ptr().add(prefix_len as usize);
        if *name == '/' as c_char && *name.add(1) != 0 {
            name = name.add(1);
        }

        /* the cgroup can go away in the meantime */
        cgrp = cgroup__new(name, open_cgroup_);
        if cgrp.is_null() {
            cn_pos = next_cn;
            continue;
        }

        /* copy the list and set to the new cgroup. */
        let mut copy_failed = false;
        evlist_for_each_entry(orig_list, |pos| unsafe {
            if copy_failed {
                return;
            }
            let evsel = evsel__clone(pos);

            if evsel.is_null() {
                copy_failed = true;
                return;
            }

            /* stash the copy during the copying. */
            (*pos).priv_ = evsel as *mut c_void;
            cgroup__put((*evsel).cgrp);
            (*evsel).cgrp = cgroup__get(cgrp);

            evlist__add(tmp_list, evsel);
        });
        if copy_failed {
            goto_out_err(ret, orig_list, tmp_list, &mut orig_metric_events);
            return ret;
        }
        /* update leader information using stashed pointer to copy. */
        evlist_for_each_entry(orig_list, |pos| unsafe {
            let evsel = (*pos).priv_ as *mut evsel;

            if !evsel__leader(pos).is_null() {
                evsel__set_leader(evsel, (*evsel__leader(pos)).priv_ as *mut evsel);
            }

            if !(*pos).metric_leader.is_null() {
                (*evsel).metric_leader = (*(*pos).metric_leader).priv_ as *mut evsel;
            }

            if !(*pos).first_wildcard_match.is_null() {
                (*evsel).first_wildcard_match =
                    (*(*pos).first_wildcard_match).priv_ as *mut evsel;
            }
        });
        /* the stashed copy is no longer used. */
        evlist_for_each_entry(orig_list, |pos| unsafe {
            (*pos).priv_ = ptr::null_mut();
        });

        /* cgroup__new() has a refcount, release it here */
        cgroup__put(cgrp);
        nr_cgroups += 1;

        if metricgroup__copy_metric_events(
            tmp_list,
            cgrp,
            evlist__metric_events(evlist),
            &mut orig_metric_events,
        ) < 0
        {
            goto_out_err(ret, orig_list, tmp_list, &mut orig_metric_events);
            return ret;
        }

        evlist__splice_list_tail(evlist, &mut (*evlist__core(tmp_list)).entries);
        (*evlist__core(tmp_list)).nr_entries = 0;

        cn_pos = next_cn;
    }

    if list_empty(&(*evlist__core(evlist)).entries) {
        fprintf(stderr, c"no cgroup matched: %s\n".as_ptr(), str_);
        goto_out_err(ret, orig_list, tmp_list, &mut orig_metric_events);
        return ret;
    }

    ret = 0;
    cgrp_event_expanded = true;

    goto_out_err(ret, orig_list, tmp_list, &mut orig_metric_events);
    ret
}

unsafe fn goto_out_err(
    _ret: c_int,
    orig_list: *mut evlist,
    tmp_list: *mut evlist,
    orig_metric_events: *mut rblist,
) {
    evlist__put(orig_list);
    evlist__put(tmp_list);
    metricgroup__rblist_exit(orig_metric_events);
    release_cgroup_list();
}

unsafe fn __cgroup__findnew(
    root: *mut rb_root,
    id: uint64_t,
    create: bool_,
    path: *const c_char,
) -> *mut cgroup {
    let mut p = &mut (*root).rb_node as *mut *mut rb_node;
    let mut parent: *mut rb_node = ptr::null_mut();
    let mut cgrp: *mut cgroup;

    while !(*p).is_null() {
        parent = *p;
        cgrp = cgroup_from_node(parent);

        if (*cgrp).id == id {
            return cgrp;
        }

        if (*cgrp).id < id {
            p = &mut (*(*p)).rb_left;
        } else {
            p = &mut (*(*p)).rb_right;
        }
    }

    if !create {
        return ptr::null_mut();
    }

    cgrp = malloc(mem::size_of::<cgroup>()) as *mut cgroup;
    if cgrp.is_null() {
        return ptr::null_mut();
    }

    (*cgrp).name = strdup(path);
    if (*cgrp).name.is_null() {
        free(cgrp as *mut c_void);
        return ptr::null_mut();
    }

    (*cgrp).fd = -1;
    (*cgrp).id = id;
    refcount_set(&mut (*cgrp).refcnt, 1);

    rb_link_node(&mut (*cgrp).node, parent, p);
    rb_insert_color(&mut (*cgrp).node, root);

    cgrp
}

#[no_mangle]
pub unsafe extern "C" fn cgroup__findnew(
    env: *mut perf_env,
    id: uint64_t,
    path: *const c_char,
) -> *mut cgroup {
    let cgrp: *mut cgroup;

    down_write(&mut (*env).cgroups.lock);
    cgrp = __cgroup__findnew(&mut (*env).cgroups.tree, id, true, path);
    up_write(&mut (*env).cgroups.lock);
    cgrp
}

#[no_mangle]
pub unsafe extern "C" fn __cgroup__find(root: *mut rb_root, id: uint64_t) -> *mut cgroup {
    __cgroup__findnew(root, id, false, ptr::null())
}

#[no_mangle]
pub unsafe extern "C" fn cgroup__find(env: *mut perf_env, id: uint64_t) -> *mut cgroup {
    let cgrp: *mut cgroup;

    down_read(&mut (*env).cgroups.lock);
    cgrp = __cgroup__findnew(&mut (*env).cgroups.tree, id, false, ptr::null());
    up_read(&mut (*env).cgroups.lock);
    cgrp
}

#[no_mangle]
pub unsafe extern "C" fn perf_env__purge_cgroups(env: *mut perf_env) {
    let mut node: *mut rb_node;
    let mut cgrp: *mut cgroup;

    down_write(&mut (*env).cgroups.lock);
    while !rb_empty_root(&mut (*env).cgroups.tree) {
        node = rb_first(&mut (*env).cgroups.tree);
        cgrp = cgroup_from_node(node);

        rb_erase(node, &mut (*env).cgroups.tree);
        cgroup__put(cgrp);
    }
    up_write(&mut (*env).cgroups.lock);
}

#[no_mangle]
pub unsafe extern "C" fn read_all_cgroups(root: *mut rb_root) {
    let mut mnt = [0 as c_char; PATH_MAX];
    let prefix_len: c_int;

    if cgroupfs_find_mountpoint(mnt.as_mut_ptr(), mem::size_of_val(&mnt), c"perf_event".as_ptr())
        != 0
    {
        return;
    }

    /* cgroup_name will have a full path, skip the root directory */
    prefix_len = strlen(mnt.as_ptr()) as c_int;

    /* collect all cgroups in the cgroup_list */
    if nftw(mnt.as_ptr(), Some(add_cgroup_name), 20, 0) < 0 {
        return;
    }

    list_init_once(&mut cgroup_list);
    let mut pos = cgroup_list.next;
    while pos != &mut cgroup_list {
        let cn = cgroup_name_from_list(pos);
        let mut name: *const c_char;
        let cgrp_id: u64;

        /* cgroup_name might have a full path, skip the prefix */
        name = (*cn).name.as_ptr().add(prefix_len as usize);
        if *name == '\0' as c_char {
            name = c"/".as_ptr();
        }

        cgrp_id = __read_cgroup_id((*cn).name.as_ptr());
        __cgroup__findnew(root, cgrp_id, true, name);

        pos = (*pos).next;
    }

    release_cgroup_list();
}
