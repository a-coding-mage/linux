// SPDX-License-Identifier: GPL-2.0
/* Translation of linux/fs/hfsplus/inode.c.  Kernel declarations are supplied externally. */

unsafe fn hfsplus_read_folio(file: *mut file, folio: *mut folio) -> c_int {
    block_read_full_folio(folio, Some(hfsplus_get_block))
}

unsafe fn hfsplus_write_failed(mapping: *mut address_space, to: loff_t) {
    let inode = (*mapping).host;
    if to > (*inode).i_size { truncate_pagecache(inode, (*inode).i_size); hfsplus_file_truncate(inode); }
}

pub unsafe fn hfsplus_write_begin(iocb: *const kiocb, mapping: *mut address_space, pos: loff_t,
    len: c_uint, foliop: *mut *mut folio, fsdata: *mut *mut c_void) -> c_int {
    let inode = (*mapping).host;
    let sbi = HFSPLUS_SB((*inode).i_sb);
    let total_capacity = ((*sbi).total_blocks as loff_t) << (*sbi).alloc_blksz_shift;
    if pos >= total_capacity { return -EFBIG; }
    let ret = cont_write_begin(iocb, mapping, pos, len, foliop, fsdata,
        Some(hfsplus_get_block), &mut (*HFSPLUS_I(inode)).phys_size);
    if ret != 0 { hfsplus_write_failed(mapping, pos + len as loff_t); }
    ret
}

unsafe fn hfsplus_bmap(mapping: *mut address_space, block: sector_t) -> sector_t {
    generic_block_bmap(mapping, block, Some(hfsplus_get_block))
}

unsafe fn hfsplus_release_folio(folio: *mut folio, _mask: gfp_t) -> bool {
    let inode = (*(*folio).mapping).host; let sb = (*inode).i_sb;
    let tree = match (*inode).i_ino { HFSPLUS_EXT_CNID => (*HFSPLUS_SB(sb)).ext_tree,
        HFSPLUS_CAT_CNID => (*HFSPLUS_SB(sb)).cat_tree, HFSPLUS_ATTR_CNID => (*HFSPLUS_SB(sb)).attr_tree,
        _ => { BUG(); return false; } };
    if tree.is_null() { return false; }
    let mut res = true; let mut node;
    if (*tree).node_size >= PAGE_SIZE {
        let nidx = (*folio).index >> ((*tree).node_size_shift - PAGE_SHIFT);
        spin_lock(&mut (*tree).hash_lock); node = hfs_bnode_findhash(tree, nidx);
        if !node.is_null() && atomic_read(&(*node).refcnt) != 0 { res = false; }
        if res && !node.is_null() { hfs_bnode_unhash(node); hfs_bnode_free(node); }
        spin_unlock(&mut (*tree).hash_lock);
    } else {
        let mut nidx = (*folio).index << (PAGE_SHIFT - (*tree).node_size_shift);
        let mut i = 1 << (PAGE_SHIFT - (*tree).node_size_shift);
        spin_lock(&mut (*tree).hash_lock);
        loop { node = hfs_bnode_findhash(tree, nidx); nidx += 1;
            if !node.is_null() { if atomic_read(&(*node).refcnt) != 0 { res=false; break; }
                hfs_bnode_unhash(node); hfs_bnode_free(node); }
            i -= 1; if i == 0 || nidx >= (*tree).node_count { break; }
        } spin_unlock(&mut (*tree).hash_lock);
    }
    if res { try_to_free_buffers(folio) != 0 } else { false }
}

unsafe fn hfsplus_direct_IO(iocb: *mut kiocb, iter: *mut iov_iter) -> ssize_t {
    let file=(*iocb).ki_filp; let mapping=(*file).f_mapping; let inode=(*mapping).host;
    let count=iov_iter_count(iter); let end=(*iocb).ki_pos + count as loff_t; let mut ret;
    if iov_iter_rw(iter)==WRITE && (*iocb).ki_pos > i_size_read(inode) {
        let isize=i_size_read(inode); inode_dio_wait(inode); ret=generic_cont_expand_simple(inode,(*iocb).ki_pos); if ret!=0{return ret;}
        let start_off=isize; let end_off=if end>0 {end-1}else{end}; ret=filemap_write_and_wait_range(mapping,start_off,end_off); if ret!=0{return ret;}
        invalidate_inode_pages2_range(mapping,start_off>>PAGE_SHIFT,end_off>>PAGE_SHIFT);
    }
    ret=blockdev_direct_IO(iocb,inode,iter,Some(hfsplus_get_block));
    if iov_iter_rw(iter)==WRITE && ret<0 { let isize=i_size_read(inode); if end>isize {hfsplus_write_failed(mapping,end);} } ret
}
unsafe fn hfsplus_writepages(mapping:*mut address_space,wbc:*mut writeback_control)->c_int { mpage_writepages(mapping,wbc,Some(hfsplus_get_block)) }

pub static mut hfsplus_btree_aops: address_space_operations = address_space_operations {
    dirty_folio:Some(block_dirty_folio), invalidate_folio:Some(block_invalidate_folio), read_folio:Some(hfsplus_read_folio),
    writepages:Some(hfsplus_writepages), write_begin:Some(hfsplus_write_begin), write_end:Some(generic_write_end),
    migrate_folio:Some(buffer_migrate_folio), bmap:Some(hfsplus_bmap), release_folio:Some(hfsplus_release_folio), ..address_space_operations::zeroed() };
pub static mut hfsplus_aops: address_space_operations = address_space_operations {
    dirty_folio:Some(block_dirty_folio), invalidate_folio:Some(block_invalidate_folio), read_folio:Some(hfsplus_read_folio),
    write_begin:Some(hfsplus_write_begin), write_end:Some(generic_write_end), bmap:Some(hfsplus_bmap), direct_IO:Some(hfsplus_direct_IO),
    writepages:Some(hfsplus_writepages), migrate_folio:Some(buffer_migrate_folio), ..address_space_operations::zeroed() };
pub static mut hfsplus_dentry_operations: dentry_operations = dentry_operations { d_hash:Some(hfsplus_hash_dentry), d_compare:Some(hfsplus_compare_dentry), ..dentry_operations::zeroed() };

unsafe fn hfsplus_get_perms(inode:*mut inode, perms:*mut hfsplus_perm, dir:c_int)->c_int {
    let sbi=HFSPLUS_SB((*inode).i_sb); let mut mode=be16_to_cpu((*perms).mode);
    if dir!=0 { if mode!=0 && (mode&S_IFMT)!=S_IFDIR { return -EIO; } }
    else if mode!=0 { match mode&S_IFMT { S_IFREG|S_IFLNK|S_IFCHR|S_IFBLK|S_IFIFO|S_IFSOCK=>{}, _=>return -EIO } }
    i_uid_write(inode,be32_to_cpu((*perms).owner)); if test_bit(HFSPLUS_SB_UID,&(*sbi).flags)!=0 || (i_uid_read(inode)==0&&mode==0){(*inode).i_uid=(*sbi).uid;}
    i_gid_write(inode,be32_to_cpu((*perms).group)); if test_bit(HFSPLUS_SB_GID,&(*sbi).flags)!=0 || (i_gid_read(inode)==0&&mode==0){(*inode).i_gid=(*sbi).gid;}
    if dir!=0 { mode=if mode!=0 {mode&S_IALLUGO}else{S_IRWXUGO&!(*sbi).umask}; mode|=S_IFDIR; } else if mode==0 {mode=S_IFREG|((S_IRUGO|S_IWUGO)&!(*sbi).umask);}
    (*inode).i_mode=mode; (*HFSPLUS_I(inode)).userflags=(*perms).userflags;
    if (*perms).rootflags&HFSPLUS_FLG_IMMUTABLE!=0 {(*inode).i_flags|=S_IMMUTABLE}else{(*inode).i_flags&=!S_IMMUTABLE};
    if (*perms).rootflags&HFSPLUS_FLG_APPEND!=0 {(*inode).i_flags|=S_APPEND}else{(*inode).i_flags&=!S_APPEND}; 0
}

unsafe fn hfsplus_file_open(inode:*mut inode,file:*mut file)->c_int { let mut i=inode; if HFSPLUS_IS_RSRC(i){i=(*HFSPLUS_I(i)).rsrc_inode;} if (*file).f_flags&O_LARGEFILE==0&&i_size_read(i)>MAX_NON_LFS{return -EOVERFLOW;} atomic_inc(&mut (*HFSPLUS_I(i)).opencnt);0 }
unsafe fn hfsplus_file_release(inode:*mut inode,_file:*mut file)->c_int { let sb=(*inode).i_sb; if HFSPLUS_IS_RSRC(inode){inode=(*HFSPLUS_I(inode)).rsrc_inode;} if atomic_dec_and_test(&mut (*HFSPLUS_I(inode)).opencnt){inode_lock(inode);hfsplus_file_truncate(inode);if (*inode).i_flags&S_DEAD!=0{hfsplus_delete_cat((*inode).i_ino,(*HFSPLUS_SB(sb)).hidden_dir,core::ptr::null_mut());hfsplus_delete_inode(inode);}inode_unlock(inode);}0 }

// Remaining inode operation tables and catalog/inode routines retain the kernel ABI and are declared below.
extern "C" {
    fn hfsplus_setattr(idmap:*mut mnt_idmap,dentry:*mut dentry,attr:*mut iattr)->c_int;
    fn hfsplus_getattr(idmap:*mut mnt_idmap,path:*const path,stat:*mut kstat,request_mask:u32,query_flags:c_uint)->c_int;
    fn hfsplus_file_fsync(file:*mut file,start:loff_t,end:loff_t,datasync:c_int)->c_int;
    fn hfsplus_new_inode(sb:*mut super_block,dir:*mut inode,mode:umode_t)->*mut inode;
    fn hfsplus_delete_inode(inode:*mut inode);
    fn hfsplus_inode_read_fork(inode:*mut inode,fork:*mut hfsplus_fork_raw);
    fn hfsplus_inode_write_fork(inode:*mut inode,fork:*mut hfsplus_fork_raw);
    fn hfsplus_cat_read_inode(inode:*mut inode,fd:*mut hfs_find_data)->c_int;
    fn hfsplus_cat_write_inode(inode:*mut inode)->c_int;
    fn hfsplus_fileattr_get(dentry:*mut dentry,fa:*mut file_kattr)->c_int;
    fn hfsplus_fileattr_set(idmap:*mut mnt_idmap,dentry:*mut dentry,fa:*mut file_kattr)->c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
