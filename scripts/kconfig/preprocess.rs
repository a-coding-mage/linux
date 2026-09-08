// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2018 Masahiro Yamada <yamada.masahiro@socionext.com>

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct gstr { _private: [u8; 0] }
#[repr(C)] pub struct FILE { _private: [u8; 0] }
#[repr(C)] #[derive(Copy, Clone, PartialEq)] pub enum variable_flavor { VAR_RECURSIVE, VAR_SIMPLE, VAR_APPEND }

extern "C" {
    static mut cur_filename: *const c_char;
    static mut yylineno: c_int;
    fn xmalloc(n: usize) -> *mut c_void;
    fn xrealloc(p: *mut c_void, n: usize) -> *mut c_void;
    fn xstrdup(s: *const c_char) -> *mut c_char;
    fn xstrndup(s: *const c_char, n: usize) -> *mut c_char;
    fn str_printf(s: *mut gstr, fmt: *const c_char, ...);
    fn getenv(name: *const c_char) -> *const c_char;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn free(p: *mut c_void);
    fn printf(fmt: *const c_char, ...);
    fn fprintf(f: *mut FILE, fmt: *const c_char, ...);
    fn stderr() -> *mut FILE;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...);
    fn popen(cmd: *const c_char, mode: *const c_char) -> *mut FILE;
    fn pclose(f: *mut FILE) -> c_int;
    fn fread(p: *mut c_void, size: usize, n: usize, f: *mut FILE) -> usize;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
}

static mut env_list: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
static mut variable_list: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };

#[repr(C)] struct env { name: *mut c_char, value: *mut c_char, node: list_head }
#[repr(C)] struct variable { name: *mut c_char, value: *mut c_char, flavor: variable_flavor, exp_count: c_int, node: list_head }

unsafe fn c(s: &str) -> CString { CString::new(s).unwrap() }
unsafe fn list_add_tail(n: *mut list_head, h: *mut list_head) { (*n).prev = (*h).prev; (*n).next = h; (*(*h).prev).next = n; (*h).prev = n; }
unsafe fn list_del(n: *mut list_head) { (*(*n).prev).next = (*n).next; (*(*n).next).prev = (*n).prev; }
unsafe fn container<T>(p: *mut list_head, off: usize) -> *mut T { (p as *mut u8).sub(off) as *mut T }

unsafe fn pperror(format: *const c_char, args: ...) -> ! {
    fprintf(stderr(), c("%s:%d: ").as_ptr(), cur_filename, yylineno);
    // C varargs are intentionally preserved at this ABI boundary.
    vfprintf(stderr(), format, args);
    fprintf(stderr(), c("\n").as_ptr()); exit(1)
}
extern "C" { fn vfprintf(f: *mut FILE, fmt: *const c_char, args: ...); }

unsafe fn env_add(name: *const c_char, value: *const c_char) {
    let e = xmalloc(std::mem::size_of::<env>()) as *mut env;
    (*e).name = xstrdup(name); (*e).value = xstrdup(value);
    list_add_tail(&mut (*e).node, &mut env_list);
}
unsafe fn env_del(e: *mut env) { list_del(&mut (*e).node); free((*e).name as _); free((*e).value as _); free(e as _); }

unsafe fn env_expand(name: *const c_char) -> *mut c_char {
    if *name == 0 { return ptr::null_mut(); }
    let mut p = env_list.next;
    while p != &mut env_list { let e = container::<env>(p, 2 * std::mem::size_of::<*mut c_char>()); if strcmp(name, (*e).name) == 0 { return xstrdup((*e).value); } p = (*p).next; }
    let value = getenv(name); if value.is_null() { return ptr::null_mut(); } env_add(name, value); xstrdup(value)
}

#[no_mangle] pub unsafe extern "C" fn env_write_dep(s: *mut gstr) {
    let mut p = env_list.next;
    while p != &mut env_list { let next = (*p).next; let e = container::<env>(p, 2 * std::mem::size_of::<*mut c_char>()); str_printf(s, c("\nifneq \"$(%s)\" \"%s\"\n$(autoconfig): FORCE\nendif\n").as_ptr(), (*e).name, (*e).value); env_del(e); p = next; }
}

type Func = unsafe extern "C" fn(c_int, *mut *mut c_char) -> *mut c_char;
#[repr(C)] struct function { name: *const c_char, min_args: u32, max_args: u32, func: Func }
unsafe extern "C" fn do_error_if(_: c_int, a: *mut *mut c_char) -> *mut c_char { if strcmp(*a, c("y").as_ptr()) == 0 { pperror(c("%s").as_ptr(), *a.add(1)); } xstrdup(c("").as_ptr()) }
unsafe extern "C" fn do_filename(_: c_int, _: *mut *mut c_char) -> *mut c_char { xstrdup(cur_filename) }
unsafe extern "C" fn do_info(_: c_int, a: *mut *mut c_char) -> *mut c_char { printf(c("%s\n").as_ptr(), *a); xstrdup(c("").as_ptr()) }
unsafe extern "C" fn do_lineno(_: c_int, _: *mut *mut c_char) -> *mut c_char { let b = xmalloc(16) as *mut c_char; sprintf(b, c("%d").as_ptr(), yylineno); b }
unsafe extern "C" fn do_shell(_: c_int, a: *mut *mut c_char) -> *mut c_char {
    let cmd = *a; let p = popen(cmd, c("r").as_ptr()); if p.is_null() { perror(cmd); exit(1); }
    let b = xmalloc(4096) as *mut c_char; let mut n = fread(b as _, 1, 4096, p); if n == 4096 { n -= 1; }
    while n > 0 && *b.add(n - 1) == b'\n' as c_char { n -= 1; } *b.add(n) = 0;
    for i in 0..n { if *b.add(i) == b'\n' as c_char { *b.add(i) = b' ' as c_char; } }
    if pclose(p) == -1 { perror(cmd); exit(1); } xstrdup(b)
}
unsafe extern "C" fn do_warning_if(_: c_int, a: *mut *mut c_char) -> *mut c_char { if strcmp(*a, c("y").as_ptr()) == 0 { fprintf(stderr(), c("%s:%d: %s\n").as_ptr(), cur_filename, yylineno, *a.add(1)); } xstrdup(c("").as_ptr()) }

static mut function_table: [function; 6] = [
    function { name: b"error-if\0".as_ptr() as _, min_args: 2, max_args: 2, func: do_error_if }, function { name: b"filename\0".as_ptr() as _, min_args: 0, max_args: 0, func: do_filename }, function { name: b"info\0".as_ptr() as _, min_args: 1, max_args: 1, func: do_info }, function { name: b"lineno\0".as_ptr() as _, min_args: 0, max_args: 0, func: do_lineno }, function { name: b"shell\0".as_ptr() as _, min_args: 1, max_args: 1, func: do_shell }, function { name: b"warning-if\0".as_ptr() as _, min_args: 2, max_args: 2, func: do_warning_if }
];

unsafe fn variable_lookup(name: *const c_char) -> *mut variable { let mut p=variable_list.next; while p != &mut variable_list { let v=container::<variable>(p, 2*std::mem::size_of::<*mut c_char>()); if strcmp(name,(*v).name)==0{return v} p=(*p).next;} ptr::null_mut() }
unsafe fn variable_del(v:*mut variable){list_del(&mut (*v).node);free((*v).name as _);free((*v).value as _);free(v as _)}
#[no_mangle] pub unsafe extern "C" fn variable_all_del(){let mut p=variable_list.next;while p!=&mut variable_list{let n=(*p).next;variable_del(container::<variable>(p,2*std::mem::size_of::<*mut c_char>()));p=n;}}
extern "C" { fn expand_string_with_args_external(s:*const c_char,argc:c_int,argv:*mut *mut c_char)->*mut c_char; }
#[no_mangle] pub unsafe extern "C" fn variable_add(name:*const c_char,value:*const c_char,mut flavor:variable_flavor){let mut v=variable_lookup(name);let mut append=false;if !v.is_null(){if flavor==variable_flavor::VAR_APPEND{flavor=(*v).flavor;append=true}else{free((*v).value as _)}}else{if flavor==variable_flavor::VAR_APPEND{flavor=variable_flavor::VAR_RECURSIVE}v=xmalloc(std::mem::size_of::<variable>()) as _;(*v).name=xstrdup(name);(*v).exp_count=0;list_add_tail(&mut (*v).node,&mut variable_list)}(*v).flavor=flavor;let nv=if flavor==variable_flavor::VAR_SIMPLE{expand_string_with_args_external(value,0,ptr::null_mut())}else{xstrdup(value)};if append{let n=strlen((*v).value)+strlen(nv)+2;(*v).value=xrealloc((*v).value as _,n) as _;let old=strlen((*v).value);*(*v).value.add(old)=b' ' as _;ptr::copy_nonoverlapping(nv,(*v).value.add(old+1),strlen(nv)+1);free(nv as _)}else{(*v).value=nv}}

unsafe fn function_expand(name:*const c_char,argc:c_int,argv:*mut *mut c_char)->*mut c_char{for f in function_table.iter(){if strcmp(f.name,name)==0{if argc<f.min_args as c_int{pperror(c("too few function arguments passed to '%s'").as_ptr(),name)}if argc>f.max_args as c_int{pperror(c("too many function arguments passed to '%s'").as_ptr(),name)}return(f.func)(argc,argv)}}ptr::null_mut()}
unsafe fn expand_dollar_with_args(strp:*mut *const c_char,argc:c_int,argv:*mut *mut c_char)->*mut c_char{let p=**strp;if p!=b'(' as _{return xstrdup(c("$").as_ptr())}let mut q=p.add(1);let mut nest=0;while *q!=0{if *q==b'(' as _{nest+=1}else if *q==b')' as _{if nest==0{break}nest-=1}q=q.add(1)}if *q==0{pperror(c("unterminated reference to '%s': missing ')'\0").as_ptr(),p)}**strp=q.add(1);xstrndup(p.add(1),q.offset_from(p.add(1)) as usize)}
#[no_mangle] pub unsafe extern "C" fn expand_dollar(s:*mut *const c_char)->*mut c_char{expand_dollar_with_args(s,0,ptr::null_mut())}
unsafe fn expand_string_inner(s:*const c_char,end:fn(c_char)->bool,argc:c_int,argv:*mut *mut c_char)->*mut c_char{let mut out=xmalloc(1) as *mut c_char;*out=0;let mut p=s;while *p!=0&&!end(*p){if *p==b'$' as _{let mut q=p.add(1);let e=expand_dollar_with_args(&mut q,argc,argv);let n=strlen(out)+q.offset_from(p) as usize+strlen(e)+1;out=xrealloc(out as _,n) as _;ptr::copy_nonoverlapping(e,out.add(strlen(out)),strlen(e)+1);free(e as _);p=q}else{p=p.add(1)}}out}
unsafe fn is_end_str(c:c_char)->bool{c==0}unsafe fn is_end_token(c:c_char)->bool{!((c as u8).is_ascii_alphanumeric()||c==b'_' as _||c==b'-' as _)}
unsafe fn expand_string_with_args(s:*const c_char,a:c_int,v:*mut *mut c_char)->*mut c_char{expand_string_inner(s,is_end_str,a,v)}
unsafe fn expand_string(s:*const c_char)->*mut c_char{expand_string_with_args(s,0,ptr::null_mut())}
#[no_mangle] pub unsafe extern "C" fn expand_one_token(s:*mut *const c_char)->*mut c_char{expand_string_inner(*s,is_end_token,0,ptr::null_mut())}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
