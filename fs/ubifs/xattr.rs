// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of UBIFS extended-attribute support. */

// Dependencies supplied by the surrounding UBIFS and Linux-compatible code.

static EMPTY_IOPS: inode_operations = inode_operations {};
static EMPTY_FOPS: file_operations = file_operations {};

unsafe fn create_xattr(c: *mut ubifs_info, host: *mut inode, nm: *const fscrypt_name, value: *const c_void, size: i32) -> i32 {
    let host_ui = ubifs_inode(host);
    let mut req = ubifs_budget_req { new_ino: 1, new_dent: 1, new_ino_d: ALIGN(size, 8), dirtied_ino: 1, dirtied_ino_d: ALIGN((*host_ui).data_len, 8) };
    if (*host_ui).xattr_cnt >= ubifs_xattr_max_cnt(c) { ubifs_err(c, "inode %llu already has too many xattrs (%d), cannot create more", (*host).i_ino, (*host_ui).xattr_cnt); return -ENOSPC; }
    let names_len = (*host_ui).xattr_names + (*host_ui).xattr_cnt + fname_len(nm) + 1;
    if names_len > XATTR_LIST_MAX { ubifs_err(c, "cannot add one more xattr name to inode %llu, total names length would become %d, max. is %d", (*host).i_ino, names_len, XATTR_LIST_MAX); return -ENOSPC; }
    let mut err = ubifs_budget_space(c, &mut req); if err != 0 { return err; }
    let inode = ubifs_new_inode(c, host, S_IFREG | S_IRWXUGO, true);
    if IS_ERR(inode) { err = PTR_ERR(inode); ubifs_release_budget(c, &mut req); return err; }
    (*(*inode).i_mapping).a_ops = &empty_aops; (*inode).i_op = &EMPTY_IOPS; (*inode).i_fop = &EMPTY_FOPS;
    (*inode).i_flags |= S_SYNC | S_NOATIME | S_NOCMTIME;
    let ui = ubifs_inode(inode); (*ui).xattr = 1; (*ui).flags |= UBIFS_XATTR_FL;
    (*ui).data = kmemdup(value, size as usize, GFP_NOFS);
    if (*ui).data.is_null() { err = -ENOMEM; make_bad_inode(inode); iput(inode); ubifs_release_budget(c, &mut req); return err; }
    (*inode).i_size = size as i64; (*ui).ui_size = size as i64; (*ui).data_len = size;
    mutex_lock(&mut (*host_ui).ui_mutex); inode_set_ctime_current(host);
    (*host_ui).xattr_cnt += 1; (*host_ui).xattr_size += CALC_DENT_SIZE(fname_len(nm)); (*host_ui).xattr_size += CALC_XATTR_BYTES(size); (*host_ui).xattr_names += fname_len(nm);
    if strcmp(fname_name(nm), UBIFS_XATTR_NAME_ENCRYPTION_CONTEXT) == 0 { (*host_ui).flags |= UBIFS_CRYPT_FL; }
    err = ubifs_jnl_update(c, host, nm, inode, 0, 1, 0);
    if err != 0 { (*host_ui).xattr_cnt -= 1; (*host_ui).xattr_size -= CALC_DENT_SIZE(fname_len(nm)); (*host_ui).xattr_size -= CALC_XATTR_BYTES(size); (*host_ui).xattr_names -= fname_len(nm); (*host_ui).flags &= !UBIFS_CRYPT_FL; mutex_unlock(&mut (*host_ui).ui_mutex); make_bad_inode(inode); iput(inode); ubifs_release_budget(c, &mut req); return err; }
    ubifs_set_inode_flags(host); mutex_unlock(&mut (*host_ui).ui_mutex); ubifs_release_budget(c, &mut req); insert_inode_hash(inode); iput(inode); 0
}

unsafe fn change_xattr(c: *mut ubifs_info, host: *mut inode, inode: *mut inode, value: *const c_void, size: i32) -> i32 {
    let host_ui = ubifs_inode(host); let ui = ubifs_inode(inode); let mut req = ubifs_budget_req { dirtied_ino: 2, dirtied_ino_d: ALIGN(size, 8) + ALIGN((*host_ui).data_len, 8), ..ubifs_budget_req::default() };
    ubifs_assert(c, (*ui).data_len == (*inode).i_size as i32); let mut err = ubifs_budget_space(c, &mut req); if err != 0 { return err; }
    let buf = kmemdup(value, size as usize, GFP_NOFS); if buf.is_null() { ubifs_release_budget(c, &mut req); return -ENOMEM; }
    kfree((*ui).data); (*ui).data = buf; (*inode).i_size = size as i64; (*ui).ui_size = size as i64; let old_size = (*ui).data_len; (*ui).data_len = size;
    mutex_lock(&mut (*host_ui).ui_mutex); inode_set_ctime_current(host); (*host_ui).xattr_size -= CALC_XATTR_BYTES(old_size); (*host_ui).xattr_size += CALC_XATTR_BYTES(size);
    err = ubifs_jnl_change_xattr(c, inode, host); if err != 0 { (*host_ui).xattr_size -= CALC_XATTR_BYTES(size); (*host_ui).xattr_size += CALC_XATTR_BYTES(old_size); mutex_unlock(&mut (*host_ui).ui_mutex); make_bad_inode(inode); ubifs_release_budget(c, &mut req); return err; }
    mutex_unlock(&mut (*host_ui).ui_mutex); ubifs_release_budget(c, &mut req); 0
}

unsafe fn iget_xattr(c: *mut ubifs_info, inum: ino_t) -> *mut inode { let inode = ubifs_iget((*c).vfs_sb, inum); if IS_ERR(inode) { ubifs_err(c, "dead extended attribute entry, error %d", PTR_ERR(inode)); return inode; } if (*ubifs_inode(inode)).xattr { return inode; } ubifs_err(c, "corrupt extended attribute entry"); iput(inode); ERR_PTR(-EINVAL) }

pub unsafe fn ubifs_xattr_set(host: *mut inode, name: *const c_char, value: *const c_void, size: usize, flags: i32, check_lock: bool) -> i32 {
    let c = (*(*host).i_sb).s_fs_info; let nm = fscrypt_name { disk_name: FSTR_INIT(name as *mut c_char, strlen(name)), ..fscrypt_name::default() }; let mut xent = kmalloc(UBIFS_MAX_XENT_NODE_SZ, GFP_NOFS) as *mut ubifs_dent_node; let mut key = ubifs_key::default(); if check_lock { ubifs_assert(c, inode_is_locked(host)); } if size > UBIFS_MAX_INO_DATA as usize { return -ERANGE; } if fname_len(&nm) > UBIFS_MAX_NLEN { return -ENAMETOOLONG; } if xent.is_null() { return -ENOMEM; }
    down_write(&mut (*ubifs_inode(host)).xattr_sem); xent_key_init(c, &mut key, (*host).i_ino, &nm); let mut err = ubifs_tnc_lookup_nm(c, &key, xent, &nm);
    if err != 0 { if err == -ENOENT { err = if flags & XATTR_REPLACE != 0 { -ENODATA } else { create_xattr(c, host, &nm, value, size as i32) }; } up_write(&mut (*ubifs_inode(host)).xattr_sem); kfree(xent as *mut c_void); return err; }
    if flags & XATTR_CREATE != 0 { up_write(&mut (*ubifs_inode(host)).xattr_sem); kfree(xent as *mut c_void); return -EEXIST; }
    let inode = iget_xattr(c, le64_to_cpu((*xent).inum)); if IS_ERR(inode) { err = PTR_ERR(inode); } else { err = change_xattr(c, host, inode, value, size as i32); iput(inode); } up_write(&mut (*ubifs_inode(host)).xattr_sem); kfree(xent as *mut c_void); err
}

pub unsafe fn ubifs_xattr_get(host: *mut inode, name: *const c_char, buf: *mut c_void, size: usize) -> isize {
    let c = (*(*host).i_sb).s_fs_info; let nm = fscrypt_name { disk_name: FSTR_INIT(name as *mut c_char, strlen(name)), ..fscrypt_name::default() }; if fname_len(&nm) > UBIFS_MAX_NLEN { return -ENAMETOOLONG as isize; } let xent = kmalloc(UBIFS_MAX_XENT_NODE_SZ, GFP_NOFS) as *mut ubifs_dent_node; if xent.is_null() { return -ENOMEM as isize; }
    down_read(&mut (*ubifs_inode(host)).xattr_sem); let mut key = ubifs_key::default(); xent_key_init(c, &mut key, (*host).i_ino, &nm); let mut err = ubifs_tnc_lookup_nm(c, &key, xent, &nm); if err == -ENOENT { err = -ENODATA; } if err != 0 { up_read(&mut (*ubifs_inode(host)).xattr_sem); kfree(xent as *mut c_void); return err as isize; }
    let inode = iget_xattr(c, le64_to_cpu((*xent).inum)); if IS_ERR(inode) { err = PTR_ERR(inode); up_read(&mut (*ubifs_inode(host)).xattr_sem); kfree(xent as *mut c_void); return err as isize; } let ui = ubifs_inode(inode); ubifs_assert(c, (*inode).i_size == (*ui).data_len as i64); if !buf.is_null() { if (*ui).data_len as usize > size { iput(inode); up_read(&mut (*ubifs_inode(host)).xattr_sem); kfree(xent as *mut c_void); return -ERANGE as isize; } memcpy(buf, (*ui).data, (*ui).data_len as usize); } err = (*ui).data_len; iput(inode); up_read(&mut (*ubifs_inode(host)).xattr_sem); kfree(xent as *mut c_void); err as isize
}

unsafe fn xattr_visible(name: *const c_char) -> bool { if strcmp(name, UBIFS_XATTR_NAME_ENCRYPTION_CONTEXT) == 0 { return false; } if strncmp(name, XATTR_TRUSTED_PREFIX, XATTR_TRUSTED_PREFIX_LEN) == 0 && !capable(CAP_SYS_ADMIN) { return false; } true }

pub unsafe fn ubifs_listxattr(dentry: *mut dentry, buffer: *mut c_char, size: usize) -> isize { let host = d_inode(dentry); let c = (*(*host).i_sb).s_fs_info; let hui = ubifs_inode(host); down_read(&mut (*hui).xattr_sem); let len = ((*hui).xattr_names + (*hui).xattr_cnt) as usize; if buffer.is_null() { up_read(&mut (*hui).xattr_sem); return len as isize; } if len > size { up_read(&mut (*hui).xattr_sem); return -ERANGE as isize; } let mut key = ubifs_key::default(); let mut nm = fscrypt_name::default(); let mut pxent: *mut ubifs_dent_node = core::ptr::null_mut(); let mut written = 0usize; lowest_xent_key(c, &mut key, (*host).i_ino); loop { let xent = ubifs_tnc_next_ent(c, &key, &mut nm); if IS_ERR(xent) { let err = PTR_ERR(xent); kfree(pxent as *mut c_void); up_read(&mut (*hui).xattr_sem); return if err == -ENOENT { written as isize } else { err as isize }; } fname_name(&mut nm) = (*xent).name; fname_len(&mut nm) = le16_to_cpu((*xent).nlen); if xattr_visible((*xent).name) { memcpy(buffer.add(written) as *mut c_void, fname_name(&nm) as *const c_void, fname_len(&nm) + 1); written += fname_len(&nm) + 1; } kfree(pxent as *mut c_void); pxent = xent; key_read(c, &(*xent).key, &mut key); } }

unsafe fn remove_xattr(c: *mut ubifs_info, host: *mut inode, inode: *mut inode, nm: *const fscrypt_name) -> i32 { let hui = ubifs_inode(host); let ui = ubifs_inode(inode); let mut req = ubifs_budget_req { dirtied_ino: 2, mod_dent: 1, dirtied_ino_d: ALIGN((*hui).data_len, 8), ..ubifs_budget_req::default() }; ubifs_assert(c, (*ui).data_len == (*inode).i_size as i32); let mut err = ubifs_budget_space(c, &mut req); if err != 0 { return err; } mutex_lock(&mut (*hui).ui_mutex); inode_set_ctime_current(host); (*hui).xattr_cnt -= 1; (*hui).xattr_size -= CALC_DENT_SIZE(fname_len(nm)); (*hui).xattr_size -= CALC_XATTR_BYTES((*ui).data_len); (*hui).xattr_names -= fname_len(nm); err = ubifs_jnl_delete_xattr(c, host, inode, nm); if err != 0 { (*hui).xattr_cnt += 1; (*hui).xattr_size += CALC_DENT_SIZE(fname_len(nm)); (*hui).xattr_size += CALC_XATTR_BYTES((*ui).data_len); (*hui).xattr_names += fname_len(nm); mutex_unlock(&mut (*hui).ui_mutex); ubifs_release_budget(c, &mut req); make_bad_inode(inode); return err; } mutex_unlock(&mut (*hui).ui_mutex); ubifs_release_budget(c, &mut req); 0 }

unsafe fn ubifs_xattr_remove(host: *mut inode, name: *const c_char) -> i32 { let c = (*(*host).i_sb).s_fs_info; let nm = fscrypt_name { disk_name: FSTR_INIT(name as *mut c_char, strlen(name)), ..fscrypt_name::default() }; if fname_len(&nm) > UBIFS_MAX_NLEN { return -ENAMETOOLONG; } let xent = kmalloc(UBIFS_MAX_XENT_NODE_SZ, GFP_NOFS) as *mut ubifs_dent_node; if xent.is_null() { return -ENOMEM; } down_write(&mut (*ubifs_inode(host)).xattr_sem); let mut key = ubifs_key::default(); xent_key_init(c, &mut key, (*host).i_ino, &nm); let mut err = ubifs_tnc_lookup_nm(c, &key, xent, &nm); if err == -ENOENT { err = -ENODATA; } if err == 0 { let inode = iget_xattr(c, le64_to_cpu((*xent).inum)); if IS_ERR(inode) { err = PTR_ERR(inode); } else { clear_nlink(inode); err = remove_xattr(c, host, inode, &nm); if err != 0 { set_nlink(inode, 1); } iput(inode); } } up_write(&mut (*ubifs_inode(host)).xattr_sem); kfree(xent as *mut c_void); err }

pub unsafe fn ubifs_purge_xattrs(host: *mut inode) -> i32 { let c = (*(*host).i_sb).s_fs_info; let hui = ubifs_inode(host); if (*hui).xattr_cnt <= ubifs_xattr_max_cnt(c) { return 0; } ubifs_warn(c, "inode %llu has too many xattrs, doing a non-atomic deletion", (*host).i_ino); down_write(&mut (*hui).xattr_sem); let mut key = ubifs_key::default(); let mut nm = fscrypt_name::default(); let mut pxent: *mut ubifs_dent_node = core::ptr::null_mut(); lowest_xent_key(c, &mut key, (*host).i_ino); loop { let xent = ubifs_tnc_next_ent(c, &key, &mut nm); if IS_ERR(xent) { let err = PTR_ERR(xent); kfree(pxent as *mut c_void); up_write(&mut (*hui).xattr_sem); return if err == -ENOENT { 0 } else { err }; } fname_name(&mut nm) = (*xent).name; fname_len(&mut nm) = le16_to_cpu((*xent).nlen); let xino = ubifs_iget((*c).vfs_sb, le64_to_cpu((*xent).inum)); if IS_ERR(xino) { let err = PTR_ERR(xino); ubifs_err(c, "dead directory entry '%s', error %d", (*xent).name, err); ubifs_ro_mode(c, err); kfree(pxent as *mut c_void); kfree(xent as *mut c_void); up_write(&mut (*hui).xattr_sem); return err; } clear_nlink(xino); let err = remove_xattr(c, host, xino, &nm); iput(xino); if err != 0 { ubifs_err(c, "cannot remove xattr, error %d", err); kfree(pxent as *mut c_void); kfree(xent as *mut c_void); up_write(&mut (*hui).xattr_sem); return err; } kfree(pxent as *mut c_void); pxent = xent; key_read(c, &(*xent).key, &mut key); } }

unsafe fn xattr_get(_handler: *const xattr_handler, _dentry: *mut dentry, inode: *mut inode, name: *const c_char, buffer: *mut c_void, size: usize) -> i32 { ubifs_xattr_get(inode, xattr_full_name(_handler, name), buffer, size) as i32 }
unsafe fn xattr_set(_handler: *const xattr_handler, _idmap: *mut mnt_idmap, _dentry: *mut dentry, inode: *mut inode, name: *const c_char, value: *const c_void, size: usize, flags: i32) -> i32 { let n = xattr_full_name(_handler, name); if !value.is_null() { ubifs_xattr_set(inode, n, value, size, flags, true) } else { ubifs_xattr_remove(inode, n) } }

static UBIFS_USER_XATTR_HANDLER: xattr_handler = xattr_handler { prefix: XATTR_USER_PREFIX, get: Some(xattr_get), set: Some(xattr_set) };
static UBIFS_TRUSTED_XATTR_HANDLER: xattr_handler = xattr_handler { prefix: XATTR_TRUSTED_PREFIX, get: Some(xattr_get), set: Some(xattr_set) };
pub static UBIFS_XATTR_HANDLERS: [*const xattr_handler; 3] = [&UBIFS_USER_XATTR_HANDLER, &UBIFS_TRUSTED_XATTR_HANDLER, core::ptr::null()];

// CONFIG_UBIFS_FS_SECURITY supplies the following security initialization callback and handler.
// Its declarations remain conditional in the surrounding build, as in the source.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
