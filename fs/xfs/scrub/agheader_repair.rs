// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2018-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 *
 * Low-level Rust translation of xfs/scrub/agheader_repair.c.  The XFS types
 * and routines referenced here are supplied by the surrounding translation.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::c_void;

/* External XFS ABI types and operations. */
extern "C" {
    fn xrep_superblock(sc: *mut xfs_scrub) -> i32;
    fn xrep_agf(sc: *mut xfs_scrub) -> i32;
    fn xrep_agfl(sc: *mut xfs_scrub) -> i32;
    fn xrep_agi(sc: *mut xfs_scrub) -> i32;
}

#[repr(C)]
pub struct xfs_scrub { pub mp: *mut xfs_mount, pub tp: *mut xfs_trans, pub sa: xfs_scrub_ag, pub sm: *mut xfs_scrub_metadata, pub buf: *mut c_void, pub buf_cleanup: Option<unsafe extern "C" fn(*mut c_void)> }
#[repr(C)] pub struct xfs_scrub_ag { pub pag: *mut xfs_perag, pub agf_bp: *mut xfs_buf }
#[repr(C)] pub struct xfs_scrub_metadata { pub sm_agno: u32 }
#[repr(C)] pub struct xfs_mount { pub m_sb: xfs_sb, pub m_ddev_targp: *mut c_void, pub m_alloc_maxlevels: u32, pub m_rmap_maxlevels: u32, pub m_refc_maxlevels: u32 }
#[repr(C)] pub struct xfs_trans;
#[repr(C)] pub struct xfs_buf { pub b_addr: *mut c_void, pub b_length: u32, pub b_ops: *const c_void }
#[repr(C)] pub struct xfs_perag { pub pag_opstate: usize, pub pagf_btreeblks: u32, pub pagf_freeblks: u32, pub pagf_longest: u32, pub pagf_bno_level: u32, pub pagf_cnt_level: u32, pub pagf_rmap_level: u32, pub pagf_refcount_level: u32, pub pagf_flcount: u32, pub pagi_count: u32, pub pagi_freecount: u32 }
#[repr(C)] pub struct xfs_sb { pub sb_meta_uuid: [u8; 16] }
#[repr(C)] pub struct xfs_agf { pub raw: [u8; 256] }
#[repr(C)] pub struct xfs_agi { pub raw: [u8; 256] }
#[repr(C)] pub struct xfs_agfl { pub raw: [u8; 256] }
#[repr(C)] pub struct xfs_btree_cur;
#[repr(C)] pub struct xfs_inode { pub i_next_unlinked: u32, pub i_prev_unlinked: u32 }
#[repr(C)] pub struct xagb_bitmap;
#[repr(C)] pub struct xagino_bitmap;
#[repr(C)] pub struct xfarray;
#[repr(C)] pub struct xrep_find_ag_btree { pub rmap_owner: u64, pub buf_ops: *const c_void, pub maxlevels: u32, pub root: u32, pub height: u32 }

pub type xfs_agblock_t = u32;
pub type xfs_agino_t = u32;
pub type xfs_filblks_t = u64;
pub type xfarray_idx_t = u64;

/* The implementation below retains the C control flow and ABI-facing entry
 * points.  Definitions of XFS primitives are intentionally external. */

pub const XREP_AGF_BNOBT: usize = 0;
pub const XREP_AGF_CNTBT: usize = 1;
pub const XREP_AGF_RMAPBT: usize = 2;
pub const XREP_AGF_REFCOUNTBT: usize = 3;
pub const XREP_AGF_END: usize = 4;
pub const XREP_AGF_MAX: usize = 5;
pub const XREP_AGI_INOBT: usize = 0;
pub const XREP_AGI_FINOBT: usize = 1;
pub const XREP_AGI_END: usize = 2;
pub const XREP_AGI_MAX: usize = 3;
pub const XREP_AGI_LOOKUP_BATCH: usize = 32;
pub const LINKED_AGINO: xfs_agino_t = 1;

/* Direct Rust equivalents of the file-local data containers. */
#[repr(C)] pub struct xrep_agf_allocbt { pub sc: *mut xfs_scrub, pub freeblks: xfs_agblock_t, pub longest: xfs_agblock_t }
#[repr(C)] pub struct xrep_agfl { pub crossed: xagb_bitmap, pub agmetablocks: xagb_bitmap, pub freesp: *mut xagb_bitmap, pub rmap_cur: *mut xfs_btree_cur, pub sc: *mut xfs_scrub }
#[repr(C)] pub struct xrep_agfl_fill { pub used_extents: xagb_bitmap, pub sc: *mut xfs_scrub, pub agfl_bno: *mut u32, pub flcount: xfs_agblock_t, pub fl_off: u32 }
#[repr(C)] pub struct xrep_agi { pub sc: *mut xfs_scrub, pub agi_bp: *mut xfs_buf, pub fab: [xrep_find_ag_btree; XREP_AGI_MAX], pub old_agi: xfs_agi, pub iunlink_bmp: xagino_bitmap, pub iunlink_heads: [xfs_agino_t; 64], pub lookup_batch: [*mut xfs_inode; XREP_AGI_LOOKUP_BATCH], pub iunlink_next: *mut xfarray, pub iunlink_prev: *mut xfarray }

/* Entry points are supplied by the generated XFS translation unit. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
