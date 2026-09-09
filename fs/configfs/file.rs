// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * file.c - operations for regular (text) files.
 *
 * Based on sysfs:
 *	sysfs is Copyright (C) 2001, 2002, 2003 Patrick Mochel
 *
 * configfs Copyright (C) 2005 Oracle.  All rights reserved.
 */

// Linux kernel dependencies supplied by other translation units.

const SIMPLE_ATTR_SIZE: usize = 4096;

#[repr(C)]
pub union ConfigfsBufferAttribute {
    pub attr: *mut configfs_attribute,
    pub bin_attr: *mut configfs_bin_attribute,
}

#[repr(C)]
pub struct configfs_buffer {
    pub count: usize,
    pub pos: loff_t,
    pub page: *mut c_char,
    pub ops: *const configfs_item_operations,
    pub mutex: mutex,
    pub needs_read_fill: i32,
    pub read_in_progress: bool,
    pub write_in_progress: bool,
    pub bin_buffer: *mut c_char,
    pub bin_buffer_size: i32,
    pub cb_max_size: i32,
    pub item: *mut config_item,
    pub owner: *mut module,
    pub attribute: ConfigfsBufferAttribute,
}

#[inline]
unsafe fn to_frag(file: *mut file) -> *mut configfs_fragment {
    let sd = (*(*file).f_path.dentry).d_fsdata as *mut configfs_dirent;
    (*sd).s_frag
}

unsafe fn fill_read_buffer(file: *mut file, buffer: *mut configfs_buffer) -> i32 {
    let frag = to_frag(file);
    let mut count: ssize_t = -ENOENT;

    if (*buffer).page.is_null() {
        (*buffer).page = kzalloc(PAGE_SIZE, GFP_KERNEL);
    }
    if (*buffer).page.is_null() { return -ENOMEM; }

    down_read(&mut (*frag).frag_sem);
    if !(*frag).frag_dead {
        count = ((*buffer).attribute.attr).as_ref().unwrap().show.unwrap()(
            (*buffer).item, (*buffer).page);
    }
    up_read(&mut (*frag).frag_sem);

    if count < 0 { return count as i32; }
    if WARN_ON_ONCE(count > SIMPLE_ATTR_SIZE as ssize_t) { return -EIO; }
    (*buffer).needs_read_fill = 0;
    (*buffer).count = count as usize;
    0
}

unsafe fn configfs_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> ssize_t {
    let file = (*iocb).ki_filp;
    let buffer = (*file).private_data as *mut configfs_buffer;
    let mut retval: ssize_t = 0;
    mutex_lock(&mut (*buffer).mutex);
    if (*buffer).needs_read_fill != 0 {
        retval = fill_read_buffer(file, buffer) as ssize_t;
        if retval != 0 { mutex_unlock(&mut (*buffer).mutex); return retval; }
    }
    pr_debug!("%s: count = %zd, pos = %lld, buf = %s\n", __func__, iov_iter_count(to), (*iocb).ki_pos, (*buffer).page);
    if (*iocb).ki_pos >= (*buffer).count as loff_t { mutex_unlock(&mut (*buffer).mutex); return 0; }
    retval = copy_to_iter((*buffer).page.offset((*iocb).ki_pos as isize), (*buffer).count - (*iocb).ki_pos as usize, to) as ssize_t;
    (*iocb).ki_pos += retval;
    if retval == 0 { retval = -EFAULT; }
    mutex_unlock(&mut (*buffer).mutex);
    retval
}

unsafe fn configfs_bin_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> ssize_t {
    let file = (*iocb).ki_filp;
    let frag = to_frag(file);
    let buffer = (*file).private_data as *mut configfs_buffer;
    let mut retval: ssize_t = 0;
    let mut len: ssize_t;
    mutex_lock(&mut (*buffer).mutex);
    if (*buffer).write_in_progress { mutex_unlock(&mut (*buffer).mutex); return -ETXTBSY; }
    (*buffer).read_in_progress = true;
    if (*buffer).needs_read_fill != 0 {
        down_read(&mut (*frag).frag_sem);
        len = if !(*frag).frag_dead { ((*buffer).attribute.bin_attr).as_ref().unwrap().read.unwrap()((*buffer).item, core::ptr::null_mut(), 0) } else { -ENOENT };
        up_read(&mut (*frag).frag_sem);
        if len <= 0 { mutex_unlock(&mut (*buffer).mutex); return len; }
        if (*buffer).cb_max_size != 0 && len > (*buffer).cb_max_size as ssize_t { mutex_unlock(&mut (*buffer).mutex); return -EFBIG; }
        (*buffer).bin_buffer = vmalloc(len as usize);
        if (*buffer).bin_buffer.is_null() { mutex_unlock(&mut (*buffer).mutex); return -ENOMEM; }
        (*buffer).bin_buffer_size = len as i32;
        down_read(&mut (*frag).frag_sem);
        len = if !(*frag).frag_dead { ((*buffer).attribute.bin_attr).as_ref().unwrap().read.unwrap()((*buffer).item, (*buffer).bin_buffer, len as usize) } else { -ENOENT };
        up_read(&mut (*frag).frag_sem);
        if len < 0 { vfree((*buffer).bin_buffer); (*buffer).bin_buffer = core::ptr::null_mut(); (*buffer).bin_buffer_size = 0; mutex_unlock(&mut (*buffer).mutex); return len; }
        (*buffer).needs_read_fill = 0;
    }
    if (*iocb).ki_pos >= (*buffer).bin_buffer_size as loff_t { mutex_unlock(&mut (*buffer).mutex); return 0; }
    retval = copy_to_iter((*buffer).bin_buffer.offset((*iocb).ki_pos as isize), (*buffer).bin_buffer_size as usize - (*iocb).ki_pos as usize, to) as ssize_t;
    (*iocb).ki_pos += retval;
    if retval == 0 { retval = -EFAULT; }
    mutex_unlock(&mut (*buffer).mutex);
    retval
}

unsafe fn fill_write_buffer(buffer: *mut configfs_buffer, from: *mut iov_iter) -> i32 {
    if (*buffer).page.is_null() { (*buffer).page = kmalloc(PAGE_SIZE, GFP_KERNEL); }
    if (*buffer).page.is_null() { return -ENOMEM; }
    let copied = copy_from_iter((*buffer).page, SIMPLE_ATTR_SIZE - 1, from) as i32;
    (*buffer).needs_read_fill = 1;
    *(*buffer).page.add(copied as usize) = 0;
    if copied != 0 { copied } else { -EFAULT }
}

unsafe fn flush_write_buffer(file: *mut file, buffer: *mut configfs_buffer, count: usize) -> i32 {
    let frag = to_frag(file);
    let mut res = -ENOENT;
    down_read(&mut (*frag).frag_sem);
    if !(*frag).frag_dead { res = ((*buffer).attribute.attr).as_ref().unwrap().store.unwrap()((*buffer).item, (*buffer).page, count) as i32; }
    up_read(&mut (*frag).frag_sem);
    res
}

unsafe fn configfs_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> ssize_t {
    let file = (*iocb).ki_filp;
    let buffer = (*file).private_data as *mut configfs_buffer;
    mutex_lock(&mut (*buffer).mutex);
    let mut len = fill_write_buffer(buffer, from);
    if len > 0 { len = flush_write_buffer(file, buffer, len as usize); }
    if len > 0 { (*iocb).ki_pos += len as loff_t; }
    mutex_unlock(&mut (*buffer).mutex);
    len as ssize_t
}

unsafe fn configfs_bin_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> ssize_t {
    let file = (*iocb).ki_filp;
    let buffer = (*file).private_data as *mut configfs_buffer;
    mutex_lock(&mut (*buffer).mutex);
    if (*buffer).read_in_progress { mutex_unlock(&mut (*buffer).mutex); return -ETXTBSY; }
    (*buffer).write_in_progress = true;
    let end_offset = (*iocb).ki_pos as usize + iov_iter_count(from);
    if end_offset > (*buffer).bin_buffer_size as usize {
        if (*buffer).cb_max_size != 0 && end_offset > (*buffer).cb_max_size as usize { mutex_unlock(&mut (*buffer).mutex); return -EFBIG; }
        let tbuf = vmalloc(end_offset);
        if tbuf.is_null() { mutex_unlock(&mut (*buffer).mutex); return -ENOMEM; }
        if !(*buffer).bin_buffer.is_null() { memcpy(tbuf, (*buffer).bin_buffer, (*buffer).bin_buffer_size as usize); vfree((*buffer).bin_buffer); }
        memset(tbuf.add((*buffer).bin_buffer_size as usize), 0, end_offset - (*buffer).bin_buffer_size as usize);
        (*buffer).bin_buffer = tbuf as *mut c_char;
        (*buffer).bin_buffer_size = end_offset as i32;
    }
    let len = copy_from_iter((*buffer).bin_buffer.offset((*iocb).ki_pos as isize), (*buffer).bin_buffer_size as usize - (*iocb).ki_pos as usize, from) as ssize_t;
    (*iocb).ki_pos += len;
    mutex_unlock(&mut (*buffer).mutex);
    if len != 0 { len } else { -EFAULT }
}

unsafe fn __configfs_open_file(inode: *mut inode, file: *mut file, type_: i32) -> i32 {
    let frag = to_frag(file);
    let buffer = kzalloc(core::mem::size_of::<configfs_buffer>(), GFP_KERNEL) as *mut configfs_buffer;
    if buffer.is_null() { return -ENOMEM; }
    down_read(&mut (*frag).frag_sem);
    if (*frag).frag_dead { up_read(&mut (*frag).frag_sem); kfree(buffer as *mut c_void); return -ENOENT; }
    (*buffer).item = to_item((*(*file).f_path.dentry).d_parent);
    if (*buffer).item.is_null() { up_read(&mut (*frag).frag_sem); kfree(buffer as *mut c_void); return -ENOENT; }
    let attr = to_attr((*file).f_path.dentry);
    if attr.is_null() { up_read(&mut (*frag).frag_sem); kfree(buffer as *mut c_void); return -ENOENT; }
    if type_ & CONFIGFS_ITEM_BIN_ATTR != 0 {
        (*buffer).attribute.bin_attr = to_bin_attr((*file).f_path.dentry);
        (*buffer).cb_max_size = (*(*buffer).attribute.bin_attr).cb_max_size;
    } else { (*buffer).attribute.attr = attr; }
    (*buffer).owner = (*attr).ca_owner;
    if !try_module_get((*buffer).owner) { up_read(&mut (*frag).frag_sem); kfree(buffer as *mut c_void); return -ENODEV; }
    if (*(*buffer).item).ci_type.is_null() { module_put((*buffer).owner); up_read(&mut (*frag).frag_sem); kfree(buffer as *mut c_void); return -EACCES; }
    (*buffer).ops = (*(*(*buffer).item).ci_type).ct_item_ops;
    if (*file).f_mode & FMODE_WRITE != 0 {
        if (*inode).i_mode & S_IWUGO == 0 || (type_ & CONFIGFS_ITEM_ATTR != 0 && (*attr).store.is_none()) || (type_ & CONFIGFS_ITEM_BIN_ATTR != 0 && (*(*buffer).attribute.bin_attr).write.is_none()) { module_put((*buffer).owner); up_read(&mut (*frag).frag_sem); kfree(buffer as *mut c_void); return -EACCES; }
    }
    if (*file).f_mode & FMODE_READ != 0 {
        if (*inode).i_mode & S_IRUGO == 0 || (type_ & CONFIGFS_ITEM_ATTR != 0 && (*attr).show.is_none()) || (type_ & CONFIGFS_ITEM_BIN_ATTR != 0 && (*(*buffer).attribute.bin_attr).read.is_none()) { module_put((*buffer).owner); up_read(&mut (*frag).frag_sem); kfree(buffer as *mut c_void); return -EACCES; }
    }
    mutex_init(&mut (*buffer).mutex); (*buffer).needs_read_fill = 1; (*buffer).read_in_progress = false; (*buffer).write_in_progress = false;
    (*file).private_data = buffer as *mut c_void;
    up_read(&mut (*frag).frag_sem); 0
}

unsafe fn configfs_release(_inode: *mut inode, filp: *mut file) -> i32 {
    let buffer = (*filp).private_data as *mut configfs_buffer;
    module_put((*buffer).owner); kfree((*buffer).page as *mut c_void); mutex_destroy(&mut (*buffer).mutex); kfree(buffer as *mut c_void); 0
}
unsafe fn configfs_open_file(i: *mut inode, f: *mut file) -> i32 { __configfs_open_file(i, f, CONFIGFS_ITEM_ATTR) }
unsafe fn configfs_open_bin_file(i: *mut inode, f: *mut file) -> i32 { __configfs_open_file(i, f, CONFIGFS_ITEM_BIN_ATTR) }
unsafe fn configfs_release_bin_file(i: *mut inode, f: *mut file) -> i32 {
    let b = (*f).private_data as *mut configfs_buffer;
    if (*b).write_in_progress { let frag = to_frag(f); down_read(&mut (*frag).frag_sem); if !(*frag).frag_dead { ((*b).attribute.bin_attr).as_ref().unwrap().write.unwrap()((*b).item, (*b).bin_buffer, (*b).bin_buffer_size as usize); } up_read(&mut (*frag).frag_sem); }
    vfree((*b).bin_buffer); configfs_release(i, f); 0
}

pub static configfs_file_operations: file_operations = file_operations { read_iter: Some(configfs_read_iter), write_iter: Some(configfs_write_iter), llseek: Some(generic_file_llseek), open: Some(configfs_open_file), release: Some(configfs_release), ..unsafe { core::mem::zeroed() } };
pub static configfs_bin_file_operations: file_operations = file_operations { read_iter: Some(configfs_bin_read_iter), write_iter: Some(configfs_bin_write_iter), llseek: None, open: Some(configfs_open_bin_file), release: Some(configfs_release_bin_file), ..unsafe { core::mem::zeroed() } };

pub unsafe fn configfs_create_file(item: *mut config_item, attr: *const configfs_attribute) -> i32 {
    let dir = (*item).ci_dentry;
    let parent_sd = (*dir).d_fsdata as *mut configfs_dirent;
    let mode = ((*attr).ca_mode & S_IALLUGO) | S_IFREG;
    inode_lock_nested(d_inode(dir), I_MUTEX_NORMAL);
    let error = configfs_make_dirent(parent_sd, core::ptr::null_mut(), attr as *mut c_void, mode, CONFIGFS_ITEM_ATTR, (*parent_sd).s_frag);
    inode_unlock(d_inode(dir));
    error
}

pub unsafe fn configfs_create_bin_file(item: *mut config_item, bin_attr: *const configfs_bin_attribute) -> i32 {
    let dir = (*item).ci_dentry;
    let parent_sd = (*dir).d_fsdata as *mut configfs_dirent;
    let mode = ((*bin_attr).cb_attr.ca_mode & S_IALLUGO) | S_IFREG;
    inode_lock_nested((*dir).d_inode, I_MUTEX_NORMAL);
    let error = configfs_make_dirent(parent_sd, core::ptr::null_mut(), bin_attr as *mut c_void, mode, CONFIGFS_ITEM_BIN_ATTR, (*parent_sd).s_frag);
    inode_unlock((*dir).d_inode);
    error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
