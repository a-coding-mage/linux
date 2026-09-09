// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMFS (as used by RIO Karma) directory operations.
 * Copyright (C) 2005 Bob Copeland <me@bobcopeland.com>
 */

// Linux and omfs.h declarations are supplied by the surrounding translation.

unsafe fn omfs_hash(name: *const c_char, namelen: c_int, modulus: c_int) -> c_int {
    let mut hash: c_int = 0;
    let mut i: c_int = 0;
    while i < namelen {
        hash ^= (tolower(*name.add(i as usize)) as c_int) << (i % 24);
        i += 1;
    }
    hash % modulus
}

/* Finds the bucket for a given name and reads the containing block;
 * *ofs is set to the offset of the first list entry. */
unsafe fn omfs_get_bucket(
    dir: *mut inode, name: *const c_char, namelen: c_int, ofs: *mut c_int,
) -> *mut buffer_head {
    let nbuckets = ((*dir).i_size - OMFS_DIR_START as i64) / 8;
    let bucket = omfs_hash(name, namelen, nbuckets as c_int);
    *ofs = OMFS_DIR_START + bucket * 8;
    omfs_bread((*dir).i_sb, (*dir).i_ino)
}

unsafe fn omfs_scan_list(
    dir: *mut inode, mut block: u64, name: *const c_char, namelen: c_int,
    prev_block: *mut u64,
) -> *mut buffer_head {
    let mut err: c_int = -ENOENT;
    *prev_block = !0;
    while block != !0 {
        let bh = omfs_bread((*dir).i_sb, block);
        if bh.is_null() { err = -EIO; break; }
        let oi = (*bh).b_data as *mut omfs_inode;
        if omfs_is_bad(OMFS_SB((*dir).i_sb), &mut (*oi).i_head, block) != 0 {
            brelse(bh); break;
        }
        if strncmp((*oi).i_name.as_ptr(), name, namelen as usize) == 0 { return bh; }
        *prev_block = block;
        block = be64_to_cpu((*oi).i_sibling);
        brelse(bh);
    }
    ERR_PTR(err)
}

unsafe fn omfs_find_entry(dir: *mut inode, name: *const c_char, namelen: c_int) -> *mut buffer_head {
    let mut ofs = 0;
    let bh = omfs_get_bucket(dir, name, namelen, &mut ofs);
    if bh.is_null() { return ERR_PTR(-EIO); }
    let block = be64_to_cpu(*( ((*bh).b_data.add(ofs as usize)) as *mut __be64 ));
    brelse(bh);
    let mut dummy = 0;
    omfs_scan_list(dir, block, name, namelen, &mut dummy)
}

pub unsafe fn omfs_make_empty(inode: *mut inode, sb: *mut super_block) -> c_int {
    let sbi = OMFS_SB(sb);
    let bh = omfs_bread(sb, (*inode).i_ino);
    if bh.is_null() { return -ENOMEM; }
    memset((*bh).b_data, 0, size_of::<omfs_inode>());
    if S_ISDIR((*inode).i_mode) {
        memset((*bh).b_data.add(OMFS_DIR_START as usize), 0xff,
               ((*sbi).s_sys_blocksize - OMFS_DIR_START) as usize);
    } else { omfs_make_empty_table(bh, OMFS_EXTENT_START); }
    let oi = (*bh).b_data as *mut omfs_inode;
    (*oi).i_head.h_self = cpu_to_be64((*inode).i_ino);
    (*oi).i_sibling = !cpu_to_be64(0u64);
    mark_buffer_dirty(bh); brelse(bh); 0
}

unsafe fn omfs_add_link(dentry: *mut dentry, inode: *mut inode) -> c_int {
    let dir = d_inode((*dentry).d_parent);
    let name = (*dentry).d_name.name;
    let namelen = (*dentry).d_name.len as c_int;
    let mut ofs = 0;
    let bh = omfs_get_bucket(dir, name, namelen, &mut ofs);
    if bh.is_null() { return -ENOMEM; }
    let entry = (*bh).b_data.add(ofs as usize) as *mut __be64;
    let block = be64_to_cpu(*entry);
    *entry = cpu_to_be64((*inode).i_ino); mark_buffer_dirty(bh); brelse(bh);
    let bh = omfs_bread((*dir).i_sb, (*inode).i_ino);
    if bh.is_null() { return -ENOMEM; }
    let oi = (*bh).b_data as *mut omfs_inode;
    memcpy((*oi).i_name.as_mut_ptr(), name, namelen as usize);
    memset((*oi).i_name.as_mut_ptr().add(namelen as usize), 0,
           (OMFS_NAMELEN - namelen) as usize);
    (*oi).i_sibling = cpu_to_be64(block); (*oi).i_parent = cpu_to_be64((*dir).i_ino);
    mark_buffer_dirty(bh); brelse(bh); inode_set_ctime_current(dir);
    mark_inode_dirty(dir); mark_inode_dirty(inode); 0
}

unsafe fn omfs_delete_entry(dentry: *mut dentry) -> c_int {
    let dir = d_inode((*dentry).d_parent); let mut err = -ENOMEM;
    let name = (*dentry).d_name.name; let namelen = (*dentry).d_name.len as c_int;
    let mut ofs = 0; let mut bh = omfs_get_bucket(dir, name, namelen, &mut ofs);
    if bh.is_null() { return err; }
    let mut entry = (*bh).b_data.add(ofs as usize) as *mut __be64;
    let block = be64_to_cpu(*entry); let mut prev = 0;
    let bh2 = omfs_scan_list(dir, block, name, namelen, &mut prev);
    if IS_ERR(bh2) { err = PTR_ERR(bh2); brelse(bh); return err; }
    let next = (*( (*bh2).b_data as *mut omfs_inode )).i_sibling; brelse(bh2);
    if prev != !0u64 {
        brelse(bh); bh = omfs_bread((*dir).i_sb, prev); if bh.is_null() { return err; }
        entry = &mut (*( (*bh).b_data as *mut omfs_inode )).i_sibling;
    }
    *entry = next; mark_buffer_dirty(bh);
    if prev != !0u64 { let dirty = omfs_iget((*dir).i_sb, prev); if !IS_ERR(dirty) { mark_inode_dirty(dirty); iput(dirty); } }
    brelse(bh); 0
}

unsafe fn omfs_dir_is_empty(inode: *mut inode) -> c_int {
    let nbuckets = ((*inode).i_size - OMFS_DIR_START as i64) / 8;
    let bh = omfs_bread((*inode).i_sb, (*inode).i_ino); if bh.is_null() { return 0; }
    let ptr = (*bh).b_data.add(OMFS_DIR_START as usize) as *mut u64;
    let mut i = 0; while i < nbuckets { if *ptr.add(i as usize) != !0 { break; } i += 1; }
    brelse(bh); if i < nbuckets { 1 } else { 0 }
}

unsafe fn omfs_remove(dir: *mut inode, dentry: *mut dentry) -> c_int {
    let inode = d_inode(dentry);
    if S_ISDIR((*inode).i_mode) && omfs_dir_is_empty(inode) == 0 { return -ENOTEMPTY; }
    let ret = omfs_delete_entry(dentry); if ret != 0 { return ret; }
    clear_nlink(inode); mark_inode_dirty(inode); mark_inode_dirty(dir); 0
}

unsafe fn omfs_add_node(dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> c_int {
    let inode = omfs_new_inode(dir, mode);
    if IS_ERR(inode) { return PTR_ERR(inode); }
    let err = omfs_make_empty(inode, (*dir).i_sb);
    if err != 0 { iput(inode); return err; }
    let err = omfs_add_link(dentry, inode);
    if err != 0 { iput(inode); return err; }
    d_instantiate(dentry, inode); 0
}

unsafe fn omfs_mkdir(_idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> *mut dentry {
    ERR_PTR(omfs_add_node(dir, dentry, mode))
}

unsafe fn omfs_create(_idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> c_int {
    omfs_add_node(dir, dentry, mode | S_IFREG)
}

unsafe fn omfs_lookup(dir: *mut inode, dentry: *mut dentry, _flags: c_uint) -> *mut dentry {
    if (*dentry).d_name.len > OMFS_NAMELEN as usize { return ERR_PTR(-ENAMETOOLONG); }
    let bh = omfs_find_entry(dir, (*dentry).d_name.name, (*dentry).d_name.len as c_int);
    let mut inode: *mut inode = core::ptr::null_mut();
    if !IS_ERR(bh) {
        let oi = (*bh).b_data as *mut omfs_inode;
        let ino = be64_to_cpu((*oi).i_head.h_self); brelse(bh); inode = omfs_iget((*dir).i_sb, ino);
    } else if bh != ERR_PTR(-ENOENT) { inode = ERR_CAST(bh); }
    d_splice_alias(inode, dentry)
}

pub unsafe fn omfs_is_bad(sbi: *mut omfs_sb_info, header: *mut omfs_header, fsblock: u64) -> c_int {
    let ino = be64_to_cpu((*header).h_self);
    let bad = (ino != fsblock || ino < (*sbi).s_root_ino || ino > (*sbi).s_num_blocks) as c_int;
    if bad != 0 { printk(KERN_WARNING, "omfs: bad hash chain detected\n"); } bad
}

unsafe fn omfs_fill_chain(dir: *mut inode, ctx: *mut dir_context, mut fsblock: u64, mut hindex: c_int) -> bool {
    while fsblock != !0 {
        let bh = omfs_bread((*dir).i_sb, fsblock); if bh.is_null() { return true; }
        let oi = (*bh).b_data as *mut omfs_inode;
        if omfs_is_bad(OMFS_SB((*dir).i_sb), &mut (*oi).i_head, fsblock) != 0 { brelse(bh); return true; }
        let self_block = fsblock; fsblock = be64_to_cpu((*oi).i_sibling);
        if hindex != 0 { hindex -= 1; brelse(bh); continue; }
        let d_type = if (*oi).i_type == OMFS_DIR { DT_DIR } else { DT_REG };
        if !dir_emit(ctx, (*oi).i_name.as_ptr(), strnlen((*oi).i_name.as_ptr(), OMFS_NAMELEN), self_block, d_type) { brelse(bh); return false; }
        brelse(bh); (*ctx).pos += 1;
    } true
}

unsafe fn omfs_rename(_idmap: *mut mnt_idmap, old_dir: *mut inode, old_dentry: *mut dentry, new_dir: *mut inode, new_dentry: *mut dentry, flags: c_uint) -> c_int {
    if flags & !RENAME_NOREPLACE != 0 { return -EINVAL; }
    let new_inode = d_inode(new_dentry); let old_inode = d_inode(old_dentry);
    if !new_inode.is_null() { let err = omfs_remove(new_dir, new_dentry); if err != 0 { return err; } }
    let err = omfs_delete_entry(old_dentry); if err != 0 { return err; }
    mark_inode_dirty(old_dir); let err = omfs_add_link(new_dentry, old_inode); if err != 0 { return err; }
    inode_set_ctime_current(old_inode); mark_inode_dirty(old_inode); 0
}

unsafe fn omfs_readdir(file: *mut file, ctx: *mut dir_context) -> c_int {
    let dir = file_inode(file); if (*ctx).pos >> 32 != 0 { return -EINVAL; }
    if (*ctx).pos < 1 << 20 { if !dir_emit_dots(file, ctx) { return 0; } (*ctx).pos = 1 << 20; }
    let nbuckets = ((*dir).i_size - OMFS_DIR_START as i64) / 8;
    let mut hchain = ((*ctx).pos >> 20) - 1; let mut hindex = (*ctx).pos & 0xfffff;
    let bh = omfs_bread((*dir).i_sb, (*dir).i_ino); if bh.is_null() { return -EINVAL; }
    let mut p = (*bh).b_data.add(OMFS_DIR_START as usize) as *mut __be64;
    p = p.add(hchain as usize);
    while hchain < nbuckets as u64 { let fsblock = be64_to_cpu(*p); p = p.add(1); if !omfs_fill_chain(dir, ctx, fsblock, hindex as c_int) { break; } hindex = 0; hchain += 1; (*ctx).pos = (hchain + 1) << 20; }
    brelse(bh); 0
}

// VFS operation definitions retain the C interfaces and use declarations supplied by kernel bindings.
pub const omfs_dir_inops: inode_operations = inode_operations { lookup: Some(omfs_lookup), mkdir: Some(omfs_mkdir), rename: Some(omfs_rename), create: Some(omfs_create), unlink: Some(omfs_remove), rmdir: Some(omfs_remove) };
pub const omfs_dir_operations: file_operations = file_operations { read: Some(generic_read_dir), iterate_shared: Some(omfs_readdir), llseek: Some(generic_file_llseek) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
