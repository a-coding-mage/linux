// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) 2019 Samsung Electronics Co., Ltd.
 */

// Linux and project headers provide the types, constants, macros, and
// functions referenced below.

use core::ffi::c_void;

static mut work_cache: *mut kmem_cache = core::ptr::null_mut();
static mut ksmbd_wq: *mut workqueue_struct = core::ptr::null_mut();

#[repr(C)] pub struct kmem_cache { _private: [u8; 0] }
#[repr(C)] pub struct workqueue_struct { _private: [u8; 0] }
#[repr(C)] pub struct kvec { pub iov_base: *mut c_void, pub iov_len: usize }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct ksmbd_conn { pub async_ida: *mut c_void }
#[repr(C)] pub struct aux_read { pub entry: list_head, pub buf: *mut c_void }

#[repr(C)] pub struct ksmbd_work {
    pub compound_fid: u64, pub compound_pfid: u64,
    pub request_entry: list_head, pub async_request_entry: list_head,
    pub fp_entry: list_head, pub notify_entry: list_head,
    pub aux_read_list: list_head,
    pub iov_alloc_cnt: i32, pub iov_cnt: i32, pub iov_idx: i32,
    pub iov_inline: [kvec; 4], pub iov: *mut kvec,
    pub saved_cred: *mut c_void, pub response_buf: *mut c_void,
    pub tr_buf: *mut c_void, pub compress_buf: *mut c_void,
    pub request_buf: *mut c_void, pub async_id: u64,
    pub conn: *mut ksmbd_conn, pub owns_conn_ref: bool,
    pub request_open: *mut c_void, pub response_sz: usize,
    pub work: work_struct,
}

extern "C" {
    fn kcalloc(n: usize, size: usize, flags: u32) -> *mut kvec;
    fn krealloc(p: *mut kvec, size: usize, flags: u32) -> *mut kvec;
    fn kmem_cache_zalloc(c: *mut kmem_cache, flags: u32) -> *mut ksmbd_work;
    fn kmem_cache_free(c: *mut kmem_cache, p: *mut ksmbd_work);
    fn kmem_cache_create(n: *const u8, size: usize, align: usize, flags: u32, ctor: *mut c_void) -> *mut kmem_cache;
    fn kmem_cache_destroy(c: *mut kmem_cache);
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kmalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(p: *mut c_void); fn kvfree(p: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn alloc_workqueue(n: *const u8, flags: u32, max_active: u32) -> *mut workqueue_struct;
    fn destroy_workqueue(wq: *mut workqueue_struct);
    fn queue_work(wq: *mut workqueue_struct, work: *mut work_struct) -> bool;
    fn ksmbd_release_id(ida: *mut c_void, id: u64);
    fn ksmbd_conn_put(conn: *mut ksmbd_conn);
    fn ksmbd_fd_put(work: *mut ksmbd_work, open: *mut c_void);
    fn inc_rfc1001_len(buf: *mut c_void, len: i32);
}

const KSMBD_WORK_INLINE_IOVS: i32 = 4;
const KSMBD_DEFAULT_GFP: u32 = 0;
const __GFP_ZERO: u32 = 0;
const SLAB_HWCACHE_ALIGN: u32 = 0;
const WQ_PERCPU: u32 = 0;
const MAX_CIFS_SMALL_BUFFER_SIZE: usize = 65536;
const KSMBD_NO_FID: u64 = u64::MAX;

unsafe fn list_init(h: *mut list_head) { (*h).next = h; (*h).prev = h; }

unsafe fn ksmbd_reserve_iov(work: *mut ksmbd_work, need_iov_cnt: i32) -> i32 {
    let mut new_alloc_cnt = (*work).iov_alloc_cnt;
    if new_alloc_cnt >= (*work).iov_cnt + need_iov_cnt { return 0; }
    while new_alloc_cnt < (*work).iov_cnt + need_iov_cnt { new_alloc_cnt += KSMBD_WORK_INLINE_IOVS; }
    let new;
    if (*work).iov == (*work).iov_inline.as_mut_ptr() {
        new = kcalloc(new_alloc_cnt as usize, core::mem::size_of::<kvec>(), KSMBD_DEFAULT_GFP);
        if new.is_null() { return -12; }
        memcpy(new as *mut c_void, (*work).iov_inline.as_ptr() as *const c_void, core::mem::size_of_val(&(*work).iov_inline));
    } else {
        new = krealloc((*work).iov, core::mem::size_of::<kvec>() * new_alloc_cnt as usize, KSMBD_DEFAULT_GFP | __GFP_ZERO);
        if new.is_null() { return -12; }
    }
    (*work).iov = new; (*work).iov_alloc_cnt = new_alloc_cnt; 0
}

#[no_mangle] pub unsafe extern "C" fn ksmbd_alloc_work_struct() -> *mut ksmbd_work {
    let work = kmem_cache_zalloc(work_cache, KSMBD_DEFAULT_GFP);
    if !work.is_null() {
        (*work).compound_fid = KSMBD_NO_FID; (*work).compound_pfid = KSMBD_NO_FID;
        list_init(&mut (*work).request_entry); list_init(&mut (*work).async_request_entry);
        list_init(&mut (*work).fp_entry); list_init(&mut (*work).notify_entry); list_init(&mut (*work).aux_read_list);
        (*work).iov_alloc_cnt = (*work).iov_inline.len() as i32; (*work).iov = (*work).iov_inline.as_mut_ptr();
    } work
}

#[no_mangle] pub unsafe extern "C" fn ksmbd_free_work_struct(work: *mut ksmbd_work) {
    kvfree((*work).response_buf); kfree((*work).tr_buf); kvfree((*work).compress_buf); kvfree((*work).request_buf);
    if (*work).iov != (*work).iov_inline.as_mut_ptr() { kfree((*work).iov as *mut c_void); }
    if (*work).async_id != 0 { ksmbd_release_id((*(*work).conn).async_ida, (*work).async_id); }
    if (*work).owns_conn_ref { ksmbd_conn_put((*work).conn); }
    ksmbd_fd_put(work, (*work).request_open); kmem_cache_free(work_cache, work);
}

#[no_mangle] pub unsafe extern "C" fn ksmbd_work_pool_destroy() { kmem_cache_destroy(work_cache); }
#[no_mangle] pub unsafe extern "C" fn ksmbd_work_pool_init() -> i32 { work_cache = kmem_cache_create(b"ksmbd_work_cache\0".as_ptr(), core::mem::size_of::<ksmbd_work>(), 0, SLAB_HWCACHE_ALIGN, core::ptr::null_mut()); if work_cache.is_null() {-12} else {0} }
#[no_mangle] pub unsafe extern "C" fn ksmbd_workqueue_init() -> i32 { ksmbd_wq = alloc_workqueue(b"ksmbd-io\0".as_ptr(), WQ_PERCPU, 0); if ksmbd_wq.is_null() {-12} else {0} }
#[no_mangle] pub unsafe extern "C" fn ksmbd_workqueue_destroy() { destroy_workqueue(ksmbd_wq); ksmbd_wq = core::ptr::null_mut(); }
#[no_mangle] pub unsafe extern "C" fn ksmbd_queue_work(work: *mut ksmbd_work) -> bool { queue_work(ksmbd_wq, &mut (*work).work) }

unsafe fn __ksmbd_iov_pin(work: *mut ksmbd_work, ib: *mut c_void, len: usize) { (*work).iov_idx += 1; (*work).iov.add((*work).iov_idx as usize).write(kvec{iov_base:ib,iov_len:len}); (*work).iov_cnt += 1; }
unsafe fn __ksmbd_iov_pin_rsp(work:*mut ksmbd_work, ib:*mut c_void, len:i32, aux_buf:*mut c_void, aux_size:usize)->i32 { if ksmbd_reserve_iov(work, if aux_size != 0 {2}else{1}) != 0{return -12;} if (*work).iov_idx==0 {(*work).iov_idx=0; (*work).iov_cnt+=1;} __ksmbd_iov_pin(work,ib,len as usize); inc_rfc1001_len((*work).iov[0].iov_base,len); if aux_size!=0 {__ksmbd_iov_pin(work,aux_buf,aux_size);inc_rfc1001_len((*work).iov[0].iov_base,aux_size as i32);} 0 }
#[no_mangle] pub unsafe extern "C" fn ksmbd_iov_pin_rsp(w:*mut ksmbd_work,b:*mut c_void,l:i32)->i32 {__ksmbd_iov_pin_rsp(w,b,l,core::ptr::null_mut(),0)}
#[no_mangle] pub unsafe extern "C" fn ksmbd_iov_pin_rsp_read(w:*mut ksmbd_work,b:*mut c_void,l:i32,a:*mut c_void,s:usize)->i32 {__ksmbd_iov_pin_rsp(w,b,l,a,s)}
#[no_mangle] pub unsafe extern "C" fn allocate_interim_rsp_buf(w:*mut ksmbd_work)->i32 {(*w).response_buf=kzalloc(MAX_CIFS_SMALL_BUFFER_SIZE,KSMBD_DEFAULT_GFP);if (*w).response_buf.is_null(){-12}else{(*w).response_sz=MAX_CIFS_SMALL_BUFFER_SIZE;0}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
