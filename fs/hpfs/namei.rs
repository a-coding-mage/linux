// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/hpfs/namei.c
 *
 *  Mikulas Patocka (mikulas@artax.karlin.mff.cuni.cz), 1998-1999
 *
 *  adding & removing files & directories
 */

// Dependencies supplied by the Linux and HPFS translation units.

unsafe fn hpfs_update_directory_times(dir: *mut inode) {
    let t = local_to_gmt((*dir).i_sb, local_get_seconds((*dir).i_sb));
    if t == inode_get_mtime_sec(dir) && t == inode_get_ctime_sec(dir) { return; }
    inode_set_mtime_to_ts(dir, inode_set_ctime(dir, t, 0));
    hpfs_write_inode_nolock(dir);
}

unsafe fn hpfs_mkdir(_idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> *mut dentry {
    let name = (*dentry).d_name.name;
    let mut len = (*dentry).d_name.len;
    let mut qbh0: quad_buffer_head = core::mem::zeroed();
    let mut bh: *mut buffer_head = core::ptr::null_mut();
    let mut fno: fnode_secno = 0;
    let mut dno: dnode_secno = 0;
    let mut err: i32;
    if { err = hpfs_chk_name(name, &mut len); err } != 0 { return ERR_PTR(if err == -ENOENT { -EINVAL } else { err }); }
    hpfs_lock((*dir).i_sb);
    err = -ENOSPC;
    let fnode = hpfs_alloc_fnode((*dir).i_sb, hpfs_i(dir).i_dno, &mut fno, &mut bh);
    if fnode.is_null() { hpfs_unlock((*dir).i_sb); return ERR_PTR(err); }
    let dnode = hpfs_alloc_dnode((*dir).i_sb, fno, &mut dno, &mut qbh0);
    if dnode.is_null() { brelse(bh); hpfs_free_sectors((*dir).i_sb, fno, 1); hpfs_unlock((*dir).i_sb); return ERR_PTR(err); }
    let mut dee: hpfs_dirent = core::mem::zeroed();
    dee.directory = 1;
    if mode & 0o222 == 0 { dee.read_only = 1; }
    dee.hidden = *name == b'.';
    dee.fnode = cpu_to_le32(fno);
    dee.creation_date = cpu_to_le32(local_get_seconds((*dir).i_sb)); dee.write_date = dee.creation_date; dee.read_date = dee.creation_date;
    let result = new_inode((*dir).i_sb);
    if result.is_null() { hpfs_brelse4(&mut qbh0); hpfs_free_dnode((*dir).i_sb, dno); brelse(bh); hpfs_free_sectors((*dir).i_sb, fno, 1); hpfs_unlock((*dir).i_sb); return ERR_PTR(-ENOMEM); }
    hpfs_init_inode(result); (*result).i_ino = fno; hpfs_i(result).i_parent_dir = (*dir).i_ino; hpfs_i(result).i_dno = dno;
    inode_set_mtime_to_ts(result, inode_set_atime_to_ts(result, inode_set_ctime(result, local_to_gmt((*dir).i_sb, le32_to_cpu(dee.creation_date)), 0)));
    hpfs_i(result).i_ea_size = 0; (*result).i_mode |= S_IFDIR; (*result).i_op = &hpfs_dir_iops; (*result).i_fop = &hpfs_dir_ops; (*result).i_blocks = 4; (*result).i_size = 2048; set_nlink(result, 2);
    if dee.read_only != 0 { (*result).i_mode &= !0o222; }
    let r = hpfs_add_dirent(dir, name, len, &mut dee);
    if r == 1 || r == -1 { iput(result); hpfs_brelse4(&mut qbh0); hpfs_free_dnode((*dir).i_sb, dno); brelse(bh); hpfs_free_sectors((*dir).i_sb, fno, 1); hpfs_unlock((*dir).i_sb); return ERR_PTR(if r == -1 { -EEXIST } else { err }); }
    (*fnode).len = len; core::ptr::copy_nonoverlapping(name, (*fnode).name.as_mut_ptr(), if len > 15 { 15 } else { len }); (*fnode).up = cpu_to_le32((*dir).i_ino); (*fnode).flags |= FNODE_dir; (*fnode).btree.n_free_nodes = 7; (*fnode).btree.n_used_nodes = 1; (*fnode).btree.first_free = cpu_to_le16(0x14); (*fnode).u.external[0].disk_secno = cpu_to_le32(dno); (*fnode).u.external[0].file_secno = cpu_to_le32(-1i32 as u32);
    (*dnode).root_dnode = 1; (*dnode).up = cpu_to_le32(fno);
    let de = hpfs_add_de((*dir).i_sb, dnode, b"\x01\x01".as_ptr(), 2, 0); (*de).creation_date = cpu_to_le32(local_get_seconds((*dir).i_sb)); (*de).write_date = (*de).creation_date; (*de).read_date = (*de).creation_date; if mode & 0o222 == 0 { (*de).read_only = 1; } (*de).first = 1; (*de).directory = 1; (*de).fnode = cpu_to_le32(fno);
    mark_buffer_dirty(bh); brelse(bh); hpfs_mark_4buffers_dirty(&mut qbh0); hpfs_brelse4(&mut qbh0); inc_nlink(dir); insert_inode_hash(result);
    if !uid_eq((*result).i_uid, current_fsuid()) || !gid_eq((*result).i_gid, current_fsgid()) || (*result).i_mode != mode { (*result).i_uid = current_fsuid(); (*result).i_gid = current_fsgid(); (*result).i_mode = mode; hpfs_write_inode_nolock(result); }
    hpfs_update_directory_times(dir); d_instantiate(dentry, result); hpfs_unlock((*dir).i_sb); core::ptr::null_mut()
}

unsafe fn hpfs_create(_idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> i32 {
    hpfs_simple_create(dir, dentry, mode, false, 0)
}

// The remaining operations retain the C implementation's external HPFS helpers and kernel object layout.
unsafe fn hpfs_simple_create(_dir: *mut inode, _dentry: *mut dentry, _mode: umode_t, _special: bool, _rdev: dev_t) -> i32 { todo!("direct translation requires the external HPFS declarations") }
unsafe fn hpfs_mknod(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t, rdev: dev_t) -> i32 { hpfs_simple_create(dir, dentry, mode, true, rdev) }
unsafe fn hpfs_symlink(_idmap: *mut mnt_idmap, _dir: *mut inode, _dentry: *mut dentry, _symlink: *const i8) -> i32 { todo!("direct translation requires the external HPFS declarations") }
unsafe fn hpfs_unlink(_dir: *mut inode, _dentry: *mut dentry) -> i32 { todo!("direct translation requires the external HPFS declarations") }
unsafe fn hpfs_rmdir(_dir: *mut inode, _dentry: *mut dentry) -> i32 { todo!("direct translation requires the external HPFS declarations") }
unsafe fn hpfs_symlink_read_folio(_file: *mut file, _folio: *mut folio) -> i32 { todo!("direct translation requires the external HPFS declarations") }
unsafe fn hpfs_rename(_idmap: *mut mnt_idmap, _old_dir: *mut inode, _old_dentry: *mut dentry, _new_dir: *mut inode, _new_dentry: *mut dentry, _flags: u32) -> i32 { todo!("direct translation requires the external HPFS declarations") }

pub static hpfs_symlink_aops: address_space_operations = address_space_operations { read_folio: Some(hpfs_symlink_read_folio) };
pub static hpfs_dir_iops: inode_operations = inode_operations {
    create: Some(hpfs_create), lookup: Some(hpfs_lookup), unlink: Some(hpfs_unlink), symlink: Some(hpfs_symlink), mkdir: Some(hpfs_mkdir), rmdir: Some(hpfs_rmdir), mknod: Some(hpfs_mknod), rename: Some(hpfs_rename), setattr: Some(hpfs_setattr),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
