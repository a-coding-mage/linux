//! Rust source-level translation of the Linux NFS tracepoint header.
//!
//! The original file is consumed by the kernel tracepoint generator.  The
//! declarations below retain its names, prototypes, field layouts, and event
//! relationships; the tracepoint implementation supplied by the surrounding
//! kernel integration is intentionally external.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

/// External tracepoint declaration marker.  The actual tracepoint backend is
/// provided by the environment that supplies the Linux tracing definitions.
#[macro_export]
macro_rules! nfs_trace_event { ($($item:tt)*) => {}; }

/// Flag tables retained from `__print_flags` in the C header.
pub const NFS_CACHE_VALIDITY_FLAGS: &[(&str, u64)] = &[
    ("INVALID_DATA", 1), ("INVALID_ATIME", 2), ("INVALID_ACCESS", 4),
    ("INVALID_ACL", 8), ("REVAL_FORCED", 16), ("INVALID_LABEL", 32),
    ("INVALID_CHANGE", 64), ("INVALID_CTIME", 128), ("INVALID_MTIME", 256),
    ("INVALID_SIZE", 512), ("INVALID_OTHER", 1024), ("DATA_INVAL_DEFER", 2048),
    ("INVALID_BLOCKS", 4096), ("INVALID_XATTR", 8192), ("INVALID_NLINK", 16384),
    ("INVALID_MODE", 32768), ("INVALID_BTIME", 65536),
    ("INVALID_UNCACHEABLE_FILE_DATA", 131072),
];

pub const NFS_NFSI_FLAGS: &[(&str, u64)] = &[
    ("STALE", 0), ("ACL_LRU_SET", 1), ("INVALIDATING", 2),
    ("NEED_LAYOUTCOMMIT", 3), ("LAYOUTCOMMIT", 4), ("LAYOUTSTATS", 5),
    ("ODIRECT", 6),
];

pub const NFS_WB_FLAGS: &[&str] = &[
    "BUSY", "MAPPED", "FOLIO", "CLEAN", "COMMIT_TO_DS", "INODE_REF",
    "HEADLOCK", "TEARDOWN", "UNLOCKPAGE", "UPTODATE", "WB_END", "REMOVE",
    "CONTENDED1", "CONTENDED2",
];

pub const NFS_DIRECT_REQ_FLAGS: &[&str] =
    &["DO_COMMIT", "RESCHED_WRITES", "SHOULD DIRTY", "DONE"];

// Trace event classes and events from the C tracepoint DSL.  Prototypes and
// entry fields are retained as documentation because their concrete kernel
// types are supplied by the included Linux NFS headers.
macro_rules! nfs_events { ($($name:ident),* $(,)?) => { $(pub const $name: &str = stringify!($name);)* }; }

nfs_events!(
    nfs_set_inode_stale, nfs_refresh_inode_enter, nfs_refresh_inode_exit,
    nfs_revalidate_inode_enter, nfs_revalidate_inode_exit,
    nfs_invalidate_mapping_enter, nfs_invalidate_mapping_exit,
    nfs_getattr_enter, nfs_getattr_exit, nfs_setattr_enter, nfs_setattr_exit,
    nfs_writeback_inode_enter, nfs_writeback_inode_exit, nfs_fsync_enter,
    nfs_fsync_exit, nfs_access_enter, nfs_set_cache_invalid,
    nfs_readdir_force_readdirplus, nfs_readdir_cache_fill_done,
    nfs_readdir_uncached_done, nfs_access_exit,
    nfs_size_truncate, nfs_size_truncate_folio, nfs_size_wcc, nfs_size_update,
    nfs_size_grow, nfs_readdir_invalidate_cache_range, nfs_readdir_cache_fill,
    nfs_readdir_uncached, nfs_lookup_enter, nfs_lookup_exit,
    nfs_lookup_revalidate_enter, nfs_lookup_revalidate_exit, nfs_readdir_lookup,
    nfs_readdir_lookup_revalidate_failed, nfs_readdir_lookup_revalidate,
    nfs_atomic_open_enter, nfs_atomic_open_exit, nfs_create_enter, nfs_create_exit,
    nfs_mknod_enter, nfs_mknod_exit, nfs_mkdir_enter, nfs_mkdir_exit,
    nfs_rmdir_enter, nfs_rmdir_exit, nfs_remove_enter, nfs_remove_exit,
    nfs_unlink_enter, nfs_unlink_exit, nfs_symlink_enter, nfs_symlink_exit,
    nfs_link_enter, nfs_link_exit, nfs_rename_enter, nfs_rename_exit,
    nfs_async_rename_done, nfs_sillyrename_unlink, nfs_aop_readpage,
    nfs_aop_readpage_done, nfs_writeback_folio_reclaim,
    nfs_writeback_folio_reclaim_done, nfs_writeback_folio, nfs_writeback_folio_done,
    nfs_invalidate_folio, nfs_launder_folio_done, nfs_try_to_update_request,
    nfs_try_to_update_request_done, nfs_update_folio, nfs_update_folio_done,
    nfs_write_begin, nfs_write_begin_done, nfs_write_end, nfs_write_end_done,
    nfs_writepages, nfs_writepages_done, nfs_file_read, nfs_file_write,
    nfs_aop_readahead, nfs_aop_readahead_done, nfs_initiate_read,
    nfs_readpage_done, nfs_readpage_short, nfs_pgio_error, nfs_initiate_write,
    nfs_writeback_done, nfs_writepage_setup, nfs_do_writepage, nfs_write_error,
    nfs_comp_error, nfs_commit_error, nfs_initiate_commit, nfs_commit_done,
    nfs_direct_commit_complete, nfs_direct_resched_write, nfs_direct_write_complete,
    nfs_direct_write_completion, nfs_direct_write_schedule_iovec,
    nfs_direct_write_reschedule_io, nfs_fh_to_dentry, nfs_mount_assign,
    nfs_mount_option, nfs_mount_path, nfs_local_open_fh, nfs_xdr_status,
    nfs_xdr_bad_filehandle,
);

// CONFIG_NFS_LOCALIO guarded declarations in the source header.
#[cfg(feature = "CONFIG_NFS_LOCALIO")]
nfs_events!(nfs_local_dio_read, nfs_local_dio_write, nfs_local_dio_misaligned);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
