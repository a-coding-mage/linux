// SPDX-License-Identifier: GPL-2.0-only
/* Overlayfs NFS export support. */

unsafe fn ovl_encode_maybe_copy_up(dentry: *mut dentry) -> i32 {
    if !ovl_dentry_upper(dentry).is_null() { return 0; }
    let err = ovl_copy_up(dentry);
    if err != 0 { pr_warn_ratelimited!("failed to copy up on encode (%pd2, err=%i)\n", dentry, err); }
    err
}

unsafe fn ovl_connectable_layer(dentry: *mut dentry) -> i32 {
    let oe = OVL_E(dentry);
    if dentry == (*(*dentry).d_sb).s_root { return ovl_numlower(oe); }
    if !ovl_dentry_upper(dentry).is_null() && !ovl_test_flag(OVL_INDEX, d_inode(dentry)) { return 0; }
    (*ovl_lowerstack(oe)).layer.as_ref().unwrap().idx
}

unsafe fn ovl_connect_layer(dentry: *mut dentry) -> i32 {
    let mut next: *mut dentry;
    let mut parent: *mut dentry = core::ptr::null_mut();
    let oe = OVL_E(dentry);
    let mut err = 0;
    if WARN_ON(dentry == (*(*dentry).d_sb).s_root) || WARN_ON(ovl_dentry_lower(dentry).is_null()) { return -EIO; }
    let origin_layer = (*ovl_lowerstack(oe)).layer.as_ref().unwrap().idx;
    if ovl_dentry_test_flag(OVL_E_CONNECTED, dentry) { return origin_layer; }
    next = dget(dentry);
    loop {
        parent = dget_parent(next);
        if WARN_ON(parent == next) { err = -EIO; break; }
        if ovl_connectable_layer(parent) < origin_layer { err = ovl_encode_maybe_copy_up(next); break; }
        if ovl_dentry_test_flag(OVL_E_CONNECTED, parent) || ovl_test_flag(OVL_INDEX, d_inode(parent)) { break; }
        dput(next); next = parent;
    }
    dput(parent); dput(next);
    if err == 0 { ovl_dentry_set_flag(OVL_E_CONNECTED, dentry); }
    if err != 0 { err } else { origin_layer }
}

unsafe fn ovl_check_encode_origin(inode: *mut inode) -> i32 {
    let ofs = OVL_FS((*inode).i_sb);
    let decodable = (*ofs).config.nfs_export;
    let dentry: *mut dentry;
    if ovl_upper_mnt(ofs).is_null() { return 1; }
    if ovl_inode_upper(inode).is_null() && !decodable { return 1; }
    if ovl_inode_lower(inode).is_null() { return 0; }
    if inode == d_inode((*(*inode).i_sb).s_root) { return 0; }
    if !ovl_inode_upper(inode).is_null() && decodable && !ovl_test_flag(OVL_INDEX, inode) { return 0; }
    if !decodable || !S_ISDIR((*inode).i_mode) { return 1; }
    dentry = d_find_any_alias(inode);
    if dentry.is_null() { return -ENOENT; }
    let err = ovl_connect_layer(dentry); dput(dentry);
    if err < 0 { return err; } 1
}

unsafe fn ovl_dentry_to_fid(ofs: *mut ovl_fs, inode: *mut inode, fid: *mut u32, buflen: i32) -> i32 {
    let mut fh: *mut ovl_fh = core::ptr::null_mut();
    let enc_lower = ovl_check_encode_origin(inode);
    if enc_lower < 0 { pr_warn_ratelimited!("failed to encode file handle (ino=%llu, err=%i)\n", (*inode).i_ino, enc_lower); return enc_lower; }
    fh = ovl_encode_real_fh(ofs, if enc_lower != 0 { ovl_inode_lower(inode) } else { ovl_inode_upper(inode) }, enc_lower == 0);
    if IS_ERR(fh) { return PTR_ERR(fh); }
    let len = OVL_FH_LEN(fh); if len <= buflen { memcpy(fid as *mut _, fh as *const _, len as usize); }
    kfree(fh as *mut _); len
}

unsafe fn ovl_encode_fh(inode: *mut inode, fid: *mut u32, max_len: *mut i32, parent: *mut inode) -> i32 {
    let ofs = OVL_FS((*inode).i_sb); let buflen = (*max_len) << 2;
    if !parent.is_null() { return FILEID_INVALID; }
    let bytes = ovl_dentry_to_fid(ofs, inode, fid, buflen);
    if bytes <= 0 || bytes > buflen { return FILEID_INVALID; }
    *max_len = bytes >> 2; OVL_FILEID_V1
}

/* The remaining helpers retain kernel pointer semantics and external overlayfs dependencies. */
unsafe fn ovl_fid_to_fh(fid: *mut fid, buflen: i32, fh_type: i32) -> *mut ovl_fh {
    if fh_type == OVL_FILEID_V1 { return fid as *mut ovl_fh; }
    if fh_type != OVL_FILEID_V0 || buflen <= OVL_FH_WIRE_OFFSET { return ERR_PTR(-EINVAL); }
    let fh = kzalloc(buflen as usize, GFP_KERNEL) as *mut ovl_fh;
    if fh.is_null() { return ERR_PTR(-ENOMEM); }
    memcpy((*fh).buf.as_mut_ptr() as *mut _, fid as *const u8, (buflen - OVL_FH_WIRE_OFFSET) as usize); fh
}

unsafe fn ovl_fh_to_dentry(sb: *mut super_block, fid: *mut fid, fh_len: i32, fh_type: i32) -> *mut dentry {
    let len = fh_len << 2; let fh = ovl_fid_to_fh(fid, len, fh_type); let mut flags = 0;
    if IS_ERR(fh) { return fh as *mut dentry; }
    let mut err = ovl_check_fh_len(fh, len); if err != 0 { kfree(fh as *mut _); return ERR_PTR(err); }
    flags = (*fh).fb.flags;
    let d = if flags & OVL_FH_FLAG_PATH_UPPER != 0 { ovl_upper_fh_to_d(sb, fh) } else { ovl_lower_fh_to_d(sb, fh) };
    if fh as *mut _ != fid as *mut _ { kfree(fh as *mut _); } d
}

unsafe fn ovl_fh_to_parent(_sb: *mut super_block, _fid: *mut fid, _fh_len: i32, _fh_type: i32) -> *mut dentry { pr_warn_ratelimited!("connectable file handles not supported; use 'no_subtree_check' exportfs option.\n"); ERR_PTR(-EACCES) }
unsafe fn ovl_get_name(_parent: *mut dentry, _name: *mut i8, _child: *mut dentry) -> i32 { WARN_ON_ONCE(true); -EIO }
unsafe fn ovl_get_parent(_dentry: *mut dentry) -> *mut dentry { WARN_ON_ONCE(true); ERR_PTR(-EIO) }

unsafe fn ovl_obtain_alias(sb: *mut super_block, upper_alias: *mut dentry, lowerpath: *mut ovl_path, index: *mut dentry) -> *mut dentry {
    let lower = if lowerpath.is_null() { core::ptr::null_mut() } else { (*lowerpath).dentry };
    let upper = if !upper_alias.is_null() { upper_alias } else { index };
    if d_is_dir(if !upper.is_null() { upper } else { lower }) { return ERR_PTR(-EIO); }
    let oe = ovl_alloc_entry(!lower.is_null()); if oe.is_null() { return ERR_PTR(-ENOMEM); }
    let mut oip = ovl_inode_params { index, upperdentry: dget(upper), oe };
    if !lower.is_null() { (*ovl_lowerstack(oe)).dentry = dget(lower); (*ovl_lowerstack(oe)).layer = (*lowerpath).layer; }
    let inode = ovl_get_inode(sb, &mut oip); if IS_ERR(inode) { ovl_free_entry(oe); dput(upper); return ERR_CAST(inode); }
    if !upper.is_null() { ovl_set_flag(OVL_UPPERDATA, inode); } d_obtain_alias(inode)
}

unsafe fn ovl_dentry_real_at(dentry: *mut dentry, idx: i32) -> *mut dentry {
    let oe = OVL_E(dentry); if idx == 0 { return ovl_dentry_upper(dentry); }
    let stack = ovl_lowerstack(oe); for i in 0..ovl_numlower(oe) { if (*stack.add(i as usize)).layer.as_ref().unwrap().idx == idx { return (*stack.add(i as usize)).dentry; } } core::ptr::null_mut()
}

unsafe fn ovl_lookup_real_inode(sb: *mut super_block, real: *mut dentry, layer: *const ovl_layer) -> *mut dentry {
    let ofs = OVL_FS(sb); let inode = ovl_lookup_inode(sb, real, (*layer).idx == 0); if IS_ERR(inode) { return ERR_CAST(inode); }
    let mut this = if !inode.is_null() { let x=d_find_any_alias(inode); iput(inode); x } else { core::ptr::null_mut() };
    if this.is_null() && (*layer).idx != 0 && !ovl_indexdir(sb) .is_null() && !WARN_ON(!d_is_dir(real)) { let index=ovl_lookup_index(ofs, core::ptr::null_mut(), real, false); if IS_ERR(index) { return index; } if !index.is_null() { let upper=ovl_index_upper(ofs,index,true); dput(index); if IS_ERR_OR_NULL(upper) { return upper; } this=ovl_lookup_real(sb,upper,&(*ofs).layers[0]); dput(upper); } }
    if IS_ERR_OR_NULL(this) { return this; } if ovl_dentry_real_at(this,(*layer).idx)!=real { dput(this); ERR_PTR(-EIO) } else { this }
}

unsafe fn ovl_lookup_real_ancestor(sb:*mut super_block, real:*mut dentry, layer:*const ovl_layer)->*mut dentry {
    if real==(*(*layer).mnt).mnt_root { return dget((*sb).s_root); }
    let mut next=dget(real); loop { let parent=dget_parent(next); let a=ovl_lookup_real_inode(sb,next,layer); if !a.is_null() { dput(parent); dput(next); return a; } if parent==(*(*layer).mnt).mnt_root { dput(parent); dput(next); return dget((*sb).s_root); } if parent==next { dput(parent); dput(next); return ERR_PTR(-EXDEV); } dput(next); next=parent; }
}

unsafe fn ovl_lookup_real(sb:*mut super_block, real:*mut dentry, layer:*const ovl_layer)->*mut dentry {
    let mut connected=ovl_lookup_real_ancestor(sb,real,layer); if IS_ERR(connected){return connected;} if ovl_dentry_real_at(connected,(*layer).idx)==real{return connected;}
    let mut next=dget(real); loop { let parent=dget_parent(next); if parent==ovl_dentry_real_at(connected,(*layer).idx) { let this=ovl_lookup_real_one(connected,next,layer); dput(parent); dput(next); if IS_ERR(this){dput(connected);return this;} dput(connected);connected=this; if ovl_dentry_real_at(connected,(*layer).idx)==real{return connected;} next=dget(real); continue; } if parent==(*(*layer).mnt).mnt_root { dput(connected);connected=dget((*sb).s_root); dput(parent);dput(next);next=dget(real);continue;} if parent==next {dput(parent);dput(next);dput(connected);return ERR_PTR(-EXDEV);} dput(next);next=parent; }
}

unsafe fn ovl_lookup_real_one(connected:*mut dentry, real:*mut dentry, layer:*const ovl_layer)->*mut dentry {
    let mut name: name_snapshot = core::mem::zeroed(); take_dentry_name_snapshot(&mut name,real);
    let this=lookup_noperm_unlocked(&name.name,connected); release_dentry_name_snapshot(&mut name);
    if ovl_dentry_real_at(connected,(*layer).idx)!=(*real).d_parent { if !IS_ERR(this){dput(this);} return ERR_PTR(-ECHILD); }
    if IS_ERR(this){return this;} if this.is_null() || (*this).d_inode.is_null(){if !this.is_null(){dput(this);}return ERR_PTR(-ENOENT);}
    if ovl_dentry_real_at(this,(*layer).idx)!=real {dput(this);return ERR_PTR(-ESTALE);} this
}

unsafe fn ovl_get_dentry(sb:*mut super_block, upper:*mut dentry, lowerpath:*mut ovl_path, index:*mut dentry)->*mut dentry {
    let ofs=OVL_FS(sb); let layer=if !upper.is_null(){&(*ofs).layers[0]}else{(*lowerpath).layer}; let real=if !upper.is_null(){upper}else if !index.is_null(){index}else{(*lowerpath).dentry};
    if !d_is_dir(real){return ovl_obtain_alias(sb,upper,lowerpath,index);} if ((*real).d_flags&DCACHE_DISCONNECTED)!=0 || d_unhashed(real){return ERR_PTR(-ENOENT);} ovl_lookup_real(sb,real,layer)
}
unsafe fn ovl_upper_fh_to_d(sb:*mut super_block, fh:*mut ovl_fh)->*mut dentry { let ofs=OVL_FS(sb); if ovl_upper_mnt(ofs).is_null(){return ERR_PTR(-EACCES);} let upper=ovl_decode_real_fh(ofs,fh,ovl_upper_mnt(ofs),true); if IS_ERR_OR_NULL(upper){return upper;} let d=ovl_get_dentry(sb,upper,core::ptr::null_mut(),core::ptr::null_mut());dput(upper);d }
unsafe fn ovl_lower_fh_to_d(sb:*mut super_block, fh:*mut ovl_fh)->*mut dentry { let ofs=OVL_FS(sb); let mut origin:ovl_path=core::mem::zeroed(); let mut stack=&mut origin; let err=ovl_check_origin_fh(ofs,fh,false,core::ptr::null_mut(),&mut stack); if err!=0{return ERR_PTR(err);} let d=ovl_get_dentry(sb,core::ptr::null_mut(),&mut origin,core::ptr::null_mut());dput(origin.dentry);d }

#[repr(C)]
pub struct export_operations { pub encode_fh: Option<unsafe fn(*mut inode,*mut u32,*mut i32,*mut inode)->i32>, pub fh_to_dentry: Option<unsafe fn(*mut super_block,*mut fid,i32,i32)->*mut dentry>, pub fh_to_parent: Option<unsafe fn(*mut super_block,*mut fid,i32,i32)->*mut dentry>, pub get_name: Option<unsafe fn(*mut dentry,*mut i8,*mut dentry)->i32>, pub get_parent: Option<unsafe fn(*mut dentry)->*mut dentry> }
pub static ovl_export_operations: export_operations = export_operations { encode_fh: Some(ovl_encode_fh), fh_to_dentry: Some(ovl_fh_to_dentry), fh_to_parent: Some(ovl_fh_to_parent), get_name: Some(ovl_get_name), get_parent: Some(ovl_get_parent) };
pub static ovl_export_fid_operations: export_operations = export_operations { encode_fh: Some(ovl_encode_fh), fh_to_dentry: None, fh_to_parent: None, get_name: None, get_parent: None };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
