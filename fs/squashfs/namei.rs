// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Squashfs - a compressed read only filesystem for Linux
 *
 * Copyright (c) 2002, 2003, 2004, 2005, 2006, 2007, 2008
 * Phillip Lougher <phillip@squashfs.org.uk>
 *
 * namei.c
 */

/* This file implements code to do filename lookup in directories. */

// Linux and Squashfs declarations supplied by the surrounding translation unit.

unsafe fn get_dir_index_using_name(
    sb: *mut super_block,
    next_block: *mut u64,
    next_offset: *mut i32,
    mut index_start: u64,
    mut index_offset: i32,
    i_count: i32,
    name: *const i8,
) -> i32 {
    let msblk = (*sb).s_fs_info as *mut squashfs_sb_info;
    let mut length: i32 = 0;
    let mut index: *mut squashfs_dir_index;

    TRACE!("Entered get_dir_index_using_name, i_count %d\\n", i_count);

    index = kmalloc(
        core::mem::size_of::<squashfs_dir_index>() + SQUASHFS_NAME_LEN + 1,
        GFP_KERNEL,
    ) as *mut squashfs_dir_index;
    if index.is_null() {
        ERROR!("Failed to allocate squashfs_dir_index\\n");
        return length + 3;
    }

    for _i in 0..i_count {
        let err = squashfs_read_metadata(
            sb,
            index as *mut core::ffi::c_void,
            &mut index_start,
            &mut index_offset,
            core::mem::size_of::<squashfs_dir_index>(),
        );
        if err < 0 {
            break;
        }

        let size = le32_to_cpu((*index).size) + 1;
        if size > SQUASHFS_NAME_LEN {
            break;
        }

        let err = squashfs_read_metadata(
            sb,
            (*index).name.as_mut_ptr() as *mut core::ffi::c_void,
            &mut index_start,
            &mut index_offset,
            size as usize,
        );
        if err < 0 {
            break;
        }

        (*index).name[size as usize] = 0;
        if strcmp((*index).name.as_ptr(), name) > 0 {
            break;
        }

        length = le32_to_cpu((*index).index) as i32;
        *next_block = le32_to_cpu((*index).start_block) as u64 + (*msblk).directory_table;
    }

    *next_offset = (length + *next_offset) % SQUASHFS_METADATA_SIZE;
    kfree(index as *mut core::ffi::c_void);

    /* Return the external f_pos, offset by 3 for invented "." and ".." entries. */
    length + 3
}

unsafe fn squashfs_lookup(
    dir: *mut inode,
    dentry: *mut dentry,
    _flags: c_uint,
) -> *mut dentry {
    let name = (*dentry).d_name.name;
    let len = (*dentry).d_name.len;
    let mut inode: *mut inode = core::ptr::null_mut();
    let msblk = (*(*dir).i_sb).s_fs_info as *mut squashfs_sb_info;
    let mut dirh: squashfs_dir_header = core::mem::zeroed();
    let mut dire: *mut squashfs_dir_entry;
    let mut block = squashfs_i(dir).start + (*msblk).directory_table;
    let mut offset = squashfs_i(dir).offset;
    let mut err: i32;
    let mut length: i32;
    let mut dir_count: u32;
    let mut size: u32;

    TRACE!("Entered squashfs_lookup [%llx:%x]\\n", block, offset);

    dire = kmalloc(
        core::mem::size_of::<squashfs_dir_entry>() + SQUASHFS_NAME_LEN + 1,
        GFP_KERNEL,
    ) as *mut squashfs_dir_entry;
    if dire.is_null() {
        ERROR!("Failed to allocate squashfs_dir_entry\\n");
        return ERR_PTR(-ENOMEM);
    }

    if len > SQUASHFS_NAME_LEN {
        err = -ENAMETOOLONG;
        kfree(dire as *mut core::ffi::c_void);
        return ERR_PTR(err);
    }

    length = get_dir_index_using_name(
        (*dir).i_sb,
        &mut block,
        &mut offset,
        squashfs_i(dir).dir_idx_start,
        squashfs_i(dir).dir_idx_offset,
        squashfs_i(dir).dir_idx_cnt,
        name,
    );

    while length < i_size_read(dir) {
        err = squashfs_read_metadata(
            (*dir).i_sb,
            &mut dirh as *mut squashfs_dir_header as *mut core::ffi::c_void,
            &mut block,
            &mut offset,
            core::mem::size_of::<squashfs_dir_header>(),
        );
        if err < 0 {
            goto_read_failure(dir, msblk, dire, err);
        }
        length += core::mem::size_of::<squashfs_dir_header>() as i32;
        dir_count = le32_to_cpu(dirh.count) + 1;
        if dir_count > SQUASHFS_DIR_COUNT {
            goto_data_error(dir, msblk, dire);
        }

        while dir_count != 0 {
            dir_count -= 1;
            err = squashfs_read_metadata(
                (*dir).i_sb,
                dire as *mut core::ffi::c_void,
                &mut block,
                &mut offset,
                core::mem::size_of::<squashfs_dir_entry>(),
            );
            if err < 0 {
                goto_read_failure(dir, msblk, dire, err);
            }
            size = le16_to_cpu((*dire).size) as u32 + 1;
            if size > SQUASHFS_NAME_LEN {
                goto_data_error(dir, msblk, dire);
            }
            err = squashfs_read_metadata(
                (*dir).i_sb,
                (*dire).name.as_mut_ptr() as *mut core::ffi::c_void,
                &mut block,
                &mut offset,
                size as usize,
            );
            if err < 0 {
                goto_read_failure(dir, msblk, dire, err);
            }
            length += core::mem::size_of::<squashfs_dir_entry>() as i32 + size as i32;
            if *name < (*dire).name[0] {
                break;
            }
            if len == size as usize && strncmp(name, (*dire).name.as_ptr(), len) == 0 {
                let blk = le32_to_cpu(dirh.start_block);
                let off = le16_to_cpu((*dire).offset);
                let ino_num = le32_to_cpu(dirh.inode_number)
                    .wrapping_add((le16_to_cpu((*dire).inode_number) as i16) as u32);
                let ino = SQUASHFS_MKINODE(blk, off);
                TRACE!("calling squashfs_iget for directory entry %s, inode  %x:%x, %d\\n", name, blk, off, ino_num);
                inode = squashfs_iget((*dir).i_sb, ino, ino_num);
                dir_count = 0;
                break;
            }
        }
        if !inode.is_null() || length >= i_size_read(dir) {
            break;
        }
    }

    kfree(dire as *mut core::ffi::c_void);
    d_splice_alias(inode, dentry)
}

// Error exits corresponding to the C function's labels.
unsafe fn goto_data_error(dir: *mut inode, msblk: *mut squashfs_sb_info, dire: *mut squashfs_dir_entry) -> ! {
    goto_read_failure(dir, msblk, dire, -EIO)
}

unsafe fn goto_read_failure(dir: *mut inode, msblk: *mut squashfs_sb_info, dire: *mut squashfs_dir_entry, err: i32) -> ! {
    ERROR!("Unable to read directory block [%llx:%x]\\n", squashfs_i(dir).start + (*msblk).directory_table, squashfs_i(dir).offset);
    kfree(dire as *mut core::ffi::c_void);
    ERR_PTR(err)
}

#[repr(C)]
pub struct inode_operations {
    pub lookup: Option<unsafe fn(*mut inode, *mut dentry, c_uint) -> *mut dentry>,
    pub listxattr: Option<unsafe fn() -> isize>,
}

#[no_mangle]
pub static squashfs_dir_inode_ops: inode_operations = inode_operations {
    .lookup: Some(squashfs_lookup),
    .listxattr: Some(squashfs_listxattr),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
