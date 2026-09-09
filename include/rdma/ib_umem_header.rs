/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/*
 * Copyright (c) 2007 Cisco Systems.  All rights reserved.
 * Copyright (c) 2020 Intel Corporation.  All rights reserved.
 */

// Translated from ib_umem.h. External kernel types, constants, and helpers
// are intentionally left as dependencies supplied by the surrounding crate.

use core::ffi::c_void;

#[repr(C)]
pub struct ib_device { _private: [u8; 0] }
#[repr(C)]
pub struct dma_buf_attach_ops { _private: [u8; 0] }
#[repr(C)]
pub struct mm_struct { _private: [u8; 0] }
#[repr(C)]
pub struct dma_buf_attachment { _private: [u8; 0] }
#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct scatterlist { _private: [u8; 0] }
#[repr(C)]
pub struct sg_table { _private: [u8; 0] }
#[repr(C)]
pub struct sg_append_table { pub sgt: sg_table_inner }
#[repr(C)]
pub struct sg_table_inner { pub sgl: *mut scatterlist }
#[repr(C)]
pub struct ib_udata { _private: [u8; 0] }
#[repr(C)]
pub struct ib_uverbs_buffer_desc { _private: [u8; 0] }
#[repr(C)]
pub struct uverbs_attr_bundle { _private: [u8; 0] }

pub type u64_ = u64;
pub type dma_addr_t = u64;

#[repr(C)]
pub struct ib_umem {
    pub ibdev: *mut ib_device,
    pub owning_mm: *mut mm_struct,
    pub iova: u64,
    pub length: usize,
    pub address: usize,
    pub dma_attrs: usize,
    // C bitfields: writable:1, is_odp:1, is_dmabuf:1.
    pub writable: u32,
    pub is_odp: u32,
    pub is_dmabuf: u32,
    pub sgt_append: sg_append_table,
}

#[repr(C)]
pub struct ib_umem_dmabuf {
    pub umem: ib_umem,
    pub attach: *mut dma_buf_attachment,
    pub sgt: *mut sg_table,
    pub first_sg: *mut scatterlist,
    pub last_sg: *mut scatterlist,
    pub first_sg_offset: usize,
    pub last_sg_trim: usize,
    pub pinned_revoke: Option<unsafe extern "C" fn(*mut c_void)>,
    pub private: *mut c_void,
    // C bitfields: pinned:1, revoked:1.
    pub pinned: u8,
    pub revoked: u8,
}

#[inline]
pub unsafe fn to_ib_umem_dmabuf(umem: *mut ib_umem) -> *mut ib_umem_dmabuf {
    umem as *mut ib_umem_dmabuf
}

#[inline]
pub unsafe fn ib_umem_offset(umem: *mut ib_umem) -> i32 {
    ((*umem).address & !PAGE_MASK) as i32
}

#[inline]
pub unsafe fn ib_umem_start_dma_addr(umem: *mut ib_umem) -> dma_addr_t {
    sg_dma_address((*umem).sgt_append.sgt.sgl) + ib_umem_offset(umem) as dma_addr_t
}

#[inline]
pub unsafe fn ib_umem_dma_offset(umem: *mut ib_umem, pgsz: usize) -> usize {
    (ib_umem_start_dma_addr(umem) as usize) & (pgsz - 1)
}

#[inline]
pub unsafe fn ib_umem_num_dma_blocks(umem: *mut ib_umem, pgsz: usize) -> usize {
    (align((*umem).iova + (*umem).length, pgsz) - align_down((*umem).iova, pgsz)) / pgsz
}

#[inline]
pub unsafe fn ib_umem_num_pages(umem: *mut ib_umem) -> usize {
    ib_umem_num_dma_blocks(umem, PAGE_SIZE)
}

#[cfg(feature = "CONFIG_INFINIBAND_USER_MEM")]
extern "C" {
    pub fn ib_umem_get_desc(device: *mut ib_device, desc: *const ib_uverbs_buffer_desc, access: i32) -> *mut ib_umem;
    pub fn ib_umem_get_attr(device: *mut ib_device, attrs: *const uverbs_attr_bundle, attr_id: u16, size: usize, access: i32) -> *mut ib_umem;
    pub fn ib_umem_get_attr_or_va(device: *mut ib_device, attrs: *const uverbs_attr_bundle, attr_id: u16, addr: u64, size: usize, access: i32) -> *mut ib_umem;
    pub fn ib_umem_get_cq_buf(device: *mut ib_device, attrs: *const uverbs_attr_bundle, size: usize, access: i32) -> *mut ib_umem;
    pub fn ib_umem_get_cq_buf_or_va(device: *mut ib_device, attrs: *const uverbs_attr_bundle, addr: u64, size: usize, access: i32) -> *mut ib_umem;
    pub fn ib_umem_release(umem: *mut ib_umem);
    pub fn ib_umem_copy_from(dst: *mut c_void, umem: *mut ib_umem, offset: usize, length: usize) -> i32;
    pub fn ib_umem_find_best_pgsz(umem: *mut ib_umem, pgsz_bitmap: usize, virt: u64) -> usize;
    pub fn ib_umem_dmabuf_get(device: *mut ib_device, offset: usize, size: usize, fd: i32, access: i32, ops: *const dma_buf_attach_ops) -> *mut ib_umem_dmabuf;
    pub fn ib_umem_dmabuf_get_pinned(device: *mut ib_device, offset: usize, size: usize, fd: i32, access: i32) -> *mut ib_umem_dmabuf;
    pub fn ib_umem_dmabuf_get_pinned_revocable_and_lock(device: *mut ib_device, offset: usize, size: usize, fd: i32, access: i32) -> *mut ib_umem_dmabuf;
    pub fn ib_umem_dmabuf_set_revoke_locked(umem_dmabuf: *mut ib_umem_dmabuf, revoke: Option<unsafe extern "C" fn(*mut c_void)>, priv_: *mut c_void);
    pub fn ib_umem_dmabuf_get_pinned_with_dma_device(device: *mut ib_device, dma_device: *mut device, offset: usize, size: usize, fd: i32, access: i32) -> *mut ib_umem_dmabuf;
    pub fn ib_umem_dmabuf_map_pages(umem_dmabuf: *mut ib_umem_dmabuf) -> i32;
    pub fn ib_umem_dmabuf_unmap_pages(umem_dmabuf: *mut ib_umem_dmabuf);
    pub fn ib_umem_dmabuf_release(umem_dmabuf: *mut ib_umem_dmabuf);
    pub fn ib_umem_dmabuf_revoke_lock(umem_dmabuf: *mut ib_umem_dmabuf);
    pub fn ib_umem_dmabuf_revoke_unlock(umem_dmabuf: *mut ib_umem_dmabuf);
    pub fn ib_umem_dmabuf_revoke(umem_dmabuf: *mut ib_umem_dmabuf);
    pub fn ib_umem_check_rereg(umem: *mut ib_umem, flags: i32, new_access_flags: i32) -> i32;
    pub fn sg_dma_address(sg: *mut scatterlist) -> dma_addr_t;
}

#[cfg(feature = "CONFIG_INFINIBAND_USER_MEM")]
#[inline]
pub unsafe fn ib_umem_get_va(device: *mut ib_device, addr: usize, size: usize, access: i32) -> *mut ib_umem {
    ib_umem_get_attr_or_va(device, core::ptr::null(), 0, addr as u64, size, access)
}

#[cfg(feature = "CONFIG_INFINIBAND_USER_MEM")]
#[inline]
pub unsafe fn ib_umem_find_best_pgoff(umem: *mut ib_umem, pgsz_bitmap: usize, pgoff_bitmask: u64) -> usize {
    ib_umem_find_best_pgsz(umem, pgsz_bitmap, ib_umem_start_dma_addr(umem) & pgoff_bitmask)
}

#[cfg(feature = "CONFIG_INFINIBAND_USER_MEM")]
#[inline]
pub unsafe fn ib_umem_is_contiguous(umem: *mut ib_umem) -> bool {
    let pgsz = ib_umem_find_best_pgsz(umem, usize::MAX, ib_umem_start_dma_addr(umem));
    pgsz != 0 && ib_umem_num_dma_blocks(umem, pgsz) == 1
}

#[cfg(not(feature = "CONFIG_INFINIBAND_USER_MEM"))]
mod user_mem_disabled {
    use super::*;
    #[inline] pub unsafe fn ib_umem_get_desc(_: *mut ib_device, _: *const ib_uverbs_buffer_desc, _: i32) -> *mut ib_umem { (-95isize) as *mut ib_umem }
    #[inline] pub unsafe fn ib_umem_get_va(_: *mut ib_device, _: usize, _: usize, _: i32) -> *mut ib_umem { (-95isize) as *mut ib_umem }
    #[inline] pub unsafe fn ib_umem_get_attr(_: *mut ib_device, _: *const uverbs_attr_bundle, _: u16, _: usize, _: i32) -> *mut ib_umem { (-95isize) as *mut ib_umem }
    #[inline] pub unsafe fn ib_umem_get_attr_or_va(_: *mut ib_device, _: *const uverbs_attr_bundle, _: u16, _: u64, _: usize, _: i32) -> *mut ib_umem { (-95isize) as *mut ib_umem }
    #[inline] pub unsafe fn ib_umem_get_cq_buf(_: *mut ib_device, _: *const uverbs_attr_bundle, _: usize, _: i32) -> *mut ib_umem { (-95isize) as *mut ib_umem }
    #[inline] pub unsafe fn ib_umem_get_cq_buf_or_va(_: *mut ib_device, _: *const uverbs_attr_bundle, _: u64, _: usize, _: i32) -> *mut ib_umem { (-95isize) as *mut ib_umem }
    #[inline] pub unsafe fn ib_umem_release(_: *mut ib_umem) {}
    #[inline] pub unsafe fn ib_umem_copy_from(_: *mut c_void, _: *mut ib_umem, _: usize, _: usize) -> i32 { -95 }
    #[inline] pub unsafe fn ib_umem_find_best_pgsz(_: *mut ib_umem, _: usize, _: u64) -> usize { 0 }
    #[inline] pub unsafe fn ib_umem_find_best_pgoff(_: *mut ib_umem, _: usize, _: u64) -> usize { 0 }
    #[inline] pub unsafe fn ib_umem_is_contiguous(_: *mut ib_umem) -> bool { false }
    #[inline] pub unsafe fn ib_umem_check_rereg(_: *mut ib_umem, _: i32, _: i32) -> i32 { -95 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
