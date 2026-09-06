/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Network port table
 *
 * SELinux must keep a mapping of network ports to labels/SIDs.  This
 * mapping is maintained as part of the normal policy but a fast cache is
 * needed to reduce the lookup overhead.
 *
 * Author: Paul Moore <paul@paul-moore.com>
 */

/*
 * (c) Copyright Hewlett-Packard Development Company, L.P., 2008
 */

/* C dependency: #include <linux/types.h> */

use core::ffi::c_int;

pub type u8 = core::ffi::c_uchar;
pub type u16 = core::ffi::c_ushort;
pub type u32 = core::ffi::c_uint;

unsafe extern "C" {
    pub fn sel_netport_flush();

    pub fn sel_netport_sid(protocol: u8, pnum: u16, sid: *mut u32) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
