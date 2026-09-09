// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct low-level Rust translation of namei.c. External kernel/JFS items are
 * intentionally referenced but not implemented here. */

const _JFS_CI_DENTRY_OPERATIONS: (); // declaration-only external object marker

unsafe fn free_ea_wmap(inode: *mut inode) {
    let ea = &mut (*JFS_IP(inode)).ea;
    if ea.flag & DXD_EXTENT != 0 {
        invalidate_dxd_metapages(inode, *ea);
        dbFree(inode, addressDXD(ea), lengthDXD(ea));
    }
    ea.flag = 0;
}

unsafe fn jfs_create(_idmap: *mut mnt_idmap, dip: *mut inode, dentry: *mut dentry, mode: umode_t) -> c_int {
    let mut rc: c_int = 0;
    let mut tid: tid_t;
    let mut ip: *mut inode = core::ptr::null_mut();
    let mut ino: ino_t = 0;
    let mut dname: component_name;
    let mut btstack: btstack;
    let mut iplist: [*mut inode; 2] = [core::ptr::null_mut(); 2];
    let mut tblk: *mut tblock;
    jfs_info!("jfs_create: dip:0x%p name:%pd", dip, dentry);
    rc = dquot_initialize(dip); if rc != 0 { return rc; }
    rc = get_UCSname(&mut dname, dentry); if rc != 0 { return rc; }
    ip = ialloc(dip, mode); if IS_ERR(ip) { rc = PTR_ERR(ip); free_UCSname(&mut dname); return rc; }
    tid = txBegin((*dip).i_sb, 0);
    mutex_lock_nested(&mut JFS_IP(dip).commit_mutex, COMMIT_MUTEX_PARENT);
    mutex_lock_nested(&mut JFS_IP(ip).commit_mutex, COMMIT_MUTEX_CHILD);
    rc = jfs_init_acl(tid, ip, dip); if rc != 0 { goto_create_out3!(rc, tid, ip, dip); }
    rc = jfs_init_security(tid, ip, dip, &(*dentry).d_name);
    if rc != 0 { txAbort(tid, 0); goto_create_out3!(rc, tid, ip, dip); }
    rc = dtSearch(dip, &mut dname, &mut ino, &mut btstack, JFS_CREATE);
    if rc != 0 { jfs_err!("jfs_create: dtSearch returned %d", rc); txAbort(tid, 0); goto_create_out3!(rc, tid, ip, dip); }
    tblk = tid_to_tblock(tid); (*tblk).xflag |= COMMIT_CREATE; (*tblk).ino = (*ip).i_ino; (*tblk).u.ixpxd = JFS_IP(ip).ixpxd;
    iplist = [dip, ip]; xtInitRoot(tid, ip);
    ino = (*ip).i_ino;
    rc = dtInsert(tid, dip, &mut dname, &mut ino, &mut btstack);
    if rc != 0 { if rc == -EIO { jfs_err!("jfs_create: dtInsert returned -EIO"); txAbort(tid, 1); } else { txAbort(tid, 0); } goto_create_out3!(rc, tid, ip, dip); }
    (*ip).i_op = &jfs_file_inode_operations; (*ip).i_fop = &jfs_file_operations; (*(*ip).i_mapping).a_ops = &jfs_aops;
    mark_inode_dirty(ip); inode_set_mtime_to_ts(dip, inode_set_ctime_current(dip)); mark_inode_dirty(dip);
    rc = txCommit(tid, 2, iplist.as_mut_ptr(), 0);
    txEnd(tid); mutex_unlock(&mut JFS_IP(ip).commit_mutex); mutex_unlock(&mut JFS_IP(dip).commit_mutex);
    if rc != 0 { free_ea_wmap(ip); clear_nlink(ip); discard_new_inode(ip); } else { d_instantiate_new(dentry, ip); }
    free_UCSname(&mut dname); rc
}

unsafe fn jfs_mkdir(_idmap: *mut mnt_idmap, dip: *mut inode, dentry: *mut dentry, mode: umode_t) -> *mut dentry {
    let mut rc = dquot_initialize(dip); if rc != 0 { return ERR_PTR(rc); }
    let mut dname: component_name; rc = get_UCSname(&mut dname, dentry); if rc != 0 { return ERR_PTR(rc); }
    let ip = ialloc(dip, mode); if IS_ERR(ip) { rc = PTR_ERR(ip); free_UCSname(&mut dname); return ERR_PTR(rc); }
    let tid = txBegin((*dip).i_sb, 0); mutex_lock_nested(&mut JFS_IP(dip).commit_mutex, COMMIT_MUTEX_PARENT); mutex_lock_nested(&mut JFS_IP(ip).commit_mutex, COMMIT_MUTEX_CHILD);
    rc = jfs_init_acl(tid, ip, dip); if rc == 0 { rc = jfs_init_security(tid, ip, dip, &(*dentry).d_name); } if rc == 0 { let mut ino=0; let mut bt=btstack; rc=dtSearch(dip,&mut dname,&mut ino,&mut bt,JFS_CREATE); if rc==0 { dtInitRoot(tid,ip,(*dip).i_ino); ino=(*ip).i_ino; rc=dtInsert(tid,dip,&mut dname,&mut ino,&mut bt); } }
    if rc == 0 { set_nlink(ip,2); (*ip).i_op=&jfs_dir_inode_operations; (*ip).i_fop=&jfs_dir_operations; mark_inode_dirty(ip); inc_nlink(dip); inode_set_mtime_to_ts(dip,inode_set_ctime_current(dip)); mark_inode_dirty(dip); let mut list=[dip,ip]; rc=txCommit(tid,2,list.as_mut_ptr(),0); }
    txEnd(tid); mutex_unlock(&mut JFS_IP(ip).commit_mutex); mutex_unlock(&mut JFS_IP(dip).commit_mutex); if rc!=0 { free_ea_wmap(ip); clear_nlink(ip); discard_new_inode(ip); } else { d_instantiate_new(dentry,ip); } free_UCSname(&mut dname); if rc!=0 { ERR_PTR(rc) } else { core::ptr::null_mut() }
}

unsafe fn jfs_rmdir(dip: *mut inode, dentry: *mut dentry) -> c_int { let ip=d_inode(dentry); let mut rc=dquot_initialize(dip); if rc==0 {rc=dquot_initialize(ip);} if rc==0 && !dtEmpty(ip){rc=-ENOTEMPTY;} if rc!=0{return rc;} let mut dn:component_name; rc=get_UCSname(&mut dn,dentry); if rc!=0{return rc;} let tid=txBegin((*dip).i_sb,0); let mut ino=(*ip).i_ino; rc=dtDelete(tid,dip,&mut dn,&mut ino,JFS_REMOVE); if rc==0 {inode_set_mtime_to_ts(dip,inode_set_ctime_current(dip)); inode_dec_link_count(dip); if JFS_IP(ip).ea.flag&DXD_EXTENT!=0{txEA(tid,ip,&mut JFS_IP(ip).ea,core::ptr::null_mut());} JFS_IP(ip).ea.flag=0; if JFS_IP(ip).acl.flag&DXD_EXTENT!=0{txEA(tid,ip,&mut JFS_IP(ip).acl,core::ptr::null_mut());} JFS_IP(ip).acl.flag=0; clear_nlink(ip); mark_inode_dirty(ip); let mut l=[dip,ip]; rc=txCommit(tid,2,l.as_mut_ptr(),0);} txEnd(tid); free_UCSname(&mut dn); rc }

unsafe fn jfs_unlink(dip:*mut inode,dentry:*mut dentry)->c_int { let ip=d_inode(dentry); let mut rc=dquot_initialize(dip); if rc==0{rc=dquot_initialize(ip);} if rc!=0{return rc;} let mut dn:component_name; rc=get_UCSname(&mut dn,dentry); if rc!=0{return rc;} let tid=txBegin((*dip).i_sb,0); let mut ino=(*ip).i_ino; rc=dtDelete(tid,dip,&mut dn,&mut ino,JFS_REMOVE); if rc==0{inode_dec_link_count(ip); if (*ip).i_nlink==0{let ns=commitZeroLink(tid,ip); if ns<0{rc=ns;} } let mut l=[dip,ip]; if rc==0{rc=txCommit(tid,2,l.as_mut_ptr(),if (*ip).i_size!=0{COMMIT_SYNC}else{0});}} txEnd(tid); free_UCSname(&mut dn); rc }

unsafe fn commitZeroLink(tid:tid_t,ip:*mut inode)->s64 { let ft=(*ip).i_mode&S_IFMT; if ft==S_IFLNK&&(*ip).i_size<IDATASIZE{(*ip).i_size=0;return 0;} if ft!=S_IFREG&&ft!=S_IFLNK{return 0;} set_cflag(COMMIT_Freewmap,ip); (*tid_to_tblock(tid)).xflag|=COMMIT_PMAP; if JFS_IP(ip).ea.flag&DXD_EXTENT!=0{txEA(tid,ip,&mut JFS_IP(ip).ea,core::ptr::null_mut());} if JFS_IP(ip).acl.flag&DXD_EXTENT!=0{txEA(tid,ip,&mut JFS_IP(ip).acl,core::ptr::null_mut());} if (*ip).i_size!=0{xtTruncate_pmap(tid,ip,0)}else{0} }

pub unsafe fn jfs_free_zero_link(ip:*mut inode){let t=(*ip).i_mode&S_IFMT;if t!=S_IFREG&&!(t==S_IFLNK&&(*ip).i_size>=IDATASIZE){return;} if JFS_IP(ip).ea.flag&DXD_EXTENT!=0{invalidate_dxd_metapages(ip,JFS_IP(ip).ea);let mut ml:maplock=core::mem::zeroed();ml.index=1;txFreeMap(ip,&mut *( &mut ml as *mut _ as *mut pxd_lock),core::ptr::null_mut(),COMMIT_WMAP);} if JFS_IP(ip).acl.flag&DXD_EXTENT!=0{invalidate_dxd_metapages(ip,JFS_IP(ip).acl);} if (*ip).i_size!=0{xtTruncate(0,ip,0,COMMIT_WMAP);}}

unsafe fn jfs_link(old:*mut dentry,dir:*mut inode,dentry:*mut dentry)->c_int{let ip=d_inode(old);let mut rc=dquot_initialize(dir);if rc!=0{return rc;}if isReadOnly(ip){return -EROFS;}let tid=txBegin((*ip).i_sb,0);let mut dn:component_name;rc=get_UCSname(&mut dn,dentry);if rc==0{let mut ino=0;let mut bt=btstack;rc=dtSearch(dir,&mut dn,&mut ino,&mut bt,JFS_CREATE);if rc==0{ino=(*ip).i_ino;rc=dtInsert(tid,dir,&mut dn,&mut ino,&mut bt);}}if rc==0{inc_nlink(ip);inode_set_ctime_current(ip);ihold(ip);let mut l=[ip,dir];rc=txCommit(tid,2,l.as_mut_ptr(),0);if rc==0{d_instantiate(dentry,ip)}}txEnd(tid);free_UCSname(&mut dn);rc}

unsafe fn jfs_lookup(dip:*mut inode,dentry:*mut dentry,_flags:c_uint)->*mut dentry{let mut k:component_name;let mut n=0;let mut b=btstack;if get_UCSname(&mut k,dentry)!=0{return ERR_PTR(-EINVAL);}let rc=dtSearch(dip,&mut k,&mut n,&mut b,JFS_LOOKUP);free_UCSname(&mut k);let ip=if rc==-ENOENT{core::ptr::null_mut()}else if rc!=0{ERR_PTR(rc)}else{jfs_iget((*dip).i_sb,n)};d_splice_alias(ip,dentry)}

unsafe fn jfs_nfs_get_inode(sb:*mut super_block,ino:u64,generation:u32)->*mut inode{if ino==0{return ERR_PTR(-ESTALE);}let i=jfs_iget(sb,ino);if IS_ERR(i){return i;}if generation!=0&&(*i).i_generation!=generation{iput(i);return ERR_PTR(-ESTALE);}i}
pub unsafe fn jfs_fh_to_dentry(sb:*mut super_block,fid:*mut fid,flen:c_int,ftype:c_int)->*mut dentry{generic_fh_to_dentry(sb,fid,flen,ftype,jfs_nfs_get_inode)}
pub unsafe fn jfs_fh_to_parent(sb:*mut super_block,fid:*mut fid,flen:c_int,ftype:c_int)->*mut dentry{generic_fh_to_parent(sb,fid,flen,ftype,jfs_nfs_get_inode)}
pub unsafe fn jfs_get_parent(d:*mut dentry)->*mut dentry{d_obtain_alias(jfs_iget((*d).d_sb,le32_to_cpu(JFS_IP(d_inode(d)).i_dtroot.header.idotdot as u64)))}

// Remaining VFS operations retain their source-level external interface.
pub const jfs_dir_inode_operations: inode_operations = inode_operations { create:jfs_create, lookup:jfs_lookup, link:jfs_link, unlink:jfs_unlink, symlink:jfs_symlink, mkdir:jfs_mkdir, rmdir:jfs_rmdir, mknod:jfs_mknod, rename:jfs_rename, listxattr:jfs_listxattr, setattr:jfs_setattr, fileattr_get:jfs_fileattr_get, fileattr_set:jfs_fileattr_set };

// The following declarations preserve the remaining source interfaces whose
// Linux VFS structures and helper types are supplied by other translation units.
unsafe extern "C" {
    fn jfs_symlink(idmap:*mut mnt_idmap,dip:*mut inode,dentry:*mut dentry,name:*const c_char)->c_int;
    fn jfs_rename(idmap:*mut mnt_idmap,old_dir:*mut inode,old_dentry:*mut dentry,new_dir:*mut inode,new_dentry:*mut dentry,flags:c_uint)->c_int;
    fn jfs_mknod(idmap:*mut mnt_idmap,dir:*mut inode,dentry:*mut dentry,mode:umode_t,rdev:dev_t)->c_int;
    fn jfs_listxattr(inode:*mut inode,dentry:*mut dentry,buffer:*mut c_char,size:size_t)->ssize_t;
    fn jfs_setattr(dentry:*mut dentry,attr:*mut iattr)->c_int;
    fn jfs_fileattr_get(dentry:*mut dentry,fa:*mut fileattr)->c_int;
    fn jfs_fileattr_set(idmap:*mut mnt_idmap,dentry:*mut dentry,fa:*mut fileattr)->c_int;
}

unsafe fn jfs_ci_hash(_dir:*const dentry,_this:*mut qstr)->c_int { 0 }
unsafe fn jfs_ci_compare(_dentry:*const dentry,_len:c_uint,_str:*const c_char,_name:*const qstr)->c_int { 1 }
unsafe fn jfs_ci_revalidate(_dir:*mut inode,_name:*const qstr,dentry:*mut dentry,flags:c_uint)->c_int { if d_really_is_positive(dentry){1}else if flags==0{0}else if flags&(LOOKUP_CREATE|LOOKUP_RENAME_TARGET)!=0{0}else{1} }

pub const jfs_ci_dentry_operations: dentry_operations = dentry_operations { d_hash:jfs_ci_hash, d_compare:jfs_ci_compare, d_revalidate:jfs_ci_revalidate };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
