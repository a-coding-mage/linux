/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * Copyright © 2001-2007 Red Hat, Inc.
 * Copyright © 2004-2010 David Woodhouse <dwmw2@infradead.org>
 *
 * Created by David Woodhouse <dwmw2@infradead.org>
 *
 * For licensing information, see the file 'LICENCE' in this directory.
 *
 */

// Dependency supplied by <linux/fs.h>.
#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

// ENOTTY is supplied by the target Linux environment.
unsafe extern "C" {
    pub static ENOTTY: core::ffi::c_long;
}

pub unsafe extern "C" fn jffs2_ioctl(
    filp: *mut file,
    cmd: core::ffi::c_uint,
    arg: core::ffi::c_ulong,
) -> core::ffi::c_long {
    // Later, this will provide for lsattr.jffs2 and chattr.jffs2, which
    // will include compression support etc.
    let _ = (filp, cmd, arg);
    -ENOTTY
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
