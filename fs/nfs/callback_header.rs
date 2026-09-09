/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/fs/nfs/callback.h
 *
 * Copyright (C) 2004 Trond Myklebust
 *
 * NFSv4 callback definitions
 */

// Dependency declarations from <linux/sunrpc/svc.h> and related headers are
// supplied by other translated files.

pub const NFS4_CALLBACK: u32 = 0x40000000;
pub const NFS4_CALLBACK_XDRSIZE: u32 = 2048;
pub const NFS4_CALLBACK_BUFSIZE: u32 = 1024 + NFS4_CALLBACK_XDRSIZE;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum nfs4_callback_procnum {
    CB_NULL = 0,
    CB_COMPOUND = 1,
}

#[repr(C)]
pub struct nfs4_slot;

#[repr(C)]
pub struct cb_process_state {
    pub clp: *mut nfs_client,
    pub slot: *mut nfs4_slot,
    pub net: *mut net,
    pub minorversion: u32,
    pub drc_status: u32,
    pub referring_calls: libc::c_uint,
}

#[repr(C)]
pub struct cb_compound_hdr_arg {
    pub taglen: libc::c_uint,
    pub tag: *const libc::c_char,
    pub minorversion: libc::c_uint,
    pub cb_ident: libc::c_uint,
    pub nops: libc::c_uint,
}

#[repr(C)]
pub struct cb_compound_hdr_res {
    pub status: *mut u32,
    pub taglen: libc::c_uint,
    pub tag: *const libc::c_char,
    pub nops: *mut u32,
}

#[repr(C)]
pub struct cb_getattrargs {
    pub fh: nfs_fh,
    pub bitmap: [u32; 3],
}

#[repr(C)]
pub struct cb_getattrres {
    pub status: u32,
    pub bitmap: [u32; 3],
    pub size: u64,
    pub change_attr: u64,
    pub atime: timespec64,
    pub ctime: timespec64,
    pub mtime: timespec64,
}

#[repr(C)]
pub struct cb_recallargs {
    pub fh: nfs_fh,
    pub stateid: nfs4_stateid,
    pub truncate: u32,
}

#[repr(C)]
pub struct referring_call {
    pub rc_sequenceid: u32,
    pub rc_slotid: u32,
}

#[repr(C)]
pub struct referring_call_list {
    pub rcl_sessionid: nfs4_sessionid,
    pub rcl_nrefcalls: u32,
    pub rcl_refcalls: *mut referring_call,
}

#[repr(C)]
pub struct cb_sequenceargs {
    pub csa_addr: *mut sockaddr,
    pub csa_sessionid: nfs4_sessionid,
    pub csa_sequenceid: u32,
    pub csa_slotid: u32,
    pub csa_highestslotid: u32,
    pub csa_cachethis: u32,
    pub csa_nrclists: u32,
    pub csa_rclists: *mut referring_call_list,
}

#[repr(C)]
pub struct cb_sequenceres {
    pub csr_status: u32,
    pub csr_sessionid: nfs4_sessionid,
    pub csr_sequenceid: u32,
    pub csr_slotid: u32,
    pub csr_highestslotid: u32,
    pub csr_target_highestslotid: u32,
}

extern "C" {
    pub fn nfs4_callback_sequence(
        argp: *mut libc::c_void,
        resp: *mut libc::c_void,
        cps: *mut cb_process_state,
    ) -> u32;
}

pub const RCA4_TYPE_MASK_RDATA_DLG: u32 = 0;
pub const RCA4_TYPE_MASK_WDATA_DLG: u32 = 1;
pub const RCA4_TYPE_MASK_DIR_DLG: u32 = 2;
pub const RCA4_TYPE_MASK_FILE_LAYOUT: u32 = 3;
pub const RCA4_TYPE_MASK_BLK_LAYOUT: u32 = 4;
pub const RCA4_TYPE_MASK_OBJ_LAYOUT_MIN: u32 = 8;
pub const RCA4_TYPE_MASK_OBJ_LAYOUT_MAX: u32 = 9;
pub const RCA4_TYPE_MASK_OTHER_LAYOUT_MIN: u32 = 12;
pub const RCA4_TYPE_MASK_OTHER_LAYOUT_MAX: u32 = 15;
pub const PNFS_FF_RCA4_TYPE_MASK_READ: u32 = 16;
pub const PNFS_FF_RCA4_TYPE_MASK_RW: u32 = 17;
pub const RCA4_TYPE_MASK_ALL: u32 = 0x3f31f;

#[repr(C)]
pub struct cb_recallanyargs {
    pub craa_objs_to_keep: u32,
    pub craa_type_mask: u32,
}

#[repr(C)]
pub struct cb_recallslotargs {
    pub crsa_target_highest_slotid: u32,
}

#[repr(C)]
pub struct cb_layoutrecallargs_layout {
    pub cbl_fh: nfs_fh,
    pub cbl_range: pnfs_layout_range,
    pub cbl_stateid: nfs4_stateid,
}

#[repr(C)]
pub union cb_layoutrecallargs_union {
    pub layout: cb_layoutrecallargs_layout,
    pub cbl_fsid: nfs_fsid,
}

#[repr(C)]
pub struct cb_layoutrecallargs {
    pub cbl_recall_type: u32,
    pub cbl_layout_type: u32,
    pub cbl_layoutchanged: u32,
    pub __bindgen_anon_1: cb_layoutrecallargs_union,
}

#[repr(C)]
pub struct cb_devicenotifyitem {
    pub cbd_notify_type: u32,
    pub cbd_layout_type: u32,
    pub cbd_dev_id: nfs4_deviceid,
    pub cbd_immediate: u32,
}

#[repr(C)]
pub struct cb_devicenotifyargs {
    pub ndevs: u32,
    pub devs: *mut cb_devicenotifyitem,
}

#[repr(C)]
pub struct cb_notify_lock_args {
    pub cbnl_fh: nfs_fh,
    pub cbnl_owner: nfs_lowner,
    pub cbnl_valid: bool,
}

#[cfg(feature = "CONFIG_NFS_V4_2")]
#[repr(C)]
pub struct cb_offloadargs {
    pub coa_fh: nfs_fh,
    pub coa_stateid: nfs4_stateid,
    pub error: u32,
    pub wr_count: u64,
    pub wr_writeverf: nfs_writeverf,
}

extern "C" {
    pub fn check_gss_callback_principal(clp: *mut nfs_client, rqst: *mut svc_rqst) -> libc::c_int;
    pub fn nfs4_callback_getattr(argp: *mut libc::c_void, resp: *mut libc::c_void, cps: *mut cb_process_state) -> u32;
    pub fn nfs4_callback_recall(argp: *mut libc::c_void, resp: *mut libc::c_void, cps: *mut cb_process_state) -> u32;
}

// Preserves the source condition: IS_ENABLED(CONFIG_NFS_V4).
#[cfg(feature = "CONFIG_NFS_V4")]
extern "C" {
    pub fn nfs_callback_up(minorversion: u32, xprt: *mut rpc_xprt) -> libc::c_int;
    pub fn nfs_callback_down(minorversion: libc::c_int, net: *mut net, xprt: *mut rpc_xprt);
}

pub const NFS41_BC_MIN_CALLBACKS: u32 = 1;
pub const NFS41_BC_MAX_CALLBACKS: u32 = 1;
pub const NFS4_MIN_NR_CALLBACK_THREADS: u32 = 1;

extern "C" {
    pub static mut nfs_callback_set_tcpport: libc::c_uint;
    pub static mut nfs_callback_nr_threads: libc::c_ushort;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
