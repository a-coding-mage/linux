/* SPDX-License-Identifier: GPL-2.0 */
/*
 * (C) 2001 Clemson University and The University of Chicago
 *
 * See COPYING in top-level directory.
 */

/*
 * Sanitized this header file to fix
 * 32-64 bit interaction issues between
 * client-core and device
 */

#[repr(C)]
pub struct orangefs_io_request_s {
    pub __pad1: i32,
    pub buf_index: i32,
    pub count: i32,
    pub __pad2: i32,
    pub offset: i64,
    pub refn: orangefs_object_kref,
    pub io_type: ORANGEFS_io_type,
    pub readahead_size: i32,
}

#[repr(C)]
pub struct orangefs_lookup_request_s {
    pub sym_follow: i32,
    pub __pad1: i32,
    pub parent_refn: orangefs_object_kref,
    pub d_name: [i8; ORANGEFS_NAME_MAX],
}

#[repr(C)]
pub struct orangefs_create_request_s {
    pub parent_refn: orangefs_object_kref,
    pub attributes: ORANGEFS_sys_attr_s,
    pub d_name: [i8; ORANGEFS_NAME_MAX],
}

#[repr(C)]
pub struct orangefs_symlink_request_s {
    pub parent_refn: orangefs_object_kref,
    pub attributes: ORANGEFS_sys_attr_s,
    pub entry_name: [i8; ORANGEFS_NAME_MAX],
    pub target: [i8; ORANGEFS_NAME_MAX],
}

#[repr(C)]
pub struct orangefs_getattr_request_s {
    pub refn: orangefs_object_kref,
    pub mask: u32,
    pub __pad1: u32,
}

#[repr(C)]
pub struct orangefs_setattr_request_s {
    pub refn: orangefs_object_kref,
    pub attributes: ORANGEFS_sys_attr_s,
}

#[repr(C)]
pub struct orangefs_remove_request_s {
    pub parent_refn: orangefs_object_kref,
    pub d_name: [i8; ORANGEFS_NAME_MAX],
}

#[repr(C)]
pub struct orangefs_mkdir_request_s {
    pub parent_refn: orangefs_object_kref,
    pub attributes: ORANGEFS_sys_attr_s,
    pub d_name: [i8; ORANGEFS_NAME_MAX],
}

#[repr(C)]
pub struct orangefs_readdir_request_s {
    pub refn: orangefs_object_kref,
    pub token: u64,
    pub max_dirent_count: i32,
    pub buf_index: i32,
}

#[repr(C)]
pub struct orangefs_readdirplus_request_s {
    pub refn: orangefs_object_kref,
    pub token: u64,
    pub max_dirent_count: i32,
    pub mask: u32,
    pub buf_index: i32,
    pub __pad1: i32,
}

#[repr(C)]
pub struct orangefs_rename_request_s {
    pub old_parent_refn: orangefs_object_kref,
    pub new_parent_refn: orangefs_object_kref,
    pub d_old_name: [i8; ORANGEFS_NAME_MAX],
    pub d_new_name: [i8; ORANGEFS_NAME_MAX],
}

#[repr(C)]
pub struct orangefs_statfs_request_s { pub fs_id: i32, pub __pad1: i32 }

#[repr(C)]
pub struct orangefs_truncate_request_s { pub refn: orangefs_object_kref, pub size: i64 }

#[repr(C)]
pub struct orangefs_ra_cache_flush_request_s { pub refn: orangefs_object_kref }

#[repr(C)]
pub struct orangefs_fs_mount_request_s {
    pub orangefs_config_server: [i8; ORANGEFS_MAX_SERVER_ADDR_LEN],
}

#[repr(C)]
pub struct orangefs_fs_umount_request_s {
    pub id: i32,
    pub fs_id: i32,
    pub orangefs_config_server: [i8; ORANGEFS_MAX_SERVER_ADDR_LEN],
}

#[repr(C)]
pub struct orangefs_getxattr_request_s {
    pub refn: orangefs_object_kref,
    pub key_sz: i32,
    pub __pad1: i32,
    pub key: [i8; ORANGEFS_MAX_XATTR_NAMELEN],
}

#[repr(C)]
pub struct orangefs_setxattr_request_s {
    pub refn: orangefs_object_kref,
    pub keyval: ORANGEFS_keyval_pair,
    pub flags: i32,
    pub __pad1: i32,
}

#[repr(C)]
pub struct orangefs_listxattr_request_s {
    pub refn: orangefs_object_kref,
    pub requested_count: i32,
    pub __pad1: i32,
    pub token: u64,
}

#[repr(C)]
pub struct orangefs_removexattr_request_s {
    pub refn: orangefs_object_kref,
    pub key_sz: i32,
    pub __pad1: i32,
    pub key: [i8; ORANGEFS_MAX_XATTR_NAMELEN],
}

#[repr(C)]
pub struct orangefs_op_cancel_s { pub op_tag: u64 }

#[repr(C)]
pub struct orangefs_fsync_request_s { pub refn: orangefs_object_kref }

#[repr(C)]
pub enum orangefs_param_request_type {
    ORANGEFS_PARAM_REQUEST_SET = 1,
    ORANGEFS_PARAM_REQUEST_GET = 2,
}

#[repr(C)]
pub enum orangefs_param_request_op {
    ORANGEFS_PARAM_REQUEST_OP_ACACHE_TIMEOUT_MSECS = 1,
    ORANGEFS_PARAM_REQUEST_OP_ACACHE_HARD_LIMIT,
    ORANGEFS_PARAM_REQUEST_OP_ACACHE_SOFT_LIMIT,
    ORANGEFS_PARAM_REQUEST_OP_ACACHE_RECLAIM_PERCENTAGE,
    ORANGEFS_PARAM_REQUEST_OP_PERF_TIME_INTERVAL_SECS,
    ORANGEFS_PARAM_REQUEST_OP_PERF_HISTORY_SIZE,
    ORANGEFS_PARAM_REQUEST_OP_PERF_RESET,
    ORANGEFS_PARAM_REQUEST_OP_NCACHE_TIMEOUT_MSECS,
    ORANGEFS_PARAM_REQUEST_OP_NCACHE_HARD_LIMIT,
    ORANGEFS_PARAM_REQUEST_OP_NCACHE_SOFT_LIMIT,
    ORANGEFS_PARAM_REQUEST_OP_NCACHE_RECLAIM_PERCENTAGE,
    ORANGEFS_PARAM_REQUEST_OP_STATIC_ACACHE_TIMEOUT_MSECS,
    ORANGEFS_PARAM_REQUEST_OP_STATIC_ACACHE_HARD_LIMIT,
    ORANGEFS_PARAM_REQUEST_OP_STATIC_ACACHE_SOFT_LIMIT,
    ORANGEFS_PARAM_REQUEST_OP_STATIC_ACACHE_RECLAIM_PERCENTAGE,
    ORANGEFS_PARAM_REQUEST_OP_CLIENT_DEBUG,
    ORANGEFS_PARAM_REQUEST_OP_CCACHE_TIMEOUT_SECS,
    ORANGEFS_PARAM_REQUEST_OP_CCACHE_HARD_LIMIT,
    ORANGEFS_PARAM_REQUEST_OP_CCACHE_SOFT_LIMIT,
    ORANGEFS_PARAM_REQUEST_OP_CCACHE_RECLAIM_PERCENTAGE,
    ORANGEFS_PARAM_REQUEST_OP_CAPCACHE_TIMEOUT_SECS,
    ORANGEFS_PARAM_REQUEST_OP_CAPCACHE_HARD_LIMIT,
    ORANGEFS_PARAM_REQUEST_OP_CAPCACHE_SOFT_LIMIT,
    ORANGEFS_PARAM_REQUEST_OP_CAPCACHE_RECLAIM_PERCENTAGE,
    ORANGEFS_PARAM_REQUEST_OP_TWO_MASK_VALUES,
    ORANGEFS_PARAM_REQUEST_OP_READAHEAD_SIZE,
    ORANGEFS_PARAM_REQUEST_OP_READAHEAD_COUNT,
    ORANGEFS_PARAM_REQUEST_OP_READAHEAD_COUNT_SIZE,
    ORANGEFS_PARAM_REQUEST_OP_READAHEAD_READCNT,
}

#[repr(C)]
pub union orangefs_param_request_s_u { pub value64: i64, pub value32: [i32; 2] }

#[repr(C)]
pub struct orangefs_param_request_s {
    pub type_: orangefs_param_request_type,
    pub op: orangefs_param_request_op,
    pub u: orangefs_param_request_s_u,
    pub s_value: [i8; ORANGEFS_MAX_DEBUG_STRING_LEN],
}

#[repr(C)]
pub enum orangefs_perf_count_request_type {
    ORANGEFS_PERF_COUNT_REQUEST_ACACHE = 1,
    ORANGEFS_PERF_COUNT_REQUEST_NCACHE,
    ORANGEFS_PERF_COUNT_REQUEST_CAPCACHE,
}

#[repr(C)]
pub struct orangefs_perf_count_request_s {
    pub type_: orangefs_perf_count_request_type,
    pub __pad1: i32,
}

#[repr(C)]
pub struct orangefs_fs_key_request_s { pub fsid: i32, pub __pad1: i32 }

/* 2.9.6 */
#[repr(C)]
pub struct orangefs_features_request_s { pub features: u64 }

#[repr(C)]
pub union orangefs_upcall_s_req {
    pub io: orangefs_io_request_s,
    pub lookup: orangefs_lookup_request_s,
    pub create: orangefs_create_request_s,
    pub sym: orangefs_symlink_request_s,
    pub getattr: orangefs_getattr_request_s,
    pub setattr: orangefs_setattr_request_s,
    pub remove: orangefs_remove_request_s,
    pub mkdir: orangefs_mkdir_request_s,
    pub readdir: orangefs_readdir_request_s,
    pub readdirplus: orangefs_readdirplus_request_s,
    pub rename: orangefs_rename_request_s,
    pub statfs: orangefs_statfs_request_s,
    pub truncate: orangefs_truncate_request_s,
    pub ra_cache_flush: orangefs_ra_cache_flush_request_s,
    pub fs_mount: orangefs_fs_mount_request_s,
    pub fs_umount: orangefs_fs_umount_request_s,
    pub getxattr: orangefs_getxattr_request_s,
    pub setxattr: orangefs_setxattr_request_s,
    pub listxattr: orangefs_listxattr_request_s,
    pub removexattr: orangefs_removexattr_request_s,
    pub cancel: orangefs_op_cancel_s,
    pub fsync: orangefs_fsync_request_s,
    pub param: orangefs_param_request_s,
    pub perf_count: orangefs_perf_count_request_s,
    pub fs_key: orangefs_fs_key_request_s,
    pub features: orangefs_features_request_s,
}

#[repr(C)]
pub struct orangefs_upcall_s {
    pub type_: i32,
    pub uid: u32,
    pub gid: u32,
    pub pid: i32,
    pub tgid: i32,
    /* Trailers unused but must be retained for protocol compatibility. */
    pub trailer_size: i64,
    pub trailer_buf: *mut i8,
    pub req: orangefs_upcall_s_req,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
