// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

// kernel only EFI/EFD definitions

// Forward declarations supplied by other translated files.
pub struct xfs_mount;
pub struct kmem_cache;

/*
 * Max number of extents in fast allocation path.
 */
pub const XFS_EFI_MAX_FAST_EXTENTS: u32 = 16;

/*
 * This is the "extent free intention" log item.  It is used to log the fact
 * that some extents need to be free.  It is used in conjunction with the
 * "extent free done" log item described below.
 *
 * The EFI is reference counted so that it is not freed prior to both the EFI
 * and EFD being committed and unpinned. This ensures the EFI is inserted into
 * the AIL even in the event of out of order EFI/EFD processing. In other words,
 * an EFI is born with two references:
 *
 * 	1.) an EFI held reference to track EFI AIL insertion
 * 	2.) an EFD held reference to track EFD commit
 *
 * On allocation, both references are the responsibility of the caller. Once the
 * EFI is added to and dirtied in a transaction, ownership of reference one
 * transfers to the transaction. The reference is dropped once the EFI is
 * inserted to the AIL or in the event of failure along the way (e.g., commit
 * failure, log I/O error, etc.). Note that the caller remains responsible for
 * the EFD reference under all circumstances to this point. The caller has no
 * means to detect failure once the transaction is committed, however.
 * Therefore, an EFD is required after this point, even in the event of
 * unrelated failure.
 *
 * Once an EFD is allocated and dirtied in a transaction, reference two
 * transfers to the transaction. The EFD reference is dropped once it reaches
 * the unpin handler. Similar to the EFI, the reference also drops in the event
 * of commit failure or log I/O errors. Note that the EFD is not inserted in the
 * AIL, so at this point both the EFI and EFD are freed.
 */
#[repr(C)]
pub struct xfs_efi_log_item {
    pub efi_item: xfs_log_item,
    pub efi_refcount: atomic_t,
    pub efi_next_extent: atomic_t,
    pub efi_format: xfs_efi_log_format,
}

pub unsafe fn xfs_efi_log_item_sizeof(nr: u32) -> usize {
    core::mem::offset_of!(xfs_efi_log_item, efi_format) + xfs_efi_log_format_sizeof(nr)
}

/*
 * This is the "extent free done" log item.  It is used to log
 * the fact that some extents earlier mentioned in an efi item
 * have been freed.
 */
#[repr(C)]
pub struct xfs_efd_log_item {
    pub efd_item: xfs_log_item,
    pub efd_efip: *mut xfs_efi_log_item,
    pub efd_next_extent: u32,
    pub efd_format: xfs_efd_log_format,
}

pub unsafe fn xfs_efd_log_item_sizeof(nr: u32) -> usize {
    core::mem::offset_of!(xfs_efd_log_item, efd_format) + xfs_efd_log_format_sizeof(nr)
}

/*
 * Max number of extents in fast allocation path.
 */
pub const XFS_EFD_MAX_FAST_EXTENTS: u32 = 16;

pub struct xfs_extent_free_item;

unsafe extern "C" {
    pub static mut xfs_efi_cache: *mut kmem_cache;
    pub static mut xfs_efd_cache: *mut kmem_cache;

    pub fn xfs_extent_free_defer_add(
        tp: *mut xfs_trans,
        xefi: *mut xfs_extent_free_item,
        dfpp: *mut *mut xfs_defer_pending,
    );

    pub fn xfs_efi_log_space(nr: u32) -> u32;
    pub fn xfs_efd_log_space(nr: u32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
