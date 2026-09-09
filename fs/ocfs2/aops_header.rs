/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2002, 2004, 2005 Oracle.  All rights reserved.
 */

/* Dependency declarations are supplied by the surrounding kernel translation. */

extern "C" {
    pub fn ocfs2_map_folio_blocks(
        folio: *mut folio,
        p_blkno: *mut u64,
        inode: *mut inode,
        from: ::core::ffi::c_uint,
        to: ::core::ffi::c_uint,
        new: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_unlock_and_free_folios(folios: *mut *mut folio, num_folios: ::core::ffi::c_int);

    pub fn walk_page_buffers(
        handle: *mut handle_t,
        head: *mut buffer_head,
        from: ::core::ffi::c_uint,
        to: ::core::ffi::c_uint,
        partial: *mut ::core::ffi::c_int,
        func: Option<unsafe extern "C" fn(*mut handle_t, *mut buffer_head) -> ::core::ffi::c_int>,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_write_end_nolock(
        mapping: *mut address_space,
        pos: loff_t,
        len: ::core::ffi::c_uint,
        copied: ::core::ffi::c_uint,
        fsdata: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_write_begin_nolock(
        mapping: *mut address_space,
        pos: loff_t,
        len: ::core::ffi::c_uint,
        type_: ocfs2_write_type_t,
        foliop: *mut *mut folio,
        fsdata: *mut *mut ::core::ffi::c_void,
        di_bh: *mut buffer_head,
        mmap_folio: *mut folio,
    ) -> ::core::ffi::c_int;

    pub fn ocfs2_read_inline_data(inode: *mut inode, folio: *mut folio, di_bh: *mut buffer_head)
        -> ::core::ffi::c_int;
    pub fn ocfs2_size_fits_inline_data(di_bh: *mut buffer_head, new_size: u64)
        -> ::core::ffi::c_int;

    pub fn ocfs2_get_block(
        inode: *mut inode,
        iblock: sector_t,
        bh_result: *mut buffer_head,
        create: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ocfs2_write_type_t {
    OCFS2_WRITE_BUFFER = 0,
    OCFS2_WRITE_DIRECT,
    OCFS2_WRITE_MMAP,
}

/* all ocfs2_dio_end_io()'s fault */
#[inline]
pub unsafe fn ocfs2_iocb_is_rw_locked(iocb: *mut kiocb) -> ::core::ffi::c_int {
    test_bit(0, &mut (*iocb).private as *mut _ as *mut ::core::ffi::c_ulong)
}

#[inline]
pub unsafe fn ocfs2_iocb_set_rw_locked(iocb: *mut kiocb, level: ::core::ffi::c_int) {
    set_bit(0, &mut (*iocb).private as *mut _ as *mut ::core::ffi::c_ulong);
    if level != 0 {
        set_bit(1, &mut (*iocb).private as *mut _ as *mut ::core::ffi::c_ulong);
    } else {
        clear_bit(1, &mut (*iocb).private as *mut _ as *mut ::core::ffi::c_ulong);
    }
}

/*
 * Using a named enum representing lock types in terms of #N bit stored in
 * iocb->private, which is going to be used for communication between
 * ocfs2_dio_end_io() and ocfs2_file_write/read_iter().
 */
#[repr(C)]
pub enum ocfs2_iocb_lock_bits {
    OCFS2_IOCB_RW_LOCK = 0,
    OCFS2_IOCB_RW_LOCK_LEVEL,
    OCFS2_IOCB_NUM_LOCKS,
}

#[inline]
pub unsafe fn ocfs2_iocb_init_rw_locked(iocb: *mut kiocb) {
    (*iocb).private = core::ptr::null_mut();
}

#[inline]
pub unsafe fn ocfs2_iocb_clear_rw_locked(iocb: *mut kiocb) {
    clear_bit(OCFS2_IOCB_RW_LOCK as ::core::ffi::c_ulong,
              &mut (*iocb).private as *mut _ as *mut ::core::ffi::c_ulong);
}

#[inline]
pub unsafe fn ocfs2_iocb_rw_locked_level(iocb: *mut kiocb) -> ::core::ffi::c_int {
    test_bit(OCFS2_IOCB_RW_LOCK_LEVEL as ::core::ffi::c_ulong,
             &mut (*iocb).private as *mut _ as *mut ::core::ffi::c_ulong)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
