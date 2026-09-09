// SPDX-License-Identifier: GPL-2.0
/* Translated from xfs_iops.c. Kernel and XFS symbols are supplied externally. */

static mut xfs_nondir_ilock_class: lock_class_key = lock_class_key {};
static mut xfs_dir_ilock_class: lock_class_key = lock_class_key {};

unsafe extern "C" {
    fn xfs_attr_change(args: *mut xfs_da_args, update: i32) -> i32;
    fn security_inode_init_security(inode: *mut inode, dir: *mut inode, qstr: *const qstr,
        initxattrs: unsafe extern "C" fn(*mut inode, *const xattr, *mut c_void) -> i32,
        data: *mut c_void) -> i32;
}

unsafe extern "C" fn xfs_initxattrs(inode: *mut inode, xattr_array: *const xattr,
                                    _fs_info: *mut c_void) -> i32 {
    let mut error = 0;
    let mut xattr = xattr_array;
    while !(*xattr).name.is_null() {
        let mut args = xfs_da_args { dp: XFS_I(inode), attr_filter: XFS_ATTR_SECURE,
            name: (*xattr).name, namelen: strlen((*xattr).name), value: (*xattr).value,
            valuelen: (*xattr).value_len, ..core::mem::zeroed() };
        error = xfs_attr_change(&mut args, XFS_ATTRUPDATE_UPSERT);
        if error < 0 { break; }
        xattr = xattr.add(1);
    }
    error
}

pub unsafe fn xfs_inode_init_security(inode: *mut inode, dir: *mut inode,
                                      qstr: *const qstr) -> i32 {
    security_inode_init_security(inode, dir, qstr, xfs_initxattrs, core::ptr::null_mut())
}

unsafe fn xfs_dentry_to_name(namep: *mut xfs_name, dentry: *mut dentry) {
    (*namep).name = (*dentry).d_name.name;
    (*namep).len = (*dentry).d_name.len;
    (*namep).type_ = XFS_DIR3_FT_UNKNOWN;
}

unsafe fn xfs_dentry_mode_to_name(namep: *mut xfs_name, dentry: *mut dentry, mode: i32) -> i32 {
    (*namep).name = (*dentry).d_name.name;
    (*namep).len = (*dentry).d_name.len;
    (*namep).type_ = xfs_mode_to_ftype(mode);
    if (*namep).type_ == XFS_DIR3_FT_UNKNOWN { return -EFSCORRUPTED; }
    0
}

unsafe fn xfs_cleanup_inode(dir: *mut inode, inode: *mut inode, dentry: *mut dentry) {
    let mut teardown: xfs_name = core::mem::zeroed();
    xfs_dentry_to_name(&mut teardown, dentry);
    xfs_remove(XFS_I(dir), &mut teardown, XFS_I(inode));
}

unsafe fn xfs_create_need_xattr(dir: *mut inode, default_acl: *mut posix_acl,
                                acl: *mut posix_acl) -> bool {
    if !acl.is_null() || !default_acl.is_null() { return true; }
    // CONFIG_SECURITY conditional from the C source.
    #[cfg(feature = "security")]
    if !(*(*dir).i_sb).s_security.is_null() { return true; }
    false
}

unsafe fn xfs_generic_create(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry,
    mode: umode_t, rdev: dev_t, tmpfile: *mut file) -> i32 {
    let mut args: xfs_icreate_args = core::mem::zeroed();
    args.idmap = idmap; args.pip = XFS_I(dir); args.rdev = rdev; args.mode = mode;
    let mut inode: *mut inode;
    let mut ip: *mut xfs_inode = core::ptr::null_mut();
    let mut default_acl: *mut posix_acl = core::ptr::null_mut();
    let mut acl: *mut posix_acl = core::ptr::null_mut();
    let mut name: xfs_name = core::mem::zeroed();
    if (S_ISCHR(args.mode) || S_ISBLK(args.mode)) &&
       (!sysv_valid_dev(args.rdev) || (MAJOR(args.rdev) & !0x1ff) != 0) { return -EINVAL; }
    if !S_ISCHR(args.mode) && !S_ISBLK(args.mode) { args.rdev = 0; }
    let mut error = posix_acl_create(dir, &mut args.mode, &mut default_acl, &mut acl);
    if error != 0 { return error; }
    error = xfs_dentry_mode_to_name(&mut name, dentry, args.mode);
    if error != 0 { posix_acl_release(default_acl); posix_acl_release(acl); return error; }
    if tmpfile.is_null() {
        if xfs_create_need_xattr(dir, default_acl, acl) { args.flags |= XFS_ICREATE_INIT_XATTRS; }
        error = xfs_create(&mut args, &mut name, &mut ip);
    } else {
        args.flags |= XFS_ICREATE_TMPFILE;
        if (*tmpfile).f_flags & O_EXCL != 0 { args.flags |= XFS_ICREATE_UNLINKABLE; }
        error = xfs_create_tmpfile(&mut args, &mut ip);
    }
    if error != 0 { posix_acl_release(default_acl); posix_acl_release(acl); return error; }
    inode = VFS_I(ip);
    error = xfs_inode_init_security(inode, dir, &(*dentry).d_name);
    if error != 0 { xfs_finish_inode_setup(ip); if tmpfile.is_null() { xfs_cleanup_inode(dir, inode, dentry); } xfs_irele(ip); posix_acl_release(default_acl); posix_acl_release(acl); return error; }
    if !default_acl.is_null() { error = __xfs_set_acl(inode, default_acl, ACL_TYPE_DEFAULT); if error != 0 { xfs_finish_inode_setup(ip); if tmpfile.is_null() { xfs_cleanup_inode(dir, inode, dentry); } xfs_irele(ip); posix_acl_release(default_acl); posix_acl_release(acl); return error; } }
    if !acl.is_null() { error = __xfs_set_acl(inode, acl, ACL_TYPE_ACCESS); if error != 0 { xfs_finish_inode_setup(ip); if tmpfile.is_null() { xfs_cleanup_inode(dir, inode, dentry); } xfs_irele(ip); posix_acl_release(default_acl); posix_acl_release(acl); return error; } }
    xfs_setup_iops(ip);
    if !tmpfile.is_null() { set_nlink(inode, 1); d_tmpfile(tmpfile, inode); } else { d_instantiate(dentry, inode); }
    xfs_finish_inode_setup(ip);
    posix_acl_release(default_acl); posix_acl_release(acl); error
}

pub unsafe fn xfs_vn_mknod(idmap:*mut mnt_idmap,dir:*mut inode,dentry:*mut dentry,mode:umode_t,rdev:dev_t)->i32 { xfs_generic_create(idmap,dir,dentry,mode,rdev,core::ptr::null_mut()) }
pub unsafe fn xfs_vn_create(idmap:*mut mnt_idmap,dir:*mut inode,dentry:*mut dentry,mode:umode_t)->i32 { xfs_generic_create(idmap,dir,dentry,mode,0,core::ptr::null_mut()) }
pub unsafe fn xfs_vn_mkdir(idmap:*mut mnt_idmap,dir:*mut inode,dentry:*mut dentry,mode:umode_t)->*mut dentry { ERR_PTR(xfs_generic_create(idmap,dir,dentry,mode,0,core::ptr::null_mut())) }

pub unsafe fn xfs_vn_lookup(dir:*mut inode,dentry:*mut dentry,_flags:u32)->*mut dentry {
    if (*dentry).d_name.len >= MAXNAMELEN { return ERR_PTR(-ENAMETOOLONG); }
    let mut name: xfs_name=core::mem::zeroed(); let mut cip:*mut xfs_inode=core::ptr::null_mut(); xfs_dentry_to_name(&mut name,dentry);
    let error=xfs_lookup(XFS_I(dir),&mut name,&mut cip,core::ptr::null_mut());
    let inode=if error==0 {VFS_I(cip)} else if error==-ENOENT {core::ptr::null_mut()} else {ERR_PTR(error) as *mut inode}; d_splice_alias(inode,dentry)
}

// Remaining VFS operations retain the C implementation's external-call structure.
pub unsafe fn xfs_vn_unlink(dir:*mut inode,dentry:*mut dentry)->i32 { let mut n:xfs_name=core::mem::zeroed(); xfs_dentry_to_name(&mut n,dentry); let e=xfs_remove(XFS_I(dir),&mut n,XFS_I(d_inode(dentry))); if e!=0{return e;} if xfs_has_asciici(XFS_M((*dir).i_sb)){d_invalidate(dentry);} 0 }
pub unsafe fn xfs_inode_supports_dax(ip:*mut xfs_inode)->bool { let mp=(*ip).i_mount; S_ISREG((*VFS_I(ip)).i_mode) && (*mp).m_sb.sb_blocksize==PAGE_SIZE && !xfs_inode_buftarg(ip).bt_daxdev.is_null() }
pub unsafe fn xfs_diflags_to_iflags(ip:*mut xfs_inode,init:bool) { let inode=VFS_I(ip); let xf=xfs_ip2xflags(ip); let mut f=0; if xf&FS_XFLAG_IMMUTABLE!=0{f|=S_IMMUTABLE;} if xf&FS_XFLAG_APPEND!=0{f|=S_APPEND;} if xf&FS_XFLAG_SYNC!=0{f|=S_SYNC;} if xf&FS_XFLAG_NOATIME!=0{f|=S_NOATIME;} if init&&xfs_inode_should_enable_dax(ip){f|=S_DAX;} (*inode).i_flags &= !(S_IMMUTABLE|S_APPEND|S_SYNC|S_NOATIME); (*inode).i_flags|=f; }
pub unsafe fn xfs_setup_inode(ip:*mut xfs_inode) { let inode=&mut (*ip).i_vnode; inode_state_set_raw(inode,I_NEW); inode_sb_list_add(inode); inode_fake_hash(inode); i_size_write(inode,(*ip).i_disk_size); xfs_diflags_to_iflags(ip,true); if xfs_is_internal_inode(ip){inode.i_flags|=S_PRIVATE; inode.i_opflags&=!IOP_XATTR;} }
pub unsafe fn xfs_setup_iops(ip:*mut xfs_inode) { let inode=&mut (*ip).i_vnode; match inode.i_mode&S_IFMT { S_IFREG=>{inode.i_op=&xfs_inode_operations;inode.i_fop=&xfs_file_operations;}, S_IFDIR=>{inode.i_op=if xfs_has_asciici(XFS_M(inode.i_sb)){&xfs_dir_ci_inode_operations}else{&xfs_dir_inode_operations};inode.i_fop=&xfs_dir_file_operations;}, S_IFLNK=>inode.i_op=&xfs_symlink_inode_operations, _=>{inode.i_op=&xfs_inode_operations;init_special_inode(inode,inode.i_mode,inode.i_rdev);} } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
