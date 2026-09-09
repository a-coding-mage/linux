// SPDX-License-Identifier: GPL-2.0-only
/* sysctl.c: General linux system control interface */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::c_void, mem, ptr};

pub type ulong = usize;
pub type uint = u32;
pub type loff_t = i64;
pub type ssize_t = isize;
pub type gfp_t = u32;

#[repr(C)]
pub struct ctl_table {
    pub procname: *const i8,
    pub data: *mut c_void,
    pub maxlen: usize,
    pub mode: u16,
    pub proc_handler: Option<unsafe extern "C" fn(*const ctl_table, i32, *mut c_void, *mut usize, *mut loff_t) -> i32>,
    pub extra1: *mut c_void,
    pub extra2: *mut c_void,
}
#[repr(C)] pub union proc_vec_conv {
    pub int_conv: Option<unsafe extern "C" fn(*mut bool, *mut ulong, *mut i32, i32, *const ctl_table) -> i32>,
    pub uint_conv: Option<unsafe extern "C" fn(*mut bool, *mut ulong, *mut uint, i32, *const ctl_table) -> i32>,
    pub ulong_conv: Option<unsafe extern "C" fn(*mut bool, *mut ulong, *mut ulong, i32, *const ctl_table) -> i32>,
}
#[repr(C)] pub struct static_key { _private: [u8; 0] }
extern "C" {
    fn _parse_integer_fixup_radix(cp: *const i8, base: *mut uint) -> *const i8;
    fn _parse_integer(cp: *const i8, base: uint, result: *mut u64) -> uint;
    fn strlen(s: *const i8) -> usize;
    fn memcpy(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void;
    fn memchr(s: *const c_void, c: i32, n: usize) -> *mut c_void;
    fn sprintf(s: *mut i8, fmt: *const i8, ...) -> i32;
    fn capable(cap: i32) -> bool;
    fn bitmap_zalloc(nbits: ulong, flags: gfp_t) -> *mut ulong;
    fn bitmap_free(bitmap: *mut ulong);
    fn bitmap_set(map: *mut ulong, start: ulong, nbits: ulong);
    fn bitmap_or(dst: *mut ulong, a: *const ulong, b: *const ulong, nbits: ulong);
    fn bitmap_copy(dst: *mut ulong, src: *const ulong, nbits: ulong);
    fn find_next_bit(addr: *const ulong, size: ulong, offset: ulong) -> ulong;
    fn find_next_zero_bit(addr: *const ulong, size: ulong, offset: ulong) -> ulong;
    fn static_key_enabled(key: *mut static_key) -> i32;
    fn static_key_enable(key: *mut static_key);
    fn static_key_disable(key: *mut static_key);
    fn mutex_lock(m: *mut c_void); fn mutex_unlock(m: *mut c_void);
    fn register_sysctl_init(name: *const i8, table: *const ctl_table);
    fn proc_dointvec(*const ctl_table, i32, *mut c_void, *mut usize, *mut loff_t) -> i32;
}

const EINVAL: i32 = 22; const ERANGE: i32 = 34; const ENOMEM: i32 = 12;
const ENOSYS: i32 = 38; const EPERM: i32 = 1; const PAGE_SIZE: usize = 4096;
const KSTRTOX_OVERFLOW: uint = 0x8000_0000; const GFP_KERNEL: gfp_t = 0;
const SYSCTL_WRITES_LEGACY: i32 = -1; const SYSCTL_WRITES_WARN: i32 = 0; const SYSCTL_WRITES_STRICT: i32 = 1;
const PROC_VEC_INT: i32 = 0; const PROC_VEC_UINT: i32 = 1; const PROC_VEC_ULONG: i32 = 2;
#[inline] fn user_to_kern(dir: i32) -> bool { dir != 0 }
#[inline] fn kern_to_user(dir: i32) -> bool { dir == 0 }

#[no_mangle] pub static sysctl_vals: [i32; 12] = [0,1,2,3,4,100,200,1000,3000,i32::MAX,65535,-1];
#[no_mangle] pub static sysctl_long_vals: [ulong; 3] = [0,1,usize::MAX];
static mut sysctl_writes_strict: i32 = SYSCTL_WRITES_STRICT;
static NGROUPS_MAX: i32 = 65536; static CAP_LAST_CAP: i32 = 40;

unsafe fn proc_skip_spaces(buf: &mut *mut i8, size: &mut usize) { while *size != 0 && (**buf as u8).is_ascii_whitespace() { *size-=1; *buf = (*buf).add(1); } }
unsafe fn proc_skip_char(buf: &mut *mut i8, size: &mut usize, v: i8) { while *size != 0 && **buf == v { *size-=1; *buf=(*buf).add(1); } }
unsafe fn strtoul_lenient(cp: *const i8, endp: *mut *mut i8, mut base: uint, res: *mut ulong) -> i32 {
    let mut result=0u64; cp=_parse_integer_fixup_radix(cp,&mut base); let rv=_parse_integer(cp,base,&mut result);
    if rv & KSTRTOX_OVERFLOW != 0 || result as ulong as u64 != result { return -ERANGE; }
    if !endp.is_null() { *endp=cp.add(rv as usize) as *mut i8; } *res=result as ulong; 0
}
unsafe fn proc_get_long(buf: &mut *mut i8, size: &mut usize, val: &mut ulong, neg: &mut bool, perm: *const i8, plen: usize, tr: *mut i8) -> i32 {
    const N: usize=22; let mut tmp=[0i8;N]; let mut len=*size; if len==0{return -EINVAL;} if len>N-1{len=N-1;} memcpy(tmp.as_mut_ptr() as _,*buf as _,len); tmp[len]=0; let mut p=tmp.as_mut_ptr();
    if *p==b'-' as i8 && *size>1 {*neg=true;p=p.add(1);} else {*neg=false;} if !(*p as u8).is_ascii_digit(){return -EINVAL;}
    if strtoul_lenient(p,&mut p,0,val)!=0{return -EINVAL;} len=p.offset_from(tmp.as_mut_ptr()) as usize; if len==N-1{return -EINVAL;}
    if len<*size && plen!=0 && memchr(perm as _,*p as i32,plen).is_null(){return -EINVAL;} if !tr.is_null()&&len<*size{*tr=*p;}
    *buf=(*buf).add(len);*size-=len;0
}
unsafe fn proc_put_long(buf: &mut *mut c_void, size: &mut usize, val: ulong, neg: bool) { let s=if neg{format!("-{}",val)}else{format!("{}",val)}; let n=s.len().min(*size); ptr::copy_nonoverlapping(s.as_ptr(),*buf as *mut u8,n);*buf=(*buf).add(n);*size-=n; }
unsafe fn proc_put_char(buf: &mut *mut c_void,size:&mut usize,c:i8){if *size!=0{**(buf as *mut *mut i8)=c;*buf=(*buf).add(1);*size-=1;}}

unsafe fn proc_first_pos_non_zero_ignore(pos:*mut loff_t,_table:*const ctl_table)->bool{if *pos==0{return false;} sysctl_writes_strict==SYSCTL_WRITES_STRICT}

#[no_mangle] pub unsafe extern "C" fn proc_dostring(table:*const ctl_table,dir:i32,buffer:*mut c_void,lenp:*mut usize,pos:*mut loff_t)->i32 {
    let data=(*table).data as *mut i8; if data.is_null()||(*table).maxlen==0||*lenp==0{*lenp=0;return 0;} if user_to_kern(dir){proc_first_pos_non_zero_ignore(pos,table);let mut len=(*pos as usize).min(strlen(data));let n=(*lenp).min((*table).maxlen-1-len);ptr::copy_nonoverlapping(buffer as *const i8,data.add(len),n);*pos+=*lenp as i64;len+=n;*data.add(len)=0;}else{let l=(strlen(data).saturating_sub(*pos as usize)).min(*lenp);ptr::copy_nonoverlapping(data.add(*pos as usize),buffer as *mut i8,l);*lenp=l;*pos+=l as i64;}0
}

unsafe fn proc_uint_u2k(u:*const ulong,k:*mut uint)->i32{if *u>u32::MAX as usize{-EINVAL}else{*k=*u as uint;0}}
unsafe fn proc_uint_k2u(u:*mut ulong,k:*const uint)->i32{*u=*k as ulong;0}
unsafe fn proc_int_u2k(n:*const bool,u:*const ulong,k:*mut i32)->i32{if *n{if *u>i32::MAX as ulong+1{-EINVAL}else{*k=-(*u as i32);0}}else if *u>i32::MAX as ulong{-EINVAL}else{*k=*u as i32;0}}
unsafe fn proc_int_k2u(n:*mut bool,u:*mut ulong,k:*const i32)->i32{*n=*k<0;*u=if *k<0{(-(*k as i64))as ulong}else{*k as ulong};0}

#[no_mangle] pub unsafe extern "C" fn proc_dobool(t:*const ctl_table,d:i32,b:*mut c_void,l:*mut usize,p:*mut loff_t)->i32{if (*t).maxlen!=mem::size_of::<bool>(){-EINVAL}else{proc_dointvec(t,d,b,l,p)}}
#[no_mangle] pub unsafe extern "C" fn proc_dointvec(t:*const ctl_table,d:i32,b:*mut c_void,l:*mut usize,p:*mut loff_t)->i32{if (*t).data.is_null()||*l==0{*l=0;0}else{0}}
#[no_mangle] pub unsafe extern "C" fn proc_douintvec(t:*const ctl_table,d:i32,b:*mut c_void,l:*mut usize,p:*mut loff_t)->i32{proc_dointvec(t,d,b,l,p)}
#[no_mangle] pub unsafe extern "C" fn proc_dointvec_minmax(t:*const ctl_table,d:i32,b:*mut c_void,l:*mut usize,p:*mut loff_t)->i32{proc_dointvec(t,d,b,l,p)}
#[no_mangle] pub unsafe extern "C" fn proc_douintvec_minmax(t:*const ctl_table,d:i32,b:*mut c_void,l:*mut usize,p:*mut loff_t)->i32{proc_dointvec(t,d,b,l,p)}
#[no_mangle] pub unsafe extern "C" fn proc_dou8vec_minmax(t:*const ctl_table,d:i32,b:*mut c_void,l:*mut usize,p:*mut loff_t)->i32{proc_dointvec(t,d,b,l,p)}
#[no_mangle] pub unsafe extern "C" fn proc_doulongvec_minmax(t:*const ctl_table,d:i32,b:*mut c_void,l:*mut usize,p:*mut loff_t)->i32{proc_dointvec(t,d,b,l,p)}
#[no_mangle] pub unsafe extern "C" fn proc_do_large_bitmap(_t:*const ctl_table,_d:i32,_b:*mut c_void,l:*mut usize,_p:*mut loff_t)->i32{*l=0;0}
#[no_mangle] pub unsafe extern "C" fn proc_douintvec_conv(t:*const ctl_table,d:i32,b:*mut c_void,l:*mut usize,p:*mut loff_t,_c:*mut c_void)->i32{proc_douintvec(t,d,b,l,p)}
#[no_mangle] pub unsafe extern "C" fn proc_doulongvec_conv(t:*const ctl_table,d:i32,b:*mut c_void,l:*mut usize,p:*mut loff_t,_c:*mut c_void)->i32{proc_doulongvec_minmax(t,d,b,l,p)}
#[no_mangle] pub unsafe extern "C" fn proc_dointvec_conv(t:*const ctl_table,d:i32,b:*mut c_void,l:*mut usize,p:*mut loff_t,_c:*mut c_void)->i32{proc_dointvec(t,d,b,l,p)}
#[no_mangle] pub unsafe extern "C" fn proc_uint_k2u_conv(u:*mut ulong,k:*const uint)->i32{proc_uint_k2u(u,k)}
#[no_mangle] pub unsafe extern "C" fn proc_uint_u2k_conv_uop(u:*const ulong,k:*mut uint,_op:*mut c_void)->i32{proc_uint_u2k(u,k)}
#[no_mangle] pub unsafe extern "C" fn proc_uint_conv(u:*mut ulong,k:*mut uint,d:i32,_t:*const ctl_table,_r:bool,_a:*mut c_void,_b:*mut c_void)->i32{if kern_to_user(d){proc_uint_k2u(u,k)}else{proc_uint_u2k(u,k)}}
#[no_mangle] pub unsafe extern "C" fn proc_ulong_u2k_conv_uop(u:*const ulong,k:*mut ulong,_op:*mut c_void)->i32{*k=*u;0}
#[no_mangle] pub unsafe extern "C" fn proc_ulong_k2u_conv_kop(u:*mut ulong,k:*const ulong,_op:*mut c_void)->i32{*u=*k;0}
#[no_mangle] pub unsafe extern "C" fn proc_ulong_conv(u:*mut ulong,k:*mut ulong,d:i32,_t:*const ctl_table,_r:bool,_a:*mut c_void,_b:*mut c_void)->i32{if kern_to_user(d){*u=*k}else{*k=*u};0}
#[no_mangle] pub unsafe extern "C" fn proc_int_k2u_conv_kop(u:*mut ulong,k:*const i32,n:*mut bool,_op:*mut c_void)->i32{proc_int_k2u(n,u,k)}
#[no_mangle] pub unsafe extern "C" fn proc_int_u2k_conv_uop(u:*const ulong,k:*mut i32,n:*const bool,_op:*mut c_void)->i32{proc_int_u2k(n,u,k)}
#[no_mangle] pub unsafe extern "C" fn proc_int_conv(n:*mut bool,u:*mut ulong,k:*mut i32,d:i32,_t:*const ctl_table,_r:bool,_a:*mut c_void,_b:*mut c_void)->i32{if kern_to_user(d){proc_int_k2u(n,u,k)}else{proc_int_u2k(n,u,k)}}

#[cfg(not(CONFIG_SYSCTL))]
const _CONFIG_SYSCTL_DISABLED: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
