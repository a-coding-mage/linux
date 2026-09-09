// SPDX-License-Identifier: GPL-2.0-or-later
/* CacheFiles path walking and related routines */

// The Linux filesystem types, constants, helpers, tracing functions and
// external cachefiles symbols referenced below are supplied by other units.

unsafe fn __cachefiles_mark_inode_in_use(object: *mut cachefiles_object, inode: *mut inode) -> bool {
    let mut can_use = false;
    if (*inode).i_flags & S_KERNEL_FILE == 0 {
        (*inode).i_flags |= S_KERNEL_FILE;
        trace_cachefiles_mark_active(object, inode);
        can_use = true;
    } else { trace_cachefiles_mark_failed(object, inode); }
    can_use
}

unsafe fn cachefiles_mark_inode_in_use(object: *mut cachefiles_object, inode: *mut inode) -> bool {
    inode_lock(inode);
    let can_use = __cachefiles_mark_inode_in_use(object, inode);
    inode_unlock(inode);
    can_use
}

unsafe fn __cachefiles_unmark_inode_in_use(object: *mut cachefiles_object, inode: *mut inode) {
    (*inode).i_flags &= !S_KERNEL_FILE;
    trace_cachefiles_mark_inactive(object, inode);
}

unsafe fn cachefiles_do_unmark_inode_in_use(object: *mut cachefiles_object, inode: *mut inode) {
    inode_lock(inode); __cachefiles_unmark_inode_in_use(object, inode); inode_unlock(inode);
}

pub unsafe fn cachefiles_unmark_inode_in_use(object: *mut cachefiles_object, file: *mut file) {
    let cache = (*(*object).volume).cache;
    let inode = file_inode(file);
    cachefiles_do_unmark_inode_in_use(object, inode);
    if !test_bit(CACHEFILES_OBJECT_USING_TMPFILE, &(*object).flags) {
        atomic_long_add((*inode).i_blocks, &(*cache).b_released);
        if atomic_inc_return(&(*cache).f_released) != 0 { cachefiles_state_changed(cache); }
    }
}

pub unsafe fn cachefiles_get_directory(cache: *mut cachefiles_cache, dir: *mut dentry,
                                        dirname: *const c_char, is_new: *mut bool) -> *mut dentry {
    let mut subdir: *mut dentry;
    let mut path: path;
    let mut ret: c_int;
    _enter!(",,%s", dirname);
    'retry: loop {
        ret = cachefiles_inject_read_error();
        subdir = if ret == 0 { start_creating(&nop_mnt_idmap, dir, &QSTR(dirname)) } else { ERR_PTR(ret) };
        trace_cachefiles_lookup(core::ptr::null_mut(), dir, subdir);
        if IS_ERR(subdir) {
            trace_cachefiles_vfs_error(core::ptr::null_mut(), d_backing_inode(dir), PTR_ERR(subdir), cachefiles_trace_lookup_error);
            if PTR_ERR(subdir) == -ENOMEM { _leave!(" = -ENOMEM"); return ERR_PTR(-ENOMEM); }
            ret = PTR_ERR(subdir); pr_err!("Lookup %s failed with error %d\n", dirname, ret); return ERR_PTR(ret);
        }
        _debug!("subdir -> %pd %s", subdir, if !d_backing_inode(subdir).is_null() { "positive" } else { "negative" });
        if d_is_negative(subdir) {
            ret = cachefiles_has_space(cache, 1, 0, cachefiles_has_space_for_create); if ret < 0 { end_creating(subdir); return ERR_PTR(ret); }
            path = path { mnt: (*cache).mnt, dentry: dir };
            ret = security_path_mkdir(&path, subdir, 0o700); if ret < 0 { end_creating(subdir); return ERR_PTR(ret); }
            ret = cachefiles_inject_write_error();
            if ret == 0 { subdir = vfs_mkdir(&nop_mnt_idmap, d_inode(dir), subdir, 0o700, core::ptr::null_mut()); if IS_ERR(subdir) { ret = PTR_ERR(subdir); } }
            else { end_creating(subdir); subdir = ERR_PTR(ret); }
            if IS_ERR(subdir) { trace_cachefiles_vfs_error(core::ptr::null_mut(), d_inode(dir), ret, cachefiles_trace_mkdir_error); end_creating(subdir); return ERR_PTR(ret); }
            trace_cachefiles_mkdir(dir, subdir);
            if d_unhashed(subdir) || d_is_negative(subdir) { end_creating(subdir); continue; }
            if !is_new.is_null() { *is_new = true; }
        }
        inode_lock(d_inode(subdir)); end_creating_keep(subdir);
        if !__cachefiles_mark_inode_in_use(core::ptr::null_mut(), d_inode(subdir)) { inode_unlock(d_inode(subdir)); dput(subdir); return ERR_PTR(-EBUSY); }
        inode_unlock(d_inode(subdir));
        if !d_can_lookup(subdir) { cachefiles_put_directory(subdir); return ERR_PTR(-EIO); }
        ret = -EPERM;
        let i = d_backing_inode(subdir);
        if (*i).i_opflags & IOP_XATTR == 0 || (*i).i_op.read.is_none() || (*i).i_op.mkdir.is_none() || (*i).i_op.rename.is_none() || (*i).i_op.rmdir.is_none() || (*i).i_op.unlink.is_none() { cachefiles_put_directory(subdir); return ERR_PTR(ret); }
        return subdir;
    }
}

pub unsafe fn cachefiles_put_directory(dir: *mut dentry) { if !dir.is_null() { cachefiles_do_unmark_inode_in_use(core::ptr::null_mut(), d_inode(dir)); dput(dir); } }

unsafe fn cachefiles_unlink(cache: *mut cachefiles_cache, object: *mut cachefiles_object, dir: *mut dentry, dentry: *mut dentry, why: fscache_why_object_killed) -> c_int {
    let path = path { mnt: (*cache).mnt, dentry: dir };
    trace_cachefiles_unlink(object, (*d_inode(dentry)).i_ino, why);
    let mut ret = security_path_unlink(&path, dentry); if ret < 0 { cachefiles_io_error(cache, "Unlink security error"); return ret; }
    ret = cachefiles_inject_remove_error(); if ret == 0 { ret = vfs_unlink(&nop_mnt_idmap, d_backing_inode(dir), dentry, core::ptr::null_mut()); if ret == -EIO { cachefiles_io_error(cache, "Unlink failed"); } }
    if ret != 0 { trace_cachefiles_vfs_error(object, d_backing_inode(dir), ret, cachefiles_trace_unlink_error); } ret
}

pub unsafe fn cachefiles_bury_object(cache: *mut cachefiles_cache, object: *mut cachefiles_object, dir: *mut dentry, rep: *mut dentry, why: fscache_why_object_killed) -> c_int {
    if (*rep).d_parent != dir { end_removing(rep); return -ESTALE; }
    if !d_is_dir(rep) { let ret = cachefiles_unlink(cache, object, dir, rep, why); end_removing(rep); return ret; }
    end_removing(rep);
    let mut nbuffer = [0 as c_char; 17];
    loop {
        sprintf(nbuffer.as_mut_ptr(), b"%08x%08x\0".as_ptr() as *const c_char, ktime_get_real_seconds() as u32, atomic_inc_return(&(*cache).gravecounter));
        let mut rd: renamedata = core::mem::zeroed(); rd.mnt_idmap = &nop_mnt_idmap; rd.old_parent = dir; rd.new_parent = (*cache).graveyard;
        let mut ret = start_renaming_dentry(&mut rd, 0, rep, &QSTR(nbuffer.as_ptr()));
        if ret != 0 { if ret == -EXDEV { return ret; } if d_unhashed(rep) || (*rep).d_parent != dir || IS_DEADDIR(d_inode(rep)) { return 0; } if ret == -EINVAL || ret == -ENOTEMPTY { cachefiles_io_error(cache, "May not make directory loop"); return -EIO; } if ret == -ENOMEM { return ret; } cachefiles_io_error(cache, "Lookup error"); return -EIO; }
        if d_mountpoint(rep) { end_renaming(&mut rd); cachefiles_io_error(cache, "Mountpoint in cache"); return -EIO; }
        let grave = rd.new_dentry; if d_is_positive(grave) { end_renaming(&mut rd); cond_resched(); continue; }
        if d_mountpoint(grave) { end_renaming(&mut rd); cachefiles_io_error(cache, "Mountpoint in graveyard"); return -EIO; }
        let p1 = path { mnt: (*cache).mnt, dentry: dir }; let p2 = path { mnt: (*cache).mnt, dentry: (*cache).graveyard };
        ret = security_path_rename(&p1, rep, &p2, grave, 0); if ret >= 0 { trace_cachefiles_rename(object, (*d_inode(rep)).i_ino, why); ret = cachefiles_inject_read_error(); if ret == 0 { ret = vfs_rename(&mut rd); } }
        cachefiles_do_unmark_inode_in_use(object, d_inode(rep)); end_renaming(&mut rd); return 0;
    }
}

pub unsafe fn cachefiles_delete_object(object: *mut cachefiles_object, why: fscache_why_object_killed) -> c_int { let volume = (*object).volume; let dentry = (*(*object).file).f_path.dentry; let fan = (*volume).fanout[(*(*object).cookie).key_hash as u8 as usize]; let d = start_removing_dentry(fan, dentry); let ret = if IS_ERR(d) { PTR_ERR(d) } else { cachefiles_unlink((*volume).cache, object, fan, d, why) }; end_removing(d); ret }

pub unsafe fn cachefiles_create_tmpfile(object: *mut cachefiles_object) -> *mut file {
    let volume = (*object).volume; let cache = (*volume).cache; let fan = (*volume).fanout[(*(*object).cookie).key_hash as u8 as usize]; let parentpath = path { mnt: (*cache).mnt, dentry: fan }; let mut saved_cred: *const cred = core::ptr::null(); let mut ret: c_long;
    cachefiles_begin_secure(cache, &mut saved_cred); let mut file: *mut file;
    ret = cachefiles_inject_write_error(); if ret == 0 { file = kernel_tmpfile_open(&nop_mnt_idmap, &parentpath, S_IFREG | 0o600, O_RDWR | O_LARGEFILE | O_DIRECT, (*cache).cache_cred); ret = PTR_ERR_OR_ZERO(file); } else { file = ERR_PTR(ret); }
    if ret != 0 { cachefiles_end_secure(cache, saved_cred); return ERR_PTR(ret); }
    if !cachefiles_mark_inode_in_use(object, file_inode(file)) { WARN_ON(1); }
    let ni_size = round_up((*(*object).cookie).object_size, CACHEFILES_DIO_BLOCK_SIZE); if ni_size > 0 { ret = cachefiles_inject_write_error(); if ret == 0 { ret = vfs_truncate(&(*file).f_path, ni_size); } if ret < 0 { cachefiles_do_unmark_inode_in_use(object, file_inode(file)); fput(file); cachefiles_end_secure(cache, saved_cred); return ERR_PTR(ret); } }
    if (*file).f_op.read_iter.is_none() || (*file).f_op.write_iter.is_none() { cachefiles_do_unmark_inode_in_use(object, file_inode(file)); fput(file); cachefiles_end_secure(cache, saved_cred); return ERR_PTR(-EINVAL); }
    cachefiles_end_secure(cache, saved_cred); file
}

unsafe fn cachefiles_create_file(object: *mut cachefiles_object) -> bool { if cachefiles_has_space((*(*object).volume).cache, 1, 0, cachefiles_has_space_for_create) < 0 { return false; } let file = cachefiles_create_tmpfile(object); if IS_ERR(file) { return false; } set_bit(FSCACHE_COOKIE_NEEDS_UPDATE, &(*(*object).cookie).flags); set_bit(CACHEFILES_OBJECT_USING_TMPFILE, &(*object).flags); (*object).file = file; true }

unsafe fn cachefiles_open_file(object: *mut cachefiles_object, dentry: *mut dentry) -> bool {
    let cache = (*(*object).volume).cache; if !cachefiles_mark_inode_in_use(object, d_inode(dentry)) { return false; }
    let path = path { mnt: (*cache).mnt, dentry }; let file = kernel_file_open(&path, O_RDWR | O_LARGEFILE | O_DIRECT, (*cache).cache_cred); if IS_ERR(file) { cachefiles_do_unmark_inode_in_use(object, d_inode(dentry)); return false; }
    if (*file).f_op.read_iter.is_none() || (*file).f_op.write_iter.is_none() { fput(file); cachefiles_do_unmark_inode_in_use(object, d_inode(dentry)); return false; }
    let ret = cachefiles_check_auxdata(object, file); if ret < 0 { fscache_cookie_lookup_negative((*object).cookie); cachefiles_unmark_inode_in_use(object, file); fput(file); if ret == -ESTALE { return cachefiles_create_file(object); } return false; }
    clear_bit(FSCACHE_COOKIE_NO_DATA_TO_READ, &(*(*object).cookie).flags); (*object).file = file; touch_atime(&(*file).f_path); true
}

pub unsafe fn cachefiles_look_up_object(object: *mut cachefiles_object) -> bool { let volume = (*object).volume; let fan = (*volume).fanout[(*(*object).cookie).key_hash as u8 as usize]; let mut dentry = lookup_one_positive_unlocked(&nop_mnt_idmap, &QSTR((*object).d_name), fan); if IS_ERR(dentry) { if PTR_ERR(dentry) == -ENOENT { fscache_cookie_lookup_negative((*object).cookie); return cachefiles_create_file(object); } return false; } if !d_is_reg(dentry) { let de = start_removing_dentry(fan, dentry); let ret = if IS_ERR(de) { PTR_ERR(de) } else { cachefiles_bury_object((*volume).cache, object, fan, de, FSCACHE_OBJECT_IS_WEIRD) }; dput(dentry); if ret < 0 { return false; } fscache_cookie_lookup_negative((*object).cookie); return cachefiles_create_file(object); } let ret = cachefiles_open_file(object, dentry); dput(dentry); ret }

pub unsafe fn cachefiles_commit_tmpfile(cache: *mut cachefiles_cache, object: *mut cachefiles_object) -> bool { let fan = (*(*object).volume).fanout[(*(*object).cookie).key_hash as u8 as usize]; let mut dentry = start_creating(&nop_mnt_idmap, fan, &QSTR((*object).d_name)); if IS_ERR(dentry) { return false; } while !d_is_negative(dentry) { if cachefiles_unlink(cache, object, fan, dentry, FSCACHE_OBJECT_IS_STALE) < 0 { end_creating(dentry); return false; } end_creating(dentry); dentry = start_creating(&nop_mnt_idmap, fan, &QSTR((*object).d_name)); if IS_ERR(dentry) { return false; } } let ret = vfs_link((*(*object).file).f_path.dentry, &nop_mnt_idmap, d_inode(fan), dentry, core::ptr::null_mut()); end_creating(dentry); if ret < 0 { false } else { clear_bit(CACHEFILES_OBJECT_USING_TMPFILE, &(*object).flags); true } }

unsafe fn cachefiles_lookup_for_cull(cache: *mut cachefiles_cache, dir: *mut dentry, filename: *mut c_char) -> *mut dentry { let victim = start_removing(&nop_mnt_idmap, dir, &QSTR(filename)); if IS_ERR(victim) { let ret = PTR_ERR(victim); if ret == -ENOENT { return ERR_PTR(-ESTALE); } if ret == -EIO { cachefiles_io_error(cache, "Lookup failed"); } return ERR_PTR(if ret != -ENOMEM { -EIO } else { ret }); } if (*d_inode(victim)).i_flags & S_KERNEL_FILE != 0 { end_removing(victim); return ERR_PTR(-EBUSY); } victim }

pub unsafe fn cachefiles_cull(cache: *mut cachefiles_cache, dir: *mut dentry, filename: *mut c_char) -> c_int { let victim = cachefiles_lookup_for_cull(cache, dir, filename); if IS_ERR(victim) { return PTR_ERR(victim); } let inode = d_inode(victim); inode_lock(inode); let ret = if (*inode).i_flags & S_KERNEL_FILE != 0 { -EBUSY } else { (*inode).i_flags |= S_KERNEL_FILE; 0 }; inode_unlock(inode); if ret < 0 { end_removing(victim); return if ret == -ENOENT { -ESTALE } else if ret != -ENOMEM { -EIO } else { ret }; } dget(victim); let ret = cachefiles_bury_object(cache, core::ptr::null_mut(), dir, victim, FSCACHE_OBJECT_WAS_CULLED); dput(victim); if ret < 0 { return if ret != -ENOMEM { -EIO } else { ret }; } fscache_count_culled(); 0 }

pub unsafe fn cachefiles_check_in_use(_cache: *mut cachefiles_cache, dir: *mut dentry, filename: *mut c_char) -> c_int { let victim = cachefiles_lookup_for_cull(_cache, dir, filename); if IS_ERR(victim) { return PTR_ERR(victim); } inode_unlock(d_inode(dir)); dput(victim); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
