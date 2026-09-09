// SPDX-License-Identifier: MIT
/*
 * VirtualBox Guest Shared Folders support: Regular file inode and file ops.
 *
 * Copyright (C) 2006-2018 Oracle Corporation
 */

// Kernel and vboxsf dependencies are supplied by other translation units.

#[repr(C)]
pub struct vboxsf_handle {
    pub handle: u64,
    pub root: u32,
    pub access_flags: u32,
    pub refcount: kref,
    pub head: list_head,
}

pub unsafe fn vboxsf_create_sf_handle(
    inode: *mut inode,
    handle: u64,
    access_flags: u32,
) -> *mut vboxsf_handle {
    let sf_i = VBOXSF_I(inode);
    let sf_handle = kmalloc_obj::<vboxsf_handle>();
    if sf_handle.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    /* the host may have given us different attr then requested */
    (*sf_i).force_restat = 1;

    /* init our handle struct and add it to the inode's handles list */
    (*sf_handle).handle = handle;
    (*sf_handle).root = (*VBOXSF_SBI((*inode).i_sb)).root;
    (*sf_handle).access_flags = access_flags;
    kref_init(&mut (*sf_handle).refcount);

    mutex_lock(&mut (*sf_i).handle_list_mutex);
    list_add(&mut (*sf_handle).head, &mut (*sf_i).handle_list);
    mutex_unlock(&mut (*sf_i).handle_list_mutex);

    sf_handle
}

unsafe fn vboxsf_file_open(inode: *mut inode, file: *mut file) -> i32 {
    let sbi = VBOXSF_SBI((*inode).i_sb);
    let mut params: shfl_createparms = core::mem::zeroed();
    let mut sf_handle: *mut vboxsf_handle;
    let mut access_flags: u32 = 0;
    let mut err: i32;

    /*
     * We check the value of params.handle afterwards to find out if
     * the call succeeded or failed, as the API does not seem to cleanly
     * distinguish error and informational messages.
     *
     * Furthermore, we must set params.handle to SHFL_HANDLE_NIL to
     * make the shared folders host service use our mode parameter.
     */
    params.handle = SHFL_HANDLE_NIL;
    if (*file).f_flags & O_CREAT != 0 {
        params.create_flags |= SHFL_CF_ACT_CREATE_IF_NEW;
        /* We ignore O_EXCL, as the Linux kernel seems to call create beforehand itself. */
        if (*file).f_flags & O_TRUNC != 0 {
            params.create_flags |= SHFL_CF_ACT_OVERWRITE_IF_EXISTS;
        } else {
            params.create_flags |= SHFL_CF_ACT_OPEN_IF_EXISTS;
        }
    } else {
        params.create_flags |= SHFL_CF_ACT_FAIL_IF_NEW;
        if (*file).f_flags & O_TRUNC != 0 {
            params.create_flags |= SHFL_CF_ACT_OVERWRITE_IF_EXISTS;
        }
    }

    match (*file).f_flags & O_ACCMODE {
        O_RDONLY => access_flags |= SHFL_CF_ACCESS_READ,
        O_WRONLY => access_flags |= SHFL_CF_ACCESS_WRITE,
        O_RDWR => access_flags |= SHFL_CF_ACCESS_READWRITE,
        _ => WARN_ON(1),
    }
    if (*file).f_flags & O_APPEND != 0 {
        access_flags |= SHFL_CF_ACCESS_APPEND;
    }
    params.create_flags |= access_flags;
    params.info.attr.mode = (*inode).i_mode;

    err = vboxsf_create_at_dentry(file_dentry(file), &mut params);
    if err == 0 && params.handle == SHFL_HANDLE_NIL {
        err = if params.result == SHFL_FILE_EXISTS { -EEXIST } else { -ENOENT };
    }
    if err != 0 { return err; }

    sf_handle = vboxsf_create_sf_handle(inode, params.handle, access_flags);
    if IS_ERR(sf_handle) {
        vboxsf_close((*sbi).root, params.handle);
        return PTR_ERR(sf_handle);
    }
    (*file).private_data = sf_handle as *mut core::ffi::c_void;
    0
}

unsafe fn vboxsf_handle_release(refcount: *mut kref) {
    let sf_handle = container_of!(refcount, vboxsf_handle, refcount);
    vboxsf_close((*sf_handle).root, (*sf_handle).handle);
    kfree(sf_handle as *mut core::ffi::c_void);
}

pub unsafe fn vboxsf_release_sf_handle(inode: *mut inode, sf_handle: *mut vboxsf_handle) {
    let sf_i = VBOXSF_I(inode);
    mutex_lock(&mut (*sf_i).handle_list_mutex);
    list_del(&mut (*sf_handle).head);
    mutex_unlock(&mut (*sf_i).handle_list_mutex);
    kref_put(&mut (*sf_handle).refcount, vboxsf_handle_release);
}

unsafe fn vboxsf_file_release(inode: *mut inode, file: *mut file) -> i32 {
    filemap_write_and_wait((*inode).i_mapping);
    vboxsf_release_sf_handle(inode, (*file).private_data as *mut vboxsf_handle);
    0
}

unsafe fn vboxsf_vma_close(vma: *mut vm_area_struct) {
    filemap_write_and_wait((*(*vma).vm_file).f_mapping);
}

pub static vboxsf_file_vm_ops: vm_operations_struct = vm_operations_struct {
    close: Some(vboxsf_vma_close), fault: Some(filemap_fault), map_pages: Some(filemap_map_pages),
};

unsafe fn vboxsf_file_mmap_prepare(desc: *mut vm_area_desc) -> i32 {
    let err = generic_file_mmap_prepare(desc);
    if err == 0 { (*desc).vm_ops = &vboxsf_file_vm_ops; }
    err
}

pub static vboxsf_reg_fops: file_operations = file_operations {
    llseek: Some(generic_file_llseek), read_iter: Some(generic_file_read_iter),
    write_iter: Some(generic_file_write_iter), mmap_prepare: Some(vboxsf_file_mmap_prepare),
    open: Some(vboxsf_file_open), release: Some(vboxsf_file_release), fsync: Some(noop_fsync),
    splice_read: Some(filemap_splice_read),
};

pub static vboxsf_reg_iops: inode_operations = inode_operations {
    getattr: Some(vboxsf_getattr), setattr: Some(vboxsf_setattr), fileattr_get: Some(vboxsf_fileattr_get),
};

unsafe fn vboxsf_read_folio(file: *mut file, folio: *mut folio) -> i32 {
    let sf_handle = (*file).private_data as *mut vboxsf_handle;
    let off = folio_pos(folio);
    let mut nread: u32 = PAGE_SIZE;
    let mut buf = kmap_local_folio(folio, 0);
    let err = vboxsf_read((*sf_handle).root, (*sf_handle).handle, off, &mut nread, buf);
    buf = folio_zero_tail(folio, nread, buf.add(nread as usize));
    kunmap_local(buf);
    folio_end_read(folio, err == 0);
    err
}

unsafe fn vboxsf_get_write_handle(sf_i: *mut vboxsf_inode) -> *mut vboxsf_handle {
    let mut h: *mut vboxsf_handle = core::ptr::null_mut();
    list_for_each_entry!(entry, (*sf_i).handle_list, vboxsf_handle, head, {
        if (*entry).access_flags == SHFL_CF_ACCESS_WRITE || (*entry).access_flags == SHFL_CF_ACCESS_READWRITE {
            kref_get(&mut (*entry).refcount); h = entry; break;
        }
    });
    h
}

unsafe fn vboxsf_writepages(mapping: *mut address_space, wbc: *mut writeback_control) -> i32 {
    let inode = (*mapping).host;
    let sf_i = VBOXSF_I(inode);
    let sf_handle = vboxsf_get_write_handle(sf_i);
    if sf_handle.is_null() { return -EBADF; }
    let mut folio: *mut folio = core::ptr::null_mut();
    let mut error = 0;
    while { folio = writeback_iter(mapping, wbc, folio, &mut error); !folio.is_null() } {
        let off = folio_pos(folio);
        let mut nwrite = folio_size(folio);
        let size = i_size_read(inode);
        if nwrite > size - off { nwrite = (size - off) as u32; }
        let buf = kmap_local_folio(folio, 0);
        error = vboxsf_write((*sf_handle).root, (*sf_handle).handle, off, &mut nwrite, buf);
        kunmap_local(buf); folio_unlock(folio);
    }
    kref_put(&mut (*sf_handle).refcount, vboxsf_handle_release);
    if error == 0 { (*sf_i).force_restat = 1; }
    error
}

unsafe fn vboxsf_write_end(iocb: *const kiocb, mapping: *mut address_space, pos: loff_t,
                           len: u32, copied: u32, folio: *mut folio, _fsdata: *mut core::ffi::c_void) -> u32 {
    let inode = (*mapping).host;
    let sf_handle = (*(*iocb).ki_filp).private_data as *mut vboxsf_handle;
    let from = offset_in_folio(folio, pos);
    let mut nwritten = len;
    if !folio_test_uptodate(folio) && copied < len { folio_zero_range(folio, from + copied as usize, (len - copied) as usize); }
    let buf = kmap(&mut (*folio).page);
    let err = vboxsf_write((*sf_handle).root, (*sf_handle).handle, pos, &mut nwritten, buf.add(from));
    kunmap(&mut (*folio).page);
    if err != 0 { nwritten = 0; } else {
        (*VBOXSF_I(inode)).force_restat = 1;
        if !folio_test_uptodate(folio) && nwritten == folio_size(folio) { folio_mark_uptodate(folio); }
        let new_pos = pos + nwritten as loff_t; if new_pos > (*inode).i_size { i_size_write(inode, new_pos); }
    }
    folio_unlock(folio); folio_put(folio); nwritten
}

unsafe fn vboxsf_get_link(dentry: *mut dentry, inode: *mut inode, done: *mut delayed_call) -> *const i8 {
    if dentry.is_null() { return ERR_PTR(-ECHILD); }
    let sbi = VBOXSF_SBI((*inode).i_sb);
    let path = vboxsf_path_from_dentry(sbi, dentry); if IS_ERR(path) { return ERR_CAST(path); }
    let link = kzalloc(PATH_MAX, GFP_KERNEL); if link.is_null() { __putname(path); return ERR_PTR(-ENOMEM); }
    let err = vboxsf_readlink((*sbi).root, path, PATH_MAX, link); __putname(path);
    if err != 0 { kfree(link as *mut core::ffi::c_void); return ERR_PTR(err); }
    set_delayed_call(done, kfree_link, link); link as *const i8
}

// The remaining address-space and symlink operations retain the source API and are
// declared below using the kernel-provided Rust representations.
pub static vboxsf_reg_aops: address_space_operations = address_space_operations {
    read_folio: Some(vboxsf_read_folio), writepages: Some(vboxsf_writepages),
    dirty_folio: Some(filemap_dirty_folio), write_begin: Some(simple_write_begin),
    write_end: Some(vboxsf_write_end), migrate_folio: Some(filemap_migrate_folio),
};

pub static vboxsf_lnk_iops: inode_operations = inode_operations {
    get_link: Some(vboxsf_get_link), fileattr_get: Some(vboxsf_fileattr_get),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
