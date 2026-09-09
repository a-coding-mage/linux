// SPDX-License-Identifier: GPL-2.0-only
/*
 * fs/bfs/inode.c
 * BFS superblock and inode operations.
 * Copyright (C) 1999-2018 Tigran Aivazian <aivazian.tigran@gmail.com>
 * From fs/minix, Copyright (C) 1991, 1992 Linus Torvalds.
 * Made endianness-clean by Andrew Stribblehill <ads@wompom.org>, 2005.
 */

// Linux kernel headers and "bfs.h" are supplied by external dependencies.
// #undef DEBUG

pub unsafe fn bfs_iget(sb: *mut super_block, ino: c_ulong) -> *mut inode {
    let mut di: *mut bfs_inode;
    let inode: *mut inode;
    let mut bh: *mut buffer_head;
    let mut block: c_int;
    let mut off: c_int;

    inode = iget_locked(sb, ino);
    if inode.is_null() { return ERR_PTR(-ENOMEM); }
    if !(inode_state_read_once(inode) & I_NEW != 0) { return inode; }

    if ino < BFS_ROOT_INO || ino > BFS_SB((*inode).i_sb).si_lasti {
        printf!("Bad inode number %s:%08lx\n", (*inode).i_sb.s_id, ino);
        goto_error(inode);
    }
    block = ((ino - BFS_ROOT_INO) / BFS_INODES_PER_BLOCK + 1) as c_int;
    bh = sb_bread((*inode).i_sb, block);
    if bh.is_null() {
        printf!("Unable to read inode %s:%08lx\n", (*inode).i_sb.s_id, ino);
        goto_error(inode);
    }
    off = ((ino - BFS_ROOT_INO) % BFS_INODES_PER_BLOCK) as c_int;
    di = (*bh).b_data as *mut bfs_inode;
    di = di.add(off as usize);

    /* BFS used only lower 9 bits of i_mode in SCO UnixWare; tolerate garbage
     * in the middle bits and reconstruct S_IFMT from i_vtype. */
    (*inode).i_mode = 0x00000FFF & le32_to_cpu((*di).i_mode);
    if le32_to_cpu((*di).i_vtype) == BFS_VDIR {
        (*inode).i_mode |= S_IFDIR;
        (*inode).i_op = &bfs_dir_inops;
        (*inode).i_fop = &bfs_dir_operations;
    } else if le32_to_cpu((*di).i_vtype) == BFS_VREG {
        (*inode).i_mode |= S_IFREG;
        (*inode).i_op = &bfs_file_inops;
        (*inode).i_fop = &bfs_file_operations;
        (*(*inode).i_mapping).a_ops = &bfs_aops;
    } else {
        brelse(bh);
        printf!("Unknown vtype=%u %s:%08lx\n", le32_to_cpu((*di).i_vtype), (*inode).i_sb.s_id, ino);
        goto_error(inode);
    }
    (*BFS_I(inode)).i_sblock = le32_to_cpu((*di).i_sblock);
    (*BFS_I(inode)).i_eblock = le32_to_cpu((*di).i_eblock);
    (*BFS_I(inode)).i_dsk_ino = le16_to_cpu((*di).i_ino);
    i_uid_write(inode, le32_to_cpu((*di).i_uid));
    i_gid_write(inode, le32_to_cpu((*di).i_gid));
    set_nlink(inode, le32_to_cpu((*di).i_nlink));
    (*inode).i_size = BFS_FILESIZE(di);
    (*inode).i_blocks = BFS_FILEBLOCKS(di);
    inode_set_atime(inode, le32_to_cpu((*di).i_atime), 0);
    inode_set_mtime(inode, le32_to_cpu((*di).i_mtime), 0);
    inode_set_ctime(inode, le32_to_cpu((*di).i_ctime), 0);
    brelse(bh);
    unlock_new_inode(inode);
    return inode;
}

unsafe fn goto_error(inode: *mut inode) -> ! {
    iget_failed(inode);
    panic!("ERR_PTR(-EIO)");
}

unsafe fn find_inode(sb: *mut super_block, mut ino: u16, p: *mut *mut buffer_head) -> *mut bfs_inode {
    if ino < BFS_ROOT_INO || ino > BFS_SB(sb).si_lasti {
        printf!("Bad inode number %s:%08x\n", (*sb).s_id, ino);
        return ERR_PTR(-EIO);
    }
    ino -= BFS_ROOT_INO as u16;
    *p = sb_bread(sb, 1 + ino / BFS_INODES_PER_BLOCK as u16);
    if (*p).is_null() {
        printf!("Unable to read inode %s:%08x\n", (*sb).s_id, ino);
        return ERR_PTR(-EIO);
    }
    ((*(*p)).b_data as *mut bfs_inode).add((ino % BFS_INODES_PER_BLOCK as u16) as usize)
}

unsafe fn bfs_write_inode(inode: *mut inode, _wbc: *mut writeback_control) -> c_int {
    let info = BFS_SB((*inode).i_sb); let ino = (*inode).i_ino as u16;
    let mut bh: *mut buffer_head = core::ptr::null_mut();
    dprintf!("ino=%08x\n", ino);
    let di = find_inode((*inode).i_sb, ino, &mut bh);
    if IS_ERR(di) { return PTR_ERR(di); }
    mutex_lock(&mut info.bfs_lock);
    (*di).i_vtype = cpu_to_le32(if ino as c_ulong == BFS_ROOT_INO { BFS_VDIR } else { BFS_VREG });
    (*di).i_ino = cpu_to_le16(ino); (*di).i_mode = cpu_to_le32((*inode).i_mode);
    (*di).i_uid = cpu_to_le32(i_uid_read(inode)); (*di).i_gid = cpu_to_le32(i_gid_read(inode));
    (*di).i_nlink = cpu_to_le32((*inode).i_nlink); (*di).i_atime = cpu_to_le32(inode_get_atime_sec(inode));
    (*di).i_mtime = cpu_to_le32(inode_get_mtime_sec(inode)); (*di).i_ctime = cpu_to_le32(inode_get_ctime_sec(inode));
    let i_sblock = BFS_I(inode).i_sblock; (*di).i_sblock = cpu_to_le32(i_sblock);
    (*di).i_eblock = cpu_to_le32(BFS_I(inode).i_eblock);
    (*di).i_eoffset = cpu_to_le32(i_sblock * BFS_BSIZE + (*inode).i_size - 1);
    mark_buffer_dirty(bh); brelse(bh); mutex_unlock(&mut info.bfs_lock); set_inode_metadata_writeback(inode); 0
}

unsafe fn bfs_sync_inode_metadata(inode: *mut inode, _wbc: *mut writeback_control) -> c_int {
    let mut bh = core::ptr::null_mut(); let di = find_inode((*inode).i_sb, (*inode).i_ino as u16, &mut bh);
    if IS_ERR(di) { return PTR_ERR(di); }
    sync_dirty_buffer(bh); let mut err = 0;
    if buffer_write_io_error(bh) { err = -EIO; } else { err = mmb_sync(&mut BFS_I(inode).i_metadata_bhs); }
    brelse(bh); err
}

unsafe fn bfs_evict_inode(inode: *mut inode) {
    let ino = (*inode).i_ino; let s = (*inode).i_sb; let info = BFS_SB(s); let bi = BFS_I(inode);
    dprintf!("ino=%08lx\n", ino); truncate_inode_pages_final(&mut (*inode).i_data);
    if (*inode).i_nlink != 0 { mmb_sync(&mut bi.i_metadata_bhs); }
    mmb_invalidate(&mut bi.i_metadata_bhs); clear_inode(inode); if (*inode).i_nlink != 0 { return; }
    let mut bh = core::ptr::null_mut(); let di = find_inode(s, (*inode).i_ino as u16, &mut bh); if IS_ERR(di) { return; }
    mutex_lock(&mut info.bfs_lock); core::ptr::write_bytes(di, 0, 1); mark_buffer_dirty(bh); brelse(bh);
    if bi.i_dsk_ino != 0 { if bi.i_sblock != 0 { info.si_freeb += bi.i_eblock + 1 - bi.i_sblock; } info.si_freei += 1; clear_bit(ino, info.si_imap); bfs_dump_imap(c"evict_inode", s); }
    if info.si_lf_eblk == bi.i_eblock { info.si_lf_eblk = bi.i_sblock - 1; } mutex_unlock(&mut info.bfs_lock);
}

unsafe fn bfs_put_super(s: *mut super_block) { let info = BFS_SB(s); if info.is_null() { return; } mutex_destroy(&mut info.bfs_lock); kfree(info); (*s).s_fs_info = core::ptr::null_mut(); }

unsafe fn bfs_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> c_int {
    let s = (*dentry).d_sb; let info = BFS_SB(s); let id = huge_encode_dev((*(*s).s_bdev).bd_dev);
    (*buf).f_type = BFS_MAGIC; (*buf).f_bsize = (*s).s_blocksize; (*buf).f_blocks = info.si_blocks;
    (*buf).f_bfree = info.si_freeb; (*buf).f_bavail = info.si_freeb; (*buf).f_files = info.si_lasti + 1 - BFS_ROOT_INO;
    (*buf).f_ffree = info.si_freei; (*buf).f_fsid = u64_to_fsid(id); (*buf).f_namelen = BFS_NAMELEN; 0
}

// The remaining filesystem registration and cache lifecycle declarations mirror
// the C source; their kernel types and helpers are supplied externally.
static mut bfs_inode_cachep: *mut kmem_cache = core::ptr::null_mut();
unsafe fn bfs_alloc_inode(sb: *mut super_block) -> *mut inode { let bi = alloc_inode_sb(sb, bfs_inode_cachep, GFP_KERNEL); if bi.is_null() { return core::ptr::null_mut(); } mmb_init(&mut (*bi).i_metadata_bhs, &mut (*bi).vfs_inode.i_data); &mut (*bi).vfs_inode }
unsafe fn bfs_free_inode(inode: *mut inode) { kmem_cache_free(bfs_inode_cachep, BFS_I(inode)); }
unsafe fn init_once(foo: *mut c_void) { inode_init_once(&mut (*(foo as *mut bfs_inode_info)).vfs_inode); }
unsafe fn init_inodecache() -> c_int { bfs_inode_cachep = kmem_cache_create(c"bfs_inode_cache", core::mem::size_of::<bfs_inode_info>(), 0, SLAB_RECLAIM_ACCOUNT | SLAB_ACCOUNT, init_once); if bfs_inode_cachep.is_null() { -ENOMEM } else { 0 } }
unsafe fn destroy_inodecache() { rcu_barrier(); kmem_cache_destroy(bfs_inode_cachep); }

pub unsafe fn bfs_dump_imap(_prefix: *const c_char, _s: *mut super_block) { /* DEBUG implementation is compile-time conditional in C. */ }

unsafe fn bfs_fill_super(s: *mut super_block, fc: *mut fs_context) -> c_int {
    let mut bh = core::ptr::null_mut(); let mut sbh = core::ptr::null_mut();
    let mut bfs_sb: *mut bfs_super_block; let mut inode: *mut inode; let mut info: *mut bfs_sb_info;
    let mut ret = -EINVAL; let silent = (*fc).sb_flags & SB_SILENT != 0;
    info = kzalloc_obj::<bfs_sb_info>(); if info.is_null() { return -ENOMEM; }
    mutex_init(&mut (*info).bfs_lock); (*s).s_fs_info = info as *mut c_void; (*s).s_time_min = 0; (*s).s_time_max = U32_MAX;
    if sb_set_blocksize(s, BFS_BSIZE) == 0 { return fill_out(s, info, ret); }
    sbh = sb_bread(s, 0); if sbh.is_null() { return fill_out(s, info, ret); }
    bfs_sb = (*sbh).b_data as *mut bfs_super_block;
    if le32_to_cpu((*bfs_sb).s_magic) != BFS_MAGIC { if !silent { printf!("No BFS filesystem on %s (magic=%08x)\n", (*s).s_id, le32_to_cpu((*bfs_sb).s_magic)); } brelse(sbh); return fill_out(s, info, ret); }
    if BFS_UNCLEAN(bfs_sb, s) && !silent { printf!("%s is unclean, continuing\n", (*s).s_id); }
    (*s).s_magic = BFS_MAGIC;
    if le32_to_cpu((*bfs_sb).s_start) > le32_to_cpu((*bfs_sb).s_end) || le32_to_cpu((*bfs_sb).s_start) < core::mem::size_of::<bfs_super_block>() as u32 + core::mem::size_of::<bfs_dirent>() as u32 { printf!("Superblock is corrupted on %s\n", (*s).s_id); brelse(sbh); return fill_out(s, info, ret); }
    (*info).si_lasti = (le32_to_cpu((*bfs_sb).s_start) - BFS_BSIZE) / core::mem::size_of::<bfs_inode>() as u32 + BFS_ROOT_INO - 1;
    if (*info).si_lasti > BFS_MAX_LASTI { printf!("Impossible last inode number %lu > %d on %s\n", (*info).si_lasti, BFS_MAX_LASTI, (*s).s_id); brelse(sbh); return fill_out(s, info, ret); }
    for i in 0..BFS_ROOT_INO { set_bit(i, (*info).si_imap); }
    (*s).s_op = &bfs_sops; inode = bfs_iget(s, BFS_ROOT_INO); if IS_ERR(inode) { ret = PTR_ERR(inode); brelse(sbh); return fill_out(s, info, ret); }
    (*s).s_root = d_make_root(inode); if (*s).s_root.is_null() { brelse(sbh); return fill_out(s, info, -ENOMEM); }
    (*info).si_blocks = (le32_to_cpu((*bfs_sb).s_end) + 1) >> BFS_BSIZE_BITS; (*info).si_freeb = (le32_to_cpu((*bfs_sb).s_end) + 1 - le32_to_cpu((*bfs_sb).s_start)) >> BFS_BSIZE_BITS;
    (*info).si_freei = 0; (*info).si_lf_eblk = 0; bh = sb_bread(s, (*info).si_blocks - 1); if bh.is_null() { dput((*s).s_root); (*s).s_root = core::ptr::null_mut(); brelse(sbh); return fill_out(s, info, -EIO); } brelse(bh);
    bh = core::ptr::null_mut(); for i in BFS_ROOT_INO..=(*info).si_lasti { let block = (i - BFS_ROOT_INO) / BFS_INODES_PER_BLOCK + 1; let off = (i - BFS_ROOT_INO) % BFS_INODES_PER_BLOCK; if off == 0 { brelse(bh); bh = sb_bread(s, block); } if bh.is_null() { continue; } let di = ((*bh).b_data as *mut bfs_inode).add(off as usize); let eoff = le32_to_cpu((*di).i_eoffset); let sb = le32_to_cpu((*di).i_sblock); let eb = le32_to_cpu((*di).i_eblock); let size = le32_to_cpu((*bfs_sb).s_end); if sb > (*info).si_blocks || eb > (*info).si_blocks || sb > eb || (eoff != u32::MAX && eoff > size) || sb * BFS_BSIZE > eoff { brelse(bh); brelse(sbh); return fill_out(s, info, -EIO); } if (*di).i_ino == 0 { (*info).si_freei += 1; continue; } set_bit(i, (*info).si_imap); (*info).si_freeb -= BFS_FILEBLOCKS(di); if eb > (*info).si_lf_eblk { (*info).si_lf_eblk = eb; } }
    brelse(bh); brelse(sbh); bfs_dump_imap(c"fill_super", s); 0
}

unsafe fn fill_out(s: *mut super_block, info: *mut bfs_sb_info, ret: c_int) -> c_int { mutex_destroy(&mut (*info).bfs_lock); kfree(info); (*s).s_fs_info = core::ptr::null_mut(); ret }
unsafe fn bfs_get_tree(fc: *mut fs_context) -> c_int { get_tree_bdev(fc, bfs_fill_super) }
unsafe fn bfs_init_fs_context(fc: *mut fs_context) -> c_int { (*fc).ops = &bfs_context_ops; 0 }
unsafe fn init_bfs_fs() -> c_int { let err = init_inodecache(); if err != 0 { return err; } let err = register_filesystem(&bfs_fs_type); if err != 0 { destroy_inodecache(); } err }
unsafe fn exit_bfs_fs() { unregister_filesystem(&bfs_fs_type); destroy_inodecache(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
