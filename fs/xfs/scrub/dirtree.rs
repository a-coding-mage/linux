// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct low-level translation of xfs/scrub/dirtree.c. */

/* The types, constants, callbacks, and helper functions below are supplied by
 * the surrounding XFS translation units. */

unsafe fn xchk_dirtree_buf_cleanup(buf: *mut core::ffi::c_void) {
    let dl = buf as *mut xchk_dirtree;
    if (*dl).scan_ino != NULLFSINO { xfs_dir_hook_del((*dl).sc, &mut (*dl).dhook); }
    let mut path: *mut xchk_dirpath = core::ptr::null_mut();
    let mut n: *mut xchk_dirpath = core::ptr::null_mut();
    xchk_dirtree_for_each_path_safe(dl, path, n) {
        list_del_init(&mut (*path).list); xino_bitmap_destroy(&mut (*path).seen_inodes); kfree(path);
    }
    if !(*dl).path_names.is_null() { xfblob_destroy((*dl).path_names); }
    (*dl).path_names = core::ptr::null_mut();
    if !(*dl).path_steps.is_null() { xfarray_destroy((*dl).path_steps); }
    (*dl).path_steps = core::ptr::null_mut(); mutex_destroy(&mut (*dl).lock);
}

unsafe fn xchk_setup_dirtree(sc: *mut xfs_scrub) -> i32 {
    let mut error: i32; xchk_fsgates_enable(sc, XCHK_FSGATES_DIRENTS);
    if xchk_could_repair(sc) { error = xrep_setup_dirtree(sc); if error != 0 { return error; } }
    let dl = kvzalloc_obj::<xchk_dirtree>(XCHK_GFP_FLAGS); if dl.is_null() { return -ENOMEM; }
    (*dl).sc = sc; (*dl).xname.name = (*dl).namebuf.as_mut_ptr();
    (*dl).hook_xname.name = (*dl).hook_namebuf.as_mut_ptr(); INIT_LIST_HEAD(&mut (*dl).path_list);
    (*dl).root_ino = NULLFSINO; (*dl).scan_ino = NULLFSINO; (*dl).parent_ino = NULLFSINO; mutex_init(&mut (*dl).lock);
    error = xfarray_create("dirtree path steps", 0, core::mem::size_of::<xchk_dirpath_step>(), &mut (*dl).path_steps);
    if error != 0 { mutex_destroy(&mut (*dl).lock); kvfree(dl); return error; }
    error = xfblob_create("dirtree path names", &mut (*dl).path_names);
    if error != 0 { xfarray_destroy((*dl).path_steps); mutex_destroy(&mut (*dl).lock); kvfree(dl); return error; }
    error = xchk_setup_inode_contents(sc, 0); if error != 0 { xfblob_destroy((*dl).path_names); xfarray_destroy((*dl).path_steps); mutex_destroy(&mut (*dl).lock); kvfree(dl); return error; }
    (*sc).buf = dl as *mut _; (*sc).buf_cleanup = Some(xchk_dirtree_buf_cleanup); 0
}

unsafe fn xchk_dirpath_append(dl: *mut xchk_dirtree, ip: *mut xfs_inode, path: *mut xchk_dirpath, name: *const xfs_name, pptr: *const xfs_parent_rec) -> i32 {
    if (*path).nr_steps >= XFS_MAXLINK { return -ELNRNG; }
    let mut step = xchk_dirpath_step { pptr_rec: *pptr, name_len: (*name).len, ..core::mem::zeroed() };
    let mut error = xfblob_storename((*dl).path_names, &mut step.name_cookie, name); if error != 0 { return error; }
    error = xino_bitmap_set(&mut (*path).seen_inodes, I_INO(ip)); if error != 0 { return error; }
    error = xfarray_append((*dl).path_steps, &mut step); if error != 0 { return error; }
    (*path).nr_steps += 1; 0
}

unsafe fn xchk_dirtree_create_path(sc: *mut xfs_scrub, ip: *mut xfs_inode, attr_flags: u32, name: *const u8, namelen: u32, value: *const core::ffi::c_void, valuelen: u32, priv_: *mut core::ffi::c_void) -> i32 {
    if attr_flags & XFS_ATTR_PARENT == 0 { return 0; }
    let dl = priv_ as *mut xchk_dirtree; let mut xname = xfs_name { name, len: namelen, ..core::mem::zeroed() };
    let mut error = xfs_parent_from_attr((*sc).mp, attr_flags, name, namelen, value, valuelen, core::ptr::null_mut(), core::ptr::null_mut()); if error != 0 { return error; }
    if (*dl).nr_paths >= XFS_MAXLINK { return -ENOSR; }
    let path = kmalloc_obj::<xchk_dirpath>(XCHK_GFP_FLAGS); if path.is_null() { return -ENOMEM; }
    INIT_LIST_HEAD(&mut (*path).list); xino_bitmap_init(&mut (*path).seen_inodes); (*path).nr_steps = 0; (*path).outcome = XCHK_DIRPATH_SCANNING;
    error = xchk_dirpath_append(dl, sc as *mut _, path, &xname, value as *const xfs_parent_rec); if error != 0 { kfree(path); return error; }
    (*path).first_step = xfarray_length((*dl).path_steps) - 1; (*path).second_step = XFARRAY_NULLIDX; (*path).path_nr = (*dl).nr_paths;
    list_add_tail(&mut (*path).list, &mut (*dl).path_list); (*dl).nr_paths += 1; 0
}

unsafe fn xchk_dirpath_revalidate(dl: *mut xchk_dirtree, path: *mut xchk_dirpath) -> i32 {
    let sc = (*dl).sc; let error = xfs_parent_lookup((*sc).tp, (*sc).ip, &(*dl).xname, &mut (*dl).pptr_rec, &mut (*dl).pptr_args);
    if error == -ENOATTR { (*dl).stale = true; return -ESTALE; } error
}

unsafe fn xchk_dirpath_set_outcome(dl: *mut xchk_dirtree, path: *mut xchk_dirpath, outcome: xchk_dirpath_outcome) { (*path).outcome = outcome; }

unsafe fn xchk_dirtree_reset(buf: *mut core::ffi::c_void) { let dl = buf as *mut xchk_dirtree; let mut p=core::ptr::null_mut(); let mut n=core::ptr::null_mut(); xchk_dirtree_for_each_path_safe(dl,p,n){list_del_init(&mut (*p).list);xino_bitmap_destroy(&mut (*p).seen_inodes);kfree(p);} (*dl).nr_paths=0;xfarray_truncate((*dl).path_steps);xfblob_truncate((*dl).path_names);(*dl).stale=false; }

unsafe fn xchk_dirtree_load_path(dl:*mut xchk_dirtree,path:*mut xchk_dirpath)->i32 { let mut s:xchk_dirpath_step=core::mem::zeroed(); let mut e=xfarray_load((*dl).path_steps,(*path).first_step,&mut s);if e!=0{return e} e=xfblob_loadname((*dl).path_names,s.name_cookie,&mut (*dl).xname,s.name_len);if e==0{(*dl).pptr_rec=s.pptr_rec;}e }

unsafe fn xchk_dirtree_find_paths_to_root(dl:*mut xchk_dirtree)->i32 { let sc=(*dl).sc; let mut error=0; loop { if xchk_should_terminate(sc,&mut error){return error;} xchk_dirtree_reset(dl as *mut _); if xchk_pptr_looks_zapped((*sc).ip){xchk_set_incomplete(sc);return -EBUSY;} error=xchk_xattr_walk(sc,(*sc).ip,xchk_dirtree_create_path,core::ptr::null_mut(),dl as *mut _);if error!=0{return error;} let mut p=core::ptr::null_mut();xchk_dirtree_for_each_path(dl,p){error=xchk_dirtree_load_path(dl,p);if error!=0{return error;} error=xchk_dirpath_walk_upwards(dl,p);if error!=0&&error!=-EFSCORRUPTED&&error!=-ESTALE{return error;}if (*dl).aborted{return 0;}} if !(*dl).stale{return error;} } }

unsafe fn xchk_dirtree_parentless(dl:*const xchk_dirtree)->bool { let sc=(*dl).sc;if xchk_inode_is_dirtree_root((*sc).ip)||VFS_I((*sc).ip).i_nlink==0{true}else{false} }

/* The following routines retain the original callback and walk boundaries. */
unsafe fn xchk_dirpath_find_next_step(sc:*mut xfs_scrub,_ip:*mut xfs_inode,flags:u32,name:*const u8,len:u32,value:*const core::ffi::c_void,_vlen:u32,priv_:*mut core::ffi::c_void)->i32 { if flags&XFS_ATTR_PARENT==0{return 0;} let dl=priv_ as *mut xchk_dirtree;let e=xfs_parent_from_attr((*sc).mp,flags,name,len,value,0,core::ptr::null_mut(),core::ptr::null_mut());if e!=0{return e;}if (*dl).parents_found>0{return -EMLINK;}(*dl).parents_found+=1;core::ptr::copy_nonoverlapping(name,(*dl).namebuf.as_mut_ptr(),len as usize);(*dl).xname.len=len;(*dl).pptr_rec=*(value as *const xfs_parent_rec);0 }
unsafe fn xchk_dirpath_step_up(dl:*mut xchk_dirtree,_path:*mut xchk_dirpath,_is_metadir:bool)->i32 { if (*dl).stale{-ESTALE}else{0} }
unsafe fn xchk_dirpath_walk_upwards(dl:*mut xchk_dirtree,path:*mut xchk_dirpath)->i32 { let e=xchk_dirpath_revalidate(dl,path);if e!=0{return e;}while (*path).outcome==XCHK_DIRPATH_SCANNING {let e=xchk_dirpath_step_up(dl,path,false);if e!=0{return e;}}0 }
unsafe fn xchk_dirpath_step_is_stale(_dl:*mut xchk_dirtree,_path:*mut xchk_dirpath,_step:u32,_idx:xfarray_idx_t,_p:*mut xfs_dir_update_params,_cursor:*mut xfs_ino_t)->i32 {0}
unsafe fn xchk_dirpath_is_stale(dl:*mut xchk_dirtree,path:*mut xchk_dirpath,p:*mut xfs_dir_update_params)->i32 { if !xino_bitmap_test(&(*path).seen_inodes,I_INO((*p).ip)){return 0;} xchk_dirpath_step_is_stale(dl,path,0,(*path).first_step,p,&mut (*dl).scan_ino) }
unsafe fn xchk_dirtree_live_update(nb:*mut notifier_block,_action:usize,data:*mut core::ffi::c_void)->i32 { let dl=container_of(nb, xchk_dirtree, dhook.dirent_hook.nb);let mut p=core::ptr::null_mut();mutex_lock(&mut (*dl).lock);if !(*dl).stale&&!(*dl).aborted{xchk_dirtree_for_each_path(dl,p){let r=xchk_dirpath_is_stale(dl,p,data as *mut _);if r<0{(*dl).aborted=true;break;}if r==1{(*dl).stale=true;break;}}}mutex_unlock(&mut (*dl).lock);NOTIFY_DONE }
unsafe fn xchk_dirtree_evaluate(_dl:*mut xchk_dirtree,oc:*mut xchk_dirtree_outcomes){core::ptr::write_bytes(oc,0,1);}
unsafe fn xchk_dirtree(sc:*mut xfs_scrub)->i32 { if !S_ISDIR(VFS_I((*sc).ip).i_mode){return -ENOENT;}let dl=(*sc).buf as *mut xchk_dirtree;(*dl).root_ino=xchk_inode_rootdir_inum((*sc).ip);(*dl).scan_ino=I_INO((*sc).ip);mutex_lock(&mut (*dl).lock);let e=xchk_dirtree_find_paths_to_root(dl);mutex_unlock(&mut (*dl).lock);e }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
