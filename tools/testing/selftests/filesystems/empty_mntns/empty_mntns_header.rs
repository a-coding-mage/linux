/* SPDX-License-Identifier: GPL-2.0-or-later */

// C dependencies: <errno.h>, <stdlib.h>, "../statmount/statmount.h"

#[cfg(not(UNSHARE_EMPTY_MNTNS))]
pub const UNSHARE_EMPTY_MNTNS: u32 = 0x00100000;

#[cfg(not(CLONE_EMPTY_MNTNS))]
pub const CLONE_EMPTY_MNTNS: u64 = 1_u64 << 37;

unsafe extern "C" {
    pub static LSMT_ROOT: u64;

    pub fn listmount(
        mnt_id: u64,
        last_mnt_id: u64,
        flags: u64,
        list: *mut u64,
        num: usize,
        spare: u64,
    ) -> isize;
}

#[inline]
pub unsafe fn count_mounts() -> isize {
    let mut list: [u64; 4096] = [0; 4096];

    unsafe { listmount(LSMT_ROOT, 0, 0, list.as_mut_ptr(), list.len(), 0) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
