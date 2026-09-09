/* Rust translation of linux/fs/hfs/inode.c. */

// External kernel and HFS declarations are supplied by other translation units.

const HFS_VALID_MODE_BITS: umode_t = S_IFREG | S_IFDIR | S_IRWXUGO;

static unsafe fn hfs_read_folio(file: *mut file, folio: *mut folio) -> c_int {
    block_read_full_folio(folio, hfs_get_block)
}

unsafe fn hfs_write_failed(mapping: *mut address_space, to: loff_t) {
    let inode = (*mapping).host;
    if to > (*inode).i_size { truncate_pagecache(inode, (*inode).i_size); hfs_file_truncate(inode); }
}

pub unsafe fn hfs_write_begin(iocb: *const kiocb, mapping: *mut address_space, pos: loff_t, len: c_uint, foliop: *mut *mut folio, fsdata: *mut *mut c_void) -> c_int {
    let inode = (*mapping).host;
    let sbi = HFS_SB((*inode).i_sb);
    let total_capacity = (*sbi).fs_ablocks as loff_t * (*sbi).alloc_blksz as loff_t;
    if pos >= total_capacity { return -EFBIG; }
    let ret = cont_write_begin(iocb, mapping, pos, len, foliop, fsdata, hfs_get_block, &mut (*HFS_I(inode)).phys_size);
    if ret != 0 { hfs_write_failed(mapping, pos + len as loff_t); }
    ret
}

unsafe fn hfs_bmap(mapping: *mut address_space, block: sector_t) -> sector_t { generic_block_bmap(mapping, block, hfs_get_block) }

unsafe fn hfs_release_folio(folio: *mut folio, _mask: gfp_t) -> bool {
    let inode = (*(*folio).mapping).host; let sb = (*inode).i_sb;
    let tree: *mut hfs_btree = match (*inode).i_ino { HFS_EXT_CNID => (*HFS_SB(sb)).ext_tree, HFS_CAT_CNID => (*HFS_SB(sb)).cat_tree, _ => { BUG(); return false } };
    if tree.is_null() { return false; }
    let mut res = true; let mut node: *mut hfs_bnode;
    if (*tree).node_size >= PAGE_SIZE {
        let nidx = (*folio).index >> ((*tree).node_size_shift - PAGE_SHIFT);
        spin_lock(&mut (*tree).hash_lock); node = hfs_bnode_findhash(tree, nidx);
        if !node.is_null() && atomic_read(&(*node).refcnt) != 0 { res = false; }
        if res && !node.is_null() { hfs_bnode_unhash(node); hfs_bnode_free(node); } spin_unlock(&mut (*tree).hash_lock);
    } else {
        let mut nidx = (*folio).index << (PAGE_SHIFT - (*tree).node_size_shift); let mut i = 1 << (PAGE_SHIFT - (*tree).node_size_shift);
        spin_lock(&mut (*tree).hash_lock); loop { node = hfs_bnode_findhash(tree, nidx); nidx += 1;
            if !node.is_null() { if atomic_read(&(*node).refcnt) != 0 { res = false; break; } hfs_bnode_unhash(node); hfs_bnode_free(node); }
            i -= 1; if i == 0 || nidx >= (*tree).node_count { break; }
        } spin_unlock(&mut (*tree).hash_lock);
    }
    if res { try_to_free_buffers(folio) } else { false }
}

unsafe fn hfs_direct_IO(iocb: *mut kiocb, iter: *mut iov_iter) -> ssize_t {
    let inode = (*(*(*iocb).ki_filp).f_mapping).host; let count = iov_iter_count(iter); let ret = blockdev_direct_IO(iocb, inode, iter, hfs_get_block);
    if iov_iter_rw(iter) == WRITE && ret < 0 { let isize = i_size_read(inode); let end = (*iocb).ki_pos + count as loff_t; if end > isize { hfs_write_failed((*(*iocb).ki_filp).f_mapping, end); } } ret
}
unsafe fn hfs_writepages(mapping: *mut address_space, wbc: *mut writeback_control) -> c_int { mpage_writepages(mapping, wbc, hfs_get_block) }

#[no_mangle] pub static hfs_btree_aops: address_space_operations = address_space_operations { dirty_folio: block_dirty_folio, invalidate_folio: block_invalidate_folio, read_folio: hfs_read_folio, writepages: hfs_writepages, write_begin: hfs_write_begin, write_end: generic_write_end, migrate_folio: buffer_migrate_folio, bmap: hfs_bmap, release_folio: hfs_release_folio };
#[no_mangle] pub static hfs_aops: address_space_operations = address_space_operations { dirty_folio: block_dirty_folio, invalidate_folio: block_invalidate_folio, read_folio: hfs_read_folio, write_begin: hfs_write_begin, write_end: generic_write_end, bmap: hfs_bmap, direct_IO: hfs_direct_IO, writepages: hfs_writepages, migrate_folio: buffer_migrate_folio };

pub unsafe fn hfs_new_inode(dir: *mut inode, name: *const qstr, mode: umode_t) -> *mut inode {
    let sb = (*dir).i_sb; let inode = new_inode(sb); let mut err = -ENOMEM;
    if inode.is_null() { return ERR_PTR(err); } err = -ERANGE;
    mutex_init(&mut (*HFS_I(inode)).extents_lock); hfs_cat_build_key(sb, &mut (*HFS_I(inode)).cat_key as *mut _ as *mut btree_key, (*dir).i_ino, name);
    let next_id = atomic64_inc_return(&mut (*HFS_SB(sb)).next_id); if next_id > U32_MAX as i64 { atomic64_dec(&mut (*HFS_SB(sb)).next_id); pr_err!("cannot create new inode: next CNID exceeds limit\n"); iput(inode); return ERR_PTR(err); }
    (*inode).i_ino = next_id as u32 - 1; (*inode).i_mode = mode; (*inode).i_uid = current_fsuid(); (*inode).i_gid = current_fsgid(); set_nlink(inode, 1); simple_inode_init_ts(inode);
    (*HFS_I(inode)).flags = 0; (*HFS_I(inode)).rsrc_inode = core::ptr::null_mut(); (*HFS_I(inode)).fs_blocks = 0; (*HFS_I(inode)).tz_secondswest = sys_tz.tz_minuteswest * 60;
    if S_ISDIR(mode) { (*inode).i_size = 2; let n = atomic64_inc_return(&mut (*HFS_SB(sb)).folder_count); if n > U32_MAX as i64 { atomic64_dec(&mut (*HFS_SB(sb)).folder_count); pr_err!("cannot create new inode: folder count exceeds limit\n"); iput(inode); return ERR_PTR(err); } if (*dir).i_ino == HFS_ROOT_CNID { (*HFS_SB(sb)).root_dirs += 1; } (*inode).i_op = &hfs_dir_inode_operations; (*inode).i_fop = &hfs_dir_operations; (*inode).i_mode |= S_IRWXUGO; (*inode).i_mode &= !(*HFS_SB((*inode).i_sb)).s_dir_umask;
    } else if S_ISREG(mode) { (*HFS_I(inode)).clump_blocks = (*HFS_SB(sb)).clumpablks; let n = atomic64_inc_return(&mut (*HFS_SB(sb)).file_count); if n > U32_MAX as i64 { atomic64_dec(&mut (*HFS_SB(sb)).file_count); pr_err!("cannot create new inode: file count exceeds limit\n"); iput(inode); return ERR_PTR(err); } if (*dir).i_ino == HFS_ROOT_CNID { (*HFS_SB(sb)).root_files += 1; } (*inode).i_op = &hfs_file_inode_operations; (*inode).i_fop = &hfs_file_operations; (*(*inode).i_mapping).a_ops = &hfs_aops; (*inode).i_mode |= S_IRUGO | S_IXUGO; if mode & S_IWUSR != 0 { (*inode).i_mode |= S_IWUGO; } (*inode).i_mode &= !(*HFS_SB((*inode).i_sb)).s_file_umask; (*HFS_I(inode)).phys_size = 0; (*HFS_I(inode)).alloc_blocks = 0; (*HFS_I(inode)).first_blocks = 0; (*HFS_I(inode)).cached_start = 0; (*HFS_I(inode)).cached_blocks = 0; }
    insert_inode_hash(inode); mark_inode_dirty(inode); set_bit(HFS_FLG_MDB_DIRTY, &mut (*HFS_SB(sb)).flags); hfs_mark_mdb_dirty(sb); inode
}

pub unsafe fn hfs_delete_inode(inode: *mut inode) { let sb = (*inode).i_sb; hfs_dbg!("ino %llu\n", (*inode).i_ino); if S_ISDIR((*inode).i_mode) { atomic64_dec(&mut (*HFS_SB(sb)).folder_count); if (*HFS_I(inode)).cat_key.ParID == cpu_to_be32(HFS_ROOT_CNID) { (*HFS_SB(sb)).root_dirs -= 1; } set_bit(HFS_FLG_MDB_DIRTY, &mut (*HFS_SB(sb)).flags); hfs_mark_mdb_dirty(sb); return; } atomic64_dec(&mut (*HFS_SB(sb)).file_count); if (*HFS_I(inode)).cat_key.ParID == cpu_to_be32(HFS_ROOT_CNID) { (*HFS_SB(sb)).root_files -= 1; } if S_ISREG((*inode).i_mode) && (*inode).i_nlink == 0 { (*inode).i_size = 0; hfs_file_truncate(inode); } set_bit(HFS_FLG_MDB_DIRTY, &mut (*HFS_SB(sb)).flags); hfs_mark_mdb_dirty(sb); }

pub unsafe fn hfs_inode_read_fork(inode: *mut inode, ext: *mut hfs_extent, log: __be32, phys: __be32, clump: u32) { let sb = (*inode).i_sb; let log_size = be32_to_cpu(log); memcpy((*HFS_I(inode)).first_extents.as_mut_ptr() as *mut c_void, ext as *const c_void, core::mem::size_of::<hfs_extent_rec>()); let mut count: u16 = 0; for i in 0..3 { count += be16_to_cpu((*ext.add(i)).count); } (*HFS_I(inode)).first_blocks = count; (*HFS_I(inode)).cached_start = 0; (*HFS_I(inode)).cached_blocks = 0; (*inode).i_size = log_size as i64; (*HFS_I(inode)).phys_size = log_size as i64; (*HFS_I(inode)).fs_blocks = (log_size + (*sb).s_blocksize - 1) >> (*sb).s_blocksize_bits; inode_set_bytes(inode, (*HFS_I(inode)).fs_blocks << (*sb).s_blocksize_bits); (*HFS_I(inode)).alloc_blocks = be32_to_cpu(phys) / (*HFS_SB(sb)).alloc_blksz; (*HFS_I(inode)).clump_blocks = clump / (*HFS_SB(sb)).alloc_blksz; if (*HFS_I(inode)).clump_blocks == 0 { (*HFS_I(inode)).clump_blocks = (*HFS_SB(sb)).clumpablks; } }

// The remaining inode operations retain the C implementation's external kernel types and helpers.
// Their declarations are intentionally kept as direct low-level Rust signatures.
pub struct hfs_iget_data { pub key: *mut hfs_cat_key, pub rec: *mut hfs_cat_rec }

unsafe fn hfs_test_inode(inode: *mut inode, data: *mut c_void) -> c_int { let d = &*(data as *const hfs_iget_data); match (*d.rec).type_ { HFS_CDR_DIR => ((*inode).i_ino == be32_to_cpu((*d.rec).dir.DirID)) as c_int, HFS_CDR_FIL => ((*inode).i_ino == be32_to_cpu((*d.rec).file.FlNum)) as c_int, _ => { BUG(); 1 } } }

unsafe fn hfs_read_inode(inode: *mut inode, data: *mut c_void) -> c_int { let d=&*(data as *const hfs_iget_data); let h=HFS_SB((*inode).i_sb); (*HFS_I(inode)).flags=0; (*HFS_I(inode)).rsrc_inode=core::ptr::null_mut(); mutex_init(&mut (*HFS_I(inode)).extents_lock); (*inode).i_uid=(*h).s_uid; (*inode).i_gid=(*h).s_gid; set_nlink(inode,1); if !d.key.is_null(){(*HFS_I(inode)).cat_key=*d.key;}else{(*HFS_I(inode)).flags|=HFS_FLG_RSRC;} (*HFS_I(inode)).tz_secondswest=sys_tz.tz_minuteswest*60; let r=&*d.rec; match r.type_ { HFS_CDR_FIL=>{if !hfs_is_valid_cnid(be32_to_cpu(r.file.FlNum),r.type_){return -EIO;} let x=if HFS_IS_RSRC(inode){(r.file.RExtRec,r.file.RLgLen,r.file.RPyLen)}else{(r.file.ExtRec,r.file.LgLen,r.file.PyLen)}; hfs_inode_read_fork(inode,x.0,x.1,x.2,be16_to_cpu(r.file.ClpSize)); (*inode).i_ino=be32_to_cpu(r.file.FlNum); (*inode).i_mode=S_IRUGO|S_IXUGO; if r.file.Flags&HFS_FIL_LOCK==0{(*inode).i_mode|=S_IWUGO;} (*inode).i_mode=((*inode).i_mode&!(*h).s_file_umask)|S_IFREG; let t=hfs_m_to_utime(r.file.MdDat); inode_set_ctime_to_ts(inode,t);inode_set_atime_to_ts(inode,t);inode_set_mtime_to_ts(inode,t);(*inode).i_op=&hfs_file_inode_operations;(*inode).i_fop=&hfs_file_operations;(*(*inode).i_mapping).a_ops=&hfs_aops;}, HFS_CDR_DIR=>{if !hfs_is_valid_cnid(be32_to_cpu(r.dir.DirID),r.type_){return -EIO;} (*inode).i_ino=be32_to_cpu(r.dir.DirID);(*inode).i_size=be16_to_cpu(r.dir.Val) as i64+2;(*HFS_I(inode)).fs_blocks=0;(*inode).i_mode=S_IFDIR|(S_IRWXUGO&!*h.s_dir_umask);let t=hfs_m_to_utime(r.dir.MdDat);inode_set_ctime_to_ts(inode,t);inode_set_atime_to_ts(inode,t);inode_set_mtime_to_ts(inode,t);(*inode).i_op=&hfs_dir_inode_operations;(*inode).i_fop=&hfs_dir_operations;}, _=>make_bad_inode(inode)} 0 }

pub unsafe fn hfs_iget(sb:*mut super_block,key:*mut hfs_cat_key,rec:*mut hfs_cat_rec)->*mut inode{let d=hfs_iget_data{key,rec};let cnid=match(*rec).type_{HFS_CDR_DIR=>be32_to_cpu((*rec).dir.DirID),HFS_CDR_FIL=>be32_to_cpu((*rec).file.FlNum),_=>return core::ptr::null_mut()};let i=iget5_locked(sb,cnid,hfs_test_inode,hfs_read_inode,&d as *const _ as *mut c_void);if !i.is_null()&&(inode_state_read_once(i)&I_NEW)!=0{unlock_new_inode(i);}i}

pub unsafe fn hfs_inode_write_fork(inode:*mut inode,ext:*mut hfs_extent,log:*mut __be32,phys:*mut __be32){memcpy(ext as *mut c_void,(*HFS_I(inode)).first_extents.as_ptr() as *const c_void,core::mem::size_of::<hfs_extent_rec>());if !log.is_null(){*log=cpu_to_be32((*inode).i_size as u32);}if !phys.is_null(){*phys=cpu_to_be32((*HFS_I(inode)).alloc_blocks*(*HFS_SB((*inode).i_sb)).alloc_blksz);}}

pub unsafe fn hfs_write_inode(inode:*mut inode,_wbc:*mut writeback_control)->c_int{let r=hfs_ext_write_extent(inode);if r!=0{return r;}if (*inode).i_ino==HFS_EXT_CNID{hfs_btree_write((*HFS_SB((*inode).i_sb)).ext_tree);return 0;}if (*inode).i_ino==HFS_CAT_CNID{hfs_btree_write((*HFS_SB((*inode).i_sb)).cat_tree);return 0;}0}

unsafe fn hfs_file_open(inode:*mut inode,_file:*mut file)->c_int{if HFS_IS_RSRC(inode){inode=(*HFS_I(inode)).rsrc_inode;}atomic_inc(&mut (*HFS_I(inode)).opencnt);0}
unsafe fn hfs_file_release(inode:*mut inode,_file:*mut file)->c_int{if HFS_IS_RSRC(inode){inode=(*HFS_I(inode)).rsrc_inode;}if atomic_dec_and_test(&mut (*HFS_I(inode)).opencnt){inode_lock(inode);hfs_file_truncate(inode);inode_unlock(inode);}0}

pub unsafe fn hfs_evict_inode(inode:*mut inode){truncate_inode_pages_final(&mut (*inode).i_data);clear_inode(inode);if HFS_IS_RSRC(inode)&&!(*HFS_I(inode)).rsrc_inode.is_null(){let i=(*HFS_I(inode)).rsrc_inode;(*HFS_I(i)).rsrc_inode=core::ptr::null_mut();iput(i);}}

static hfs_file_operations:file_operations=file_operations{llseek:generic_file_llseek,read_iter:generic_file_read_iter,write_iter:generic_file_write_iter,mmap_prepare:generic_file_mmap_prepare,splice_read:filemap_splice_read,splice_write:iter_file_splice_write,fsync:hfs_file_fsync,open:hfs_file_open,release:hfs_file_release};
static hfs_file_inode_operations:inode_operations=inode_operations{lookup:hfs_file_lookup,setattr:hfs_inode_setattr,listxattr:generic_listxattr,fileattr_get:hfs_fileattr_get};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
