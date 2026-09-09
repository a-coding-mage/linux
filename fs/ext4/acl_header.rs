// SPDX-License-Identifier: GPL-2.0
/*
  File: fs/ext4/acl.h

  (C) 2001 Andreas Gruenbacher, <a.gruenbacher@computer.org>
*/

// Dependency supplied by the surrounding kernel translation:
// #include <linux/posix_acl_xattr.h>

pub const EXT4_ACL_VERSION: u16 = 0x0001;

#[repr(C)]
pub struct ext4_acl_entry {
    pub e_tag: u16,
    pub e_perm: u16,
    pub e_id: u32,
}

#[repr(C)]
pub struct ext4_acl_entry_short {
    pub e_tag: u16,
    pub e_perm: u16,
}

#[repr(C)]
pub struct ext4_acl_header {
    pub a_version: u32,
}

#[inline]
pub fn ext4_acl_size(count: i32) -> usize {
    if count <= 4 {
        std::mem::size_of::<ext4_acl_header>()
            .wrapping_add((count as usize).wrapping_mul(std::mem::size_of::<ext4_acl_entry_short>()))
    } else {
        std::mem::size_of::<ext4_acl_header>()
            .wrapping_add(4usize.wrapping_mul(std::mem::size_of::<ext4_acl_entry_short>()))
            .wrapping_add(((count - 4) as usize).wrapping_mul(std::mem::size_of::<ext4_acl_entry>()))
    }
}

#[inline]
pub fn ext4_acl_count(mut size: usize) -> i32 {
    size = size.wrapping_sub(std::mem::size_of::<ext4_acl_header>());
    let s = size
        .wrapping_sub(4usize.wrapping_mul(std::mem::size_of::<ext4_acl_entry_short>()))
        as isize;
    if s < 0 {
        if size % std::mem::size_of::<ext4_acl_entry_short>() != 0 {
            return -1;
        }
        (size / std::mem::size_of::<ext4_acl_entry_short>()) as i32
    } else {
        if (s as usize) % std::mem::size_of::<ext4_acl_entry>() != 0 {
            return -1;
        }
        (s as usize / std::mem::size_of::<ext4_acl_entry>()) as i32 + 4
    }
}

// CONFIG_EXT4_FS_POSIX_ACL controls the following declarations.
#[cfg(feature = "CONFIG_EXT4_FS_POSIX_ACL")]
extern "C" {
    pub fn ext4_get_acl(inode: *mut inode, type_: i32, rcu: bool) -> *mut posix_acl;
    pub fn ext4_set_acl(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        acl: *mut posix_acl,
        type_: i32,
    ) -> i32;
    pub fn ext4_init_acl(handle: *mut handle_t, inode: *mut inode, dir: *mut inode) -> i32;
}

#[cfg(not(feature = "CONFIG_EXT4_FS_POSIX_ACL"))]
pub const ext4_get_acl: Option<unsafe extern "C" fn(*mut inode, i32, bool) -> *mut posix_acl> = None;

#[cfg(not(feature = "CONFIG_EXT4_FS_POSIX_ACL"))]
pub const ext4_set_acl: Option<unsafe extern "C" fn(*mut mnt_idmap, *mut dentry, *mut posix_acl, i32) -> i32> = None;

#[cfg(not(feature = "CONFIG_EXT4_FS_POSIX_ACL"))]
#[inline]
pub unsafe fn ext4_init_acl(_handle: *mut handle_t, _inode: *mut inode, _dir: *mut inode) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
