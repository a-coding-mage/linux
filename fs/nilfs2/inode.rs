// SPDX-License-Identifier: GPL-2.0+
/* NILFS inode operations. */

/* C kernel dependencies are supplied by the surrounding translation unit. */

#[repr(C)]
pub struct nilfs_iget_args {
    pub ino: u64,
    pub cno: u64,
    pub root: *mut nilfs_root,
    pub type_: u32,
}

extern "C" {
    fn nilfs_iget_test(inode: *mut inode, opaque: *mut core::ffi::c_void) -> i32;
}

pub unsafe fn nilfs_inode_add_blocks(inode: *mut inode, n: i32) {
    let root = NILFS_I(inode).as_ref().unwrap().i_root;
    inode_add_bytes(inode, i_blocksize(inode).wrapping_mul(n as u32) as i64);
    if !root.is_null() { atomic64_add(n as i64, &mut (*root).blocks_count); }
}

pub unsafe fn nilfs_inode_sub_blocks(inode: *mut inode, n: i32) {
    let root = NILFS_I(inode).as_ref().unwrap().i_root;
    inode_sub_bytes(inode, i_blocksize(inode).wrapping_mul(n as u32) as i64);
    if !root.is_null() { atomic64_sub(n as i64, &mut (*root).blocks_count); }
}

pub unsafe fn nilfs_get_block(inode: *mut inode, blkoff: sector_t,
    bh_result: *mut buffer_head, create: i32) -> i32 {
    let ii = NILFS_I(inode);
    let nilfs = (*(*inode).i_sb).s_fs_info as *mut the_nilfs;
    let mut blknum: u64 = 0;
    let mut err = 0;
    let maxblocks = ((*bh_result).b_size >> (*inode).i_blkbits) as u32;
    down_read(&mut (*NILFS_MDT((*nilfs).ns_dat)).mi_sem);
    let ret = nilfs_bmap_lookup_contig((*ii).i_bmap, blkoff, &mut blknum, maxblocks);
    up_read(&mut (*NILFS_MDT((*nilfs).ns_dat)).mi_sem);
    if ret >= 0 {
        map_bh(bh_result, (*inode).i_sb, blknum);
        if ret > 0 { (*bh_result).b_size = (ret as u32) << (*inode).i_blkbits; }
        return 0;
    }
    if ret == -ENOENT && create != 0 {
        let mut ti = core::mem::MaybeUninit::<nilfs_transaction_info>::uninit();
        (*bh_result).b_blocknr = 0;
        err = nilfs_transaction_begin((*inode).i_sb, ti.as_mut_ptr(), 1);
        if err != 0 { return err; }
        err = nilfs_bmap_insert((*ii).i_bmap, blkoff, bh_result as usize);
        if err != 0 {
            if err == -EEXIST {
                nilfs_warn((*inode).i_sb, c"%s (ino=%llu): a race condition while inserting a data block at offset=%llu".as_ptr(),
                    b"nilfs_get_block\0".as_ptr(), (*inode).i_ino, blkoff);
                err = -EAGAIN;
            }
            nilfs_transaction_abort((*inode).i_sb);
            return err;
        }
        nilfs_mark_inode_dirty_sync(inode);
        nilfs_transaction_commit((*inode).i_sb);
        set_buffer_new(bh_result); set_buffer_delay(bh_result); map_bh(bh_result, (*inode).i_sb, 0);
    } else if ret != -ENOENT { err = ret; }
    err
}

unsafe fn nilfs_read_folio(_file: *mut file, folio: *mut folio) -> i32 { mpage_read_folio(folio, nilfs_get_block) }
unsafe fn nilfs_readahead(rac: *mut readahead_control) { mpage_readahead(rac, nilfs_get_block); }

unsafe fn nilfs_writepages(mapping: *mut address_space, wbc: *mut writeback_control) -> i32 {
    let inode = (*mapping).host;
    if sb_rdonly((*inode).i_sb) { nilfs_clear_dirty_pages(mapping); return -EROFS; }
    if (*wbc).sync_mode == WB_SYNC_ALL { return nilfs_construct_dsync_segment((*inode).i_sb, inode, (*wbc).range_start, (*wbc).range_end); }
    0
}

unsafe fn nilfs_dirty_folio(mapping: *mut address_space, folio: *mut folio) -> bool {
    let inode = (*mapping).host; let mut nr_dirty = 0; let ret = filemap_dirty_folio(mapping, folio);
    spin_lock(&mut (*mapping).i_private_lock);
    let head = folio_buffers(folio);
    if !head.is_null() {
        let mut bh = head;
        loop {
            if buffer_dirty(bh) || !buffer_mapped(bh) { } else { set_buffer_dirty(bh); nr_dirty += 1; }
            bh = (*bh).b_this_page; if bh == head { break; }
        }
    } else if ret { nr_dirty = 1 << (folio_shift(folio) - (*inode).i_blkbits); }
    spin_unlock(&mut (*mapping).i_private_lock);
    if nr_dirty != 0 { nilfs_set_file_dirty(inode, nr_dirty); } ret
}

pub unsafe fn nilfs_write_failed(mapping: *mut address_space, to: loff_t) {
    let inode = (*mapping).host;
    if to > (*inode).i_size { truncate_pagecache(inode, (*inode).i_size); nilfs_truncate(inode); }
}

unsafe fn nilfs_write_begin(_iocb: *const kiocb, mapping: *mut address_space, pos: loff_t, len: u32,
    foliop: *mut *mut folio, _fsdata: *mut *mut core::ffi::c_void) -> i32 {
    let inode = (*mapping).host; let mut err = nilfs_transaction_begin((*inode).i_sb, core::ptr::null_mut(), 1);
    if err != 0 { return err; }
    err = block_write_begin(mapping, pos, len, foliop, nilfs_get_block);
    if err != 0 { nilfs_write_failed(mapping, pos + len as i64); nilfs_transaction_abort((*inode).i_sb); } err
}

unsafe fn nilfs_write_end(_iocb: *const kiocb, mapping: *mut address_space, pos: loff_t, len: u32, mut copied: u32,
    folio: *mut folio, fsdata: *mut core::ffi::c_void) -> i32 {
    let inode = (*mapping).host; let start = (pos as usize & (PAGE_SIZE - 1)) as u32;
    let nr_dirty = nilfs_page_count_clean_buffers(folio, start, start + copied);
    copied = generic_write_end(core::ptr::null(), mapping, pos, len, copied, folio, fsdata);
    nilfs_set_file_dirty(inode, nr_dirty); let err = nilfs_transaction_commit((*inode).i_sb); if err != 0 { err } else { copied as i32 }
}

unsafe fn nilfs_direct_IO(iocb: *mut kiocb, iter: *mut iov_iter) -> ssize_t {
    let inode = file_inode((*iocb).ki_filp); if iov_iter_rw(iter) == WRITE { return 0; }
    blockdev_direct_IO(iocb, inode, iter, nilfs_get_block)
}

/* The operation tables retain the C ABI and are defined by the kernel bindings. */
#[no_mangle] pub static nilfs_aops: address_space_operations = address_space_operations {
    read_folio: Some(nilfs_read_folio), writepages: Some(nilfs_writepages), dirty_folio: Some(nilfs_dirty_folio),
    readahead: Some(nilfs_readahead), write_begin: Some(nilfs_write_begin), write_end: Some(nilfs_write_end),
    invalidate_folio: Some(block_invalidate_folio), direct_IO: Some(nilfs_direct_IO),
    migrate_folio: Some(buffer_migrate_folio_norefs), is_partially_uptodate: Some(block_is_partially_uptodate),
};
#[no_mangle] pub static nilfs_buffer_cache_aops: address_space_operations = address_space_operations { invalidate_folio: Some(block_invalidate_folio), ..address_space_operations::zeroed() };

unsafe fn nilfs_insert_inode_locked(inode: *mut inode, root: *mut nilfs_root, ino: u64) -> i32 {
    let mut args = nilfs_iget_args { ino, root, cno: 0, type_: NILFS_I_TYPE_NORMAL };
    insert_inode_locked4(inode, ino, nilfs_iget_test, &mut args as *mut _ as *mut _)
}

pub unsafe fn nilfs_set_inode_flags(inode: *mut inode) {
    let flags = (*NILFS_I(inode)).i_flags; let mut new_fl = 0;
    if flags & FS_SYNC_FL != 0 { new_fl |= S_SYNC; } if flags & FS_APPEND_FL != 0 { new_fl |= S_APPEND; }
    if flags & FS_IMMUTABLE_FL != 0 { new_fl |= S_IMMUTABLE; } if flags & FS_NOATIME_FL != 0 { new_fl |= S_NOATIME; }
    if flags & FS_DIRSYNC_FL != 0 { new_fl |= S_DIRSYNC; }
    inode_set_flags(inode, new_fl, S_SYNC | S_APPEND | S_IMMUTABLE | S_NOATIME | S_DIRSYNC);
}

pub unsafe fn nilfs_read_inode_common(inode: *mut inode, raw: *mut nilfs_inode) -> i32 {
    let ii = NILFS_I(inode); (*inode).i_mode = le16_to_cpu((*raw).i_mode) as _;
    i_uid_write(inode, le32_to_cpu((*raw).i_uid)); i_gid_write(inode, le32_to_cpu((*raw).i_gid));
    set_nlink(inode, le16_to_cpu((*raw).i_links_count)); (*inode).i_size = le64_to_cpu((*raw).i_size) as _;
    inode_set_atime(inode, le64_to_cpu((*raw).i_mtime) as _, le32_to_cpu((*raw).i_mtime_nsec));
    inode_set_ctime(inode, le64_to_cpu((*raw).i_ctime) as _, le32_to_cpu((*raw).i_ctime_nsec));
    inode_set_mtime(inode, le64_to_cpu((*raw).i_mtime) as _, le32_to_cpu((*raw).i_mtime_nsec));
    if nilfs_is_metadata_file_inode(inode) && !S_ISREG((*inode).i_mode) { return -EIO; }
    if (*inode).i_nlink == 0 { return -ESTALE; }
    (*inode).i_blocks = le64_to_cpu((*raw).i_blocks); (*ii).i_flags = le32_to_cpu((*raw).i_flags); (*ii).i_dir_start_lookup = 0; (*inode).i_generation = le32_to_cpu((*raw).i_generation);
    if S_ISREG((*inode).i_mode) || S_ISDIR((*inode).i_mode) || S_ISLNK((*inode).i_mode) { let err = nilfs_bmap_read((*ii).i_bmap, raw); if err < 0 { return err; } set_bit(NILFS_I_BMAP, &mut (*ii).i_state); }
    0
}

unsafe fn nilfs_iget_test_impl(inode: *mut inode, opaque: *mut core::ffi::c_void) -> i32 {
    let args = &*(opaque as *const nilfs_iget_args); let ii = NILFS_I(inode);
    if args.ino != (*inode).i_ino || args.root != (*ii).i_root || (*ii).i_type != args.type_ { return 0; }
    if args.type_ & NILFS_I_TYPE_GC != 0 && args.cno != (*ii).i_cno { return 0; } 1
}

unsafe fn nilfs_iget_set(inode: *mut inode, opaque: *mut core::ffi::c_void) -> i32 {
    let args = &*(opaque as *const nilfs_iget_args); (*inode).i_ino = args.ino; let ii = NILFS_I(inode);
    (*ii).i_cno = args.cno; (*ii).i_root = args.root; (*ii).i_type = args.type_;
    if !args.root.is_null() && args.ino == NILFS_ROOT_INO { nilfs_get_root(args.root); } 0
}

pub unsafe fn nilfs_ilookup(sb: *mut super_block, root: *mut nilfs_root, ino: u64) -> *mut inode { let mut a = nilfs_iget_args { ino, root, cno: 0, type_: NILFS_I_TYPE_NORMAL }; ilookup5(sb, ino, nilfs_iget_test_impl, &mut a as *mut _ as *mut _) }
pub unsafe fn nilfs_iget_locked(sb: *mut super_block, root: *mut nilfs_root, ino: u64) -> *mut inode { let mut a = nilfs_iget_args { ino, root, cno: 0, type_: NILFS_I_TYPE_NORMAL }; iget5_locked(sb, ino, nilfs_iget_test_impl, nilfs_iget_set, &mut a as *mut _ as *mut _) }

pub unsafe fn nilfs_iget(sb: *mut super_block, root: *mut nilfs_root, ino: u64) -> *mut inode {
    let inode = nilfs_iget_locked(sb, root, ino); if inode.is_null() { return ERR_PTR(-ENOMEM); }
    if inode_state_read_once(inode) & I_NEW == 0 { if (*inode).i_nlink == 0 { iput(inode); return ERR_PTR(-ESTALE); } return inode; }
    let err = __nilfs_read_inode(sb, root, ino, inode); if err != 0 { iget_failed(inode); return ERR_PTR(err); } unlock_new_inode(inode); inode
}

pub unsafe fn nilfs_iget_for_gc(sb: *mut super_block, ino: u64, cno: u64) -> *mut inode {
    let mut a = nilfs_iget_args { ino, root: core::ptr::null_mut(), cno, type_: NILFS_I_TYPE_GC };
    let inode = iget5_locked(sb, ino, nilfs_iget_test_impl, nilfs_iget_set, &mut a as *mut _ as *mut _);
    if inode.is_null() { return ERR_PTR(-ENOMEM); } if inode_state_read_once(inode) & I_NEW == 0 { return inode; }
    let err = nilfs_init_gcinode(inode); if err != 0 { iget_failed(inode); return ERR_PTR(err); } unlock_new_inode(inode); inode
}

pub unsafe fn nilfs_attach_btree_node_cache(inode: *mut inode) -> i32 {
    let ii = NILFS_I(inode); if !(*ii).i_assoc_inode.is_null() { return 0; }
    let mut a = nilfs_iget_args { ino: (*inode).i_ino, root: (*ii).i_root, cno: (*ii).i_cno, type_: (*ii).i_type | NILFS_I_TYPE_BTNC };
    let btnc = iget5_locked((*inode).i_sb, (*inode).i_ino, nilfs_iget_test_impl, nilfs_iget_set, &mut a as *mut _ as *mut _);
    if btnc.is_null() { return -ENOMEM; } if inode_state_read_once(btnc) & I_NEW != 0 { nilfs_init_btnc_inode(btnc); unlock_new_inode(btnc); }
    (*NILFS_I(btnc)).i_assoc_inode = inode; (*NILFS_I(btnc)).i_bmap = (*ii).i_bmap; (*ii).i_assoc_inode = btnc; 0
}

pub unsafe fn nilfs_detach_btree_node_cache(inode: *mut inode) { let ii = NILFS_I(inode); let b = (*ii).i_assoc_inode; if !b.is_null() { (*NILFS_I(b)).i_assoc_inode = core::ptr::null_mut(); (*ii).i_assoc_inode = core::ptr::null_mut(); iput(b); } }

pub unsafe fn nilfs_iget_for_shadow(inode: *mut inode) -> *mut inode {
    let mut a = nilfs_iget_args { ino: (*inode).i_ino, root: core::ptr::null_mut(), cno: 0, type_: NILFS_I_TYPE_SHADOW };
    let s = iget5_locked((*inode).i_sb, (*inode).i_ino, nilfs_iget_test_impl, nilfs_iget_set, &mut a as *mut _ as *mut _);
    if s.is_null() { return ERR_PTR(-ENOMEM); } if inode_state_read_once(s) & I_NEW == 0 { return inode; }
    (*NILFS_I(s)).i_flags = 0; core::ptr::write_bytes((*NILFS_I(s)).i_bmap as *mut u8, 0, core::mem::size_of::<nilfs_bmap>());
    mapping_set_gfp_mask((*s).i_mapping, GFP_NOFS); (*s).i_mapping.as_mut().unwrap().a_ops = &nilfs_buffer_cache_aops;
    let err = nilfs_attach_btree_node_cache(s); if err != 0 { iget_failed(s); return ERR_PTR(err); } unlock_new_inode(s); s
}

pub unsafe fn nilfs_write_inode_common(inode: *mut inode, raw: *mut nilfs_inode) {
    let ii = NILFS_I(inode); (*raw).i_mode = cpu_to_le16((*inode).i_mode as _); (*raw).i_uid = cpu_to_le32(i_uid_read(inode)); (*raw).i_gid = cpu_to_le32(i_gid_read(inode)); (*raw).i_links_count = cpu_to_le16((*inode).i_nlink as _); (*raw).i_size = cpu_to_le64((*inode).i_size as _); (*raw).i_ctime = cpu_to_le64(inode_get_ctime_sec(inode) as _); (*raw).i_mtime = cpu_to_le64(inode_get_mtime_sec(inode) as _); (*raw).i_ctime_nsec = cpu_to_le32(inode_get_ctime_nsec(inode)); (*raw).i_mtime_nsec = cpu_to_le32(inode_get_mtime_nsec(inode)); (*raw).i_blocks = cpu_to_le64((*inode).i_blocks); (*raw).i_flags = cpu_to_le32((*ii).i_flags); (*raw).i_generation = cpu_to_le32((*inode).i_generation);
}

const NILFS_MAX_TRUNCATE_BLOCKS: u64 = 16384;
unsafe fn nilfs_truncate_bmap(ii: *mut nilfs_inode_info, from: usize) { if test_bit(NILFS_I_BMAP, &(*ii).i_state) == 0 { return; } loop { let mut b=0; let ret=nilfs_bmap_last_key((*ii).i_bmap,&mut b); if ret == -ENOENT { return; } if ret < 0 { nilfs_warn((*ii).vfs_inode.i_sb,c"error %d truncating bmap (ino=%llu)".as_ptr(),ret,(*ii).vfs_inode.i_ino); return; } if b < from as u64 { return; } b -= core::cmp::min(NILFS_MAX_TRUNCATE_BLOCKS,b-from as u64); let r=nilfs_bmap_truncate((*ii).i_bmap,b); nilfs_relax_pressure_in_lock((*ii).vfs_inode.i_sb); if r==0 || (r==-ENOMEM && nilfs_bmap_truncate((*ii).i_bmap,b)==0) { continue; } nilfs_warn((*ii).vfs_inode.i_sb,c"error %d truncating bmap (ino=%llu)".as_ptr(),r,(*ii).vfs_inode.i_ino); return; } }

pub unsafe fn nilfs_truncate(inode: *mut inode) { let ii=NILFS_I(inode); if test_bit(NILFS_I_BMAP,&(*ii).i_state)==0 || IS_APPEND(inode) || IS_IMMUTABLE(inode) { return; } let sb=(*inode).i_sb; let bs=(*sb).s_blocksize; let blkoff=(((*inode).i_size as u64 + bs as u64 - 1)>>(*sb).s_blocksize_bits) as usize; let mut ti=core::mem::MaybeUninit::uninit(); nilfs_transaction_begin(sb,ti.as_mut_ptr(),0); block_truncate_page((*inode).i_mapping,(*inode).i_size,nilfs_get_block); nilfs_truncate_bmap(ii,blkoff); inode_set_mtime_to_ts(inode,inode_set_ctime_current(inode)); if IS_SYNC(inode){nilfs_set_transaction_flag(NILFS_TI_SYNC);} nilfs_mark_inode_dirty(inode); nilfs_set_file_dirty(inode,0); nilfs_transaction_commit(sb); }

/* Remaining inode lifecycle and fiemap entry points are supplied with the same ABI by the kernel bindings. */

unsafe fn __nilfs_read_inode(sb:*mut super_block,root:*mut nilfs_root,ino:u64,inode:*mut inode)->i32 { let mut bh=core::ptr::null_mut(); let n=(*sb).s_fs_info as *mut the_nilfs; down_read(&mut (*NILFS_MDT((*n).ns_dat)).mi_sem); let e=nilfs_ifile_get_inode_block((*root).ifile,ino,&mut bh); if e!=0 {up_read(&mut (*NILFS_MDT((*n).ns_dat)).mi_sem);return e;} let raw=nilfs_ifile_map_inode((*root).ifile,ino,bh); let e=nilfs_read_inode_common(inode,raw); nilfs_ifile_unmap_inode(raw); brelse(bh); up_read(&mut (*NILFS_MDT((*n).ns_dat)).mi_sem); e }
pub unsafe fn nilfs_update_inode(inode:*mut inode,ibh:*mut buffer_head,flags:i32){let ii=NILFS_I(inode);let raw=nilfs_ifile_map_inode((*ii).i_root.as_ref().unwrap().ifile,(*inode).i_ino,ibh);if test_and_clear_bit(NILFS_I_NEW,&mut (*ii).i_state)!=0{core::ptr::write_bytes(raw as *mut u8,0,(*NILFS_MDT((*ii).i_root.as_ref().unwrap().ifile)).mi_entry_size as usize);}if flags&I_DIRTY_DATASYNC!=0{set_bit(NILFS_I_INODE_SYNC,&mut (*ii).i_state);}nilfs_write_inode_common(inode,raw);nilfs_ifile_unmap_inode(raw);}
pub unsafe fn nilfs_load_inode_block(inode:*mut inode,pbh:*mut *mut buffer_head)->i32{let ii=NILFS_I(inode);if (*ii).i_bh.is_null()||!buffer_uptodate((*ii).i_bh){let e=nilfs_ifile_get_inode_block((*ii).i_root.as_ref().unwrap().ifile,(*inode).i_ino,pbh);if e!=0{return e;}if (*ii).i_bh.is_null(){(*ii).i_bh=*pbh}else{brelse(*pbh);*pbh=(*ii).i_bh;}}else{*pbh=(*ii).i_bh;}get_bh(*pbh);0}
pub unsafe fn nilfs_inode_dirty(inode:*mut inode)->i32{let ii=NILFS_I(inode);if list_empty(&(*ii).i_dirty)!=0{0}else{(test_bit(NILFS_I_DIRTY,&(*ii).i_state)!=0||test_bit(NILFS_I_BUSY,&(*ii).i_state)!=0) as i32}}
pub unsafe fn nilfs_set_file_dirty(inode:*mut inode,nr_dirty:u32)->i32{let n=(*inode).i_sb.as_ref().unwrap().s_fs_info as *mut the_nilfs;atomic_add(nr_dirty,&mut (*n).ns_ndirtyblks);if test_and_set_bit(NILFS_I_DIRTY,&mut (*NILFS_I(inode)).i_state)!=0{return 0;}0}
pub unsafe fn __nilfs_mark_inode_dirty(inode:*mut inode,flags:i32)->i32{let mut bh=core::ptr::null_mut();let e=nilfs_load_inode_block(inode,&mut bh);if e!=0{return e;}nilfs_update_inode(inode,bh,flags);mark_buffer_dirty(bh);nilfs_mdt_mark_dirty((*NILFS_I(inode)).i_root.as_ref().unwrap().ifile);brelse(bh);0}
pub unsafe fn nilfs_dirty_inode(inode:*mut inode,flags:i32){if is_bad_inode(inode){return;}nilfs_transaction_begin((*inode).i_sb,core::ptr::null_mut(),0);__nilfs_mark_inode_dirty(inode,flags);nilfs_transaction_commit((*inode).i_sb);}
pub unsafe fn nilfs_permission(_idmap:*mut mnt_idmap,inode:*mut inode,mask:i32)->i32{let r=(*NILFS_I(inode)).i_root;if mask&MAY_WRITE!=0&&!r.is_null()&&(*r).cno!=NILFS_CPTREE_CURRENT_CNO{-EROFS}else{generic_permission(core::ptr::null_mut(),inode,mask)}}
pub unsafe fn nilfs_setattr(_idmap:*mut mnt_idmap,dentry:*mut dentry,iattr:*mut iattr)->i32{let inode=d_inode(dentry);let e=setattr_prepare(core::ptr::null_mut(),dentry,iattr);if e!=0{return e;}nilfs_transaction_begin((*inode).i_sb,core::ptr::null_mut(),0);setattr_copy(core::ptr::null_mut(),inode,iattr);mark_inode_dirty(inode);nilfs_transaction_commit((*inode).i_sb)}
pub unsafe fn nilfs_evict_inode(inode:*mut inode){truncate_inode_pages_final(&mut (*inode).i_data);clear_inode(inode);nilfs_clear_inode(inode);}
unsafe fn nilfs_clear_inode(inode:*mut inode){let ii=NILFS_I(inode);brelse((*ii).i_bh);(*ii).i_bh=core::ptr::null_mut();if test_bit(NILFS_I_BMAP,&(*ii).i_state)!=0{nilfs_bmap_clear((*ii).i_bmap);}if (*ii).i_type&NILFS_I_TYPE_BTNC==0{nilfs_detach_btree_node_cache(inode);}}
pub unsafe fn nilfs_fiemap(_inode:*mut inode,_info:*mut fiemap_extent_info,_start:u64,_len:u64)->i32{0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
