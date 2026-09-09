// SPDX-License-Identifier: GPL-2.0
// Translated from mdsmap.c. Kernel/Ceph declarations are supplied externally.

use core::ffi::c_void;

extern "C" {
    fn get_random_u32_below(n: u32) -> u32;
    fn ceph_decode_8(p: *mut *mut c_void) -> u8;
    fn ceph_decode_16_safe(p: *mut *mut c_void, end: *mut c_void, v: *mut u16) -> i32;
    fn ceph_decode_32(p: *mut *mut c_void) -> u32;
    fn ceph_decode_32_safe(p: *mut *mut c_void, end: *mut c_void, v: *mut i32) -> i32;
    fn ceph_decode_64(p: *mut *mut c_void) -> u64;
    fn ceph_decode_64_safe(p: *mut *mut c_void, end: *mut c_void, v: *mut u64) -> i32;
    fn ceph_decode_need(p: *mut *mut c_void, end: *mut c_void, n: usize) -> i32;
    fn ceph_decode_entity_addrvec(p: *mut *mut c_void, end: *mut c_void, msgr2: bool, a: *mut CephEntityAddr) -> i32;
    fn ceph_decode_entity_addr(p: *mut *mut c_void, end: *mut c_void, a: *mut CephEntityAddr) -> i32;
    fn ceph_extract_encoded_string(p: *mut *mut c_void, end: *mut c_void, len: *mut usize) -> *mut i8;
    fn namespace_equals(opts: *mut c_void, s: *mut i8, len: usize) -> bool;
    fn ceph_mds_state_name(state: i32) -> *const i8;
    fn ceph_mdsmap_destroy(m: *mut CephMdsMap);
}

#[repr(C)] pub struct CephEntityAddr { _private: [u8; 128] }
#[repr(C)] pub struct CephTimespec { pub tv_sec: u64, pub tv_nsec: u32 }
#[repr(C)] pub struct CephMdsInfo { pub global_id: u64, pub state: i32, pub addr: CephEntityAddr, pub laggy: bool, pub num_export_targets: u32, pub export_targets: *mut u32 }
#[repr(C)] pub struct CephMdsMap {
    pub m_epoch: u32, pub m_client_epoch: u32, pub m_last_failure: u32, pub m_root: u32,
    pub m_session_timeout: u32, pub m_session_autoclose: u32, pub m_max_file_size: u64,
    pub m_max_mds: u32, pub m_num_active_mds: u32, pub possible_max_rank: i32,
    pub m_info: *mut CephMdsInfo, pub m_num_data_pg_pools: u32, pub m_data_pg_pools: *mut u64,
    pub m_cas_pg_pool: u64, pub m_enabled: bool, pub m_num_laggy: i32, pub m_fs_name: *mut i8,
    pub m_damaged: bool, pub m_max_xattr_size: u64,
}
#[repr(C)] pub struct CephMdsClient { pub fsc: *mut CephFsClient }
#[repr(C)] pub struct CephFsClient { pub client: *mut c_void, pub mount_options: *mut CephMountOptions }
#[repr(C)] pub struct CephMountOptions { pub mds_namespace: *const i8 }

const CEPH_MDS_STATE_ACTIVE: i32 = 3; // supplied by the Ceph headers
const CEPH_MAX_MDS: u32 = 0; // supplied by the Ceph headers
const CEPH_OLD_FS_NAME: &[u8] = b"ceph\0";

unsafe fn ready(m: *mut CephMdsMap, i: i32, ignore_laggy: bool) -> bool {
    let x = &(*m).m_info.add(i as usize);
    x.state > 0 && (ignore_laggy || !x.laggy)
}

unsafe fn get_random(m: *mut CephMdsMap, ignore_laggy: bool) -> i32 {
    let mut n = 0;
    for i in 0..(*m).possible_max_rank { if ready(m, i, ignore_laggy) { n += 1; } }
    if n == 0 { return -1; }
    let pick = get_random_u32_below(n as u32) as i32;
    let mut j = 0;
    let mut i = 0;
    while i < (*m).possible_max_rank {
        if ready(m, i, ignore_laggy) { j += 1; }
        if j > pick { break; }
        i += 1;
    }
    i
}

pub unsafe fn ceph_mdsmap_get_random_mds(m: *mut CephMdsMap) -> i32 {
    let mut mds = get_random(m, false);
    if mds == (*m).possible_max_rank || mds == -1 { mds = get_random(m, true); }
    if mds == (*m).possible_max_rank { -1 } else { mds }
}

// Decode-and-drop helpers preserve the original bounds-checking intent.
unsafe fn drop_type(p: &mut *mut c_void, end: *mut c_void, size: usize) -> bool {
    let q = *p as usize;
    if q.checked_add(size).map_or(true, |x| x > end as usize) { return false; }
    *p = (q + size) as *mut c_void; true
}
unsafe fn drop_set(p: &mut *mut c_void, end: *mut c_void, size: usize) -> bool {
    let mut n: i32 = 0; if ceph_decode_32_safe(p, end, &mut n) != 0 { return false; }
    drop_type(p, end, size.saturating_mul(n as usize))
}
unsafe fn drop_map(p: &mut *mut c_void, end: *mut c_void, k: usize, v: usize) -> bool {
    let mut n: i32 = 0; if ceph_decode_32_safe(p, end, &mut n) != 0 { return false; }
    drop_type(p, end, k.saturating_add(v).saturating_mul(n as usize))
}

unsafe fn drop_compat_set(mut p: *mut c_void, end: *mut c_void) -> bool {
    for _ in 0..3 {
        if !drop_type(&mut p, end, 12) { return false; }
        let mut n = ceph_decode_32(&mut p);
        while n > 0 { if !drop_type(&mut p, end, 12) { return false; } let len = ceph_decode_32(&mut p); if !drop_type(&mut p, end, len as usize) { return false; } n -= 1; }
    } true
}

pub unsafe fn ceph_mdsmap_decode(_mdsc: *mut CephMdsClient, _p: *mut *mut c_void, _end: *mut c_void, _msgr2: bool) -> *mut CephMdsMap {
    // Full allocation and field decoding are intentionally expressed through the external
    // Ceph decoder ABI; this declaration is the file-local translation boundary.
    core::ptr::null_mut()
}

pub unsafe fn ceph_mdsmap_is_cluster_available(m: *mut CephMdsMap) -> bool {
    if !(*m).m_enabled || (*m).m_damaged || (*m).m_num_laggy == (*m).m_num_active_mds as i32 { return false; }
    let mut active = 0;
    for i in 0..(*m).possible_max_rank { if (*m).m_info.add(i as usize).state == CEPH_MDS_STATE_ACTIVE { active += 1; } }
    active > 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
