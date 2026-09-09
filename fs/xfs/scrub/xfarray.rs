// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of xfs/scrub/xfarray.c.  Kernel dependencies are external. */

use core::{ffi::c_void, ptr};

pub type xfarray_idx_t = i64;
pub type loff_t = i64;
pub type xfarray_cmp_fn = unsafe extern "C" fn(*const c_void, *const c_void) -> i32;

#[repr(C)] pub struct xfile;
#[repr(C)] pub struct folio;
#[repr(C)] pub struct xchk_relax { pub killable: bool }
#[repr(C)] pub struct xfarray {
    pub xfile: *mut xfile, pub obj_size: usize, pub obj_size_log: i32,
    pub max_nr: xfarray_idx_t, pub nr: xfarray_idx_t, pub unset_slots: u64,
}
#[repr(C)] pub struct xfarray_sortinfo {
    pub array: *mut xfarray, pub cmp_fn: xfarray_cmp_fn, pub flags: u32,
    pub max_stack_depth: i32, pub max_stack_used: u8, pub stack_depth: i32,
    pub relax: xchk_relax, pub folio: *mut folio,
    pub first_folio_idx: xfarray_idx_t, pub last_folio_idx: xfarray_idx_t,
}

extern "C" {
    fn xfile_create(*const i8, u32, *mut *mut xfile) -> i32;
    fn xfile_destroy(*mut xfile); fn xfile_load(*mut xfile,*mut c_void,usize,loff_t)->i32;
    fn xfile_store(*mut xfile,*const c_void,usize,loff_t)->i32;
    fn xfile_seek_data(*mut xfile,loff_t)->loff_t; fn xfile_bytes(*mut xfile)->u64;
    fn xfile_discard(*mut xfile,loff_t,loff_t); fn xfile_get_folio(*mut xfile,loff_t,u64,u32)->*mut folio;
    fn xfile_put_folio(*mut xfile,*mut folio); fn folio_address(*mut folio)->*mut u8;
    fn folio_pos(*mut folio)->loff_t; fn folio_next_pos(*mut folio)->loff_t;
    fn offset_in_folio(*mut folio,loff_t)->usize; fn xchk_maybe_relax(*mut xchk_relax)->bool;
    fn sort(*mut c_void,usize,usize,xfarray_cmp_fn,*mut c_void);
    fn memchr_inv(*const c_void,i32,usize)->*mut c_void;
}

const PAGE_SIZE: usize = 4096; const MAX_LFS_FILESIZE: loff_t = i64::MAX;
const XFARRAY_ISORT_NR: xfarray_idx_t = 16; const XFARRAY_ISORT_SHIFT: i32 = 4;
const XFARRAY_QSORT_PIVOT_NR: usize = 9; const XFARRAY_SORT_KILLABLE: u32 = 1;
const XFILE_ALLOC: u32 = 1; const XFILE_MAX_FOLIO_SIZE: u64 = 2 * 1024 * 1024;
const ENODATA: i32 = 61; const ENOMEM: i32 = 12; const EFBIG: i32 = 27;
const ENXIO: i32 = 6; const EINTR: i32 = 4; const E2BIG: i32 = 7; const EFSCORRUPTED: i32 = 990;

#[inline] unsafe fn scratch(a:*mut xfarray)->*mut c_void { a.add(1) as *mut c_void }
unsafe fn idx(a:*mut xfarray,p:loff_t)->xfarray_idx_t { if (*a).obj_size_log>=0 { p >> (*a).obj_size_log } else { p / (*a).obj_size as i64 } }
unsafe fn pos(a:*mut xfarray,i:xfarray_idx_t)->loff_t { if (*a).obj_size_log>=0 { i << (*a).obj_size_log } else { i * (*a).obj_size as i64 } }

#[no_mangle] pub unsafe extern "C" fn xfarray_create(d:*const i8, req:u64, size:usize, out:*mut *mut xfarray)->i32 {
    let mut x=ptr::null_mut(); let mut e=xfile_create(d,0,&mut x); if e!=0{return e};
    let a=libc_alloc(core::mem::size_of::<xfarray>()+size); if a.is_null(){xfile_destroy(x);return -ENOMEM}
    let a=a as *mut xfarray; ptr::write_bytes(a,0,1); (*a).xfile=x; (*a).obj_size=size;
    (*a).obj_size_log=if size.is_power_of_two(){size.trailing_zeros() as i32}else{-1}; (*a).max_nr=idx(a,MAX_LFS_FILESIZE);
    if req>0 {if (*a).max_nr<req as i64 {libc_free(a as *mut c_void);xfile_destroy(x);return -ENOMEM} (*a).max_nr=req as i64;} *out=a;0
}
extern "C" { fn libc_alloc(usize)->*mut c_void; fn libc_free(*mut c_void); }
#[no_mangle] pub unsafe extern "C" fn xfarray_destroy(a:*mut xfarray){xfile_destroy((*a).xfile);libc_free(a as *mut c_void)}
#[no_mangle] pub unsafe extern "C" fn xfarray_load(a:*mut xfarray,i:xfarray_idx_t,p:*mut c_void)->i32{if i>=(*a).nr{-ENODATA}else{xfile_load((*a).xfile,p,(*a).obj_size,pos(a,i))}}
unsafe fn is_null(a:*mut xfarray,p:*const c_void)->bool{memchr_inv(p,0,(*a).obj_size).is_null()}
unsafe fn is_unset(a:*mut xfarray,p:loff_t)->bool{if (*a).unset_slots==0{return false}let t=scratch(a);xfile_load((*a).xfile,t,(*a).obj_size,p)==0&&is_null(a,t)}
#[no_mangle] pub unsafe extern "C" fn xfarray_element_is_null(a:*mut xfarray,p:*const c_void)->bool{is_null(a,p)}
#[no_mangle] pub unsafe extern "C" fn xfarray_unset(a:*mut xfarray,i:xfarray_idx_t)->i32{if i>=(*a).nr{return -ENODATA}if i==(*a).nr-1{(*a).nr-=1;return 0}let p=pos(a,i);if is_unset(a,p){return 0}let t=scratch(a);ptr::write_bytes(t,0,(*a).obj_size);let e=xfile_store((*a).xfile,t,(*a).obj_size,p);if e==0{(*a).unset_slots+=1}e}
#[no_mangle] pub unsafe extern "C" fn xfarray_store(a:*mut xfarray,i:xfarray_idx_t,p:*const c_void)->i32{if i>=(*a).max_nr{return -EFBIG}let e=xfile_store((*a).xfile,p,(*a).obj_size,pos(a,i));if e==0&&i+1>(*a).nr{(*a).nr=i+1}e}
#[no_mangle] pub unsafe extern "C" fn xfarray_length(a:*mut xfarray)->u64{(*a).nr as u64}
#[no_mangle] pub unsafe extern "C" fn xfarray_store_anywhere(a:*mut xfarray,p:*const c_void)->i32{let t=scratch(a);let mut q=0;let end=pos(a,(*a).nr);while q<end&&(*a).unset_slots>0{if xfile_load((*a).xfile,t,(*a).obj_size,q)==0&&is_null(a,t){let e=xfile_store((*a).xfile,p,(*a).obj_size,q);if e!=0{return e}(*a).unset_slots-=1;return 0}q+=(*a).obj_size as i64}(*a).unset_slots=0;xfarray_store(a,(*a).nr,p)}
#[no_mangle] pub unsafe extern "C" fn xfarray_bytes(a:*mut xfarray)->u64{xfile_bytes((*a).xfile)}
#[no_mangle] pub unsafe extern "C" fn xfarray_truncate(a:*mut xfarray){xfile_discard((*a).xfile,0,MAX_LFS_FILESIZE);(*a).nr=0}
#[no_mangle] pub unsafe extern "C" fn xfarray_load_next(a:*mut xfarray, ip:*mut xfarray_idx_t, rec:*mut c_void)->i32 { let mut cur=*ip; while cur<(*a).nr { if xfarray_load(a,cur,rec)!=0{return -ENODATA}; cur+=1; if !is_null(a,rec){*ip=cur;return 0} } -ENODATA }
#[no_mangle] pub unsafe extern "C" fn xfarray_sort(a:*mut xfarray, cmp:xfarray_cmp_fn, _flags:u32)->i32 { if (*a).nr<2{return 0}; let n=(*a).nr as usize; let bytes=n*(*a).obj_size; let b=libc_alloc(bytes); if b.is_null(){return -ENOMEM}; for i in 0..n {if xfarray_load(a,i as i64,(b as *mut u8).add(i*(*a).obj_size) as *mut c_void)!=0{libc_free(b);return -ENODATA}} sort(b,n,(*a).obj_size,cmp,ptr::null_mut()); for i in 0..n {let e=xfarray_store(a,i as i64,(b as *mut u8).add(i*(*a).obj_size) as *const c_void);if e!=0{libc_free(b);return e}} libc_free(b);0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
