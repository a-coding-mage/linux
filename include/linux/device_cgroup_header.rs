/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by linux/fs.h in the original header.

pub const DEVCG_ACC_MKNOD: i32 = 1;
pub const DEVCG_ACC_READ: i32 = 2;
pub const DEVCG_ACC_WRITE: i32 = 4;
pub const DEVCG_ACC_MASK: i32 = DEVCG_ACC_MKNOD | DEVCG_ACC_READ | DEVCG_ACC_WRITE;

pub const DEVCG_DEV_BLOCK: i16 = 1;
pub const DEVCG_DEV_CHAR: i16 = 2;
pub const DEVCG_DEV_ALL: i16 = 4; /* this represents all devices */

// Original condition: CONFIG_CGROUP_DEVICE || CONFIG_CGROUP_BPF.
#[cfg(any(feature = "CONFIG_CGROUP_DEVICE", feature = "CONFIG_CGROUP_BPF"))]
extern "C" {
    pub fn devcgroup_check_permission(
        type_: i16,
        major: u32,
        minor: u32,
        access: i16,
    ) -> i32;
}

#[cfg(any(feature = "CONFIG_CGROUP_DEVICE", feature = "CONFIG_CGROUP_BPF"))]
#[inline]
pub unsafe fn devcgroup_inode_permission(inode: *mut inode, mask: i32) -> i32 {
    let mut access: i16 = 0;

    if likely(!S_ISBLK((*inode).i_mode) && !S_ISCHR((*inode).i_mode)) {
        return 0;
    }

    if (*inode).i_rdev == 0 {
        return 0;
    }

    let type_: i16;
    if S_ISBLK((*inode).i_mode) {
        type_ = DEVCG_DEV_BLOCK;
    } else {
        // S_ISCHR by the test above.
        type_ = DEVCG_DEV_CHAR;
    }

    if mask & MAY_WRITE != 0 {
        access |= DEVCG_ACC_WRITE as i16;
    }
    if mask & MAY_READ != 0 {
        access |= DEVCG_ACC_READ as i16;
    }

    devcgroup_check_permission(type_, imajor(inode), iminor(inode), access)
}

#[cfg(any(feature = "CONFIG_CGROUP_DEVICE", feature = "CONFIG_CGROUP_BPF"))]
#[inline]
pub unsafe fn devcgroup_inode_mknod(mode: i32, dev: dev_t) -> i32 {
    let type_: i16;

    if !S_ISBLK(mode) && !S_ISCHR(mode) {
        return 0;
    }

    if S_ISCHR(mode) && dev == WHITEOUT_DEV {
        return 0;
    }

    if S_ISBLK(mode) {
        type_ = DEVCG_DEV_BLOCK;
    } else {
        type_ = DEVCG_DEV_CHAR;
    }

    devcgroup_check_permission(type_, MAJOR(dev), MINOR(dev), DEVCG_ACC_MKNOD as i16)
}

// Original condition: neither CONFIG_CGROUP_DEVICE nor CONFIG_CGROUP_BPF.
#[cfg(not(any(feature = "CONFIG_CGROUP_DEVICE", feature = "CONFIG_CGROUP_BPF")))]
#[inline]
pub unsafe fn devcgroup_check_permission(_type_: i16, _major: u32, _minor: u32, _access: i16) -> i32 {
    0
}

#[cfg(not(any(feature = "CONFIG_CGROUP_DEVICE", feature = "CONFIG_CGROUP_BPF")))]
#[inline]
pub unsafe fn devcgroup_inode_permission(_inode: *mut inode, _mask: i32) -> i32 {
    0
}

#[cfg(not(any(feature = "CONFIG_CGROUP_DEVICE", feature = "CONFIG_CGROUP_BPF")))]
#[inline]
pub unsafe fn devcgroup_inode_mknod(_mode: i32, _dev: dev_t) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
