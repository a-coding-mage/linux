/* SPDX-License-Identifier: GPL-2.0 */
/* NFSv4 flexfile layout driver data structures. */

pub const FF_FLAGS_NO_LAYOUTCOMMIT: u32 = 1;
pub const FF_FLAGS_NO_IO_THRU_MDS: u32 = 2;
pub const FF_FLAGS_NO_READ_IO: u32 = 4;
pub const NFS4_FLEXFILE_LAYOUT_MAX_MIRROR_CNT: u32 = 4096;
pub const NFS4_FLEXFILE_LAYOUT_MAX_STRIPE_CNT: u32 = 4096;
pub const FF_LAYOUTSTATS_REPORT_INTERVAL: i64 = 60000;
pub const FF_LAYOUTSTATS_MAXDEV: usize = 4;

#[repr(C)]
pub struct nfs4_ff_ds_version { pub version: u32, pub minor_version: u32, pub rsize: u32, pub wsize: u32, pub tightly_coupled: bool }

#[repr(C)]
pub struct nfs4_ff_layout_ds { pub id_node: nfs4_deviceid_node, pub ds_versions_cnt: u32, pub ds_versions: *mut nfs4_ff_ds_version, pub ds: *mut nfs4_pnfs_ds }

#[repr(C)]
pub struct nfs4_ff_layout_ds_err { pub list: list_head, pub offset: u64, pub length: u64, pub status: i32, pub opnum: nfs_opnum4, pub stateid: nfs4_stateid, pub deviceid: nfs4_deviceid }

#[repr(C)]
pub struct nfs4_ff_io_stat { pub ops_requested: u64, pub bytes_requested: u64, pub ops_completed: u64, pub bytes_completed: u64, pub bytes_not_delivered: u64, pub total_busy_time: ktime_t, pub aggregate_completion_time: ktime_t }

#[repr(C)]
pub struct nfs4_ff_busy_timer { pub start_time: ktime_t, pub n_ops: atomic_t }

#[repr(C)]
pub struct nfs4_ff_layoutstat { pub io_stat: nfs4_ff_io_stat, pub busy_timer: nfs4_ff_busy_timer }

#[repr(C)]
pub struct nfs4_ff_layout_mirror;

#[repr(C)]
pub struct nfs4_ff_layout_ds_stripe {
    pub mirror: *mut nfs4_ff_layout_mirror, pub devid: nfs4_deviceid, pub efficiency: u32,
    pub mirror_ds: *mut nfs4_ff_layout_ds, pub fh_versions_cnt: u32, pub fh_versions: *mut nfs_fh,
    pub stateid: nfs4_stateid, pub ro_cred: *const cred, pub rw_cred: *const cred,
    pub nfl: nfs_file_localio, pub read_stat: nfs4_ff_layoutstat, pub write_stat: nfs4_ff_layoutstat,
    pub start_time: ktime_t,
}

#[repr(C)]
pub struct nfs4_ff_layout_mirror {
    pub layout: *mut pnfs_layout_hdr, pub mirrors: list_head, pub dss_count: u32,
    pub dss: *mut nfs4_ff_layout_ds_stripe, pub ref_: refcount_t, pub lock: spinlock_t,
    pub flags: c_ulong, pub report_interval: u32,
}

pub const NFS4_FF_MIRROR_STAT_AVAIL: u32 = 0;

#[repr(C)]
pub struct nfs4_ff_layout_segment {
    pub generic_hdr: pnfs_layout_segment, pub stripe_unit: u64, pub flags: u32,
    pub mirror_array_cnt: u32, pub mirror_array: [*mut nfs4_ff_layout_mirror; 0],
}

pub const NFS4_FF_HDR_NO_IO_THRU_MDS: usize = 0;

#[repr(C)]
pub struct nfs4_flexfile_layout {
    pub generic_hdr: pnfs_layout_hdr, pub commit_info: pnfs_ds_commit_info, pub mirrors: list_head,
    pub error_list: list_head, pub last_report_time: ktime_t, pub flags: c_ulong,
}

#[repr(C)]
pub struct nfs4_flexfile_layoutreturn_args {
    pub errors: list_head, pub devinfo: [nfs42_layoutstat_devinfo; FF_LAYOUTSTATS_MAXDEV],
    pub num_errors: c_uint, pub num_dev: c_uint, pub pages: [*mut page; 1],
}

#[inline]
pub unsafe fn FF_LAYOUT_FROM_HDR(lo: *mut pnfs_layout_hdr) -> *mut nfs4_flexfile_layout { container_of(lo, nfs4_flexfile_layout, generic_hdr) }
#[inline]
pub unsafe fn FF_LAYOUT_LSEG(lseg: *mut pnfs_layout_segment) -> *mut nfs4_ff_layout_segment { container_of(lseg, nfs4_ff_layout_segment, generic_hdr) }
#[inline]
pub unsafe fn FF_LAYOUT_MIRROR_DS(node: *mut nfs4_deviceid_node) -> *mut nfs4_ff_layout_ds { container_of(node, nfs4_ff_layout_ds, id_node) }

#[inline]
pub unsafe fn FF_LAYOUT_COMP(lseg: *mut pnfs_layout_segment, idx: u32) -> *mut nfs4_ff_layout_mirror {
    let fls = FF_LAYOUT_LSEG(lseg);
    if idx < (*fls).mirror_array_cnt { *(*fls).mirror_array.add(idx as usize) } else { core::ptr::null_mut() }
}
#[inline]
pub unsafe fn FF_LAYOUT_DEVID_NODE(lseg: *mut pnfs_layout_segment, idx: u32, dss_id: u32) -> *mut nfs4_deviceid_node {
    let mirror = FF_LAYOUT_COMP(lseg, idx);
    if !mirror.is_null() { let mirror_ds = (*(*mirror).dss.add(dss_id as usize)).mirror_ds; if !IS_ERR_OR_NULL(mirror_ds) { return &mut (*mirror_ds).id_node; } }
    core::ptr::null_mut()
}
#[inline] pub unsafe fn FF_LAYOUT_MIRROR_COUNT(lseg: *mut pnfs_layout_segment) -> u32 { (*FF_LAYOUT_LSEG(lseg)).mirror_array_cnt }
#[inline] pub unsafe fn ff_layout_no_fallback_to_mds(lseg: *mut pnfs_layout_segment) -> bool { (*FF_LAYOUT_LSEG(lseg)).flags & FF_FLAGS_NO_IO_THRU_MDS != 0 }
#[inline] pub unsafe fn ff_layout_hdr_no_fallback_to_mds(lo: *mut pnfs_layout_hdr) -> bool { test_bit(NFS4_FF_HDR_NO_IO_THRU_MDS, &(*FF_LAYOUT_FROM_HDR(lo)).flags) }
#[inline] pub unsafe fn ff_layout_no_read_on_rw(lseg: *mut pnfs_layout_segment) -> bool { (*FF_LAYOUT_LSEG(lseg)).flags & FF_FLAGS_NO_READ_IO != 0 }
#[inline] pub unsafe fn nfs4_ff_layout_ds_version(mirror: *const nfs4_ff_layout_mirror, dss_id: u32) -> i32 { (*(*(*mirror).dss.add(dss_id as usize)).mirror_ds).ds_versions.as_ref().unwrap().version as i32 }
#[inline] pub unsafe fn nfs4_ff_layout_calc_dss_id(stripe_unit: u64, dss_count: u32, offset: loff_t) -> u32 { let mut tmp = offset as u64; if dss_count == 1 || stripe_unit == 0 { return 0; } tmp /= stripe_unit; (tmp % dss_count as u64) as u32 }

extern "C" {
    pub fn nfs4_ff_alloc_deviceid_node(server: *mut nfs_server, pdev: *mut pnfs_device, gfp_flags: gfp_t) -> *mut nfs4_ff_layout_ds;
    pub fn nfs4_ff_layout_put_deviceid(mirror_ds: *mut nfs4_ff_layout_ds);
    pub fn nfs4_ff_layout_free_deviceid(mirror_ds: *mut nfs4_ff_layout_ds);
    pub fn ff_layout_track_ds_error(flo: *mut nfs4_flexfile_layout, mirror: *mut nfs4_ff_layout_mirror, dss_id: u32, offset: u64, length: u64, status: i32, opnum: nfs_opnum4, gfp_flags: gfp_t) -> i32;
    pub fn ff_layout_send_layouterror(lseg: *mut pnfs_layout_segment);
    pub fn ff_layout_encode_ds_ioerr(xdr: *mut xdr_stream, head: *const list_head) -> i32;
    pub fn ff_layout_free_ds_ioerr(head: *mut list_head);
    pub fn ff_layout_fetch_ds_ioerr(lo: *mut pnfs_layout_hdr, range: *const pnfs_layout_range, head: *mut list_head, maxnum: c_uint) -> c_uint;
    pub fn nfs4_ff_layout_select_ds_fh(mirror: *mut nfs4_ff_layout_mirror, dss_id: u32) -> *mut nfs_fh;
    pub fn nfs4_ff_layout_select_ds_stateid(mirror: *const nfs4_ff_layout_mirror, dss_id: u32, stateid: *mut nfs4_stateid);
    pub fn nfs4_ff_layout_prepare_ds(lseg: *mut pnfs_layout_segment, mirror: *mut nfs4_ff_layout_mirror, dss_id: u32, fail_return: bool) -> *mut nfs4_pnfs_ds;
    pub fn nfs4_ff_find_or_create_ds_client(mirror: *mut nfs4_ff_layout_mirror, ds_clp: *mut nfs_client, inode: *mut inode, dss_id: u32) -> *mut rpc_clnt;
    pub fn ff_layout_get_ds_cred(mirror: *mut nfs4_ff_layout_mirror, range: *const pnfs_layout_range, mdscred: *const cred, dss_id: u32) -> *const cred;
    pub fn ff_layout_avoid_mds_available_ds(lseg: *mut pnfs_layout_segment) -> bool;
    pub fn ff_layout_avoid_read_on_rw(lseg: *mut pnfs_layout_segment) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
