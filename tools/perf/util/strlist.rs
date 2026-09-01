// SPDX-License-Identifier: GPL-2.0-only
/*
 * (c) 2009 Arnaldo Carvalho de Melo <acme@redhat.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const F_OK: c_int = 0;

#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rblist {
    pub node_cmp: Option<unsafe extern "C" fn(*mut rb_node, *const c_void) -> c_int>,
    pub node_new: Option<unsafe extern "C" fn(*mut rblist, *const c_void) -> *mut rb_node>,
    pub node_delete: Option<unsafe extern "C" fn(*mut rblist, *mut rb_node)>,
}

#[repr(C)]
pub struct str_node {
    pub rb_node: rb_node,
    pub s: *mut c_char,
}

#[repr(C)]
pub struct strlist {
    pub rblist: rblist,
    pub file_only: bool,
}

#[repr(C)]
pub struct strlist_config {
    pub dirname: *const c_char,
    pub file_only: bool,
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: c_int;

    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fclose(stream: *mut FILE) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;

    fn zfree(ptr: *mut *mut c_char);
    fn rblist__add_node(rblist: *mut rblist, entry: *const c_void) -> c_int;
    fn rblist__remove_node(rblist: *mut rblist, rb_node: *mut rb_node);
    fn rblist__find(rblist: *mut rblist, entry: *const c_void) -> *mut rb_node;
    fn rblist__init(rblist: *mut rblist);
    fn rblist__delete(rblist: *mut rblist);
    fn rblist__entry(rblist: *const rblist, idx: c_uint) -> *mut rb_node;
}

unsafe fn str_node_from_rb_node(rb_node: *mut rb_node) -> *mut str_node {
    rb_node as *mut str_node
}

unsafe extern "C" fn strlist__node_new(
    _rblist: *mut rblist,
    entry: *const c_void,
) -> *mut rb_node {
    let s = entry as *const c_char;
    let mut rc: *mut rb_node = ptr::null_mut();
    let snode = unsafe { malloc(mem::size_of::<str_node>()) as *mut str_node };

    if !snode.is_null() {
        unsafe {
            (*snode).s = strdup(s);
            if (*snode).s.is_null() {
                free(snode as *mut c_void);
                return ptr::null_mut();
            }
            rc = &mut (*snode).rb_node;
        }
    }

    rc
}

unsafe fn str_node__delete(snode: *mut str_node) {
    unsafe {
        zfree(&mut (*snode).s);
        free(snode as *mut c_void);
    }
}

unsafe extern "C" fn strlist__node_delete(_rblist: *mut rblist, rb_node: *mut rb_node) {
    let snode = unsafe { str_node_from_rb_node(rb_node) };

    unsafe {
        str_node__delete(snode);
    }
}

unsafe extern "C" fn strlist__node_cmp(rb_node: *mut rb_node, entry: *const c_void) -> c_int {
    let str_ = entry as *const c_char;
    let snode = unsafe { str_node_from_rb_node(rb_node) };

    unsafe { strcmp((*snode).s, str_) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strlist__add(slist: *mut strlist, new_entry: *const c_char) -> c_int {
    unsafe { rblist__add_node(&mut (*slist).rblist, new_entry as *const c_void) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strlist__load(slist: *mut strlist, filename: *const c_char) -> c_int {
    let mut entry = [0 as c_char; 1024];
    let mut err: c_int;
    let fp = unsafe { fopen(filename, c"r".as_ptr()) };

    if fp.is_null() {
        return unsafe { -errno };
    }

    while unsafe { !fgets(entry.as_mut_ptr(), mem::size_of_val(&entry) as c_int, fp).is_null() } {
        let len = unsafe { strlen(entry.as_ptr()) };

        if len == 0 {
            continue;
        }
        entry[len - 1] = '\0' as c_char;

        err = unsafe { strlist__add(slist, entry.as_ptr()) };
        if err != 0 {
            unsafe {
                fclose(fp);
            }
            return err;
        }
    }

    err = 0;
    unsafe {
        fclose(fp);
    }
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strlist__remove(slist: *mut strlist, snode: *mut str_node) {
    unsafe {
        rblist__remove_node(&mut (*slist).rblist, &mut (*snode).rb_node);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strlist__find(
    slist: *mut strlist,
    entry: *const c_char,
) -> *mut str_node {
    let mut snode: *mut str_node = ptr::null_mut();
    let rb_node = unsafe { rblist__find(&mut (*slist).rblist, entry as *const c_void) };

    if !rb_node.is_null() {
        snode = unsafe { str_node_from_rb_node(rb_node) };
    }

    snode
}

unsafe fn strlist__parse_list_entry(
    slist: *mut strlist,
    s: *const c_char,
    subst_dir: *const c_char,
) -> c_int {
    let mut err: c_int;
    let mut subst: *mut c_char = ptr::null_mut();

    if unsafe { strncmp(s, c"file://".as_ptr(), 7) } == 0 {
        return unsafe { strlist__load(slist, s.add(7)) };
    }

    if !subst_dir.is_null() {
        err = -ENOMEM;
        if unsafe { asprintf(&mut subst, c"%s/%s".as_ptr(), subst_dir, s) } < 0 {
            unsafe {
                free(subst as *mut c_void);
            }
            return err;
        }

        if unsafe { access(subst, F_OK) } == 0 {
            err = unsafe { strlist__load(slist, subst) };
            unsafe {
                free(subst as *mut c_void);
            }
            return err;
        }

        if unsafe { (*slist).file_only } {
            err = -ENOENT;
            unsafe {
                free(subst as *mut c_void);
            }
            return err;
        }
    }

    err = unsafe { strlist__add(slist, s) };
    unsafe {
        free(subst as *mut c_void);
    }
    err
}

unsafe fn strlist__parse_list(
    slist: *mut strlist,
    list: *const c_char,
    subst_dir: *const c_char,
) -> c_int {
    let mut sep: *mut c_char;
    let mut s = unsafe { strdup(list) };
    let sdup = s;
    let mut err: c_int;

    if s.is_null() {
        return -ENOMEM;
    }

    loop {
        sep = unsafe { strchr(s, ',' as c_int) };
        if sep.is_null() {
            break;
        }
        unsafe {
            *sep = '\0' as c_char;
        }
        err = unsafe { strlist__parse_list_entry(slist, s, subst_dir) };
        if err != 0 {
            return err;
        }
        s = unsafe { sep.add(1) };
    }

    err = unsafe {
        if *s != 0 {
            strlist__parse_list_entry(slist, s, subst_dir)
        } else {
            0
        }
    };
    unsafe {
        free(sdup as *mut c_void);
    }
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strlist__new(
    list: *const c_char,
    config: *const strlist_config,
) -> *mut strlist {
    let slist = unsafe { malloc(mem::size_of::<strlist>()) as *mut strlist };

    if !slist.is_null() {
        let mut file_only = false;
        let mut dirname: *const c_char = ptr::null();

        if !config.is_null() {
            unsafe {
                dirname = (*config).dirname;
                file_only = (*config).file_only;
            }
        }

        unsafe {
            rblist__init(&mut (*slist).rblist);
            (*slist).rblist.node_cmp = Some(strlist__node_cmp);
            (*slist).rblist.node_new = Some(strlist__node_new);
            (*slist).rblist.node_delete = Some(strlist__node_delete);

            (*slist).file_only = file_only;

            if !list.is_null() && strlist__parse_list(slist, list, dirname) != 0 {
                free(slist as *mut c_void);
                return ptr::null_mut();
            }
        }
    }

    slist
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strlist__delete(slist: *mut strlist) {
    if !slist.is_null() {
        unsafe {
            rblist__delete(&mut (*slist).rblist);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strlist__entry(
    slist: *const strlist,
    idx: c_uint,
) -> *mut str_node {
    let mut snode: *mut str_node = ptr::null_mut();
    let rb_node: *mut rb_node;

    rb_node = unsafe { rblist__entry(&(*slist).rblist, idx) };
    if !rb_node.is_null() {
        snode = unsafe { str_node_from_rb_node(rb_node) };
    }

    snode
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
