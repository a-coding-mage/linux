// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2017-2018 HUAWEI, Inc.
 *             https://www.huawei.com/
 * Copyright (C) 2021, Alibaba Cloud
 */
// Translated from inode.c. External kernel and EROFS symbols are supplied by
// the surrounding translation units.

unsafe fn erofs_fill_symlink(inode: *mut inode, bptr: *mut core::ffi::c_void, mut ofs: u32) -> i32 {
    let vi = EROFS_I(inode);
    let mut link: *mut i8;
    let mut end: i64 = 0;

    ofs += (*vi).xattr_isize;
    if (*vi).datalayout == EROFS_INODE_FLAT_INLINE
        && !check_add_overflow(ofs, (*inode).i_size, &mut end)
        && end <= i_blocksize(inode)
    {
        link = kmemdup_nul((bptr as *mut u8).add(ofs as usize), (*inode).i_size as usize, GFP_KERNEL);
        if link.is_null() { return -ENOMEM; }
        if (*inode).i_size == 0 || strlen(link) != (*inode).i_size as usize {
            erofs_err((*inode).i_sb, "invalid fast symlink size %llu @ nid %llu", (*inode).i_size as u64, (*vi).nid);
            kfree(link as *mut core::ffi::c_void);
            return -EFSCORRUPTED;
        }
        inode_set_cached_link(inode, link, (*inode).i_size as usize);
    }
    0
}

unsafe fn erofs_read_inode(inode: *mut inode) -> i32 {
    let sb = (*inode).i_sb;
    let mut blkaddr = erofs_blknr(sb, erofs_iloc(inode));
    let mut ofs = erofs_blkoff(sb, erofs_iloc(inode));
    let in_mbox = erofs_inode_in_metabox(inode);
    let mut buf = __EROFS_BUF_INITIALIZER;
    let sbi = EROFS_SB(sb);
    let mut addrmask = (1u64 << 48) - 1;
    let vi = EROFS_I(inode);
    let mut copied = core::mem::MaybeUninit::<erofs_inode_extended>::uninit();
    let mut ifmt: u32;
    let mut ptr = erofs_read_metabuf(&mut buf, sb, erofs_pos(sb, blkaddr), in_mbox);
    let mut err = 0;
    if IS_ERR(ptr) { err = PTR_ERR(ptr); erofs_err(sb, "failed to read inode meta block (nid: %llu): %d", (*vi).nid, err); return erofs_read_inode_err(&mut buf, err); }

    let dic = ptr.add(ofs as usize) as *mut erofs_inode_compact;
    ifmt = le16_to_cpu((*dic).i_format) as u32;
    if ifmt & !EROFS_I_ALL != 0 { erofs_err(sb, "unsupported i_format %u of nid %llu", ifmt, (*vi).nid); err = -EOPNOTSUPP; return erofs_read_inode_err(&mut buf, err); }
    (*vi).datalayout = erofs_inode_datalayout(ifmt);
    if (*vi).datalayout >= EROFS_INODE_DATALAYOUT_MAX { erofs_err(sb, "unsupported datalayout %u of nid %llu", (*vi).datalayout, (*vi).nid); err = -EOPNOTSUPP; return erofs_read_inode_err(&mut buf, err); }

    match erofs_inode_version(ifmt) {
        EROFS_INODE_LAYOUT_EXTENDED => {
            (*vi).inode_isize = core::mem::size_of::<erofs_inode_extended>() as u32;
            let die = dic as *mut erofs_inode_extended;
            (*vi).xattr_isize = erofs_xattr_ibody_size((*die).i_xattr_icount);
            (*inode).i_mode = le16_to_cpu((*die).i_mode);
            i_uid_write(inode, le32_to_cpu((*die).i_uid)); i_gid_write(inode, le32_to_cpu((*die).i_gid));
            set_nlink(inode, le32_to_cpu((*die).i_nlink));
            inode_set_mtime(inode, le64_to_cpu((*die).i_mtime), le32_to_cpu((*die).i_mtime_nsec));
            (*inode).i_size = le64_to_cpu((*die).i_size) as i64;
            ofs += (*vi).inode_isize;
        }
        EROFS_INODE_LAYOUT_COMPACT => {
            (*vi).inode_isize = core::mem::size_of::<erofs_inode_compact>() as u32; ofs += (*vi).inode_isize;
            (*vi).xattr_isize = erofs_xattr_ibody_size((*dic).i_xattr_icount);
            (*inode).i_mode = le16_to_cpu((*dic).i_mode); i_uid_write(inode, le16_to_cpu((*dic).i_uid)); i_gid_write(inode, le16_to_cpu((*dic).i_gid));
            set_nlink(inode, le16_to_cpu((*dic).i_nb.nlink));
            inode_set_mtime(inode, (*sbi).epoch + le32_to_cpu((*dic).i_mtime) as u64, (*sbi).fixed_nsec);
            (*inode).i_size = le32_to_cpu((*dic).i_size) as i64;
        }
        _ => { erofs_err(sb, "unsupported on-disk inode version %u of nid %llu", erofs_inode_version(ifmt), (*vi).nid); err = -EOPNOTSUPP; return erofs_read_inode_err(&mut buf, err); }
    }
    if (*inode).i_size < 0 { erofs_err(sb, "negative i_size @ nid %llu", (*vi).nid); err = -EFSCORRUPTED; return erofs_read_inode_err(&mut buf, err); }
    if IS_ENABLED(CONFIG_EROFS_FS_POSIX_ACL) && erofs_inode_has_noacl(inode, ptr, ofs) { cache_no_acl(inode); }
    erofs_put_metabuf(&mut buf); err
}

unsafe fn erofs_read_inode_err(buf: *mut erofs_buf, err: i32) -> i32 { erofs_put_metabuf(buf); err }

unsafe fn erofs_fill_inode(inode: *mut inode) -> i32 {
    trace_erofs_fill_inode(inode); let err = erofs_read_inode(inode); if err != 0 { return err; }
    match (*inode).i_mode & S_IFMT {
        S_IFREG => { (*inode).i_op = &erofs_generic_iops; (*inode).i_fop = if erofs_ishare_fill_inode(inode) { &erofs_ishare_fops } else { &erofs_file_fops }; }
        S_IFDIR => { (*inode).i_op = &erofs_dir_iops; (*inode).i_fop = &erofs_dir_fops; inode_nohighmem(inode); }
        S_IFLNK => { (*inode).i_op = if !(*inode).i_link.is_null() { &erofs_fast_symlink_iops } else { &erofs_symlink_iops }; inode_nohighmem(inode); }
        _ => { (*inode).i_op = &erofs_generic_iops; init_special_inode(inode, (*inode).i_mode, (*inode).i_rdev); return 0; }
    }
    mapping_set_large_folios((*inode).i_mapping); let aops = erofs_get_aops(inode); if IS_ERR(aops) { return PTR_ERR(aops); } (*inode).i_mapping.a_ops = aops; 0
}

/* ino_t is 32-bits on 32-bit arch. We have to squash the 64-bit value down so that it will fit. */
unsafe fn erofs_squash_ino(sb: *mut super_block, nid: erofs_nid_t) -> ino_t { let mut ino64 = erofs_nid_to_ino64(EROFS_SB(sb), nid); if core::mem::size_of::<ino_t>() < core::mem::size_of::<erofs_nid_t>() { ino64 ^= ino64 >> ((core::mem::size_of::<erofs_nid_t>() - core::mem::size_of::<ino_t>()) * 8); } ino64 as ino_t }
unsafe fn erofs_iget5_eq(inode: *mut inode, opaque: *mut core::ffi::c_void) -> i32 { ((*EROFS_I(inode)).nid == *(opaque as *mut erofs_nid_t)) as i32 }
unsafe fn erofs_iget5_set(inode: *mut inode, opaque: *mut core::ffi::c_void) -> i32 { let nid = *(opaque as *mut erofs_nid_t); (*inode).i_ino = erofs_squash_ino((*inode).i_sb, nid); (*EROFS_I(inode)).nid = nid; 0 }

unsafe fn erofs_iget(sb: *mut super_block, nid: erofs_nid_t) -> *mut inode { let inode = iget5_locked(sb, erofs_squash_ino(sb, nid), erofs_iget5_eq, erofs_iget5_set, &nid as *const _ as *mut _); if inode.is_null() { return ERR_PTR(-ENOMEM); } if inode_state_read_once(inode) & I_NEW != 0 { let err = erofs_fill_inode(inode); if err != 0 { iget_failed(inode); return ERR_PTR(err); } unlock_new_inode(inode); } inode }

unsafe fn erofs_getattr(idmap: *mut mnt_idmap, path: *const path, stat: *mut kstat, request_mask: u32, _query_flags: u32) -> i32 { let inode = d_inode((*path).dentry); let bdev = (*(*inode).i_sb).s_bdev; let compressed = erofs_inode_is_data_compressed((*EROFS_I(inode)).datalayout); if compressed { (*stat).attributes |= STATX_ATTR_COMPRESSED; } (*stat).attributes |= STATX_ATTR_IMMUTABLE; (*stat).attributes_mask |= STATX_ATTR_COMPRESSED | STATX_ATTR_IMMUTABLE; if request_mask & STATX_DIOALIGN != 0 && S_ISREG((*inode).i_mode) { (*stat).result_mask |= STATX_DIOALIGN; if !bdev.is_null() && !compressed { (*stat).dio_mem_align = bdev_dma_alignment(bdev) + 1; (*stat).dio_offset_align = bdev_logical_block_size(bdev); } } generic_fillattr(idmap, request_mask, inode, stat); 0 }

unsafe fn erofs_ioctl_get_volume_label(inode: *mut inode, arg: *mut core::ffi::c_void) -> i32 { let sbi = EROFS_I_SB(inode); let ret = if (*sbi).volume_name.is_null() { clear_user(arg, 1) } else { copy_to_user(arg, (*sbi).volume_name, strlen((*sbi).volume_name) + 1) }; if ret != 0 { -EFAULT } else { 0 } }
unsafe fn erofs_ioctl(filp: *mut file, cmd: u32, arg: u64) -> i64 { let inode = file_inode(filp); match cmd { FS_IOC_GETFSLABEL => erofs_ioctl_get_volume_label(inode, arg as *mut _) as i64, _ => -ENOTTY as i64 } }

#[cfg(CONFIG_COMPAT)]
unsafe fn erofs_compat_ioctl(filp: *mut file, cmd: u32, arg: u64) -> i64 { erofs_ioctl(filp, cmd, compat_ptr(arg) as u64) }

pub static erofs_generic_iops: inode_operations = inode_operations { getattr: Some(erofs_getattr), listxattr: Some(erofs_listxattr), get_inode_acl: Some(erofs_get_acl), fiemap: Some(erofs_fiemap) };
pub static erofs_symlink_iops: inode_operations = inode_operations { get_link: Some(page_get_link), getattr: Some(erofs_getattr), listxattr: Some(erofs_listxattr), get_inode_acl: Some(erofs_get_acl) };
pub static erofs_fast_symlink_iops: inode_operations = inode_operations { get_link: Some(simple_get_link), getattr: Some(erofs_getattr), listxattr: Some(erofs_listxattr), get_inode_acl: Some(erofs_get_acl) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
