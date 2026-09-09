// SPDX-License-Identifier: GPL-2.0-or-later
/* file-nommu.c: no-MMU version of ramfs
 *
 * Copyright (C) 2005 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Kernel dependencies supplied by the surrounding translation unit/build.

extern "C" {
    fn generic_file_read_iter(file: *mut file, iocb: *mut kiocb, iter: *mut iov_iter) -> ssize_t;
    fn generic_file_write_iter(file: *mut file, iocb: *mut kiocb, iter: *mut iov_iter) -> ssize_t;
    fn noop_fsync(file: *mut file, start: loff_t, end: loff_t, datasync: c_int) -> c_int;
    fn filemap_splice_read(pipe: *mut pipe_inode_info, file: *mut file, ppos: *mut loff_t, len: usize, flags: u32) -> ssize_t;
    fn iter_file_splice_write(pipe: *mut pipe_inode_info, file: *mut file, ppos: *mut loff_t, len: usize, flags: u32) -> ssize_t;
    fn generic_file_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t;
}

unsafe extern "C" fn ramfs_mmap_capabilities(_file: *mut file) -> c_uint {
    NOMMU_MAP_DIRECT | NOMMU_MAP_COPY | NOMMU_MAP_READ | NOMMU_MAP_WRITE | NOMMU_MAP_EXEC
}

#[repr(C)]
pub struct file_operations {
    pub mmap_capabilities: Option<unsafe extern "C" fn(*mut file) -> c_uint>,
    pub mmap_prepare: Option<unsafe extern "C" fn(*mut vm_area_desc) -> c_int>,
    pub get_unmapped_area: Option<unsafe extern "C" fn(*mut file, ulong, ulong, ulong, ulong) -> ulong>,
    pub read_iter: Option<unsafe extern "C" fn(*mut file, *mut kiocb, *mut iov_iter) -> ssize_t>,
    pub write_iter: Option<unsafe extern "C" fn(*mut file, *mut kiocb, *mut iov_iter) -> ssize_t>,
    pub fsync: Option<unsafe extern "C" fn(*mut file, loff_t, loff_t, c_int) -> c_int>,
    pub splice_read: Option<unsafe extern "C" fn(*mut pipe_inode_info, *mut file, *mut loff_t, usize, u32) -> ssize_t>,
    pub splice_write: Option<unsafe extern "C" fn(*mut pipe_inode_info, *mut file, *mut loff_t, usize, u32) -> ssize_t>,
    pub llseek: Option<unsafe extern "C" fn(*mut file, loff_t, c_int) -> loff_t>,
}

pub static mut ramfs_file_operations: file_operations = file_operations {
    mmap_capabilities: Some(ramfs_mmap_capabilities),
    mmap_prepare: Some(ramfs_nommu_mmap_prepare),
    get_unmapped_area: Some(ramfs_nommu_get_unmapped_area),
    read_iter: Some(generic_file_read_iter),
    write_iter: Some(generic_file_write_iter),
    fsync: Some(noop_fsync),
    splice_read: Some(filemap_splice_read),
    splice_write: Some(iter_file_splice_write),
    llseek: Some(generic_file_llseek),
};

#[repr(C)]
pub struct inode_operations {
    pub setattr: Option<unsafe extern "C" fn(*mut mnt_idmap, *mut dentry, *mut iattr) -> c_int>,
    pub getattr: Option<unsafe extern "C" fn(*mut mnt_idmap, *mut dentry, *mut kstat, u32, u32) -> c_int>,
}

pub static mut ramfs_file_inode_operations: inode_operations = inode_operations {
    setattr: Some(ramfs_nommu_setattr),
    getattr: Some(simple_getattr),
};

/*****************************************************************************/
/*
 * add a contiguous set of pages into a ramfs inode when it's truncated from
 * size 0 on the assumption that it's going to be used for an mmap of shared
 * memory
 */
pub unsafe extern "C" fn ramfs_nommu_expand_for_mapping(inode: *mut inode, mut newsize: usize) -> c_int {
    let mut npages: ulong;
    let mut xpages: ulong;
    let mut loop_: ulong;
    let mut pages: *mut page;
    let order: c_uint;
    let data: *mut c_void;
    let ret: c_int;
    let gfp: gfp_t = mapping_gfp_mask((*inode).i_mapping);

    if newsize == 0 { return 0; }
    order = get_order(newsize);
    if order > MAX_PAGE_ORDER { return -EFBIG; }
    ret = inode_newsize_ok(inode, newsize as loff_t);
    if ret != 0 { return ret; }
    i_size_write(inode, newsize as loff_t);
    pages = alloc_pages(gfp, order);
    if pages.is_null() { return -ENOMEM; }
    xpages = 1UL << order;
    npages = ((newsize + PAGE_SIZE - 1) >> PAGE_SHIFT) as ulong;
    split_page(pages, order);
    loop_ = npages;
    while loop_ < xpages { __free_page(pages.add(loop_ as usize)); loop_ += 1; }
    newsize = PAGE_SIZE * npages as usize;
    data = page_address(pages);
    memset(data, 0, newsize);
    loop_ = 0;
    while loop_ < npages {
        let page = pages.add(loop_ as usize);
        ret = add_to_page_cache_lru(page, (*inode).i_mapping, loop_ as pgoff_t, gfp);
        if ret < 0 {
            while loop_ < npages { __free_page(pages.add(loop_ as usize)); loop_ += 1; }
            return ret;
        }
        SetPageDirty(page);
        SetPageUptodate(page);
        unlock_page(page);
        put_page(page);
        loop_ += 1;
    }
    0
}

/*****************************************************************************/
/*
 *
 */
unsafe extern "C" fn ramfs_nommu_resize(inode: *mut inode, newsize: loff_t, size: loff_t) -> c_int {
    if size == 0 {
        if (newsize >> 32) != 0 { return -EFBIG; }
        return ramfs_nommu_expand_for_mapping(inode, newsize as usize);
    }
    if newsize < size {
        let ret = nommu_shrink_inode_mappings(inode, size, newsize);
        if ret < 0 { return ret; }
    }
    truncate_setsize(inode, newsize);
    0
}

/*****************************************************************************/
/*
 * handle a change of attributes
 * - we're specifically interested in a change of size
 */
unsafe extern "C" fn ramfs_nommu_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry, ia: *mut iattr) -> c_int {
    let inode = d_inode(dentry);
    let old_ia_valid = (*ia).ia_valid;
    let mut ret = setattr_prepare(&nop_mnt_idmap, dentry, ia);
    if ret != 0 { return ret; }
    if ((*ia).ia_valid & ATTR_SIZE) != 0 {
        let size = (*inode).i_size;
        if (*ia).ia_size != size {
            ret = ramfs_nommu_resize(inode, (*ia).ia_size, size);
            if ret < 0 || (*ia).ia_valid == ATTR_SIZE { (*ia).ia_valid = old_ia_valid; return ret; }
        } else { (*ia).ia_valid |= ATTR_MTIME | ATTR_CTIME; }
    }
    setattr_copy(&nop_mnt_idmap, inode, ia);
    (*ia).ia_valid = old_ia_valid;
    ret
}

/*****************************************************************************/
/*
 * try to determine where a shared mapping can be made
 * - we require that:
 *   - the pages to be mapped must exist
 *   - the pages be physically contiguous in sequence
 */
unsafe extern "C" fn ramfs_nommu_get_unmapped_area(file: *mut file, _addr: ulong, len: ulong, mut pgoff: ulong, _flags: ulong) -> ulong {
    let inode = file_inode(file);
    let mut fbatch: folio_batch = core::mem::zeroed();
    let isize = i_size_read(inode);
    let lpages = (len + PAGE_SIZE - 1) >> PAGE_SHIFT;
    let maxpages = (isize as ulong + PAGE_SIZE - 1) >> PAGE_SHIFT;
    let mut ret: ulong = -ENOSYS as ulong;
    if pgoff >= maxpages || maxpages - pgoff < lpages { return ret; }
    folio_batch_init(&mut fbatch);
    let mut nr_pages: ulong = 0;
    loop {
        let nr_folios = filemap_get_folios_contig((*inode).i_mapping, &mut pgoff, ULONG_MAX, &mut fbatch);
        if nr_folios == 0 { return -ENOSYS as ulong; }
        let mut pfn: ulong = 0;
        if ret == -ENOSYS as ulong { ret = folio_address(fbatch.folios[0]) as ulong; pfn = folio_pfn(fbatch.folios[0]); }
        let mut loop_: ulong = 0;
        while loop_ < nr_folios {
            if pfn + nr_pages != folio_pfn(fbatch.folios[loop_ as usize]) { ret = -ENOSYS as ulong; folio_batch_release(&mut fbatch); return ret; }
            nr_pages += folio_nr_pages(fbatch.folios[loop_ as usize]);
            if nr_pages >= lpages { folio_batch_release(&mut fbatch); return ret; }
            loop_ += 1;
        }
        if nr_pages < lpages { folio_batch_release(&mut fbatch); continue; }
        folio_batch_release(&mut fbatch);
        return ret;
    }
}

/*****************************************************************************/
/*
 * set up a mapping for shared memory segments
 */
unsafe extern "C" fn ramfs_nommu_mmap_prepare(desc: *mut vm_area_desc) -> c_int {
    if !is_nommu_shared_vma_flags((*desc).vma_flags) { return -ENOSYS; }
    file_accessed((*desc).file);
    (*desc).vm_ops = &generic_file_vm_ops;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
