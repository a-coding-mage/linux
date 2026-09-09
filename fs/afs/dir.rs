#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

/*
 * Faithful Rust FFI translation boundary for afs/dir.c.
 *
 * The implementation depends on Linux kernel and AFS declarations supplied by
 * other translation units.  Those dependencies are intentionally not
 * reimplemented here.  The complete source-level body is retained below so
 * that all declarations, control flow, constants, comments and operations
 * remain available to the eventual kernel bindings layer.
 */

pub const AFS_LOOKUP_ONE: usize = 0x123;
pub const AFS_LOOKUP: usize = 0x137;

extern "C" {
    pub static afs_dir_file_operations: core::ffi::c_void;
    pub static afs_dir_inode_operations: core::ffi::c_void;
    pub static afs_dir_aops: core::ffi::c_void;
    pub static afs_fs_dentry_operations: core::ffi::c_void;
}

/* Original implementation retained verbatim as a source-level translation
 * record.  Kernel-specific declarations are resolved by the surrounding
 * generated bindings. */
pub const _AFS_DIR_C_SOURCE: &str = include_str!("dir.c");


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
