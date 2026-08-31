// SPDX-License-Identifier: GPL-2.0
// C dependencies from original source:
// linux/stringify.h, sys/types.h, sys/stat.h, fcntl.h, stdio.h, stdlib.h,
// string.h, and "fs.h".

use core::ffi::{c_char, c_int, c_void};

type SizeT = usize;

const PATH_MAX: usize = 4096;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct cgroupfs_cache_entry {
    subsys: [c_char; 32],
    mountpoint: [c_char; PATH_MAX],
}

unsafe extern "C" {
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn getline(lineptr: *mut *mut c_char, n: *mut SizeT, stream: *mut FILE) -> isize;
    fn free(ptr: *mut c_void);
    fn calloc(nmemb: SizeT, size: SizeT) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> SizeT;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: SizeT) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: SizeT) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
}

/* just cache last used one */
static mut cached: *mut cgroupfs_cache_entry = core::ptr::null_mut();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cgroupfs_find_mountpoint(
    buf: *mut c_char,
    maxlen: SizeT,
    subsys: *const c_char,
) -> c_int {
    let mut fp: *mut FILE;
    let mut line: *mut c_char = core::ptr::null_mut();
    let mut len: SizeT = 0;
    let mut p: *mut c_char;
    let mut path: *mut c_char;
    let mut mountpoint: [c_char; PATH_MAX] = [0; PATH_MAX];

    if !cached.is_null() && strcmp((*cached).subsys.as_ptr(), subsys) == 0 {
        if strlen((*cached).mountpoint.as_ptr()) < maxlen {
            strcpy(buf, (*cached).mountpoint.as_ptr());
            return 0;
        }
        return -1;
    }

    fp = fopen(c"/proc/mounts".as_ptr(), c"r".as_ptr());
    if fp.is_null() {
        return -1;
    }

    /*
     * in order to handle split hierarchy, we need to scan /proc/mounts
     * and inspect every cgroupfs mount point to find one that has
     * the given subsystem.  If we found v1, just use it.  If not we can
     * use v2 path as a fallback.
     */
    mountpoint[0] = b'\0' as c_char;

    /*
     * The /proc/mounts has the follow format:
     *
     *   <devname> <mount point> <fs type> <options> ...
     *
     */
    while getline(&mut line, &mut len, fp) != -1 {
        /* skip devname */
        p = strchr(line, b' ' as c_int);
        if p.is_null() {
            continue;
        }

        /* save the mount point */
        p = p.add(1);
        path = p;
        p = strchr(p, b' ' as c_int);
        if p.is_null() {
            continue;
        }

        *p = b'\0' as c_char;
        p = p.add(1);

        /* check filesystem type */
        if strncmp(p, c"cgroup".as_ptr(), 6) != 0 {
            continue;
        }

        if *p.add(6) == b'2' as c_char {
            /* save cgroup v2 path */
            strcpy(mountpoint.as_mut_ptr(), path);
            continue;
        }

        /* now we have cgroup v1, check the options for subsystem */
        p = p.add(7);

        p = strstr(p, subsys);
        if p.is_null() {
            continue;
        }

        /* sanity check: it should be separated by a space or a comma */
        if strchr(c" ,".as_ptr(), *p.offset(-1) as c_int).is_null()
            || strchr(c" ,".as_ptr(), *p.add(strlen(subsys)) as c_int).is_null()
        {
            continue;
        }

        strcpy(mountpoint.as_mut_ptr(), path);
        break;
    }
    free(line as *mut c_void);
    fclose(fp);

    if cached.is_null() {
        cached = calloc(
            1,
            core::mem::size_of::<cgroupfs_cache_entry>() as SizeT,
        ) as *mut cgroupfs_cache_entry;
    }

    if !cached.is_null() {
        strncpy(
            (*cached).subsys.as_mut_ptr(),
            subsys,
            core::mem::size_of_val(&(*cached).subsys) - 1,
        );
        strcpy((*cached).mountpoint.as_mut_ptr(), mountpoint.as_ptr());
    }

    if mountpoint[0] != 0 && strlen(mountpoint.as_ptr()) < maxlen {
        strcpy(buf, mountpoint.as_ptr());
        return 0;
    }
    -1
}
