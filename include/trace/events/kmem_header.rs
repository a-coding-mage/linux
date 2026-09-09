/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of trace/events/kmem.h.  The tracepoint registration
// macros and their dependent kernel types are supplied by other components.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_void;

extern "C" {
    fn page_to_pfn(page: *mut page) -> usize;
    fn pfn_to_page(pfn: usize) -> *mut page;
    fn get_pageblock_migratetype(page: *mut page) -> i32;
    fn ptr_to_hashval(ptr: *const c_void, hashval: *mut usize) -> i32;
    fn show_gfp_flags(flags: usize) -> *const u8;
}

#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct kmem_cache {
    pub name: *const u8,
    pub object_size: usize,
    pub size: usize,
    pub flags: usize,
}
#[repr(C)] pub struct zone { _private: [u8; 0] }
#[repr(C)] pub struct mm_struct { _private: [u8; 0] }

pub type gfp_t = usize;

pub const __GFP_ACCOUNT: usize = 0;
pub const SLAB_ACCOUNT: usize = 0;
pub const PAGE_SHIFT: usize = 0;
pub const PF_KTHREAD: usize = 0;
pub const WMARK_MIN: usize = 0;
pub const WMARK_LOW: usize = 1;
pub const WMARK_HIGH: usize = 2;
pub const WMARK_PROMO: usize = 3;

#[inline]
pub unsafe fn mm_ptr_to_hash(ptr: *const c_void) -> u32 {
    let mut hashval = 0usize;
    if ptr_to_hashval(ptr, &mut hashval) != 0 { return 0; }
    hashval as u32
}

// TRACE_EVENT(kmem_cache_alloc):
#[repr(C)] pub struct kmem_cache_alloc_entry {
    pub call_site: usize, pub ptr: *const c_void,
    pub name: *const u8, pub bytes_req: usize, pub bytes_alloc: usize,
    pub gfp_flags: usize, pub node: i32, pub accounted: bool,
}
#[inline] pub unsafe fn kmem_cache_alloc_assign(e: &mut kmem_cache_alloc_entry, call_site: usize, ptr: *const c_void, s: *mut kmem_cache, gfp_flags: gfp_t, node: i32) {
    e.call_site=call_site; e.ptr=ptr; e.name=(*s).name; e.bytes_req=(*s).object_size;
    e.bytes_alloc=(*s).size; e.gfp_flags=gfp_flags; e.node=node;
    e.accounted=(gfp_flags & __GFP_ACCOUNT)!=0 || ((*s).flags & SLAB_ACCOUNT)!=0;
}

// TRACE_EVENT(kmalloc)
#[repr(C)] pub struct kmalloc_entry { pub call_site: usize, pub ptr: *const c_void, pub bytes_req: usize, pub bytes_alloc: usize, pub gfp_flags: usize, pub node: i32 }
// TRACE_EVENT(kfree)
#[repr(C)] pub struct kfree_entry { pub call_site: usize, pub ptr: *const c_void }
// TRACE_EVENT(kmem_cache_free)
#[repr(C)] pub struct kmem_cache_free_entry { pub call_site: usize, pub ptr: *const c_void, pub name: *const u8 }
// TRACE_EVENT(mm_page_free), TRACE_EVENT(mm_page_free_batched)
#[repr(C)] pub struct mm_page_entry { pub pfn: usize, pub order: u32 }
// TRACE_EVENT(mm_page_alloc)
#[repr(C)] pub struct mm_page_alloc_entry { pub pfn: usize, pub order: u32, pub gfp_flags: usize, pub migratetype: i32 }
// DECLARE_EVENT_CLASS(mm_page), DEFINE_EVENT(mm_page, mm_page_alloc_zone_locked)
#[repr(C)] pub struct mm_page_class_entry { pub pfn: usize, pub order: u32, pub migratetype: i32, pub percpu_refill: i32 }
// TRACE_EVENT(mm_page_pcpu_drain)
#[repr(C)] pub struct mm_page_pcpu_drain_entry { pub pfn: usize, pub order: u32, pub migratetype: i32 }
// TRACE_EVENT(mm_page_alloc_extfrag)
#[repr(C)] pub struct mm_page_alloc_extfrag_entry { pub pfn: usize, pub alloc_order: i32, pub fallback_order: i32, pub alloc_migratetype: i32, pub fallback_migratetype: i32, pub change_ownership: i32 }
// TRACE_EVENT(mm_setup_per_zone_wmarks), mm_setup_per_zone_lowmem_reserve
#[repr(C)] pub struct mm_setup_zone_entry { pub node_id: i32, pub name: *const u8, pub upper_name: *const u8, pub watermark_min: usize, pub watermark_low: usize, pub watermark_high: usize, pub watermark_promo: usize, pub lowmem_reserve: isize }
// TRACE_EVENT(mm_calculate_totalreserve_pages)
#[repr(C)] pub struct mm_calculate_totalreserve_pages_entry { pub totalreserve_pages: usize }
// TRACE_EVENT(rss_stat)
#[repr(C)] pub struct rss_stat_entry { pub mm_id: u32, pub curr: u32, pub member: i32, pub size: isize }

pub const TRACE_MM_PAGES: &[i32] = &[]; // MM_FILEPAGES, MM_ANONPAGES, MM_SWAPENTS, MM_SHMEMPAGES

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
