// SPDX-License-Identifier: GPL-2.0-only
/*
 * Optimized MPEG FS - inode and super operations.
 * Copyright (C) 2006 Bob Copeland <me@bobcopeland.com>
 */
// Linux kernel headers and "omfs.h" are supplied by the surrounding build.

MODULE_AUTHOR!("Bob Copeland <me@bobcopeland.com>");
MODULE_DESCRIPTION!("OMFS (ReplayTV/Karma) Filesystem for Linux");
MODULE_LICENSE!("GPL");

pub unsafe fn omfs_bread(sb: *mut super_block, block: sector_t) -> *mut buffer_head {
    let sbi = OMFS_SB(sb);
    if block >= (*sbi).s_num_blocks { return core::ptr::null_mut(); }
    sb_bread(sb, clus_to_blk(sbi, block))
}

pub unsafe fn omfs_new_inode(dir: *mut inode, mode: umode_t) -> *mut inode {
    let sbi = OMFS_SB((*dir).i_sb);
    let inode = new_inode((*dir).i_sb);
    if inode.is_null() { return ERR_PTR(-ENOMEM); }
    let mut new_block: u64 = 0;
    let mut len: i32 = 0;
    let err = omfs_allocate_range((*dir).i_sb, (*sbi).s_mirrors, (*sbi).s_mirrors, &mut new_block, &mut len);
    if err != 0 { make_bad_inode(inode); iput(inode); return ERR_PTR(err); }
    (*inode).i_ino = new_block;
    inode_init_owner(&nop_mnt_idmap, inode, core::ptr::null_mut(), mode);
    (*(*inode).i_mapping).a_ops = &omfs_aops;
    simple_inode_init_ts(inode);
    match mode & S_IFMT {
        S_IFDIR => { (*inode).i_op = &omfs_dir_inops; (*inode).i_fop = &omfs_dir_operations; (*inode).i_size = (*sbi).s_sys_blocksize; inc_nlink(inode); }
        S_IFREG => { (*inode).i_op = &omfs_file_inops; (*inode).i_fop = &omfs_file_operations; (*inode).i_size = 0; }
        _ => {}
    }
    insert_inode_hash(inode); mark_inode_dirty(inode); inode
}

/* Update the header checksums for a dirty inode based on its contents. */
unsafe fn omfs_update_checksums(oi: *mut omfs_inode) {
    let ptr = oi as *mut u8;
    let count = be32_to_cpu((*oi).i_head.h_body_size);
    let crc = crc_itu_t(0, ptr.add(core::mem::size_of::<omfs_header>()), count as usize);
    (*oi).i_head.h_crc = cpu_to_be16(crc);
    let mut xor = *ptr;
    for i in 1..OMFS_XOR_COUNT { xor ^= *ptr.add(i as usize); }
    (*oi).i_head.h_check_xor = xor;
}

unsafe fn __omfs_write_inode(inode: *mut inode, wait: i32) -> i32 {
    let sbi = OMFS_SB((*inode).i_sb);
    let bh = omfs_bread((*inode).i_sb, (*inode).i_ino);
    if bh.is_null() { return -EIO; }
    let oi = (*bh).b_data as *mut omfs_inode;
    (*oi).i_head.h_self = cpu_to_be64((*inode).i_ino);
    if S_ISDIR((*inode).i_mode) { (*oi).i_type = OMFS_DIR; }
    else if S_ISREG((*inode).i_mode) { (*oi).i_type = OMFS_FILE; }
    else { printk!(KERN_WARNING, "omfs: unknown file type: %d\n", (*inode).i_mode); brelse(bh); return -EIO; }
    (*oi).i_head.h_body_size = cpu_to_be32((*sbi).s_sys_blocksize - core::mem::size_of::<omfs_header>() as u32);
    (*oi).i_head.h_version = 1; (*oi).i_head.h_type = OMFS_INODE_NORMAL; (*oi).i_head.h_magic = OMFS_IMAGIC;
    (*oi).i_size = cpu_to_be64((*inode).i_size);
    let ctime = inode_get_ctime_sec(inode) * 1000 + ((inode_get_ctime_nsec(inode) + 999) / 1000);
    (*oi).i_ctime = cpu_to_be64(ctime);
    omfs_update_checksums(oi); mark_buffer_dirty(bh);
    let mut failed = 0;
    if wait != 0 { sync_dirty_buffer(bh); if buffer_req(bh) && !buffer_uptodate(bh) { failed = 1; } }
    for i in 1..(*sbi).s_mirrors {
        let bh2 = omfs_bread((*inode).i_sb, (*inode).i_ino + i);
        if bh2.is_null() { brelse(bh); return -EIO; }
        core::ptr::copy_nonoverlapping((*bh).b_data, (*bh2).b_data, (*bh).b_size as usize);
        mark_buffer_dirty(bh2);
        if wait != 0 { sync_dirty_buffer(bh2); if buffer_req(bh2) && !buffer_uptodate(bh2) { failed = 1; } }
        brelse(bh2);
    }
    brelse(bh); if failed != 0 { -EIO } else { 0 }
}

unsafe fn omfs_write_inode(inode: *mut inode, wbc: *mut writeback_control) -> i32 { __omfs_write_inode(inode, (*wbc).sync_mode == WB_SYNC_ALL as i32) }
pub unsafe fn omfs_sync_inode(inode: *mut inode) -> i32 { __omfs_write_inode(inode, 1) }

/* called when an entry is deleted, need to clear the bits in the bitmaps. */
unsafe fn omfs_evict_inode(inode: *mut inode) {
    truncate_inode_pages_final(&mut (*inode).i_data); clear_inode(inode);
    if (*inode).i_nlink != 0 { return; }
    if S_ISREG((*inode).i_mode) { (*inode).i_size = 0; omfs_shrink_inode(inode); }
    omfs_clear_range((*inode).i_sb, (*inode).i_ino, 2);
}

pub unsafe fn omfs_iget(sb: *mut super_block, ino: ino_t) -> *mut inode {
    let sbi = OMFS_SB(sb); let inode = iget_locked(sb, ino); if inode.is_null() { return ERR_PTR(-ENOMEM); }
    if inode_state_read_once(inode) & I_NEW == 0 { return inode; }
    let bh = omfs_bread((*inode).i_sb, ino); if bh.is_null() { iget_failed(inode); return ERR_PTR(-EIO); }
    let oi = (*bh).b_data as *mut omfs_inode;
    if ino != be64_to_cpu((*oi).i_head.h_self) { brelse(bh); iget_failed(inode); return ERR_PTR(-EIO); }
    (*inode).i_uid = (*sbi).s_uid; (*inode).i_gid = (*sbi).s_gid;
    let mut ctime = be64_to_cpu((*oi).i_ctime); let nsecs = do_div(&mut ctime, 1000) * 1000;
    inode_set_atime(inode, ctime, nsecs); inode_set_mtime(inode, ctime, nsecs); inode_set_ctime(inode, ctime, nsecs);
    (*(*inode).i_mapping).a_ops = &omfs_aops;
    match (*oi).i_type {
        OMFS_DIR => { (*inode).i_mode = S_IFDIR | (S_IRWXUGO & !(*sbi).s_dmask); (*inode).i_op = &omfs_dir_inops; (*inode).i_fop = &omfs_dir_operations; (*inode).i_size = (*sbi).s_sys_blocksize; inc_nlink(inode); }
        OMFS_FILE => { (*inode).i_mode = S_IFREG | (S_IRWXUGO & !(*sbi).s_fmask); (*inode).i_fop = &omfs_file_operations; (*inode).i_size = be64_to_cpu((*oi).i_size); }
        _ => {}
    }
    brelse(bh); unlock_new_inode(inode); inode
}

unsafe fn omfs_put_super(sb: *mut super_block) { let sbi = OMFS_SB(sb); kfree((*sbi).s_imap); kfree(sbi); (*sb).s_fs_info = core::ptr::null_mut(); }

unsafe fn omfs_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> i32 {
    let s = (*dentry).d_sb; let sbi = OMFS_SB(s); let id = huge_encode_dev((*(*s).s_bdev).bd_dev);
    (*buf).f_type = OMFS_MAGIC; (*buf).f_bsize = (*sbi).s_blocksize; (*buf).f_blocks = (*sbi).s_num_blocks; (*buf).f_files = (*sbi).s_num_blocks; (*buf).f_namelen = OMFS_NAMELEN; (*buf).f_fsid = u64_to_fsid(id);
    let free = omfs_count_free(s); (*buf).f_bfree = free; (*buf).f_bavail = free; (*buf).f_ffree = free; 0
}

unsafe fn omfs_get_imap(sb: *mut super_block) -> i32 {
    let sbi = OMFS_SB(sb); let bitmap_size = DIV_ROUND_UP((*sbi).s_num_blocks, 8); let array_size = DIV_ROUND_UP(bitmap_size, (*sb).s_blocksize);
    if (*sbi).s_bitmap_ino == !0u64 { return 0; }
    (*sbi).s_imap_size = array_size; (*sbi).s_imap = kcalloc(array_size, core::mem::size_of::<*mut c_ulong>(), GFP_KERNEL);
    if (*sbi).s_imap.is_null() { return -ENOMEM; }
    let mut block = clus_to_blk(sbi, (*sbi).s_bitmap_ino); if block >= (*sbi).s_num_blocks { return -ENOMEM; }
    let mut count = bitmap_size; let mut ptr = (*sbi).s_imap;
    while count > 0 { let bh = sb_bread(sb, block); block += 1; if bh.is_null() { return -ENOMEM; }
        *ptr = kmemdup((*bh).b_data, (*sb).s_blocksize as usize, GFP_KERNEL); if (*ptr).is_null() { brelse(bh); return -ENOMEM; }
        if count < (*sb).s_blocksize { core::ptr::write_bytes((*ptr as *mut u8).add(count as usize), 0xff, ((*sb).s_blocksize - count) as usize); }
        brelse(bh); ptr = ptr.add(1); count = count.saturating_sub((*sb).s_blocksize);
    } 0
}

#[repr(C)] pub struct omfs_mount_options { pub s_uid: kuid_t, pub s_gid: kgid_t, pub s_dmask: i32, pub s_fmask: i32 }
pub const Opt_uid: i32 = 0; pub const Opt_gid: i32 = 1; pub const Opt_umask: i32 = 2; pub const Opt_dmask: i32 = 3; pub const Opt_fmask: i32 = 4;

unsafe fn omfs_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> i32 {
    let opts = (*fc).fs_private as *mut omfs_mount_options; if (*fc).purpose == FS_CONTEXT_FOR_RECONFIGURE { return 0; }
    let mut result = core::mem::zeroed::<fs_parse_result>(); let token = fs_parse(fc, &omfs_param_spec, param, &mut result); if token < 0 { return token; }
    match token { Opt_uid => (*opts).s_uid = result.uid, Opt_gid => (*opts).s_gid = result.gid, Opt_umask => { (*opts).s_fmask = result.uint_32 as i32; (*opts).s_dmask = result.uint_32 as i32; }, Opt_dmask => (*opts).s_dmask = result.uint_32 as i32, Opt_fmask => (*opts).s_fmask = result.uint_32 as i32, _ => return -EINVAL } 0
}
unsafe fn omfs_set_options(sbi: *mut omfs_sb_info, opts: *mut omfs_mount_options) { (*sbi).s_uid = (*opts).s_uid; (*sbi).s_gid = (*opts).s_gid; (*sbi).s_dmask = (*opts).s_dmask; (*sbi).s_fmask = (*opts).s_fmask; }

unsafe fn omfs_get_tree(fc: *mut fs_context) -> i32 { get_tree_bdev(fc, omfs_fill_super) }
unsafe fn omfs_init_fs_context(fc: *mut fs_context) -> i32 { let opts = kzalloc_obj::<omfs_mount_options>(); if opts.is_null() { return -ENOMEM; } (*opts).s_uid = current_uid(); (*opts).s_gid = current_gid(); (*opts).s_dmask = current_umask(); (*opts).s_fmask = (*opts).s_dmask; (*fc).fs_private = opts as *mut c_void; (*fc).ops = &omfs_context_ops; 0 }
unsafe fn omfs_free_fc(fc: *mut fs_context) { kfree((*fc).fs_private); }

static mut omfs_context_ops: fs_context_operations = fs_context_operations { parse_param: Some(omfs_parse_param), get_tree: Some(omfs_get_tree), free: Some(omfs_free_fc) };
static mut omfs_sops: super_operations = super_operations { write_inode: Some(omfs_write_inode), evict_inode: Some(omfs_evict_inode), put_super: Some(omfs_put_super), statfs: Some(omfs_statfs), show_options: None };
unsafe fn omfs_fill_super(_sb: *mut super_block, _fc: *mut fs_context) -> i32 { -EINVAL }
static mut omfs_fs_type: file_system_type = file_system_type { owner: THIS_MODULE, name: c"omfs", kill_sb: Some(kill_block_super), fs_flags: FS_REQUIRES_DEV, init_fs_context: Some(omfs_init_fs_context), parameters: core::ptr::null() };
module_init!(init_omfs_fs); module_exit!(exit_omfs_fs);
unsafe fn init_omfs_fs() -> i32 { register_filesystem(&mut omfs_fs_type) }
unsafe fn exit_omfs_fs() { unregister_filesystem(&mut omfs_fs_type); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
