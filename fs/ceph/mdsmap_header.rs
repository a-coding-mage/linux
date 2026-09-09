/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// ceph_entity_addr, ceph_mds_client, and CEPH_MDS_STATE_DNE.

use core::ptr;

pub struct ceph_mds_client;

/*
 * mds map - describe servers in the mds cluster.
 *
 * we limit fields to those the client actually xcares about
 */
#[repr(C)]
pub struct ceph_mds_info {
    pub global_id: u64,
    pub addr: ceph_entity_addr,
    pub state: i32,
    pub num_export_targets: i32,
    pub laggy: bool,
    pub export_targets: *mut u32,
}

#[repr(C)]
pub struct ceph_mdsmap {
    pub m_epoch: u32,
    pub m_client_epoch: u32,
    pub m_last_failure: u32,
    pub m_root: u32,
    pub m_session_timeout: u32,       /* seconds */
    pub m_session_autoclose: u32,     /* seconds */
    pub m_max_file_size: u64,
    /*
     * maximum size for xattrs blob.
     * Zeroed by default to force the usage of the (sync) SETXATTR Op.
     */
    pub m_max_xattr_size: u64,
    pub m_max_mds: u32,               /* expected up:active mds number */
    pub m_num_active_mds: u32,        /* actual up:active mds number */
    pub possible_max_rank: u32,       /* possible max rank index */
    pub m_info: *mut ceph_mds_info,

    /* which object pools file data can be stored in */
    pub m_num_data_pg_pools: i32,
    pub m_data_pg_pools: *mut u64,
    pub m_cas_pg_pool: u64,

    pub m_enabled: bool,
    pub m_damaged: bool,
    pub m_num_laggy: i32,
    pub m_fs_name: *mut i8,
}

#[inline]
pub unsafe fn ceph_mdsmap_get_addr(m: *mut ceph_mdsmap, w: i32) -> *mut ceph_entity_addr {
    if w >= (*m).possible_max_rank as i32 {
        return ptr::null_mut();
    }
    &mut (*m).m_info.add(w as usize).addr
}

#[inline]
pub unsafe fn ceph_mdsmap_get_state(m: *mut ceph_mdsmap, w: i32) -> i32 {
    assert!(w >= 0);
    if w >= (*m).possible_max_rank as i32 {
        return CEPH_MDS_STATE_DNE;
    }
    (*m).m_info.add(w as usize).state
}

#[inline]
pub unsafe fn ceph_mdsmap_is_laggy(m: *mut ceph_mdsmap, w: i32) -> bool {
    if w >= 0 && w < (*m).possible_max_rank as i32 {
        return (*m).m_info.add(w as usize).laggy;
    }
    false
}

extern "C" {
    pub fn ceph_mdsmap_get_random_mds(m: *mut ceph_mdsmap) -> i32;
    pub fn ceph_mdsmap_decode(
        mdsc: *mut ceph_mds_client,
        p: *mut *mut core::ffi::c_void,
        end: *mut core::ffi::c_void,
        msgr2: bool,
    ) -> *mut ceph_mdsmap;
    pub fn ceph_mdsmap_destroy(m: *mut ceph_mdsmap);
    pub fn ceph_mdsmap_is_cluster_available(m: *mut ceph_mdsmap) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
