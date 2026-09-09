/* SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause) */
/*
 * Copyright 2015-2016 Freescale Semiconductor, Inc.
 * Copyright 2017 NXP
 */

// Translated from sg_sw_qm2.h.  The declarations below are supplied by the
// surrounding DPAA2/scatterlist implementation.

#[allow(non_camel_case_types)]
pub type dma_addr_t = u64;

#[allow(non_camel_case_types)]
pub type u32 = ::core::primitive::u32;

#[allow(non_camel_case_types)]
pub type u16 = ::core::primitive::u16;

#[repr(C)]
pub struct dpaa2_sg_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct scatterlist {
    _private: [u8; 0],
}

#[allow(non_camel_case_types)]
pub const dpaa2_sg_single: u32 = 0;

extern "C" {
    pub fn dpaa2_sg_set_addr(qm_sg_ptr: *mut dpaa2_sg_entry, dma: dma_addr_t);
    pub fn dpaa2_sg_set_format(qm_sg_ptr: *mut dpaa2_sg_entry, format: u32);
    pub fn dpaa2_sg_set_final(qm_sg_ptr: *mut dpaa2_sg_entry, final_: bool);
    pub fn dpaa2_sg_set_len(qm_sg_ptr: *mut dpaa2_sg_entry, len: u32);
    pub fn dpaa2_sg_set_bpid(qm_sg_ptr: *mut dpaa2_sg_entry, bpid: u16);
    pub fn dpaa2_sg_set_offset(qm_sg_ptr: *mut dpaa2_sg_entry, offset: u16);
    pub fn sg_dma_len(sg: *mut scatterlist) -> i32;
    pub fn sg_dma_address(sg: *mut scatterlist) -> dma_addr_t;
    pub fn sg_next(sg: *mut scatterlist) -> *mut scatterlist;
}

#[inline]
pub unsafe fn dma_to_qm_sg_one(
    qm_sg_ptr: *mut dpaa2_sg_entry,
    dma: dma_addr_t,
    len: u32,
    offset: u16,
) {
    dpaa2_sg_set_addr(qm_sg_ptr, dma);
    dpaa2_sg_set_format(qm_sg_ptr, dpaa2_sg_single);
    dpaa2_sg_set_final(qm_sg_ptr, false);
    dpaa2_sg_set_len(qm_sg_ptr, len);
    dpaa2_sg_set_bpid(qm_sg_ptr, 0);
    dpaa2_sg_set_offset(qm_sg_ptr, offset);
}

/*
 * convert scatterlist to h/w link table format
 * but does not have final bit; instead, returns last entry
 */
#[inline]
pub unsafe fn sg_to_qm_sg(
    mut sg: *mut scatterlist,
    mut len: i32,
    mut qm_sg_ptr: *mut dpaa2_sg_entry,
    offset: u16,
) -> *mut dpaa2_sg_entry {
    let mut ent_len: i32;

    while len != 0 {
        ent_len = ::core::cmp::min(sg_dma_len(sg), len);

        dma_to_qm_sg_one(qm_sg_ptr, sg_dma_address(sg), ent_len as u32, offset);
        qm_sg_ptr = qm_sg_ptr.add(1);
        sg = sg_next(sg);
        len -= ent_len;
    }
    qm_sg_ptr.sub(1)
}

/*
 * convert scatterlist to h/w link table format
 * scatterlist must have been previously dma mapped
 */
#[inline]
pub unsafe fn sg_to_qm_sg_last(
    sg: *mut scatterlist,
    len: i32,
    mut qm_sg_ptr: *mut dpaa2_sg_entry,
    offset: u16,
) {
    qm_sg_ptr = sg_to_qm_sg(sg, len, qm_sg_ptr, offset);
    dpaa2_sg_set_final(qm_sg_ptr, true);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
