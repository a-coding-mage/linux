/* SPDX-License-Identifier: GPL-2.0-or-later */
/*

    Types and defines needed for RDS. This is included by
    saa6588.c and every driver (e.g. bttv-driver.c) that wants
    to use the saa6588 module.

    (c) 2005 by Hans J. Koch

*/

// C header dependency: the kernel definitions used below are supplied by
// the surrounding translation unit.

#[repr(C)]
pub struct saa6588_command {
    pub block_count: ::core::ffi::c_uint,
    pub nonblocking: bool,
    pub result: ::core::ffi::c_int,
    pub buffer: *mut ::core::ffi::c_uchar,
    pub instance: *mut file,
    pub event_list: *mut poll_table,
    pub poll_mask: __poll_t,
}

/* These ioctls are internal to the kernel. */
// The _IOW/_IOR ioctl encodings are supplied by the kernel translation
// environment; these preserve the original macro expressions and intent.
pub const SAA6588_CMD_CLOSE: _ = _IOW('R', 2, ::core::ffi::c_int);
pub const SAA6588_CMD_READ: _ = _IOR('R', 3, ::core::ffi::c_int);
pub const SAA6588_CMD_POLL: _ = _IOR('R', 4, ::core::ffi::c_int);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
