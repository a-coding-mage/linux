// SPDX-License-Identifier: GPL-2.0
/*
  FUSE: Filesystem in Userspace
  Copyright (C) 2001-2018  Miklos Szeredi <miklos@szeredi.hu>
*/

// Kernel/FUSE declarations and macros are supplied by the surrounding translation unit.

unsafe fn fuse_use_readdirplus(dir: *mut inode, ctx: *mut dir_context) -> bool {
    let fc = get_fuse_conn(dir);
    let fi = get_fuse_inode(dir);
    if !(*fc).do_readdirplus { return false; }
    if !(*fc).readdirplus_auto { return true; }
    if test_and_clear_bit(FUSE_I_ADVISE_RDPLUS, &mut (*fi).state) { return true; }
    if (*ctx).pos == 0 { return true; }
    false
}

unsafe fn fuse_add_dirent_to_cache(file: *mut file, dirent: *mut fuse_dirent, pos: loff_t) {
    let fi = get_fuse_inode(file_inode(file));
    let reclen = FUSE_DIRENT_SIZE(dirent);
    if reclen > PAGE_SIZE { return; }
    spin_lock(&mut (*fi).rdc.lock);
    if (*fi).rdc.cached || pos != (*fi).rdc.pos {
        spin_unlock(&mut (*fi).rdc.lock); return;
    }
    let version = (*fi).rdc.version;
    let size = (*fi).rdc.size;
    let mut offset = offset_in_page(size);
    let mut index = size >> PAGE_SHIFT;
    if offset + reclen > PAGE_SIZE { index += 1; offset = 0; }
    spin_unlock(&mut (*fi).rdc.lock);
    let page = if offset != 0 { find_lock_page((*file).f_mapping, index) }
               else { find_or_create_page((*file).f_mapping, index, mapping_gfp_mask((*file).f_mapping)) };
    if page.is_null() { return; }
    spin_lock(&mut (*fi).rdc.lock);
    if (*fi).rdc.version != version || (*fi).rdc.size != size || WARN_ON((*fi).rdc.pos != pos) {
        spin_unlock(&mut (*fi).rdc.lock); unlock_page(page); put_page(page); return;
    }
    let addr = kmap_local_page(page);
    if offset == 0 { clear_page(addr); SetPageUptodate(page); }
    memcpy(addr.add(offset), dirent as *const _, reclen);
    kunmap_local(addr);
    (*fi).rdc.size = (index << PAGE_SHIFT) + offset + reclen;
    (*fi).rdc.pos = (*dirent).off;
    spin_unlock(&mut (*fi).rdc.lock);
    unlock_page(page); put_page(page);
}

unsafe fn fuse_readdir_cache_end(file: *mut file, pos: loff_t) {
    let fi = get_fuse_inode(file_inode(file));
    spin_lock(&mut (*fi).rdc.lock);
    if (*fi).rdc.pos != pos { spin_unlock(&mut (*fi).rdc.lock); return; }
    (*fi).rdc.cached = true;
    let end = ALIGN((*fi).rdc.size, PAGE_SIZE);
    spin_unlock(&mut (*fi).rdc.lock);
    truncate_inode_pages((*file).f_mapping, end);
}

unsafe fn fuse_emit(file: *mut file, ctx: *mut dir_context, dirent: *mut fuse_dirent) -> bool {
    let ff = (*file).private_data as *mut fuse_file;
    if (*ff).open_flags & FOPEN_CACHE_DIR != 0 { fuse_add_dirent_to_cache(file, dirent, (*ctx).pos); }
    dir_emit(ctx, (*dirent).name, (*dirent).namelen, (*dirent).ino,
             (*dirent).type_ | FILLDIR_FLAG_NOINTR)
}

unsafe fn parse_dirfile(mut buf: *mut u8, mut nbytes: usize, file: *mut file, ctx: *mut dir_context) -> i32 {
    while nbytes >= FUSE_NAME_OFFSET {
        let dirent = buf as *mut fuse_dirent;
        let reclen = FUSE_DIRENT_SIZE(dirent);
        if (*dirent).namelen == 0 || (*dirent).namelen > FUSE_NAME_MAX { return -EIO; }
        if reclen > nbytes { break; }
        if !memchr((*dirent).name, b'/' as i32, (*dirent).namelen).is_null() { return -EIO; }
        if !fuse_emit(file, ctx, dirent) { break; }
        buf = buf.add(reclen); nbytes -= reclen; (*ctx).pos = (*dirent).off;
    }
    0
}

unsafe fn fuse_direntplus_link(file: *mut file, direntplus: *mut fuse_direntplus, attr_version: u64, evict_ctr: u64) -> i32 {
    let o = &mut (*direntplus).entry_out;
    let dirent = &mut (*direntplus).dirent;
    let parent = (*file).f_path.dentry;
    let mut name = QSTR_INIT(dirent.name, dirent.namelen);
    let mut dentry; let alias; let dir = d_inode(parent); let fc; let mut inode; let epoch;
    if o.nodeid == 0 { return 0; }
    if name.name[0] == b'.' { if name.len == 1 || (name.name[1] == b'.' && name.len == 2) { return 0; } }
    if invalid_nodeid(o.nodeid) || fuse_invalid_attr(&o.attr) { return -EIO; }
    fc = get_fuse_conn(dir); epoch = atomic_read(&(*fc).epoch);
    name.hash = full_name_hash(parent, name.name, name.len);
    dentry = d_lookup(parent, &name);
    if dentry.is_null() { dentry = d_alloc_parallel(parent, &name); if IS_ERR(dentry) { return PTR_ERR(dentry); } }
    if !d_in_lookup(dentry) {
        let mut existing = d_inode(dentry); if !existing.is_null() && get_node_id(existing) != o.nodeid { existing = core::ptr::null_mut(); }
        inode = existing;
        if inode.is_null() || fuse_stale_inode(inode, o.generation, &o.attr) { if !inode.is_null() { fuse_make_bad(inode); } d_invalidate(dentry); dput(dentry); return -EIO; }
        if fuse_is_bad(inode) { dput(dentry); return -EIO; }
        let fi = get_fuse_inode(inode); spin_lock(&mut (*fi).lock); (*fi).nlookup += 1; spin_unlock(&mut (*fi).lock);
        forget_all_cached_acls(inode); fuse_change_attributes(inode, &o.attr, core::ptr::null_mut(), ATTR_TIMEOUT(o), attr_version);
    } else {
        inode = fuse_iget((*dir).i_sb, o.nodeid, o.generation, &o.attr, ATTR_TIMEOUT(o), attr_version, evict_ctr);
        if inode.is_null() { inode = ERR_PTR(-ENOMEM); }
        alias = d_splice_alias(inode, dentry); d_lookup_done(dentry);
        if !alias.is_null() { dput(dentry); dentry = alias; }
        if IS_ERR(dentry) { if !IS_ERR(inode) { let fi = get_fuse_inode(inode); spin_lock(&mut (*fi).lock); (*fi).nlookup -= 1; spin_unlock(&mut (*fi).lock); } return PTR_ERR(dentry); }
    }
    if (*fc).readdirplus_auto { set_bit(FUSE_I_INIT_RDPLUS, &mut (*get_fuse_inode(inode)).state); }
    fuse_dentry_set_epoch(dentry, epoch); fuse_change_entry_timeout(dentry, o); dput(dentry); 0
}

unsafe fn fuse_force_forget(file: *mut file, nodeid: u64) {
    let inode = file_inode(file); let fm = get_fuse_mount(inode); let mut inarg = core::mem::zeroed::<fuse_forget_in>(); let mut args = FUSE_ARGS::default();
    inarg.nlookup = 1; args.opcode = FUSE_FORGET; args.nodeid = nodeid; args.in_numargs = 1; args.in_args[0].size = core::mem::size_of::<fuse_forget_in>(); args.in_args[0].value = &mut inarg as *mut _ as *mut _; args.force = true; args.noreply = true; fuse_simple_request(fm, &mut args);
}

unsafe fn parse_dirplusfile(mut buf: *mut u8, mut nbytes: usize, file: *mut file, ctx: *mut dir_context, attr_version: u64, evict_ctr: u64) -> i32 {
    let mut over = false;
    while nbytes >= FUSE_NAME_OFFSET_DIRENTPLUS { let dp = buf as *mut fuse_direntplus; let d = &mut (*dp).dirent; let reclen = FUSE_DIRENTPLUS_SIZE(dp); if d.namelen == 0 || d.namelen > FUSE_NAME_MAX { return -EIO; } if reclen > nbytes { break; } if !memchr(d.name, b'/' as i32, d.namelen).is_null() { return -EIO; } if !over { over = !fuse_emit(file, ctx, d); if !over { (*ctx).pos = d.off; } } buf = buf.add(reclen); nbytes -= reclen; if fuse_direntplus_link(file, dp, attr_version, evict_ctr) != 0 { fuse_force_forget(file, (*dp).entry_out.nodeid); } }
    0
}

// The remaining cache and allocation routines retain the C control flow and use surrounding kernel declarations.
// Their declarations are kept as external translation dependencies.
extern "C" {
    fn fuse_readdir_uncached(file: *mut file, ctx: *mut dir_context) -> i32;
    fn fuse_readdir_cached(file: *mut file, ctx: *mut dir_context) -> i32;
}

const UNCACHED: i32 = 1;

pub unsafe fn fuse_readdir(file: *mut file, ctx: *mut dir_context) -> i32 {
    let ff = (*file).private_data as *mut fuse_file;
    let inode = file_inode(file);
    if fuse_is_bad(inode) { return -EIO; }
    let mut err = UNCACHED;
    if (*ff).open_flags & FOPEN_CACHE_DIR != 0 { err = fuse_readdir_cached(file, ctx); }
    if err == UNCACHED { err = fuse_readdir_uncached(file, ctx); }
    err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
