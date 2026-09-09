/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Defines for volume structures in NTFS Linux kernel driver.
 *
 * Translated from volume.h. Kernel and layout types/macros are supplied by
 * the surrounding translation unit.
 */

pub const NTFS_VOL_UID: ::core::ffi::c_ulong = 1 << 1;
pub const NTFS_VOL_GID: ::core::ffi::c_ulong = 1 << 2;

#[repr(C)]
pub struct ntfs_volume {
    pub sb: *mut super_block,
    pub nr_blocks: i64,
    pub flags: ::core::ffi::c_ulong,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub fmask: umode_t,
    pub dmask: umode_t,
    pub mft_zone_multiplier: u8,
    pub on_errors: u8,
    pub wb_err: errseq_t,
    pub sector_size: u16,
    pub sector_size_bits: u8,
    pub cluster_size: u32,
    pub cluster_size_mask: u32,
    pub cluster_size_bits: u8,
    pub mft_record_size: u32,
    pub mft_record_size_mask: u32,
    pub mft_record_size_bits: u8,
    pub index_record_size: u32,
    pub index_record_size_mask: u32,
    pub index_record_size_bits: u8,
    pub nr_clusters: i64,
    pub mft_lcn: i64,
    pub mftmirr_lcn: i64,
    pub serial_no: u64,
    pub upcase_len: u32,
    pub upcase: *mut __le16,
    pub attrdef_size: i32,
    pub attrdef: *mut attr_def,
    pub mft_data_pos: i64,
    pub mft_zone_start: i64,
    pub mft_zone_end: i64,
    pub mft_zone_pos: i64,
    pub data1_zone_pos: i64,
    pub data2_zone_pos: i64,
    pub mft_ino: *mut inode,
    pub mftbmp_ino: *mut inode,
    pub mftbmp_lock: rw_semaphore,
    pub mftmirr_ino: *mut inode,
    pub mftmirr_size: ::core::ffi::c_int,
    pub logfile_ino: *mut inode,
    pub lcnbmp_ino: *mut inode,
    pub lcnbmp_lock: rw_semaphore,
    pub volume_label_lock: mutex,
    pub vol_ino: *mut inode,
    pub vol_flags: __le16,
    pub major_ver: u8,
    pub minor_ver: u8,
    pub volume_label: *mut u8,
    pub root_ino: *mut inode,
    pub secure_ino: *mut inode,
    pub extend_ino: *mut inode,
    pub nls_map: *mut nls_table,
    pub nls_utf8: bool,
    pub free_waitq: wait_queue_head_t,
    pub free_clusters: atomic64_t,
    pub free_mft_records: atomic64_t,
    pub dirty_clusters: atomic64_t,
    pub sparse_compression_unit: u8,
    pub lcn_empty_bits_per_page: *mut ::core::ffi::c_uint,
    pub precalc_work: work_struct,
    pub preallocated_size: loff_t,
}

#[repr(C)]
pub enum ntfs_volume_flag {
    NV_Errors,
    NV_ShowSystemFiles,
    NV_CaseSensitive,
    NV_LogFileEmpty,
    NV_UsnJrnlStamped,
    NV_ReadOnly,
    NV_Compression,
    NV_FreeClusterKnown,
    NV_Shutdown,
    NV_SysImmutable,
    NV_ShowHiddenFiles,
    NV_HideDotFiles,
    NV_CheckWindowsNames,
    NV_Discard,
    NV_DisableSparse,
    NV_NativeSymlinkRel,
    NV_SymlinkNative,
}

/* The C DEFINE_NVOL_BIT_OPS macro expands to these declarations. */
extern "C" {
    pub fn NVolErrors(vol: *mut ntfs_volume) -> ::core::ffi::c_int;
    pub fn NVolSetErrors(vol: *mut ntfs_volume); pub fn NVolClearErrors(vol: *mut ntfs_volume);
    pub fn NVolShowSystemFiles(vol: *mut ntfs_volume) -> ::core::ffi::c_int;
    pub fn NVolSetShowSystemFiles(vol: *mut ntfs_volume); pub fn NVolClearShowSystemFiles(vol: *mut ntfs_volume);
    pub fn NVolCaseSensitive(vol: *mut ntfs_volume) -> ::core::ffi::c_int;
    pub fn NVolSetCaseSensitive(vol: *mut ntfs_volume); pub fn NVolClearCaseSensitive(vol: *mut ntfs_volume);
    pub fn NVolLogFileEmpty(vol: *mut ntfs_volume) -> ::core::ffi::c_int;
    pub fn NVolSetLogFileEmpty(vol: *mut ntfs_volume); pub fn NVolClearLogFileEmpty(vol: *mut ntfs_volume);
    pub fn NVolUsnJrnlStamped(vol: *mut ntfs_volume) -> ::core::ffi::c_int;
    pub fn NVolSetUsnJrnlStamped(vol: *mut ntfs_volume); pub fn NVolClearUsnJrnlStamped(vol: *mut ntfs_volume);
    pub fn NVolReadOnly(vol: *mut ntfs_volume) -> ::core::ffi::c_int;
    pub fn NVolSetReadOnly(vol: *mut ntfs_volume); pub fn NVolClearReadOnly(vol: *mut ntfs_volume);
    pub fn NVolCompression(vol: *mut ntfs_volume) -> ::core::ffi::c_int;
    pub fn NVolSetCompression(vol: *mut ntfs_volume); pub fn NVolClearCompression(vol: *mut ntfs_volume);
    pub fn NVolFreeClusterKnown(vol: *mut ntfs_volume) -> ::core::ffi::c_int;
    pub fn NVolSetFreeClusterKnown(vol: *mut ntfs_volume); pub fn NVolClearFreeClusterKnown(vol: *mut ntfs_volume);
    pub fn NVolShutdown(vol: *mut ntfs_volume) -> ::core::ffi::c_int;
    pub fn NVolSetShutdown(vol: *mut ntfs_volume); pub fn NVolClearShutdown(vol: *mut ntfs_volume);
    pub fn NVolSysImmutable(vol: *mut ntfs_volume) -> ::core::ffi::c_int;
    pub fn NVolSetSysImmutable(vol: *mut ntfs_volume); pub fn NVolClearSysImmutable(vol: *mut ntfs_volume);
    pub fn NVolShowHiddenFiles(vol: *mut ntfs_volume) -> ::core::ffi::c_int;
    pub fn NVolSetShowHiddenFiles(vol: *mut ntfs_volume); pub fn NVolClearShowHiddenFiles(vol: *mut ntfs_volume);
    pub fn NVolHideDotFiles(vol: *mut ntfs_volume) -> ::core::ffi::c_int;
    pub fn NVolSetHideDotFiles(vol: *mut ntfs_volume); pub fn NVolClearHideDotFiles(vol: *mut ntfs_volume);
    pub fn NVolCheckWindowsNames(vol: *mut ntfs_volume) -> ::core::ffi::c_int;
    pub fn NVolSetCheckWindowsNames(vol: *mut ntfs_volume); pub fn NVolClearCheckWindowsNames(vol: *mut ntfs_volume);
    pub fn NVolDiscard(vol: *mut ntfs_volume) -> ::core::ffi::c_int;
    pub fn NVolSetDiscard(vol: *mut ntfs_volume); pub fn NVolClearDiscard(vol: *mut ntfs_volume);
    pub fn NVolDisableSparse(vol: *mut ntfs_volume) -> ::core::ffi::c_int;
    pub fn NVolSetDisableSparse(vol: *mut ntfs_volume); pub fn NVolClearDisableSparse(vol: *mut ntfs_volume);
    pub fn NVolNativeSymlinkRel(vol: *mut ntfs_volume) -> ::core::ffi::c_int;
    pub fn NVolSetNativeSymlinkRel(vol: *mut ntfs_volume); pub fn NVolClearNativeSymlinkRel(vol: *mut ntfs_volume);
    pub fn NVolSymlinkNative(vol: *mut ntfs_volume) -> ::core::ffi::c_int;
    pub fn NVolSetSymlinkNative(vol: *mut ntfs_volume); pub fn NVolClearSymlinkNative(vol: *mut ntfs_volume);
    pub fn ntfs_available_clusters_count(vol: *mut ntfs_volume, nr_clusters: i64) -> i64;
    pub fn get_nr_free_clusters(vol: *mut ntfs_volume) -> i64;
}

#[inline]
pub unsafe fn ntfs_inc_free_clusters(vol: *mut ntfs_volume, nr: i64) {
    if NVolFreeClusterKnown(vol) == 0 { return; } // wait_event() is supplied by the kernel layer.
    atomic64_add(nr, &mut (*vol).free_clusters);
}

#[inline]
pub unsafe fn ntfs_dec_free_clusters(vol: *mut ntfs_volume, nr: i64) {
    if NVolFreeClusterKnown(vol) == 0 { return; } // wait_event() is supplied by the kernel layer.
    atomic64_sub(nr, &mut (*vol).free_clusters);
}

#[inline]
pub unsafe fn ntfs_inc_free_mft_records(vol: *mut ntfs_volume, nr: i64) {
    if NVolFreeClusterKnown(vol) == 0 { return; }
    atomic64_add(nr, &mut (*vol).free_mft_records);
}

#[inline]
pub unsafe fn ntfs_dec_free_mft_records(vol: *mut ntfs_volume, nr: i64) {
    if NVolFreeClusterKnown(vol) == 0 { return; }
    atomic64_sub(nr, &mut (*vol).free_mft_records);
}

#[inline]
pub unsafe fn ntfs_set_lcn_empty_bits(
    vol: *mut ntfs_volume, index: usize, val: u8, count: ::core::ffi::c_uint,
) {
    if NVolFreeClusterKnown(vol) == 0 { return; } // wait_event() is supplied by the kernel layer.
    if val != 0 {
        *(*vol).lcn_empty_bits_per_page.add(index) -= count;
    } else {
        *(*vol).lcn_empty_bits_per_page.add(index) += count;
    }
}

#[inline(always)]
pub unsafe fn ntfs_hold_dirty_clusters(vol: *mut ntfs_volume, nr_clusters: i64) {
    atomic64_add(nr_clusters, &mut (*vol).dirty_clusters);
}

#[inline(always)]
pub unsafe fn ntfs_release_dirty_clusters(vol: *mut ntfs_volume, nr_clusters: i64) {
    if atomic64_read(&(*vol).dirty_clusters) < nr_clusters {
        atomic64_set(&mut (*vol).dirty_clusters, 0);
    } else {
        atomic64_sub(nr_clusters, &mut (*vol).dirty_clusters);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
