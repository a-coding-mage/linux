// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * eCryptfs: Linux filesystem encryption layer
 * This is where eCryptfs coordinates the symmetric encryption and
 * decryption of the file data as it passes between the lower
 * encrypted file and the upper decrypted file.
 */

/* Linux kernel dependencies are supplied by the surrounding translation. */

unsafe fn ecryptfs_writepages(mapping: *mut address_space, wbc: *mut writeback_control) -> i32 {
    let mut folio: *mut folio = core::ptr::null_mut();
    let mut error: i32 = 0;
    while {
        folio = writeback_iter(mapping, wbc, folio, &mut error);
        !folio.is_null()
    } {
        error = ecryptfs_encrypt_page(folio);
        if error != 0 {
            ecryptfs_printk(KERN_WARNING, c"Error encrypting folio (index [0x%.16lx])\n", (*folio).index);
            folio_clear_uptodate(folio);
            mapping_set_error(mapping, error);
        }
        folio_unlock(folio);
    }
    error
}

unsafe fn strip_xattr_flag(page_virt: *mut i8, crypt_stat: *mut ecryptfs_crypt_stat) {
    if (*crypt_stat).flags & ECRYPTFS_METADATA_IN_XATTR != 0 {
        let mut written: usize = 0;
        (*crypt_stat).flags &= !ECRYPTFS_METADATA_IN_XATTR;
        ecryptfs_write_crypt_stat_flags(page_virt, crypt_stat, &mut written);
        (*crypt_stat).flags |= ECRYPTFS_METADATA_IN_XATTR;
    }
}

unsafe fn ecryptfs_copy_up_encrypted_with_header(folio: *mut folio, crypt_stat: *mut ecryptfs_crypt_stat) -> i32 {
    let mut extent_num_in_page: i64 = 0;
    let num_extents_per_page: i64 = (PAGE_SIZE / (*crypt_stat).extent_size) as i64;
    let mut rc: i32 = 0;
    while extent_num_in_page < num_extents_per_page {
        let view_extent_num = ((*folio).index as i64 * num_extents_per_page) + extent_num_in_page;
        let num_header_extents_at_front = ((*crypt_stat).metadata_size / (*crypt_stat).extent_size) as usize;
        if view_extent_num < num_header_extents_at_front as i64 {
            let page_virt = kmap_local_folio(folio, 0);
            core::ptr::write_bytes(page_virt, 0, PAGE_SIZE);
            if view_extent_num == 0 {
                let mut written: usize = 0;
                rc = ecryptfs_read_xattr_region(page_virt, (*folio).mapping.host);
                strip_xattr_flag(page_virt.add(16), crypt_stat);
                ecryptfs_write_header_metadata(page_virt.add(20), crypt_stat, &mut written);
            }
            kunmap_local(page_virt);
            flush_dcache_folio(folio);
            if rc != 0 { goto_out!(); }
        } else {
            let lower_offset = (view_extent_num * (*crypt_stat).extent_size as i64) - (*crypt_stat).metadata_size as i64;
            rc = ecryptfs_read_lower_page_segment(folio, (lower_offset >> PAGE_SHIFT) as u64,
                (lower_offset & !(PAGE_MASK as i64)) as usize, (*crypt_stat).extent_size,
                (*folio).mapping.host);
            if rc != 0 { goto_out!(); }
        }
        extent_num_in_page += 1;
    }
    return rc;
    goto_out!();
    rc
}

unsafe fn ecryptfs_read_folio(_file: *mut file, folio: *mut folio) -> i32 {
    let inode = (*folio).mapping.host;
    let crypt_stat = &mut ecryptfs_inode_to_private(inode).crypt_stat;
    let mut err = 0;
    if crypt_stat.is_null() || crypt_stat.flags & ECRYPTFS_ENCRYPTED == 0 {
        err = ecryptfs_read_lower_page_segment(folio, (*folio).index, 0, folio_size(folio), inode);
    } else if crypt_stat.flags & ECRYPTFS_VIEW_AS_ENCRYPTED != 0 {
        if crypt_stat.flags & ECRYPTFS_METADATA_IN_XATTR != 0 {
            err = ecryptfs_copy_up_encrypted_with_header(folio, crypt_stat);
        } else {
            err = ecryptfs_read_lower_page_segment(folio, (*folio).index, 0, folio_size(folio), inode);
        }
    } else {
        err = ecryptfs_decrypt_page(folio);
    }
    folio_end_read(folio, err == 0);
    err
}

unsafe fn fill_zeros_to_end_of_page(folio: *mut folio, mut to: u32) -> i32 {
    let inode = (*folio).mapping.host;
    if i_size_read(inode) / PAGE_SIZE as i64 != (*folio).index as i64 {
        return 0;
    }
    let mut end_byte_in_page = (i_size_read(inode) % PAGE_SIZE as i64) as u32;
    if to > end_byte_in_page { end_byte_in_page = to; }
    folio_zero_segment(folio, end_byte_in_page, PAGE_SIZE as u32);
    0
}

unsafe fn ecryptfs_write_begin(iocb: *const kiocb, mapping: *mut address_space, pos: i64, len: u32,
    foliop: *mut *mut folio, _fsdata: *mut *mut core::ffi::c_void) -> i32 {
    let index = (pos >> PAGE_SHIFT) as u64;
    let folio = __filemap_get_folio(mapping, index, FGP_WRITEBEGIN, mapping_gfp_mask(mapping));
    if IS_ERR(folio) { return PTR_ERR(folio); }
    *foliop = folio;
    let prev_page_end_size = (index as i64) << PAGE_SHIFT;
    if !folio_test_uptodate(folio) {
        let crypt_stat = &mut ecryptfs_inode_to_private((*mapping).host).crypt_stat;
        if crypt_stat.flags & ECRYPTFS_ENCRYPTED == 0 {
            if ecryptfs_read_lower_page_segment(folio, index, 0, PAGE_SIZE, (*mapping).host) != 0 { folio_clear_uptodate(folio); }
            else { folio_mark_uptodate(folio); }
        } else if crypt_stat.flags & ECRYPTFS_VIEW_AS_ENCRYPTED != 0 {
            let rc = if crypt_stat.flags & ECRYPTFS_METADATA_IN_XATTR != 0 {
                ecryptfs_copy_up_encrypted_with_header(folio, crypt_stat)
            } else { ecryptfs_read_lower_page_segment(folio, index, 0, PAGE_SIZE, (*mapping).host) };
            if rc != 0 { folio_clear_uptodate(folio); } else { folio_mark_uptodate(folio); }
        } else if prev_page_end_size >= i_size_read((*mapping).host) {
            folio_zero_range(folio, 0, PAGE_SIZE); folio_mark_uptodate(folio);
        } else if len < PAGE_SIZE as u32 {
            if ecryptfs_decrypt_page(folio) != 0 { folio_clear_uptodate(folio); }
            else { folio_mark_uptodate(folio); }
        }
    }
    if index != 0 && prev_page_end_size > i_size_read((*mapping).host) {
        let rc = ecryptfs_truncate((*iocb).ki_filp.f_path.dentry, prev_page_end_size);
        if rc != 0 { folio_unlock(folio); folio_put(folio); return rc; }
    }
    if i_size_read((*mapping).host) == prev_page_end_size && pos != 0 { folio_zero_range(folio, 0, PAGE_SIZE); }
    0
}

unsafe fn ecryptfs_write_inode_size_to_header(inode: *mut inode) -> i32 {
    let file_size = cpu_to_be64(i_size_read(inode));
    let mut rc = ecryptfs_write_lower(inode, &file_size as *const _ as *mut i8, 0, core::mem::size_of_val(&file_size));
    if rc >= 0 { rc = 0; }
    rc
}

static mut ecryptfs_xattr_cache: *mut kmem_cache = core::ptr::null_mut();

unsafe fn ecryptfs_write_inode_size_to_xattr(inode: *mut inode) -> i32 {
    let lower_dentry = ecryptfs_inode_to_private(inode).lower_file.f_path.dentry;
    let lower_inode = d_inode(lower_dentry);
    if (*lower_inode).i_opflags & IOP_XATTR == 0 { return -ENOSYS; }
    let xattr_virt = kmem_cache_alloc(ecryptfs_xattr_cache, GFP_KERNEL);
    if xattr_virt.is_null() { return -ENOMEM; }
    inode_lock(lower_inode);
    let mut size = __vfs_getxattr(lower_dentry, lower_inode, ECRYPTFS_XATTR_NAME, xattr_virt, PAGE_SIZE);
    if size < 0 { size = 8; }
    put_unaligned_be64(i_size_read(inode), xattr_virt);
    let rc = __vfs_setxattr(&nop_mnt_idmap, lower_dentry, lower_inode, ECRYPTFS_XATTR_NAME, xattr_virt, size, 0);
    inode_unlock(lower_inode);
    kmem_cache_free(ecryptfs_xattr_cache, xattr_virt);
    rc
}

pub unsafe fn ecryptfs_write_inode_size_to_metadata(inode: *mut inode) -> i32 {
    let crypt_stat = &mut ecryptfs_inode_to_private(inode).crypt_stat;
    BUG_ON(crypt_stat.flags & ECRYPTFS_ENCRYPTED == 0);
    if crypt_stat.flags & ECRYPTFS_METADATA_IN_XATTR != 0 { ecryptfs_write_inode_size_to_xattr(inode) }
    else { ecryptfs_write_inode_size_to_header(inode) }
}

unsafe fn ecryptfs_write_end(_iocb: *const kiocb, mapping: *mut address_space, pos: i64, _len: u32,
    copied: u32, folio: *mut folio, _fsdata: *mut core::ffi::c_void) -> i32 {
    let index = (pos >> PAGE_SHIFT) as u64;
    let from = (pos & (PAGE_SIZE as i64 - 1)) as u32;
    let to = from + copied;
    let inode = (*mapping).host;
    let crypt_stat = &mut ecryptfs_inode_to_private(inode).crypt_stat;
    if crypt_stat.flags & ECRYPTFS_ENCRYPTED == 0 {
        let mut rc = ecryptfs_write_lower_page_segment(inode, folio, 0, to);
        if rc == 0 { fsstack_copy_inode_size(inode, ecryptfs_inode_to_lower(inode)); rc = copied as i32; }
        folio_unlock(folio); folio_put(folio); return rc;
    }
    if !folio_test_uptodate(folio) {
        if copied < PAGE_SIZE as u32 { folio_unlock(folio); folio_put(folio); return 0; }
        folio_mark_uptodate(folio);
    }
    let mut rc = fill_zeros_to_end_of_page(folio, to);
    if rc == 0 { rc = ecryptfs_encrypt_page(folio); }
    if rc == 0 {
        if pos + copied as i64 > i_size_read(inode) { i_size_write(inode, pos + copied as i64); }
        rc = ecryptfs_write_inode_size_to_metadata(inode);
        if rc == 0 { rc = copied as i32; }
    }
    folio_unlock(folio); folio_put(folio); rc
}

unsafe fn ecryptfs_bmap(mapping: *mut address_space, mut block: sector_t) -> sector_t {
    let lower_inode = ecryptfs_inode_to_lower((*mapping).host);
    if bmap(lower_inode, &mut block) != 0 { 0 } else { block }
}

pub static ecryptfs_aops: address_space_operations = address_space_operations {
    dirty_folio: Some(filemap_dirty_folio),
    writepages: Some(ecryptfs_writepages),
    read_folio: Some(ecryptfs_read_folio),
    write_begin: Some(ecryptfs_write_begin),
    write_end: Some(ecryptfs_write_end),
    migrate_folio: Some(filemap_migrate_folio),
    bmap: Some(ecryptfs_bmap),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
