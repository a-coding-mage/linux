// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/fs/adfs/dir.c
 *
 *  Copyright (C) 1999-2000 Russell King
 *
 *  Common directory handling for ADFS
 */

// Kernel headers and symbols supplied by the surrounding translation unit.

/* For future.  This should probably be per-directory. */
static mut ADFS_DIR_RWSEM: RwSemaphore = RwSemaphore::new();

pub unsafe fn adfs_dir_copyfrom(dst: *mut core::ffi::c_void, dir: *mut adfs_dir,
                                mut offset: u32, mut len: usize) -> i32 {
    let sb = (*dir).sb;
    let mut index = offset >> (*sb).s_blocksize_bits;
    offset &= (*sb).s_blocksize - 1;
    let remain = (*sb).s_blocksize - offset;
    if index + (remain < len as u32) as u32 >= (*dir).nr_buffers { return -EINVAL; }
    let mut dst = dst as *mut u8;
    if remain < len as u32 {
        core::ptr::copy_nonoverlapping((*(*dir).bhs.add(index as usize)).b_data.add(offset as usize), dst, remain as usize);
        dst = dst.add(remain as usize); len -= remain as usize; index += 1; offset = 0;
    }
    core::ptr::copy_nonoverlapping((*(*dir).bhs.add(index as usize)).b_data.add(offset as usize), dst, len);
    0
}

pub unsafe fn adfs_dir_copyto(dir: *mut adfs_dir, mut offset: u32, src: *const core::ffi::c_void,
                              mut len: usize) -> i32 {
    let sb = (*dir).sb;
    let mut index = offset >> (*sb).s_blocksize_bits;
    offset &= (*sb).s_blocksize - 1;
    let remain = (*sb).s_blocksize - offset;
    if index + (remain < len as u32) as u32 >= (*dir).nr_buffers { return -EINVAL; }
    let mut src = src as *const u8;
    if remain < len as u32 {
        core::ptr::copy_nonoverlapping(src, (*(*dir).bhs.add(index as usize)).b_data.add(offset as usize), remain as usize);
        src = src.add(remain as usize); len -= remain as usize; index += 1; offset = 0;
    }
    core::ptr::copy_nonoverlapping(src, (*(*dir).bhs.add(index as usize)).b_data.add(offset as usize), len);
    0
}

unsafe fn __adfs_dir_cleanup(dir: *mut adfs_dir) {
    (*dir).nr_buffers = 0;
    if (*dir).bhs != (*dir).bh.as_mut_ptr() { kfree((*dir).bhs as *mut core::ffi::c_void); }
    (*dir).bhs = core::ptr::null_mut(); (*dir).sb = core::ptr::null_mut();
}

pub unsafe fn adfs_dir_relse(dir: *mut adfs_dir) {
    for i in 0..(*dir).nr_buffers { brelse(*(*dir).bhs.add(i as usize)); }
    __adfs_dir_cleanup(dir);
}

unsafe fn adfs_dir_forget(dir: *mut adfs_dir) {
    for i in 0..(*dir).nr_buffers { bforget(*(*dir).bhs.add(i as usize)); }
    __adfs_dir_cleanup(dir);
}

pub unsafe fn adfs_dir_read_buffers(sb: *mut super_block, indaddr: u32, size: u32, dir: *mut adfs_dir) -> i32 {
    let num = (ALIGN(size, (*sb).s_blocksize) >> (*sb).s_blocksize_bits) as u32;
    let mut bhs: *mut *mut buffer_head;
    if num > (*dir).bh.len() as u32 {
        if (*dir).bhs != (*dir).bh.as_mut_ptr() { return -EINVAL; }
        bhs = kzalloc_objs::<*mut buffer_head>(num as usize);
        if bhs.is_null() { return -ENOMEM; }
        if (*dir).nr_buffers != 0 { core::ptr::copy_nonoverlapping((*dir).bhs, bhs, (*dir).nr_buffers as usize); }
        (*dir).bhs = bhs;
    }
    for i in (*dir).nr_buffers..num {
        let block = __adfs_block_map(sb, indaddr, i);
        if block == 0 { adfs_error(sb, c_str!("dir %06x has a hole at offset %u"), indaddr, i); adfs_dir_relse(dir); return -EIO; }
        *(*dir).bhs.add(i as usize) = sb_bread(sb, block);
        if (*(*dir).bhs.add(i as usize)).is_null() { adfs_error(sb, c_str!("dir %06x failed read at offset %u, mapped block 0x%08x"), indaddr, i, block); adfs_dir_relse(dir); return -EIO; }
        (*dir).nr_buffers += 1;
    }
    0
}

unsafe fn adfs_dir_read(sb: *mut super_block, indaddr: u32, size: u32, dir: *mut adfs_dir) -> i32 {
    (*dir).sb = sb; (*dir).bhs = (*dir).bh.as_mut_ptr(); (*dir).nr_buffers = 0;
    ((*ADFS_SB(sb)).s_dir).read.unwrap()(sb, indaddr, size, dir)
}

unsafe fn adfs_dir_read_inode(sb: *mut super_block, inode: *mut inode, dir: *mut adfs_dir) -> i32 {
    let ret = adfs_dir_read(sb, (*ADFS_I(inode)).indaddr, (*inode).i_size, dir);
    if ret != 0 { return ret; }
    if (*ADFS_I(inode)).parent_id != (*dir).parent_id {
        adfs_error(sb, c_str!("parent directory id changed under me! (%06x but got %06x)\n"), (*ADFS_I(inode)).parent_id, (*dir).parent_id);
        adfs_dir_relse(dir); return -EIO;
    }
    0
}

unsafe fn adfs_dir_mark_dirty(dir: *mut adfs_dir) { for i in 0..(*dir).nr_buffers { mark_buffer_dirty(*(*dir).bhs.add(i as usize)); } }

unsafe fn adfs_dir_sync(dir: *mut adfs_dir) -> i32 {
    let mut err = 0;
    let mut i = (*dir).nr_buffers as i32 - 1;
    while i >= 0 { let bh = *(*dir).bhs.add(i as usize); sync_dirty_buffer(bh); if buffer_req(bh) && !buffer_uptodate(bh) { err = -EIO; } i -= 1; }
    err
}

pub unsafe fn adfs_object_fixup(dir: *mut adfs_dir, obj: *mut object_info) {
    let mut dots = 0;
    for i in 0..(*obj).name_len { if *(*obj).name.add(i as usize) == b'/' { *(*obj).name.add(i as usize) = b'.'; dots += 1; } }
    if (*obj).name_len <= 2 && dots == (*obj).name_len { *(*obj).name = b'^'; }
    if ((*obj).attr & ADFS_NDA_DIRECTORY) == 0 && (*ADFS_SB((*dir).sb)).s_ftsuffix {
        let filetype = adfs_filetype((*obj).loadaddr);
        if filetype != ADFS_FILETYPE_NONE { for shift in [0u32, 8, 4, 0].iter() { if *shift == 0 { *(*obj).name.add((*obj).name_len as usize) = b','; (*obj).name_len += 1; } else { *(*obj).name.add((*obj).name_len as usize) = hex_asc_lo(filetype >> *shift); (*obj).name_len += 1; } } }
    }
}

unsafe fn adfs_iterate(file: *mut file, ctx: *mut dir_context) -> i32 {
    let inode = file_inode(file); let sb = (*inode).i_sb; let ops = (*ADFS_SB(sb)).s_dir; let mut dir = core::mem::zeroed::<adfs_dir>();
    down_read(&mut ADFS_DIR_RWSEM); let mut ret = adfs_dir_read_inode(sb, inode, &mut dir);
    if ret != 0 { up_read(&mut ADFS_DIR_RWSEM); return ret; }
    if (*ctx).pos == 0 { if !dir_emit_dot(file, ctx) { up_read(&mut ADFS_DIR_RWSEM); adfs_dir_relse(&mut dir); return ret; } (*ctx).pos = 1; }
    if (*ctx).pos == 1 { if !dir_emit(ctx, c_str!(".."), 2, dir.parent_id, DT_DIR) { up_read(&mut ADFS_DIR_RWSEM); adfs_dir_relse(&mut dir); return ret; } (*ctx).pos = 2; }
    ret = ops.iterate.unwrap()(&mut dir, ctx); up_read(&mut ADFS_DIR_RWSEM); adfs_dir_relse(&mut dir); ret
}

pub unsafe fn adfs_dir_update(sb: *mut super_block, obj: *mut object_info, wait: i32) -> i32 {
    let ops = (*ADFS_SB(sb)).s_dir; if !IS_ENABLED_CONFIG_ADFS_FS_RW || ops.update.is_none() { return -EINVAL; }
    let mut dir = core::mem::zeroed::<adfs_dir>(); down_write(&mut ADFS_DIR_RWSEM); let mut ret = adfs_dir_read(sb, (*obj).parent_id, 0, &mut dir);
    if ret != 0 { up_write(&mut ADFS_DIR_RWSEM); return ret; }
    ret = ops.update.unwrap()(&mut dir, obj); if ret == 0 { ret = ops.commit.unwrap()(&mut dir); }
    if ret != 0 { if ret == -ENOENT { adfs_dir_relse(&mut dir); } else { adfs_dir_forget(&mut dir); } up_write(&mut ADFS_DIR_RWSEM); return ret; }
    up_write(&mut ADFS_DIR_RWSEM); adfs_dir_mark_dirty(&mut dir); if wait != 0 { ret = adfs_dir_sync(&mut dir); } adfs_dir_relse(&mut dir); ret
}

unsafe fn adfs_tolower(mut c: u8) -> u8 { if c >= b'A' && c <= b'Z' { c = c.wrapping_add(b'a' - b'A'); } c }
unsafe fn __adfs_compare(qstr: *const u8, qlen: u32, str_: *const u8, len: u32) -> i32 {
    if qlen != len { return 1; } for i in 0..qlen { if adfs_tolower(*qstr.add(i as usize)) != adfs_tolower(*str_.add(i as usize)) { return 1; } } 0
}

unsafe fn adfs_dir_lookup_byname(inode: *mut inode, qstr: *const qstr, obj: *mut object_info) -> i32 {
    let sb = (*inode).i_sb; let ops = (*ADFS_SB(sb)).s_dir; let mut dir = core::mem::zeroed::<adfs_dir>(); down_read(&mut ADFS_DIR_RWSEM);
    let mut ret = adfs_dir_read_inode(sb, inode, &mut dir); if ret == 0 { ret = ops.setpos.unwrap()(&mut dir, 0); }
    if ret == 0 { ret = -ENOENT; while ops.getnext.unwrap()(&mut dir, obj) == 0 { if __adfs_compare((*qstr).name, (*qstr).len, (*obj).name, (*obj).name_len) == 0 { ret = 0; break; } } (*obj).parent_id = (*ADFS_I(inode)).indaddr; }
    up_read(&mut ADFS_DIR_RWSEM); if !dir.bhs.is_null() { adfs_dir_relse(&mut dir); } ret
}

pub static adfs_dir_operations: file_operations = file_operations { read: Some(generic_read_dir), llseek: Some(generic_file_llseek), iterate_shared: Some(adfs_iterate), fsync: Some(simple_fsync) };

unsafe fn adfs_hash(parent: *const dentry, qstr: *mut qstr) -> i32 { if (*qstr).len > (*ADFS_SB((*parent).d_sb)).s_namelen { return -ENAMETOOLONG; } let mut len = (*qstr).len; let mut name = (*qstr).name; let mut hash = init_name_hash(parent); while len != 0 { hash = partial_name_hash(adfs_tolower(*name), hash); name = name.add(1); len -= 1; } (*qstr).hash = end_name_hash(hash); 0 }
unsafe fn adfs_compare(_dentry: *const dentry, len: u32, str_: *const u8, qstr: *const qstr) -> i32 { __adfs_compare((*qstr).name, (*qstr).len, str_, len) }
pub static adfs_dentry_operations: dentry_operations = dentry_operations { d_hash: Some(adfs_hash), d_compare: Some(adfs_compare) };
unsafe fn adfs_lookup(dir: *mut inode, dentry: *mut dentry, _flags: u32) -> *mut dentry { let mut obj = core::mem::zeroed::<object_info>(); let mut inode = core::ptr::null_mut(); let error = adfs_dir_lookup_byname(dir, &(*dentry).d_name, &mut obj); if error == 0 { inode = adfs_iget((*dir).i_sb, &obj); if inode.is_null() { inode = ERR_PTR(-EACCES); } } else if error != -ENOENT { inode = ERR_PTR(error); } d_splice_alias(inode, dentry) }
pub static adfs_dir_inode_operations: inode_operations = inode_operations { lookup: Some(adfs_lookup), setattr: Some(adfs_setattr) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
