/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) International Business Machines Corp., 2000-2001
 */

/*
 * C header guard: _H_JFS_EXTENT
 */

/*  get block allocation hint as location of disk inode */
#[macro_export]
macro_rules! INOHINT {
    ($ip:expr) => {
        addressPXD(&JFS_IP($ip).ixpxd) + lengthPXD(&JFS_IP($ip).ixpxd) - 1
    };
}

extern "C" {
    pub fn extAlloc(
        inode: *mut inode,
        x: s64,
        y: s64,
        xp: *mut xad_t,
        abnr: bool,
    ) -> i32;
    pub fn extHint(inode: *mut inode, x: s64, xp: *mut xad_t) -> i32;
    pub fn extRecord(inode: *mut inode, xp: *mut xad_t) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
