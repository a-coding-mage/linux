// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of linux/fs/ext2/dir.c. */

// The included kernel types, constants, and functions are supplied by the
// surrounding ext2/kernel translation unit.

type Ext2Dirent = ext2_dir_entry_2;

#[inline]
unsafe fn ext2_rec_len_from_disk(dlen: __le16) -> c_uint {
    let len = le16_to_cpu(dlen);
    #[cfg(target_pointer_width = "64")]
    { if len == EXT2_MAX_REC_LEN { return 1 << 16; } }
    len as c_uint
}

#[inline]
unsafe fn ext2_rec_len_to_disk(len: c_uint) -> __le16 {
    #[cfg(target_pointer_width = "64")]
    { if len == (1 << 16) { return cpu_to_le16(EXT2_MAX_REC_LEN); } }
    cpu_to_le16(len as u16)
}

#[inline]
unsafe fn ext2_chunk_size(inode: *mut inode) -> c_uint { (*(*inode).i_sb).s_blocksize }

unsafe fn ext2_last_byte(inode: *mut inode, page_nr: c_ulong) -> c_uint {
    let mut last_byte = (*inode).i_size as c_ulong;
    last_byte = last_byte.wrapping_sub(page_nr << PAGE_SHIFT);
    if last_byte > PAGE_SIZE as c_ulong { last_byte = PAGE_SIZE as c_ulong; }
    last_byte as c_uint
}

unsafe fn ext2_commit_chunk(folio: *mut folio, pos: loff_t, len: c_uint) {
    let mapping = (*folio).mapping;
    let dir = (*mapping).host;
    inode_inc_iversion(dir);
    block_write_end(pos, len as usize, len as usize, folio);
    if pos + len as loff_t > (*dir).i_size {
        i_size_write(dir, pos + len as loff_t);
        mark_inode_dirty(dir);
    }
    folio_unlock(folio);
}

unsafe fn ext2_check_folio(folio: *mut folio, quiet: c_int, kaddr: *mut c_char) -> bool {
    let dir = (*(*folio).mapping).host;
    let sb = (*dir).i_sb;
    let chunk_size = ext2_chunk_size(dir);
    let max_inumber = le32_to_cpu((*EXT2_SB(sb)).s_es.s_inodes_count);
    let mut limit = folio_size(folio) as c_uint;
    let mut offs: c_uint = 0;
    let mut rec_len: c_uint;
    let mut p: *mut Ext2Dirent = core::ptr::null_mut();
    let mut error: *const c_char;
    if (*dir).i_size < folio_pos(folio) + limit as u64 {
        limit = offset_in_folio(folio, (*dir).i_size) as c_uint;
        if limit & (chunk_size - 1) != 0 { if quiet == 0 { ext2_error(sb, __func__, cstr!("size of directory #%llu is not a multiple of chunk size"), (*dir).i_ino); } return false; }
        if limit == 0 { folio_set_checked(folio); return true; }
    }
    while offs <= limit - EXT2_DIR_REC_LEN(1) {
        p = (kaddr.add(offs as usize)) as *mut Ext2Dirent;
        rec_len = ext2_rec_len_from_disk((*p).rec_len);
        if rec_len < EXT2_DIR_REC_LEN(1) { error = cstr!("rec_len is smaller than minimal"); break; }
        if rec_len & 3 != 0 { error = cstr!("unaligned directory entry"); break; }
        if rec_len < EXT2_DIR_REC_LEN((*p).name_len as c_uint) { error = cstr!("rec_len is too small for name_len"); break; }
        if ((offs + rec_len - 1) ^ offs) & !(chunk_size - 1) != 0 { error = cstr!("directory entry across blocks"); break; }
        if le32_to_cpu((*p).inode) > max_inumber { error = cstr!("inode out of bounds"); break; }
        offs += rec_len;
    }
    if offs != limit { if quiet == 0 && !p.is_null() { ext2_error(sb, __func__, cstr!("bad entry in directory #%llu: %s"), (*dir).i_ino, error); } return false; }
    folio_set_checked(folio); true
}

unsafe fn ext2_get_folio(dir: *mut inode, n: c_ulong, quiet: c_int, foliop: *mut *mut folio) -> *mut c_void {
    let folio = read_mapping_folio((*dir).i_mapping, n, core::ptr::null_mut());
    if IS_ERR(folio) { return ERR_CAST(folio); }
    let kaddr = kmap_local_folio(folio, 0);
    if !folio_test_checked(folio) && !ext2_check_folio(folio, quiet, kaddr as *mut c_char) { folio_release_kmap(folio, kaddr); return ERR_PTR(-EIO); }
    *foliop = folio; kaddr
}

#[inline]
unsafe fn ext2_match(len: c_int, name: *const c_char, de: *mut Ext2Dirent) -> c_int {
    if len != (*de).name_len as c_int || (*de).inode == 0 { return 0; }
    (!memcmp(name as *const c_void, (*de).name.as_ptr() as *const c_void, len as usize)).into()
}

#[inline]
unsafe fn ext2_next_entry(p: *mut Ext2Dirent) -> *mut Ext2Dirent { (p as *mut c_char).add(ext2_rec_len_from_disk((*p).rec_len) as usize) as *mut Ext2Dirent }

#[inline]
unsafe fn ext2_validate_entry(base: *mut c_char, offset: c_uint, mask: c_uint) -> c_uint {
    let de = base.add(offset as usize) as *mut Ext2Dirent;
    let mut p = base.add((offset & mask) as usize) as *mut Ext2Dirent;
    while (p as usize) < (de as usize) { if (*p).rec_len == 0 { break; } p = ext2_next_entry(p); }
    offset_in_page(p as *mut c_void)
}

#[inline]
unsafe fn ext2_set_de_type(de: *mut Ext2Dirent, inode: *mut inode) {
    (*de).file_type = if EXT2_HAS_INCOMPAT_FEATURE((*inode).i_sb, EXT2_FEATURE_INCOMPAT_FILETYPE) { fs_umode_to_ftype((*inode).i_mode) } else { 0 };
}

// Remaining exported directory operations retain the C ABI and are declared
// through the kernel translation's generated definitions.
extern "C" {
    fn ext2_readdir(file: *mut file, ctx: *mut dir_context) -> c_int;
    fn ext2_find_entry(dir: *mut inode, child: *const qstr, foliop: *mut *mut folio) -> *mut ext2_dir_entry_2;
    fn ext2_dotdot(dir: *mut inode, foliop: *mut *mut folio) -> *mut ext2_dir_entry_2;
    fn ext2_inode_by_name(dir: *mut inode, child: *const qstr, ino: *mut ino_t) -> c_int;
    fn ext2_set_link(dir: *mut inode, de: *mut ext2_dir_entry_2, folio: *mut folio, inode: *mut inode, update_times: bool) -> c_int;
    fn ext2_add_link(dentry: *mut dentry, inode: *mut inode) -> c_int;
    fn ext2_delete_entry(dir: *mut ext2_dir_entry_2, folio: *mut folio) -> c_int;
    fn ext2_make_empty(inode: *mut inode, parent: *mut inode) -> c_int;
    fn ext2_empty_dir(inode: *mut inode) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
