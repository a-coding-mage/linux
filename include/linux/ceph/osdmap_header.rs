/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/ceph/osdmap.h.  Types and symbols supplied by the
// surrounding kernel/Ceph translation are intentionally referenced here.

#[repr(C)]
pub struct ceph_pg {
    pub pool: u64,
    pub seed: u32,
}

pub const CEPH_SPG_NOSHARD: i8 = -1;

#[repr(C)]
pub struct ceph_spg {
    pub pgid: ceph_pg,
    pub shard: i8,
}

pub const CEPH_POOL_FLAG_HASHPSPOOL: u64 = 1u64 << 0;
pub const CEPH_POOL_FLAG_FULL: u64 = 1u64 << 1;
pub const CEPH_POOL_FLAG_FULL_QUOTA: u64 = 1u64 << 10;
pub const CEPH_POOL_FLAG_NEARFULL: u64 = 1u64 << 11;

#[repr(C)]
pub struct ceph_pg_pool_info {
    pub node: rb_node,
    pub id: i64,
    pub type_: u8,
    pub size: u8,
    pub min_size: u8,
    pub crush_ruleset: u8,
    pub object_hash: u8,
    pub last_force_request_resend: u32,
    pub pg_num: u32,
    pub pgp_num: u32,
    pub pg_num_mask: i32,
    pub pgp_num_mask: i32,
    pub read_tier: i64,
    pub write_tier: i64,
    pub flags: u64,
    pub name: *mut std::ffi::c_char,
    pub was_full: bool,
}

pub unsafe fn ceph_can_shift_osds(pool: *mut ceph_pg_pool_info) -> bool {
    match (*pool).type_ {
        CEPH_POOL_TYPE_REP => true,
        CEPH_POOL_TYPE_EC => false,
        _ => { BUG(); false }
    }
}

#[repr(C)]
pub struct ceph_object_locator {
    pub pool: i64,
    pub pool_ns: *mut ceph_string,
}

pub unsafe fn ceph_oloc_init(oloc: *mut ceph_object_locator) {
    (*oloc).pool = -1;
    (*oloc).pool_ns = std::ptr::null_mut();
}

pub unsafe fn ceph_oloc_empty(oloc: *const ceph_object_locator) -> bool { (*oloc).pool == -1 }

pub const CEPH_OID_INLINE_LEN: usize = 52;

#[repr(C)]
pub struct ceph_object_id {
    pub name: *mut std::ffi::c_char,
    pub inline_name: [std::ffi::c_char; CEPH_OID_INLINE_LEN],
    pub name_len: i32,
}

pub unsafe fn ceph_oid_init(oid: *mut ceph_object_id) {
    (*oid).name = (*oid).inline_name.as_mut_ptr();
}

pub unsafe fn ceph_oid_empty(oid: *const ceph_object_id) -> bool {
    (*oid).name == (*oid).inline_name.as_ptr() as *mut _ && (*oid).name_len == 0
}

#[repr(C)]
pub struct workspace_manager {
    pub idle_ws: list_head,
    pub ws_lock: spinlock_t,
    pub free_ws: i32,
    pub total_ws: atomic_t,
    pub ws_wait: wait_queue_head_t,
}

#[repr(C)]
pub struct ceph_pg_mapping {
    pub node: rb_node,
    pub pgid: ceph_pg,
    pub data: ceph_pg_mapping_data,
}

#[repr(C)]
pub union ceph_pg_mapping_data {
    pub pg_temp: ceph_pg_mapping_set,
    pub pg_upmap: ceph_pg_mapping_set,
    pub primary_temp: ceph_pg_mapping_primary,
    pub pg_upmap_items: ceph_pg_mapping_items,
}

#[repr(C)]
pub struct ceph_pg_mapping_set { pub len: i32, pub osds: [i32; 0] }
#[repr(C)]
pub struct ceph_pg_mapping_primary { pub osd: i32 }
#[repr(C)]
pub struct ceph_pg_mapping_items { pub len: i32, pub from_to: [[i32; 2]; 0] }

#[repr(C)]
pub struct ceph_osdmap {
    pub fsid: ceph_fsid,
    pub epoch: u32,
    pub created: ceph_timespec,
    pub modified: ceph_timespec,
    pub flags: u32,
    pub max_osd: u32,
    pub osd_state: *mut u32,
    pub osd_weight: *mut u32,
    pub osd_addr: *mut ceph_entity_addr,
    pub pg_temp: rb_root,
    pub primary_temp: rb_root,
    pub pg_upmap: rb_root,
    pub pg_upmap_items: rb_root,
    pub osd_primary_affinity: *mut u32,
    pub pg_pools: rb_root,
    pub pool_max: u32,
    pub crush: *mut crush_map,
    pub crush_wsm: workspace_manager,
}

pub unsafe fn ceph_osd_exists(map: *mut ceph_osdmap, osd: i32) -> bool {
    osd >= 0 && (osd as u32) < (*map).max_osd && (*map).osd_state.add(osd as usize).read() & CEPH_OSD_EXISTS != 0
}
pub unsafe fn ceph_osd_is_up(map: *mut ceph_osdmap, osd: i32) -> bool {
    ceph_osd_exists(map, osd) && (*map).osd_state.add(osd as usize).read() & CEPH_OSD_UP != 0
}
pub unsafe fn ceph_osd_is_down(map: *mut ceph_osdmap, osd: i32) -> bool { !ceph_osd_is_up(map, osd) }
pub unsafe fn ceph_osd_addr(map: *mut ceph_osdmap, osd: i32) -> *mut ceph_entity_addr {
    if osd >= (*map).max_osd as i32 { std::ptr::null_mut() } else { (*map).osd_addr.add(osd as usize) }
}

pub const CEPH_PGID_ENCODING_LEN: usize = 1 + 8 + 4 + 4;

pub unsafe fn ceph_decode_pgid(p: *mut *mut std::ffi::c_void, end: *mut std::ffi::c_void, pgid: *mut ceph_pg) -> i32 {
    let version = ceph_decode_8(p);
    if version > 1 { return -EINVAL; }
    (*pgid).pool = ceph_decode_64(p);
    (*pgid).seed = ceph_decode_32(p);
    *p = (*p as *mut u8).add(4) as *mut std::ffi::c_void;
    0
}

#[repr(C)]
pub struct ceph_osds { pub osds: [i32; CEPH_PG_MAX_SIZE], pub size: i32, pub primary: i32 }
pub unsafe fn ceph_osds_init(set: *mut ceph_osds) { (*set).size = 0; (*set).primary = -1; }

extern "C" {
    pub fn ceph_pg_is_split(pgid: *const ceph_pg, old_pg_num: u32, new_pg_num: u32) -> bool;
    pub fn ceph_is_new_interval(old_acting: *const ceph_osds, new_acting: *const ceph_osds,
        old_up: *const ceph_osds, new_up: *const ceph_osds, old_size: i32, new_size: i32,
        old_min_size: i32, new_min_size: i32, old_pg_num: u32, new_pg_num: u32,
        old_sort_bitwise: bool, new_sort_bitwise: bool, old_recovery_deletes: bool,
        new_recovery_deletes: bool, pgid: *const ceph_pg) -> bool;
    pub fn ceph_osds_changed(old_acting: *const ceph_osds, new_acting: *const ceph_osds, any_change: bool) -> bool;
    pub fn __ceph_object_locator_to_pg(pi: *mut ceph_pg_pool_info, oid: *const ceph_object_id,
        oloc: *const ceph_object_locator, raw_pgid: *mut ceph_pg);
    pub fn ceph_object_locator_to_pg(osdmap: *mut ceph_osdmap, oid: *const ceph_object_id,
        oloc: *const ceph_object_locator, raw_pgid: *mut ceph_pg) -> i32;
    pub fn ceph_pg_to_up_acting_osds(osdmap: *mut ceph_osdmap, pi: *mut ceph_pg_pool_info,
        raw_pgid: *const ceph_pg, up: *mut ceph_osds, acting: *mut ceph_osds);
    pub fn ceph_pg_to_primary_shard(osdmap: *mut ceph_osdmap, pi: *mut ceph_pg_pool_info,
        raw_pgid: *const ceph_pg, spgid: *mut ceph_spg) -> bool;
    pub fn ceph_pg_to_acting_primary(osdmap: *mut ceph_osdmap, raw_pgid: *const ceph_pg) -> i32;
}

pub struct crush_loc { pub cl_type_name: *mut std::ffi::c_char, pub cl_name: *mut std::ffi::c_char }
#[repr(C)]
pub struct crush_loc_node { pub cl_node: rb_node, pub cl_loc: crush_loc, pub cl_data: [std::ffi::c_char; 0] }

extern "C" {
    pub fn ceph_pg_compare(lhs: *const ceph_pg, rhs: *const ceph_pg) -> i32;
    pub fn ceph_spg_compare(lhs: *const ceph_spg, rhs: *const ceph_spg) -> i32;
    pub fn ceph_oloc_copy(dest: *mut ceph_object_locator, src: *const ceph_object_locator);
    pub fn ceph_oloc_destroy(oloc: *mut ceph_object_locator);
    pub fn ceph_oid_copy(dest: *mut ceph_object_id, src: *const ceph_object_id);
    pub fn ceph_oid_printf(oid: *mut ceph_object_id, fmt: *const std::ffi::c_char, ...);
    pub fn ceph_oid_aprintf(oid: *mut ceph_object_id, gfp: gfp_t, fmt: *const std::ffi::c_char, ...) -> i32;
    pub fn ceph_oid_destroy(oid: *mut ceph_object_id);
    pub fn ceph_osdmap_state_str(str_: *mut std::ffi::c_char, len: i32, state: u32) -> *mut std::ffi::c_char;
    pub fn ceph_get_primary_affinity(map: *mut ceph_osdmap, osd: i32) -> u32;
    pub fn ceph_osdmap_alloc() -> *mut ceph_osdmap;
    pub fn ceph_osdmap_decode(p: *mut *mut std::ffi::c_void, end: *mut std::ffi::c_void, msgr2: bool) -> *mut ceph_osdmap;
    pub fn osdmap_apply_incremental(p: *mut *mut std::ffi::c_void, end: *mut std::ffi::c_void, msgr2: bool, map: *mut ceph_osdmap) -> *mut ceph_osdmap;
    pub fn ceph_osdmap_destroy(map: *mut ceph_osdmap);
    pub fn ceph_osds_copy(dest: *mut ceph_osds, src: *const ceph_osds);
    pub fn ceph_parse_crush_location(crush_location: *mut std::ffi::c_char, locs: *mut rb_root) -> i32;
    pub fn ceph_compare_crush_locs(locs1: *mut rb_root, locs2: *mut rb_root) -> i32;
    pub fn ceph_clear_crush_locs(locs: *mut rb_root);
    pub fn ceph_get_crush_locality(osdmap: *mut ceph_osdmap, id: i32, locs: *mut rb_root) -> i32;
    pub fn ceph_pg_pool_by_id(map: *mut ceph_osdmap, id: u64) -> *mut ceph_pg_pool_info;
    pub fn ceph_pg_pool_name_by_id(map: *mut ceph_osdmap, id: u64) -> *const std::ffi::c_char;
    pub fn ceph_pg_poolid_by_name(map: *mut ceph_osdmap, name: *const std::ffi::c_char) -> i32;
    pub fn ceph_pg_pool_flags(map: *mut ceph_osdmap, id: u64) -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
