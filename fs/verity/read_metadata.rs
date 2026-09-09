// SPDX-License-Identifier: GPL-2.0-only
/*
 * Ioctl to read verity metadata
 *
 * Copyright 2021 Google LLC
 */

// Dependencies supplied by the kernel and fsverity headers are intentionally
// referenced here rather than redefined.

use core::ffi::c_void;

unsafe fn fsverity_read_merkle_tree(
    inode: *mut inode,
    vi: *const fsverity_info,
    mut buf: *mut c_void,
    mut offset: u64,
    length: i32,
) -> i32 {
    let vops = (*(*inode).i_sb).s_vop;
    let end_offset = core::cmp::min(offset.wrapping_add(length as u64), (*vi).tree_params.tree_size);
    let mut retval: i32 = 0;
    let mut err: i32 = 0;

    if offset >= end_offset {
        return 0;
    }
    let mut offs_in_page = offset_in_page(offset);
    let mut index = offset >> PAGE_SHIFT;
    let last_index = (end_offset - 1) >> PAGE_SHIFT;

    /*
     * Kick off readahead for the range we are going to read to ensure a
     * single large sequential read instead of lots of small ones.
     */
    if !(*vops).readahead_merkle_tree.is_none() {
        filemap_invalidate_lock_shared((*inode).i_mapping);
        ((*vops).readahead_merkle_tree.unwrap())(
            inode, index, last_index - index + 1,
        );
        filemap_invalidate_unlock_shared((*inode).i_mapping);
    }

    /*
     * Iterate through each Merkle tree page in the requested range and copy
     * the requested portion to userspace.  Note that the Merkle tree block
     * size isn't important here, as we are returning a byte stream; i.e.,
     * we can just work with pages even if the tree block size != PAGE_SIZE.
     */
    while index <= last_index {
        let bytes_to_copy = core::cmp::min(
            end_offset - offset,
            (PAGE_SIZE - offs_in_page) as u64,
        ) as usize;
        let page = ((*vops).read_merkle_tree_page)(inode, index);
        if IS_ERR(page) {
            err = PTR_ERR(page);
            fsverity_err(inode, b"Error %d reading Merkle tree page %lu\0".as_ptr(), err, index);
            break;
        }

        let virt = kmap_local_page(page);
        if copy_to_user(buf, (virt as *const u8).add(offs_in_page as usize), bytes_to_copy) != 0 {
            kunmap_local(virt);
            put_page(page);
            err = -EFAULT;
            break;
        }
        kunmap_local(virt);
        put_page(page);

        retval += bytes_to_copy as i32;
        buf = (buf as *mut u8).add(bytes_to_copy) as *mut c_void;
        offset += bytes_to_copy as u64;

        if fatal_signal_pending(current) {
            err = -EINTR;
            break;
        }
        cond_resched();
        offs_in_page = 0;
        index += 1;
    }
    if retval != 0 { retval } else { err }
}

/* Copy the requested portion of the buffer to userspace. */
unsafe fn fsverity_read_buffer(
    mut dst: *mut c_void,
    offset: u64,
    mut length: i32,
    mut src: *const c_void,
    mut src_length: usize,
) -> i32 {
    if offset >= src_length as u64 { return 0; }
    src = (src as *const u8).add(offset as usize) as *const c_void;
    src_length -= offset as usize;
    length = core::cmp::min(length as usize, src_length) as i32;
    if copy_to_user(dst, src, length as usize) != 0 { return -EFAULT; }
    length
}

unsafe fn fsverity_read_descriptor(
    inode: *mut inode, buf: *mut c_void, offset: u64, length: i32,
) -> i32 {
    let mut desc: *mut fsverity_descriptor = core::ptr::null_mut();
    let res = fsverity_get_descriptor(inode, &mut desc);
    if res != 0 { return res; }

    /* don't include the builtin signature */
    let desc_size = core::mem::offset_of!(fsverity_descriptor, signature);
    (*desc).sig_size = 0;
    let res = fsverity_read_buffer(buf, offset, length, desc as *const c_void, desc_size);
    kfree(desc as *mut c_void);
    res
}

unsafe fn fsverity_read_signature(
    inode: *mut inode, buf: *mut c_void, offset: u64, length: i32,
) -> i32 {
    let mut desc: *mut fsverity_descriptor = core::ptr::null_mut();
    let mut res = fsverity_get_descriptor(inode, &mut desc);
    if res != 0 { return res; }
    if (*desc).sig_size == 0 {
        res = -ENODATA;
    } else {
        /*
         * Include only the builtin signature.  fsverity_get_descriptor()
         * already verified that sig_size is in-bounds.
         */
        res = fsverity_read_buffer(
            buf, offset, length, (*desc).signature.as_ptr() as *const c_void,
            le32_to_cpu((*desc).sig_size) as usize,
        );
    }
    kfree(desc as *mut c_void);
    res
}

/**
 * fsverity_ioctl_read_metadata() - read verity metadata from a file
 * @filp: file to read the metadata from
 * @uarg: user pointer to fsverity_read_metadata_arg
 *
 * Return: length read on success, 0 on EOF, -errno on failure
 */
pub unsafe fn fsverity_ioctl_read_metadata(
    filp: *mut file, uarg: *const c_void,
) -> i32 {
    let inode = file_inode(filp);
    let vi = fsverity_get_info(inode);
    if vi.is_null() { return -ENODATA; /* not a verity file */ }

    let mut arg: fsverity_read_metadata_arg = core::mem::zeroed();
    if copy_from_user(&mut arg as *mut _ as *mut c_void, uarg, core::mem::size_of_val(&arg)) != 0 {
        return -EFAULT;
    }
    if arg.__reserved != 0 { return -EINVAL; }
    /* offset + length must not overflow. */
    if arg.offset.wrapping_add(arg.length) < arg.offset { return -EINVAL; }
    /* Ensure that the return value will fit in INT_MAX. */
    let length = core::cmp::min(arg.length, INT_MAX as u64) as i32;
    let buf = u64_to_user_ptr(arg.buf_ptr);

    match arg.metadata_type {
        FS_VERITY_METADATA_TYPE_MERKLE_TREE => fsverity_read_merkle_tree(inode, vi, buf, arg.offset, length),
        FS_VERITY_METADATA_TYPE_DESCRIPTOR => fsverity_read_descriptor(inode, buf, arg.offset, length),
        FS_VERITY_METADATA_TYPE_SIGNATURE => fsverity_read_signature(inode, buf, arg.offset, length),
        _ => -EINVAL,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
