/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Network node table
 *
 * SELinux must keep a mapping of network nodes to labels/SIDs.  This
 * mapping is maintained as part of the normal policy but a fast cache is
 * needed to reduce the lookup overhead since most of these queries happen on
 * a per-packet basis.
 *
 * Author: Paul Moore <paul@paul-moore.com>
 */

/*
 * (c) Copyright Hewlett-Packard Development Company, L.P., 2007
 */

/* Dependency intent from C: #include <linux/types.h> provides u16 and u32. */

unsafe extern "C" {
    pub fn sel_netnode_flush();

    pub fn sel_netnode_sid(addr: *const core::ffi::c_void, family: u16, sid: *mut u32) -> core::ffi::c_int;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
