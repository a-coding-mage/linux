/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * sysfile.h
 *
 * Function prototypes
 *
 * Copyright (C) 2002, 2004 Oracle.  All rights reserved.
 */

/*
 * C header guard: OCFS2_SYSFILE_H
 *
 * The declarations below depend on the externally supplied `inode` and
 * `ocfs2_super` types.
 */

extern "C" {
    pub fn ocfs2_get_system_file_inode(
        osb: *mut ocfs2_super,
        type_: core::ffi::c_int,
        slot: u32,
    ) -> *mut inode;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
