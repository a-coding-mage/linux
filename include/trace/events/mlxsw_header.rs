/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/* Copyright (c) 2019 Mellanox Technologies. All rights reserved */

//! Rust representation of the mlxsw tracepoint header.
//!
//! The C `TRACE_EVENT` machinery is supplied by the Linux tracepoint
//! dependency.  The declarations below preserve the event payload layout and
//! the event names; registration and printing remain external tracepoint
//! functionality.

use core::ffi::c_void;

#[repr(C)]
pub struct mlxsw_sp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mlxsw_sp_acl_atcam_region {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mlxsw_sp_acl_tcam_vregion {
    _private: [u8; 0],
}

#[repr(C)]
pub struct MlxswSpAclAtcamEntryAddCtcamSpillEntry {
    pub mlxsw_sp: *const c_void,
    pub aregion: *const c_void,
}

#[repr(C)]
pub struct MlxswSpAclTcamVregionRehashEntry {
    pub mlxsw_sp: *const c_void,
    pub vregion: *const c_void,
}

#[repr(C)]
pub struct MlxswSpAclTcamVregionMigrateEntry {
    pub mlxsw_sp: *const c_void,
    pub vregion: *const c_void,
}

#[repr(C)]
pub struct MlxswSpAclTcamVregionMigrateEndEntry {
    pub mlxsw_sp: *const c_void,
    pub vregion: *const c_void,
}

#[repr(C)]
pub struct MlxswSpAclTcamVregionRehashRollbackFailedEntry {
    pub mlxsw_sp: *const c_void,
    pub vregion: *const c_void,
}

/// Tracepoint payload for `mlxsw_sp_acl_atcam_entry_add_ctcam_spill`.
#[inline]
pub unsafe fn mlxsw_sp_acl_atcam_entry_add_ctcam_spill(
    mlxsw_sp: *const mlxsw_sp,
    aregion: *const mlxsw_sp_acl_atcam_region,
) -> MlxswSpAclAtcamEntryAddCtcamSpillEntry {
    MlxswSpAclAtcamEntryAddCtcamSpillEntry {
        mlxsw_sp: mlxsw_sp.cast(),
        aregion: aregion.cast(),
    }
}

/// Tracepoint payload for `mlxsw_sp_acl_tcam_vregion_rehash`.
#[inline]
pub unsafe fn mlxsw_sp_acl_tcam_vregion_rehash(
    mlxsw_sp: *const mlxsw_sp,
    vregion: *const mlxsw_sp_acl_tcam_vregion,
) -> MlxswSpAclTcamVregionRehashEntry {
    MlxswSpAclTcamVregionRehashEntry {
        mlxsw_sp: mlxsw_sp.cast(),
        vregion: vregion.cast(),
    }
}

/// Tracepoint payload for `mlxsw_sp_acl_tcam_vregion_migrate`.
#[inline]
pub unsafe fn mlxsw_sp_acl_tcam_vregion_migrate(
    mlxsw_sp: *const mlxsw_sp,
    vregion: *const mlxsw_sp_acl_tcam_vregion,
) -> MlxswSpAclTcamVregionMigrateEntry {
    MlxswSpAclTcamVregionMigrateEntry {
        mlxsw_sp: mlxsw_sp.cast(),
        vregion: vregion.cast(),
    }
}

/// Tracepoint payload for `mlxsw_sp_acl_tcam_vregion_migrate_end`.
#[inline]
pub unsafe fn mlxsw_sp_acl_tcam_vregion_migrate_end(
    mlxsw_sp: *const mlxsw_sp,
    vregion: *const mlxsw_sp_acl_tcam_vregion,
) -> MlxswSpAclTcamVregionMigrateEndEntry {
    MlxswSpAclTcamVregionMigrateEndEntry {
        mlxsw_sp: mlxsw_sp.cast(),
        vregion: vregion.cast(),
    }
}

/// Tracepoint payload for `mlxsw_sp_acl_tcam_vregion_rehash_rollback_failed`.
#[inline]
pub unsafe fn mlxsw_sp_acl_tcam_vregion_rehash_rollback_failed(
    mlxsw_sp: *const mlxsw_sp,
    vregion: *const mlxsw_sp_acl_tcam_vregion,
) -> MlxswSpAclTcamVregionRehashRollbackFailedEntry {
    MlxswSpAclTcamVregionRehashRollbackFailedEntry {
        mlxsw_sp: mlxsw_sp.cast(),
        vregion: vregion.cast(),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
