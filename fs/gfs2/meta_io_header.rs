/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
 * Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
 */

// Translated from meta_io.h.  Linux headers and incore.h provide the types
// and symbols referenced below.

#[inline]
pub unsafe fn gfs2_buffer_clear(bh: *mut buffer_head) {
    core::ptr::write_bytes((*bh).b_data, 0, (*bh).b_size as usize);
}

#[inline]
pub unsafe fn gfs2_buffer_clear_tail(bh: *mut buffer_head, head: i32) {
    // Equivalent to BUG_ON(head > bh->b_size).
    if head > (*bh).b_size {
        core::hint::unreachable_unchecked();
    }
    core::ptr::write_bytes(
        (*bh).b_data.add(head as usize),
        0,
        ((*bh).b_size - head) as usize,
    );
}

#[inline]
pub unsafe fn gfs2_buffer_copy_tail(
    to_bh: *mut buffer_head,
    to_head: i32,
    from_bh: *mut buffer_head,
    from_head: i32,
) {
    // Equivalent to BUG_ON(from_head < to_head).
    if from_head < to_head {
        core::hint::unreachable_unchecked();
    }
    core::ptr::copy_nonoverlapping(
        (*from_bh).b_data.add(from_head as usize),
        (*to_bh).b_data.add(to_head as usize),
        ((*from_bh).b_size - from_head) as usize,
    );
    core::ptr::write_bytes(
        (*to_bh)
            .b_data
            .add(((*to_bh).b_size + to_head - from_head) as usize),
        0,
        (from_head - to_head) as usize,
    );
}

extern "C" {
    pub static gfs2_meta_aops: address_space_operations;
    pub static gfs2_rgrp_aops: address_space_operations;

    pub fn gfs2_meta_new(gl: *mut gfs2_glock, blkno: u64) -> *mut buffer_head;
    pub fn gfs2_meta_read(
        gl: *mut gfs2_glock,
        blkno: u64,
        flags: i32,
        rahead: i32,
        bhp: *mut *mut buffer_head,
    ) -> i32;
    pub fn gfs2_meta_wait(sdp: *mut gfs2_sbd, bh: *mut buffer_head) -> i32;
    pub fn gfs2_getbuf(gl: *mut gfs2_glock, blkno: u64, create: i32) -> *mut buffer_head;
    pub fn gfs2_meta_ra(
        gl: *mut gfs2_glock,
        dblock: u64,
        extlen: u32,
    ) -> *mut buffer_head;
    pub fn gfs2_journal_wipe(ip: *mut gfs2_inode, bstart: u64, blen: u32);
    pub fn gfs2_meta_buffer(
        ip: *mut gfs2_inode,
        mtype: u32,
        num: u64,
        bhp: *mut *mut buffer_head,
    ) -> i32;
    pub fn glock_sbd(gla: *mut gfs2_glock_aspace) -> *mut gfs2_sbd;
}

#[repr(C)]
pub enum buffer_head {}
#[repr(C)]
pub enum address_space_operations {}
#[repr(C)]
pub enum address_space {}
#[repr(C)]
pub enum inode {}
#[repr(C)]
pub enum gfs2_glock {}
#[repr(C)]
pub enum gfs2_sbd {}
#[repr(C)]
pub enum gfs2_inode {}
#[repr(C)]
pub enum gfs2_glock_aspace {}

pub const REMOVE_JDATA: i32 = 0;
pub const REMOVE_META: i32 = 1;

// Equivalent to the C inline gfs2_mapping2sbd helper.  Its structure layout,
// container_of operation, and glock_sbd dependency are supplied by incore.h
// and the Linux headers.
#[inline]
pub unsafe fn gfs2_mapping2sbd(mapping: *mut address_space) -> *mut gfs2_sbd {
    // File-local Rust cannot express the external Linux structure layout
    // without importing those dependency definitions.
    extern_mapping2sbd(mapping)
}

extern "C" {
    fn extern_mapping2sbd(mapping: *mut address_space) -> *mut gfs2_sbd;
}

#[inline]
pub unsafe fn gfs2_meta_inode_buffer(
    ip: *mut gfs2_inode,
    bhp: *mut *mut buffer_head,
) -> i32 {
    gfs2_meta_buffer(ip, GFS2_METATYPE_DI as u32, (*ip).i_no_addr, bhp)
}

#[inline]
pub unsafe fn buffer_busy(bh: *const buffer_head) -> bool {
    ((*bh).b_state & ((1usize << BH_Dirty) | (1usize << BH_Lock) | (1usize << BH_Pinned))) != 0
}

// The following names are supplied by the translated dependency headers.
extern "C" {
    static GFS2_METATYPE_DI: u32;
    static BH_Dirty: u32;
    static BH_Lock: u32;
    static BH_Pinned: u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
