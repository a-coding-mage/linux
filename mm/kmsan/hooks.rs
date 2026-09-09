// SPDX-License-Identifier: GPL-2.0
/* KMSAN hooks for kernel subsystems. */

use core::ffi::c_void;

// Types, constants, globals, and functions supplied by the kernel/KMSAN headers.
#[repr(C)] pub struct task_struct { pub kmsan_ctx: kmsan_ctx }
#[repr(C)] pub struct kmsan_ctx { pub depth: u32 }
#[repr(C)] pub struct kmem_cache { pub ctor: *mut c_void, pub flags: u32, pub object_size: usize }
#[repr(C)] pub struct page;
#[repr(C)] pub struct scatterlist { pub length: usize }
#[repr(C)] pub struct urb { pub transfer_buffer: *mut c_void, pub transfer_buffer_length: usize }
#[repr(C)] pub struct pt_regs;
#[repr(C)] pub struct pgprot_t { pub pgprot: usize }
pub type gfp_t = u32;
pub type phys_addr_t = u64;
pub type dma_data_direction = u32;

extern "C" {
    static mut kmsan_enabled: bool;
    static mut current: *mut task_struct;
    fn kmsan_enter_runtime(); fn kmsan_leave_runtime(); fn kmsan_in_runtime() -> bool;
    fn kmsan_internal_task_create(task: *mut task_struct);
    fn kmsan_internal_unpoison_memory(addr: *mut c_void, size: usize, checked: bool);
    fn kmsan_internal_poison_memory(addr: *mut c_void, size: usize, flags: gfp_t, reason: u32);
    fn kmsan_internal_check_memory(addr: *mut c_void, size: usize, user_addr: *mut c_void, reason: u32);
    fn kmsan_internal_memmove_metadata(to: *mut c_void, from: *mut c_void, size: usize);
    fn kmsan_get_metadata(addr: *mut c_void, meta: u32) -> *mut c_void;
    fn __vunmap_range_noflush(start: usize, end: usize);
    fn flush_cache_vmap(start: usize, end: usize);
    fn virt_to_head_page(addr: *mut c_void) -> *mut page;
    fn page_address(p: *mut page) -> *mut c_void;
    fn page_size(p: *mut page) -> usize;
    fn alloc_pages(flags: gfp_t, order: u32) -> *mut page;
    fn __free_pages(p: *mut page, order: u32);
    fn __vmap_pages_range_noflush(start: usize, end: usize, prot: pgprot_t, pages: *mut *mut page, shift: u32) -> i32;
    fn kmsan_vmalloc_to_page_or_null(addr: *mut c_void) -> *mut page;
    fn user_access_save() -> usize; fn user_access_restore(flags: usize);
    fn phys_to_virt(phys: phys_addr_t) -> *mut c_void;
    fn offset_in_page(addr: *mut c_void) -> u64;
    fn sg_phys(sg: *mut scatterlist) -> phys_addr_t;
}

const __GFP_ZERO: gfp_t = 0x8000;
const GFP_KERNEL: gfp_t = 0x10;
const __GFP_RECLAIM: gfp_t = 0x400;
const SLAB_TYPESAFE_BY_RCU: u32 = 0x80000000;
const KMSAN_POISON_CHECK: u32 = 1;
const KMSAN_POISON_FREE: u32 = 2;
const KMSAN_POISON_NOCHECK: u32 = 4;
const KMSAN_META_SHADOW: u32 = 0;
const KMSAN_META_ORIGIN: u32 = 1;
const REASON_COPY_TO_USER: u32 = 0;
const REASON_SUBMIT_URB: u32 = 1;
const REASON_ANY: u32 = 2;
const PAGE_SIZE: usize = 4096;
const PAGE_SHIFT: u32 = 12;
const DMA_BIDIRECTIONAL: dma_data_direction = 0;
const DMA_TO_DEVICE: dma_data_direction = 1;
const DMA_FROM_DEVICE: dma_data_direction = 2;
const DMA_NONE: dma_data_direction = 3;
const ENOMEM: i32 = 12;

unsafe fn vmalloc_shadow(addr: usize) -> usize { kmsan_get_metadata(addr as *mut c_void, KMSAN_META_SHADOW) as usize }
unsafe fn vmalloc_origin(addr: usize) -> usize { kmsan_get_metadata(addr as *mut c_void, KMSAN_META_ORIGIN) as usize }

#[no_mangle] pub unsafe extern "C" fn kmsan_task_create(task: *mut task_struct) { kmsan_enter_runtime(); kmsan_internal_task_create(task); kmsan_leave_runtime(); }
#[no_mangle] pub unsafe extern "C" fn kmsan_task_exit(_task: *mut task_struct) { if !kmsan_enabled || kmsan_in_runtime() { return; } kmsan_disable_current(); }

#[no_mangle] pub unsafe extern "C" fn kmsan_slab_alloc(s: *mut kmem_cache, object: *mut c_void, flags: gfp_t) {
    if object.is_null() || !kmsan_enabled || kmsan_in_runtime() || !(*s).ctor.is_null() || ((*s).flags & SLAB_TYPESAFE_BY_RCU) != 0 { return; }
    kmsan_enter_runtime(); if flags & __GFP_ZERO != 0 { kmsan_internal_unpoison_memory(object, (*s).object_size, true); } else { kmsan_internal_poison_memory(object, (*s).object_size, flags, KMSAN_POISON_CHECK); } kmsan_leave_runtime();
}
#[no_mangle] pub unsafe extern "C" fn kmsan_slab_free(s: *mut kmem_cache, object: *mut c_void) {
    if !kmsan_enabled || kmsan_in_runtime() || ((*s).flags & SLAB_TYPESAFE_BY_RCU) != 0 || !(*s).ctor.is_null() { return; }
    kmsan_enter_runtime(); kmsan_internal_poison_memory(object, (*s).object_size, GFP_KERNEL & !__GFP_RECLAIM, KMSAN_POISON_CHECK | KMSAN_POISON_FREE); kmsan_leave_runtime();
}
#[no_mangle] pub unsafe extern "C" fn kmsan_kmalloc_large(ptr: *const c_void, size: usize, flags: gfp_t) { if ptr.is_null() || !kmsan_enabled || kmsan_in_runtime() { return; } kmsan_enter_runtime(); if flags & __GFP_ZERO != 0 { kmsan_internal_unpoison_memory(ptr as *mut c_void, size, true); } else { kmsan_internal_poison_memory(ptr as *mut c_void, size, flags, KMSAN_POISON_CHECK); } kmsan_leave_runtime(); }
#[no_mangle] pub unsafe extern "C" fn kmsan_kfree_large(ptr: *const c_void) { if !kmsan_enabled || kmsan_in_runtime() { return; } kmsan_enter_runtime(); let p = virt_to_head_page(ptr as *mut c_void); kmsan_internal_poison_memory(ptr as *mut c_void, page_size(p), GFP_KERNEL & !__GFP_RECLAIM, KMSAN_POISON_CHECK | KMSAN_POISON_FREE); kmsan_leave_runtime(); }

#[no_mangle] pub unsafe extern "C" fn kmsan_vunmap_range_noflush(start: usize, end: usize) { __vunmap_range_noflush(vmalloc_shadow(start), vmalloc_shadow(end)); __vunmap_range_noflush(vmalloc_origin(start), vmalloc_origin(end)); flush_cache_vmap(vmalloc_shadow(start), vmalloc_shadow(end)); flush_cache_vmap(vmalloc_origin(start), vmalloc_origin(end)); }

#[no_mangle] pub unsafe extern "C" fn kmsan_ioremap_page_range(start: usize, end: usize, _phys_addr: phys_addr_t, prot: pgprot_t, _page_shift: u32) -> i32 {
    if !kmsan_enabled || kmsan_in_runtime() { return 0; }
    let nr = (end - start) / PAGE_SIZE; let mut off = 0; let mut err = 0; let mut clean = 0; let mut shadow: *mut page = core::ptr::null_mut(); let mut origin: *mut page = core::ptr::null_mut(); kmsan_enter_runtime();
    for i in 0..nr { shadow = alloc_pages(GFP_KERNEL | __GFP_ZERO, 1); origin = alloc_pages(GFP_KERNEL | __GFP_ZERO, 1); if shadow.is_null() || origin.is_null() { err = -ENOMEM; clean = i; break; } let mut sp = shadow; let mapped = __vmap_pages_range_noflush(vmalloc_shadow(start+off), vmalloc_shadow(start+off+PAGE_SIZE), prot, &mut sp, PAGE_SHIFT); if mapped != 0 { err = mapped; clean = i; break; } shadow = core::ptr::null_mut(); let mut op = origin; let mapped = __vmap_pages_range_noflush(vmalloc_origin(start+off), vmalloc_origin(start+off+PAGE_SIZE), prot, &mut op, PAGE_SHIFT); if mapped != 0 { __vunmap_range_noflush(vmalloc_shadow(start+off), vmalloc_shadow(start+off+PAGE_SIZE)); err = mapped; clean = i; break; } origin = core::ptr::null_mut(); off += PAGE_SIZE; clean = i + 1; }
    if clean == nr { clean = 0; } if clean > 0 { if !shadow.is_null() { __free_pages(shadow,1); } if !origin.is_null() { __free_pages(origin,1); } __vunmap_range_noflush(vmalloc_shadow(start), vmalloc_shadow(start + clean*PAGE_SIZE)); __vunmap_range_noflush(vmalloc_origin(start), vmalloc_origin(start + clean*PAGE_SIZE)); } flush_cache_vmap(vmalloc_shadow(start), vmalloc_shadow(end)); flush_cache_vmap(vmalloc_origin(start), vmalloc_origin(end)); kmsan_leave_runtime(); err
}

#[no_mangle] pub unsafe extern "C" fn kmsan_iounmap_page_range(start: usize, end: usize) { if !kmsan_enabled || kmsan_in_runtime() { return; } let nr=(end-start)/PAGE_SIZE; kmsan_enter_runtime(); let mut vs=vmalloc_shadow(start); let mut vo=vmalloc_origin(start); for _ in 0..nr { let s=kmsan_vmalloc_to_page_or_null(vs as *mut c_void); let o=kmsan_vmalloc_to_page_or_null(vo as *mut c_void); __vunmap_range_noflush(vs,vmalloc_shadow(end)); __vunmap_range_noflush(vo,vmalloc_origin(end)); if !s.is_null(){__free_pages(s,1);} if !o.is_null(){__free_pages(o,1);} vs+=PAGE_SIZE; vo+=PAGE_SIZE; } flush_cache_vmap(vmalloc_shadow(start),vmalloc_shadow(end)); flush_cache_vmap(vmalloc_origin(start),vmalloc_origin(end)); kmsan_leave_runtime(); }

#[no_mangle] pub unsafe extern "C" fn kmsan_copy_to_user(to:*mut c_void, from:*const c_void, to_copy:usize, left:usize) { if !kmsan_enabled||kmsan_in_runtime()||to_copy==0||to_copy<=left{return;} let f=user_access_save(); kmsan_internal_check_memory(from as *mut c_void,to_copy-left,to,REASON_COPY_TO_USER); user_access_restore(f); }
#[no_mangle] pub unsafe extern "C" fn kmsan_memmove(to:*mut c_void,from:*const c_void,size:usize){if !kmsan_enabled||kmsan_in_runtime(){return;} kmsan_enter_runtime();kmsan_internal_memmove_metadata(to,from as *mut c_void,size);kmsan_leave_runtime();}
#[no_mangle] pub unsafe extern "C" fn kmsan_handle_urb(urb:*const urb,is_out:bool){if urb.is_null(){return;}if is_out{kmsan_internal_check_memory((*urb).transfer_buffer,(*urb).transfer_buffer_length,core::ptr::null_mut(),REASON_SUBMIT_URB)}else{kmsan_internal_unpoison_memory((*urb).transfer_buffer,(*urb).transfer_buffer_length,false)}}
unsafe fn kmsan_handle_dma_page(addr:*const c_void,size:usize,dir:dma_data_direction){match dir{DMA_BIDIRECTIONAL=>{kmsan_internal_check_memory(addr as *mut c_void,size,core::ptr::null_mut(),REASON_ANY);kmsan_internal_unpoison_memory(addr as *mut c_void,size,false)},DMA_TO_DEVICE=>kmsan_internal_check_memory(addr as *mut c_void,size,core::ptr::null_mut(),REASON_ANY),DMA_FROM_DEVICE=>kmsan_internal_unpoison_memory(addr as *mut c_void,size,false),DMA_NONE=>{},_=>{}}}
#[no_mangle] pub unsafe extern "C" fn kmsan_handle_dma(phys:phys_addr_t,mut size:usize,dir:dma_data_direction){let mut addr=phys_to_virt(phys);while size>0{let off=offset_in_page(addr);let n=core::cmp::min(PAGE_SIZE as u64-off,size as u64) as usize;kmsan_handle_dma_page(addr,n,dir);addr=addr.add(n);size-=n;}}
#[no_mangle] pub unsafe extern "C" fn kmsan_handle_dma_sg(sg:*mut scatterlist,nents:i32,dir:dma_data_direction){for i in 0..nents{kmsan_handle_dma(sg_phys(sg.add(i as usize)),(*sg.add(i as usize)).length,dir);}}

#[no_mangle] pub unsafe extern "C" fn kmsan_poison_memory(address:*const c_void,size:usize,flags:gfp_t){if !kmsan_enabled||kmsan_in_runtime(){return;}kmsan_enter_runtime();kmsan_internal_poison_memory(address as *mut c_void,size,flags,KMSAN_POISON_NOCHECK);kmsan_leave_runtime();}
#[no_mangle] pub unsafe extern "C" fn kmsan_unpoison_memory(address:*const c_void,size:usize){if !kmsan_enabled{return;}let f=user_access_save();kmsan_internal_unpoison_memory(address as *mut c_void,size,KMSAN_POISON_NOCHECK!=0);user_access_restore(f);}
#[no_mangle] pub unsafe extern "C" fn kmsan_unpoison_entry_regs(regs:*const pt_regs){kmsan_unpoison_memory(regs as *const c_void,core::mem::size_of::<pt_regs>());}
#[no_mangle] pub unsafe extern "C" fn kmsan_check_memory(addr:*const c_void,size:usize){if kmsan_enabled{kmsan_internal_check_memory(addr as *mut c_void,size,core::ptr::null_mut(),REASON_ANY);}}
#[no_mangle] pub unsafe extern "C" fn kmsan_enable_current(){(*current).kmsan_ctx.depth-=1;}
#[no_mangle] pub unsafe extern "C" fn kmsan_disable_current(){(*current).kmsan_ctx.depth+=1;}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
