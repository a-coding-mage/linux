// SPDX-License-Identifier: MIT
/*
 * Copyright (C) 2012-2014 Canonical Ltd (Maarten Lankhorst)
 *
 * Based on bo.c; see the original source for the complete dual-license text.
 */

// Linux dependencies supplied by other translation units.
use core::ffi::c_void;

const DMA_RESV_LIST_MASK: usize = 0x3;

#[repr(C)]
pub struct dma_resv_list {
    pub rcu: rcu_head,
    pub num_fences: u32,
    pub max_fences: u32,
    pub table: [*mut dma_fence; 0],
}

#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct dma_resv { pub lock: ww_mutex, pub fences: *mut dma_resv_list }
#[repr(C)] pub struct dma_fence { pub context: u64 }
#[repr(C)] pub struct dma_fence_array { pub base: dma_fence }
#[repr(C)] pub struct ww_mutex { _private: [u8; 0] }
#[repr(C)] pub struct dma_resv_iter {
    pub obj: *mut dma_resv, pub fences: *mut dma_resv_list, pub fence: *mut dma_fence,
    pub index: u32, pub num_fences: u32, pub usage: dma_resv_usage,
    pub fence_usage: dma_resv_usage, pub is_restarted: bool,
}
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct ktime_t { _private: [u8; 0] }
#[repr(C)] pub struct ww_acquire_ctx { _private: [u8; 0] }
#[repr(C)] pub struct mm_struct { _private: [u8; 0] }
#[repr(C)] pub struct address_space { _private: [u8; 0] }

#[repr(i32)] #[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum dma_resv_usage { KERNEL = 0, WRITE = 1, READ = 2, BOOKKEEP = 3 }

extern "C" {
    static mut reservation_ww_class: c_void;
    fn dma_resv_held(_: *mut dma_resv) -> bool;
    fn dma_resv_assert_held(_: *mut dma_resv);
    fn dma_fence_get(_: *mut dma_fence) -> *mut dma_fence;
    fn dma_fence_put(_: *mut dma_fence);
    fn dma_fence_is_signaled(_: *mut dma_fence) -> bool;
    fn dma_fence_is_container(_: *mut dma_fence) -> bool;
    fn dma_fence_is_later_or_same(_: *mut dma_fence, _: *mut dma_fence) -> bool;
    fn dma_fence_get_rcu(_: *mut dma_fence) -> *mut dma_fence;
    fn dma_fence_wait_timeout(_: *mut dma_fence, _: bool, _: usize) -> isize;
    fn dma_fence_set_deadline(_: *mut dma_fence, _: ktime_t);
    fn dma_fence_describe(_: *mut dma_fence, _: *mut seq_file);
    fn dma_fence_context_alloc(_: u32) -> u64;
    fn dma_fence_array_create(_: u32, _: *mut *mut dma_fence, _: u64, _: u32) -> *mut dma_fence_array;
    fn dma_resv_iter_begin(_: *mut dma_resv_iter, _: *mut dma_resv, _: dma_resv_usage);
    fn dma_resv_iter_end(_: *mut dma_resv_iter);
    fn dma_resv_iter_is_restarted(_: *mut dma_resv_iter) -> bool;
    fn dma_resv_iter_usage(_: *mut dma_resv_iter) -> dma_resv_usage;
    fn ww_mutex_init(_: *mut ww_mutex, _: *mut c_void); fn ww_mutex_destroy(_: *mut ww_mutex);
    fn ww_mutex_unlock(_: *mut ww_mutex); fn ww_acquire_init(_: *mut ww_acquire_ctx, _: *mut c_void);
    fn ww_acquire_fini(_: *mut ww_acquire_ctx); fn dma_resv_lock(_: *mut dma_resv, _: *mut ww_acquire_ctx) -> i32;
    fn dma_resv_lock_slow(_: *mut dma_resv, _: *mut ww_acquire_ctx);
    fn rcu_read_lock(); fn rcu_read_unlock();
    fn dma_resv_iter_next(_: *mut dma_resv_iter) -> *mut dma_fence;
    fn dma_resv_fences_list(_: *mut dma_resv) -> *mut dma_resv_list;
    fn kmalloc(_: usize, _: u32) -> *mut c_void; fn kfree(_: *mut c_void);
    fn kmalloc_size_roundup(_: usize) -> usize; fn roundup_pow_of_two(_: usize) -> usize;
    fn krealloc_array(_: *mut c_void, _: usize, _: usize, _: u32) -> *mut c_void;
    fn seq_printf(_: *mut seq_file, _: *const i8, ...);
}

unsafe fn dma_resv_list_entry(list: *mut dma_resv_list, index: usize, fence: *mut *mut dma_fence, usage: *mut dma_resv_usage) {
    let tmp = (*list).table.as_ptr().add(index).read() as usize;
    *fence = (tmp & !DMA_RESV_LIST_MASK) as *mut dma_fence;
    if !usage.is_null() { *usage = (tmp & DMA_RESV_LIST_MASK) as dma_resv_usage; }
}
unsafe fn dma_resv_list_set(list: *mut dma_resv_list, index: usize, fence: *mut dma_fence, usage: dma_resv_usage) {
    (*list).table.as_mut_ptr().add(index).write(((fence as usize) | usage as usize) as *mut dma_fence);
}
unsafe fn dma_resv_list_alloc(max_fences: usize) -> *mut dma_resv_list {
    let size = kmalloc_size_roundup(core::mem::size_of::<dma_resv_list>() + max_fences * core::mem::size_of::<*mut dma_fence>());
    let list = kmalloc(size, 0) as *mut dma_resv_list;
    if list.is_null() { return core::ptr::null_mut(); }
    (*list).max_fences = ((size - core::mem::size_of::<rcu_head>() - 8) / core::mem::size_of::<*mut dma_fence>()) as u32;
    list
}
unsafe fn dma_resv_list_free(list: *mut dma_resv_list) {
    if list.is_null() { return; }
    for i in 0..(*list).num_fences { let mut f = core::ptr::null_mut(); dma_resv_list_entry(list, i as usize, &mut f, core::ptr::null_mut()); dma_fence_put(f); }
    kfree(list as *mut c_void);
}

pub unsafe fn dma_resv_init(obj: *mut dma_resv) { ww_mutex_init(&mut (*obj).lock, &mut reservation_ww_class); (*obj).fences = core::ptr::null_mut(); }
pub unsafe fn dma_resv_fini(obj: *mut dma_resv) { dma_resv_list_free((*obj).fences); ww_mutex_destroy(&mut (*obj).lock); }

// The remaining public operations retain the source API and are expressed using
// the same external Linux primitives; iterator macros are represented by loops.
pub unsafe fn dma_resv_iter_first(cursor: *mut dma_resv_iter) -> *mut dma_fence { dma_resv_assert_held((*cursor).obj); (*cursor).index = 0; (*cursor).fences = dma_resv_fences_list((*cursor).obj); (*cursor).is_restarted = true; dma_resv_iter_next(cursor) }

pub unsafe fn dma_resv_reserve_fences(obj: *mut dma_resv, num: u32) -> i32 {
    dma_resv_assert_held(obj); if num == 0 { return -22; }
    let old = dma_resv_fences_list(obj); let oldn = if old.is_null(){0}else{(*old).num_fences};
    let oldmax = if old.is_null(){0}else{(*old).max_fences};
    if oldmax != 0 && oldn.saturating_add(num) <= oldmax { return 0; }
    let max = if oldmax != 0 { core::cmp::max(oldn + num, oldmax * 2) } else { core::cmp::max(4, roundup_pow_of_two(num as usize) as u32) };
    let new = dma_resv_list_alloc(max as usize); if new.is_null(){return -12;}
    let mut j=0; for i in 0..oldn { let mut f=core::ptr::null_mut(); let mut u=dma_resv_usage::KERNEL; dma_resv_list_entry(old,i as usize,&mut f,&mut u); if !dma_fence_is_signaled(f){dma_resv_list_set(new,j as usize,f,u);j+=1;}else{dma_fence_put(f);} }
    (*new).num_fences=j; (*obj).fences=new; 0
}
pub unsafe fn dma_resv_add_fence(obj:*mut dma_resv, fence:*mut dma_fence, usage:dma_resv_usage) {
    dma_fence_get(fence); dma_resv_assert_held(obj); let list=dma_resv_fences_list(obj); let count=(*list).num_fences;
    for i in 0..count { let mut old=core::ptr::null_mut(); let mut ou=dma_resv_usage::KERNEL; dma_resv_list_entry(list,i as usize,&mut old,&mut ou); if ( (*old).context==(*fence).context && ou>=usage && dma_fence_is_later_or_same(fence,old)) || dma_fence_is_signaled(old) {dma_resv_list_set(list,i as usize,fence,usage);dma_fence_put(old);return;} }
    dma_resv_list_set(list,count as usize,fence,usage); (*list).num_fences=count+1;
}
pub unsafe fn dma_resv_replace_fences(obj:*mut dma_resv, context:u64, replacement:*mut dma_fence, usage:dma_resv_usage) { dma_resv_assert_held(obj); let l=dma_resv_fences_list(obj); if l.is_null(){return;} for i in 0..(*l).num_fences {let mut f=core::ptr::null_mut();dma_resv_list_entry(l,i as usize,&mut f,core::ptr::null_mut());if (*f).context==context{dma_resv_list_set(l,i as usize,dma_fence_get(replacement),usage);dma_fence_put(f);}} }
pub unsafe fn dma_resv_iter_next_locked(c:*mut dma_resv_iter)->*mut dma_fence { dma_resv_assert_held((*c).obj); (*c).is_restarted=false; loop {if (*c).fences.is_null()||(*c).index>=(*(*c).fences).num_fences{return core::ptr::null_mut();}let mut f=core::ptr::null_mut();dma_resv_list_entry((*c).fences,(*c).index as usize,&mut f,&mut (*c).fence_usage);(*c).index+=1;if (*c).fence_usage<=(*c).usage{return f;}} }
pub unsafe fn dma_resv_get_fences(obj:*mut dma_resv, usage:dma_resv_usage, n:*mut u32, out:*mut *mut *mut dma_fence)->i32 { *n=0;*out=core::ptr::null_mut(); let l=dma_resv_fences_list(obj);if l.is_null(){return 0;}let p=kmalloc((*l).num_fences as usize*core::mem::size_of::<*mut dma_fence>(),0) as *mut *mut dma_fence;if p.is_null()&&(*l).num_fences!=0{return -12;}for i in 0..(*l).num_fences{let mut f=core::ptr::null_mut();let mut u=dma_resv_usage::KERNEL;dma_resv_list_entry(l,i as usize,&mut f,&mut u);if u<=usage{p.add(*n as usize).write(dma_fence_get(f));*n+=1;}}*out=p;0 }
pub unsafe fn dma_resv_test_signaled(obj:*mut dma_resv, usage:dma_resv_usage)->bool {let l=dma_resv_fences_list(obj);if l.is_null(){return true;}for i in 0..(*l).num_fences{let mut f=core::ptr::null_mut();let mut u=dma_resv_usage::KERNEL;dma_resv_list_entry(l,i as usize,&mut f,&mut u);if u<=usage&&!dma_fence_is_signaled(f){return false;}}true}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
