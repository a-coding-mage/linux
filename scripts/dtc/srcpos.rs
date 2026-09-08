// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2007 Jon Loeliger, Freescale Semiconductor, Inc.
 */

use std::ffi::{c_char, c_int, c_void};
use std::ptr;

#[repr(C)]
pub struct FILE { _private: [u8; 0] }

#[repr(C)]
pub struct search_path {
    pub next: *mut search_path,
    pub dirname: *const c_char,
}

#[repr(C)]
pub struct srcfile_state {
    pub f: *mut FILE,
    pub name: *mut c_char,
    pub dir: *mut c_char,
    pub prev: *mut srcfile_state,
    pub lineno: c_int,
    pub colno: c_int,
}

#[repr(C)]
pub struct srcpos {
    pub file: *mut srcfile_state,
    pub first_line: c_int,
    pub first_column: c_int,
    pub last_line: c_int,
    pub last_column: c_int,
    pub next: *mut srcpos,
}

unsafe extern "C" {
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn free(p: *mut c_void);
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fputc(c: c_int, stream: *mut FILE) -> c_int;
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...);
    fn vfprintf(stream: *mut FILE, fmt: *const c_char, ap: *mut c_void) -> c_int;
    fn strerror(errnum: c_int) -> *const c_char;
    fn __errno_location() -> *mut c_int;
    fn xmalloc(size: usize) -> *mut c_void;
    fn xstrdup(s: *const c_char) -> *mut c_char;
    fn xasprintf(ret: *mut *mut c_char, fmt: *const c_char, ...);
    fn join_path(dirname: *const c_char, fname: *const c_char) -> *mut c_char;
    fn streq(a: *const c_char, b: *const c_char) -> bool;
    fn die(fmt: *const c_char, ... ) -> !;
    fn fprint_path_escaped(stream: *mut FILE, path: *const c_char);
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
}

static mut search_path_head: *mut search_path = ptr::null_mut();
static mut search_path_tail: *mut *mut search_path = ptr::null_mut();
static mut srcfile_depth: c_int = 0;
pub static mut depfile: *mut FILE = ptr::null_mut();
pub static mut current_srcfile: *mut srcfile_state = ptr::null_mut();
static mut initial_path: *mut c_char = ptr::null_mut();
static mut initial_pathlen: c_int = 0;
static mut initial_cpp: bool = true;

unsafe fn get_dirname(path: *const c_char) -> *mut c_char {
    let slash = strrchr(path, b'/' as c_int);
    if !slash.is_null() {
        let len = slash.offset_from(path) as usize;
        let dir = xmalloc(len + 1) as *mut c_char;
        memcpy(dir as *mut c_void, path as *const c_void, len);
        *dir.add(len) = 0;
        return dir;
    }
    ptr::null_mut()
}

unsafe fn set_initial_path(fname: *mut c_char) {
    let len = strlen(fname);
    xasprintf(&mut initial_path, b"%s\0".as_ptr() as *const c_char, fname);
    initial_pathlen = 0;
    for i in 0..len {
        if *initial_path.add(i) == b'/' as c_char { initial_pathlen += 1; }
    }
}

unsafe fn shorten_to_initial_path(fname: *mut c_char) -> *mut c_char {
    let mut p1 = fname;
    let mut p2 = initial_path;
    let mut prevslash1: *mut c_char = ptr::null_mut();
    let mut slashes = 0;
    while *p1 != 0 && *p2 != 0 {
        if *p1 != *p2 { break; }
        if *p1 == b'/' as c_char { prevslash1 = p1; slashes += 1; }
        p1 = p1.add(1); p2 = p2.add(1);
    }
    if !prevslash1.is_null() {
        let diff = initial_pathlen - slashes;
        let restlen = strlen(fname) - prevslash1.add(1).offset_from(fname) as usize;
        let res = xmalloc((3 * diff as usize) + restlen + 1) as *mut c_char;
        let mut j = 0;
        for _ in 0..diff { *res.add(j)=b'.' as c_char; *res.add(j+1)=b'.' as c_char; *res.add(j+2)=b'/' as c_char; j += 3; }
        strcpy(res.add(j), prevslash1.add(1));
        return res;
    }
    ptr::null_mut()
}

unsafe fn is_absolute_path(path: *const c_char) -> bool {
    if *path == b'/' as c_char { return true; }
    #[cfg(windows)] { return ((*path >= b'A' as c_char && *path <= b'Z' as c_char) || (*path >= b'a' as c_char && *path <= b'z' as c_char)) && *path.add(1) == b':' as c_char; }
    #[cfg(not(windows))] { false }
}

unsafe fn try_open(dirname: *const c_char, fname: *const c_char, fp: *mut *mut FILE) -> *mut c_char {
    let fullname = if dirname.is_null() || is_absolute_path(fname) { xstrdup(fname) } else { join_path(dirname, fname) };
    *fp = fopen(fullname, b"rb\0".as_ptr() as *const c_char);
    if (*fp).is_null() { free(fullname as *mut c_void); ptr::null_mut() } else { fullname }
}

unsafe fn fopen_any_on_path(fname: *const c_char, fp: *mut *mut FILE) -> *mut c_char {
    let cur_dir = if !current_srcfile.is_null() { (*current_srcfile).dir as *const c_char } else { ptr::null() };
    let mut fullname = try_open(cur_dir, fname, fp);
    let mut node = search_path_head;
    while (*fp).is_null() && !node.is_null() { fullname = try_open((*node).dirname, fname, fp); node = (*node).next; }
    fullname
}

pub unsafe fn srcfile_relative_open(fname: *const c_char, fullnamep: *mut *mut c_char) -> *mut FILE {
    let (f, fullname) = if streq(fname, b"-\0".as_ptr() as *const c_char) { (stdin, xstrdup(b"<stdin>\0".as_ptr() as *const c_char)) } else {
        let mut f = ptr::null_mut(); let fullname = fopen_any_on_path(fname, &mut f);
        if f.is_null() { die(b"Couldn't open \"%s\": %s\n\0".as_ptr() as *const c_char, fname, strerror(*__errno_location())); }
        (f, fullname)
    };
    if !depfile.is_null() { fputc(b' ' as c_int, depfile); fprint_path_escaped(depfile, fullname); }
    if !fullnamep.is_null() { *fullnamep = fullname; } else { free(fullname as *mut c_void); }
    f
}

pub unsafe fn srcfile_push(fname: *const c_char) {
    if srcfile_depth >= 200 { die(b"Includes nested too deeply\0".as_ptr() as *const c_char); }
    srcfile_depth += 1;
    let srcfile = xmalloc(std::mem::size_of::<srcfile_state>()) as *mut srcfile_state;
    (*srcfile).f = srcfile_relative_open(fname, &mut (*srcfile).name);
    (*srcfile).dir = get_dirname((*srcfile).name); (*srcfile).prev = current_srcfile; (*srcfile).lineno=1; (*srcfile).colno=1; current_srcfile=srcfile;
    if srcfile_depth == 1 { set_initial_path((*srcfile).name); }
}

pub unsafe fn srcfile_pop() -> bool {
    let srcfile = current_srcfile; assert!(!srcfile.is_null()); current_srcfile=(*srcfile).prev;
    if fclose((*srcfile).f) != 0 { die(b"Error closing \"%s\": %s\n\0".as_ptr() as *const c_char, (*srcfile).name, strerror(*__errno_location())); }
    !current_srcfile.is_null()
}

pub unsafe fn srcfile_add_search_path(dirname: *const c_char) {
    let node=xmalloc(std::mem::size_of::<search_path>()) as *mut search_path; (*node).next=ptr::null_mut(); (*node).dirname=xstrdup(dirname);
    if !search_path_tail.is_null() { *search_path_tail=node; } else { search_path_head=node; } search_path_tail=&mut (*node).next;
}

pub unsafe fn srcpos_update(pos: *mut srcpos, text: *const c_char, len: c_int) { (*pos).file=current_srcfile; (*pos).first_line=(*current_srcfile).lineno; (*pos).first_column=(*current_srcfile).colno; for i in 0..len { if *text.add(i as usize)==b'\n' as c_char { (*current_srcfile).lineno+=1; (*current_srcfile).colno=1; } else { (*current_srcfile).colno+=1; } } (*pos).last_line=(*current_srcfile).lineno; (*pos).last_column=(*current_srcfile).colno; }

pub unsafe fn srcpos_copy(pos: *mut srcpos) -> *mut srcpos { if pos.is_null(){return ptr::null_mut();} assert!((*pos).next.is_null()); let n=xmalloc(std::mem::size_of::<srcpos>()) as *mut srcpos; memcpy(n as *mut c_void,pos as *const c_void,std::mem::size_of::<srcpos>()); let f=xmalloc(std::mem::size_of::<srcfile_state>()) as *mut srcfile_state; memcpy(f as *mut c_void,(*pos).file as *const c_void,std::mem::size_of::<srcfile_state>()); (*n).file=f; n }
pub unsafe fn srcpos_extend(pos:*mut srcpos,newtail:*mut srcpos)->*mut srcpos { if pos.is_null(){return newtail;} let mut p=pos; while !(*p).next.is_null(){p=(*p).next;} (*p).next=newtail; pos }
pub unsafe fn srcpos_free(mut pos:*mut srcpos){while !pos.is_null(){let n=(*pos).next;free(pos as *mut c_void);pos=n;}}

pub unsafe fn srcpos_string(pos:*mut srcpos)->*mut c_char { let fname=if !(*pos).file.is_null()&&!(*(*pos).file).name.is_null(){(*(*pos).file).name}else{b"<no-file>\0".as_ptr() as *mut c_char}; let mut out=ptr::null_mut(); if (*pos).first_line!=(*pos).last_line{xasprintf(&mut out,b"%s:%d.%d-%d.%d\0".as_ptr() as *const c_char,fname,(*pos).first_line,(*pos).first_column,(*pos).last_line,(*pos).last_column)}else if (*pos).first_column!=(*pos).last_column{xasprintf(&mut out,b"%s:%d.%d-%d\0".as_ptr() as *const c_char,fname,(*pos).first_line,(*pos).first_column,(*pos).last_column)}else{xasprintf(&mut out,b"%s:%d.%d\0".as_ptr() as *const c_char,fname,(*pos).first_line,(*pos).first_column)} out }

unsafe fn srcpos_string_comment(pos:*mut srcpos,first_line:bool,level:c_int)->*mut c_char { if pos.is_null(){if level>1{let mut s=ptr::null_mut();xasprintf(&mut s,b"<no-file>:<no-line>\0".as_ptr() as *const c_char);return s;}return ptr::null_mut();} let fname=if (*pos).file.is_null(){b"<no-file>\0".as_ptr() as *const c_char}else if (*(*pos).file).name.is_null(){b"<no-filename>\0".as_ptr() as *const c_char}else{(*(*pos).file).name as *const c_char}; let mut first=ptr::null_mut(); if level>1{xasprintf(&mut first,b"%s:%d:%d-%d:%d\0".as_ptr() as *const c_char,fname,(*pos).first_line,(*pos).first_column,(*pos).last_line,(*pos).last_column)}else{xasprintf(&mut first,b"%s:%d\0".as_ptr() as *const c_char,fname,if first_line{(*pos).first_line}else{(*pos).last_line})} if !(*pos).next.is_null(){let rest=srcpos_string_comment((*pos).next,first_line,level);let mut out=ptr::null_mut();xasprintf(&mut out,b"%s, %s\0".as_ptr() as *const c_char,first,rest);free(first as *mut c_void);free(rest as *mut c_void);out}else{first} }
pub unsafe fn srcpos_string_first(pos:*mut srcpos,level:c_int)->*mut c_char{srcpos_string_comment(pos,true,level)}
pub unsafe fn srcpos_string_last(pos:*mut srcpos,level:c_int)->*mut c_char{srcpos_string_comment(pos,false,level)}

pub unsafe fn srcpos_verror(pos:*mut srcpos,prefix:*const c_char,fmt:*const c_char,va:*mut c_void){let s=srcpos_string(pos);fprintf(stderr,b"%s: %s \0".as_ptr() as *const c_char,prefix,s);vfprintf(stderr,fmt,va);fprintf(stderr,b"\n\0".as_ptr() as *const c_char);free(s as *mut c_void)}
pub unsafe fn srcpos_error(pos:*mut srcpos,prefix:*const c_char,fmt:*const c_char, mut args:...){srcpos_verror(pos,prefix,fmt,&mut args as *mut _ as *mut c_void)}
pub unsafe fn srcpos_set_line(f:*mut c_char,l:c_int){(*current_srcfile).name=f;(*current_srcfile).lineno=l;if initial_cpp{initial_cpp=false;set_initial_path(f);}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
