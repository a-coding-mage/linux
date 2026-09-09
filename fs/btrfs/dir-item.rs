// SPDX-License-Identifier: GPL-2.0
/* Translated from dir-item.c. C headers and external symbols are supplied by dependencies. */

use core::ffi::c_void;

extern "C" {
    fn btrfs_insert_empty_item(trans: *mut btrfs_trans_handle, root: *mut btrfs_root,
        path: *mut btrfs_path, key: *const btrfs_key, data_size: u32) -> i32;
    fn btrfs_extend_item(trans: *mut btrfs_trans_handle, path: *mut btrfs_path, data_size: u32);
    fn btrfs_item_ptr(leaf: *mut extent_buffer, slot: i32, ty: *const c_void) -> *mut c_void;
    fn btrfs_item_size(leaf: *mut extent_buffer, slot: i32) -> u32;
    fn btrfs_name_hash(name: *const i8, len: u16) -> u64;
    fn btrfs_cpu_key_to_disk(disk: *mut btrfs_disk_key, key: *const btrfs_key);
    fn btrfs_set_dir_item_key(leaf: *mut extent_buffer, item: *mut btrfs_dir_item, key: *const btrfs_disk_key);
    fn btrfs_set_dir_flags(leaf: *mut extent_buffer, item: *mut btrfs_dir_item, flags: u8);
    fn btrfs_set_dir_name_len(leaf: *mut extent_buffer, item: *mut btrfs_dir_item, len: u16);
    fn btrfs_set_dir_transid(leaf: *mut extent_buffer, item: *mut btrfs_dir_item, transid: u64);
    fn btrfs_set_dir_data_len(leaf: *mut extent_buffer, item: *mut btrfs_dir_item, len: u16);
    fn write_extent_buffer(leaf: *mut extent_buffer, src: *const c_void, dst: usize, len: usize);
    fn btrfs_alloc_path() -> *mut btrfs_path;
    fn btrfs_release_path(path: *mut btrfs_path);
    fn btrfs_insert_delayed_dir_index(trans: *mut btrfs_trans_handle, name: *const i8, len: u16,
        dir: *mut btrfs_inode, key: *const btrfs_disk_key, ty: u8, index: u64) -> i32;
    fn btrfs_search_slot(trans: *mut btrfs_trans_handle, root: *mut btrfs_root, key: *mut btrfs_key,
        path: *mut btrfs_path, ins_len: i32, cow: i32) -> i32;
    fn btrfs_ino(dir: *mut btrfs_inode) -> u64;
    fn btrfs_leaf_data_size(fs_info: *mut btrfs_fs_info) -> u32;
    fn btrfs_item_ptr_offset(leaf: *mut extent_buffer, slot: i32) -> usize;
    fn memcmp_extent_buffer(leaf: *mut extent_buffer, src: *const i8, dst: usize, len: i32) -> i32;
    fn btrfs_dir_name_len(leaf: *mut extent_buffer, item: *const btrfs_dir_item) -> u16;
    fn btrfs_dir_data_len(leaf: *mut extent_buffer, item: *const btrfs_dir_item) -> u16;
    fn btrfs_del_item(trans: *mut btrfs_trans_handle, root: *mut btrfs_root, path: *mut btrfs_path) -> i32;
    fn memmove_extent_buffer(leaf: *mut extent_buffer, dst: usize, src: usize, len: usize);
    fn btrfs_truncate_item(trans: *mut btrfs_trans_handle, path: *mut btrfs_path, len: u32, from_end: i32);
}

#[repr(C)] pub struct btrfs_trans_handle { pub transid: u64 }
#[repr(C)] pub struct btrfs_root { pub fs_info: *mut btrfs_fs_info }
#[repr(C)] pub struct btrfs_fs_info { pub tree_root: *mut btrfs_root }
#[repr(C)] pub struct btrfs_path { pub nodes: [*mut extent_buffer; 1], pub slots: [i32; 1] }
#[repr(C)] pub struct extent_buffer;
#[repr(C)] pub struct btrfs_dir_item;
#[repr(C)] pub struct btrfs_disk_key;
#[repr(C)] pub struct btrfs_item;
#[repr(C)] pub struct btrfs_inode { pub root: *mut btrfs_root, pub vfs_inode: c_void }
#[repr(C)] pub struct fscrypt_str { pub name: *const i8, pub len: u16 }
#[repr(C)] #[derive(Clone, Copy)] pub struct btrfs_key { pub objectid: u64, pub type_: u8, pub offset: u64 }

const EEXIST: i32 = 17; const ENOSPC: i32 = 28; const ENOMEM: i32 = 12;
const ENOENT: i32 = 2; const EOVERFLOW: i32 = 75;
const BTRFS_DIR_ITEM_KEY: u8 = 84; const BTRFS_DIR_INDEX_KEY: u8 = 96;
const BTRFS_XATTR_ITEM_KEY: u8 = 24; const BTRFS_FT_XATTR: u8 = 8; const BTRFS_FT_ENCRYPTED: u8 = 8;

#[inline] unsafe fn err_ptr<T>(e: i32) -> *mut T { e as isize as *mut T }
#[inline] unsafe fn ptr_err<T>(p: *mut T) -> i32 { p as isize as i32 }
#[inline] unsafe fn is_err<T>(p: *mut T) -> bool { (p as isize) < 0 }

unsafe fn insert_with_overflow(trans: *mut btrfs_trans_handle, root: *mut btrfs_root,
    path: *mut btrfs_path, cpu_key: *const btrfs_key, data_size: u32,
    name: *const i8, name_len: i32) -> *mut btrfs_dir_item {
    let ret = btrfs_insert_empty_item(trans, root, path, cpu_key, data_size);
    if ret == -EEXIST {
        let di = btrfs_match_dir_item_name(path, name, name_len);
        if !di.is_null() { return err_ptr(-EEXIST); }
        btrfs_extend_item(trans, path, data_size);
    } else if ret < 0 { return err_ptr(ret); }
    let leaf = (*path).nodes[0];
    let ptr = btrfs_item_ptr(leaf, (*path).slots[0], core::ptr::null());
    let size = btrfs_item_size(leaf, (*path).slots[0]);
    (ptr as *mut u8).add((size - data_size) as usize) as *mut btrfs_dir_item
}

pub unsafe fn btrfs_insert_xattr_item(trans: *mut btrfs_trans_handle, root: *mut btrfs_root,
    path: *mut btrfs_path, objectid: u64, name: *const i8, name_len: u16,
    data: *const c_void, data_len: u16) -> i32 {
    let mut key = btrfs_key { objectid, type_: BTRFS_XATTR_ITEM_KEY, offset: btrfs_name_hash(name, name_len) };
    let data_size = core::mem::size_of::<btrfs_dir_item>() as u32 + name_len as u32 + data_len as u32;
    let di = insert_with_overflow(trans, root, path, &key, data_size, name, name_len as i32);
    if is_err(di) { return ptr_err(di); }
    let mut location = btrfs_key { objectid: 0, type_: 0, offset: 0 };
    let mut disk_key = core::mem::zeroed::<btrfs_disk_key>();
    let leaf = (*path).nodes[0]; btrfs_cpu_key_to_disk(&mut disk_key, &location);
    btrfs_set_dir_item_key(leaf, di, &disk_key); btrfs_set_dir_flags(leaf, di, BTRFS_FT_XATTR);
    btrfs_set_dir_name_len(leaf, di, name_len); btrfs_set_dir_transid(leaf, di, (*trans).transid);
    btrfs_set_dir_data_len(leaf, di, data_len);
    let name_ptr = di.add(1) as usize; write_extent_buffer(leaf, name as *const c_void, name_ptr, name_len as usize);
    write_extent_buffer(leaf, data, name_ptr + name_len as usize, data_len as usize); 0
}

pub unsafe fn btrfs_insert_dir_item(trans: *mut btrfs_trans_handle, name: *const fscrypt_str,
    dir: *mut btrfs_inode, location: *const btrfs_key, mut ty: u8, index: u64) -> i32 {
    let root = (*dir).root; let path = btrfs_alloc_path(); if path.is_null() { return -ENOMEM; }
    let mut key = btrfs_key { objectid: btrfs_ino(dir), type_: BTRFS_DIR_ITEM_KEY, offset: btrfs_name_hash((*name).name, (*name).len) };
    let mut disk_key = core::mem::zeroed::<btrfs_disk_key>(); btrfs_cpu_key_to_disk(&mut disk_key, location);
    let size = core::mem::size_of::<btrfs_dir_item>() as u32 + (*name).len as u32;
    let di = insert_with_overflow(trans, root, path, &key, size, (*name).name, (*name).len as i32);
    let mut ret = 0; let mut ret2 = 0;
    if is_err(di) { ret = ptr_err(di); if ret != -EEXIST { return ret; } }
    if !is_err(di) { let leaf = (*path).nodes[0]; btrfs_set_dir_item_key(leaf, di, &disk_key); btrfs_set_dir_flags(leaf, di, ty); btrfs_set_dir_data_len(leaf, di, 0); btrfs_set_dir_name_len(leaf, di, (*name).len); btrfs_set_dir_transid(leaf, di, (*trans).transid); write_extent_buffer(leaf, (*name).name as *const c_void, di.add(1) as usize, (*name).len as usize); }
    if root != (*(*root).fs_info).tree_root { btrfs_release_path(path); ret2 = btrfs_insert_delayed_dir_index(trans, (*name).name, (*name).len, dir, &disk_key, ty, index); }
    if ret != 0 { ret } else if ret2 != 0 { ret2 } else { 0 }
}

unsafe fn btrfs_lookup_match_dir(trans: *mut btrfs_trans_handle, root: *mut btrfs_root, path: *mut btrfs_path,
    key: *mut btrfs_key, name: *const i8, name_len: i32, modifier: i32) -> *mut btrfs_dir_item {
    let ret = btrfs_search_slot(trans, root, key, path, if modifier < 0 { -1 } else { 0 }, if modifier != 0 { 1 } else { 0 });
    if ret < 0 { return err_ptr(ret); } if ret > 0 { return err_ptr(-ENOENT); }
    btrfs_match_dir_item_name(path, name, name_len)
}

pub unsafe fn btrfs_lookup_dir_item(trans: *mut btrfs_trans_handle, root: *mut btrfs_root, path: *mut btrfs_path,
    dir: u64, name: *const fscrypt_str, modifier: i32) -> *mut btrfs_dir_item {
    let mut key = btrfs_key { objectid: dir, type_: BTRFS_DIR_ITEM_KEY, offset: btrfs_name_hash((*name).name, (*name).len) };
    let p = btrfs_lookup_match_dir(trans, root, path, &mut key, (*name).name, (*name).len as i32, modifier);
    if is_err(p) && ptr_err(p) == -ENOENT { core::ptr::null_mut() } else { p }
}

pub unsafe fn btrfs_check_dir_item_collision(root: *mut btrfs_root, dir_ino: u64, name: *const fscrypt_str) -> i32 {
    let path = btrfs_alloc_path(); if path.is_null() { return -ENOMEM; }
    let mut key = btrfs_key { objectid: dir_ino, type_: BTRFS_DIR_ITEM_KEY, offset: btrfs_name_hash((*name).name, (*name).len) };
    let di = btrfs_lookup_match_dir(core::ptr::null_mut(), root, path, &mut key, (*name).name, (*name).len as i32, 0);
    if is_err(di) { let ret = ptr_err(di); return if ret == -ENOENT { 0 } else { ret }; }
    if !di.is_null() { return -EEXIST; }
    let size = core::mem::size_of::<btrfs_dir_item>() as u32 + (*name).len as u32;
    let leaf = (*path).nodes[0]; let slot = (*path).slots[0];
    if size + btrfs_item_size(leaf, slot) + core::mem::size_of::<btrfs_item>() as u32 > btrfs_leaf_data_size((*root).fs_info) { -EOVERFLOW } else { 0 }
}

pub unsafe fn btrfs_lookup_dir_index_item(trans: *mut btrfs_trans_handle, root: *mut btrfs_root, path: *mut btrfs_path,
    dir: u64, index: u64, name: *const fscrypt_str, modifier: i32) -> *mut btrfs_dir_item {
    let mut key = btrfs_key { objectid: dir, type_: BTRFS_DIR_INDEX_KEY, offset: index };
    let di = btrfs_lookup_match_dir(trans, root, path, &mut key, (*name).name, (*name).len as i32, modifier);
    if di == err_ptr(-ENOENT) { core::ptr::null_mut() } else { di }
}

pub unsafe fn btrfs_lookup_xattr(trans: *mut btrfs_trans_handle, root: *mut btrfs_root, path: *mut btrfs_path,
    dir: u64, name: *const i8, name_len: u16, modifier: i32) -> *mut btrfs_dir_item {
    let mut key = btrfs_key { objectid: dir, type_: BTRFS_XATTR_ITEM_KEY, offset: btrfs_name_hash(name, name_len) };
    let di = btrfs_lookup_match_dir(trans, root, path, &mut key, name, name_len as i32, modifier);
    if is_err(di) && ptr_err(di) == -ENOENT { core::ptr::null_mut() } else { di }
}

pub unsafe fn btrfs_match_dir_item_name(path: *const btrfs_path, name: *const i8, name_len: i32) -> *mut btrfs_dir_item {
    let leaf = (*path).nodes[0]; let mut di = btrfs_item_ptr(leaf, (*path).slots[0], core::ptr::null()) as *mut btrfs_dir_item;
    let total = btrfs_item_size(leaf, (*path).slots[0]); let mut cur = 0;
    while cur < total { let this_len = core::mem::size_of::<btrfs_dir_item>() as u32 + btrfs_dir_name_len(leaf, di) as u32 + btrfs_dir_data_len(leaf, di) as u32;
        if btrfs_dir_name_len(leaf, di) as i32 == name_len && memcmp_extent_buffer(leaf, name, di.add(1) as usize, name_len) == 0 { return di; }
        cur += this_len; di = (di as *mut u8).add(this_len as usize) as *mut btrfs_dir_item; }
    core::ptr::null_mut()
}

pub unsafe fn btrfs_search_dir_index_item(root: *mut btrfs_root, path: *mut btrfs_path,
    dirid: u64, name: *const fscrypt_str) -> *mut btrfs_dir_item {
    let mut key = btrfs_key { objectid: dirid, type_: BTRFS_DIR_INDEX_KEY, offset: 0 };
    let mut ret = 0;
    // C btrfs_for_each_slot(root, &key, &key, path, ret) loop.
    loop {
        if key.objectid != dirid || key.type_ != BTRFS_DIR_INDEX_KEY { break; }
        let di = btrfs_match_dir_item_name(path, (*name).name, (*name).len as i32);
        if !di.is_null() { return di; }
        ret = btrfs_search_slot(core::ptr::null_mut(), root, &mut key, path, 0, 0);
        if ret != 0 { break; }
    }
    if ret >= 0 { ret = -ENOENT; }
    err_ptr(ret)
}

pub unsafe fn btrfs_delete_one_dir_name(trans: *mut btrfs_trans_handle, root: *mut btrfs_root, path: *mut btrfs_path, di: *const btrfs_dir_item) -> i32 {
    let leaf = (*path).nodes[0]; let sub = core::mem::size_of::<btrfs_dir_item>() as u32 + btrfs_dir_name_len(leaf, di) as u32 + btrfs_dir_data_len(leaf, di) as u32; let item = btrfs_item_size(leaf, (*path).slots[0]);
    if sub == item { btrfs_del_item(trans, root, path) } else { let ptr = di as usize; let start = btrfs_item_ptr_offset(leaf, (*path).slots[0]); memmove_extent_buffer(leaf, ptr, ptr + sub as usize, item as usize - (ptr + sub as usize - start)); btrfs_truncate_item(trans, path, item - sub, 1); 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
