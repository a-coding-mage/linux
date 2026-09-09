// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * mmap.c
 *
 * Code to deal with the mess that is clustered mmap.
 *
 * Copyright (C) 2002, 2004 Oracle.  All rights reserved.
 */

// Linux kernel and OCFS2 dependencies are supplied by the surrounding crate.

unsafe fn ocfs2_fault(vmf: *mut vm_fault) -> vm_fault_t {
    let ip_blkno: u64 = (*OCFS2_I(file_inode((*(*vmf).vma).vm_file))).ip_blkno;
    let mut oldset: sigset_t = core::mem::zeroed();
    let ret: vm_fault_t;

    ocfs2_block_signals(&mut oldset);
    ret = filemap_fault(vmf);
    ocfs2_unblock_signals(&mut oldset);

    trace_ocfs2_fault(ip_blkno, (*vmf).page, (*vmf).pgoff);
    ret
}

unsafe fn __ocfs2_page_mkwrite(
    file: *mut file,
    di_bh: *mut buffer_head,
    folio: *mut folio,
) -> vm_fault_t {
    let mut err: i32;
    let mut ret: vm_fault_t = VM_FAULT_NOPAGE;
    let inode: *mut inode = file_inode(file);
    let mapping: *mut address_space = (*inode).i_mapping;
    let pos: loff_t = folio_pos(folio);
    let mut len: u32 = PAGE_SIZE;
    let last_index: pgoff_t;
    let mut locked_folio: *mut folio = core::ptr::null_mut();
    let mut fsdata: *mut core::ffi::c_void = core::ptr::null_mut();
    let size: loff_t = i_size_read(inode);

    last_index = ((size - 1) >> PAGE_SHIFT) as pgoff_t;

    /*
     * There are cases that lead to the page no longer belonging to the
     * mapping.
     * 1) pagecache truncates locally due to memory pressure.
     * 2) pagecache truncates when another is taking EX lock against
     * inode lock. see ocfs2_data_convert_worker.
     *
     * The i_size check doesn't catch the case where nodes truncated and
     * then re-extended the file. We'll re-check the page mapping after
     * taking the page lock inside of ocfs2_write_begin_nolock().
     *
     * Let VM retry with these cases.
     */
    if ((*folio).mapping != (*inode).i_mapping
        || !folio_test_uptodate(folio)
        || pos >= size
    ) {
        return ret;
    }

    /*
     * Call ocfs2_write_begin() and ocfs2_write_end() to take
     * advantage of the allocation code there. We pass a write
     * length of the whole page (chopped to i_size) to make sure
     * the whole thing is allocated.
     *
     * Since we know the page is up to date, we don't have to
     * worry about ocfs2_write_begin() skipping some buffer reads
     * because the "write" would invalidate their data.
     */
    if (*folio).index == last_index {
        len = (((size - 1) & !(PAGE_MASK as loff_t)) + 1) as u32;
    }

    err = ocfs2_write_begin_nolock(
        mapping, pos, len, OCFS2_WRITE_MMAP,
        &mut locked_folio, &mut fsdata, di_bh, folio,
    );
    if err != 0 {
        if err != -ENOSPC {
            mlog_errno(err);
        }
        ret = vmf_error(err);
        return ret;
    }

    if locked_folio.is_null() {
        return VM_FAULT_NOPAGE;
    }
    err = ocfs2_write_end_nolock(mapping, pos, len, len, fsdata);
    BUG_ON(err != len as i32);
    VM_FAULT_LOCKED
}

unsafe fn ocfs2_page_mkwrite(vmf: *mut vm_fault) -> vm_fault_t {
    let folio: *mut folio = page_folio((*vmf).page);
    let inode: *mut inode = file_inode((*(*vmf).vma).vm_file);
    let mut di_bh: *mut buffer_head = core::ptr::null_mut();
    let mut oldset: sigset_t = core::mem::zeroed();
    let mut err: i32;
    let ret: vm_fault_t;

    sb_start_pagefault((*inode).i_sb);
    ocfs2_block_signals(&mut oldset);

    /*
     * The cluster locks taken will block a truncate from another
     * node. Taking the data lock will also ensure that we don't
     * attempt page truncation as part of a downconvert.
     */
    err = ocfs2_inode_lock(inode, &mut di_bh, 1);
    if err < 0 {
        mlog_errno(err);
        ret = vmf_error(err);
    } else {
        /*
         * The alloc sem should be enough to serialize with
         * ocfs2_truncate_file() changing i_size as well as any thread
         * modifying the inode btree.
         */
        down_write(&mut (*OCFS2_I(inode)).ip_alloc_sem);
        ret = __ocfs2_page_mkwrite((*(*vmf).vma).vm_file, di_bh, folio);
        up_write(&mut (*OCFS2_I(inode)).ip_alloc_sem);
        brelse(di_bh);
        ocfs2_inode_unlock(inode, 1);
    }

    ocfs2_unblock_signals(&mut oldset);
    sb_end_pagefault((*inode).i_sb);
    ret
}

#[repr(C)]
static ocfs2_file_vm_ops: vm_operations_struct = vm_operations_struct {
    fault: Some(ocfs2_fault),
    page_mkwrite: Some(ocfs2_page_mkwrite),
};

pub unsafe fn ocfs2_mmap_prepare(desc: *mut vm_area_desc) -> i32 {
    let file: *mut file = (*desc).file;
    let mut ret: i32 = 0;
    let mut lock_level: i32 = 0;

    ret = ocfs2_inode_lock_atime(
        file_inode(file), (*file).f_path.mnt, &mut lock_level, 1,
    );
    if ret < 0 {
        mlog_errno(ret);
    } else {
        ocfs2_inode_unlock(file_inode(file), lock_level);
    }
    (*desc).vm_ops = &ocfs2_file_vm_ops;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
