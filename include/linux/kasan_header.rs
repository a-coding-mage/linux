/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/kasan.h. Configuration branches are retained as comments. */

use core::ffi::c_void;

#[repr(transparent)]
pub struct kasan_vmalloc_flags_t(pub u32);
pub const KASAN_VMALLOC_NONE: kasan_vmalloc_flags_t = kasan_vmalloc_flags_t(0x00);
pub const KASAN_VMALLOC_INIT: kasan_vmalloc_flags_t = kasan_vmalloc_flags_t(0x01);
pub const KASAN_VMALLOC_VM_ALLOC: kasan_vmalloc_flags_t = kasan_vmalloc_flags_t(0x02);
pub const KASAN_VMALLOC_PROT_NORMAL: kasan_vmalloc_flags_t = kasan_vmalloc_flags_t(0x04);
pub const KASAN_VMALLOC_KEEP_TAG: kasan_vmalloc_flags_t = kasan_vmalloc_flags_t(0x08);
pub const KASAN_VMALLOC_PAGE_RANGE: u32 = 0x1;
pub const KASAN_VMALLOC_TLB_FLUSH: u32 = 0x2;

pub enum kmem_cache {}
pub enum page {}
pub enum slab {}
pub enum vm_struct {}
pub enum task_struct {}

extern "C" {
    pub fn kasan_enabled() -> bool;
    pub fn kasan_hw_tags_enabled() -> bool;
    pub fn _RET_IP() -> usize;
    pub fn arch_kasan_reset_tag(addr: *const c_void) -> *mut c_void;
}

#[inline]
pub unsafe fn kasan_has_integrated_init() -> bool { kasan_hw_tags_enabled() }

/* CONFIG_KASAN_GENERIC || CONFIG_KASAN_SW_TAGS */
extern "C" {
    pub static mut kasan_early_shadow_page: [u8; 4096];
    pub fn kasan_populate_early_shadow(shadow_start: *const c_void, shadow_end: *const c_void) -> i32;
    pub fn kasan_add_zero_shadow(start: *mut c_void, size: usize) -> i32;
    pub fn kasan_remove_zero_shadow(start: *mut c_void, size: usize);
    pub fn kasan_enable_current();
    pub fn kasan_disable_current();
}

#[inline]
pub unsafe fn kasan_mem_to_shadow(addr: *const c_void) -> *mut c_void {
    ((addr as usize) >> 3) as *mut u8 as *mut c_void
}

extern "C" {
    pub fn __kasan_unpoison_range(addr: *const c_void, size: usize);
    pub fn __kasan_poison_pages(page: *mut page, order: u32, init: bool);
    pub fn __kasan_unpoison_pages(page: *mut page, order: u32, init: bool) -> bool;
    pub fn __kasan_poison_slab(slab: *mut slab);
    pub fn __kasan_unpoison_new_object(cache: *mut kmem_cache, object: *mut c_void);
    pub fn __kasan_poison_new_object(cache: *mut kmem_cache, object: *mut c_void);
    pub fn __kasan_init_slab_obj(cache: *mut kmem_cache, object: *const c_void) -> *mut c_void;
    pub fn __kasan_slab_pre_free(s: *mut kmem_cache, object: *mut c_void, ip: usize) -> bool;
    pub fn __kasan_slab_free(s: *mut kmem_cache, object: *mut c_void, init: bool, still_accessible: bool, no_quarantine: bool) -> bool;
    pub fn __kasan_kfree_large(ptr: *mut c_void, ip: usize);
    pub fn __kasan_slab_alloc(s: *mut kmem_cache, object: *mut c_void, flags: usize, init: bool) -> *mut c_void;
    pub fn __kasan_kmalloc(s: *mut kmem_cache, object: *const c_void, size: usize, flags: usize) -> *mut c_void;
    pub fn __kasan_kmalloc_large(ptr: *const c_void, size: usize, flags: usize) -> *mut c_void;
    pub fn __kasan_krealloc(object: *const c_void, new_size: usize, flags: usize) -> *mut c_void;
    pub fn __kasan_mempool_poison_pages(p: *mut page, order: u32, ip: usize) -> bool;
    pub fn __kasan_mempool_unpoison_pages(p: *mut page, order: u32, ip: usize);
    pub fn __kasan_mempool_poison_object(ptr: *mut c_void, ip: usize) -> bool;
    pub fn __kasan_mempool_unpoison_object(ptr: *mut c_void, size: usize, ip: usize);
    pub fn __kasan_check_byte(addr: *const c_void, ip: usize) -> bool;
}

#[inline] pub unsafe fn kasan_unpoison_range(a:*const c_void,s:usize){if kasan_enabled(){__kasan_unpoison_range(a,s)}}
#[inline] pub unsafe fn kasan_poison_pages(p:*mut page,o:u32,i:bool){if kasan_enabled(){__kasan_poison_pages(p,o,i)}}
#[inline] pub unsafe fn kasan_unpoison_pages(p:*mut page,o:u32,i:bool)->bool{if kasan_enabled(){__kasan_unpoison_pages(p,o,i)}else{false}}
#[inline] pub unsafe fn kasan_poison_slab(s:*mut slab){if kasan_enabled(){__kasan_poison_slab(s)}}
#[inline] pub unsafe fn kasan_unpoison_new_object(c:*mut kmem_cache,o:*mut c_void){if kasan_enabled(){__kasan_unpoison_new_object(c,o)}}
#[inline] pub unsafe fn kasan_poison_new_object(c:*mut kmem_cache,o:*mut c_void){if kasan_enabled(){__kasan_poison_new_object(c,o)}}
#[inline] pub unsafe fn kasan_init_slab_obj(c:*mut kmem_cache,o:*const c_void)->*mut c_void{if kasan_enabled(){__kasan_init_slab_obj(c,o)}else{o as *mut c_void}}
#[inline] pub unsafe fn kasan_slab_pre_free(s:*mut kmem_cache,o:*mut c_void)->bool{if kasan_enabled(){__kasan_slab_pre_free(s,o,_RET_IP())}else{false}}
#[inline] pub unsafe fn kasan_slab_free(s:*mut kmem_cache,o:*mut c_void,i:bool,a:bool,n:bool)->bool{if kasan_enabled(){__kasan_slab_free(s,o,i,a,n)}else{false}}
#[inline] pub unsafe fn kasan_kfree_large(p:*mut c_void){if kasan_enabled(){__kasan_kfree_large(p,_RET_IP())}}
#[inline] pub unsafe fn kasan_slab_alloc(s:*mut kmem_cache,o:*mut c_void,f:usize,i:bool)->*mut c_void{if kasan_enabled(){__kasan_slab_alloc(s,o,f,i)}else{o}}
#[inline] pub unsafe fn kasan_kmalloc(s:*mut kmem_cache,o:*const c_void,z:usize,f:usize)->*mut c_void{if kasan_enabled(){__kasan_kmalloc(s,o,z,f)}else{o as *mut c_void}}
#[inline] pub unsafe fn kasan_kmalloc_large(p:*const c_void,z:usize,f:usize)->*mut c_void{if kasan_enabled(){__kasan_kmalloc_large(p,z,f)}else{p as *mut c_void}}
#[inline] pub unsafe fn kasan_krealloc(o:*const c_void,z:usize,f:usize)->*mut c_void{if kasan_enabled(){__kasan_krealloc(o,z,f)}else{o as *mut c_void}}
#[inline] pub unsafe fn kasan_mempool_poison_pages(p:*mut page,o:u32)->bool{if kasan_enabled(){__kasan_mempool_poison_pages(p,o,_RET_IP())}else{true}}
#[inline] pub unsafe fn kasan_mempool_unpoison_pages(p:*mut page,o:u32){if kasan_enabled(){__kasan_mempool_unpoison_pages(p,o,_RET_IP())}}
#[inline] pub unsafe fn kasan_mempool_poison_object(p:*mut c_void)->bool{if kasan_enabled(){__kasan_mempool_poison_object(p,_RET_IP())}else{true}}
#[inline] pub unsafe fn kasan_mempool_unpoison_object(p:*mut c_void,z:usize){if kasan_enabled(){__kasan_mempool_unpoison_object(p,z,_RET_IP())}}
#[inline] pub unsafe fn kasan_check_byte(a:*const c_void)->bool{if kasan_enabled(){__kasan_check_byte(a,_RET_IP())}else{true}}

extern "C" {
    pub fn kasan_unpoison_task_stack(task: *mut task_struct);
    pub fn kasan_unpoison_task_stack_below(watermark: *const c_void);
    pub fn kasan_reset_tag(addr: *const c_void) -> *mut c_void;
    pub fn kasan_report(addr: *const c_void, size: usize, is_write: bool, ip: usize) -> bool;
    pub fn kasan_init_generic();
    pub fn kasan_init_sw_tags();
    pub fn kasan_init_hw_tags_cpu();
    pub fn kasan_init_hw_tags();
    pub fn kasan_report_async();
    pub fn kasan_populate_early_vm_area_shadow(start: *mut c_void, size: usize);
    pub fn kasan_populate_vmalloc(start: usize, size: usize, gfp_mask: usize) -> i32;
    pub fn kasan_release_vmalloc(start: usize, end: usize, free_region_start: usize, free_region_end: usize, flags: usize);
    pub fn kasan_unpoison_vmalloc(start: *const c_void, size: usize, flags: kasan_vmalloc_flags_t) -> *mut c_void;
    pub fn kasan_poison_vmalloc(start: *const c_void, size: usize);
    pub fn kasan_unpoison_vmap_areas(vms: *mut *mut vm_struct, nr_vms: i32, flags: kasan_vmalloc_flags_t);
    pub fn kasan_vrealloc(start: *const c_void, old_size: usize, new_size: usize);
    pub fn kasan_alloc_module_shadow(addr: *mut c_void, size: usize, gfp_mask: usize) -> i32;
    pub fn kasan_free_module_shadow(vm: *const vm_struct);
    pub fn kasan_non_canonical_hook(addr: usize);
}

#[repr(C)] pub struct kasan_cache { pub alloc_meta_offset: i32, pub free_meta_offset: i32 }
extern "C" { pub fn kasan_metadata_size(cache:*mut kmem_cache,in_object:bool)->usize; pub fn kasan_cache_create(cache:*mut kmem_cache,size:*mut u32,flags:*mut usize); pub fn kasan_cache_shrink(cache:*mut kmem_cache); pub fn kasan_cache_shutdown(cache:*mut kmem_cache); pub fn kasan_record_aux_stack(ptr:*mut c_void); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
