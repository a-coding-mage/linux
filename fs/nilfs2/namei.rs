// SPDX-License-Identifier: GPL-2.0+
/*
 * NILFS pathname lookup operations.
 *
 * Copyright (C) 2005-2008 Nippon Telegraph and Telephone Corporation.
 *
 * Modified for NILFS by Amagai Yoshiji and Ryusuke Konishi.
 */
/*
 *  linux/fs/ext2/namei.c
 *
 * Copyright (C) 1992, 1993, 1994, 1995
 * Remy Card (card@masi.ibp.fr)
 * Laboratoire MASI - Institut Blaise Pascal
 * Universite Pierre et Marie Curie (Paris VI)
 *
 *  from
 *
 *  linux/fs/minix/namei.c
 *
 *  Copyright (C) 1991, 1992  Linus Torvalds
 *
 *  Big-endian to little-endian byte-swapping/bitmaps by
 *        David S. Miller (davem@caip.rutgers.edu), 1995
 */

// Dependencies supplied by the surrounding kernel translation unit.

const NILFS_FID_SIZE_NON_CONNECTABLE: usize = offset_of!(nilfs_fid, parent_gen) / 4;
const NILFS_FID_SIZE_CONNECTABLE: usize = size_of::<nilfs_fid>() / 4;

unsafe fn nilfs_add_nondir(dentry: *mut dentry, inode: *mut inode) -> c_int {
    let err = nilfs_add_link(dentry, inode);
    if err == 0 {
        d_instantiate_new(dentry, inode);
        return 0;
    }
    inode_dec_link_count(inode);
    unlock_new_inode(inode);
    iput(inode);
    err
}

/* Methods themselves. */

unsafe fn nilfs_lookup(dir: *mut inode, dentry: *mut dentry, _flags: c_uint) -> *mut dentry {
    let mut inode: *mut inode;
    let mut ino: u64 = 0;
    let res: c_int;
    if (*dentry).d_name.len > NILFS_NAME_LEN { return ERR_PTR(-ENAMETOOLONG); }
    res = nilfs_inode_by_name(dir, &(*dentry).d_name, &mut ino);
    if res != 0 {
        if res != -ENOENT { return ERR_PTR(res); }
        inode = core::ptr::null_mut();
    } else {
        inode = nilfs_iget((*dir).i_sb, NILFS_I(dir).i_root, ino);
        if inode == ERR_PTR(-ESTALE) {
            nilfs_error((*dir).i_sb, c"deleted inode referenced: %llu", ino);
            return ERR_PTR(-EIO);
        }
    }
    d_splice_alias(inode, dentry)
}

unsafe fn nilfs_create(_idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> c_int {
    let mut ti: nilfs_transaction_info = core::mem::zeroed();
    let mut err = nilfs_transaction_begin((*dir).i_sb, &mut ti, 1);
    if err != 0 { return err; }
    let inode = nilfs_new_inode(dir, mode);
    err = PTR_ERR(inode);
    if !IS_ERR(inode) {
        (*inode).i_op = &nilfs_file_inode_operations;
        (*inode).i_fop = &nilfs_file_operations;
        (*(*inode).i_mapping).a_ops = &nilfs_aops;
        nilfs_mark_inode_dirty(inode);
        err = nilfs_add_nondir(dentry, inode);
    }
    if err == 0 { err = nilfs_transaction_commit((*dir).i_sb); }
    else { nilfs_transaction_abort((*dir).i_sb); }
    err
}

unsafe fn nilfs_mknod(_idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t, rdev: dev_t) -> c_int {
    let mut ti: nilfs_transaction_info = core::mem::zeroed();
    let mut err = nilfs_transaction_begin((*dir).i_sb, &mut ti, 1);
    if err != 0 { return err; }
    let inode = nilfs_new_inode(dir, mode);
    err = PTR_ERR(inode);
    if !IS_ERR(inode) {
        init_special_inode(inode, (*inode).i_mode, rdev);
        nilfs_mark_inode_dirty(inode);
        err = nilfs_add_nondir(dentry, inode);
    }
    if err == 0 { err = nilfs_transaction_commit((*dir).i_sb); }
    else { nilfs_transaction_abort((*dir).i_sb); }
    err
}

unsafe fn nilfs_symlink(_idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, symname: *const c_char) -> c_int {
    let mut ti: nilfs_transaction_info = core::mem::zeroed();
    let sb = (*dir).i_sb;
    let l = strlen(symname) + 1;
    if l > (*sb).s_blocksize as usize { return -ENAMETOOLONG; }
    let mut err = nilfs_transaction_begin((*dir).i_sb, &mut ti, 1);
    if err != 0 { return err; }
    let inode = nilfs_new_inode(dir, S_IFLNK | 0o777);
    err = PTR_ERR(inode);
    if IS_ERR(inode) { return nilfs_transaction_abort((*dir).i_sb); }
    (*inode).i_op = &nilfs_symlink_inode_operations;
    inode_nohighmem(inode);
    mapping_set_gfp_mask((*inode).i_mapping, mapping_gfp_constraint((*inode).i_mapping, !__GFP_FS));
    (*(*inode).i_mapping).a_ops = &nilfs_aops;
    err = page_symlink(inode, symname, l);
    if err != 0 {
        drop_nlink(inode); nilfs_mark_inode_dirty(inode); unlock_new_inode(inode); iput(inode);
        nilfs_transaction_abort((*dir).i_sb); return err;
    }
    err = nilfs_add_nondir(dentry, inode);
    if err == 0 { nilfs_transaction_commit((*dir).i_sb) } else { nilfs_transaction_abort((*dir).i_sb) }
    err
}

unsafe fn nilfs_link(old_dentry: *mut dentry, dir: *mut inode, dentry: *mut dentry) -> c_int {
    let inode = d_inode(old_dentry); let mut ti: nilfs_transaction_info = core::mem::zeroed();
    let mut err = nilfs_transaction_begin((*dir).i_sb, &mut ti, 1); if err != 0 { return err; }
    inode_set_ctime_current(inode); inode_inc_link_count(inode); ihold(inode);
    err = nilfs_add_link(dentry, inode);
    if err == 0 { d_instantiate(dentry, inode); nilfs_transaction_commit((*dir).i_sb) }
    else { inode_dec_link_count(inode); iput(inode); nilfs_transaction_abort((*dir).i_sb); } err
}

unsafe fn nilfs_mkdir(_idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> *mut dentry {
    let mut ti: nilfs_transaction_info = core::mem::zeroed(); let mut err = nilfs_transaction_begin((*dir).i_sb, &mut ti, 1);
    if err != 0 { return ERR_PTR(err); } inc_nlink(dir);
    let inode = nilfs_new_inode(dir, mode); err = PTR_ERR(inode);
    if IS_ERR(inode) { drop_nlink(dir); nilfs_mark_inode_dirty(dir); nilfs_transaction_abort((*dir).i_sb); return ERR_PTR(err); }
    (*inode).i_op = &nilfs_dir_inode_operations; (*inode).i_fop = &nilfs_dir_operations; (*(*inode).i_mapping).a_ops = &nilfs_aops; inc_nlink(inode);
    err = nilfs_make_empty(inode, dir);
    if err == 0 { err = nilfs_add_link(dentry, inode); }
    if err == 0 { nilfs_mark_inode_dirty(inode); d_instantiate_new(dentry, inode); }
    else { drop_nlink(inode); drop_nlink(inode); nilfs_mark_inode_dirty(inode); unlock_new_inode(inode); iput(inode); drop_nlink(dir); nilfs_mark_inode_dirty(dir); }
    if err == 0 { nilfs_transaction_commit((*dir).i_sb); } else { nilfs_transaction_abort((*dir).i_sb); } if err != 0 { ERR_PTR(err) } else { core::ptr::null_mut() }
}

unsafe fn nilfs_do_unlink(dir: *mut inode, dentry: *mut dentry) -> c_int {
    let mut folio: *mut folio = core::ptr::null_mut(); let de = nilfs_find_entry(dir, &(*dentry).d_name, &mut folio); if IS_ERR(de) { return PTR_ERR(de); }
    let inode = d_inode(dentry); if le64_to_cpu((*de).inode) != (*inode).i_ino { return -EIO; }
    if (*inode).i_nlink == 0 { nilfs_warn((*inode).i_sb, c"deleting nonexistent file (ino=%llu), %d", (*inode).i_ino, (*inode).i_nlink); set_nlink(inode, 1); }
    let mut err = nilfs_delete_entry(de, folio); folio_release_kmap(folio, de); if err != 0 { return err; }
    inode_set_ctime_to_ts(inode, inode_get_ctime(dir)); drop_nlink(inode); err
}

unsafe fn nilfs_unlink(dir: *mut inode, dentry: *mut dentry) -> c_int { let mut ti: nilfs_transaction_info = core::mem::zeroed(); let mut err = nilfs_transaction_begin((*dir).i_sb, &mut ti, 0); if err != 0 { return err; } err = nilfs_do_unlink(dir,dentry); if err == 0 { nilfs_mark_inode_dirty(dir); nilfs_mark_inode_dirty(d_inode(dentry)); nilfs_transaction_commit((*dir).i_sb) } else { nilfs_transaction_abort((*dir).i_sb); } err }

unsafe fn nilfs_rmdir(dir: *mut inode, dentry: *mut dentry) -> c_int { let inode=d_inode(dentry); let mut ti: nilfs_transaction_info=core::mem::zeroed(); let mut err=nilfs_transaction_begin((*dir).i_sb,&mut ti,0); if err!=0{return err;} err=-ENOTEMPTY; if nilfs_empty_dir(inode){err=nilfs_do_unlink(dir,dentry); if err==0{(*inode).i_size=0;drop_nlink(inode);nilfs_mark_inode_dirty(inode);drop_nlink(dir);nilfs_mark_inode_dirty(dir);}} if err==0{nilfs_transaction_commit((*dir).i_sb)}else{nilfs_transaction_abort((*dir).i_sb)} err }

unsafe fn nilfs_rename(_idmap: *mut mnt_idmap, old_dir: *mut inode, old_dentry: *mut dentry, new_dir: *mut inode, new_dentry: *mut dentry, flags: c_uint) -> c_int {
    if flags & !RENAME_NOREPLACE != 0 { return -EINVAL; }
    let mut ti: nilfs_transaction_info = core::mem::zeroed(); let mut err=nilfs_transaction_begin((*old_dir).i_sb,&mut ti,1); if err!=0{return err;}
    let old_inode=d_inode(old_dentry); let new_inode=d_inode(new_dentry); let old_is_dir=S_ISDIR((*old_inode).i_mode); let mut old_folio: *mut folio=core::ptr::null_mut(); let old_de=nilfs_find_entry(old_dir,&(*old_dentry).d_name,&mut old_folio); if IS_ERR(old_de){return PTR_ERR(old_de);}
    let mut dir_folio: *mut folio=core::ptr::null_mut(); let mut dir_de: *mut nilfs_dir_entry=core::ptr::null_mut();
    if old_is_dir && old_dir != new_dir { err=-EIO; dir_de=nilfs_dotdot(old_inode,&mut dir_folio); if dir_de.is_null(){folio_release_kmap(old_folio,old_de);nilfs_transaction_abort((*old_dir).i_sb);return err;} }
    if !new_inode.is_null() { if old_is_dir && !nilfs_empty_dir(new_inode){err=-ENOTEMPTY;} else {let mut nf: *mut folio=core::ptr::null_mut(); let nd=nilfs_find_entry(new_dir,&(*new_dentry).d_name,&mut nf); if IS_ERR(nd){err=PTR_ERR(nd);} else {err=nilfs_set_link(new_dir,nd,nf,old_inode);folio_release_kmap(nf,nd);if err==0{nilfs_mark_inode_dirty(new_dir);inode_set_ctime_current(new_inode);if old_is_dir{drop_nlink(new_inode);}drop_nlink(new_inode);nilfs_mark_inode_dirty(new_inode);}}} }
    else if err==0 {err=nilfs_add_link(new_dentry,old_inode);if err==0&&old_is_dir{inc_nlink(new_dir);nilfs_mark_inode_dirty(new_dir);}}
    if err==0 {inode_set_ctime_current(old_inode);err=nilfs_delete_entry(old_de,old_folio);if err==0{if old_is_dir&&old_dir!=new_dir{err=nilfs_set_link(old_inode,dir_de,dir_folio,new_dir);}if old_is_dir{drop_nlink(old_dir);}nilfs_mark_inode_dirty(old_dir);}nilfs_mark_inode_dirty(old_inode);}
    if !dir_de.is_null(){folio_release_kmap(dir_folio,dir_de);} folio_release_kmap(old_folio,old_de); if err==0{nilfs_transaction_commit((*old_dir).i_sb)}else{nilfs_transaction_abort((*old_dir).i_sb)} err
}

unsafe fn nilfs_get_parent(child: *mut dentry) -> *mut dentry { let mut ino=0u64; let r=nilfs_inode_by_name(d_inode(child),&dotdot_name,&mut ino); if r!=0{return ERR_PTR(r);} d_obtain_alias(nilfs_iget((*child).d_sb,NILFS_I(d_inode(child)).i_root,ino)) }
unsafe fn nilfs_get_dentry(sb: *mut super_block,cno:u64,ino:u64,gen:u32)->*mut dentry { if ino<NILFS_FIRST_INO(sb)&&ino!=NILFS_ROOT_INO{return ERR_PTR(-ESTALE);} let root=nilfs_lookup_root((*sb).s_fs_info,cno);if root.is_null(){return ERR_PTR(-ESTALE);}let inode=nilfs_iget(sb,root,ino);nilfs_put_root(root);if IS_ERR(inode){return ERR_CAST(inode);}if gen!=0&&(*inode).i_generation!=gen{iput(inode);return ERR_PTR(-ESTALE);}d_obtain_alias(inode) }
unsafe fn nilfs_fh_to_dentry(sb:*mut super_block,fh:*mut fid,fh_len:c_int,fh_type:c_int)->*mut dentry{let f=fh as *mut nilfs_fid;if fh_len<NILFS_FID_SIZE_NON_CONNECTABLE as c_int||(fh_type!=FILEID_NILFS_WITH_PARENT&&fh_type!=FILEID_NILFS_WITHOUT_PARENT){return core::ptr::null_mut();}nilfs_get_dentry(sb,(*f).cno,(*f).ino,(*f).gen)}
unsafe fn nilfs_fh_to_parent(sb:*mut super_block,fh:*mut fid,fh_len:c_int,fh_type:c_int)->*mut dentry{let f=fh as *mut nilfs_fid;if fh_len<NILFS_FID_SIZE_CONNECTABLE as c_int||fh_type!=FILEID_NILFS_WITH_PARENT{return core::ptr::null_mut();}nilfs_get_dentry(sb,(*f).cno,(*f).parent_ino,(*f).parent_gen)}
unsafe fn nilfs_encode_fh(inode:*mut inode,fh:*mut u32,lenp:*mut c_int,parent:*mut inode)->c_int{let f=fh as *mut nilfs_fid;let root=NILFS_I(inode).i_root;if !parent.is_null()&&*lenp<NILFS_FID_SIZE_CONNECTABLE as c_int{*lenp=NILFS_FID_SIZE_CONNECTABLE as c_int;return FILEID_INVALID;}if *lenp<NILFS_FID_SIZE_NON_CONNECTABLE as c_int{*lenp=NILFS_FID_SIZE_NON_CONNECTABLE as c_int;return FILEID_INVALID;}(*f).cno=(*root).cno;(*f).ino=(*inode).i_ino;(*f).gen=(*inode).i_generation;if !parent.is_null(){(*f).parent_ino=(*parent).i_ino;(*f).parent_gen=(*parent).i_generation;*lenp=NILFS_FID_SIZE_CONNECTABLE as c_int;FILEID_NILFS_WITH_PARENT}else{*lenp=NILFS_FID_SIZE_NON_CONNECTABLE as c_int;FILEID_NILFS_WITHOUT_PARENT}}

pub static nilfs_dir_inode_operations: inode_operations = inode_operations { create: Some(nilfs_create), lookup: Some(nilfs_lookup), link: Some(nilfs_link), unlink: Some(nilfs_unlink), symlink: Some(nilfs_symlink), mkdir: Some(nilfs_mkdir), rmdir: Some(nilfs_rmdir), mknod: Some(nilfs_mknod), rename: Some(nilfs_rename), setattr: Some(nilfs_setattr), permission: Some(nilfs_permission), fiemap: Some(nilfs_fiemap), fileattr_get: Some(nilfs_fileattr_get), fileattr_set: Some(nilfs_fileattr_set) };
pub static nilfs_special_inode_operations: inode_operations = inode_operations { setattr: Some(nilfs_setattr), permission: Some(nilfs_permission) };
pub static nilfs_symlink_inode_operations: inode_operations = inode_operations { get_link: Some(page_get_link), permission: Some(nilfs_permission) };
pub static nilfs_export_ops: export_operations = export_operations { encode_fh: Some(nilfs_encode_fh), fh_to_dentry: Some(nilfs_fh_to_dentry), fh_to_parent: Some(nilfs_fh_to_parent), get_parent: Some(nilfs_get_parent) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
