/* SPDX-License-Identifier: GPL-2.0 */
/*
  File: fs/ext2/acl.h

  (C) 2001 Andreas Gruenbacher, <a.gruenbacher@computer.org>
*/

// Dependency supplied by the surrounding kernel translation:
// #include <linux/posix_acl_xattr.h>

pub const EXT2_ACL_VERSION: u16 = 0x0001;

#[repr(C)]
pub struct ext2_acl_entry {
    pub e_tag: __le16,
    pub e_perm: __le16,
    pub e_id: __le32,
}

#[repr(C)]
pub struct ext2_acl_entry_short {
    pub e_tag: __le16,
    pub e_perm: __le16,
}

#[repr(C)]
pub struct ext2_acl_header {
    pub a_version: __le32,
}

#[inline]
pub fn ext2_acl_size(count: libc::c_int) -> usize {
    if count <= 4 {
        core::mem::size_of::<ext2_acl_header>()
            + (count as usize) * core::mem::size_of::<ext2_acl_entry_short>()
    } else {
        core::mem::size_of::<ext2_acl_header>()
            + 4 * core::mem::size_of::<ext2_acl_entry_short>()
            + ((count - 4) as usize) * core::mem::size_of::<ext2_acl_entry>()
    }
}

#[inline]
pub fn ext2_acl_count(mut size: usize) -> libc::c_int {
    let s: isize;
    size -= core::mem::size_of::<ext2_acl_header>();
    s = size as isize - 4 * core::mem::size_of::<ext2_acl_entry_short>() as isize;
    if s < 0 {
        if size % core::mem::size_of::<ext2_acl_entry_short>() != 0 {
            return -1;
        }
        (size / core::mem::size_of::<ext2_acl_entry_short>()) as libc::c_int
    } else {
        if (s as usize) % core::mem::size_of::<ext2_acl_entry>() != 0 {
            return -1;
        }
        (s as usize / core::mem::size_of::<ext2_acl_entry>() + 4) as libc::c_int
    }
}

#[cfg(CONFIG_EXT2_FS_POSIX_ACL)]
extern "C" {
    pub fn ext2_get_acl(inode: *mut inode, type_: libc::c_int, rcu: bool) -> *mut posix_acl;
    pub fn ext2_set_acl(
        idmap: *mut mnt_idmap,
        dentry: *mut dentry,
        acl: *mut posix_acl,
        type_: libc::c_int,
    ) -> libc::c_int;
    pub fn ext2_init_acl(inode: *mut inode, dir: *mut inode) -> libc::c_int;
}

#[cfg(not(CONFIG_EXT2_FS_POSIX_ACL))]
// #include <linux/sched.h>
pub const ext2_get_acl: *const () = core::ptr::null();

#[cfg(not(CONFIG_EXT2_FS_POSIX_ACL))]
pub const ext2_set_acl: *const () = core::ptr::null();

#[cfg(not(CONFIG_EXT2_FS_POSIX_ACL))]
#[inline]
pub fn ext2_init_acl(_inode: *mut inode, _dir: *mut inode) -> libc::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
