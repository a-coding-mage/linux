/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency declarations supplied by the surrounding translation unit. */
pub type uint = u32;

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct exportfs_block_ops {
    _private: [u8; 0],
}

/* CONFIG_EXPORTFS_BLOCK_OPS */
#[cfg(feature = "CONFIG_EXPORTFS_BLOCK_OPS")]
extern "C" {
    pub fn xfs_break_leased_layouts(
        inode: *mut inode,
        iolock: *mut uint,
        did_unlock: *mut bool,
    ) -> i32;
}

#[cfg(not(feature = "CONFIG_EXPORTFS_BLOCK_OPS"))]
#[inline]
pub unsafe fn xfs_break_leased_layouts(
    _inode: *mut inode,
    _iolock: *mut uint,
    _did_unlock: *mut bool,
) -> i32 {
    0
}

extern "C" {
    pub static xfs_export_block_ops: exportfs_block_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
