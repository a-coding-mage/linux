/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by linux/fs.h in the original header.

pub const I_VERSION_QUERIED_SHIFT: u32 = 1;
pub const I_VERSION_QUERIED: u64 = 1u64 << (I_VERSION_QUERIED_SHIFT - 1);
pub const I_VERSION_INCREMENT: u64 = 1u64 << I_VERSION_QUERIED_SHIFT;

pub unsafe fn inode_set_iversion_raw(inode: *mut inode, val: u64) {
    (*inode).i_version.set(val);
}

pub unsafe fn inode_peek_iversion_raw(inode: *const inode) -> u64 {
    (*inode).i_version.read()
}

pub unsafe fn inode_set_max_iversion_raw(inode: *mut inode, val: u64) {
    let mut cur = inode_peek_iversion_raw(inode);
    loop {
        if cur > val {
            break;
        }
        if (*inode).i_version.try_cmpxchg(&mut cur, val) {
            break;
        }
    }
}

pub unsafe fn inode_set_iversion(inode: *mut inode, val: u64) {
    inode_set_iversion_raw(inode, val << I_VERSION_QUERIED_SHIFT);
}

pub unsafe fn inode_set_iversion_queried(inode: *mut inode, val: u64) {
    inode_set_iversion_raw(
        inode,
        (val << I_VERSION_QUERIED_SHIFT) | I_VERSION_QUERIED,
    );
}

extern "C" {
    pub fn inode_maybe_inc_iversion(inode: *mut inode, force: bool) -> bool;
}

pub unsafe fn inode_inc_iversion(inode: *mut inode) {
    inode_maybe_inc_iversion(inode, true);
}

pub unsafe fn inode_iversion_need_inc(inode: *mut inode) -> bool {
    inode_peek_iversion_raw(inode) & I_VERSION_QUERIED != 0
}

pub unsafe fn inode_inc_iversion_raw(inode: *mut inode) {
    (*inode).i_version.fetch_add(1);
}

pub unsafe fn inode_peek_iversion(inode: *const inode) -> u64 {
    inode_peek_iversion_raw(inode) >> I_VERSION_QUERIED_SHIFT
}

pub unsafe fn time_to_chattr(t: *const timespec64) -> u64 {
    let mut chattr = (*t).tv_sec as u64;
    chattr <<= 32;
    chattr += (*t).tv_nsec as u64;
    chattr
}

extern "C" {
    pub fn inode_query_iversion(inode: *mut inode) -> u64;
}

pub unsafe fn inode_eq_iversion_raw(inode: *const inode, old: u64) -> bool {
    inode_peek_iversion_raw(inode) == old
}

pub unsafe fn inode_eq_iversion(inode: *const inode, old: u64) -> bool {
    inode_peek_iversion(inode) == old
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
