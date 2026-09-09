/* SPDX-License-Identifier: GPL-2.0 */
// C dependencies: <linux/fs.h>, <linux/qnx4_fs.h>

pub const QNX4_DEBUG: i32 = 0;

// When QNX4_DEBUG is enabled, the C macro expands to printk X; otherwise it
// is a no-op. The printk dependency is supplied by the surrounding kernel.

#[repr(C)]
pub struct qnx4_sb_info {
    pub Version: core::ffi::c_uint, // may be useful
    pub BitMap: *mut qnx4_inode_entry, // useful
}

#[repr(C)]
pub struct qnx4_inode_info {
    pub raw: qnx4_inode_entry,
    pub mmu_private: loff_t,
    pub vfs_inode: inode,
}

extern "C" {
    pub fn qnx4_iget(sb: *mut super_block, ino: core::ffi::c_ulong) -> *mut inode;
    pub fn qnx4_lookup(
        dir: *mut inode,
        dentry: *mut dentry,
        flags: core::ffi::c_uint,
    ) -> *mut dentry;
    pub fn qnx4_count_free_blocks(sb: *mut super_block) -> core::ffi::c_ulong;
    pub fn qnx4_block_map(inode: *mut inode, iblock: core::ffi::c_long) -> core::ffi::c_ulong;

    pub static qnx4_dir_inode_operations: inode_operations;
    pub static qnx4_dir_operations: file_operations;
    pub fn qnx4_is_free(sb: *mut super_block, block: core::ffi::c_long) -> core::ffi::c_int;
}

#[inline]
pub unsafe fn qnx4_sb(sb: *mut super_block) -> *mut qnx4_sb_info {
    (*sb).s_fs_info as *mut qnx4_sb_info
}

#[inline]
pub unsafe fn qnx4_i(inode: *mut inode) -> *mut qnx4_inode_info {
    // C equivalent: container_of(inode, struct qnx4_inode_info, vfs_inode)
    (inode as *mut u8).sub(core::mem::offset_of!(qnx4_inode_info, vfs_inode))
        as *mut qnx4_inode_info
}

#[inline]
pub unsafe fn qnx4_raw_inode(inode: *mut inode) -> *mut qnx4_inode_entry {
    &mut (*qnx4_i(inode)).raw
}

/*
 * A qnx4 directory entry is an inode entry or link info
 * depending on the status field in the last byte. The
 * first byte is where the name start either way, and a
 * zero means it's empty.
 *
 * Also, due to a bug in gcc, we don't want to use the
 * real (differently sized) name arrays in the inode and
 * link entries, but always the 'de_name[]' one in the
 * fake struct entry.
 *
 * See
 *
 *   https://gcc.gnu.org/bugzilla/show_bug.cgi?id=99578#c6
 *
 * for details, but basically gcc will take the size of the
 * 'name' array from one of the used union entries randomly.
 *
 * This use of 'de_name[]' (48 bytes) avoids the false positive
 * warnings that would happen if gcc decides to use 'inode.di_name'
 * (16 bytes) even when the pointer and size were to come from
 * 'link.dl_name' (48 bytes).
 *
 * In all cases the actual name pointer itself is the same, it's
 * only the gcc internal 'what is the size of this field' logic
 * that can get confused.
 */
#[repr(C)]
pub struct qnx4_directory_entry_name {
    pub de_name: [core::ffi::c_char; 48],
    pub de_pad: [u8; 15],
    pub de_status: u8,
}

#[repr(C)]
pub union qnx4_directory_entry {
    pub de: qnx4_directory_entry_name,
    pub inode: qnx4_inode_entry,
    pub link: qnx4_link_info,
}

#[inline]
pub unsafe fn get_entry_fname(
    de: *mut qnx4_directory_entry,
    size: *mut core::ffi::c_int,
) -> *const core::ffi::c_char {
    // C BUILD_BUG_ON checks preserve the required layout invariants.
    if (*de).de.de_name[0] == 0 {
        return core::ptr::null();
    }
    if ((*de).de.de_status & (QNX4_FILE_USED | QNX4_FILE_LINK)) == 0 {
        return core::ptr::null();
    }
    if ((*de).de.de_status & QNX4_FILE_LINK) == 0 {
        *size = core::mem::size_of::<qnx4_inode_entry>() as core::ffi::c_int;
    } else {
        *size = core::mem::size_of::<qnx4_link_info>() as core::ffi::c_int;
    }

    let limit = *size as usize;
    let mut length = 0usize;
    while length < limit && (*de).de.de_name[length] != 0 {
        length += 1;
    }
    *size = length as core::ffi::c_int;

    (*de).de.de_name.as_ptr()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
