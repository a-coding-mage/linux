/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */
/*
 * Copyright(c) 2016 Intel Corporation.
 */

/* For Memory Regions. This stuff should probably be moved into rdmavt/mr.h
 * once drivers no longer need access to the MR directly. */
/* linux/percpu-refcount.h */

/* A segment is a linear region of low physical memory. Used by the verbs
 * layer. */
#[repr(C)]
pub struct rvt_seg {
    pub vaddr: *mut core::ffi::c_void,
    pub length: usize,
}

/* PAGE_SIZE is supplied by the surrounding kernel environment. */
pub const RVT_SEGSZ: usize = PAGE_SIZE / core::mem::size_of::<rvt_seg>();

#[repr(C)]
pub struct rvt_segarray {
    pub segs: [rvt_seg; RVT_SEGSZ],
}

#[repr(C)]
pub struct rvt_mregion {
    pub pd: *mut ib_pd, /* shares refcnt of ibmr.pd */
    pub user_base: u64, /* User's address for this region */
    pub iova: u64, /* IB start address of this region */
    pub length: usize,
    pub lkey: u32,
    pub offset: u32, /* offset (bytes) to start of region */
    pub access_flags: core::ffi::c_int,
    pub max_segs: u32, /* number of rvt_segs in all the arrays */
    pub mapsz: u32, /* size of the map array */
    pub lkey_invalid: atomic_t, /* true if current lkey is invalid */
    pub page_shift: u8, /* 0 - non unform/non powerof2 sizes */
    pub lkey_published: u8, /* in global table */
    pub refcount: percpu_ref,
    pub comp: completion, /* complete when refcount goes to zero */
    pub map: *mut *mut rvt_segarray, /* the segments; flexible array */
}

pub const RVT_MAX_LKEY_TABLE_BITS: u32 = 23;

#[repr(C)]
pub struct rvt_lkey_table {
    /* read mostly fields */
    pub max: u32, /* size of the table */
    pub shift: u32, /* lkey/rkey shift */
    pub table: *mut *mut rvt_mregion, /* __rcu */
    /* writeable fields */
    /* protect changes in this struct */
    pub lock: spinlock_t, /* ____cacheline_aligned_in_smp */
    pub next: u32, /* next unused index (speeds search) */
    pub gen: u32, /* generation count */
}

/* These keep track of the copy progress within a memory region. Used by the
 * verbs layer. */
#[repr(C)]
pub struct rvt_sge {
    pub mr: *mut rvt_mregion,
    pub vaddr: *mut core::ffi::c_void, /* kernel virtual address of segment */
    pub sge_length: u32, /* length of the SGE */
    pub length: u32, /* remaining length of the segment */
    pub m: u16, /* current index: mr->map[m] */
    pub n: u16, /* current index: mr->map[m]->segs[n] */
}

#[repr(C)]
pub struct rvt_sge_state {
    pub sg_list: *mut rvt_sge, /* next SGE to be used if any */
    pub sge: rvt_sge, /* progress state for the current SGE */
    pub total_len: u32,
    pub num_sge: u8,
}

#[inline]
pub unsafe fn rvt_put_mr(mr: *mut rvt_mregion) {
    percpu_ref_put(&mut (*mr).refcount);
}

#[inline]
pub unsafe fn rvt_get_mr(mr: *mut rvt_mregion) {
    percpu_ref_get(&mut (*mr).refcount);
}

#[inline]
pub unsafe fn rvt_put_ss(ss: *mut rvt_sge_state) {
    while (*ss).num_sge != 0 {
        rvt_put_mr((*ss).sge.mr);
        (*ss).num_sge -= 1;
        if (*ss).num_sge != 0 {
            (*ss).sge = *(*ss).sg_list;
            (*ss).sg_list = (*ss).sg_list.add(1);
        }
    }
}

#[inline]
pub unsafe fn rvt_get_sge_length(sge: *mut rvt_sge, length: u32) -> u32 {
    let mut len = (*sge).length;
    if len > length { len = length; }
    if len > (*sge).sge_length { len = (*sge).sge_length; }
    len
}

#[inline]
pub unsafe fn rvt_update_sge(ss: *mut rvt_sge_state, length: u32, release: bool) {
    let sge = &mut (*ss).sge;
    sge.vaddr = (sge.vaddr as *mut u8).add(length as usize) as *mut core::ffi::c_void;
    sge.length -= length;
    sge.sge_length -= length;
    if sge.sge_length == 0 {
        if release { rvt_put_mr(sge.mr); }
        (*ss).num_sge -= 1;
        if (*ss).num_sge != 0 {
            *sge = *(*ss).sg_list;
            (*ss).sg_list = (*ss).sg_list.add(1);
        }
    } else if sge.length == 0 && (*sge.mr).lkey != 0 {
        sge.n += 1;
        if sge.n as usize >= RVT_SEGSZ {
            sge.m += 1;
            if sge.m as u32 >= (*sge.mr).mapsz { return; }
            sge.n = 0;
        }
        let seg = &*(*(*sge.mr).map.add(sge.m as usize)).segs[sge.n as usize..][0];
        sge.vaddr = seg.vaddr;
        sge.length = seg.length as u32;
    }
}

#[inline]
pub unsafe fn rvt_skip_sge(ss: *mut rvt_sge_state, mut length: u32, release: bool) {
    while length != 0 {
        let len = rvt_get_sge_length(&mut (*ss).sge, length);
        WARN_ON_ONCE(len == 0);
        rvt_update_sge(ss, len, release);
        length -= len;
    }
}

extern "C" {
    pub fn rvt_ss_has_lkey(ss: *mut rvt_sge_state, lkey: u32) -> bool;
    pub fn rvt_mr_has_lkey(mr: *mut rvt_mregion, lkey: u32) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
