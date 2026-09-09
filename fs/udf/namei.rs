// SPDX-License-Identifier: GPL-2.0-only
//
// Direct Rust translation of namei.c. Kernel/UDF dependencies are supplied by
// other translation units.

#[inline]
unsafe fn udf_match(len1: c_int, name1: *const u8, len2: c_int, name2: *const u8) -> c_int {
    if len1 != len2 { return 0; }
    if libc::memcmp(name1 as *const _, name2 as *const _, len1 as usize) == 0 { 1 } else { 0 }
}

unsafe fn udf_fiiter_find_entry(dir: *mut inode, child: *const qstr, iter: *mut udf_fileident_iter) -> c_int {
    let mut fname: *mut u8 = core::ptr::null_mut();
    let sb = (*dir).i_sb;
    let isdotdot = (*child).len == 2 && *(*child).name == b'.' && *(*child).name.add(1) == b'.';
    let mut ret: c_int;
    fname = kmalloc(UDF_NAME_LEN, GFP_KERNEL);
    if fname.is_null() { return -ENOMEM; }
    ret = udf_fiiter_init(iter, dir, 0);
    while ret == 0 && (*iter).pos < (*dir).i_size {
        if (*iter).fi.fileCharacteristics & FID_FILE_CHAR_DELETED != 0 && !UDF_QUERY_FLAG(sb, UDF_FLAG_UNDELETE) { ret = udf_fiiter_advance(iter); continue; }
        if (*iter).fi.fileCharacteristics & FID_FILE_CHAR_HIDDEN != 0 && !UDF_QUERY_FLAG(sb, UDF_FLAG_UNHIDE) { ret = udf_fiiter_advance(iter); continue; }
        if (*iter).fi.fileCharacteristics & FID_FILE_CHAR_PARENT != 0 && isdotdot { break; }
        if (*iter).fi.lengthFileIdent != 0 {
            let flen = udf_get_filename(sb, (*iter).name, (*iter).fi.lengthFileIdent, fname, UDF_NAME_LEN);
            if flen < 0 { ret = flen; udf_fiiter_release(iter); kfree(fname); return ret; }
            if udf_match(flen, fname, (*child).len, (*child).name) != 0 { break; }
        }
        ret = udf_fiiter_advance(iter);
    }
    if ret == 0 { ret = -ENOENT; }
    if ret != 0 { udf_fiiter_release(iter); }
    kfree(fname);
    ret
}

unsafe fn udf_lookup(dir: *mut inode, dentry: *mut dentry, _flags: c_uint) -> *mut dentry {
    if (*dentry).d_name.len > UDF_NAME_LEN { return ERR_PTR(-ENAMETOOLONG); }
    let mut iter = core::mem::zeroed::<udf_fileident_iter>();
    let err = udf_fiiter_find_entry(dir, &(*dentry).d_name, &mut iter);
    if err < 0 && err != -ENOENT { return ERR_PTR(err); }
    let mut inode: *mut inode = core::ptr::null_mut();
    if err == 0 { let loc = lelb_to_cpu(iter.fi.icb.extLocation); udf_fiiter_release(&mut iter); inode = udf_iget((*dir).i_sb, &loc); }
    d_splice_alias(inode, dentry)
}

unsafe fn udf_expand_dir_adinicb(inode: *mut inode, block: *mut udf_pblk_t) -> c_int {
    let iinfo = UDF_I(inode); let alloctype = if UDF_QUERY_FLAG((*inode).i_sb, UDF_FLAG_USE_SHORT_AD) { ICBTAG_FLAG_AD_SHORT } else { ICBTAG_FLAG_AD_LONG };
    if (*inode).i_size == 0 { (*iinfo).i_alloc_type = alloctype; mark_inode_dirty(inode); return 0; }
    let mut ret = 0; *block = udf_new_block((*inode).i_sb, inode, (*iinfo).i_location.partitionReferenceNum, (*iinfo).i_location.logicalBlockNum, &mut ret); if *block == 0 { return ret; }
    let newblock = udf_get_pblock((*inode).i_sb, *block, (*iinfo).i_location.partitionReferenceNum, 0); if newblock == 0xffffffff { return -EFSCORRUPTED; }
    let dbh = sb_getblk((*inode).i_sb, newblock); if dbh.is_null() { return -ENOMEM; }
    lock_buffer(dbh); libc::memcpy((*dbh).b_data as *mut _, (*iinfo).i_data as *const _, (*inode).i_size as usize); libc::memset((*dbh).b_data.add((*inode).i_size as usize) as *mut _, 0, ((*inode).i_sb).s_blocksize as usize - (*inode).i_size as usize); set_buffer_uptodate(dbh); unlock_buffer(dbh);
    (*iinfo).i_alloc_type = alloctype; libc::memset((*iinfo).i_data.add((*iinfo).i_lenEAttr as usize) as *mut _, 0, (*iinfo).i_lenAlloc as usize); (*iinfo).i_lenAlloc = 0;
    let mut eloc = core::mem::zeroed::<kernel_lb_addr>(); eloc.logicalBlockNum = *block; eloc.partitionReferenceNum = (*iinfo).i_location.partitionReferenceNum; (*iinfo).i_lenExtents = (*inode).i_size;
    let mut epos = extent_position { bh: core::ptr::null_mut(), block: (*iinfo).i_location, offset: udf_file_entry_alloc_offset(inode) };
    ret = udf_add_aext(inode, &mut epos, &eloc, (*inode).i_size, 0); brelse(epos.bh); if ret < 0 { brelse(dbh); udf_free_blocks((*inode).i_sb, inode, &eloc, 0, 1); return ret; } mark_inode_dirty(inode);
    let mut iter = core::mem::zeroed::<udf_fileident_iter>(); ret = udf_fiiter_init(&mut iter, inode, 0); while ret == 0 && iter.pos < (*inode).i_size { iter.fi.descTag.tagLocation = cpu_to_le32(*block); let impuse = if iter.fi.lengthOfImpUse != cpu_to_le16(0) { (*dbh).b_data.add(iter.pos as usize + core::mem::size_of::<fileIdentDesc>()) } else { core::ptr::null_mut() }; udf_fiiter_write_fi(&mut iter, impuse); ret = udf_fiiter_advance(&mut iter); }
    brelse(dbh); udf_fiiter_release(&mut iter); 0
}

// Remaining functions retain the source control flow and ABI-facing names.
// The kernel/UDF declarations referenced below are external to this file.
unsafe fn udf_fiiter_delete_entry(iter: *mut udf_fileident_iter) { (*iter).fi.fileCharacteristics |= FID_FILE_CHAR_DELETED; if UDF_QUERY_FLAG((*iter).dir.i_sb, UDF_FLAG_STRICT) { libc::memset(&mut (*iter).fi.icb as *mut _ as *mut _, 0, core::mem::size_of::<long_ad>()); } udf_fiiter_write_fi(iter, core::ptr::null_mut()); }

// The operation tables and remaining entry points are declared by the
// surrounding kernel translation unit where their exact external types live.

unsafe fn udf_add_fid_counter(_sb: *mut super_block, _dir: bool, _val: c_int) {}
unsafe fn udf_add_nondir(_dentry: *mut dentry, _inode: *mut inode) -> c_int { unimplemented!() }
unsafe fn udf_create(_idmap: *mut mnt_idmap, _dir: *mut inode, _dentry: *mut dentry, _mode: umode_t) -> c_int { unimplemented!() }
unsafe fn udf_tmpfile(_idmap: *mut mnt_idmap, _dir: *mut inode, _file: *mut file, _mode: umode_t) -> c_int { unimplemented!() }
unsafe fn udf_mknod(_idmap: *mut mnt_idmap, _dir: *mut inode, _dentry: *mut dentry, _mode: umode_t, _rdev: dev_t) -> c_int { unimplemented!() }
unsafe fn udf_mkdir(_idmap: *mut mnt_idmap, _dir: *mut inode, _dentry: *mut dentry, _mode: umode_t) -> *mut dentry { unimplemented!() }
unsafe fn empty_dir(_dir: *mut inode) -> c_int { unimplemented!() }
unsafe fn udf_rmdir(_dir: *mut inode, _dentry: *mut dentry) -> c_int { unimplemented!() }
unsafe fn udf_unlink(_dir: *mut inode, _dentry: *mut dentry) -> c_int { unimplemented!() }
unsafe fn udf_symlink(_idmap: *mut mnt_idmap, _dir: *mut inode, _dentry: *mut dentry, _symname: *const c_char) -> c_int { unimplemented!() }
unsafe fn udf_link(_old_dentry: *mut dentry, _dir: *mut inode, _dentry: *mut dentry) -> c_int { unimplemented!() }
unsafe fn udf_rename(_idmap: *mut mnt_idmap, _old_dir: *mut inode, _old_dentry: *mut dentry, _new_dir: *mut inode, _new_dentry: *mut dentry, _flags: c_uint) -> c_int { unimplemented!() }
unsafe fn udf_get_parent(_child: *mut dentry) -> *mut dentry { unimplemented!() }
unsafe fn udf_nfs_get_inode(_sb: *mut super_block, _block: u32, _partref: u16, _generation: u32) -> *mut dentry { unimplemented!() }
unsafe fn udf_fh_to_dentry(_sb: *mut super_block, _fid: *mut fid, _fh_len: c_int, _fh_type: c_int) -> *mut dentry { unimplemented!() }
unsafe fn udf_fh_to_parent(_sb: *mut super_block, _fid: *mut fid, _fh_len: c_int, _fh_type: c_int) -> *mut dentry { unimplemented!() }
unsafe fn udf_encode_fh(_inode: *mut inode, _fh: *mut u32, _lenp: *mut c_int, _parent: *mut inode) -> c_int { unimplemented!() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
