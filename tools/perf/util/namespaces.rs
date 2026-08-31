// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2017 Hari Bathini, IBM Corporation
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem;
use core::ptr;

// Translated from includes:
// "namespaces.h", "event.h", sys/types.h, sys/stat.h, fcntl.h, limits.h,
// sched.h, stdlib.h, stdio.h, string.h, unistd.h, asm/bug.h,
// linux/kernel.h, linux/zalloc.h.
// The following types, constants, macros, and external functions are supplied
// by those dependencies in the original C translation unit.

type pid_t = c_int;
type size_t = usize;
type ssize_t = isize;
type u64 = u64;
type bool_ = bool;

const PATH_MAX: usize = 4096;
const O_RDONLY: c_int = 0;
const CLONE_NEWNS: c_int = 0x0002_0000;

extern "C" {
    static NET_NS_INDEX: c_uint;
    static UTS_NS_INDEX: c_uint;
    static IPC_NS_INDEX: c_uint;
    static PID_NS_INDEX: c_uint;
    static USER_NS_INDEX: c_uint;
    static MNT_NS_INDEX: c_uint;
    static CGROUP_NS_INDEX: c_uint;
    static NR_NAMESPACES: c_uint;

    fn zalloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
    fn fclose(stream: *mut FILE) -> c_int;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn setns(fd: c_int, nstype: c_int) -> c_int;
    fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char;
    fn chdir(path: *const c_char) -> c_int;
    fn realpath(path: *const c_char, resolved_path: *mut c_char) -> *mut c_char;

    fn refcount_set(r: *mut refcount_t, n: c_int);
    fn refcount_read(r: *const refcount_t) -> c_int;
    fn refcount_inc(r: *mut refcount_t);
    fn refcount_dec_and_test(r: *mut refcount_t) -> bool_;
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stat {
    pub st_dev: u64,
    pub st_ino: u64,
    _rest: [u8; 0],
}

#[repr(C)]
pub struct refcount_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_ns_link_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_record_namespaces {
    pub nr_namespaces: u64,
    pub link_info: [perf_ns_link_info; 0],
}

#[repr(C)]
pub struct namespaces {
    pub end_time: i64,
    pub link_info: [perf_ns_link_info; 0],
}

#[repr(C)]
pub struct nsinfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nsinfo_rc {
    pub refcnt: refcount_t,
    pub pid: pid_t,
    pub tgid: pid_t,
    pub nstgid: pid_t,
    pub need_setns: bool_,
    pub in_pidns: bool_,
    pub mntns_path: *mut c_char,
}

#[repr(C)]
pub struct nscookie {
    pub oldns: c_int,
    pub newns: c_int,
    pub oldcwd: *mut c_char,
}

extern "C" {
    fn RC_CHK_ACCESS(nsi: *const nsinfo) -> *mut nsinfo_rc;
    fn ADD_RC_CHK(res: *mut *mut nsinfo, nsi: *mut nsinfo_rc) -> bool_;
    fn RC_CHK_FREE(nsi: *mut nsinfo);
    fn RC_CHK_GET(result: *mut *mut nsinfo, nsi: *mut nsinfo) -> bool_;
    fn RC_CHK_PUT(nsi: *mut nsinfo);
    fn zfree(ptr: *mut *mut c_char);
    fn WARN_ONCE(condition: bool_, fmt: *const c_char, ...);
    fn WARN_ON_ONCE(condition: c_int) -> bool_;
}

static mut perf_ns__names: [*const c_char; 7] = [
    b"net\0".as_ptr() as *const c_char,
    b"uts\0".as_ptr() as *const c_char,
    b"ipc\0".as_ptr() as *const c_char,
    b"pid\0".as_ptr() as *const c_char,
    b"user\0".as_ptr() as *const c_char,
    b"mnt\0".as_ptr() as *const c_char,
    b"cgroup\0".as_ptr() as *const c_char,
];

#[no_mangle]
pub unsafe extern "C" fn perf_ns__name(id: c_uint) -> *const c_char {
    if id as usize >= perf_ns__names.len() {
        return b"UNKNOWN\0".as_ptr() as *const c_char;
    }
    perf_ns__names[id as usize]
}

#[no_mangle]
pub unsafe extern "C" fn namespaces__new(event: *mut perf_record_namespaces) -> *mut namespaces {
    let mut namespaces_ptr: *mut namespaces;
    let link_info_size: u64 =
        (if !event.is_null() { (*event).nr_namespaces } else { NR_NAMESPACES as u64 })
            * mem::size_of::<perf_ns_link_info>() as u64;

    namespaces_ptr = zalloc(mem::size_of::<namespaces>() + link_info_size as usize) as *mut namespaces;
    if namespaces_ptr.is_null() {
        return ptr::null_mut();
    }

    (*namespaces_ptr).end_time = -1;

    if !event.is_null() {
        memcpy(
            (*namespaces_ptr).link_info.as_mut_ptr() as *mut c_void,
            (*event).link_info.as_ptr() as *const c_void,
            link_info_size as size_t,
        );
    }

    namespaces_ptr
}

#[no_mangle]
pub unsafe extern "C" fn namespaces__free(namespaces_ptr: *mut namespaces) {
    free(namespaces_ptr as *mut c_void);
}

unsafe extern "C" fn nsinfo__get_nspid(
    tgid: *mut pid_t,
    nstgid: *mut pid_t,
    in_pidns: *mut bool_,
    path: *const c_char,
) -> c_int {
    let mut f: *mut FILE = ptr::null_mut();
    let mut statln: *mut c_char = ptr::null_mut();
    let mut linesz: size_t = 0;
    let mut nspid: *mut c_char;

    f = fopen(path, b"r\0".as_ptr() as *const c_char);
    if f.is_null() {
        return -1;
    }

    while getline(&mut statln, &mut linesz, f) != -1 {
        /* Use tgid if CONFIG_PID_NS is not defined. */
        if !strstr(statln, b"Tgid:\0".as_ptr() as *const c_char).is_null() {
            *tgid = strtol(strrchr(statln, b'\t' as c_int), ptr::null_mut(), 10) as pid_t;
            *nstgid = *tgid;
        }

        if !strstr(statln, b"NStgid:\0".as_ptr() as *const c_char).is_null() {
            nspid = strrchr(statln, b'\t' as c_int);
            *nstgid = strtol(nspid, ptr::null_mut(), 10) as pid_t;
            /*
             * If innermost tgid is not the first, process is in a different
             * PID namespace.
             */
            *in_pidns = statln.add(mem::size_of_val(b"NStgid:\0") - 1) != nspid;
            break;
        }
    }

    fclose(f);
    free(statln as *mut c_void);
    0
}

#[no_mangle]
pub unsafe extern "C" fn nsinfo__init(nsi: *mut nsinfo) -> c_int {
    let mut oldns: [c_char; PATH_MAX] = [0; PATH_MAX];
    let mut spath: [c_char; PATH_MAX] = [0; PATH_MAX];
    let mut newns: *mut c_char = ptr::null_mut();
    let mut old_stat: stat = mem::zeroed();
    let mut new_stat: stat = mem::zeroed();
    let mut rv: c_int = -1;

    if snprintf(
        oldns.as_mut_ptr(),
        PATH_MAX,
        b"/proc/self/ns/mnt\0".as_ptr() as *const c_char,
    ) >= PATH_MAX as c_int
    {
        return rv;
    }

    if asprintf(
        &mut newns,
        b"/proc/%d/ns/mnt\0".as_ptr() as *const c_char,
        nsinfo__pid(nsi),
    ) == -1
    {
        return rv;
    }

    if stat(oldns.as_ptr(), &mut old_stat) < 0 {
        goto_out(&mut newns);
        return rv;
    }

    if stat(newns, &mut new_stat) < 0 {
        goto_out(&mut newns);
        return rv;
    }

    /* Check if the mount namespaces differ, if so then indicate that we
     * want to switch as part of looking up dso/map data.
     */
    if old_stat.st_ino != new_stat.st_ino {
        (*RC_CHK_ACCESS(nsi)).need_setns = true;
        (*RC_CHK_ACCESS(nsi)).mntns_path = newns;
        newns = ptr::null_mut();
    }

    /* If we're dealing with a process that is in a different PID namespace,
     * attempt to work out the innermost tgid for the process.
     */
    if snprintf(
        spath.as_mut_ptr(),
        PATH_MAX,
        b"/proc/%d/status\0".as_ptr() as *const c_char,
        nsinfo__pid(nsi),
    ) >= PATH_MAX as c_int
    {
        goto_out(&mut newns);
        return rv;
    }

    rv = nsinfo__get_nspid(
        &mut (*RC_CHK_ACCESS(nsi)).tgid,
        &mut (*RC_CHK_ACCESS(nsi)).nstgid,
        &mut (*RC_CHK_ACCESS(nsi)).in_pidns,
        spath.as_ptr(),
    );

    goto_out(&mut newns);
    rv
}

unsafe fn goto_out(newns: *mut *mut c_char) {
    free(*newns as *mut c_void);
}

unsafe extern "C" fn nsinfo__alloc() -> *mut nsinfo {
    let mut res: *mut nsinfo = ptr::null_mut();
    let mut nsi: *mut nsinfo_rc;

    nsi = calloc(1, mem::size_of::<nsinfo_rc>()) as *mut nsinfo_rc;
    if ADD_RC_CHK(&mut res, nsi) {
        refcount_set(&mut (*nsi).refcnt, 1);
    }

    res
}

#[no_mangle]
pub unsafe extern "C" fn nsinfo__new(pid: pid_t) -> *mut nsinfo {
    let mut nsi: *mut nsinfo;

    if pid == 0 {
        return ptr::null_mut();
    }

    nsi = nsinfo__alloc();
    if nsi.is_null() {
        return ptr::null_mut();
    }

    (*RC_CHK_ACCESS(nsi)).pid = pid;
    (*RC_CHK_ACCESS(nsi)).tgid = pid;
    (*RC_CHK_ACCESS(nsi)).nstgid = pid;
    nsinfo__clear_need_setns(nsi);
    (*RC_CHK_ACCESS(nsi)).in_pidns = false;
    /* Init may fail if the process exits while we're trying to look at its
     * proc information. In that case, save the pid but don't try to enter
     * the namespace.
     */
    if nsinfo__init(nsi) == -1 {
        nsinfo__clear_need_setns(nsi);
    }

    nsi
}

unsafe extern "C" fn nsinfo__mntns_path(nsi: *const nsinfo) -> *const c_char {
    (*RC_CHK_ACCESS(nsi)).mntns_path
}

#[no_mangle]
pub unsafe extern "C" fn nsinfo__copy(nsi: *const nsinfo) -> *mut nsinfo {
    let mut nnsi: *mut nsinfo;

    if nsi.is_null() {
        return ptr::null_mut();
    }

    nnsi = nsinfo__alloc();
    if nnsi.is_null() {
        return ptr::null_mut();
    }

    (*RC_CHK_ACCESS(nnsi)).pid = nsinfo__pid(nsi);
    (*RC_CHK_ACCESS(nnsi)).tgid = nsinfo__tgid(nsi);
    (*RC_CHK_ACCESS(nnsi)).nstgid = nsinfo__nstgid(nsi);
    (*RC_CHK_ACCESS(nnsi)).need_setns = nsinfo__need_setns(nsi);
    (*RC_CHK_ACCESS(nnsi)).in_pidns = nsinfo__in_pidns(nsi);
    if !nsinfo__mntns_path(nsi).is_null() {
        (*RC_CHK_ACCESS(nnsi)).mntns_path = strdup(nsinfo__mntns_path(nsi));
        if (*RC_CHK_ACCESS(nnsi)).mntns_path.is_null() {
            nsinfo__put(nnsi);
            return ptr::null_mut();
        }
    }

    nnsi
}

unsafe extern "C" fn nsinfo__refcnt(nsi: *mut nsinfo) -> *mut refcount_t {
    &mut (*RC_CHK_ACCESS(nsi)).refcnt
}

unsafe extern "C" fn nsinfo__delete(nsi: *mut nsinfo) {
    if !nsi.is_null() {
        WARN_ONCE(
            refcount_read(nsinfo__refcnt(nsi)) != 0,
            b"nsinfo refcnt unbalanced\n\0".as_ptr() as *const c_char,
        );
        zfree(&mut (*RC_CHK_ACCESS(nsi)).mntns_path);
        RC_CHK_FREE(nsi);
    }
}

#[no_mangle]
pub unsafe extern "C" fn nsinfo__get(nsi: *mut nsinfo) -> *mut nsinfo {
    let mut result: *mut nsinfo = ptr::null_mut();

    if RC_CHK_GET(&mut result, nsi) {
        refcount_inc(nsinfo__refcnt(nsi));
    }

    result
}

#[no_mangle]
pub unsafe extern "C" fn nsinfo__put(nsi: *mut nsinfo) {
    if !nsi.is_null() && refcount_dec_and_test(nsinfo__refcnt(nsi)) {
        nsinfo__delete(nsi);
    } else {
        RC_CHK_PUT(nsi);
    }
}

#[no_mangle]
pub unsafe extern "C" fn nsinfo__need_setns(nsi: *const nsinfo) -> bool_ {
    (*RC_CHK_ACCESS(nsi)).need_setns
}

#[no_mangle]
pub unsafe extern "C" fn nsinfo__clear_need_setns(nsi: *mut nsinfo) {
    (*RC_CHK_ACCESS(nsi)).need_setns = false;
}

#[no_mangle]
pub unsafe extern "C" fn nsinfo__tgid(nsi: *const nsinfo) -> pid_t {
    (*RC_CHK_ACCESS(nsi)).tgid
}

#[no_mangle]
pub unsafe extern "C" fn nsinfo__nstgid(nsi: *const nsinfo) -> pid_t {
    (*RC_CHK_ACCESS(nsi)).nstgid
}

#[no_mangle]
pub unsafe extern "C" fn nsinfo__pid(nsi: *const nsinfo) -> pid_t {
    (*RC_CHK_ACCESS(nsi)).pid
}

#[no_mangle]
pub unsafe extern "C" fn nsinfo__in_pidns(nsi: *const nsinfo) -> bool_ {
    (*RC_CHK_ACCESS(nsi)).in_pidns
}

#[no_mangle]
pub unsafe extern "C" fn nsinfo__set_in_pidns(nsi: *mut nsinfo) {
    (*RC_CHK_ACCESS(nsi)).in_pidns = true;
}

#[no_mangle]
pub unsafe extern "C" fn nsinfo__mountns_enter(nsi: *mut nsinfo, nc: *mut nscookie) {
    let mut curpath: [c_char; PATH_MAX] = [0; PATH_MAX];
    let mut oldns: c_int = -1;
    let mut newns: c_int = -1;
    let mut oldcwd: *mut c_char = ptr::null_mut();

    if nc.is_null() {
        return;
    }

    (*nc).oldns = -1;
    (*nc).newns = -1;

    if nsi.is_null() || !nsinfo__need_setns(nsi) {
        return;
    }

    if getcwd(curpath.as_mut_ptr(), curpath.len()).is_null() {
        return;
    }

    oldcwd = strdup(curpath.as_ptr());
    if oldcwd.is_null() {
        return;
    }

    oldns = open(b"/proc/self/ns/mnt\0".as_ptr() as *const c_char, O_RDONLY);
    if oldns < 0 {
        free(oldcwd as *mut c_void);
        return;
    }

    newns = open(nsinfo__mntns_path(nsi), O_RDONLY);
    if newns < 0 {
        free(oldcwd as *mut c_void);
        if oldns > -1 {
            close(oldns);
        }
        return;
    }

    if setns(newns, CLONE_NEWNS) < 0 {
        free(oldcwd as *mut c_void);
        if oldns > -1 {
            close(oldns);
        }
        if newns > -1 {
            close(newns);
        }
        return;
    }

    (*nc).oldcwd = oldcwd;
    (*nc).oldns = oldns;
    (*nc).newns = newns;
}

#[no_mangle]
pub unsafe extern "C" fn nsinfo__mountns_exit(nc: *mut nscookie) {
    if nc.is_null() || (*nc).oldns == -1 || (*nc).newns == -1 || (*nc).oldcwd.is_null() {
        return;
    }

    setns((*nc).oldns, CLONE_NEWNS);

    if !(*nc).oldcwd.is_null() {
        WARN_ON_ONCE(chdir((*nc).oldcwd));
        zfree(&mut (*nc).oldcwd);
    }

    if (*nc).oldns > -1 {
        close((*nc).oldns);
        (*nc).oldns = -1;
    }

    if (*nc).newns > -1 {
        close((*nc).newns);
        (*nc).newns = -1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn nsinfo__realpath(path: *const c_char, nsi: *mut nsinfo) -> *mut c_char {
    let mut rpath: *mut c_char;
    let mut nsc: nscookie = mem::zeroed();

    nsinfo__mountns_enter(nsi, &mut nsc);
    rpath = realpath(path, ptr::null_mut());
    nsinfo__mountns_exit(&mut nsc);

    rpath
}

#[no_mangle]
pub unsafe extern "C" fn nsinfo__stat(
    filename: *const c_char,
    st: *mut stat,
    nsi: *mut nsinfo,
) -> c_int {
    let mut ret: c_int;
    let mut nsc: nscookie = mem::zeroed();

    nsinfo__mountns_enter(nsi, &mut nsc);
    ret = stat(filename, st);
    nsinfo__mountns_exit(&mut nsc);

    ret
}

#[no_mangle]
pub unsafe extern "C" fn nsinfo__is_in_root_namespace() -> bool_ {
    let mut tgid: pid_t = 0;
    let mut nstgid: pid_t = 0;
    let mut in_pidns: bool_ = false;

    nsinfo__get_nspid(
        &mut tgid,
        &mut nstgid,
        &mut in_pidns,
        b"/proc/self/status\0".as_ptr() as *const c_char,
    );
    !in_pidns
}
