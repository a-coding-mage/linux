// SPDX-License-Identifier: GPL-2.0
/* Translation of linux/fs/hfsplus/attributes.c. */

static mut HFSPLUS_ATTR_TREE_CACHEP: *mut kmem_cache = core::ptr::null_mut();

pub unsafe extern "C" fn hfsplus_create_attr_tree_cache() -> i32 {
    if !HFSPLUS_ATTR_TREE_CACHEP.is_null() { return -EEXIST; }
    HFSPLUS_ATTR_TREE_CACHEP = kmem_cache_create(
        b"hfsplus_attr_cache\0".as_ptr() as *const i8,
        core::mem::size_of::<hfsplus_attr_entry>(), 0,
        SLAB_HWCACHE_ALIGN, core::ptr::null_mut());
    if HFSPLUS_ATTR_TREE_CACHEP.is_null() { return -ENOMEM; }
    0
}

pub unsafe extern "C" fn hfsplus_destroy_attr_tree_cache() {
    kmem_cache_destroy(HFSPLUS_ATTR_TREE_CACHEP);
}

pub unsafe extern "C" fn hfsplus_attr_bin_cmp_key(k1: *const hfsplus_btree_key, k2: *const hfsplus_btree_key) -> i32 {
    let k1_cnid = (*k1).attr.cnid;
    let k2_cnid = (*k2).attr.cnid;
    if k1_cnid != k2_cnid { return if be32_to_cpu(k1_cnid) < be32_to_cpu(k2_cnid) { -1 } else { 1 }; }
    hfsplus_strcmp(&(*k1).attr.key_name as *const _ as *const hfsplus_unistr,
                   &(*k2).attr.key_name as *const _ as *const hfsplus_unistr)
}

pub unsafe extern "C" fn hfsplus_attr_build_key(sb: *mut super_block, key: *mut hfsplus_btree_key, cnid: u32, name: *const i8) -> i32 {
    core::ptr::write_bytes(key as *mut u8, 0, core::mem::size_of::<hfsplus_attr_key>());
    (*key).attr.cnid = cpu_to_be32(cnid);
    let len: u16;
    if !name.is_null() {
        let res = hfsplus_asc2uni(sb, &mut (*key).attr.key_name as *mut _ as *mut hfsplus_unistr,
            HFSPLUS_ATTR_MAX_STRLEN, name, strlen(name), HFS_XATTR_NAME);
        if res != 0 { return res; }
        len = be16_to_cpu((*key).attr.key_name.length);
    } else { (*key).attr.key_name.length = 0; len = 0; }
    (*key).key_len = cpu_to_be16((core::mem::offset_of!(hfsplus_attr_key, key_name) + 2 * len as usize) as u16);
    0
}

pub unsafe extern "C" fn hfsplus_alloc_attr_entry() -> *mut hfsplus_attr_entry {
    kmem_cache_alloc(HFSPLUS_ATTR_TREE_CACHEP, GFP_KERNEL)
}

pub unsafe extern "C" fn hfsplus_destroy_attr_entry(entry: *mut hfsplus_attr_entry) {
    if !entry.is_null() { kmem_cache_free(HFSPLUS_ATTR_TREE_CACHEP, entry); }
}

const HFSPLUS_INVALID_ATTR_RECORD: i32 = -1;

unsafe fn hfsplus_attr_build_record(entry: *mut hfsplus_attr_entry, record_type: i32, _cnid: u32, value: *const core::ffi::c_void, size: usize) -> i32 {
    if record_type == HFSPLUS_ATTR_FORK_DATA {
        core::ptr::write_bytes(entry as *mut u8, 0, core::mem::size_of::<hfsplus_attr_entry>());
        return core::mem::size_of::<hfsplus_attr_fork_data>() as i32;
    } else if record_type == HFSPLUS_ATTR_EXTENTS {
        core::ptr::write_bytes(entry as *mut u8, 0, core::mem::size_of::<hfsplus_attr_entry>());
        return core::mem::size_of::<hfsplus_attr_extents>() as i32;
    } else if record_type == HFSPLUS_ATTR_INLINE_DATA {
        core::ptr::write_bytes(entry as *mut u8, 0, core::mem::size_of::<hfsplus_attr_inline_data>());
        (*entry).inline_data.record_type = cpu_to_be32(record_type as u32);
        let len = if size <= HFSPLUS_MAX_INLINE_DATA_SIZE { size as u16 } else { hfs_dbg(b"value size %zu is too big\n".as_ptr() as *const i8, size); return HFSPLUS_INVALID_ATTR_RECORD; };
        (*entry).inline_data.length = cpu_to_be16(len);
        core::ptr::copy_nonoverlapping(value as *const u8, (*entry).inline_data.raw_bytes.as_mut_ptr(), len as usize);
        return (core::mem::offset_of!(hfsplus_attr_inline_data, raw_bytes) + ((len as usize + 1) & !1)) as i32;
    }
    core::ptr::write_bytes(entry as *mut u8, 0, core::mem::size_of::<hfsplus_attr_entry>());
    HFSPLUS_INVALID_ATTR_RECORD
}

pub unsafe extern "C" fn hfsplus_find_attr(sb: *mut super_block, cnid: u32, name: *const i8, fd: *mut hfs_find_data) -> i32 {
    let mut err = 0;
    hfs_dbg(b"name %s, cnid %d\n\0".as_ptr() as *const i8, name, cnid);
    if (*HFSPLUS_SB(sb)).attr_tree.is_null() { pr_err(b"attributes file doesn't exist\0".as_ptr() as *const i8); return -EINVAL; }
    err = hfsplus_attr_build_key(sb, (*fd).search_key, cnid, name);
    if err != 0 { return err; }
    err = if name.is_null() { hfs_brec_find(fd, hfs_find_1st_rec_by_cnid) } else { hfs_brec_find(fd, hfs_find_rec_by_key) };
    if err == -ENOENT { -ENODATA } else { err }
}

pub unsafe extern "C" fn hfsplus_attr_exists(inode: *mut inode, name: *const i8) -> i32 {
    let sb = (*inode).i_sb;
    if (*HFSPLUS_SB(sb)).attr_tree.is_null() { return 0; }
    let mut fd = core::mem::zeroed::<hfs_find_data>();
    if hfs_find_init((*HFSPLUS_SB(sb)).attr_tree, &mut fd) != 0 { return 0; }
    let err = hfsplus_find_attr(sb, (*inode).i_ino as u32, name, &mut fd);
    hfs_find_exit(&mut fd);
    if err == 0 { 1 } else { 0 }
}

unsafe fn hfsplus_create_attr_nolock(inode: *mut inode, name: *const i8, value: *const core::ffi::c_void, size: usize, fd: *mut hfs_find_data, entry: *mut hfsplus_attr_entry) -> i32 {
    let sb = (*inode).i_sb;
    if name.is_null() { return -EINVAL; }
    let mut err = hfsplus_attr_build_key(sb, (*fd).search_key, (*inode).i_ino as u32, name);
    if err != 0 { return err; }
    let entry_size = hfsplus_attr_build_record(entry, HFSPLUS_ATTR_INLINE_DATA, (*inode).i_ino as u32, value, size);
    if entry_size == HFSPLUS_INVALID_ATTR_RECORD { return if size > HFSPLUS_MAX_INLINE_DATA_SIZE { -E2BIG } else { -EINVAL }; }
    err = hfs_brec_find(fd, hfs_find_rec_by_key);
    if err != -ENOENT { return if err == 0 { -EEXIST } else { err }; }
    err = hfs_brec_insert(fd, entry, entry_size);
    if err != 0 { return err; }
    hfsplus_mark_inode_dirty(HFSPLUS_ATTR_TREE_I(sb), HFSPLUS_I_ATTR_DIRTY);
    hfsplus_mark_inode_dirty(inode, HFSPLUS_I_ATTR_DIRTY);
    0
}

pub unsafe extern "C" fn hfsplus_create_attr(inode: *mut inode, name: *const i8, value: *const core::ffi::c_void, size: usize) -> i32 {
    let sb = (*inode).i_sb;
    if (*HFSPLUS_SB(sb)).attr_tree.is_null() { return -EINVAL; }
    let entry = hfsplus_alloc_attr_entry(); if entry.is_null() { return -ENOMEM; }
    let mut fd = core::mem::zeroed::<hfs_find_data>();
    let mut err = hfs_find_init((*HFSPLUS_SB(sb)).attr_tree, &mut fd);
    if err == 0 { err = hfs_bmap_reserve(fd.tree, (*fd.tree).depth + 1); }
    if err == 0 { err = hfsplus_create_attr_nolock(inode, name, value, size, &mut fd, entry); }
    if !fd.tree.is_null() { hfs_find_exit(&mut fd); }
    hfsplus_destroy_attr_entry(entry); err
}

unsafe fn __hfsplus_delete_attr(inode: *mut inode, cnid: u32, fd: *mut hfs_find_data) -> i32 {
    let mut found_cnid = U32_MAX; hfs_bnode_read((*fd).bnode, &mut found_cnid, (*fd).keyoffset + core::mem::offset_of!(hfsplus_attr_key, cnid), 4);
    if cnid != be32_to_cpu(found_cnid) { return -ENODATA; }
    let mut record_type = 0; hfs_bnode_read((*fd).bnode, &mut record_type, (*fd).entryoffset, 4);
    match be32_to_cpu(record_type) as i32 { HFSPLUS_ATTR_INLINE_DATA => (), HFSPLUS_ATTR_FORK_DATA | HFSPLUS_ATTR_EXTENTS => return -EOPNOTSUPP, _ => return -ENOENT }
    hfs_bnode_read((*fd).bnode, (*fd).search_key, (*fd).keyoffset, (*fd).keylength);
    let err = hfs_brec_remove(fd); if err != 0 { return err; }
    hfsplus_mark_inode_dirty(HFSPLUS_ATTR_TREE_I((*inode).i_sb), HFSPLUS_I_ATTR_DIRTY); hfsplus_mark_inode_dirty(inode, HFSPLUS_I_ATTR_DIRTY); 0
}

unsafe fn hfsplus_delete_attr_nolock(inode: *mut inode, name: *const i8, fd: *mut hfs_find_data) -> i32 {
    if name.is_null() { return -EINVAL; }
    let mut err = hfsplus_attr_build_key((*inode).i_sb, (*fd).search_key, (*inode).i_ino as u32, name); if err != 0 { return err; }
    err = hfs_brec_find(fd, hfs_find_rec_by_key); if err == -ENOENT { return -ENODATA; } if err != 0 { return err; }
    __hfsplus_delete_attr(inode, (*inode).i_ino as u32, fd)
}

pub unsafe extern "C" fn hfsplus_delete_attr(inode: *mut inode, name: *const i8) -> i32 {
    let sb = (*inode).i_sb; if (*HFSPLUS_SB(sb)).attr_tree.is_null() { return -EINVAL; }
    let mut fd = core::mem::zeroed::<hfs_find_data>(); let mut err = hfs_find_init((*HFSPLUS_SB(sb)).attr_tree, &mut fd); if err == 0 { err = hfs_bmap_reserve(fd.tree, (*fd.tree).depth); } if err == 0 { err = hfsplus_delete_attr_nolock(inode, name, &mut fd); } if !fd.tree.is_null() { hfs_find_exit(&mut fd); } err
}

pub unsafe extern "C" fn hfsplus_delete_all_attrs(dir: *mut inode, cnid: u32) -> i32 {
    let sb = (*dir).i_sb; if (*HFSPLUS_SB(sb)).attr_tree.is_null() { return -EINVAL; }
    let mut fd = core::mem::zeroed::<hfs_find_data>(); let mut err = hfs_find_init((*HFSPLUS_SB(sb)).attr_tree, &mut fd); if err != 0 { return err; }
    loop { err = hfsplus_find_attr(sb, cnid, core::ptr::null(), &mut fd); if err == -ENOENT || err == -ENODATA { err = -ENODATA; break; } if err != 0 { break; } err = __hfsplus_delete_attr(dir, cnid, &mut fd); if err != 0 { break; } }
    hfs_find_exit(&mut fd); err
}

pub unsafe extern "C" fn hfsplus_replace_attr(inode: *mut inode, name: *const i8, value: *const core::ffi::c_void, size: usize) -> i32 {
    let sb = (*inode).i_sb; if (*HFSPLUS_SB(sb)).attr_tree.is_null() { return -EINVAL; }
    let entry = hfsplus_alloc_attr_entry(); if entry.is_null() { return -ENOMEM; }
    let mut fd = core::mem::zeroed::<hfs_find_data>(); let mut err = hfs_find_init((*HFSPLUS_SB(sb)).attr_tree, &mut fd);
    if err == 0 { err = hfs_bmap_reserve(fd.tree, (*fd.tree).depth + 1); } if err == 0 { err = hfsplus_delete_attr_nolock(inode, name, &mut fd); } if err == 0 { err = hfsplus_create_attr_nolock(inode, name, value, size, &mut fd, entry); }
    if !fd.tree.is_null() { hfs_find_exit(&mut fd); } hfsplus_destroy_attr_entry(entry); err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
