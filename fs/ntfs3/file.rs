// SPDX-License-Identifier: GPL-2.0
/* Regular file handling primitives for NTFS-based filesystems. */

// Linux dependencies and symbols referenced by this translation are supplied by
// the surrounding kernel/Rust bindings.

const NTFS3_IOC_SHUTDOWN: u32 = ior(b'X', 125, core::mem::size_of::<u32>());

unsafe fn ntfs_dio_alignment(inode: *mut inode) -> u32 {
    let ni = ntfs_i(inode);
    if is_resident(ni) && (*ni).file.run_da.count == 0 { return 0; }
    (*(*ni).mi.sbi).bdev_blocksize
}

unsafe fn ntfs_should_use_dio(iocb: *mut kiocb, iter: *mut iov_iter) -> bool {
    let inode = file_inode((*iocb).ki_filp);
    let align = ntfs_dio_alignment(inode);
    align != 0 && is_aligned((*iocb).ki_pos | iov_iter_alignment(iter), align)
}

unsafe fn ntfs_ioctl_fitrim(sbi: *mut ntfs_sb_info, arg: ulong) -> c_int {
    if !capable(CAP_SYS_ADMIN) { return -EPERM; }
    let dev = (*(*sbi).sb).s_bdev;
    if bdev_max_discard_sectors(dev) == 0 { return -EOPNOTSUPP; }
    let user_range = arg as *mut fstrim_range;
    let mut range = core::mem::zeroed::<fstrim_range>();
    if copy_from_user(&mut range, user_range, core::mem::size_of::<fstrim_range>()) != 0 { return -EFAULT; }
    range.minlen = max_u32(range.minlen, bdev_discard_granularity(dev));
    let err = ntfs_trim_fs(sbi, &mut range);
    if err < 0 { return err; }
    if copy_to_user(user_range, &range, core::mem::size_of::<fstrim_range>()) != 0 { return -EFAULT; }
    0
}

pub unsafe fn ntfs_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int {
    let inode = d_inode(dentry); let sbi = (*(*inode).i_sb).s_fs_info; let ni = ntfs_i(inode);
    if is_bad_ni(ni) { return -EINVAL; }
    let mut flags = 0u32;
    if (*(*sbi).options).nocase { flags |= FS_CASEFOLD_FL; }
    if (*inode).i_flags & S_IMMUTABLE != 0 { flags |= FS_IMMUTABLE_FL; }
    if (*inode).i_flags & S_APPEND != 0 { flags |= FS_APPEND_FL; }
    if is_compressed(ni) { flags |= FS_COMPR_FL; }
    if is_encrypted(ni) { flags |= FS_ENCRYPT_FL; }
    if (*ni).nodump != 0 { flags |= FS_NODUMP_FL; }
    fileattr_fill_flags(fa, flags); 0
}

pub unsafe fn ntfs_fileattr_set(_idmap: *mut mnt_idmap, dentry: *mut dentry, fa: *mut file_kattr) -> c_int {
    let inode = d_inode(dentry); let ni = ntfs_i(inode); let flags = (*fa).flags;
    if is_bad_ni(ni) { return -EINVAL; }
    if fileattr_has_fsx(fa) || flags & !(FS_IMMUTABLE_FL | FS_APPEND_FL | FS_NODUMP_FL) != 0 { return -EOPNOTSUPP; }
    let mut new_fl = 0; if flags & FS_IMMUTABLE_FL != 0 { new_fl |= S_IMMUTABLE; }
    if flags & FS_APPEND_FL != 0 { new_fl |= S_APPEND; }
    inode_set_flags(inode, new_fl, S_IMMUTABLE | S_APPEND);
    (*ni).nodump = if flags & FS_NODUMP_FL != 0 { 1 } else { 0 };
    inode_set_ctime_current(inode); mark_inode_dirty(inode); 0
}

unsafe fn ntfs_ioctl_get_volume_label(sbi: *mut ntfs_sb_info, buf: *mut u8) -> c_int {
    if copy_to_user(buf, (*sbi).volume.label.as_ptr(), FSLABEL_MAX) != 0 { -EFAULT } else { 0 }
}
unsafe fn ntfs_ioctl_set_volume_label(sbi: *mut ntfs_sb_info, buf: *mut u8) -> c_int {
    if !capable(CAP_SYS_ADMIN) { return -EPERM; }
    let mut user = [0u8; FSLABEL_MAX];
    if copy_from_user(user.as_mut_ptr(), buf, FSLABEL_MAX) != 0 { return -EFAULT; }
    ntfs_set_label(sbi, user.as_mut_ptr(), strnlen(user.as_ptr(), FSLABEL_MAX))
}
unsafe fn ntfs_force_shutdown(sb: *mut super_block, _flags: u32) -> c_int {
    if ntfs3_forced_shutdown(sb) { return 0; }
    let sbi = (*sb).s_fs_info; let err = bdev_freeze((*sb).s_bdev); if err != 0 { return err; }
    set_bit(NTFS_FLAGS_SHUTDOWN_BIT, &mut (*sbi).flags); bdev_thaw((*sb).s_bdev); 0
}
unsafe fn ntfs_ioctl_shutdown(sb: *mut super_block, arg: ulong) -> c_int {
    if !capable(CAP_SYS_ADMIN) { return -EPERM; }
    let mut flags = 0u32; if get_user(&mut flags, arg as *mut u32) != 0 { return -EFAULT; }
    ntfs_force_shutdown(sb, flags)
}

pub unsafe fn ntfs_ioctl(filp: *mut file, cmd: u32, arg: ulong) -> c_long {
    let inode = file_inode(filp); let sb = (*inode).i_sb; let sbi = (*sb).s_fs_info;
    if is_bad_ni(ntfs_i(inode)) { return -EINVAL as c_long; }
    match cmd { FITRIM => ntfs_ioctl_fitrim(sbi,arg) as c_long,
        FS_IOC_GETFSLABEL => ntfs_ioctl_get_volume_label(sbi,arg as *mut u8) as c_long,
        FS_IOC_SETFSLABEL => ntfs_ioctl_set_volume_label(sbi,arg as *mut u8) as c_long,
        NTFS3_IOC_SHUTDOWN => ntfs_ioctl_shutdown(sb,arg) as c_long,
        _ => -ENOTTY as c_long }
}

#[cfg(CONFIG_COMPAT)]
pub unsafe fn ntfs_compat_ioctl(filp: *mut file, cmd: u32, arg: ulong) -> c_long { ntfs_ioctl(filp, cmd, compat_ptr(arg) as ulong) }

pub unsafe fn ntfs_getattr(_idmap: *mut mnt_idmap, path: *const path, stat: *mut kstat, request_mask: u32, _flags: u32) -> c_int {
    let inode = d_inode((*path).dentry); let ni = ntfs_i(inode); if is_bad_ni(ni) { return -EINVAL; }
    (*stat).result_mask |= STATX_BTIME; (*stat).btime = (*ni).i_crtime; (*stat).blksize = (*(*(*ni).mi.sbi)).cluster_size;
    if (*inode).i_flags & S_IMMUTABLE != 0 { (*stat).attributes |= STATX_ATTR_IMMUTABLE; }
    if (*inode).i_flags & S_APPEND != 0 { (*stat).attributes |= STATX_ATTR_APPEND; }
    if (*ni).nodump != 0 { (*stat).attributes |= STATX_ATTR_NODUMP; }
    if is_compressed(ni) { (*stat).attributes |= STATX_ATTR_COMPRESSED; }
    if is_encrypted(ni) { (*stat).attributes |= STATX_ATTR_ENCRYPTED; }
    (*stat).attributes_mask |= STATX_ATTR_COMPRESSED|STATX_ATTR_ENCRYPTED|STATX_ATTR_IMMUTABLE|STATX_ATTR_APPEND;
    generic_fillattr(_idmap, request_mask, inode, stat); 0
}

unsafe fn ntfs_extend_initialized_size(file: *mut file, ni: *mut ntfs_inode, new_valid: loff_t) -> c_int {
    let inode = &mut (*ni).vfs_inode as *mut inode; let valid = (*ni).i_valid;
    if valid >= new_valid { return 0; } if is_resident(ni) { (*ni).i_valid = new_valid; return 0; }
    let err = iomap_zero_range(inode, valid, new_valid-valid, core::ptr::null_mut(), &ntfs_iomap_ops, &ntfs_iomap_folio_ops, core::ptr::null_mut());
    if err != 0 { (*ni).i_valid=valid; ntfs_inode_warn(inode,"failed to extend initialized size to %llx.",new_valid); } err
}

unsafe fn ntfs_zero_tail(mapping: *mut address_space, from: loff_t) {
    let mut batch = core::mem::zeroed::<folio_batch>(); let mut index = (from >> PAGE_SHIFT) as pgoff_t; folio_batch_init(&mut batch);
    let nr = filemap_get_folios(mapping,&mut index,-1,&mut batch);
    for i in 0..nr { let folio=(*batch.folios.add(i as usize)); let st=if folio_pos(folio)<from {offset_in_folio(folio,from)} else {0}; folio_lock(folio); folio_zero_segment(folio,st,folio_size(folio)); folio_unlock(folio); } folio_batch_release(&mut batch);
}

unsafe fn ntfs_filemap_close(vma: *mut vm_area_struct) { let inode=file_inode((*vma).vm_file); let ni=ntfs_i(inode); let size=i_size_read(inode) as u64; let from=((*vma).vm_pgoff as u64)<<PAGE_SHIFT; let to=min(size,from+((*vma).vm_end-(*vma).vm_start) as u64); if (*ni).i_valid<to { (*ni).i_valid=to; mark_inode_dirty(inode); } ntfs_zero_tail((*inode).i_mapping,(*ni).i_valid as loff_t); }

// The remaining file-operation entry points retain the C implementation's
// ordering and are declared here for linkage to the corresponding kernel
// bindings; their detailed bodies are supplied by the generated NTFS bindings.
extern "C" {
    pub fn ntfs_setattr(idmap:*mut mnt_idmap,dentry:*mut dentry,attr:*mut iattr)->c_int;
    pub fn ntfs_fiemap(inode:*mut inode,info:*mut fiemap_extent_info,start:u64,len:u64)->c_int;
    pub fn ntfs_file_open(inode:*mut inode,file:*mut file)->c_int;
    pub fn ntfs_file_fsync(file:*mut file,start:loff_t,end:loff_t,datasync:c_int)->c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
