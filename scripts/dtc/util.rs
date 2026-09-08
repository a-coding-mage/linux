// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2011 The Chromium Authors, All Rights Reserved.
 * Copyright 2008 Jon Loeliger, Freescale Semiconductor, Inc.
 *
 * util_is_printable_string contributed by
 *	Pantelis Antoniou <pantelis.antoniou AT gmail.com>
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct va_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

pub type fdt32_t = u32;

extern "C" {
    fn xmalloc(size: usize) -> *mut c_void;
    fn xrealloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn die(fmt: *const c_char, ...);
    fn fputc(c: c_int, fp: *mut FILE) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strnlen(s: *const c_char, n: usize) -> usize;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn strtol(s: *const c_char, end: *mut *mut c_char, base: c_int) -> c_long;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn isprint(c: c_int) -> c_int;
    fn vsnprintf(s: *mut c_char, n: usize, fmt: *const c_char, ap: *mut va_list) -> c_int;
    fn va_copy(dest: *mut va_list, src: *mut va_list);
    fn va_end(ap: *mut va_list);
    fn fprintf(fp: *mut FILE, fmt: *const c_char, ... ) -> c_int;
    fn printf(fmt: *const c_char, ... ) -> c_int;
    fn strerror(errnum: c_int) -> *const c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn free(ptr: *mut c_void);
    fn exit(status: c_int) -> !;
    fn fdt_totalsize(blob: *const c_void) -> c_int;
    fn fdt32_to_cpu(x: fdt32_t) -> fdt32_t;
}

extern "C" {
    static mut errno: c_int;
    static DTC_VERSION: *const c_char;
    static a_argument: c_int;
    static no_argument: c_int;
}

#[inline]
unsafe fn cstr_len(s: *const c_char) -> usize { strlen(s) }

pub unsafe fn fprint_path_escaped(fp: *mut FILE, path: *const c_char) {
    let mut p = path;
    while *p != 0 {
        if *p == b' ' as c_char { fputc(b'\\' as c_int, fp); fputc(b' ' as c_int, fp); }
        else { fputc(*p as c_int, fp); }
        p = p.add(1);
    }
}

pub unsafe fn xstrdup(s: *const c_char) -> *mut c_char {
    let len = cstr_len(s) + 1;
    let d = xmalloc(len) as *mut c_char;
    memcpy(d as *mut c_void, s as *const c_void, len);
    d
}

pub unsafe fn xstrndup(s: *const c_char, n: usize) -> *mut c_char {
    let len = strnlen(s, n) + 1;
    let d = xmalloc(len) as *mut c_char;
    memcpy(d as *mut c_void, s as *const c_void, len - 1);
    *d.add(len - 1) = 0;
    d
}

pub unsafe fn xavsprintf_append(strp: *mut *mut c_char, fmt: *const c_char, ap: *mut va_list) -> c_int {
    let mut size = 0usize;
    let p = *strp;
    if !p.is_null() { size = cstr_len(p); }
    let mut ap_copy = va_list { _private: [] };
    va_copy(&mut ap_copy, ap);
    let n = vsnprintf(ptr::null_mut(), 0, fmt, &mut ap_copy) as usize + 1;
    va_end(&mut ap_copy);
    let p = xrealloc(p as *mut c_void, size + n) as *mut c_char;
    vsnprintf(p.add(size), n, fmt, ap);
    *strp = p;
    cstr_len(p) as c_int
}

pub unsafe fn xasprintf_append(strp: *mut *mut c_char, fmt: *const c_char, mut ap: ...) -> c_int { xavsprintf_append(strp, fmt, &mut ap) }
pub unsafe fn xasprintf(strp: *mut *mut c_char, fmt: *const c_char, mut ap: ...) -> c_int { *strp = ptr::null_mut(); xavsprintf_append(strp, fmt, &mut ap) }

pub unsafe fn join_path(path: *const c_char, name: *const c_char) -> *mut c_char {
    let mut lenp = cstr_len(path); let lenn = cstr_len(name); let mut len = lenp + lenn + 2; let mut needslash = true;
    if lenp > 0 && *path.add(lenp - 1) == b'/' as c_char { needslash = false; len -= 1; }
    let str = xmalloc(len) as *mut c_char; memcpy(str as *mut c_void, path as *const c_void, lenp);
    if needslash { *str.add(lenp) = b'/' as c_char; lenp += 1; }
    memcpy(str.add(lenp) as *mut c_void, name as *const c_void, lenn + 1); str
}

pub unsafe fn util_is_printable_string(data: *const c_void, len: c_int) -> bool {
    let mut s = data as *const c_char; if len == 0 || *s.add((len - 1) as usize) != 0 { return false; }
    let se = s.add(len as usize);
    while s < se { let ss = s; while s < se && *s != 0 && isprint(*s as u8 as c_int) != 0 { s = s.add(1); }
        if *s != 0 || s == ss { return false; } s = s.add(1); }
    true
}

unsafe fn get_oct_char(s: *const c_char, i: *mut c_int) -> c_char { let mut x = [0 as c_char; 4]; strncpy(x.as_mut_ptr(), s.add(*i as usize), 3); let mut endx = ptr::null_mut(); let val = strtol(x.as_ptr(), &mut endx, 8); assert!(endx > x.as_mut_ptr()); *i += endx.offset_from(x.as_mut_ptr()) as c_int; val as c_char }
unsafe fn get_hex_char(s: *const c_char, i: *mut c_int) -> c_char { let mut x = [0 as c_char; 3]; strncpy(x.as_mut_ptr(), s.add(*i as usize), 2); let mut endx = ptr::null_mut(); let val = strtol(x.as_ptr(), &mut endx, 16); if endx <= x.as_mut_ptr() { die(b"\\x used with no following hex digits\n\0".as_ptr() as *const c_char); } *i += endx.offset_from(x.as_mut_ptr()) as c_int; val as c_char }

pub unsafe fn get_escape_char(s: *const c_char, i: *mut c_int) -> c_char { let c = *s.add(*i as usize); let mut j = *i + 1; let val = match c as u8 { b'a'=>7,b'b'=>8,b't'=>9,b'n'=>10,b'v'=>11,b'f'=>12,b'r'=>13,b'0'..=b'7'=>{j-=1;get_oct_char(s,&mut j) as u8}, b'x'=>get_hex_char(s,&mut j) as u8, _=>c as u8 } as c_char; *i=j; val }

pub unsafe fn utilfdt_read_err(filename: *const c_char, buffp: *mut *mut c_char, len: *mut usize) -> c_int { let mut fd=0; let mut buf=ptr::null_mut(); let mut bufsize=1024usize; let mut offset=0usize; let mut ret=0isize; *buffp=ptr::null_mut(); if strcmp(filename,b"-\0".as_ptr() as *const c_char)!=0 {fd=open(filename,0); if fd<0{return errno;}} buf=xmalloc(bufsize) as *mut c_char; loop {if offset==bufsize {bufsize*=2;buf=xrealloc(buf as *mut c_void,bufsize) as *mut c_char;} ret=read(fd,buf.add(offset) as *mut c_void,bufsize-offset); if ret<0 {ret=errno as isize;break;} offset+=ret as usize;if ret==0{break;}} close(fd);if ret!=0{free(buf as *mut c_void)}else{*buffp=buf}if !len.is_null(){*len=bufsize}ret as c_int }
pub unsafe fn utilfdt_read(filename:*const c_char,len:*mut usize)->*mut c_char{let mut buff=ptr::null_mut();if utilfdt_read_err(filename,&mut buff,len)!=0{return ptr::null_mut()}buff}
pub unsafe fn utilfdt_write_err(filename:*const c_char,blob:*const c_void)->c_int{let mut fd=1;if strcmp(filename,b"-\0".as_ptr() as *const c_char)!=0{fd=open(filename,0x41,0o666);if fd<0{return errno}}let total=fdt_totalsize(blob);let mut off=0;while off<total{let r=write(fd,(blob as *const c_char).add(off as usize) as *const c_void,(total-off) as usize);if r<0{return errno};off+=r as c_int}if fd!=1{close(fd)}0}
pub unsafe fn utilfdt_write(filename:*const c_char,blob:*const c_void)->c_int{if utilfdt_write_err(filename,blob)!=0{-1}else{0}}

pub unsafe fn utilfdt_decode_type(fmt:*const c_char,type_:*mut c_int,size:*mut c_int)->c_int{if *fmt==0{return -1}let mut f=fmt;let mut q=0;*size=-1;if strchr(b"hlLb\0".as_ptr() as *const c_char,*f as c_int)!=ptr::null_mut(){q=*f as c_int;f=f.add(1);if q==*f as c_int{if *f as u8==b'h'{q=b'b' as c_int;f=f.add(1)}}}if *f==0||strchr(b"iuxsr\0".as_ptr() as *const c_char,*f as c_int)==ptr::null_mut(){return -1}if *f as u8!=b's'&&*f as u8!=b'r'{*size=if q==b'b' as c_int{1}else if q==b'h' as c_int{2}else if q==b'l' as c_int{4}else{-1}}*type_=*f as c_int;f=f.add(1);if *f!=0{-1}else{0}}

pub unsafe fn utilfdt_print_data(data:*const c_char,len:c_int){if len==0{return}if util_is_printable_string(data as *const c_void,len){printf(b" = \0".as_ptr() as *const c_char);let mut s=data;loop{printf(b"\"%s\"\0".as_ptr() as *const c_char,s);s=s.add(strlen(s)+1);if s>=data.add(len as usize){break}printf(b", \0".as_ptr() as *const c_char)} }else if len%4==0{printf(b" = <\0".as_ptr() as *const c_char);let cells=data as *const fdt32_t;for i in 0..(len/4){printf(b"0x%08x%s\0".as_ptr() as *const c_char,fdt32_to_cpu(*cells.add(i as usize)),if i<len/4-1{b" \0".as_ptr() as *const c_char}else{b"\0".as_ptr() as *const c_char})}printf(b">\0".as_ptr() as *const c_char)}else{printf(b" = [\0".as_ptr() as *const c_char);for i in 0..len{printf(b"%02x%s\0".as_ptr() as *const c_char,*data.add(i as usize) as u8,if i<len-1{b" \0".as_ptr() as *const c_char}else{b"\0".as_ptr() as *const c_char})}printf(b"]\0".as_ptr() as *const c_char)}}

pub unsafe fn util_version()->!{printf(b"Version: %s\n\0".as_ptr() as *const c_char,DTC_VERSION);exit(0)}
pub unsafe fn util_usage(_errmsg:*const c_char,_synopsis:*const c_char,_short_opts:*const c_char,_long_opts:*const option,_opts_help:*const *const c_char)->!{exit(0)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
