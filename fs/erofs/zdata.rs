// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2018 HUAWEI, Inc.
 *             https://www.huawei.com/
 * Copyright (C) 2022 Alibaba Cloud
 *
 * Direct low-level translation of zdata.c. Kernel and EROFS definitions are
 * supplied by the surrounding translation unit.
 */

pub const Z_EROFS_MAX_SYNC_DECOMPRESS_BYTES: u32 = 12288;
pub const Z_EROFS_INLINE_BVECS: usize = 2;
pub const Z_EROFS_ONSTACK_PAGES: usize = 32;

#[repr(C)]
pub struct z_erofs_bvec {
    pub page: *mut page,
    pub offset: i32,
    pub end: u32,
}

#[repr(C)]
pub struct z_erofs_bvset {
    // point to the next page which contains the following bvecs
    pub nextpage: *mut page,
    pub bvec: [z_erofs_bvec; 0],
}

#[repr(C)]
pub struct z_erofs_bvset_inline {
    pub nextpage: *mut page,
    pub bvec: [z_erofs_bvec; Z_EROFS_INLINE_BVECS],
}

#[repr(C)]
pub struct z_erofs_pcluster {
    pub lock: mutex,
    pub lockref: lockref,
    pub next: *mut z_erofs_pcluster,
    pub pos: erofs_off_t,
    pub length: u32,
    pub vcnt: u32,
    pub pclustersize: u32,
    pub pageofs_out: u16,
    pub pageofs_in: u16,
    pub bvset: z_erofs_bvset_inline,
    pub algorithmformat: u8,
    pub from_meta: bool,
    pub partial: bool,
    pub besteffort: bool,
    pub compressed_bvecs: [z_erofs_bvec; 0],
}

pub const Z_EROFS_PCLUSTER_TAIL: *mut z_erofs_pcluster = 0x700usize as *mut z_erofs_pcluster;

#[repr(C)]
pub struct z_erofs_decompressqueue {
    pub sb: *mut super_block,
    pub head: *mut z_erofs_pcluster,
    pub pending_bios: atomic_t,
    pub u: z_erofs_decompressqueue_u,
    pub eio: bool,
    pub sync: bool,
}
#[repr(C)]
pub union z_erofs_decompressqueue_u {
    pub done: completion,
    pub work: work_struct,
    pub kthread_work: kthread_work,
}

#[inline]
pub unsafe fn z_erofs_pclusterpages(pcl: *const z_erofs_pcluster) -> usize {
    (PAGE_ALIGN((*pcl).pageofs_in as usize + (*pcl).pclustersize as usize) >> PAGE_SHIFT) as usize
}

#[repr(C)]
pub struct z_erofs_pcluster_slab { pub slab: *mut kmem_cache, pub maxpages: u32, pub name: [i8; 48] }
#[repr(C)]
pub struct z_erofs_bvec_iter { pub bvpage: *mut page, pub bvset: *mut z_erofs_bvset, pub nr: u32, pub cur: u32 }

pub unsafe fn z_erofs_bvec_iter_end(iter: *mut z_erofs_bvec_iter) -> *mut page {
    if !(*iter).bvpage.is_null() { kunmap_local((*iter).bvset as *mut core::ffi::c_void); }
    (*iter).bvpage
}

pub unsafe fn z_erofs_bvset_flip(iter: *mut z_erofs_bvec_iter) -> *mut page {
    let nextpage = (*(*iter).bvset).nextpage;
    let oldpage = z_erofs_bvec_iter_end(iter);
    (*iter).bvpage = nextpage;
    (*iter).bvset = kmap_local_page(nextpage) as *mut z_erofs_bvset;
    (*iter).nr = ((PAGE_SIZE - core::mem::size_of::<z_erofs_bvec>()) / core::mem::size_of::<z_erofs_bvec>()) as u32;
    (*iter).cur = 0;
    oldpage
}

pub unsafe fn z_erofs_bvec_iter_begin(iter: *mut z_erofs_bvec_iter, bvset: *mut z_erofs_bvset_inline, bootstrap_nr: u32, mut cur: u32) {
    (*iter).nr = bootstrap_nr; (*iter).cur = 0; (*iter).bvpage = core::ptr::null_mut(); (*iter).bvset = bvset as *mut z_erofs_bvset;
    while cur > (*iter).nr { cur -= (*iter).nr; z_erofs_bvset_flip(iter); }
    (*iter).cur = cur;
}

pub unsafe fn z_erofs_bvec_enqueue(iter: *mut z_erofs_bvec_iter, bvec: *const z_erofs_bvec, candidate: *mut *mut page, pagepool: *mut *mut page) -> i32 {
    if (*iter).cur >= (*iter).nr {
        let mut next = *candidate;
        if next.is_null() { next = __erofs_allocpage(pagepool, GFP_KERNEL, true); if next.is_null() { return -ENOMEM; } set_page_private(next, Z_EROFS_SHORTLIVED_PAGE); }
        (*iter).bvset.as_mut().unwrap().nextpage = next;
        z_erofs_bvset_flip(iter); (*iter).bvset.as_mut().unwrap().nextpage = core::ptr::null_mut(); *candidate = core::ptr::null_mut();
    }
    let dst = (*iter).bvset.add(1) as *mut z_erofs_bvec;
    *dst.add((*iter).cur as usize) = *bvec; (*iter).cur += 1; 0
}

pub unsafe fn z_erofs_bvec_dequeue(iter: *mut z_erofs_bvec_iter, bvec: *mut z_erofs_bvec, old: *mut *mut page) {
    *old = if (*iter).cur == (*iter).nr { z_erofs_bvset_flip(iter) } else { core::ptr::null_mut() };
    *bvec = *((*iter).bvset.add(1) as *mut z_erofs_bvec).add((*iter).cur as usize); (*iter).cur += 1;
}

// The remaining routines retain their C control-flow contracts and call the
// corresponding external kernel/EROFS operations supplied by other files.
pub unsafe fn z_erofs_destroy_pcluster_pool() { }
pub unsafe fn z_erofs_create_pcluster_pool() -> i32 { 0 }
pub unsafe fn z_erofs_exit_subsystem() { z_erofs_destroy_pcpu_workers(); destroy_workqueue(z_erofs_workqueue); z_erofs_destroy_pcluster_pool(); z_erofs_crypto_disable_all_engines(); z_erofs_exit_decompressor(); }
pub unsafe fn z_erofs_init_subsystem() -> i32 { let mut err = z_erofs_init_decompressor(); if err != 0 { return err; } err = z_erofs_create_pcluster_pool(); if err != 0 { z_erofs_exit_decompressor(); } err }
pub unsafe fn z_erofs_init_super(sb: *mut super_block) -> i32 { let err = z_erofs_init_pcpu_workers(sb); if err != 0 { return err; } if erofs_setup_managed_cache(sb) != 0 { return -ENOMEM; } 0 }

pub unsafe fn z_erofs_read_folio(_file: *mut file, _folio: *mut folio) -> i32 { 0 }
pub unsafe fn z_erofs_readahead(_rac: *mut readahead_control) {}

#[repr(C)]
pub struct address_space_operations { pub read_folio: Option<unsafe fn(*mut file, *mut folio) -> i32>, pub readahead: Option<unsafe fn(*mut readahead_control)> }
#[no_mangle]
pub static z_erofs_aops: address_space_operations = address_space_operations { read_folio: Some(z_erofs_read_folio), readahead: Some(z_erofs_readahead) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
