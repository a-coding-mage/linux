// SPDX-License-Identifier: GPL-2.0
/*
 * fs/f2fs/verity.c: fs-verity support for f2fs
 *
 * Copyright 2019 Google LLC
 */

/*
 * Implementation of fsverity_operations for f2fs.
 *
 * Like ext4, f2fs stores the verity metadata (Merkle tree and
 * fsverity_descriptor) past the end of the file, starting at the first 64K
 * boundary beyond i_size.  This approach works because (a) verity files are
 * readonly, and (b) pages fully beyond i_size aren't visible to userspace but
 * can be read/written internally by f2fs with only some relatively small
 * changes to f2fs.  Extended attributes cannot be used because (a) f2fs limits
 * the total size of an inode's xattr entries to 4096 bytes, which wouldn't be
 * enough for even a single Merkle tree block, and (b) f2fs encryption doesn't
 * encrypt xattrs, yet the verity metadata *must* be encrypted when the file is
 * because it contains hashes of the plaintext data.
 *
 * Using a 64K boundary rather than a 4K one keeps things ready for
 * architectures with 64K pages, and it doesn't necessarily waste space on-disk
 * since there can be a hole between i_size and the start of the Merkle tree.
 */

// Dependencies supplied by the surrounding kernel translation unit.

pub const F2FS_VERIFY_VER: u32 = 1;

#[inline]
unsafe fn f2fs_verity_metadata_pos(inode: *const inode) -> loff_t {
    round_up((*inode).i_size, 65536)
}

/* Read some verity metadata from the inode. __vfs_read() can't be used because
 * we need to read beyond i_size. */
unsafe fn pagecache_read(mut inode: *mut inode, mut buf: *mut c_void,
                         mut count: usize, mut pos: loff_t) -> c_int {
    while count != 0 {
        let n = core::cmp::min(count, PAGE_SIZE - offset_in_page(pos));
        let page = read_mapping_page((*inode).i_mapping, pos >> PAGE_SHIFT, core::ptr::null_mut());
        if IS_ERR(page) {
            return PTR_ERR(page);
        }
        memcpy_from_page(buf, page, offset_in_page(pos), n);
        put_page(page);
        buf = (buf as *mut u8).add(n) as *mut c_void;
        pos += n as loff_t;
        count -= n;
    }
    0
}

/* Write some verity metadata to the inode for FS_IOC_ENABLE_VERITY.
 * kernel_write() can't be used because the file descriptor is readonly. */
unsafe fn pagecache_write(mut inode: *mut inode, mut buf: *const c_void,
                          mut count: usize, mut pos: loff_t) -> c_int {
    let mapping = (*inode).i_mapping;
    let aops = (*mapping).a_ops;
    if pos + count as loff_t > F2FS_BLK_TO_BYTES(max_file_blocks(inode)) {
        return -EFBIG;
    }
    while count != 0 {
        let n = core::cmp::min(count, PAGE_SIZE - offset_in_page(pos));
        let mut folio: *mut folio = core::ptr::null_mut();
        let mut fsdata: *mut c_void = core::ptr::null_mut();
        let res = ((*aops).write_begin)(core::ptr::null_mut(), mapping, pos, n,
                                        &mut folio, &mut fsdata);
        if res != 0 { return res; }
        memcpy_to_folio(folio, offset_in_folio(folio, pos), buf, n);
        let res = ((*aops).write_end)(core::ptr::null_mut(), mapping, pos, n, n,
                                      folio, fsdata);
        if res < 0 { return res; }
        if res != n as c_int { return -EIO; }
        buf = (buf as *const u8).add(n) as *const c_void;
        pos += n as loff_t;
        count -= n;
    }
    0
}

/* Format of f2fs verity xattr. */
#[repr(C)]
struct fsverity_descriptor_location {
    version: __le32,
    size: __le32,
    pos: __le64,
}

unsafe fn f2fs_begin_enable_verity(filp: *mut file) -> c_int {
    let inode = file_inode(filp);
    if f2fs_verity_in_progress(inode) { return -EBUSY; }
    if f2fs_is_atomic_file(inode) { return -EOPNOTSUPP; }
    let mut err = f2fs_dquot_initialize(inode);
    if err != 0 { return err; }
    err = f2fs_convert_inline_inode(inode);
    if err != 0 { return err; }
    set_inode_flag(inode, FI_VERITY_IN_PROGRESS);
    0
}

unsafe fn f2fs_end_enable_verity(filp: *mut file, desc: *const c_void,
                                 desc_size: usize, merkle_tree_size: u64) -> c_int {
    let inode = file_inode(filp);
    let sbi = F2FS_I_SB(inode);
    let desc_pos = f2fs_verity_metadata_pos(inode) as u64 + merkle_tree_size;
    let dloc = fsverity_descriptor_location {
        version: cpu_to_le32(F2FS_VERIFY_VER),
        size: cpu_to_le32(desc_size as u32),
        pos: cpu_to_le64(desc_pos),
    };
    let mut err = 0;
    let mut err2 = 0;
    if desc.is_null() { goto_cleanup!(); }
    err = pagecache_write(inode, desc, desc_size, desc_pos as loff_t);
    if err != 0 { goto_cleanup!(); }
    err = filemap_write_and_wait((*inode).i_mapping);
    if err != 0 { goto_cleanup!(); }
    err = f2fs_setxattr(inode, F2FS_XATTR_INDEX_VERITY, F2FS_XATTR_NAME_VERITY,
                        &dloc as *const _ as *const c_void, core::mem::size_of_val(&dloc),
                        core::ptr::null_mut(), XATTR_CREATE);
    if err != 0 { goto_cleanup!(); }
    file_set_verity(inode);
    f2fs_set_inode_flags(inode);
    f2fs_mark_inode_dirty_sync(inode, true);
    clear_inode_flag(inode, FI_VERITY_IN_PROGRESS);
    return 0;
cleanup:
    f2fs_down_write(&mut (*F2FS_I(inode)).i_gc_rwsem[WRITE]);
    truncate_inode_pages((*inode).i_mapping, (*inode).i_size);
    err2 = f2fs_truncate(inode);
    if err2 != 0 {
        f2fs_err(sbi, "Truncating verity metadata failed (errno=%d)", err2);
        set_sbi_flag(sbi, SBI_NEED_FSCK);
    }
    f2fs_up_write(&mut (*F2FS_I(inode)).i_gc_rwsem[WRITE]);
    clear_inode_flag(inode, FI_VERITY_IN_PROGRESS);
    if err != 0 { err } else { err2 }
}

unsafe fn f2fs_get_verity_descriptor(inode: *mut inode, buf: *mut c_void,
                                     buf_size: usize) -> c_int {
    let mut dloc: fsverity_descriptor_location = core::mem::zeroed();
    let res = f2fs_getxattr(inode, F2FS_XATTR_INDEX_VERITY, F2FS_XATTR_NAME_VERITY,
                            &mut dloc as *mut _ as *mut c_void, core::mem::size_of_val(&dloc),
                            core::ptr::null_mut());
    if res < 0 && res != -ERANGE { return res; }
    if res as usize != core::mem::size_of_val(&dloc) || dloc.version != cpu_to_le32(F2FS_VERIFY_VER) {
        f2fs_warn(F2FS_I_SB(inode), "unknown verity xattr format");
        return -EINVAL;
    }
    let size = le32_to_cpu(dloc.size);
    let pos = le64_to_cpu(dloc.pos);
    if pos.wrapping_add(size as u64) < pos || pos + size as u64 > F2FS_BLK_TO_BYTES(max_file_blocks(inode)) as u64 ||
       pos < f2fs_verity_metadata_pos(inode) as u64 || size > INT_MAX as u32 {
        f2fs_warn(F2FS_I_SB(inode), "invalid verity xattr");
        f2fs_handle_error(F2FS_I_SB(inode), ERROR_CORRUPTED_VERITY_XATTR);
        fserror_report_file_metadata(inode, -EFSCORRUPTED, GFP_NOFS);
        return -EFSCORRUPTED;
    }
    if buf_size != 0 {
        if size as usize > buf_size { return -ERANGE; }
        let res = pagecache_read(inode, buf, size as usize, pos as loff_t);
        if res != 0 { return res; }
    }
    size as c_int
}

unsafe fn f2fs_read_merkle_tree_page(inode: *mut inode, mut index: pgoff_t) -> *mut page {
    index += (f2fs_verity_metadata_pos(inode) >> PAGE_SHIFT) as pgoff_t;
    generic_read_merkle_tree_page(inode, index)
}

unsafe fn f2fs_readahead_merkle_tree(inode: *mut inode, mut index: pgoff_t, nr_pages: c_ulong) {
    index += (f2fs_verity_metadata_pos(inode) >> PAGE_SHIFT) as pgoff_t;
    generic_readahead_merkle_tree(inode, index, nr_pages);
}

unsafe fn f2fs_write_merkle_tree_block(file: *mut file, buf: *const c_void,
                                       pos: u64, size: c_uint) -> c_int {
    pagecache_write(file_inode(file), buf, size as usize,
                    (pos + f2fs_verity_metadata_pos(file_inode(file)) as u64) as loff_t)
}

#[no_mangle]
pub static f2fs_verityops: fsverity_operations = fsverity_operations {
    begin_enable_verity: Some(f2fs_begin_enable_verity),
    end_enable_verity: Some(f2fs_end_enable_verity),
    get_verity_descriptor: Some(f2fs_get_verity_descriptor),
    read_merkle_tree_page: Some(f2fs_read_merkle_tree_page),
    readahead_merkle_tree: Some(f2fs_readahead_merkle_tree),
    write_merkle_tree_block: Some(f2fs_write_merkle_tree_block),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
