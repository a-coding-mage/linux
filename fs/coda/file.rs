// SPDX-License-Identifier: GPL-2.0
/*
 * File operations for Coda.
 * Original version: (C) 1996 Peter Braam
 * Rewritten for Linux 2.1: (C) 1997 Carnegie Mellon University
 *
 * Carnegie Mellon encourages users of this code to contribute improvements
 * to the Coda project. Contact Peter Braam <coda@cs.cmu.edu>.
 */

// Linux kernel and Coda headers provide the external types, constants, and functions used here.

#[repr(C)]
pub struct coda_vm_ops {
    pub refcnt: refcount_t,
    pub coda_file: *mut file,
    pub host_vm_ops: *const vm_operations_struct,
    pub vm_ops: vm_operations_struct,
}

unsafe fn coda_file_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> ssize_t {
    let coda_file = (*iocb).ki_filp;
    let coda_inode = file_inode(coda_file);
    let cfi = coda_ftoc(coda_file);
    let ki_pos = (*iocb).ki_pos;
    let count = iov_iter_count(to);
    let mut ret: ssize_t;

    ret = venus_access_intent((*coda_inode).i_sb, coda_i2f(coda_inode),
                              &mut (*cfi).cfi_access_intent,
                              count, ki_pos, CODA_ACCESS_TYPE_READ);
    if ret != 0 {
        return return_finish_read(coda_inode, cfi, count, ki_pos, ret);
    }
    ret = vfs_iter_read((*cfi).cfi_container, to, &mut (*iocb).ki_pos, 0);

    return_finish_read(coda_inode, cfi, count, ki_pos, ret)
}

unsafe fn return_finish_read(coda_inode: *mut inode, cfi: *mut coda_file_info,
                             count: usize, ki_pos: loff_t, ret: ssize_t) -> ssize_t {
    venus_access_intent((*coda_inode).i_sb, coda_i2f(coda_inode),
                        &mut (*cfi).cfi_access_intent,
                        count, ki_pos, CODA_ACCESS_TYPE_READ_FINISH);
    ret
}

unsafe fn coda_file_write_iter(iocb: *mut kiocb, to: *mut iov_iter) -> ssize_t {
    let coda_file = (*iocb).ki_filp;
    let coda_inode = file_inode(coda_file);
    let cfi = coda_ftoc(coda_file);
    let host_file = (*cfi).cfi_container;
    let ki_pos = (*iocb).ki_pos;
    let count = iov_iter_count(to);
    let mut ret = venus_access_intent((*coda_inode).i_sb, coda_i2f(coda_inode),
                                      &mut (*cfi).cfi_access_intent,
                                      count, ki_pos, CODA_ACCESS_TYPE_WRITE);
    if ret == 0 {
        inode_lock(coda_inode);
        ret = vfs_iter_write((*cfi).cfi_container, to, &mut (*iocb).ki_pos, 0);
        (*coda_inode).i_size = (*file_inode(host_file)).i_size;
        (*coda_inode).i_blocks = ((*coda_inode).i_size + 511) >> 9;
        inode_set_mtime_to_ts(coda_inode, inode_set_ctime_current(coda_inode));
        inode_unlock(coda_inode);
    }
    venus_access_intent((*coda_inode).i_sb, coda_i2f(coda_inode),
                        &mut (*cfi).cfi_access_intent,
                        count, ki_pos, CODA_ACCESS_TYPE_WRITE_FINISH);
    ret
}

unsafe fn coda_file_splice_read(coda_file: *mut file, ppos: *mut loff_t,
                                pipe: *mut pipe_inode_info, len: usize,
                                flags: c_uint) -> ssize_t {
    let coda_inode = file_inode(coda_file);
    let cfi = coda_ftoc(coda_file);
    let input = (*cfi).cfi_container;
    let ki_pos = *ppos;
    let mut ret = venus_access_intent((*coda_inode).i_sb, coda_i2f(coda_inode),
                                      &mut (*cfi).cfi_access_intent,
                                      len, ki_pos, CODA_ACCESS_TYPE_READ);
    if ret == 0 {
        ret = vfs_splice_read(input, ppos, pipe, len, flags);
    }
    venus_access_intent((*coda_inode).i_sb, coda_i2f(coda_inode),
                        &mut (*cfi).cfi_access_intent,
                        len, ki_pos, CODA_ACCESS_TYPE_READ_FINISH);
    ret
}

unsafe fn coda_vm_open(vma: *mut vm_area_struct) {
    let cvm_ops = container_of((*vma).vm_ops, coda_vm_ops, vm_ops);
    refcount_inc(&mut (*cvm_ops).refcnt);
    if !(*cvm_ops).host_vm_ops.is_null() {
        if let Some(open) = (*(*cvm_ops).host_vm_ops).open { open(vma); }
    }
}

unsafe fn coda_vm_close(vma: *mut vm_area_struct) {
    let cvm_ops = container_of((*vma).vm_ops, coda_vm_ops, vm_ops);
    if !(*cvm_ops).host_vm_ops.is_null() {
        if let Some(close) = (*(*cvm_ops).host_vm_ops).close { close(vma); }
    }
    if refcount_dec_and_test(&mut (*cvm_ops).refcnt) {
        (*vma).vm_ops = (*cvm_ops).host_vm_ops;
        fput((*cvm_ops).coda_file);
        kfree(cvm_ops as *mut core::ffi::c_void);
    }
}

unsafe fn coda_file_mmap(coda_file: *mut file, vma: *mut vm_area_struct) -> c_int {
    let coda_inode = file_inode(coda_file);
    let cfi = coda_ftoc(coda_file);
    let host_file = (*cfi).cfi_container;
    let host_inode = file_inode(host_file);
    let cii: *mut coda_inode_info;
    let cvm_ops: *mut coda_vm_ops;
    let ppos = ((*vma).vm_pgoff * PAGE_SIZE) as loff_t;
    let count = (*vma).vm_end - (*vma).vm_start;

    if !can_mmap_file(host_file) { return -ENODEV; }
    if coda_file != (*vma).vm_file { return -EIO; }
    let ret = venus_access_intent((*coda_inode).i_sb, coda_i2f(coda_inode),
                                  &mut (*cfi).cfi_access_intent,
                                  count, ppos, CODA_ACCESS_TYPE_MMAP);
    if ret != 0 { return ret; }
    cvm_ops = kmalloc_obj::<coda_vm_ops>();
    if cvm_ops.is_null() { return -ENOMEM; }

    cii = ITOC(coda_inode);
    spin_lock(&mut (*cii).c_lock);
    (*coda_file).f_mapping = (*host_file).f_mapping;
    if (*coda_inode).i_mapping == &mut (*coda_inode).i_data {
        (*coda_inode).i_mapping = (*host_inode).i_mapping;
    } else if (*coda_inode).i_mapping != (*host_inode).i_mapping {
        spin_unlock(&mut (*cii).c_lock);
        kfree(cvm_ops as *mut core::ffi::c_void);
        return -EBUSY;
    }
    (*cii).c_mapcount += 1;
    (*cfi).cfi_mapcount += 1;
    spin_unlock(&mut (*cii).c_lock);

    (*vma).vm_file = get_file(host_file);
    let mut ret = vfs_mmap((*vma).vm_file, vma);
    if ret != 0 {
        fput(coda_file);
        kfree(cvm_ops as *mut core::ffi::c_void);
    } else {
        (*cvm_ops).host_vm_ops = (*vma).vm_ops;
        if !(*vma).vm_ops.is_null() { (*cvm_ops).vm_ops = *(*vma).vm_ops; }
        (*cvm_ops).vm_ops.open = Some(coda_vm_open);
        (*cvm_ops).vm_ops.close = Some(coda_vm_close);
        (*cvm_ops).coda_file = coda_file;
        refcount_set(&mut (*cvm_ops).refcnt, 1);
        (*vma).vm_ops = &mut (*cvm_ops).vm_ops;
    }
    ret
}

pub unsafe fn coda_open(coda_inode: *mut inode, coda_file: *mut file) -> c_int {
    let mut host_file: *mut file = core::ptr::null_mut();
    let flags = (*coda_file).f_flags & !O_EXCL;
    let coda_flags = coda_flags_to_cflags(flags);
    let cfi = kmalloc_obj::<coda_file_info>();
    if cfi.is_null() { return -ENOMEM; }
    let mut error = venus_open((*coda_inode).i_sb, coda_i2f(coda_inode), coda_flags, &mut host_file);
    if host_file.is_null() { error = -EIO; }
    if error != 0 { kfree(cfi as *mut core::ffi::c_void); return error; }
    (*host_file).f_flags |= (*coda_file).f_flags & (O_APPEND | O_SYNC);
    (*cfi).cfi_magic = CODA_MAGIC;
    (*cfi).cfi_mapcount = 0;
    (*cfi).cfi_container = host_file;
    (*cfi).cfi_access_intent = true;
    BUG_ON(!(*coda_file).private_data.is_null());
    (*coda_file).private_data = cfi as *mut core::ffi::c_void;
    0
}

pub unsafe fn coda_release(coda_inode: *mut inode, coda_file: *mut file) -> c_int {
    let flags = (*coda_file).f_flags & !O_EXCL;
    let coda_flags = coda_flags_to_cflags(flags);
    let cfi = coda_ftoc(coda_file);
    venus_close((*coda_inode).i_sb, coda_i2f(coda_inode), coda_flags, (*(*coda_file).f_cred).fsuid);
    let host_inode = file_inode((*cfi).cfi_container);
    let cii = ITOC(coda_inode);
    spin_lock(&mut (*cii).c_lock);
    if (*coda_inode).i_mapping == &mut (*host_inode).i_data {
        (*cii).c_mapcount -= (*cfi).cfi_mapcount;
        if (*cii).c_mapcount == 0 { (*coda_inode).i_mapping = &mut (*coda_inode).i_data; }
    }
    spin_unlock(&mut (*cii).c_lock);
    fput((*cfi).cfi_container);
    kfree((*coda_file).private_data);
    (*coda_file).private_data = core::ptr::null_mut();
    0
}

pub unsafe fn coda_fsync(coda_file: *mut file, start: loff_t, end: loff_t, datasync: c_int) -> c_int {
    let coda_inode = file_inode(coda_file);
    if !((*coda_inode).i_mode & S_IFMT == S_IFREG || (*coda_inode).i_mode & S_IFMT == S_IFDIR || (*coda_inode).i_mode & S_IFMT == S_IFLNK) { return -EINVAL; }
    let mut err = filemap_write_and_wait_range((*coda_inode).i_mapping, start, end);
    if err != 0 { return err; }
    inode_lock(coda_inode);
    let host_file = (*coda_ftoc(coda_file)).cfi_container;
    err = vfs_fsync(host_file, datasync);
    if err == 0 && datasync == 0 { err = venus_fsync((*coda_inode).i_sb, coda_i2f(coda_inode)); }
    inode_unlock(coda_inode);
    err
}

#[no_mangle]
pub static mut coda_file_operations: file_operations = file_operations {
    llseek: Some(generic_file_llseek),
    read_iter: Some(coda_file_read_iter),
    write_iter: Some(coda_file_write_iter),
    mmap: Some(coda_file_mmap),
    open: Some(coda_open),
    release: Some(coda_release),
    fsync: Some(coda_fsync),
    splice_read: Some(coda_file_splice_read),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
