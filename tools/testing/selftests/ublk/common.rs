// SPDX-License-Identifier: GPL-2.0

// Translated from common.c.  The original C source includes "kublk.h"; the
// declarations below represent the externally supplied pieces used here.

use libc::{
    c_char, c_int, c_ulong, c_uint, c_void, close, fstat, fsync, ioctl, open,
    stat, strerror, EBADF, EINVAL, O_DIRECT, O_RDWR, S_ISBLK, S_ISREG,
};

unsafe extern "C" {
    static mut errno: c_int;

    fn ublk_assert(expr: bool);
    fn ublk_dbg(level: c_int, fmt: *const c_char, ...);
    fn ublk_err(fmt: *const c_char, ...);
}

pub const UBLK_DBG_DEV: c_int = 0;
pub const BLKGETSIZE64: c_ulong = 0x8008_1272;

#[repr(C)]
pub struct ublk_tgt {
    pub nr_backing_files: c_int,
    pub backing_file: *mut *mut c_char,
    pub backing_file_size: *mut c_ulong,
}

#[repr(C)]
pub struct ublk_dev {
    pub nr_fds: c_int,
    pub fds: *mut c_int,
    pub tgt: ublk_tgt,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn backing_file_tgt_deinit(dev: *mut ublk_dev) {
    let mut i: c_int;

    i = 1;
    while i < (*dev).nr_fds {
        fsync(*(*dev).fds.add(i as usize));
        close(*(*dev).fds.add(i as usize));
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn backing_file_tgt_init(
    dev: *mut ublk_dev,
    nr_direct: c_uint,
) -> c_int {
    let mut fd: c_int;
    let mut i: c_int;

    ublk_assert((*dev).nr_fds == 1);

    i = 0;
    while i < (*dev).tgt.nr_backing_files {
        let file: *mut c_char = *(*dev).tgt.backing_file.add(i as usize);
        let mut bytes: c_ulong;
        let mut st: stat = core::mem::zeroed();

        ublk_dbg(
            UBLK_DBG_DEV,
            b"%s: file %d: %s\n\0".as_ptr() as *const c_char,
            b"backing_file_tgt_init\0".as_ptr() as *const c_char,
            i,
            file,
        );

        fd = open(
            file,
            O_RDWR | if (i as c_uint) < nr_direct { O_DIRECT } else { 0 },
        );
        if fd < 0 {
            ublk_err(
                b"%s: backing file %s can't be opened: %s\n\0".as_ptr()
                    as *const c_char,
                b"backing_file_tgt_init\0".as_ptr() as *const c_char,
                file,
                strerror(errno),
            );
            return -EBADF;
        }

        if fstat(fd, &mut st as *mut stat) < 0 {
            close(fd);
            return -EBADF;
        }

        if S_ISREG(st.st_mode) {
            bytes = st.st_size as c_ulong;
        } else if S_ISBLK(st.st_mode) {
            if ioctl(
                fd,
                BLKGETSIZE64,
                &mut bytes as *mut c_ulong as *mut c_void,
            ) != 0
            {
                return -1;
            }
        } else {
            return -EINVAL;
        }

        *(*dev).tgt.backing_file_size.add(i as usize) = bytes;
        *(*dev).fds.add((*dev).nr_fds as usize) = fd;
        (*dev).nr_fds += 1;

        i += 1;
    }

    0
}
