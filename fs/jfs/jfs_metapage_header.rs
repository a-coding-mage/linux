/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) International Business Machines Corp., 2000-2002
 *   Portions Copyright (C) Christoph Hellwig, 2001-2002
 */

// Dependency supplied by the surrounding kernel/JFS translation.

#[repr(C)]
pub struct metapage {
    /* Common logsyncblk prefix (see jfs_logmgr.h) */
    pub xflag: u16,
    pub unused: u16,
    pub lid: lid_t,
    pub lsn: ::core::ffi::c_int,
    pub synclist: list_head,
    /* End of logsyncblk prefix */

    pub flag: ::core::ffi::c_ulong, /* See Below */
    pub count: ::core::ffi::c_ulong, /* Reference count */
    pub data: *mut ::core::ffi::c_void, /* Data pointer */
    pub index: sector_t, /* block address of page */
    pub wait: wait_queue_head_t,

    /* implementation */
    pub folio: *mut folio,
    pub sb: *mut super_block,
    pub logical_size: ::core::ffi::c_uint,

    /* Journal management */
    pub clsn: ::core::ffi::c_int,
    pub nohomeok: ::core::ffi::c_int,
    pub log: *mut jfs_log,
}

/* metapage flag */
pub const META_locked: usize = 0;
pub const META_dirty: usize = 2;
pub const META_sync: usize = 3;
pub const META_discard: usize = 4;
pub const META_forcewrite: usize = 5;
pub const META_io: usize = 6;

#[inline]
pub unsafe fn mark_metapage_dirty(mp: *mut metapage) {
    set_bit(META_dirty, &mut (*mp).flag);
}

/* function prototypes */
extern "C" {
    pub fn metapage_init() -> ::core::ffi::c_int;
    pub fn metapage_exit();
    pub fn __get_metapage(
        inode: *mut inode,
        lblock: ::core::ffi::c_ulong,
        size: ::core::ffi::c_uint,
        absolute: ::core::ffi::c_int,
        new: bool,
    ) -> *mut metapage;
    pub fn release_metapage(mp: *mut metapage);
    pub fn grab_metapage(mp: *mut metapage);
    pub fn force_metapage(mp: *mut metapage);
    pub fn hold_metapage(mp: *mut metapage);
    pub fn put_metapage(mp: *mut metapage);
}

#[inline]
pub unsafe fn read_metapage(inode: *mut inode, lblock: ::core::ffi::c_ulong,
                            size: ::core::ffi::c_uint, absolute: ::core::ffi::c_int)
                            -> *mut metapage {
    __get_metapage(inode, lblock, size, absolute, false)
}

#[inline]
pub unsafe fn get_metapage(inode: *mut inode, lblock: ::core::ffi::c_ulong,
                           size: ::core::ffi::c_uint, absolute: ::core::ffi::c_int)
                           -> *mut metapage {
    __get_metapage(inode, lblock, size, absolute, true)
}

#[inline]
pub unsafe fn write_metapage(mp: *mut metapage) {
    set_bit(META_dirty, &mut (*mp).flag);
    release_metapage(mp);
}

#[inline]
pub unsafe fn flush_metapage(mp: *mut metapage) {
    set_bit(META_sync, &mut (*mp).flag);
    write_metapage(mp);
}

#[inline]
pub unsafe fn discard_metapage(mp: *mut metapage) {
    clear_bit(META_dirty, &mut (*mp).flag);
    set_bit(META_discard, &mut (*mp).flag);
    release_metapage(mp);
}

#[inline]
pub unsafe fn metapage_nohomeok(mp: *mut metapage) {
    let folio = (*mp).folio;
    folio_lock(folio);
    if (*mp).nohomeok == 0 {
        (*mp).nohomeok += 1;
        mark_metapage_dirty(mp);
        folio_get(folio);
        folio_wait_writeback(folio);
    } else {
        (*mp).nohomeok += 1;
    }
    folio_unlock(folio);
}

#[inline]
pub unsafe fn metapage_wait_for_io(mp: *mut metapage) {
    if test_bit(META_io, &(*mp).flag) {
        folio_wait_writeback((*mp).folio);
    }
}

#[inline]
pub unsafe fn _metapage_homeok(mp: *mut metapage) {
    (*mp).nohomeok -= 1;
    if (*mp).nohomeok == 0 {
        folio_put((*mp).folio);
    }
}

#[inline]
pub unsafe fn metapage_homeok(mp: *mut metapage) {
    hold_metapage(mp);
    _metapage_homeok(mp);
    put_metapage(mp);
}

extern "C" {
    pub static jfs_metapage_aops: address_space_operations;
    pub fn __invalidate_metapages(inode: *mut inode, address: s64, length: ::core::ffi::c_int);
}

#[inline]
pub unsafe fn invalidate_pxd_metapages(ip: *mut inode, pxd: *mut pxd) {
    __invalidate_metapages(ip, addressPXD(pxd), lengthPXD(pxd));
}

#[inline]
pub unsafe fn invalidate_dxd_metapages(ip: *mut inode, dxd: *mut dxd) {
    __invalidate_metapages(ip, addressDXD(dxd), lengthDXD(dxd));
}

#[inline]
pub unsafe fn invalidate_xad_metapages(ip: *mut inode, xad: *mut xad) {
    __invalidate_metapages(ip, addressXAD(xad), lengthXAD(xad));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
