// SPDX-License-Identifier: GPL-2.0-or-later
/* AFS dynamic root handling
 *
 * Copyright (C) 2018 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies supplied by the surrounding kernel/AFS translation.

const AFS_MIN_DYNROOT_CELL_INO: u32 = 4; /* Allow for ., .., @cell, .@cell */
const AFS_MAX_DYNROOT_CELL_INO: u32 = i32::MAX as u32;

unsafe fn afs_iget5_pseudo_test(inode: *mut inode, opaque: *mut core::ffi::c_void) -> i32 {
    let fid = opaque as *mut afs_fid;
    ((*inode).i_ino == (*fid).vnode) as i32
}

unsafe fn afs_iget5_pseudo_set(inode: *mut inode, opaque: *mut core::ffi::c_void) -> i32 {
    let as_ = AFS_FS_S((*inode).i_sb);
    let vnode = AFS_FS_I(inode);
    let fid = opaque as *mut afs_fid;

    (*vnode).volume = (*as_).volume;
    (*vnode).fid = *fid;
    (*inode).i_ino = (*fid).vnode;
    (*inode).i_generation = (*fid).unique;
    0
}

unsafe fn afs_iget_pseudo_dir(sb: *mut super_block, ino: ino_t) -> *mut inode {
    let mut fid = afs_fid { vid: 0, vnode: ino, unique: 1 };

    _enter("");
    let inode = iget5_locked(sb, fid.vnode, afs_iget5_pseudo_test, afs_iget5_pseudo_set, &mut fid as *mut _ as *mut _);
    if inode.is_null() {
        _leave(" = -ENOMEM");
        return ERR_PTR(-ENOMEM);
    }

    _debug("GOT INODE %p { ino=%llu, vl=%llx, vn=%llx, u=%x }", inode, (*inode).i_ino, fid.vid, fid.vnode, fid.unique);
    let vnode = AFS_FS_I(inode);
    if inode_state_read_once(inode) & I_NEW != 0 {
        netfs_inode_init(&mut (*vnode).netfs, core::ptr::null_mut(), false);
        simple_inode_init_ts(inode);
        set_nlink(inode, 2);
        (*inode).i_size = 0;
        (*inode).i_mode = S_IFDIR | 0o555;
        (*inode).i_op = &afs_autocell_inode_operations;
        (*inode).i_uid = GLOBAL_ROOT_UID;
        (*inode).i_gid = GLOBAL_ROOT_GID;
        (*inode).i_blocks = 0;
        (*inode).i_generation = 0;
        (*inode).i_flags |= S_AUTOMOUNT | S_NOATIME;
        set_bit(AFS_VNODE_PSEUDODIR, &mut (*vnode).flags);
        set_bit(AFS_VNODE_MOUNTPOINT, &mut (*vnode).flags);
        unlock_new_inode(inode);
    }
    _leave(" = %p", inode);
    inode
}

unsafe fn afs_dynroot_lookup_cell(_dir: *mut inode, dentry: *mut dentry, _flags: u32) -> *mut dentry {
    let mut cell: *mut afs_cell = core::ptr::null_mut();
    let net = afs_d2net(dentry);
    let mut inode: *mut inode = core::ptr::null_mut();
    let mut name = (*dentry).d_name.name;
    let mut len = (*dentry).d_name.len;
    let mut dotted = false;
    let mut ret = -ENOENT;
    if *name == b'.' { name = name.add(1); len -= 1; dotted = true; }
    cell = afs_lookup_cell(net, name, len, core::ptr::null_mut(), AFS_LOOKUP_CELL_DYNROOT, afs_cell_trace_use_lookup_dynroot);
    if IS_ERR(cell) { ret = PTR_ERR(cell); return if inode.is_null() { d_splice_alias(inode, dentry) } else if ret == -ENOENT { core::ptr::null_mut() } else { ERR_PTR(ret) }; }
    inode = afs_iget_pseudo_dir((*dentry).d_inode.i_sb, (*cell).dynroot_ino * 2 + dotted as u64);
    if IS_ERR(inode) { ret = PTR_ERR(inode); afs_unuse_cell(cell, afs_cell_trace_unuse_lookup_dynroot); return if ret == -ENOENT { core::ptr::null_mut() } else { ERR_PTR(ret) }; }
    (*dentry).d_fsdata = cell as *mut _;
    d_splice_alias(inode, dentry)
}

unsafe fn afs_dynroot_lookup(dir: *mut inode, dentry: *mut dentry, flags: u32) -> *mut dentry {
    _enter("%pd", dentry);
    if flags & LOOKUP_CREATE != 0 { return ERR_PTR(-EOPNOTSUPP); }
    if (*dentry).d_name.len >= AFSNAMEMAX { _leave(" = -ENAMETOOLONG"); return ERR_PTR(-ENAMETOOLONG); }
    if (*dentry).d_name.len == 5 && memcmp((*dentry).d_name.name, b"@cell".as_ptr() as _, 5) == 0 { return afs_lookup_atcell(dir, dentry, 2); }
    if (*dentry).d_name.len == 6 && memcmp((*dentry).d_name.name, b".@cell".as_ptr() as _, 6) == 0 { return afs_lookup_atcell(dir, dentry, 3); }
    afs_dynroot_lookup_cell(dir, dentry, flags)
}

#[no_mangle]
pub static afs_dynroot_inode_operations: inode_operations = inode_operations { lookup: Some(afs_dynroot_lookup) };

unsafe fn afs_dynroot_d_release(dentry: *mut dentry) { afs_unuse_cell((*dentry).d_fsdata as *mut afs_cell, afs_cell_trace_unuse_dynroot_mntpt); }
unsafe fn afs_dynroot_delete_dentry(dentry: *const dentry) -> i32 {
    let name = &(*dentry).d_name;
    if (name.len == 5 && memcmp(name.name, b"@cell".as_ptr() as _, 5) == 0) || (name.len == 6 && memcmp(name.name, b".@cell".as_ptr() as _, 6) == 0) { 0 } else { 1 }
}

#[no_mangle]
pub static afs_dynroot_dentry_operations: dentry_operations = dentry_operations { d_delete: Some(afs_dynroot_delete_dentry), d_release: Some(afs_dynroot_d_release), d_automount: Some(afs_d_automount) };

unsafe fn afs_atcell_delayed_put_cell(arg: *mut core::ffi::c_void) { afs_put_cell(arg as *mut afs_cell, afs_cell_trace_put_atcell); }

unsafe fn afs_atcell_get_link(dentry: *mut dentry, inode: *mut inode, done: *mut delayed_call) -> *const i8 {
    let vnode = AFS_FS_I(inode); let net = afs_i2net(inode); let dotted = (*vnode).fid.vnode == 3;
    if !rcu_access_pointer((*net).ws_cell) { return ERR_PTR(-ENOENT); }
    let cell;
    let name;
    if dentry.is_null() { cell = rcu_dereference((*net).ws_cell); name = if dotted { (*cell).name.sub(1) } else { (*cell).name }; return name; }
    down_read(&(*net).cells_lock);
    cell = rcu_dereference_protected((*net).ws_cell, lockdep_is_held(&(*net).cells_lock));
    name = if dotted { (*cell).name.sub(1) } else { (*cell).name };
    afs_get_cell(cell, afs_cell_trace_get_atcell); set_delayed_call(done, afs_atcell_delayed_put_cell, cell as *mut _); up_read(&(*net).cells_lock); name
}

static afs_atcell_inode_operations: inode_operations = inode_operations { get_link: Some(afs_atcell_get_link) };

unsafe fn afs_lookup_atcell(dir: *mut inode, dentry: *mut dentry, ino: ino_t) -> *mut dentry {
    let fid = afs_fid { vid: 0, vnode: ino, unique: 1 };
    let inode = iget5_locked((*dir).i_sb, fid.vnode, afs_iget5_pseudo_test, afs_iget5_pseudo_set, &fid as *const _ as *mut _);
    if inode.is_null() { return ERR_PTR(-ENOMEM); }
    let vnode = AFS_FS_I(inode);
    if inode_state_read_once(inode) & I_NEW != 0 { netfs_inode_init(&mut (*vnode).netfs, core::ptr::null_mut(), false); simple_inode_init_ts(inode); set_nlink(inode, 1); (*inode).i_size = 0; (*inode).i_mode = S_IFLNK | 0o555; (*inode).i_op = &afs_atcell_inode_operations; (*inode).i_uid = GLOBAL_ROOT_UID; (*inode).i_gid = GLOBAL_ROOT_GID; (*inode).i_blocks = 0; (*inode).i_generation = 0; (*inode).i_flags |= S_NOATIME; unlock_new_inode(inode); }
    d_splice_alias(inode, dentry)
}

// The remaining directory iteration and root-inode setup retain the kernel ABI fields and helpers.
unsafe fn afs_dynroot_readdir_cells(net: *mut afs_net, ctx: *mut dir_context) -> i32 {
    loop { let mut ix = ((*ctx).pos >> 1) as u32; let cell = idr_get_next(&mut (*net).cells_dyn_ino, &mut ix); if cell.is_null() { return 0; } if READ_ONCE((*cell).state) == AFS_CELL_REMOVING || READ_ONCE((*cell).state) == AFS_CELL_DEAD { (*ctx).pos += 2; (*ctx).pos &= !1; continue; } let newpos = (ix as loff_t) << 1; if newpos > (*ctx).pos { (*ctx).pos = newpos; } if (*ctx).pos & 1 == 0 { if !dir_emit(ctx, (*cell).name, (*cell).name_len, (*cell).dynroot_ino, DT_DIR) { return 0; } (*ctx).pos += 1; } if (*ctx).pos & 1 == 1 { if !dir_emit(ctx, (*cell).name.sub(1), (*cell).name_len + 1, (*cell).dynroot_ino + 1, DT_DIR) { return 0; } (*ctx).pos += 1; } }
}

unsafe fn afs_dynroot_readdir(file: *mut file, ctx: *mut dir_context) -> i32 { let net = afs_d2net((*file).f_path.dentry); if !dir_emit_dots(file, ctx) { return 0; } if (*ctx).pos == 2 { if rcu_access_pointer((*net).ws_cell) && !dir_emit(ctx, b"@cell".as_ptr() as _, 5, 2, DT_LNK) { return 0; } (*ctx).pos = 3; } if (*ctx).pos == 3 { if rcu_access_pointer((*net).ws_cell) && !dir_emit(ctx, b".@cell".as_ptr() as _, 6, 3, DT_LNK) { return 0; } (*ctx).pos = 4; } if (*ctx).pos as u64 <= AFS_MAX_DYNROOT_CELL_INO as u64 { down_read(&(*net).cells_lock); let ret = afs_dynroot_readdir_cells(net, ctx); up_read(&(*net).cells_lock); return ret; } 0 }

static afs_dynroot_file_operations: file_operations = file_operations { llseek: Some(generic_file_llseek), read: Some(generic_read_dir), iterate_shared: Some(afs_dynroot_readdir), fsync: Some(noop_fsync) };

#[no_mangle]
pub unsafe fn afs_dynroot_iget_root(sb: *mut super_block) -> *mut inode { let as_ = AFS_FS_S(sb); let mut fid = afs_fid { vid: 0, vnode: 1, unique: 1 }; if !(*as_).volume.is_null() { fid.vid = (*(*as_).volume).vid; } let inode = iget5_locked(sb, fid.vnode, afs_iget5_pseudo_test, afs_iget5_pseudo_set, &mut fid as *mut _ as *mut _); if inode.is_null() { return ERR_PTR(-ENOMEM); } let vnode = AFS_FS_I(inode); if inode_state_read_once(inode) & I_NEW != 0 { netfs_inode_init(&mut (*vnode).netfs, core::ptr::null_mut(), false); simple_inode_init_ts(inode); set_nlink(inode, 2); (*inode).i_size = 0; (*inode).i_mode = S_IFDIR | 0o555; (*inode).i_op = &afs_dynroot_inode_operations; (*inode).i_fop = &afs_dynroot_file_operations; (*inode).i_uid = GLOBAL_ROOT_UID; (*inode).i_gid = GLOBAL_ROOT_GID; (*inode).i_blocks = 0; (*inode).i_generation = 0; (*inode).i_flags |= S_NOATIME; set_bit(AFS_VNODE_PSEUDODIR, &mut (*vnode).flags); unlock_new_inode(inode); } _leave(" = %p", inode); inode }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
