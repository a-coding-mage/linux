/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * NILFS segment usage file.
 *
 * Copyright (C) 2006-2008 Nippon Telegraph and Telephone Corporation.
 *
 * Written by Koji Sato.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/errno.h, linux/fs.h, linux/buffer_head.h, and mdt.h.

#[inline]
pub unsafe fn nilfs_sufile_get_nsegments(sufile: *mut inode) -> ::core::ffi::c_ulong {
    (*((*sufile).i_sb).s_fs_info as *mut the_nilfs).as_ref().unwrap().ns_nsegments
}

extern "C" {
    pub fn nilfs_sufile_get_ncleansegs(sufile: *mut inode) -> ::core::ffi::c_ulong;
    pub fn nilfs_sufile_set_alloc_range(sufile: *mut inode, start: u64, end: u64) -> i32;
    pub fn nilfs_sufile_alloc(sufile: *mut inode, segnum: *mut u64) -> i32;
    pub fn nilfs_sufile_mark_dirty(sufile: *mut inode, segnum: u64) -> i32;
    pub fn nilfs_sufile_set_segment_usage(
        sufile: *mut inode, segnum: u64, nblocks: ::core::ffi::c_ulong, modtime: time64_t,
    ) -> i32;
    pub fn nilfs_sufile_get_stat(sufile: *mut inode, stat: *mut nilfs_sustat) -> i32;
    pub fn nilfs_sufile_get_suinfo(
        sufile: *mut inode, segnum: u64, buf: *mut ::core::ffi::c_void,
        nitems: u32, bufsz: usize,
    ) -> isize;
    pub fn nilfs_sufile_set_suinfo(
        sufile: *mut inode, buf: *mut ::core::ffi::c_void, nitems: u32, bufsz: usize,
    ) -> isize;
    pub fn nilfs_sufile_updatev(
        sufile: *mut inode, segnumv: *mut u64, nsegs: usize, create: i32,
        ndone: *mut usize,
        dofunc: Option<unsafe extern "C" fn(*mut inode, u64, *mut buffer_head, *mut buffer_head)>,
    ) -> i32;
    pub fn nilfs_sufile_update(
        sufile: *mut inode, segnum: u64, create: i32,
        dofunc: Option<unsafe extern "C" fn(*mut inode, u64, *mut buffer_head, *mut buffer_head)>,
    ) -> i32;
    pub fn nilfs_sufile_do_scrap(*mut inode, u64, *mut buffer_head, *mut buffer_head);
    pub fn nilfs_sufile_do_free(*mut inode, u64, *mut buffer_head, *mut buffer_head);
    pub fn nilfs_sufile_do_cancel_free(*mut inode, u64, *mut buffer_head, *mut buffer_head);
    pub fn nilfs_sufile_do_set_error(*mut inode, u64, *mut buffer_head, *mut buffer_head);
    pub fn nilfs_sufile_resize(sufile: *mut inode, newnsegs: u64) -> i32;
    pub fn nilfs_sufile_read(
        sb: *mut super_block, susize: usize, raw_inode: *mut nilfs_inode,
        inodep: *mut *mut inode,
    ) -> i32;
    pub fn nilfs_sufile_trim_fs(sufile: *mut inode, range: *mut fstrim_range) -> i32;
}

/**
 * nilfs_sufile_warn_on_error - warn on unexpected sufile error
 *
 * Returns 0 if `err` is 0, `-EROFS` in read-only degraded mode, and `-EIO`
 * otherwise.  The kernel WARN_ONCE/sb_rdonly semantics are retained here.
 */
#[macro_export]
macro_rules! nilfs_sufile_warn_on_error {
    ($sufile:expr, $err:expr) => {{
        let mut _err = $err;
        if _err != 0 {
            _err = if unsafe { WARN_ONCE(!sb_rdonly((*$sufile).i_sb),
                "unexpected sufile error %d\n", _err) } { -EIO } else { -EROFS };
        }
        _err
    }};
}

#[inline]
pub unsafe fn nilfs_sufile_scrap(sufile: *mut inode, segnum: u64) -> i32 {
    nilfs_sufile_update(sufile, segnum, 1, Some(nilfs_sufile_do_scrap))
}

#[inline]
pub unsafe fn nilfs_sufile_free(sufile: *mut inode, segnum: u64) -> i32 {
    nilfs_sufile_update(sufile, segnum, 0, Some(nilfs_sufile_do_free))
}

#[inline]
pub unsafe fn nilfs_sufile_freev(sufile: *mut inode, segnumv: *mut u64, nsegs: usize, ndone: *mut usize) -> i32 {
    nilfs_sufile_updatev(sufile, segnumv, nsegs, 0, ndone, Some(nilfs_sufile_do_free))
}

#[inline]
pub unsafe fn nilfs_sufile_cancel_freev(sufile: *mut inode, segnumv: *mut u64, nsegs: usize, ndone: *mut usize) -> i32 {
    nilfs_sufile_updatev(sufile, segnumv, nsegs, 0, ndone, Some(nilfs_sufile_do_cancel_free))
}

#[inline]
pub unsafe fn nilfs_sufile_set_error(sufile: *mut inode, segnum: u64) -> i32 {
    nilfs_sufile_update(sufile, segnum, 0, Some(nilfs_sufile_do_set_error))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
