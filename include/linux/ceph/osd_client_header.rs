/* SPDX-License-Identifier: GPL-2.0 */
// Translated from osd_client.h. Kernel dependency declarations are supplied by
// the surrounding translation unit.

use core::ffi::{c_char, c_void, c_uint, c_ulong};

pub type ceph_osdc_callback_t = unsafe extern "C" fn(*mut ceph_osd_request);
pub const CEPH_HOMELESS_OSD: i32 = -1;

#[repr(C, packed)]
pub struct ceph_sparse_extent { pub off: u64, pub len: u64 }

#[repr(C)]
pub enum ceph_sparse_read_state { CEPH_SPARSE_READ_HDR = 0, CEPH_SPARSE_READ_EXTENTS, CEPH_SPARSE_READ_DATA_LEN, CEPH_SPARSE_READ_DATA_PRE, CEPH_SPARSE_READ_DATA }

#[repr(C)]
pub struct ceph_sparse_read {
    pub sr_state: ceph_sparse_read_state, pub sr_req_off: u64, pub sr_req_len: u64,
    pub sr_pos: u64, pub sr_index: i32, pub sr_datalen: u32, pub sr_count: u32,
    pub sr_ext_len: i32, pub sr_extent: *mut ceph_sparse_extent,
}

#[repr(C)]
pub struct ceph_osd {
    pub o_ref: refcount_t, pub o_sparse_op_idx: i32, pub o_osdc: *mut ceph_osd_client,
    pub o_osd: i32, pub o_incarnation: i32, pub o_node: rb_node, pub o_con: ceph_connection,
    pub o_requests_lock: spinlock_t, pub o_requests: rb_root, pub o_linger_requests: rb_root,
    pub o_backoff_mappings: rb_root, pub o_backoffs_by_id: rb_root, pub o_osd_lru: list_head,
    pub o_auth: ceph_auth_handshake, pub lru_ttl: c_ulong, pub o_keepalive_item: list_head,
    pub lock: mutex, pub o_sparse_read: ceph_sparse_read,
}

pub const CEPH_OSD_SLAB_OPS: u32 = 2;
pub const CEPH_OSD_MAX_OPS: u32 = 16;

#[repr(C)]
pub enum ceph_osd_data_type { CEPH_OSD_DATA_TYPE_NONE = 0, CEPH_OSD_DATA_TYPE_PAGES, CEPH_OSD_DATA_TYPE_PAGELIST, CEPH_OSD_DATA_TYPE_BIO, CEPH_OSD_DATA_TYPE_BVECS, CEPH_OSD_DATA_TYPE_ITER }

#[repr(C)]
pub struct ceph_osd_data_pages { pub pages: *mut *mut page, pub length: u64, pub alignment: u32, pub pages_from_pool: bool, pub own_pages: bool }
#[repr(C)]
pub struct ceph_osd_data_bio { pub bio_pos: ceph_bio_iter, pub bio_length: u32 }
#[repr(C)]
pub struct ceph_osd_data_bvec { pub bvec_pos: ceph_bvec_iter, pub num_bvecs: u32 }
#[repr(C)]
pub union ceph_osd_data_union { pub pages: ceph_osd_data_pages, pub pagelist: *mut ceph_pagelist, pub bio: ceph_osd_data_bio, pub bvecs: ceph_osd_data_bvec, pub iter: iov_iter }
#[repr(C)]
pub struct ceph_osd_data { pub type_: ceph_osd_data_type, pub u: ceph_osd_data_union }

#[repr(C)]
pub struct ceph_osd_req_op {
    pub op: u16, pub flags: u32, pub indata_len: u32, pub outdata_len: u32, pub rval: i32,
    pub u: ceph_osd_req_op_union,
}
#[repr(C)] pub struct ceph_osd_req_op_extent { pub offset: u64, pub length: u64, pub truncate_size: u64, pub truncate_seq: u32, pub sparse_ext_cnt: i32, pub sparse_ext: *mut ceph_sparse_extent, pub osd_data: ceph_osd_data }
#[repr(C)] pub struct ceph_osd_req_op_xattr { pub name_len: u32, pub value_len: u32, pub cmp_op: u8, pub cmp_mode: u8, pub osd_data: ceph_osd_data }
#[repr(C)] pub struct ceph_osd_req_op_cls { pub class_name: *const c_char, pub method_name: *const c_char, pub request_info: ceph_osd_data, pub request_data: ceph_osd_data, pub response_data: ceph_osd_data, pub class_len: u8, pub method_len: u8, pub indata_len: u32 }
#[repr(C)] pub struct ceph_osd_req_op_watch { pub cookie: u64, pub op: u8, pub gen: u32 }
#[repr(C)] pub struct ceph_osd_req_op_notify { pub cookie: u64, pub request_data: ceph_osd_data, pub response_data: ceph_osd_data }
#[repr(C)] pub struct ceph_osd_req_op_alloc_hint { pub expected_object_size: u64, pub expected_write_size: u64, pub flags: u32 }
#[repr(C)] pub struct ceph_osd_req_op_copy_from { pub snapid: u64, pub src_version: u64, pub flags: u8, pub src_fadvise_flags: u32, pub osd_data: ceph_osd_data }
#[repr(C)] pub union ceph_osd_req_op_union { pub raw_data_in: ceph_osd_data, pub extent: ceph_osd_req_op_extent, pub xattr: ceph_osd_req_op_xattr, pub cls: ceph_osd_req_op_cls, pub watch: ceph_osd_req_op_watch, pub notify_ack: ceph_osd_data, pub notify: ceph_osd_req_op_notify, pub list_watchers: ceph_osd_data, pub alloc_hint: ceph_osd_req_op_alloc_hint, pub copy_from: ceph_osd_req_op_copy_from, pub assert_ver: u64 }

#[repr(C)]
pub struct ceph_osd_request_target { pub base_oid: ceph_object_id, pub base_oloc: ceph_object_locator, pub target_oid: ceph_object_id, pub target_oloc: ceph_object_locator, pub pgid: ceph_pg, pub spgid: ceph_spg, pub pg_num: u32, pub pg_num_mask: u32, pub acting: ceph_osds, pub up: ceph_osds, pub size: i32, pub min_size: i32, pub sort_bitwise: bool, pub recovery_deletes: bool, pub flags: c_uint, pub used_replica: bool, pub paused: bool, pub epoch: u32, pub last_force_resend: u32, pub osd: i32 }

#[repr(C)]
pub struct ceph_osd_request {
    pub r_tid: u64, pub r_node: rb_node, pub r_mc_node: rb_node, pub r_complete_work: work_struct, pub r_osd: *mut ceph_osd,
    pub r_t: ceph_osd_request_target, pub r_request: *mut ceph_msg, pub r_reply: *mut ceph_msg, pub r_sent: u32,
    pub r_num_ops: c_uint, pub r_result: i32, pub r_osdc: *mut ceph_osd_client, pub r_kref: kref, pub r_mempool: bool, pub r_linger: bool,
    pub r_completion: completion, pub r_callback: ceph_osdc_callback_t, pub r_inode: *mut inode, pub r_private_item: list_head, pub r_priv: *mut c_void,
    pub r_snapid: u64, pub r_snapc: *mut ceph_snap_context, pub r_mtime: timespec64, pub r_data_offset: u64,
    pub r_version: u64, pub r_stamp: c_ulong, pub r_start_stamp: c_ulong, pub r_start_latency: ktime_t, pub r_end_latency: ktime_t,
    pub r_attempts: i32, pub r_map_dne_bound: u32,
    pub r_ops: [ceph_osd_req_op; 0],
}

#[repr(C)] pub struct ceph_request_redirect { pub oloc: ceph_object_locator }
#[repr(C, packed)] pub struct ceph_osd_reqid { pub name: ceph_entity_name, pub tid: u64, pub inc: u32 }
#[repr(C, packed)] pub struct ceph_blkin_trace_info { pub trace_id: u64, pub span_id: u64, pub parent_span_id: u64 }
pub type rados_watchcb2_t = unsafe extern "C" fn(*mut c_void, u64, u64, u64, *mut c_void, usize);
pub type rados_watcherrcb_t = unsafe extern "C" fn(*mut c_void, u64, i32);

#[repr(C)] pub struct ceph_watch_item { pub name: ceph_entity_name, pub cookie: u64, pub addr: ceph_entity_addr }
#[repr(C)] pub struct ceph_spg_mapping { pub node: rb_node, pub spgid: ceph_spg, pub backoffs: rb_root }
#[repr(C)] pub struct ceph_hobject_id { pub key: *mut c_void, pub key_len: usize, pub oid: *mut c_void, pub oid_len: usize, pub snapid: u64, pub hash: u32, pub is_max: u8, pub nspace: *mut c_void, pub nspace_len: usize, pub pool: i64, pub hash_reverse_bits: u32 }
#[inline] pub unsafe fn ceph_hoid_build_hash_cache(hoid: *mut ceph_hobject_id) { (*hoid).hash_reverse_bits = (*hoid).hash.reverse_bits(); }
#[repr(C)] pub struct ceph_osd_backoff { pub spg_node: rb_node, pub id_node: rb_node, pub spgid: ceph_spg, pub id: u64, pub begin: *mut ceph_hobject_id, pub end: *mut ceph_hobject_id }
pub const CEPH_LINGER_ID_START: u64 = 0xffff000000000000;

#[repr(C)] pub struct ceph_osd_linger_request { pub osdc: *mut ceph_osd_client, pub linger_id: u64, pub committed: bool, pub is_watch: bool, pub osd: *mut ceph_osd, pub reg_req: *mut ceph_osd_request, pub ping_req: *mut ceph_osd_request, pub ping_sent: c_ulong, pub watch_valid_thru: c_ulong, pub pending_lworks: list_head, pub t: ceph_osd_request_target, pub map_dne_bound: u32, pub mtime: timespec64, pub kref: kref, pub lock: mutex, pub node: rb_node, pub osdc_node: rb_node, pub mc_node: rb_node, pub scan_item: list_head, pub reg_commit_wait: completion, pub notify_finish_wait: completion, pub reg_commit_error: i32, pub notify_finish_error: i32, pub last_error: i32, pub register_gen: u32, pub notify_id: u64, pub wcb: rados_watchcb2_t, pub errcb: rados_watcherrcb_t, pub data: *mut c_void, pub request_pl: *mut ceph_pagelist, pub notify_id_pages: *mut *mut page, pub preply_pages: *mut *mut *mut page, pub preply_len: *mut usize }

#[repr(C)] pub struct ceph_osd_client { pub client: *mut ceph_client, pub osdmap: *mut ceph_osdmap, pub lock: rw_semaphore, pub osds: rb_root, pub osd_lru: list_head, pub osd_lru_lock: spinlock_t, pub epoch_barrier: u32, pub homeless_osd: ceph_osd, pub last_tid: atomic64_t, pub last_linger_id: u64, pub linger_requests: rb_root, pub map_checks: rb_root, pub linger_map_checks: rb_root, pub num_requests: atomic_t, pub num_homeless: atomic_t, pub abort_err: i32, pub timeout_work: delayed_work, pub osds_timeout_work: delayed_work, pub req_mempool: *mut mempool_t, pub msgpool_op: ceph_msgpool, pub msgpool_op_reply: ceph_msgpool, pub notify_wq: *mut workqueue_struct, pub completion_wq: *mut workqueue_struct }

#[inline] pub unsafe fn ceph_osdmap_flag(osdc: *mut ceph_osd_client, flag: i32) -> bool { ((*osdc).osdmap).as_ref().unwrap().flags & flag != 0 }
pub const CEPH_SPARSE_EXT_ARRAY_INITIAL: i32 = 16;

extern "C" {
    pub fn ceph_osdc_setup() -> i32; pub fn ceph_osdc_cleanup(); pub fn ceph_osdc_init(osdc: *mut ceph_osd_client, client: *mut ceph_client) -> i32; pub fn ceph_osdc_stop(osdc: *mut ceph_osd_client); pub fn ceph_osdc_reopen_osds(osdc: *mut ceph_osd_client); pub fn ceph_osdc_handle_map(osdc: *mut ceph_osd_client, msg: *mut ceph_msg); pub fn ceph_osdc_update_epoch_barrier(osdc: *mut ceph_osd_client, eb: u32); pub fn ceph_osdc_abort_requests(osdc: *mut ceph_osd_client, err: i32); pub fn ceph_osdc_clear_abort_err(osdc: *mut ceph_osd_client);
    pub fn osd_req_op_init(req: *mut ceph_osd_request, which: c_uint, opcode: u16, flags: u32) -> *mut ceph_osd_req_op;
    pub fn __ceph_alloc_sparse_ext_map(op: *mut ceph_osd_req_op, cnt: i32) -> i32;
    pub fn osd_req_op_raw_data_in_pages(req: *mut ceph_osd_request, which: c_uint, pages: *mut *mut page, length: u64, alignment: u32, pages_from_pool: bool, own_pages: bool);
    pub fn osd_req_op_extent_init(req: *mut ceph_osd_request, which: c_uint, opcode: u16, offset: u64, length: u64, truncate_size: u64, truncate_seq: u32);
    pub fn osd_req_op_extent_update(req: *mut ceph_osd_request, which: c_uint, length: u64);
    pub fn osd_req_op_extent_dup_last(req: *mut ceph_osd_request, which: c_uint, offset_inc: u64);
    pub fn osd_req_op_extent_osd_data(req: *mut ceph_osd_request, which: c_uint) -> *mut ceph_osd_data;
    pub fn osd_req_op_extent_osd_data_pages(req: *mut ceph_osd_request, which: c_uint, pages: *mut *mut page, length: u64, alignment: u32, pages_from_pool: bool, own_pages: bool);
    pub fn osd_req_op_extent_osd_data_bvecs(req: *mut ceph_osd_request, which: c_uint, bvecs: *mut bio_vec, num_bvecs: u32, bytes: u32);
    pub fn osd_req_op_extent_osd_data_bvec_pos(req: *mut ceph_osd_request, which: c_uint, bvec_pos: *mut ceph_bvec_iter);
    pub fn osd_req_op_extent_osd_iter(req: *mut ceph_osd_request, which: c_uint, iter: *mut iov_iter);
    pub fn osd_req_op_cls_request_data_pages(req: *mut ceph_osd_request, which: c_uint, pages: *mut *mut page, length: u64, alignment: u32, pages_from_pool: bool, own_pages: bool);
    pub fn osd_req_op_cls_request_data_bvecs(req: *mut ceph_osd_request, which: c_uint, bvecs: *mut bio_vec, num_bvecs: u32, bytes: u32);
    pub fn osd_req_op_cls_response_data_pages(req: *mut ceph_osd_request, which: c_uint, pages: *mut *mut page, length: u64, alignment: u32, pages_from_pool: bool, own_pages: bool);
    pub fn osd_req_op_cls_init(req: *mut ceph_osd_request, which: c_uint, class_: *const c_char, method: *const c_char) -> i32;
    pub fn osd_req_op_xattr_init(req: *mut ceph_osd_request, which: c_uint, opcode: u16, name: *const c_char, value: *const c_void, size: usize, cmp_op: u8, cmp_mode: u8) -> i32;
    pub fn osd_req_op_alloc_hint_init(req: *mut ceph_osd_request, which: c_uint, expected_object_size: u64, expected_write_size: u64, flags: u32);
    pub fn osd_req_op_copy_from_init(req: *mut ceph_osd_request, src_snapid: u64, src_version: u64, src_oid: *mut ceph_object_id, src_oloc: *mut ceph_object_locator, src_fadvise_flags: u32, dst_fadvise_flags: u32, truncate_seq: u32, truncate_size: u64, copy_from_flags: u8) -> i32;
    pub fn ceph_osdc_alloc_request(osdc: *mut ceph_osd_client, snapc: *mut ceph_snap_context, num_ops: c_uint, use_mempool: bool, gfp_flags: gfp_t) -> *mut ceph_osd_request;
    pub fn ceph_osdc_alloc_messages(req: *mut ceph_osd_request, gfp: gfp_t) -> i32;
    pub fn ceph_osdc_get_request(req: *mut ceph_osd_request); pub fn ceph_osdc_put_request(req: *mut ceph_osd_request);
    pub fn ceph_osdc_start_request(osdc: *mut ceph_osd_client, req: *mut ceph_osd_request); pub fn ceph_osdc_cancel_request(req: *mut ceph_osd_request); pub fn ceph_osdc_wait_request(osdc: *mut ceph_osd_client, req: *mut ceph_osd_request) -> i32; pub fn ceph_osdc_sync(osdc: *mut ceph_osd_client);
}
#[inline] pub unsafe fn ceph_alloc_sparse_ext_map(op: *mut ceph_osd_req_op, mut cnt: i32) -> i32 { if cnt == 0 { cnt = CEPH_SPARSE_EXT_ARRAY_INITIAL; } __ceph_alloc_sparse_ext_map(op, cnt) }
#[inline] pub unsafe fn ceph_sparse_ext_map_end(op: *mut ceph_osd_req_op) -> u64 { if (*op).u.extent.sparse_ext_cnt == 0 { return 0; } let e = &*(*op).u.extent.sparse_ext.add((*op).u.extent.sparse_ext_cnt as usize - 1); e.off + e.len - (*op).u.extent.offset }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
