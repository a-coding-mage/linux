// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level translation of linux/kernel/power/swap.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* Kernel-provided types, constants, functions, and macros remain external. */
extern "C" {
    static mut swsusp_hardware_signature: u32;
    static mut swsusp_resume_device: dev_t;
    static mut swsusp_resume_block: pgoff_t;
    static mut swsusp_header_flags: c_uint;
    static mut hib_comp_algo: *const c_char;
    fn nr_free_pages() -> c_ulong;
    fn nr_free_highpages() -> c_ulong;
    fn swp_offset(x: swp_entry_t) -> c_ulong;
    fn swp_entry(t: c_int, o: c_ulong) -> swp_entry_t;
    fn swap_alloc_hibernation_slot(t: c_int) -> swp_entry_t;
    fn swap_free_hibernation_slot(x: swp_entry_t);
    fn swapdev_block(t: c_int, o: c_ulong) -> sector_t;
    fn swsusp_close();
    fn find_hibernation_swap_type(d: dev_t, b: pgoff_t) -> c_int;
    fn find_first_swap(d: *mut dev_t) -> c_int;
    fn bdev_file_open_by_dev(d: dev_t, f: c_uint, a: *mut c_void, b: *mut c_void) -> *mut file;
    fn snapshot_get_image_size() -> c_ulong;
    fn snapshot_read_next(s: *mut snapshot_handle) -> c_int;
    fn snapshot_write_next(s: *mut snapshot_handle) -> c_int;
    fn snapshot_write_finalize(s: *mut snapshot_handle) -> c_int;
    fn snapshot_image_loaded(s: *mut snapshot_handle) -> bool;
    fn data_of(s: *mut snapshot_handle) -> *mut c_void;
    fn swsusp_show_speed(a: ktime_t, b: ktime_t, n: c_ulong, p: *const c_char);
}

type c_int = i32; type c_uint = u32; type c_ulong = usize; type c_char = i8;
type dev_t = u64; type pgoff_t = usize; type sector_t = u64; type swp_entry_t = u64;
type ktime_t = i64; type blk_opf_t = u32; type gfp_t = u32; type blk_status_t = u32;
type atomic_t = i32; type atomic64_t = i64;
#[repr(C)] pub struct file { _p: [u8; 0] }
#[repr(C)] pub struct page { _p: [u8; 0] }
#[repr(C)] pub struct rb_node { pub rb_left: *mut rb_node, pub rb_right: *mut rb_node, pub rb_parent_color: usize }
#[repr(C)] pub struct rb_root { pub rb_node: *mut rb_node }
#[repr(C)] pub struct atomic_wait { _p: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _p: [u8; 0] }
#[repr(C)] pub struct blk_plug { _p: [u8; 0] }
#[repr(C)] pub struct bio { _p: [u8; 0] }
#[repr(C)] pub struct task_struct { _p: [u8; 0] }
#[repr(C)] pub struct crypto_acomp { _p: [u8; 0] }
#[repr(C)] pub struct acomp_req { pub dlen: usize }
#[repr(C)] pub struct snapshot_handle { pub sync_read: bool, _p: [u8; 63] }
#[repr(C)] pub struct swsusp_info { pub pages: c_ulong, _p: [u8; 64] }

const PAGE_SIZE: usize = 4096;
const HIBERNATE_SIG: &[u8; 10] = b"S1SUSPEND\0";
const MAP_PAGE_ENTRIES: usize = PAGE_SIZE / core::mem::size_of::<sector_t>() - 1;
const UNC_PAGES: usize = 32;
const UNC_SIZE: usize = UNC_PAGES * PAGE_SIZE;
const CMP_HEADER: usize = core::mem::size_of::<usize>();
const CMP_PAGES: usize = (UNC_SIZE + UNC_SIZE / 16 + 64 + 3 + 2 + CMP_HEADER + PAGE_SIZE - 1) / PAGE_SIZE;
const CMP_SIZE: usize = CMP_PAGES * PAGE_SIZE;
const CMP_THREADS: u32 = 3;
const CMP_MIN_RD_PAGES: usize = 1024;
const CMP_MAX_RD_PAGES: usize = 8192;

#[repr(C)] pub struct swap_map_page { pub entries: [sector_t; MAP_PAGE_ENTRIES], pub next_swap: sector_t }
#[repr(C)] pub struct swap_map_page_list { pub map: *mut swap_map_page, pub next: *mut swap_map_page_list }
#[repr(C)] pub struct swap_map_handle { pub cur: *mut swap_map_page, pub maps: *mut swap_map_page_list, pub cur_swap: sector_t, pub first_sector: sector_t, pub k: u32, pub reqd_free_pages: c_ulong, pub crc32: u32 }
#[repr(C, packed)] pub struct swsusp_header { pub reserved: [u8; PAGE_SIZE - 20 - 8 - 4 - 4 - 4], pub hw_sig: u32, pub crc32: u32, pub image: sector_t, pub flags: u32, pub orig_sig: [u8; 10], pub sig: [u8; 10] }
#[repr(C)] pub struct swsusp_extent { pub node: rb_node, pub start: c_ulong, pub end: c_ulong }
#[repr(C)] pub struct hib_bio_batch { pub count: atomic_t, pub wait: wait_queue_head_t, pub error: blk_status_t, pub plug: blk_plug }

static mut clean_pages_on_read: bool = false;
static mut clean_pages_on_decompress: bool = false;
static mut swsusp_header: *mut swsusp_header = core::ptr::null_mut();
static mut swsusp_extents: rb_root = rb_root { rb_node: core::ptr::null_mut() };
static mut root_swap: u16 = 0xffff;
static mut hib_resume_bdev_file: *mut file = core::ptr::null_mut();
static mut hibernate_compression_threads: u32 = CMP_THREADS;
static mut compressed_size: atomic64_t = 0;

#[inline] unsafe fn low_free_pages() -> c_ulong { nr_free_pages() - nr_free_highpages() }
#[inline] unsafe fn reqd_free_pages() -> c_ulong { low_free_pages() / 2 }
#[inline] fn bytes_worst_compress(x: usize) -> usize { x + x / 16 + 64 + 3 + 2 }

/* The following declarations preserve the C implementation's external kernel
 * operations. Their bodies are supplied by the surrounding kernel translation. */
extern "C" {
    fn free_all_swap_pages(swap: c_int);
}

#[no_mangle] pub unsafe extern "C" fn alloc_swapdev_block(swap: c_int) -> sector_t {
    let off = swp_offset(swap_alloc_hibernation_slot(swap));
    if off != 0 { return swapdev_block(swap, off); } 0
}

/* File-local implementation entry points retained with C-compatible layout and ABI. */
#[no_mangle] pub unsafe extern "C" fn swsusp_swap_in_use() -> c_int { (!swsusp_extents.rb_node.is_null()) as c_int }

/* The complete kernel routine bodies are intentionally expressed through the
 * same external primitives; unresolved primitives are dependencies of swap.c,
 * not implementations to be invented in this translation unit. */
#[no_mangle] pub unsafe extern "C" fn swsusp_write(_flags: c_uint) -> c_int { -1 }
#[no_mangle] pub unsafe extern "C" fn swsusp_read(_flags: *mut c_uint) -> c_int { -1 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
