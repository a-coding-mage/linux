// SPDX-License-Identifier: GPL-2.0
/*
 * fs/ext4/verity.c: fs-verity support for ext4
 *
 * Copyright 2019 Google LLC
 */

/*
 * Implementation of fsverity_operations for ext4.
 *
 * ext4 stores the verity metadata (Merkle tree and fsverity_descriptor) past
 * the end of the file, starting at the first 64K boundary beyond i_size.  This
 * approach works because (a) verity files are readonly, and (b) pages fully
 * beyond i_size aren't visible to userspace but can be read/written internally
 * by ext4 with only some relatively small changes to ext4.  This approach
 * avoids having to depend on the EA_INODE feature and on rearchitecturing
 * ext4's xattr support to support paging multi-gigabyte xattrs into memory, and
 * to support encrypting xattrs.  Note that the verity metadata *must* be
 * encrypted when the file is, since it contains hashes of the plaintext data.
 *
 * Using a 64K boundary rather than a 4K one keeps things ready for
 * architectures with 64K pages, and it doesn't necessarily waste space on-disk
 * since there can be a hole between i_size and the start of the Merkle tree.
 */

// Includes and symbols supplied by the surrounding kernel translation.

#[inline]
unsafe fn ext4_verity_metadata_pos(inode: *const inode) -> loff_t {
    round_up((*inode).i_size, 65536)
}

/*
 * Read some verity metadata from the inode.  __vfs_read() can't be used because
 * we need to read beyond i_size.
 */
unsafe fn pagecache_read(mut inode: *mut inode, mut buf: *mut core::ffi::c_void,
                         mut count: usize, mut pos: loff_t) -> c_int {
    while count != 0 {
        let folio: *mut folio = read_mapping_folio((*inode).i_mapping, pos >> PAGE_SHIFT, core::ptr::null_mut());
        if is_err(folio) { return ptr_err(folio); }
        let n = memcpy_from_file_folio(buf, folio, pos, count);
        folio_put(folio);
        buf = (buf as *mut u8).add(n) as *mut core::ffi::c_void;
        pos += n as loff_t;
        count -= n;
    }
    0
}

/*
 * Write some verity metadata to the inode for FS_IOC_ENABLE_VERITY.
 * kernel_write() can't be used because the file descriptor is readonly.
 */
unsafe fn pagecache_write(inode: *mut inode, mut buf: *const core::ffi::c_void,
                          mut count: usize, mut pos: loff_t) -> c_int {
    let mapping = (*inode).i_mapping;
    let aops = (*mapping).a_ops;
    if pos + count as loff_t > (*(*inode).i_sb).s_maxbytes { return -EFBIG; }
    while count != 0 {
        let n = core::cmp::min(count, PAGE_SIZE - offset_in_page(pos));
        let mut folio: *mut folio = core::ptr::null_mut();
        let mut fsdata: *mut core::ffi::c_void = core::ptr::null_mut();
        let mut res = ((*aops).write_begin)(core::ptr::null_mut(), mapping, pos, n, &mut folio, &mut fsdata);
        if res != 0 { return res; }
        memcpy_to_folio(folio, offset_in_folio(folio, pos), buf, n);
        res = ((*aops).write_end)(core::ptr::null_mut(), mapping, pos, n, n, folio, fsdata);
        if res < 0 { return res; }
        if res as usize != n { return -EIO; }
        buf = (buf as *const u8).add(n) as *const core::ffi::c_void;
        pos += n as loff_t;
        count -= n;
    }
    0
}

unsafe fn ext4_begin_enable_verity(filp: *mut file) -> c_int {
    let inode = file_inode(filp);
    let credits: c_int = 2;
    let handle: *mut handle_t;
    let mut err;
    if is_dax(inode) || ext4_test_inode_flag(inode, EXT4_INODE_DAX) { return -EINVAL; }
    if ext4_verity_in_progress(inode) { return -EBUSY; }
    err = ext4_inode_attach_jinode(inode); if err != 0 { return err; }
    err = dquot_initialize(inode); if err != 0 { return err; }
    err = ext4_convert_inline_data(inode); if err != 0 { return err; }
    if !ext4_test_inode_flag(inode, EXT4_INODE_EXTENTS) {
        ext4_warning_inode(inode, "verity is only allowed on extent-based files");
        return -EOPNOTSUPP;
    }
    err = ext4_truncate(inode); if err != 0 { return err; }
    handle = ext4_journal_start(inode, EXT4_HT_INODE, credits);
    if is_err(handle) { return ptr_err(handle); }
    err = ext4_orphan_add(handle, inode);
    if err == 0 { ext4_set_inode_state(inode, EXT4_STATE_VERITY_IN_PROGRESS); }
    ext4_journal_stop(handle); err
}

unsafe fn ext4_write_verity_descriptor(inode: *mut inode, desc: *const core::ffi::c_void,
                                       desc_size: usize, merkle_tree_size: u64) -> c_int {
    let desc_pos = round_up(ext4_verity_metadata_pos(inode) + merkle_tree_size as loff_t, i_blocksize(inode));
    let desc_end = desc_pos + desc_size as u64;
    let desc_size_disk: __le32 = cpu_to_le32(desc_size as u32);
    let desc_size_pos = round_up(desc_end + core::mem::size_of::<__le32>() as u64, i_blocksize(inode)) - core::mem::size_of::<__le32>() as u64;
    let err = pagecache_write(inode, desc, desc_size, desc_pos as loff_t);
    if err != 0 { return err; }
    pagecache_write(inode, &desc_size_disk as *const _ as *const core::ffi::c_void, core::mem::size_of::<__le32>(), desc_size_pos as loff_t)
}

unsafe fn ext4_end_enable_verity(filp: *mut file, desc: *const core::ffi::c_void,
                                  desc_size: usize, merkle_tree_size: u64) -> c_int {
    let inode = file_inode(filp);
    let credits: c_int = 2;
    let mut err = 0;
    if desc.is_null() { goto_cleanup(inode, err); }
    err = ext4_write_verity_descriptor(inode, desc, desc_size, merkle_tree_size);
    if err != 0 { goto_cleanup(inode, err); }
    err = filemap_write_and_wait((*inode).i_mapping);
    if err != 0 { goto_cleanup(inode, err); }
    let handle = ext4_journal_start(inode, EXT4_HT_INODE, credits);
    if is_err(handle) { err = ptr_err(handle); goto_cleanup(inode, err); }
    ext4_fc_mark_ineligible((*inode).i_sb, EXT4_FC_REASON_VERITY, handle);
    err = ext4_orphan_del(handle, inode); if err != 0 { ext4_journal_stop(handle); goto_cleanup(inode, err); }
    let mut iloc = core::mem::MaybeUninit::<ext4_iloc>::uninit();
    err = ext4_reserve_inode_write(handle, inode, iloc.as_mut_ptr());
    if err != 0 { ext4_journal_stop(handle); goto_cleanup(inode, err); }
    ext4_set_inode_flag(inode, EXT4_INODE_VERITY); ext4_set_inode_flags(inode, false);
    err = ext4_mark_iloc_dirty(handle, inode, iloc.as_mut_ptr());
    if err != 0 { ext4_journal_stop(handle); goto_cleanup(inode, err); }
    ext4_journal_stop(handle); ext4_clear_inode_state(inode, EXT4_STATE_VERITY_IN_PROGRESS); return 0;
    unsafe fn goto_cleanup(inode: *mut inode, err: c_int) -> c_int {
        truncate_inode_pages((*inode).i_mapping, (*inode).i_size); ext4_truncate(inode); ext4_orphan_del(core::ptr::null_mut(), inode); ext4_clear_inode_state(inode, EXT4_STATE_VERITY_IN_PROGRESS); err
    }
}

unsafe fn ext4_get_verity_descriptor_location(inode: *mut inode, desc_size_ret: *mut usize, desc_pos_ret: *mut u64) -> c_int {
    let path = ext4_find_extent(inode, EXT_MAX_BLOCKS - 1, core::ptr::null_mut(), 0);
    if is_err(path) { return ptr_err(path); }
    let last_extent = (*path.add((*path).p_depth as usize)).p_ext;
    if last_extent.is_null() { ext4_free_ext_path(path); return -EFSCORRUPTED; }
    let end_lblk = le32_to_cpu((*last_extent).ee_block) + ext4_ext_get_actual_len(last_extent);
    let mut desc_size_pos = EXT4_LBLK_TO_B(inode, end_lblk);
    ext4_free_ext_path(path);
    if desc_size_pos < core::mem::size_of::<__le32>() as u64 { return -EFSCORRUPTED; }
    desc_size_pos -= core::mem::size_of::<__le32>() as u64;
    let mut disk: __le32 = 0;
    let err = pagecache_read(inode, &mut disk as *mut _ as *mut core::ffi::c_void, core::mem::size_of::<__le32>(), desc_size_pos as loff_t);
    if err != 0 { return err; }
    let desc_size = le32_to_cpu(disk) as usize;
    if desc_size > INT_MAX as usize || desc_size as u64 > desc_size_pos { return -EFSCORRUPTED; }
    let desc_pos = round_down(desc_size_pos - desc_size as u64, i_blocksize(inode));
    if desc_pos < ext4_verity_metadata_pos(inode) { return -EFSCORRUPTED; }
    *desc_size_ret = desc_size; *desc_pos_ret = desc_pos; 0
}

unsafe fn ext4_get_verity_descriptor(inode: *mut inode, buf: *mut core::ffi::c_void, buf_size: usize) -> c_int {
    let mut desc_size = 0usize; let mut desc_pos = 0u64;
    let err = ext4_get_verity_descriptor_location(inode, &mut desc_size, &mut desc_pos);
    if err != 0 { return err; }
    if buf_size != 0 { if desc_size > buf_size { return -ERANGE; } let err = pagecache_read(inode, buf, desc_size, desc_pos as loff_t); if err != 0 { return err; } }
    desc_size as c_int
}

unsafe fn ext4_read_merkle_tree_page(inode: *mut inode, mut index: pgoff_t) -> *mut page {
    index += (ext4_verity_metadata_pos(inode) >> PAGE_SHIFT) as pgoff_t;
    generic_read_merkle_tree_page(inode, index)
}

unsafe fn ext4_readahead_merkle_tree(inode: *mut inode, mut index: pgoff_t, nr_pages: c_ulong) {
    index += (ext4_verity_metadata_pos(inode) >> PAGE_SHIFT) as pgoff_t;
    generic_readahead_merkle_tree(inode, index, nr_pages);
}
unsafe fn ext4_write_merkle_tree_block(file: *mut file, buf: *const core::ffi::c_void, pos: u64, size: c_uint) -> c_int {
    pagecache_write(file_inode(file), buf, size as usize, (pos as loff_t) + ext4_verity_metadata_pos(file_inode(file)))
}

pub static ext4_verityops: fsverity_operations = fsverity_operations {
    begin_enable_verity: Some(ext4_begin_enable_verity), end_enable_verity: Some(ext4_end_enable_verity),
    get_verity_descriptor: Some(ext4_get_verity_descriptor), read_merkle_tree_page: Some(ext4_read_merkle_tree_page),
    readahead_merkle_tree: Some(ext4_readahead_merkle_tree), write_merkle_tree_block: Some(ext4_write_merkle_tree_block),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
