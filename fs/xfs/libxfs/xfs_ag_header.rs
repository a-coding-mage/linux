/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2018 Red Hat, Inc.
 * All rights reserved.
 */

// Dependency: xfs_group.h

use core::ffi::c_void;

pub struct xfs_mount;
pub struct xfs_trans;

#[repr(C)]
pub struct xfs_ag_resv {
    pub ar_orig_reserved: xfs_extlen_t,
    pub ar_reserved: xfs_extlen_t,
    pub ar_asked: xfs_extlen_t,
}

#[repr(C)]
pub struct xfs_perag {
    pub pag_group: xfs_group,
    pub pag_opstate: libc::c_ulong,
    pub pagf_bno_level: u8,
    pub pagf_cnt_level: u8,
    pub pagf_rmap_level: u8,
    pub pagf_flcount: u32,
    pub pagf_freeblks: xfs_extlen_t,
    pub pagf_longest: xfs_extlen_t,
    pub pagf_btreeblks: u32,
    pub pagi_freecount: xfs_agino_t,
    pub pagi_count: xfs_agino_t,
    pub pagl_pagino: xfs_agino_t,
    pub pagl_leftrec: xfs_agino_t,
    pub pagl_rightrec: xfs_agino_t,
    pub pagf_refcount_level: u8,
    pub pag_meta_resv: xfs_ag_resv,
    pub pag_rmapbt_resv: xfs_ag_resv,
    pub agino_min: xfs_agino_t,
    pub agino_max: xfs_agino_t,
}

pub unsafe fn to_perag(xg: *mut xfs_group) -> *mut xfs_perag {
    container_of!(xg, xfs_perag, pag_group)
}

pub unsafe fn pag_group(pag: *mut xfs_perag) -> *mut xfs_group { &mut (*pag).pag_group }
pub unsafe fn pag_mount(pag: *const xfs_perag) -> *mut xfs_mount { (*pag).pag_group.xg_mount }
pub unsafe fn pag_agno(pag: *const xfs_perag) -> xfs_agnumber_t { (*pag).pag_group.xg_gno }

pub const XFS_AGSTATE_AGF_INIT: u32 = 0;
pub const XFS_AGSTATE_AGI_INIT: u32 = 1;
pub const XFS_AGSTATE_PREFERS_METADATA: u32 = 2;
pub const XFS_AGSTATE_ALLOWS_INODES: u32 = 3;
pub const XFS_AGSTATE_AGFL_NEEDS_RESET: u32 = 4;

macro_rules! __XFS_AG_OPSTATE {
    ($name:ident, $bit:ident) => {
        pub unsafe fn $name(pag: *mut xfs_perag) -> bool {
            test_bit(XFS_AGSTATE_$bit, &mut (*pag).pag_opstate)
        }
    };
}
__XFS_AG_OPSTATE!(xfs_perag_initialised_agf, AGF_INIT);
__XFS_AG_OPSTATE!(xfs_perag_initialised_agi, AGI_INIT);
__XFS_AG_OPSTATE!(xfs_perag_prefers_metadata, PREFERS_METADATA);
__XFS_AG_OPSTATE!(xfs_perag_allows_inodes, ALLOWS_INODES);
__XFS_AG_OPSTATE!(xfs_perag_agfl_needs_reset, AGFL_NEEDS_RESET);

extern "C" {
    pub fn xfs_initialize_perag(mp: *mut xfs_mount, orig_agcount: xfs_agnumber_t, new_agcount: xfs_agnumber_t, dcount: xfs_rfsblock_t, maxagi: *mut xfs_agnumber_t) -> libc::c_int;
    pub fn xfs_free_perag_range(mp: *mut xfs_mount, first_agno: xfs_agnumber_t, end_agno: xfs_agnumber_t);
    pub fn xfs_initialize_perag_data(mp: *mut xfs_mount, agno: xfs_agnumber_t) -> libc::c_int;
    pub fn xfs_update_last_ag_size(mp: *mut xfs_mount, prev_agcount: xfs_agnumber_t) -> libc::c_int;
}

pub unsafe fn xfs_perag_get(mp: *mut xfs_mount, agno: xfs_agnumber_t) -> *mut xfs_perag { to_perag(xfs_group_get(mp, agno, XG_TYPE_AG)) }
pub unsafe fn xfs_perag_hold(pag: *mut xfs_perag) -> *mut xfs_perag { to_perag(xfs_group_hold(pag_group(pag))) }
pub unsafe fn xfs_perag_put(pag: *mut xfs_perag) { xfs_group_put(pag_group(pag)); }
pub unsafe fn xfs_perag_grab(mp: *mut xfs_mount, agno: xfs_agnumber_t) -> *mut xfs_perag { to_perag(xfs_group_grab(mp, agno, XG_TYPE_AG)) }
pub unsafe fn xfs_perag_rele(pag: *mut xfs_perag) { xfs_group_rele(pag_group(pag)); }

pub unsafe fn xfs_perag_next_range(mp: *mut xfs_mount, pag: *mut xfs_perag, start_agno: xfs_agnumber_t, end_agno: xfs_agnumber_t) -> *mut xfs_perag {
    to_perag(xfs_group_next_range(mp, if !pag.is_null() { pag_group(pag) } else { core::ptr::null_mut() }, start_agno, end_agno, XG_TYPE_AG))
}
pub unsafe fn xfs_perag_next_from(mp: *mut xfs_mount, pag: *mut xfs_perag, start_agno: xfs_agnumber_t) -> *mut xfs_perag { xfs_perag_next_range(mp, pag, start_agno, (*mp).m_sb.sb_agcount - 1) }
pub unsafe fn xfs_perag_next(mp: *mut xfs_mount, pag: *mut xfs_perag) -> *mut xfs_perag { xfs_perag_next_from(mp, pag, 0) }

pub unsafe fn xfs_verify_agbno(pag: *mut xfs_perag, agbno: xfs_agblock_t) -> bool { xfs_verify_gbno(pag_group(pag), agbno) }
pub unsafe fn xfs_verify_agbext(pag: *mut xfs_perag, agbno: xfs_agblock_t, len: xfs_agblock_t) -> bool { xfs_verify_gbext(pag_group(pag), agbno, len) }

pub unsafe fn xfs_ag_contains_log(mp: *mut xfs_mount, agno: xfs_agnumber_t) -> bool {
    (*mp).m_sb.sb_logstart > 0 && agno == XFS_FSB_TO_AGNO(mp, (*mp).m_sb.sb_logstart)
}

pub unsafe fn xfs_perag_next_wrap(pag: *mut xfs_perag, agno: *mut xfs_agnumber_t, stop_agno: xfs_agnumber_t, restart_agno: xfs_agnumber_t, wrap_agno: xfs_agnumber_t) -> *mut xfs_perag {
    let mp = pag_mount(pag);
    *agno = pag_agno(pag) + 1;
    xfs_perag_rele(pag);
    while *agno != stop_agno {
        if *agno >= wrap_agno {
            if restart_agno >= stop_agno { break; }
            *agno = restart_agno;
        }
        pag = xfs_perag_grab(mp, *agno);
        if !pag.is_null() { return pag; }
        *agno += 1;
    }
    core::ptr::null_mut()
}

macro_rules! for_each_perag_wrap_range {
    ($mp:expr, $start:expr, $restart:expr, $wrap:expr, $agno:expr, $pag:expr, $body:block) => {{
        $agno = $start; $pag = unsafe { xfs_perag_grab($mp, $agno) };
        while !$pag.is_null() { $body; $pag = unsafe { xfs_perag_next_wrap($pag, &mut $agno, $start, $restart, $wrap) }; }
    }};
}
macro_rules! for_each_perag_wrap_at { ($($t:tt)*) => { for_each_perag_wrap_range!($($t)*) }; }
macro_rules! for_each_perag_wrap { ($($t:tt)*) => { for_each_perag_wrap_at!($($t)*) }; }

extern "C" {
    pub fn xfs_ag_block_count(mp: *mut xfs_mount, agno: xfs_agnumber_t) -> xfs_agblock_t;
    pub fn xfs_agino_range(mp: *mut xfs_mount, agno: xfs_agnumber_t, first: *mut xfs_agino_t, last: *mut xfs_agino_t);
    pub fn xfs_ag_init_headers(mp: *mut xfs_mount, id: *mut aghdr_init_data) -> libc::c_int;
    pub fn xfs_ag_shrink_space(pag: *mut xfs_perag, tpp: *mut *mut xfs_trans, delta: xfs_extlen_t) -> libc::c_int;
    pub fn xfs_ag_extend_space(pag: *mut xfs_perag, tp: *mut xfs_trans, len: xfs_extlen_t) -> libc::c_int;
    pub fn xfs_ag_get_geometry(pag: *mut xfs_perag, ageo: *mut xfs_ag_geometry) -> libc::c_int;
    pub fn xfs_growfs_compute_agcount(mp: *mut xfs_mount, nb: *mut xfs_rfsblock_t) -> xfs_agnumber_t;
}

pub unsafe fn xfs_verify_agino(pag: *mut xfs_perag, agino: xfs_agino_t) -> bool { agino >= (*pag).agino_min && agino <= (*pag).agino_max }
pub unsafe fn xfs_verify_agino_or_null(pag: *mut xfs_perag, agino: xfs_agino_t) -> bool { agino == NULLAGINO || xfs_verify_agino(pag, agino) }

#[repr(C)]
pub struct aghdr_init_data {
    pub agno: xfs_agblock_t, pub agsize: xfs_extlen_t, pub buffer_list: list_head,
    pub nfree: xfs_rfsblock_t, pub daddr: xfs_daddr_t, pub numblks: usize,
    pub bc_ops: *const xfs_btree_ops,
}

pub unsafe fn xfs_agbno_to_fsb(pag: *mut xfs_perag, agbno: xfs_agblock_t) -> xfs_fsblock_t { XFS_AGB_TO_FSB(pag_mount(pag), pag_agno(pag), agbno) }
pub unsafe fn xfs_agbno_to_daddr(pag: *mut xfs_perag, agbno: xfs_agblock_t) -> xfs_daddr_t { XFS_AGB_TO_DADDR(pag_mount(pag), pag_agno(pag), agbno) }
pub unsafe fn xfs_agino_to_ino(pag: *mut xfs_perag, agino: xfs_agino_t) -> xfs_ino_t { XFS_AGINO_TO_INO(pag_mount(pag), pag_agno(pag), agino) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
