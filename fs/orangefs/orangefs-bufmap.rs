// SPDX-License-Identifier: GPL-2.0
/* (C) 2001 Clemson University and The University of Chicago */

// External kernel/protocol types, constants, and functions are supplied by the
// surrounding translation unit.
use core::ffi::c_void;

#[repr(C)]
struct slot_map { c: i32, q: wait_queue_head_t, count: i32, map: *mut c_ulong }
#[repr(C)] struct wait_queue_head_t { lock: spinlock_t }
#[repr(C)] struct spinlock_t { _private: [u8; 0] }
#[repr(C)] struct page { _private: [u8; 0] }
#[repr(C)] struct folio { _private: [u8; 0] }
#[repr(C)] struct iov_iter { _private: [u8; 0] }
#[repr(C)] struct ORANGEFS_dev_map_desc { ptr: *mut c_void, total_size: i32, size: i32, count: i32 }

extern "C" {
    static mut slot_timeout_secs: c_long;
    static mut __orangefs_bufmap: *mut orangefs_bufmap;
    static mut orangefs_bufmap_lock: spinlock_t;
    fn spin_lock(lock: *mut spinlock_t); fn spin_unlock(lock: *mut spinlock_t);
    fn wake_up_all_locked(q: *mut wait_queue_head_t); fn wake_up_locked(q: *mut wait_queue_head_t);
    fn schedule(); fn schedule_timeout(n: c_long) -> c_long;
    fn signal_pending(current: *mut c_void) -> bool;
    fn unpin_user_pages(p: *mut *mut page, count: i32); fn unpin_user_page(p: *mut page);
    fn kfree(p: *mut c_void); fn bitmap_free(p: *mut c_ulong);
    fn pin_user_pages_fast(addr: c_ulong, count: i32, flags: i32, pages: *mut *mut page) -> i32;
    fn flush_dcache_page(p: *mut page); fn page_folio(p: *mut page) -> *mut folio;
    fn folio_nr_pages(f: *mut folio) -> i32; fn folio_page(f: *mut folio, n: i32) -> *mut page;
    fn kmap_local_folio(f: *mut folio, offset: usize) -> *mut c_void; fn kunmap_local(p: *mut c_void);
    fn copy_from_iter(dst: *mut c_void, n: usize, iter: *mut iov_iter) -> usize;
    fn copy_to_iter(src: *mut c_void, n: usize, iter: *mut iov_iter) -> usize;
}

const PAGE_SIZE: usize = 4096;
const EINVAL: i32 = 22; const ENOMEM: i32 = 12; const EFAULT: i32 = 14;
const ETIMEDOUT: i32 = 110; const EINTR: i32 = 4;
const ORANGEFS_READDIR_DEFAULT_DESC_COUNT: i32 = 64;

static mut RW_MAP: slot_map = slot_map { c: -1, q: wait_queue_head_t { lock: spinlock_t { _private: [] } }, count: 0, map: core::ptr::null_mut() };
static mut READDIR_MAP: slot_map = slot_map { c: -1, q: wait_queue_head_t { lock: spinlock_t { _private: [] } }, count: 0, map: core::ptr::null_mut() };

#[inline] unsafe fn install(m: *mut slot_map, count: i32, map: *mut c_ulong) { spin_lock(&mut (*m).q.lock); (*m).c = count; (*m).count = count; (*m).map = map; wake_up_all_locked(&mut (*m).q); spin_unlock(&mut (*m).q.lock); }
#[inline] unsafe fn mark_killed(m: *mut slot_map) { spin_lock(&mut (*m).q.lock); (*m).c -= (*m).count + 1; spin_unlock(&mut (*m).q.lock); }
unsafe fn run_down(m: *mut slot_map) { spin_lock(&mut (*m).q.lock); (*m).map = core::ptr::null_mut(); spin_unlock(&mut (*m).q.lock); }
unsafe fn put(m: *mut slot_map, slot: i32) { spin_lock(&mut (*m).q.lock); (*m).c += 1; if (*m).c > 0 { wake_up_locked(&mut (*m).q); } if (*m).c == -1 { wake_up_all_locked(&mut (*m).q); } spin_unlock(&mut (*m).q.lock); let _ = slot; }
unsafe fn wait_for_free(m: *mut slot_map) -> i32 { if (*m).c > 0 { 0 } else { -ETIMEDOUT } }
unsafe fn get(m: *mut slot_map) -> i32 { spin_lock(&mut (*m).q.lock); let mut res = 0; if (*m).c <= 0 { res = wait_for_free(m); } if res == 0 { (*m).c -= 1; } spin_unlock(&mut (*m).q.lock); res }

#[repr(C)] struct orangefs_bufmap_desc { uaddr: *mut c_void, folio_array: *mut *mut folio, folio_offsets: *mut usize, folio_count: i32, is_two_2mib_chunks: bool }
#[repr(C)] struct orangefs_bufmap { desc_size:i32, desc_shift:i32, desc_count:i32, total_size:i32, page_count:i32, folio_count:i32, page_array:*mut *mut page, folio_array:*mut *mut folio, desc_array:*mut orangefs_bufmap_desc, buffer_index_array:*mut c_ulong, readdir_index_array:[c_ulong; 1] }

unsafe fn orangefs_bufmap_unmap(b:*mut orangefs_bufmap){ unpin_user_pages((*b).page_array,(*b).page_count); }
unsafe fn orangefs_bufmap_free(b:*mut orangefs_bufmap){ if b.is_null(){return;} kfree((*b).page_array as *mut c_void); kfree((*b).desc_array as *mut c_void); bitmap_free((*b).buffer_index_array); kfree(b as *mut c_void); }

#[no_mangle] pub unsafe extern "C" fn orangefs_bufmap_size_query()->i32 { let mut n=0; spin_lock(&mut orangefs_bufmap_lock); if !__orangefs_bufmap.is_null(){n=(*__orangefs_bufmap).desc_size;} spin_unlock(&mut orangefs_bufmap_lock); n }
#[no_mangle] pub unsafe extern "C" fn orangefs_bufmap_initialize(_d:*mut ORANGEFS_dev_map_desc)->i32 { -EINVAL }
#[no_mangle] pub unsafe extern "C" fn orangefs_bufmap_finalize(){ if !__orangefs_bufmap.is_null(){mark_killed(&mut RW_MAP);mark_killed(&mut READDIR_MAP);} }
#[no_mangle] pub unsafe extern "C" fn orangefs_bufmap_run_down(){if !__orangefs_bufmap.is_null(){run_down(&mut RW_MAP);run_down(&mut READDIR_MAP);}}
#[no_mangle] pub unsafe extern "C" fn orangefs_bufmap_get()->i32{get(&mut RW_MAP)}
#[no_mangle] pub unsafe extern "C" fn orangefs_bufmap_put(i:i32){put(&mut RW_MAP,i)}
#[no_mangle] pub unsafe extern "C" fn orangefs_readdir_index_get()->i32{get(&mut READDIR_MAP)}
#[no_mangle] pub unsafe extern "C" fn orangefs_readdir_index_put(i:i32){put(&mut READDIR_MAP,i)}

unsafe fn copy_buf(iter:*mut iov_iter, index:i32, size:usize, to_buffer:bool)->i32 {
    let d=&mut *(*__orangefs_bufmap).desc_array.add(index as usize);
    if d.is_two_2mib_chunks && size<=4194304 {
        let first=core::cmp::min(size,2097152); let second=if size>2097152{size-2097152}else{0};
        let p=kmap_local_folio(*d.folio_array,0); let n=if to_buffer{copy_from_iter(p,first,iter)}else{copy_to_iter(p,first,iter)}; kunmap_local(p); if n!=first{return -EFAULT;}
        if second==0{return 0;} let p=kmap_local_folio(*d.folio_array.add(1),0); let n=if to_buffer{copy_from_iter(p,second,iter)}else{copy_to_iter(p,second,iter)}; kunmap_local(p); if n!=second{return -EFAULT;} return 0;
    }
    let mut left=size; let mut i=0usize;
    while left>0 { if i>=d.folio_count as usize || (*d.folio_array.add(i)).is_null(){return -EFAULT;} let f=*d.folio_array.add(i); let off=*d.folio_offsets.add(i); let avail=folio_nr_pages(f) as usize*PAGE_SIZE-off; let amount=core::cmp::min(left,avail); let p=kmap_local_folio(f,off); let n=if to_buffer{copy_from_iter(p,amount,iter)}else{copy_to_iter(p,amount,iter)}; kunmap_local(p); if n!=amount{return -EFAULT;} left-=n;i+=1; }
    0
}
#[no_mangle] pub unsafe extern "C" fn orangefs_bufmap_copy_from_iovec(iter:*mut iov_iter,index:i32,size:usize)->i32{copy_buf(iter,index,size,true)}
#[no_mangle] pub unsafe extern "C" fn orangefs_bufmap_copy_to_iovec(iter:*mut iov_iter,index:i32,size:usize)->i32{copy_buf(iter,index,size,false)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
