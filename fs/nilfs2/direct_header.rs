/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * NILFS direct block pointer.
 *
 * Copyright (C) 2006-2008 Nippon Telegraph and Telephone Corporation.
 *
 * Written by Koji Sato.
 */

/* C dependencies: linux/types.h, linux/buffer_head.h, and bmap.h. */

pub const NILFS_DIRECT_NBLOCKS: usize =
    NILFS_BMAP_SIZE / core::mem::size_of::<u64>() - 1;
pub const NILFS_DIRECT_KEY_MIN: usize = 0;
pub const NILFS_DIRECT_KEY_MAX: usize = NILFS_DIRECT_NBLOCKS - 1;

extern "C" {
    pub fn nilfs_direct_init(bmap: *mut nilfs_bmap) -> i32;
    pub fn nilfs_direct_delete_and_convert(
        bmap: *mut nilfs_bmap,
        key: u64,
        oldkey: *mut u64,
        oldptr: *mut u64,
        level: i32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
