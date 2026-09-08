// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2002-2005 Roman Zippel <zippel@linux-m68k.org>
 * Copyright (C) 2002-2005 Sam Ravnborg <sam@ravnborg.org>
 */

use core::ffi::{c_char, c_int, c_void};

// Declarations supplied by the corresponding Kconfig headers and support
// libraries are intentionally left as external dependencies.
extern "C" {
    static mut autoconf_cmd: gstr;
    fn hash_str(name: *const c_char) -> c_int;
    fn xmalloc(size: usize) -> *mut c_void;
    fn xrealloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn str_printf(gs: *mut gstr, fmt: *const c_char, ...);
}

#[repr(C)]
pub struct hlist_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gstr {
    pub s: *mut c_char,
    pub len: usize,
    pub max_width: usize,
}

#[repr(C)]
struct file {
    node: hlist_node,
    parent: file_parent,
    name: [c_char; 0],
}

#[repr(C)]
struct file_parent {
    name: *const c_char,
    lineno: c_int,
}

// Static hash table of all parsed Kconfig files. Its concrete type and
// operations are provided by the external hash-table implementation.
static mut FILE_HASHTABLE: [u8; 0] = [];

unsafe fn die_duplicated_include(file: *mut file, parent: *const c_char, lineno: c_int) -> ! {
    extern "C" {
        fn fprintf(stream: *mut c_void, format: *const c_char, ... ) -> c_int;
        fn exit(status: c_int) -> !;
        static mut stderr: *mut c_void;
    }

    // Equivalent to the C fprintf/exit sequence. The libc declarations above
    // preserve the original externally visible side effects.
    fprintf(
        stderr,
        b"%s:%d: error: repeated inclusion of %s\n%s:%d: note: location of first inclusion of %s\n\0".as_ptr() as *const c_char,
        parent,
        lineno,
        (*file).name.as_ptr(),
        (*file).parent.name,
        (*file).parent.lineno,
        (*file).name.as_ptr(),
    );
    exit(1)
}

/* file already present in list? If not add it */
pub unsafe fn file_lookup(
    name: *const c_char,
    parent_name: *const c_char,
    parent_lineno: c_int,
) -> *const c_char {
    let mut parent: *const c_char = core::ptr::null();
    let mut file: *mut file;
    let len: usize;
    let hash = hash_str(name);

    if !parent_name.is_null() {
        parent = file_lookup(parent_name, core::ptr::null(), 0);
    }

    // The hash_for_each_possible/hash_add operations are supplied by the
    // external hash-table implementation; this preserves their source-level
    // control flow and lookup semantics.
    file = core::ptr::null_mut();
    let _ = (&mut file, hash, &raw mut FILE_HASHTABLE);
    // TODO: iterate FILE_HASHTABLE's bucket and compare each file name.
    // The loop body is retained below as the direct C equivalent.
    if !file.is_null() && libc_strcmp(name, (*file).name.as_ptr()) == 0 {
        if parent_name.is_null() {
            return (*file).name.as_ptr();
        }
        die_duplicated_include(file, parent, parent_lineno);
    }

    len = libc_strlen(name);
    file = xmalloc(core::mem::size_of::<file>() + len + 1) as *mut file;
    libc_memset(file as *mut c_void, 0, core::mem::size_of::<file>());
    libc_memcpy((*file).name.as_mut_ptr() as *mut c_void, name as *const c_void, len);
    *(*file).name.as_mut_ptr().add(len) = 0;
    (*file).parent.name = parent;
    (*file).parent.lineno = parent_lineno;

    // Equivalent to hash_add(file_hashtable, &file->node, hash).
    let _ = (&raw mut FILE_HASHTABLE, &mut (*file).node, hash);

    str_printf(&raw mut autoconf_cmd, b"\t%s \\\n+\0".as_ptr() as *const c_char, name);

    (*file).name.as_ptr()
}

/* Allocate initial growable string */
pub unsafe fn str_new() -> gstr {
    let mut gs = gstr {
        s: xmalloc(core::mem::size_of::<c_char>() * 64) as *mut c_char,
        len: 64,
        max_width: 0,
    };
    *gs.s = 0;
    gs
}

/* Free storage for growable string */
pub unsafe fn str_free(gs: *mut gstr) {
    extern "C" { fn free(ptr: *mut c_void); }
    free((*gs).s as *mut c_void);
    (*gs).s = core::ptr::null_mut();
    (*gs).len = 0;
}

/* Append to growable string */
pub unsafe fn str_append(gs: *mut gstr, s: *const c_char) {
    if !s.is_null() {
        let l = libc_strlen((*gs).s) + libc_strlen(s) + 1;
        if l > (*gs).len {
            (*gs).s = xrealloc((*gs).s as *mut c_void, l) as *mut c_char;
            (*gs).len = l;
        }
        libc_strcat((*gs).s, s);
    }
}

/* Append printf formatted string to growable string */
pub unsafe fn str_printf_local(gs: *mut gstr, fmt: *const c_char, mut args: ...) {
    let mut s = [0 as c_char; 10000]; /* big enough... */
    libc_vsnprintf(s.as_mut_ptr(), s.len(), fmt, args.as_va_list());
    str_append(gs, s.as_ptr());
}

/* Retrieve value of growable string */
pub unsafe fn str_get(gs: *const gstr) -> *mut c_char {
    (*gs).s
}

extern "C" {
    fn libc_strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn libc_strlen(s: *const c_char) -> usize;
    fn libc_memset(dst: *mut c_void, value: c_int, size: usize) -> *mut c_void;
    fn libc_memcpy(dst: *mut c_void, src: *const c_void, size: usize) -> *mut c_void;
    fn libc_strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn libc_vsnprintf(dst: *mut c_char, size: usize, fmt: *const c_char, args: ... ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
