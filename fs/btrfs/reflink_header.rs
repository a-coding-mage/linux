/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: BTRFS_REFLINK_H
// Dependency: <linux/types.h>

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

extern "C" {
    pub fn btrfs_remap_file_range(
        file_in: *mut file,
        pos_in: i64,
        file_out: *mut file,
        pos_out: i64,
        len: i64,
        remap_flags: u32,
    ) -> i64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
