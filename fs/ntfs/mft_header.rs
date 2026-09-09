/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Defines for mft record handling in NTFS Linux kernel driver.
 *
 * Copyright (c) 2001-2004 Anton Altaparmakov
 */

// Dependencies supplied by the surrounding NTFS/kernel translation.

extern "C" {
    pub fn map_mft_record(ni: *mut ntfs_inode) -> *mut mft_record;
    pub fn unmap_mft_record(ni: *mut ntfs_inode);
    pub fn map_extent_mft_record(
        base_ni: *mut ntfs_inode,
        mref: u64,
        ntfs_ino: *mut *mut ntfs_inode,
    ) -> *mut mft_record;

    pub fn __mark_mft_record_dirty(ni: *mut ntfs_inode);

    pub fn ntfs_sync_mft_mirror(
        vol: *mut ntfs_volume,
        mft_no: u64,
        m: *mut mft_record,
    ) -> i32;
    pub fn write_mft_record_nolock(
        ni: *mut ntfs_inode,
        m: *mut mft_record,
        sync: i32,
    ) -> i32;

    pub fn ntfs_mft_record_alloc(
        vol: *mut ntfs_volume,
        mode: i32,
        ni: *mut *mut ntfs_inode,
        base_ni: *mut ntfs_inode,
        ni_mrec: *mut *mut mft_record,
    ) -> i32;
    pub fn ntfs_mft_record_free(vol: *mut ntfs_volume, ni: *mut ntfs_inode) -> i32;
    pub fn ntfs_mft_records_write(
        vol: *const ntfs_volume,
        mref: u64,
        count: i64,
        b: *mut mft_record,
    ) -> i32;
    pub fn ntfs_mft_record_check(
        vol: *const ntfs_volume,
        m: *mut mft_record,
        mft_no: u64,
    ) -> i32;
    pub fn ntfs_mft_writepages(
        mapping: *mut address_space,
        wbc: *mut writeback_control,
    ) -> i32;
    pub fn ntfs_mft_mark_dirty(folio: *mut folio);
}

pub unsafe fn unmap_extent_mft_record(ni: *mut ntfs_inode) {
    unmap_mft_record(ni);
}

/*
 * mark_mft_record_dirty - set the mft record and the page containing it dirty
 * @ni:     ntfs inode describing the mapped mft record
 *
 * Set the mapped (extent) mft record of the (base or extent) ntfs inode @ni,
 * as well as the page containing the mft record, dirty.  Also, mark the base
 * vfs inode dirty.  This ensures that any changes to the mft record are
 * written out to disk.
 *
 * NOTE:  Do not do anything if the mft record is already marked dirty.
 */
pub unsafe fn mark_mft_record_dirty(ni: *mut ntfs_inode) {
    if !NInoTestSetDirty(ni) {
        __mark_mft_record_dirty(ni);
    }
}

/*
 * write_mft_record - write out a mapped (extent) mft record
 * @ni:     ntfs inode describing the mapped (extent) mft record
 * @m:      mapped (extent) mft record to write
 * @sync:   if true, wait for i/o completion
 *
 * This is just a wrapper for write_mft_record_nolock() (see mft.c), which
 * locks the page for the duration of the write.  This ensures that there are
 * no race conditions between writing the mft record via the dirty inode code
 * paths and via the page cache write back code paths or between writing
 * neighbouring mft records residing in the same page.
 *
 * Locking the page also serializes us against ->read_folio() if the page is not
 * uptodate.
 *
 * On success, clean the mft record and return 0.  On error, leave the mft
 * record dirty and return -errno.
 */
pub unsafe fn write_mft_record(
    ni: *mut ntfs_inode,
    m: *mut mft_record,
    sync: i32,
) -> i32 {
    let folio = (*ni).folio;
    let err: i32;

    folio_lock(folio);
    err = write_mft_record_nolock(ni, m, sync);
    folio_unlock(folio);

    err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
