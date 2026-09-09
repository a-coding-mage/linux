/* SPDX-License-Identifier: GPL-2.0-or-later */
/* MTD-based superblock handling
 *
 * Copyright © 2006 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

use core::ffi::c_int;

/* The declarations below are enabled for kernel builds. */

/* Types supplied by the Linux filesystem and MTD dependencies. */
pub struct fs_context;
pub struct super_block;

unsafe extern "C" {
    pub fn get_tree_mtd(
        fc: *mut fs_context,
        fill_super: Option<
            unsafe extern "C" fn(
                sb: *mut super_block,
                fc: *mut fs_context,
            ) -> c_int,
        >,
    ) -> c_int;

    pub fn kill_mtd_super(sb: *mut super_block);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
