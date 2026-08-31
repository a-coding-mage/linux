// SPDX-License-Identifier: GPL-2.0-only
/*
 * Manage printing of source lines
 * Copyright (c) 2017, Intel Corporation.
 * Author: Andi Kleen
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

const MAXSRCCACHE: c_long = 32 * 1024 * 1024;
const MAXSRCFILES: c_int = 64;
const SRC_HTAB_SZ: usize = 64;

const O_RDONLY: c_int = 0;
const PROT_READ: c_int = 0x1;
const MAP_SHARED: c_int = 0x01;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

#[repr(C)]
pub struct hlist_node {
    pub next: *mut hlist_node,
    pub pprev: *mut *mut hlist_node,
}

#[repr(C)]
pub struct hlist_head {
    pub first: *mut hlist_node,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct stat {
    pub _private: [u8; 0],
}

unsafe extern "C" {
    static page_size: c_ulong;

    fn memchr(s: *const c_void, c: c_int, n: usize) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn malloc(size: usize) -> *mut c_void;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;

    fn str_hash(s: *const c_char) -> usize;
    fn pr_debug(fmt: *const c_char, ...);

    fn list_del_init(entry: *mut list_head);
    fn hlist_del(n: *mut hlist_node);
    fn list_move(list: *mut list_head, head: *mut list_head);
    fn list_empty(head: *const list_head) -> c_int;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn hlist_add_head(n: *mut hlist_node, h: *mut hlist_head);
    fn zfree(ptr: *mut *mut c_void);
}

#[repr(C)]
struct srcfile {
    hash_nd: hlist_node,
    nd: list_head,
    fn_: *mut c_char,
    lines: *mut *mut c_char,
    map: *mut c_char,
    numlines: c_uint,
    maplen: usize,
}

static mut SRCFILE_HTAB: [hlist_head; SRC_HTAB_SZ] = [const { hlist_head { first: core::ptr::null_mut() } }; SRC_HTAB_SZ];
static mut SRCFILE_LIST: list_head = list_head {
    next: core::ptr::addr_of_mut!(SRCFILE_LIST),
    prev: core::ptr::addr_of_mut!(SRCFILE_LIST),
};
static mut MAP_TOTAL_SZ: c_long = 0;
static mut NUM_SRCFILES: c_int = 0;

unsafe fn countlines(map: *mut c_char, maplen: c_int) -> c_int {
    let mut numl: c_int;
    let end = unsafe { map.add(maplen as usize) };
    let mut p = map;

    if maplen == 0 {
        return 0;
    }
    numl = 0;
    while p < end {
        let found = unsafe { memchr(p as *const c_void, '\n' as c_int, end.offset_from(p) as usize) };
        p = found as *mut c_char;
        if p.is_null() {
            break;
        }
        numl += 1;
        p = unsafe { p.add(1) };
    }
    if p < end {
        numl += 1;
    }
    numl
}

unsafe fn fill_lines(lines: *mut *mut c_char, maxline: c_int, map: *mut c_char, maplen: c_int) {
    let mut l: c_int;
    let end = unsafe { map.add(maplen as usize) };
    let mut p = map;

    if maplen == 0 || maxline == 0 {
        return;
    }
    l = 0;
    unsafe {
        *lines.add(l as usize) = map;
    }
    l += 1;
    while p < end {
        let found = unsafe { memchr(p as *const c_void, '\n' as c_int, end.offset_from(p) as usize) };
        p = found as *mut c_char;
        if p.is_null() {
            break;
        }
        if l >= maxline {
            return;
        }
        p = unsafe { p.add(1) };
        unsafe {
            *lines.add(l as usize) = p;
        }
        l += 1;
    }
    if p < end {
        unsafe {
            *lines.add(l as usize) = p;
        }
    }
}

unsafe fn free_srcfile(sf: *mut srcfile) {
    unsafe {
        list_del_init(&mut (*sf).nd);
        hlist_del(&mut (*sf).hash_nd);
        MAP_TOTAL_SZ -= (*sf).maplen as c_long;
        munmap((*sf).map as *mut c_void, (*sf).maplen);
        zfree(&mut (*sf).lines as *mut *mut *mut c_char as *mut *mut c_void);
        zfree(&mut (*sf).fn_ as *mut *mut c_char as *mut *mut c_void);
        free(sf as *mut c_void);
        NUM_SRCFILES -= 1;
    }
}

unsafe fn hlist_entry_from_hash_nd(node: *mut hlist_node) -> *mut srcfile {
    node.cast::<srcfile>()
}

unsafe fn list_entry_from_nd(node: *mut list_head) -> *mut srcfile {
    (node as *mut u8).sub(core::mem::offset_of!(srcfile, nd)).cast::<srcfile>()
}

unsafe fn find_srcfile(fn_: *mut c_char) -> *mut srcfile {
    let mut st: stat = stat { _private: [] };
    let mut h: *mut srcfile;
    let fd: c_int;
    let sz: c_ulong;
    let hval = unsafe { str_hash(fn_) % SRC_HTAB_SZ };

    h = unsafe { SRCFILE_HTAB[hval].first }.cast::<srcfile>();
    while !h.is_null() {
        if unsafe { strcmp(fn_, (*h).fn_) } == 0 {
            /* Move to front */
            unsafe {
                list_move(&mut (*h).nd, &raw mut SRCFILE_LIST);
            }
            return h;
        }
        h = unsafe {
            if (*h).hash_nd.next.is_null() {
                core::ptr::null_mut()
            } else {
                hlist_entry_from_hash_nd((*h).hash_nd.next)
            }
        };
    }

    /* Only prune if there is more than one entry */
    while unsafe {
        (NUM_SRCFILES > MAXSRCFILES || MAP_TOTAL_SZ > MAXSRCCACHE)
            && SRCFILE_LIST.next != &raw mut SRCFILE_LIST
    } {
        unsafe {
            assert!(list_empty(&raw const SRCFILE_LIST) == 0);
            h = list_entry_from_nd(SRCFILE_LIST.prev);
            free_srcfile(h);
        }
    }

    fd = unsafe { open(fn_, O_RDONLY) };
    if fd < 0 || unsafe { fstat(fd, &mut st) } < 0 {
        unsafe {
            pr_debug(c"cannot open source file %s\n".as_ptr(), fn_);
        }
        return core::ptr::null_mut();
    }

    h = unsafe { malloc(core::mem::size_of::<srcfile>()) as *mut srcfile };
    if h.is_null() {
        return core::ptr::null_mut();
    }

    unsafe {
        (*h).fn_ = strdup(fn_);
    }
    if unsafe { (*h).fn_.is_null() } {
        unsafe {
            free(h as *mut c_void);
        }
        return core::ptr::null_mut();
    }

    unsafe {
        /* st.st_size is used here in the C source; the concrete stat layout is provided externally. */
        (*h).maplen = *(&st as *const stat as *const usize);
        sz = ((*h).maplen as c_ulong + page_size - 1) & !(page_size - 1);
        (*h).map = mmap(
            core::ptr::null_mut(),
            sz as usize,
            PROT_READ,
            MAP_SHARED,
            fd,
            0,
        ) as *mut c_char;
        close(fd);
    }
    if unsafe { (*h).map as *mut c_void == MAP_FAILED } {
        unsafe {
            pr_debug(c"cannot mmap source file %s\n".as_ptr(), fn_);
            zfree(&mut (*h).fn_ as *mut *mut c_char as *mut *mut c_void);
            free(h as *mut c_void);
        }
        return core::ptr::null_mut();
    }
    unsafe {
        (*h).numlines = countlines((*h).map, (*h).maplen as c_int) as c_uint;
        (*h).lines = calloc((*h).numlines as usize, core::mem::size_of::<*mut c_char>()) as *mut *mut c_char;
    }
    if unsafe { (*h).lines.is_null() } {
        unsafe {
            munmap((*h).map as *mut c_void, sz as usize);
            zfree(&mut (*h).fn_ as *mut *mut c_char as *mut *mut c_void);
            free(h as *mut c_void);
        }
        return core::ptr::null_mut();
    }
    unsafe {
        fill_lines((*h).lines, (*h).numlines as c_int, (*h).map, (*h).maplen as c_int);
        list_add(&mut (*h).nd, &raw mut SRCFILE_LIST);
        hlist_add_head(&mut (*h).hash_nd, &raw mut SRCFILE_HTAB[hval]);
        MAP_TOTAL_SZ += (*h).maplen as c_long;
        NUM_SRCFILES += 1;
    }
    h
}

/* Result is not 0 terminated */
#[no_mangle]
pub unsafe extern "C" fn find_sourceline(
    fn_: *mut c_char,
    mut line: c_uint,
    lenp: *mut c_int,
) -> *mut c_char {
    let l: *mut c_char;
    let p: *mut c_char;
    let sf = unsafe { find_srcfile(fn_) };
    if sf.is_null() {
        return core::ptr::null_mut();
    }
    line -= 1;
    if unsafe { line >= (*sf).numlines } {
        return core::ptr::null_mut();
    }
    l = unsafe { *(*sf).lines.add(line as usize) };
    if l.is_null() {
        return core::ptr::null_mut();
    }
    p = unsafe {
        memchr(
            l as *const c_void,
            '\n' as c_int,
            (*sf).map.add((*sf).maplen).offset_from(l) as usize,
        ) as *mut c_char
    };
    unsafe {
        *lenp = p.offset_from(l) as c_int;
    }
    l
}
