// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2018-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* Dependencies are supplied by the surrounding kernel/XFS Rust environment. */

/*
 * Swappable Temporary Memory
 * ==========================
 *
 * Online checking sometimes needs to be able to stage a large amount of data
 * in memory.  This information might not fit in the available memory and it
 * doesn't all need to be accessible at all times.  In other words, we want an
 * indexed data buffer to store data that can be paged out.
 *
 * When CONFIG_TMPFS=y, shmemfs is enough of a filesystem to meet those
 * requirements.  Therefore, the xfile mechanism uses an unlinked shmem file
 * to store our staging data.  This file is not installed in the file
 * descriptor table so that user programs cannot access the data, which means
 * that the xfile must be freed with xfile_destroy.
 *
 * xfiles assume that the caller will handle all required concurrency
 * management; standard vfs locks (freezer and inode) are not taken.  Reads
 * and writes are satisfied directly from the page cache.
 */

/*
 * xfiles must not be exposed to userspace and require upper layers to
 * coordinate access to the one handle returned by the constructor, so
 * establish a separate lock class for xfiles to avoid confusing lockdep.
 */
static mut XFILE_I_MUTEX_KEY: lock_class_key = lock_class_key {};

/* Create an xfile of the given size.  The description will be used in trace output. */
pub unsafe extern "C" fn xfile_create(
    description: *const c_char,
    isize: loff_t,
    xfilep: *mut *mut xfile,
) -> c_int {
    let mut xf: *mut xfile = kmalloc_obj::<xfile>(XCHK_GFP_FLAGS);
    if xf.is_null() {
        return -ENOMEM;
    }

    (*xf).file = shmem_kernel_file_setup(
        description,
        isize,
        mk_vma_flags(VMA_NORESERVE_BIT),
    );
    if IS_ERR((*xf).file) {
        let error = PTR_ERR((*xf).file);
        kfree(xf);
        return error;
    }

    let inode = file_inode((*xf).file);
    lockdep_set_class(&mut (*inode).i_rwsem, &raw mut XFILE_I_MUTEX_KEY);

    /* We don't want to bother with kmapping data during repair, so don't
     * allow highmem pages to back this mapping. */
    mapping_set_gfp_mask((*inode).i_mapping, GFP_KERNEL);

    trace_xfile_create(xf);
    *xfilep = xf;
    0
}

/* Close the file and release all resources. */
pub unsafe extern "C" fn xfile_destroy(xf: *mut xfile) {
    let inode = file_inode((*xf).file);

    trace_xfile_destroy(xf);
    lockdep_set_class(
        &mut (*inode).i_rwsem,
        &raw mut (*(*inode).i_sb).s_type.i_mutex_key,
    );
    fput((*xf).file);
    kfree(xf);
}

/* Load an object.  Since we're treating this file as "memory", any error or
 * short IO is treated as a failure to allocate memory. */
pub unsafe extern "C" fn xfile_load(
    xf: *mut xfile,
    mut buf: *mut c_void,
    mut count: size_t,
    mut pos: loff_t,
) -> c_int {
    let inode = file_inode((*xf).file);
    let mut pflags: c_uint;

    if count > MAX_RW_COUNT || (*(*inode).i_sb).s_maxbytes - pos < count as loff_t {
        return -ENOMEM;
    }
    trace_xfile_load(xf, pos, count);
    pflags = memalloc_nofs_save();
    while count > 0 {
        let mut folio: *mut folio = core::ptr::null_mut();
        let mut len: size_t;
        let offset: size_t;

        if shmem_get_folio(inode, (pos >> PAGE_SHIFT) as pgoff_t, 0, &mut folio, SGP_READ) < 0 {
            break;
        }
        if folio.is_null() {
            len = core::cmp::min(count, (PAGE_SIZE - offset_in_page(pos)) as size_t);
            core::ptr::write_bytes(buf as *mut u8, 0, len);
        } else {
            if filemap_check_wb_err((*inode).i_mapping, 0) != 0 {
                folio_unlock(folio);
                folio_put(folio);
                break;
            }
            offset = offset_in_folio(folio, pos);
            len = core::cmp::min(count, folio_size(folio) - offset);
            core::ptr::copy_nonoverlapping(
                folio_address(folio).add(offset), buf as *mut u8, len,
            );
            folio_unlock(folio);
            folio_put(folio);
        }
        count -= len;
        pos += len as loff_t;
        buf = (buf as *mut u8).add(len) as *mut c_void;
    }
    memalloc_nofs_restore(pflags);
    if count != 0 { -ENOMEM } else { 0 }
}

/* Store an object.  Since we're treating this file as "memory", any error or
 * short IO is treated as a failure to allocate memory. */
pub unsafe extern "C" fn xfile_store(
    xf: *mut xfile,
    mut buf: *const c_void,
    mut count: size_t,
    mut pos: loff_t,
) -> c_int {
    let inode = file_inode((*xf).file);
    if count > MAX_RW_COUNT || (*(*inode).i_sb).s_maxbytes - pos < count as loff_t {
        return -ENOMEM;
    }
    trace_xfile_store(xf, pos, count);
    if pos + count as loff_t > i_size_read(inode) {
        i_size_write(inode, pos + count as loff_t);
    }

    let pflags = memalloc_nofs_save();
    while count > 0 {
        let mut folio: *mut folio = core::ptr::null_mut();
        if shmem_get_folio(inode, (pos >> PAGE_SHIFT) as pgoff_t, 0, &mut folio, SGP_CACHE) < 0 {
            break;
        }
        if filemap_check_wb_err((*inode).i_mapping, 0) != 0 {
            folio_unlock(folio);
            folio_put(folio);
            break;
        }
        let offset = offset_in_folio(folio, pos);
        let len = core::cmp::min(count, folio_size(folio) - offset);
        core::ptr::copy_nonoverlapping(buf as *const u8, folio_address(folio).add(offset), len);
        folio_mark_dirty(folio);
        folio_unlock(folio);
        folio_put(folio);
        count -= len;
        pos += len as loff_t;
        buf = buf.add(len);
    }
    memalloc_nofs_restore(pflags);
    if count != 0 { -ENOMEM } else { 0 }
}

/* Find the next written area in the xfile data for a given offset. */
pub unsafe extern "C" fn xfile_seek_data(xf: *mut xfile, pos: loff_t) -> loff_t {
    let ret = vfs_llseek((*xf).file, pos, SEEK_DATA);
    trace_xfile_seek_data(xf, pos, ret);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
