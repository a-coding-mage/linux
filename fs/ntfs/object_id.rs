// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Pocessing of object ids
 *
 * Part of this file is based on code from the NTFS-3G.
 *
 * Copyright (c) 2009-2019 Jean-Pierre Andre
 * Copyright (c) 2026 LG Electronics Co., Ltd.
 */

#[repr(C, packed)]
struct object_id_index_key {
    object_id: object_id_index_key_object_id,
}

#[repr(C)]
union object_id_index_key_object_id {
    alignment: u32,
    guid: guid,
}

#[repr(C, packed)]
struct object_id_index_data {
    file_id: __le64,
    birth_volume_id: guid,
    birth_object_id: guid,
    domain_id: guid,
}

/* Index entry in $Extend/$ObjId */
#[repr(C, packed)]
struct object_id_index {
    header: index_entry_header,
    key: object_id_index_key,
    data: object_id_index_data,
}

static mut objid_index_name: [__le16; 3] = [
    cpu_to_le16(b'$' as u16),
    cpu_to_le16(b'O' as u16),
    0,
];

/*
 * open_object_id_index - Open the $Extend/$ObjId file and its index
 * @vol: NTFS volume structure
 *
 * Opens the $ObjId system file and retrieves its index context.
 *
 * Return: The index context if opened successfully, or NULL if an error
 *	   occurred.
 */
unsafe fn open_object_id_index(vol: *mut ntfs_volume) -> *mut ntfs_index_context {
    let mut dir_vi: *mut inode;
    let mut vi: *mut inode;
    let mut dir_ni: *mut ntfs_inode;
    let mut xo: *mut ntfs_index_context = core::ptr::null_mut();
    let mut name: *mut ntfs_name = core::ptr::null_mut();
    let mut mref: u64;
    let uname_len: i32;
    let mut uname: *mut __le16 = core::ptr::null_mut();

    uname_len = ntfs_nlstoucs(vol, b"$ObjId\0".as_ptr() as *const i8, 6, &mut uname,
        NTFS_MAX_NAME_LEN);
    if uname_len < 0 {
        return core::ptr::null_mut();
    }

    /* do not use path_name_to inode - could reopen root */
    dir_vi = ntfs_iget((*vol).sb, FILE_Extend);
    if IS_ERR(dir_vi) {
        kmem_cache_free(ntfs_name_cache, uname as *mut core::ffi::c_void);
        return core::ptr::null_mut();
    }
    dir_ni = NTFS_I(dir_vi);

    mutex_lock_nested(&mut (*dir_ni).mrec_lock, NTFS_EXTEND_MUTEX_PARENT);
    mref = ntfs_lookup_inode_by_name(dir_ni, uname, uname_len, &mut name);
    mutex_unlock(&mut (*dir_ni).mrec_lock);
    kfree(name as *mut core::ffi::c_void);
    kmem_cache_free(ntfs_name_cache, uname as *mut core::ffi::c_void);
    if !IS_ERR_MREF(mref) {
        vi = ntfs_iget((*vol).sb, MREF(mref));
        if !IS_ERR(vi) {
            xo = ntfs_index_ctx_get(NTFS_I(vi), objid_index_name.as_mut_ptr(), 2);
            if xo.is_null() {
                iput(vi);
            }
        }
    }
    iput(dir_vi);
    xo
}

/*
 * remove_object_id_index - Remove an object id index entry if attribute present
 * @ni: NTFS inode structure containing the attribute
 * @xo:	Index context for the object id index
 *
 * Reads the existing object ID attribute and removes it from the index.
 *
 * Return: 0 on success, or a negative error code on failure.
 */
unsafe fn remove_object_id_index(ni: *mut ntfs_inode, xo: *mut ntfs_index_context) -> i32 {
    let mut key: object_id_index_key = core::mem::zeroed();
    let size: i64;

    if (*ni).data_size == 0 {
        return -ENODATA;
    }

    /* read the existing object id attribute */
    size = ntfs_inode_attr_pread(VFS_I(ni), 0, core::mem::size_of::<guid>(),
        &mut key as *mut _ as *mut i8);
    if size != core::mem::size_of::<guid>() as i64 {
        return -ENODATA;
    }

    if !ntfs_index_lookup(&key as *const _ as *const core::ffi::c_void,
        core::mem::size_of::<object_id_index_key>(), xo) {
        return ntfs_index_rm(xo);
    }

    0
}

/*
 * ntfs_delete_object_id_index - Delete an object_id index entry
 * @ni: NTFS inode structure
 *
 * Opens the object ID index and removes the entry corresponding to the inode.
 *
 * Return: 0 on success, or a negative error code on failure.
 */
pub unsafe fn ntfs_delete_object_id_index(ni: *mut ntfs_inode) -> i32 {
    let mut xo: *mut ntfs_index_context;
    let mut xoni: *mut ntfs_inode;
    let attr_vi: *mut inode;
    let mut ret: i32 = 0;

    attr_vi = ntfs_attr_iget(VFS_I(ni), AT_OBJECT_ID, AT_UNNAMED, 0);
    if IS_ERR(attr_vi) {
        return PTR_ERR(attr_vi);
    }

    /*
     * read the existing object id and un-index it
     */
    xo = open_object_id_index((*ni).vol);
    if !xo.is_null() {
        xoni = (*xo).idx_ni;
        mutex_lock_nested(&mut (*xoni).mrec_lock, NTFS_EXTEND_MUTEX_PARENT);
        ret = remove_object_id_index(NTFS_I(attr_vi), xo);
        if ret == 0 {
            ntfs_index_entry_mark_dirty(xo);
            mark_mft_record_dirty(xoni);
        }
        ntfs_index_ctx_put(xo);
        mutex_unlock(&mut (*xoni).mrec_lock);
        iput(VFS_I(xoni));
    }

    iput(attr_vi);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
