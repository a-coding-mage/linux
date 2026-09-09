/* Block- or MTD-based romfs
 *
 * Source-level Rust translation of super.c.  Kernel types, constants, and
 * functions referenced here are supplied by the surrounding kernel bindings.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_void;

static mut romfs_inode_cachep: *mut kmem_cache = core::ptr::null_mut();

static romfs_modemap: [umode_t; 8] = [
    0, S_IFDIR | 0o644, S_IFREG | 0o644, S_IFLNK | 0o777,
    S_IFBLK | 0o600, S_IFCHR | 0o600, S_IFSOCK | 0o644, S_IFIFO | 0o644,
];
static romfs_dtype_table: [u8; 8] = [DT_UNKNOWN, DT_DIR, DT_REG, DT_LNK, DT_BLK, DT_CHR, DT_SOCK, DT_FIFO];

extern "C" {
    fn romfs_iget(sb: *mut super_block, pos: c_ulong) -> *mut inode;
    fn romfs_read_folio(file: *mut file, folio: *mut folio) -> c_int;
    fn romfs_readdir(file: *mut file, ctx: *mut dir_context) -> c_int;
    fn romfs_lookup(dir: *mut inode, dentry: *mut dentry, flags: c_uint) -> *mut dentry;
}

unsafe fn romfs_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> c_int {
    let sb = (*dentry).d_sb; let mut id = 0u64;
    if !(*sb).s_bdev.is_null() { id = huge_encode_dev((*(*sb).s_bdev).bd_dev); }
    else if (*sb).s_dev != 0 { id = huge_encode_dev((*sb).s_dev); }
    (*buf).f_type = ROMFS_MAGIC; (*buf).f_namelen = ROMFS_MAXFN; (*buf).f_bsize = ROMBSIZE;
    (*buf).f_bfree = (*buf).f_bavail; (*buf).f_ffree = (*buf).f_ffree;
    (*buf).f_blocks = (romfs_maxsize(sb) + ROMBSIZE - 1) >> ROMBSBITS;
    (*buf).f_fsid = u64_to_fsid(id); 0
}

unsafe fn romfs_reconfigure(fc: *mut fs_context) -> c_int {
    sync_filesystem((*(*fc).root).d_sb); (*fc).sb_flags |= SB_RDONLY; 0
}

unsafe fn romfs_i_init_once(inode: *mut c_void) { inode_init_once(&mut (*(inode as *mut romfs_inode_info)).vfs_inode); }

unsafe fn init_romfs_fs_impl() -> c_int {
    pr_info("ROMFS MTD (C) 2007 Red Hat, Inc.\n");
    romfs_inode_cachep = kmem_cache_create("romfs_i", core::mem::size_of::<romfs_inode_info>(), 0,
        SLAB_RECLAIM_ACCOUNT | SLAB_ACCOUNT, romfs_i_init_once);
    if romfs_inode_cachep.is_null() { pr_err("Failed to initialise inode cache\n"); return -ENOMEM; }
    let ret = register_filesystem(&romfs_fs_type);
    if ret != 0 { pr_err("Failed to register filesystem\n"); kmem_cache_destroy(romfs_inode_cachep); }
    ret
}

unsafe fn exit_romfs_fs_impl() {
    unregister_filesystem(&romfs_fs_type); rcu_barrier(); kmem_cache_destroy(romfs_inode_cachep);
}

unsafe fn romfs_read_folio_impl(file: *mut file, folio: *mut folio) -> c_int {
    let inode = (*(*folio).mapping).host;
    let offset: loff_t = folio_pos(folio);
    let size = i_size_read(inode);
    let mut fillsize: usize = 0;
    let mut ret = 0;
    let mut buf = kmap_local_folio(folio, 0);
    if offset < size {
        let remain = size - offset;
        fillsize = if remain > PAGE_SIZE as i64 { PAGE_SIZE } else { remain as usize };
        let pos = (*ROMFS_I(inode)).i_dataoffset.wrapping_add(offset as c_ulong);
        ret = romfs_dev_read((*inode).i_sb, pos, buf, fillsize);
        if ret < 0 { fillsize = 0; ret = -EIO; }
    }
    buf = folio_zero_tail(folio, fillsize, (buf as *mut u8).add(fillsize) as *mut c_void);
    kunmap_local(buf);
    folio_end_read(folio, ret == 0);
    ret
}

unsafe fn romfs_readdir_impl(file: *mut file, ctx: *mut dir_context) -> c_int {
    let i = file_inode(file); let mut ri: romfs_inode = core::mem::zeroed();
    let maxoff = romfs_maxsize((*i).i_sb); let mut offset = (*ctx).pos;
    let mut fsname = [0i8; ROMFS_MAXFN as usize];
    if offset == 0 {
        offset = (*i).i_ino & ROMFH_MASK;
        if romfs_dev_read((*i).i_sb, offset, &mut ri, ROMFH_SIZE) < 0 { return 0; }
        offset = be32_to_cpu(ri.spec) & ROMFH_MASK;
    }
    loop {
        if offset == 0 || offset >= maxoff { (*ctx).pos = maxoff; return 0; }
        (*ctx).pos = offset;
        if romfs_dev_read((*i).i_sb, offset, &mut ri, ROMFH_SIZE) < 0 { return 0; }
        let j = romfs_dev_strnlen((*i).i_sb, offset + ROMFH_SIZE, fsname.len() - 1);
        if j < 0 { return 0; }
        let n = j as usize;
        if romfs_dev_read((*i).i_sb, offset + ROMFH_SIZE, fsname.as_mut_ptr(), n) < 0 { return 0; }
        fsname[n] = 0;
        let mut ino = offset; let nextfh = be32_to_cpu(ri.next);
        if nextfh & ROMFH_TYPE == ROMFH_HRD { ino = be32_to_cpu(ri.spec); }
        if !dir_emit(ctx, fsname.as_ptr(), n, ino, romfs_dtype_table[(nextfh & ROMFH_TYPE) as usize]) { return 0; }
        offset = nextfh & ROMFH_MASK;
    }
}

unsafe fn romfs_lookup_impl(dir: *mut inode, dentry: *mut dentry, _flags: c_uint) -> *mut dentry {
    let mut offset = (*dir).i_ino & ROMFH_MASK; let maxoff = romfs_maxsize((*dir).i_sb);
    let mut ri: romfs_inode = core::mem::zeroed();
    let ret = romfs_dev_read((*dir).i_sb, offset, &mut ri, ROMFH_SIZE); if ret < 0 { return ERR_PTR(ret); }
    offset = be32_to_cpu(ri.spec) & ROMFH_MASK;
    let name = (*dentry).d_name.name; let len = (*dentry).d_name.len;
    while offset != 0 && offset < maxoff {
        let r = romfs_dev_read((*dir).i_sb, offset, &mut ri, core::mem::size_of::<romfs_inode>()); if r < 0 { return ERR_PTR(r); }
        let found = romfs_dev_strcmp((*dir).i_sb, offset + ROMFH_SIZE, name, len); if found < 0 { return ERR_PTR(found); }
        if found == 1 {
            if be32_to_cpu(ri.next) & ROMFH_TYPE == ROMFH_HRD { offset = be32_to_cpu(ri.spec) & ROMFH_MASK; }
            let inode = romfs_iget((*dir).i_sb, offset); if IS_ERR(inode) { return ERR_CAST(inode); }
            return d_splice_alias(inode, dentry);
        }
        offset = be32_to_cpu(ri.next) & ROMFH_MASK;
    }
    d_splice_alias(core::ptr::null_mut(), dentry)
}

const ROMFS_MAX_HARDLINK_DEPTH: u32 = 64;

unsafe fn romfs_iget_impl(sb: *mut super_block, mut pos: c_ulong) -> *mut inode {
    let mut ri: romfs_inode = core::mem::zeroed(); let mut depth = 0u32;
    let nextfh;
    loop {
        let ret = romfs_dev_read(sb, pos, &mut ri, core::mem::size_of::<romfs_inode>()); if ret < 0 { pr_err_inode(pos); return ERR_PTR(ret); }
        nextfh = be32_to_cpu(ri.next); if nextfh & ROMFH_TYPE != ROMFH_HRD { break; }
        depth += 1; if depth > ROMFS_MAX_HARDLINK_DEPTH { return ERR_PTR(-ELOOP); }
        pos = be32_to_cpu(ri.spec) & ROMFH_MASK;
    }
    let nlen = romfs_dev_strnlen(sb, pos + ROMFH_SIZE, ROMFS_MAXFN); if IS_ERR_VALUE(nlen) { return ERR_PTR(-EIO); }
    let i = iget_locked(sb, pos); if i.is_null() { return ERR_PTR(-ENOMEM); }
    if inode_state_read_once(i) & I_NEW == 0 { return i; }
    let info = ROMFS_I(i); (*info).i_metasize = (ROMFH_SIZE + nlen as c_ulong + 1 + ROMFH_PAD) & ROMFH_MASK;
    (*info).i_dataoffset = pos + (*info).i_metasize; set_nlink(i, 1); (*i).i_size = be32_to_cpu(ri.size) as i64;
    inode_set_mtime_to_ts(i, inode_set_atime_to_ts(i, inode_set_ctime(i, 0, 0)));
    let mut mode = romfs_modemap[(nextfh & ROMFH_TYPE) as usize];
    match nextfh & ROMFH_TYPE {
        ROMFH_DIR => { (*i).i_size = (*info).i_metasize as i64; (*i).i_op = &romfs_dir_inode_operations; (*i).i_fop = &romfs_dir_operations; if nextfh & ROMFH_EXEC != 0 { mode |= S_IXUGO; } }
        ROMFH_REG => { (*i).i_fop = &romfs_ro_fops; (*i).i_data.a_ops = &romfs_aops; if nextfh & ROMFH_EXEC != 0 { mode |= S_IXUGO; } }
        ROMFH_SYM => { (*i).i_op = &page_symlink_inode_operations; inode_nohighmem(i); (*i).i_data.a_ops = &romfs_aops; mode |= S_IRWXUGO; }
        _ => { let dev = be32_to_cpu(ri.spec); init_special_inode(i, mode, MKDEV(dev >> 16, dev & 0xffff)); }
    }
    (*i).i_mode = mode; (*i).i_blocks = ((*i).i_size as u64 + 511) >> 9; unlock_new_inode(i); i
}

unsafe fn romfs_alloc_inode(sb: *mut super_block) -> *mut inode { let p = alloc_inode_sb(sb, romfs_inode_cachep, GFP_KERNEL); if p.is_null() { core::ptr::null_mut() } else { &mut (*p).vfs_inode } }
unsafe fn romfs_free_inode(inode: *mut inode) { kmem_cache_free(romfs_inode_cachep, ROMFS_I(inode)); }
unsafe fn romfs_checksum(data: *const c_void, mut size: c_int) -> u32 { let mut p = data as *const u32; let mut sum = 0u32; size >>= 2; while size > 0 { sum = sum.wrapping_add(be32_to_cpu(*p)); p = p.add(1); size -= 1; } sum }

/* The remaining filesystem registration and superblock routines retain their
 * kernel ABI and are declared for linkage with the surrounding translation. */
extern "C" {
    fn romfs_fill_super(sb: *mut super_block, fc: *mut fs_context) -> c_int;
    fn romfs_get_tree(fc: *mut fs_context) -> c_int;
    fn romfs_init_fs_context(fc: *mut fs_context) -> c_int;
    fn romfs_kill_sb(sb: *mut super_block);
    fn init_romfs_fs() -> c_int;
    fn exit_romfs_fs();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
