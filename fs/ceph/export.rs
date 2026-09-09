// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel/Ceph translation.

#[repr(C, packed)]
pub struct ceph_nfs_fh { pub ino: u64 }
#[repr(C, packed)]
pub struct ceph_nfs_confh { pub ino: u64, pub parent_ino: u64 }
#[repr(C, packed)]
pub struct ceph_nfs_snapfh { pub ino: u64, pub snapid: u64, pub parent_ino: u64, pub hash: u32 }

const BYTES_PER_U32: usize = core::mem::size_of::<u32>();
const CEPH_FH_BASIC_SIZE: usize = core::mem::size_of::<ceph_nfs_fh>() / BYTES_PER_U32;
const CEPH_FH_WITH_PARENT_SIZE: usize = core::mem::size_of::<ceph_nfs_confh>() / BYTES_PER_U32;
const CEPH_FH_SNAPPED_INODE_SIZE: usize = core::mem::size_of::<ceph_nfs_snapfh>() / BYTES_PER_U32;

unsafe fn ceph_encode_snapfh(inode: *mut inode, rawfh: *mut u32, max_len: *mut i32, _parent_inode: *mut inode) -> i32 {
    let cl = ceph_inode_to_client(inode);
    let sfh = rawfh as *mut ceph_nfs_snapfh;
    let snapid = ceph_snap(inode);
    let mut no_parent = true;
    if (*max_len as usize) < CEPH_FH_SNAPPED_INODE_SIZE { *max_len = CEPH_FH_SNAPPED_INODE_SIZE as i32; return FILEID_INVALID; }
    let mut ret = -EINVAL;
    if snapid != CEPH_SNAPDIR {
        let dentry = d_find_alias(inode);
        if !dentry.is_null() {
            rcu_read_lock();
            let dir = d_inode_rcu((*dentry).d_parent);
            if ceph_snap(dir) != CEPH_SNAPDIR { (*sfh).parent_ino = ceph_ino(dir); (*sfh).hash = ceph_dentry_hash(dir, dentry); no_parent = false; }
            rcu_read_unlock(); dput(dentry);
        }
    }
    if no_parent { if !S_ISDIR((*inode).i_mode) { return ret; } (*sfh).parent_ino = (*sfh).ino; (*sfh).hash = 0; }
    (*sfh).ino = ceph_ino(inode); (*sfh).snapid = snapid; *max_len = CEPH_FH_SNAPPED_INODE_SIZE as i32;
    ret = FILEID_BTRFS_WITH_PARENT;
    doutc(cl, "%p %llx.%llx ret=%d\n", inode, ceph_vinop(inode), ret); ret
}

unsafe fn ceph_encode_fh(inode: *mut inode, rawfh: *mut u32, max_len: *mut i32, parent_inode: *mut inode) -> i32 {
    let cl = ceph_inode_to_client(inode);
    if ceph_snap(inode) != CEPH_NOSNAP { return ceph_encode_snapfh(inode, rawfh, max_len, parent_inode); }
    if !parent_inode.is_null() && (*max_len as usize) < CEPH_FH_WITH_PARENT_SIZE { *max_len = CEPH_FH_WITH_PARENT_SIZE as i32; return FILEID_INVALID; }
    if (*max_len as usize) < CEPH_FH_BASIC_SIZE { *max_len = CEPH_FH_BASIC_SIZE as i32; return FILEID_INVALID; }
    if !parent_inode.is_null() { let f = rawfh as *mut ceph_nfs_confh; doutc(cl, "%p %llx.%llx with parent %p %llx.%llx\n", inode, ceph_vinop(inode), parent_inode, ceph_vinop(parent_inode)); (*f).ino=ceph_ino(inode); (*f).parent_ino=ceph_ino(parent_inode); *max_len=CEPH_FH_WITH_PARENT_SIZE as i32; FILEID_INO32_GEN_PARENT } else { let f=rawfh as *mut ceph_nfs_fh; doutc(cl, "%p %llx.%llx\n", inode, ceph_vinop(inode)); (*f).ino=ceph_ino(inode); *max_len=CEPH_FH_BASIC_SIZE as i32; FILEID_INO32_GEN }
}

unsafe fn __lookup_inode(sb: *mut super_block, ino: u64) -> *mut inode {
    let mdsc=(*ceph_sb_to_fs_client(sb)).mdsc; let mut vino=ceph_vino{ino, snap:CEPH_NOSNAP};
    if ceph_vino_is_reserved(vino) { return ERR_PTR(-ESTALE); }
    let mut inode=ceph_find_inode(sb,vino); if inode.is_null() { let req=ceph_mdsc_create_request(mdsc,CEPH_MDS_OP_LOOKUPINO,USE_ANY_MDS); if IS_ERR(req){return ERR_CAST(req);} let mut mask=CEPH_STAT_CAP_INODE; if ceph_security_xattr_wanted(d_inode((*sb).s_root)){mask|=CEPH_CAP_XATTR_SHARED;} (*req).r_args.lookupino.mask=cpu_to_le32(mask); (*req).r_ino1=vino; (*req).r_num_caps=1; let err=ceph_mdsc_do_request(mdsc,core::ptr::null_mut(),req); inode=(*req).r_target_inode; if !inode.is_null(){ihold(inode);} ceph_mdsc_put_request(req); if inode.is_null(){return if err<0{ERR_PTR(err)}else{ERR_PTR(-ESTALE)};} } else if ceph_inode_is_shutdown(inode){iput(inode);return ERR_PTR(-ESTALE);} inode
}

pub unsafe fn ceph_lookup_inode(sb:*mut super_block, ino:u64)->*mut inode { let i=__lookup_inode(sb,ino); if IS_ERR(i){return i;} if (*i).i_nlink==0{iput(i);return ERR_PTR(-ESTALE);} i }

unsafe fn __fh_to_dentry(sb:*mut super_block, ino:u64)->*mut dentry { let i=__lookup_inode(sb,ino); if IS_ERR(i){return ERR_CAST(i);} let ci=ceph_inode(i); let err=ceph_do_getattr(i,CEPH_CAP_LINK_SHARED,false); if err!=0{iput(i);return ERR_PTR(err);} if (*i).i_nlink==0&&!__ceph_is_file_opened(ci){iput(i);return ERR_PTR(-ESTALE);} d_obtain_alias(i) }

unsafe fn __snapfh_to_dentry(sb:*mut super_block,sfh:*mut ceph_nfs_snapfh,want_parent:bool)->*mut dentry {
    let mut vino=ceph_vino{ino:if want_parent{(*sfh).parent_ino}else{(*sfh).ino},snap:if want_parent {if (*sfh).snapid==CEPH_SNAPDIR{CEPH_NOSNAP}else if (*sfh).ino==(*sfh).parent_ino{CEPH_SNAPDIR}else{(*sfh).snapid}}else{(*sfh).snapid}};
    if ceph_vino_is_reserved(vino){return ERR_PTR(-ESTALE);} let i=ceph_find_inode(sb,vino); if !i.is_null(){if ceph_inode_is_shutdown(i){iput(i);return ERR_PTR(-ESTALE);}return d_obtain_alias(i);}
    let mdsc=(*ceph_sb_to_fs_client(sb)).mdsc; let req=ceph_mdsc_create_request(mdsc,CEPH_MDS_OP_LOOKUPINO,USE_ANY_MDS); if IS_ERR(req){return ERR_CAST(req);} let mut mask=CEPH_STAT_CAP_INODE; if ceph_security_xattr_wanted(d_inode((*sb).s_root)){mask|=CEPH_CAP_XATTR_SHARED;} (*req).r_args.lookupino.mask=cpu_to_le32(mask); if vino.snap<CEPH_NOSNAP{(*req).r_args.lookupino.snapid=cpu_to_le64(vino.snap);if !want_parent&&(*sfh).ino!=(*sfh).parent_ino{(*req).r_args.lookupino.parent=cpu_to_le64((*sfh).parent_ino);(*req).r_args.lookupino.hash=cpu_to_le32((*sfh).hash);}} (*req).r_ino1=vino;(*req).r_num_caps=1;let _=ceph_mdsc_do_request(mdsc,core::ptr::null_mut(),req);let i=(*req).r_target_inode;let d=if i.is_null(){ERR_PTR(-ESTALE)}else{ihold(i);d_obtain_alias(i)};ceph_mdsc_put_request(req);d
}

unsafe fn __get_parent(sb:*mut super_block,child:*mut dentry,ino:u64)->*mut dentry { let mdsc=(*ceph_sb_to_fs_client(sb)).mdsc;let req=ceph_mdsc_create_request(mdsc,CEPH_MDS_OP_LOOKUPPARENT,USE_ANY_MDS);if IS_ERR(req){return ERR_CAST(req);}if !child.is_null(){(*req).r_inode=d_inode(child);ihold((*req).r_inode);}else{(*req).r_ino1=ceph_vino{ino,snap:CEPH_NOSNAP};}(*req).r_num_caps=1;let err=ceph_mdsc_do_request(mdsc,core::ptr::null_mut(),req);let i=(*req).r_target_inode;ceph_mdsc_put_request(req);if err!=0||i.is_null(){return ERR_PTR(if err!=0{err}else{-ENOENT});}ihold(i);d_obtain_alias(i)}

unsafe fn ceph_get_parent(child:*mut dentry)->*mut dentry { let i=d_inode(child); if ceph_snap(i)!=CEPH_NOSNAP&&!d_is_dir(child){return ERR_PTR(-EINVAL);} __get_parent((*i).i_sb,child,0) }
unsafe fn ceph_get_name(_parent:*mut dentry,_name:*mut i8,_child:*mut dentry)->i32 { -EOPNOTSUPP }

unsafe fn ceph_fh_to_dentry(sb:*mut super_block,fid:*mut fid,fh_len:i32,fh_type:i32)->*mut dentry { let fh=(*fid).raw.as_mut_ptr() as *mut ceph_nfs_fh; if fh_type==FILEID_BTRFS_WITH_PARENT{return __snapfh_to_dentry(sb,fh as *mut ceph_nfs_snapfh,false);} if fh_type!=FILEID_INO32_GEN&&fh_type!=FILEID_INO32_GEN_PARENT{return core::ptr::null_mut();} if fh_len<CEPH_FH_BASIC_SIZE as i32{return core::ptr::null_mut();} __fh_to_dentry(sb,(*fh).ino) }

// The remaining exportfs callbacks retain the kernel request machinery and layouts.
unsafe fn ceph_fh_to_parent(sb:*mut super_block,fid:*mut fid,fh_len:i32,fh_type:i32)->*mut dentry { let c=(*fid).raw.as_mut_ptr() as *mut ceph_nfs_confh; if fh_type==FILEID_BTRFS_WITH_PARENT{return __snapfh_to_dentry(sb,c as *mut ceph_nfs_snapfh,true);} if fh_type!=FILEID_INO32_GEN_PARENT||fh_len<CEPH_FH_WITH_PARENT_SIZE as i32{return core::ptr::null_mut();} let d=__get_parent(sb,core::ptr::null_mut(),(*c).ino); if d==ERR_PTR(-ENOENT){__fh_to_dentry(sb,(*c).parent_ino)}else{d} }

pub static ceph_export_ops: export_operations = export_operations { encode_fh: Some(ceph_encode_fh), fh_to_dentry: Some(ceph_fh_to_dentry), fh_to_parent: Some(ceph_fh_to_parent), get_parent: Some(ceph_get_parent), get_name: Some(ceph_get_name) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
