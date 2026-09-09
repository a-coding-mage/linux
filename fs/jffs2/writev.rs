/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * Copyright © 2001-2007 Red Hat, Inc.
 *
 * Created by David Woodhouse <dwmw2@infradead.org>
 *
 * For licensing information, see the file 'LICENCE' in this directory.
 *
 */

use core::ffi::c_void;

pub type u_char = u8;
pub type loff_t = i64;

#[repr(C)]
pub struct mtd_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct jffs2_sb_info {
    pub mtd: *mut mtd_info,
}

#[repr(C)]
pub struct kvec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}

extern "C" {
    fn jffs2_is_writebuffered(c: *mut jffs2_sb_info) -> bool;
    fn jffs2_sum_active() -> bool;
    fn jffs2_sum_add_kvec(
        c: *mut jffs2_sb_info,
        vecs: *const kvec,
        count: libc::c_ulong,
        ofs: u32,
    ) -> libc::c_int;
    fn mtd_writev(
        mtd: *mut mtd_info,
        vecs: *const kvec,
        count: libc::c_ulong,
        to: loff_t,
        retlen: *mut usize,
    ) -> libc::c_int;
    fn mtd_write(
        mtd: *mut mtd_info,
        ofs: loff_t,
        len: usize,
        retlen: *mut usize,
        buf: *const u_char,
    ) -> libc::c_int;
}

pub unsafe fn jffs2_flash_direct_writev(
    c: *mut jffs2_sb_info,
    vecs: *const kvec,
    count: libc::c_ulong,
    to: loff_t,
    retlen: *mut usize,
) -> libc::c_int {
    if !jffs2_is_writebuffered(c) {
        if jffs2_sum_active() {
            let res: libc::c_int;
            res = jffs2_sum_add_kvec(c, vecs, count, to as u32);
            if res != 0 {
                return res;
            }
        }
    }

    mtd_writev((*c).mtd, vecs, count, to, retlen)
}

pub unsafe fn jffs2_flash_direct_write(
    c: *mut jffs2_sb_info,
    ofs: loff_t,
    len: usize,
    retlen: *mut usize,
    buf: *const u_char,
) -> libc::c_int {
    let ret: libc::c_int;
    ret = mtd_write((*c).mtd, ofs, len, retlen, buf);

    if jffs2_sum_active() {
        let mut vecs: [kvec; 1] = [kvec {
            iov_base: core::ptr::null_mut(),
            iov_len: 0,
        }];
        let res: libc::c_int;

        vecs[0].iov_base = buf as *mut c_void;
        vecs[0].iov_len = len;

        res = jffs2_sum_add_kvec(c, vecs.as_ptr(), 1, ofs as u32);
        if res != 0 {
            return res;
        }
    }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
