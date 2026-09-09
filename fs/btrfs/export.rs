// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel/Btrfs translation.

const BTRFS_FID_SIZE_NON_CONNECTABLE: usize = core::mem::offset_of!(btrfs_fid, parent_objectid) / 4;
const BTRFS_FID_SIZE_CONNECTABLE: usize = core::mem::offset_of!(btrfs_fid, parent_root_objectid) / 4;
const BTRFS_FID_SIZE_CONNECTABLE_ROOT: usize = core::mem::size_of::<btrfs_fid>() / 4;

unsafe fn btrfs_encode_fh(
    inode: *mut inode,
    fh: *mut u32,
    max_len: *mut i32,
    parent: *mut inode,
) -> i32 {
    let fid = fh as *mut btrfs_fid;
    let mut len = *max_len;
    let mut ty: i32;

    if !parent.is_null() && (len < BTRFS_FID_SIZE_CONNECTABLE as i32) {
        if btrfs_root_id((*BTRFS_I(inode)).root) != btrfs_root_id((*BTRFS_I(parent)).root) {
            *max_len = BTRFS_FID_SIZE_CONNECTABLE_ROOT as i32;
        } else {
            *max_len = BTRFS_FID_SIZE_CONNECTABLE as i32;
        }
        return FILEID_INVALID;
    } else if len < BTRFS_FID_SIZE_NON_CONNECTABLE as i32 {
        *max_len = BTRFS_FID_SIZE_NON_CONNECTABLE as i32;
        return FILEID_INVALID;
    }

    len = BTRFS_FID_SIZE_NON_CONNECTABLE as i32;
    ty = FILEID_BTRFS_WITHOUT_PARENT;

    (*fid).objectid = btrfs_ino(BTRFS_I(inode));
    (*fid).root_objectid = btrfs_root_id((*BTRFS_I(inode)).root);
    (*fid).gen = (*inode).i_generation;

    if !parent.is_null() {
        let parent_root_id: u64;
        (*fid).parent_objectid = btrfs_ino(BTRFS_I(parent));
        (*fid).parent_gen = (*parent).i_generation;
        parent_root_id = btrfs_root_id((*BTRFS_I(parent)).root);

        if parent_root_id != (*fid).root_objectid {
            if *max_len < BTRFS_FID_SIZE_CONNECTABLE_ROOT as i32 {
                return FILEID_INVALID;
            }
            (*fid).parent_root_objectid = parent_root_id;
            len = BTRFS_FID_SIZE_CONNECTABLE_ROOT as i32;
            ty = FILEID_BTRFS_WITH_PARENT_ROOT;
        } else {
            len = BTRFS_FID_SIZE_CONNECTABLE as i32;
            ty = FILEID_BTRFS_WITH_PARENT;
        }
    }

    *max_len = len;
    ty
}

/* Read dentry of inode with @objectid from filesystem root @root_objectid.
 * Return dentry alias for the inode, otherwise an error. In case the
 * generation does not match return ESTALE.
 */
pub unsafe fn btrfs_get_dentry(
    sb: *mut super_block,
    objectid: u64,
    root_objectid: u64,
    generation: u64,
) -> *mut dentry {
    let fs_info = btrfs_sb(sb);
    let root = btrfs_get_fs_root(fs_info, root_objectid, true);
    if IS_ERR(root) { return ERR_CAST(root); }

    let inode = btrfs_iget(objectid, root);
    btrfs_put_root(root);
    if IS_ERR(inode) { return ERR_CAST(inode); }

    if generation != 0 && generation != (*(*inode).vfs_inode).i_generation {
        iput(&mut (*inode).vfs_inode);
        return ERR_PTR(-ESTALE);
    }
    d_obtain_alias(&mut (*inode).vfs_inode)
}

unsafe fn btrfs_fh_to_parent(sb: *mut super_block, fh: *mut fid, fh_len: i32, fh_type: i32) -> *mut dentry {
    let fid = fh as *mut btrfs_fid;
    let root_objectid: u64;
    if fh_type == FILEID_BTRFS_WITH_PARENT {
        if fh_len < BTRFS_FID_SIZE_CONNECTABLE as i32 { return core::ptr::null_mut(); }
        root_objectid = (*fid).root_objectid;
    } else if fh_type == FILEID_BTRFS_WITH_PARENT_ROOT {
        if fh_len < BTRFS_FID_SIZE_CONNECTABLE_ROOT as i32 { return core::ptr::null_mut(); }
        root_objectid = (*fid).parent_root_objectid;
    } else { return core::ptr::null_mut(); }
    btrfs_get_dentry(sb, (*fid).parent_objectid, root_objectid, (*fid).parent_gen as u64)
}

unsafe fn btrfs_fh_to_dentry(sb: *mut super_block, fh: *mut fid, fh_len: i32, fh_type: i32) -> *mut dentry {
    let fid = fh as *mut btrfs_fid;
    if (fh_type != FILEID_BTRFS_WITH_PARENT || fh_len < BTRFS_FID_SIZE_CONNECTABLE as i32)
        && (fh_type != FILEID_BTRFS_WITH_PARENT_ROOT || fh_len < BTRFS_FID_SIZE_CONNECTABLE_ROOT as i32)
        && (fh_type != FILEID_BTRFS_WITHOUT_PARENT || fh_len < BTRFS_FID_SIZE_NON_CONNECTABLE as i32) {
        return core::ptr::null_mut();
    }
    btrfs_get_dentry(sb, (*fid).objectid, (*fid).root_objectid, (*fid).gen as u64)
}

pub unsafe fn btrfs_get_parent(child: *mut dentry) -> *mut dentry {
    let dir = BTRFS_I(d_inode(child));
    let mut root = (*dir).root;
    let fs_info = (*root).fs_info;
    let path = btrfs_alloc_path();
    if path.is_null() { return ERR_PTR(-ENOMEM); }
    let mut key = btrfs_key { objectid: 0, type_: 0, offset: 0 };
    if btrfs_ino(dir) == BTRFS_FIRST_FREE_OBJECTID {
        key.objectid = btrfs_root_id(root); key.type_ = BTRFS_ROOT_BACKREF_KEY; key.offset = u64::MAX;
        root = (*fs_info).tree_root;
    } else {
        key.objectid = btrfs_ino(dir); key.type_ = BTRFS_INODE_REF_KEY; key.offset = u64::MAX;
    }
    let ret = btrfs_search_slot(core::ptr::null_mut(), root, &mut key, path, 0, 0);
    if ret < 0 { btrfs_free_path(path); return ERR_PTR(ret); }
    if ret == 0 || (*path).slots[0] == 0 { btrfs_free_path(path); return ERR_PTR(if ret == 0 { -EUCLEAN } else { -ENOENT }); }
    (*path).slots[0] -= 1;
    let leaf = (*path).nodes[0];
    let mut found_key = btrfs_key { objectid: 0, type_: 0, offset: 0 };
    btrfs_item_key_to_cpu(leaf, &mut found_key, (*path).slots[0]);
    if found_key.objectid != key.objectid || found_key.type_ != key.type_ { btrfs_free_path(path); return ERR_PTR(-ENOENT); }
    if found_key.type_ == BTRFS_ROOT_BACKREF_KEY {
        let rf = btrfs_item_ptr::<btrfs_root_ref>(leaf, (*path).slots[0]);
        key.objectid = btrfs_root_ref_dirid(leaf, rf);
    } else { key.objectid = found_key.offset; }
    btrfs_free_path(path);
    if found_key.type_ == BTRFS_ROOT_BACKREF_KEY { return btrfs_get_dentry((*fs_info).sb, key.objectid, found_key.offset, 0); }
    let inode = btrfs_iget(key.objectid, root);
    if IS_ERR(inode) { return ERR_CAST(inode); }
    d_obtain_alias(&mut (*inode).vfs_inode)
}

unsafe fn btrfs_get_name(parent: *mut dentry, name: *mut i8, child: *mut dentry) -> i32 {
    let inode = BTRFS_I(d_inode(child));
    let dir = BTRFS_I(d_inode(parent));
    if !S_ISDIR((*dir).vfs_inode.i_mode) { return -EINVAL; }
    let mut root = (*dir).root;
    let fs_info = (*root).fs_info;
    let path = btrfs_alloc_path();
    if path.is_null() { return -ENOMEM; }
    let ino = btrfs_ino(inode);
    let mut key = btrfs_key { objectid: ino, type_: BTRFS_INODE_REF_KEY, offset: btrfs_ino(dir) };
    if ino == BTRFS_FIRST_FREE_OBJECTID { key.objectid = btrfs_root_id((*inode).root); key.type_ = BTRFS_ROOT_BACKREF_KEY; key.offset = u64::MAX; root = (*fs_info).tree_root; }
    let ret = btrfs_search_slot(core::ptr::null_mut(), root, &mut key, path, 0, 0);
    if ret < 0 { return ret; }
    if ret > 0 { if ino == BTRFS_FIRST_FREE_OBJECTID { (*path).slots[0] -= 1; } else { return -ENOENT; } }
    let leaf = (*path).nodes[0];
    let (name_ptr, name_len) = if ino == BTRFS_FIRST_FREE_OBJECTID { let r = btrfs_item_ptr::<btrfs_root_ref>(leaf, (*path).slots[0]); ((r as usize + core::mem::size_of::<btrfs_root_ref>()) as u64, btrfs_root_ref_name_len(leaf, r)) } else { let r = btrfs_item_ptr::<btrfs_inode_ref>(leaf, (*path).slots[0]); ((r as usize + core::mem::size_of::<btrfs_inode_ref>()) as u64, btrfs_inode_ref_name_len(leaf, r)) };
    read_extent_buffer(leaf, name, name_ptr, name_len);
    *name.add(name_len as usize) = 0;
    0
}

pub static btrfs_export_ops: export_operations = export_operations {
    encode_fh: Some(btrfs_encode_fh), fh_to_dentry: Some(btrfs_fh_to_dentry), fh_to_parent: Some(btrfs_fh_to_parent), get_parent: Some(btrfs_get_parent), get_name: Some(btrfs_get_name),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
