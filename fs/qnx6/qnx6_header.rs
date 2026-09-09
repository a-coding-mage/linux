/* SPDX-License-Identifier: GPL-2.0 */
/*
 * QNX6 file system, Linux implementation.
 *
 * Version : 1.0.0
 *
 * History :
 *
 * 01-02-2012 by Kai Bankett (chaosman@ontika.net) : first release.
 * 16-02-2012 page map extension by Al Viro
 *
 */

/* Linux kernel dependencies: fs.h, pagemap.h, and qnx6_fs.h. */

pub type __fs16 = u16;
pub type __fs32 = u32;
pub type __fs64 = u64;

#[repr(C)]
pub struct qnx6_sb_info {
    pub sb_buf: *mut buffer_head, /* superblock buffer */
    pub sb: *mut qnx6_super_block, /* our superblock */
    pub s_blks_off: i32, /* blkoffset fs-startpoint */
    pub s_ptrbits: i32, /* indirect pointer bitfield */
    pub s_mount_opt: c_ulong, /* all mount options */
    pub s_bytesex: i32, /* holds endianess info */
    pub inodes: *mut inode,
    pub longfile: *mut inode,
}

#[repr(C)]
pub struct qnx6_inode_info {
    pub di_block_ptr: [__fs32; QNX6_NO_DIRECT_POINTERS],
    pub di_filelevels: u8,
    pub i_dir_start_lookup: u32,
    pub vfs_inode: inode,
}

extern "C" {
    pub fn qnx6_iget(sb: *mut super_block, ino: c_uint) -> *mut inode;
    pub fn qnx6_lookup(
        dir: *mut inode,
        dentry: *mut dentry,
        flags: c_uint,
    ) -> *mut dentry;

    #[cfg(CONFIG_QNX6FS_DEBUG)]
    pub fn qnx6_superblock_debug(sb: *mut qnx6_super_block, s: *mut super_block);

    pub static qnx6_dir_inode_operations: inode_operations;
    pub static qnx6_dir_operations: file_operations;

    pub fn le64_to_cpu(n: u64) -> u64;
    pub fn be64_to_cpu(n: u64) -> u64;
    pub fn cpu_to_le64(n: u64) -> u64;
    pub fn cpu_to_be64(n: u64) -> u64;
    pub fn le32_to_cpu(n: u32) -> u32;
    pub fn be32_to_cpu(n: u32) -> u32;
    pub fn cpu_to_le32(n: u32) -> u32;
    pub fn cpu_to_be32(n: u32) -> u32;
    pub fn le16_to_cpu(n: u16) -> u16;
    pub fn be16_to_cpu(n: u16) -> u16;
    pub fn cpu_to_le16(n: u16) -> u16;
    pub fn cpu_to_be16(n: u16) -> u16;
}

#[repr(C)]
pub struct buffer_head;
#[repr(C)]
pub struct qnx6_super_block;
#[repr(C)]
pub struct super_block {
    pub s_fs_info: *mut core::ffi::c_void,
}
#[repr(C)]
pub struct inode;
#[repr(C)]
pub struct dentry;
#[repr(C)]
pub struct inode_operations;
#[repr(C)]
pub struct file_operations;

pub type c_uint = core::ffi::c_uint;
pub type c_ulong = core::ffi::c_ulong;

#[inline]
pub unsafe fn QNX6_SB(sb: *mut super_block) -> *mut qnx6_sb_info {
    (*sb).s_fs_info as *mut qnx6_sb_info
}

#[inline]
pub unsafe fn QNX6_I(inode_ptr: *mut inode) -> *mut qnx6_inode_info {
    /* Equivalent to container_of(inode, struct qnx6_inode_info, vfs_inode). */
    let base = inode_ptr as *mut u8;
    base.sub(core::mem::offset_of!(qnx6_inode_info, vfs_inode)) as *mut qnx6_inode_info
}

#[macro_export]
macro_rules! clear_opt {
    ($o:expr, $opt:ident) => {{ $o &= !(concat_idents!(QNX6_MOUNT_, $opt)); }};
}

#[macro_export]
macro_rules! set_opt {
    ($o:expr, $opt:ident) => {{ $o |= concat_idents!(QNX6_MOUNT_, $opt); }};
}

#[macro_export]
macro_rules! test_opt {
    ($sb:expr, $opt:ident) => {{
        (unsafe { QNX6_SB($sb) }.as_ref().unwrap().s_mount_opt
            & concat_idents!(QNX6_MOUNT_, $opt))
    }};
}

pub const BYTESEX_LE: i32 = 0;
pub const BYTESEX_BE: i32 = 1;

#[inline]
pub unsafe fn fs64_to_cpu(sbi: *mut qnx6_sb_info, n: __fs64) -> u64 {
    if (*sbi).s_bytesex == BYTESEX_LE {
        le64_to_cpu(n)
    } else {
        be64_to_cpu(n)
    }
}

#[inline]
pub unsafe fn cpu_to_fs64(sbi: *mut qnx6_sb_info, n: u64) -> __fs64 {
    if (*sbi).s_bytesex == BYTESEX_LE {
        cpu_to_le64(n)
    } else {
        cpu_to_be64(n)
    }
}

#[inline]
pub unsafe fn fs32_to_cpu(sbi: *mut qnx6_sb_info, n: __fs32) -> u32 {
    if (*sbi).s_bytesex == BYTESEX_LE { le32_to_cpu(n) } else { be32_to_cpu(n) }
}

#[inline]
pub unsafe fn cpu_to_fs32(sbi: *mut qnx6_sb_info, n: u32) -> __fs32 {
    if (*sbi).s_bytesex == BYTESEX_LE { cpu_to_le32(n) } else { cpu_to_be32(n) }
}

#[inline]
pub unsafe fn fs16_to_cpu(sbi: *mut qnx6_sb_info, n: __fs16) -> u16 {
    if (*sbi).s_bytesex == BYTESEX_LE { le16_to_cpu(n) } else { be16_to_cpu(n) }
}

#[inline]
pub unsafe fn cpu_to_fs16(sbi: *mut qnx6_sb_info, n: u16) -> __fs16 {
    if (*sbi).s_bytesex == BYTESEX_LE { cpu_to_le16(n) } else { cpu_to_be16(n) }
}

extern "C" {
    pub fn qnx6_mmi_fill_super(s: *mut super_block, silent: i32) -> *mut qnx6_super_block;
    pub fn qnx6_find_ino(len: i32, dir: *mut inode, name: *const core::ffi::c_char) -> c_uint;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
