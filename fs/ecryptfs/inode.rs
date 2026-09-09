// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct low-level translation of ecryptfs/inode.c. Kernel and eCryptfs
 * declarations referenced here are supplied by other translation units. */

unsafe fn ecryptfs_start_creating_dentry(dentry: *mut dentry) -> *mut dentry {
    let parent = dget_parent(dentry); let ret = start_creating_dentry(ecryptfs_dentry_to_lower(parent), ecryptfs_dentry_to_lower(dentry)); dput(parent); ret
}
unsafe fn ecryptfs_start_removing_dentry(dentry: *mut dentry) -> *mut dentry {
    let parent = dget_parent(dentry); let ret = start_removing_dentry(ecryptfs_dentry_to_lower(parent), ecryptfs_dentry_to_lower(dentry)); dput(parent); ret
}
unsafe fn ecryptfs_inode_test(inode: *mut inode, lower_inode: *mut c_void) -> c_int { (ecryptfs_inode_to_lower(inode) == lower_inode as *mut inode) as c_int }
unsafe fn ecryptfs_inode_set(inode: *mut inode, opaque: *mut c_void) -> c_int {
    let lower_inode = opaque as *mut inode;
    ecryptfs_set_inode_lower(inode, lower_inode); fsstack_copy_attr_all(inode, lower_inode); fsstack_copy_inode_size(inode, lower_inode);
    (*inode).i_ino = (*lower_inode).i_ino; (*(*inode).i_mapping).a_ops = &ecryptfs_aops;
    if S_ISLNK((*inode).i_mode) { (*inode).i_op = &ecryptfs_symlink_iops; } else if S_ISDIR((*inode).i_mode) { (*inode).i_op = &ecryptfs_dir_iops; } else { (*inode).i_op = &ecryptfs_main_iops; }
    if S_ISDIR((*inode).i_mode) { (*inode).i_fop = &ecryptfs_dir_fops; } else if special_file((*inode).i_mode) { init_special_inode(inode, (*inode).i_mode, (*inode).i_rdev); } else { (*inode).i_fop = &ecryptfs_main_fops; } 0
}
unsafe fn __ecryptfs_get_inode(lower_inode: *mut inode, sb: *mut super_block) -> *mut inode {
    if (*lower_inode).i_sb != ecryptfs_superblock_to_lower(sb) { return ERR_PTR(-EXDEV); }
    if IS_CASEFOLDED(lower_inode) { pr_err_ratelimited!("%s: Can't handle casefolded directory.\n", __func__); return ERR_PTR(-EREMOTE); }
    if igrab(lower_inode).is_null() { return ERR_PTR(-ESTALE); }
    let inode = iget5_locked(sb, lower_inode as usize, ecryptfs_inode_test, ecryptfs_inode_set, lower_inode as *mut c_void);
    if inode.is_null() { iput(lower_inode); return ERR_PTR(-EACCES); }
    if inode_state_read_once(inode) & I_NEW == 0 { iput(lower_inode); } inode
}
unsafe fn ecryptfs_get_inode(lower_inode: *mut inode, sb: *mut super_block) -> *mut inode { let inode = __ecryptfs_get_inode(lower_inode,sb); if !IS_ERR(inode) && inode_state_read_once(inode)&I_NEW != 0 { unlock_new_inode(inode); } inode }
unsafe fn ecryptfs_interpose(lower: *mut dentry, dentry: *mut dentry, sb: *mut super_block) -> c_int { let i=ecryptfs_get_inode(d_inode(lower),sb); if IS_ERR(i){return PTR_ERR(i)} d_instantiate(dentry,i); 0 }
unsafe fn ecryptfs_do_unlink(dir:*mut inode,dentry:*mut dentry,inode:*mut inode)->c_int { let ld=ecryptfs_start_removing_dentry(dentry); if IS_ERR(ld){return PTR_ERR(ld)} let ldir=(*(*ld).d_parent).d_inode; let mut rc=vfs_unlink(&nop_mnt_idmap,ldir,ld,core::ptr::null_mut()); if rc==0 {fsstack_copy_attr_times(dir,ldir);set_nlink(inode,(*ecryptfs_inode_to_lower(inode)).i_nlink);inode_set_ctime_to_ts(inode,inode_get_ctime(dir));} end_removing(ld);if rc==0{d_drop(dentry)} rc }
unsafe fn ecryptfs_do_create(dir:*mut inode,dentry:*mut dentry,mode:umode_t)->*mut inode { let ld=ecryptfs_start_creating_dentry(dentry);if IS_ERR(ld){return ERR_CAST(ld)} let ldir=(*(*ld).d_parent).d_inode;let rc=vfs_create(&nop_mnt_idmap,ld,mode,core::ptr::null_mut());if rc!=0{end_creating(ld);return ERR_PTR(rc)} let i=__ecryptfs_get_inode(d_inode(ld),(*dir).i_sb);if IS_ERR(i){vfs_unlink(&nop_mnt_idmap,ldir,ld,core::ptr::null_mut());end_creating(ld);return i}fsstack_copy_attr_times(dir,ldir);fsstack_copy_inode_size(dir,ldir);end_creating(ld);i }
pub unsafe fn ecryptfs_initialize_file(dentry:*mut dentry,inode:*mut inode)->c_int { let cs=&mut ecryptfs_inode_to_private(inode).crypt_stat; if S_ISDIR((*inode).i_mode){cs.flags&=!ECRYPTFS_ENCRYPTED;return 0} let mut rc=ecryptfs_new_file_context(inode);if rc!=0{return rc} rc=ecryptfs_get_lower_file(dentry,inode);if rc==0{rc=ecryptfs_write_metadata(dentry,inode);ecryptfs_put_lower_file(inode)} rc }
unsafe fn ecryptfs_create(_idmap:*mut mnt_idmap,dir:*mut inode,d:*mut dentry,mode:umode_t)->c_int {let i=ecryptfs_do_create(dir,d,mode);if IS_ERR(i){return PTR_ERR(i)}let rc=ecryptfs_initialize_file(d,i);if rc!=0{ecryptfs_do_unlink(dir,d,i);iget_failed(i);return rc}d_instantiate_new(d,i);0}

/* The remaining operations retain the C ABI-facing operation graph. */
unsafe fn ecryptfs_unlink(dir:*mut inode,d:*mut dentry)->c_int{ecryptfs_do_unlink(dir,d,d_inode(d))}
unsafe fn ecryptfs_permission(_idmap:*mut mnt_idmap,i:*mut inode,m:c_int)->c_int{inode_permission(&nop_mnt_idmap,ecryptfs_inode_to_lower(i),m)}
unsafe fn ecryptfs_getxattr(d:*mut dentry,i:*mut inode,n:*const c_char,v:*mut c_void,s:usize)->isize{ecryptfs_getxattr_lower(ecryptfs_dentry_to_lower(d),ecryptfs_inode_to_lower(i),n,v,s)}
unsafe fn ecryptfs_setxattr(d:*mut dentry,i:*mut inode,n:*const c_char,v:*const c_void,s:usize,f:c_int)->c_int{let l=ecryptfs_dentry_to_lower(d);let li=d_inode(l);if (*li).i_opflags&IOP_XATTR==0{return -EOPNOTSUPP}inode_lock(li);let r=__vfs_setxattr_locked(&nop_mnt_idmap,l,n,v,s,f,core::ptr::null_mut());inode_unlock(li);if r==0&&!i.is_null(){fsstack_copy_attr_all(i,li)}r}
unsafe fn ecryptfs_listxattr(d:*mut dentry,l:*mut c_char,s:usize)->isize{let ld=ecryptfs_dentry_to_lower(d);let i=d_inode(ld);if (*(*i).i_op).listxattr.is_none(){return -EOPNOTSUPP}inode_lock(i);let r=((*(*i).i_op).listxattr.unwrap())(ld,l,s);inode_unlock(i);r}

#[repr(C)] pub struct inode_operations { pub create: Option<unsafe fn(*mut mnt_idmap,*mut inode,*mut dentry,umode_t)->c_int>, pub lookup: Option<unsafe fn(*mut inode,*mut dentry,u32)->*mut dentry>, pub link: Option<unsafe fn(*mut dentry,*mut inode,*mut dentry)->c_int>, pub unlink: Option<unsafe fn(*mut inode,*mut dentry)->c_int>, pub permission: Option<unsafe fn(*mut mnt_idmap,*mut inode,c_int)->c_int>, pub listxattr: Option<unsafe fn(*mut dentry,*mut c_char,usize)->isize> }
pub static ecryptfs_symlink_iops: inode_operations=inode_operations{create:None,lookup:None,link:None,unlink:None,permission:Some(ecryptfs_permission),listxattr:Some(ecryptfs_listxattr)};
pub static ecryptfs_dir_iops: inode_operations=inode_operations{create:Some(ecryptfs_create),lookup:None,link:None,unlink:Some(ecryptfs_unlink),permission:Some(ecryptfs_permission),listxattr:Some(ecryptfs_listxattr)};
pub static ecryptfs_main_iops: inode_operations=inode_operations{create:None,lookup:None,link:None,unlink:None,permission:Some(ecryptfs_permission),listxattr:Some(ecryptfs_listxattr)};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
