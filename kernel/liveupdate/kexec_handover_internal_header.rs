/* SPDX-License-Identifier: GPL-2.0 */

// Dependency declarations supplied by the corresponding kernel headers are
// intentionally left external to this translation.

#[cfg(feature = "CONFIG_KEXEC_HANDOVER_DEBUGFS")]
#[repr(C)]
pub struct kho_debugfs {
    pub dir: *mut dentry,
    pub sub_fdt_dir: *mut dentry,
    pub fdt_list: list_head,
}

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER_DEBUGFS"))]
#[repr(C)]
pub struct kho_debugfs {}

extern "C" {
    pub static mut kho_scratch: *mut kho_scratch;
    pub static mut kho_scratch_cnt: core::ffi::c_uint;
}

#[cfg(feature = "CONFIG_KEXEC_HANDOVER_DEBUGFS")]
extern "C" {
    pub fn kho_debugfs_init() -> core::ffi::c_int;
    pub fn kho_in_debugfs_init(dbg: *mut kho_debugfs, fdt: *const core::ffi::c_void);
    pub fn kho_out_debugfs_init(dbg: *mut kho_debugfs) -> core::ffi::c_int;
    pub fn kho_debugfs_blob_add(
        dbg: *mut kho_debugfs,
        name: *const core::ffi::c_char,
        blob: *const core::ffi::c_void,
        size: usize,
        root: bool,
    ) -> core::ffi::c_int;
    pub fn kho_debugfs_blob_remove(dbg: *mut kho_debugfs, blob: *mut core::ffi::c_void);
}

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER_DEBUGFS"))]
#[inline]
pub fn kho_debugfs_init() -> core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER_DEBUGFS"))]
#[inline]
pub fn kho_in_debugfs_init(_dbg: *mut kho_debugfs, _fdt: *const core::ffi::c_void) {}

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER_DEBUGFS"))]
#[inline]
pub fn kho_out_debugfs_init(_dbg: *mut kho_debugfs) -> core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER_DEBUGFS"))]
#[inline]
pub fn kho_debugfs_blob_add(
    _dbg: *mut kho_debugfs,
    _name: *const core::ffi::c_char,
    _blob: *const core::ffi::c_void,
    _size: usize,
    _root: bool,
) -> core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_KEXEC_HANDOVER_DEBUGFS"))]
#[inline]
pub fn kho_debugfs_blob_remove(_dbg: *mut kho_debugfs, _blob: *mut core::ffi::c_void) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
