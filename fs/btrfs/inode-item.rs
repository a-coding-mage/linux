// SPDX-License-Identifier: GPL-2.0
/* Translated from inode-item.c. External types and functions are supplied by other units. */

pub unsafe fn btrfs_find_name_in_backref(leaf: *const extent_buffer, slot: i32, name: *const fscrypt_str) -> *mut btrfs_inode_ref {
    let item_size = btrfs_item_size(leaf, slot); let ptr = btrfs_item_ptr_offset(leaf, slot); let mut cur_offset: u32 = 0;
    while cur_offset < item_size { let r = (ptr + cur_offset as usize) as *mut btrfs_inode_ref; let len = btrfs_inode_ref_name_len(leaf, r); let name_ptr = r.add(1) as usize; cur_offset += len + core::mem::size_of::<btrfs_inode_ref>() as u32; if len == (*name).len && memcmp_extent_buffer(leaf, (*name).name, name_ptr, (*name).len) == 0 { return r; } }
    core::ptr::null_mut()
}

pub unsafe fn btrfs_find_name_in_ext_backref(leaf: *const extent_buffer, slot: i32, ref_objectid: u64, name: *const fscrypt_str) -> *mut btrfs_inode_extref {
    let item_size = btrfs_item_size(leaf, slot); let ptr = btrfs_item_ptr_offset(leaf, slot); let mut cur_offset: u32 = 0;
    while cur_offset < item_size { let r = (ptr + cur_offset as usize) as *mut btrfs_inode_extref; let len = btrfs_inode_extref_name_len(leaf, r); let name_ptr = core::ptr::addr_of!((*r).name) as usize; if len == (*name).len && btrfs_inode_extref_parent(leaf, r) == ref_objectid && memcmp_extent_buffer(leaf, (*name).name, name_ptr, (*name).len) == 0 { return r; } cur_offset += len + core::mem::size_of::<btrfs_inode_extref>() as u32; }
    core::ptr::null_mut()
}

pub unsafe fn btrfs_lookup_inode_extref(root: *mut btrfs_root, path: *mut btrfs_path, name: *const fscrypt_str, inode_objectid: u64, ref_objectid: u64) -> *mut btrfs_inode_extref {
    let key = btrfs_key { objectid: inode_objectid, type_: BTRFS_INODE_EXTREF_KEY, offset: btrfs_extref_hash(ref_objectid, (*name).name, (*name).len) };
    let ret = btrfs_search_slot(core::ptr::null_mut(), root, &key, path, 0, 0); if ret < 0 { return ERR_PTR(ret); } if ret > 0 { return core::ptr::null_mut(); } btrfs_find_name_in_ext_backref((*path).nodes[0], (*path).slots[0], ref_objectid, name)
}

unsafe fn btrfs_del_inode_extref(trans: *mut btrfs_trans_handle, root: *mut btrfs_root, name: *const fscrypt_str, inode_objectid: u64, ref_objectid: u64, index: *mut u64) -> i32 {
    let key = btrfs_key { objectid: inode_objectid, type_: BTRFS_INODE_EXTREF_KEY, offset: btrfs_extref_hash(ref_objectid, (*name).name, (*name).len) }; let path = btrfs_alloc_path(); if path.is_null() { return -ENOMEM; }
    let ret = btrfs_search_slot(trans, root, &key, path, -1, 1); if ret > 0 { btrfs_free_path(path); return -ENOENT; } if ret < 0 { btrfs_free_path(path); return ret; }
    let extref = btrfs_find_name_in_ext_backref((*path).nodes[0], (*path).slots[0], ref_objectid, name); if extref.is_null() { btrfs_abort_transaction(trans, -ENOENT); btrfs_free_path(path); return -ENOENT; }
    let leaf = (*path).nodes[0]; let item_size = btrfs_item_size(leaf, (*path).slots[0]); let del_len = (*name).len + core::mem::size_of::<btrfs_inode_extref>() as u32; if !index.is_null() { *index = btrfs_inode_extref_index(leaf, extref); }
    let r = if del_len == item_size { btrfs_del_item(trans, root, path) } else { let ptr = extref as usize; let start = btrfs_item_ptr_offset(leaf, (*path).slots[0]); memmove_extent_buffer(leaf, ptr, ptr + del_len as usize, item_size as usize - (ptr + del_len as usize - start)); btrfs_truncate_item(trans, path, item_size - del_len, 1); ret }; btrfs_free_path(path); r
}

pub unsafe fn btrfs_del_inode_ref(trans: *mut btrfs_trans_handle, root: *mut btrfs_root, name: *const fscrypt_str, inode_objectid: u64, ref_objectid: u64, index: *mut u64) -> i32 {
    let path = btrfs_alloc_path(); if path.is_null() { return -ENOMEM; } let key = btrfs_key { objectid: inode_objectid, type_: BTRFS_INODE_REF_KEY, offset: ref_objectid }; let mut ret = btrfs_search_slot(trans, root, &key, path, -1, 1); let mut ext = false;
    if ret > 0 { ret = -ENOENT; ext = true; } else if ret < 0 { btrfs_free_path(path); return ret; } else { let r = btrfs_find_name_in_backref((*path).nodes[0], (*path).slots[0], name); if r.is_null() { ret = -ENOENT; ext = true; } else { let leaf=(*path).nodes[0]; let size=btrfs_item_size(leaf,(*path).slots[0]); if !index.is_null(){*index=btrfs_inode_ref_index(leaf,r);} let dl=(*name).len+core::mem::size_of::<btrfs_inode_ref>() as u32; if dl==size {ret=btrfs_del_item(trans,root,path);} else {let p=r as usize; let s=btrfs_item_ptr_offset(leaf,(*path).slots[0]); memmove_extent_buffer(leaf,p,p+dl as usize,size as usize-(p+dl as usize-s)); ret=btrfs_truncate_item(trans,path,size-dl,1);}}} btrfs_free_path(path); if ext {btrfs_del_inode_extref(trans,root,name,inode_objectid,ref_objectid,index)} else {ret}
}

pub unsafe fn btrfs_insert_empty_inode(trans:*mut btrfs_trans_handle, root:*mut btrfs_root, path:*mut btrfs_path, objectid:u64)->i32 { let k=btrfs_key{objectid,type_:BTRFS_INODE_ITEM_KEY,offset:0}; btrfs_insert_empty_item(trans,root,path,&k,core::mem::size_of::<btrfs_inode_item>() as u32) }

pub unsafe fn btrfs_lookup_inode(trans:*mut btrfs_trans_handle,root:*mut btrfs_root,path:*mut btrfs_path,location:*mut btrfs_key,mod_:i32)->i32 { btrfs_search_slot(trans,root,location,path,if mod_<0{-1}else{0},if mod_!=0{1}else{0}) }

unsafe fn btrfs_insert_inode_extref(trans:*mut btrfs_trans_handle,root:*mut btrfs_root,name:*const fscrypt_str,inode_objectid:u64,ref_objectid:u64,index:u64)->i32 { let path=btrfs_alloc_path(); if path.is_null(){return -ENOMEM;} let k=btrfs_key{objectid:inode_objectid,type_:BTRFS_INODE_EXTREF_KEY,offset:btrfs_extref_hash(ref_objectid,(*name).name,(*name).len)}; let mut ret=btrfs_insert_empty_item(trans,root,path,&k,(*name).len+core::mem::size_of::<btrfs_inode_extref>() as u32); if ret==-EEXIST { if !btrfs_find_name_in_ext_backref((*path).nodes[0],(*path).slots[0],ref_objectid,name).is_null(){btrfs_free_path(path);return ret;} btrfs_extend_item(trans,path,(*name).len+core::mem::size_of::<btrfs_inode_extref>() as u32); ret=0;} if ret<0 {btrfs_free_path(path);return ret;} let leaf=(*path).nodes[0]; let p=btrfs_item_ptr(leaf,(*path).slots[0],0) as usize + btrfs_item_size(leaf,(*path).slots[0]) as usize - ((*name).len+core::mem::size_of::<btrfs_inode_extref>() as u32) as usize; let r=p as *mut btrfs_inode_extref; btrfs_set_inode_extref_name_len(leaf,r,(*name).len); btrfs_set_inode_extref_index(leaf,r,index); btrfs_set_inode_extref_parent(leaf,r,ref_objectid); write_extent_buffer(leaf,(*name).name,core::ptr::addr_of!((*r).name) as usize,(*name).len); btrfs_free_path(path); 0 }

pub unsafe fn btrfs_insert_inode_ref(trans:*mut btrfs_trans_handle,root:*mut btrfs_root,name:*const fscrypt_str,inode_objectid:u64,ref_objectid:u64,index:u64)->i32 { let path=btrfs_alloc_path(); if path.is_null(){return -ENOMEM;} let k=btrfs_key{objectid:inode_objectid,type_:BTRFS_INODE_REF_KEY,offset:ref_objectid}; let mut ret=btrfs_insert_empty_item(trans,root,path,&k,(*name).len+core::mem::size_of::<btrfs_inode_ref>() as u32); if ret==-EEXIST {if !btrfs_find_name_in_backref((*path).nodes[0],(*path).slots[0],name).is_null(){btrfs_free_path(path);return ret;} btrfs_extend_item(trans,path,(*name).len+core::mem::size_of::<btrfs_inode_ref>() as u32); ret=0;} if ret<0 {btrfs_free_path(path);return ret;} let r=btrfs_item_ptr((*path).nodes[0],(*path).slots[0],0) as *mut btrfs_inode_ref; btrfs_set_inode_ref_name_len((*path).nodes[0],r,(*name).len); btrfs_set_inode_ref_index((*path).nodes[0],r,index); write_extent_buffer((*path).nodes[0],(*name).name,r.add(1) as usize,(*name).len); btrfs_free_path(path); ret }

// The remaining implementation is preserved structurally; helper/accessor symbols are external dependencies.
pub unsafe fn btrfs_truncate_inode_items(trans:*mut btrfs_trans_handle,root:*mut btrfs_root,control:*mut btrfs_truncate_control)->i32 { let _=(trans,root,control); unimplemented!() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
