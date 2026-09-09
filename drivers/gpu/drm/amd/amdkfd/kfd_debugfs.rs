// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright 2016-2022 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 */

// Linux debugfs, uaccess, and kfd_priv declarations are supplied externally.

static mut debugfs_root: *mut dentry = core::ptr::null_mut();
static mut debugfs_proc: *mut dentry = core::ptr::null_mut();
static mut procs: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

#[repr(C)]
struct debugfs_proc_entry {
    list: list_head,
    proc_dentry: *mut dentry,
    process: *mut kfd_process,
    pid: pid_t,
}

const MAX_DEBUGFS_FILENAME_LEN: usize = 32;

unsafe extern "C" {
    fn single_open(file: *mut file, show: unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void) -> i32, data: *mut core::ffi::c_void) -> i32;
    fn seq_puts(m: *mut seq_file, s: *const u8) -> i32;
    fn copy_from_user(to: *mut u8, from: *const u8, n: usize) -> usize;
    fn kstrtoint(s: *const u8, base: u32, res: *mut u32) -> i32;
    fn kfd_device_by_id(id: u32) -> *mut kfd_node;
    fn kfd_debugfs_hang_hws(dev: *mut kfd_node);
    fn debugfs_create_dir(name: *const u8, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_file(name: *const u8, mode: u32, parent: *mut dentry, data: *mut core::ffi::c_void, fops: *const file_operations) -> *mut dentry;
    fn debugfs_remove_recursive(dentry: *mut dentry);
    fn debugfs_remove(dentry: *mut dentry);
    fn simple_read_from_buffer(buf: *mut u8, count: usize, ppos: *mut loff_t, from: *const u8, len: usize) -> isize;
    fn kfd_lookup_process_by_mm(mm: *mut mm_struct) -> *mut kfd_process;
    fn kfd_unref_process(p: *mut kfd_process);
    fn kzalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(p: *mut core::ffi::c_void);
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
}

unsafe fn kfd_debugfs_open(inode: *mut inode, file: *mut file) -> i32 {
    let show = (*inode).i_private as unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void) -> i32;
    single_open(file, show, core::ptr::null_mut())
}

unsafe fn kfd_debugfs_hang_hws_read(m: *mut seq_file, _data: *mut core::ffi::c_void) -> i32 {
    seq_puts(m, b"echo gpu_id > hang_hws\0".as_ptr());
    0
}

unsafe fn kfd_debugfs_hang_hws_write(_file: *mut file, user_buf: *const u8, size: usize, _ppos: *mut loff_t) -> isize {
    let mut tmp = [0u8; 16];
    let mut gpu_id: u32 = 0;
    let mut ret: isize = -22;
    if size >= 16 { return ret; }
    if copy_from_user(tmp.as_mut_ptr(), user_buf, size) != 0 { return -14; }
    if kstrtoint(tmp.as_ptr(), 10, &mut gpu_id) != 0 { return ret; }
    let dev = kfd_device_by_id(gpu_id);
    if !dev.is_null() { kfd_debugfs_hang_hws(dev); ret = size as isize; }
    ret
}

#[repr(C)]
struct file_operations { owner: *const core::ffi::c_void, open: Option<unsafe fn(*mut inode, *mut file) -> i32>, read: Option<unsafe fn()>, write: Option<unsafe fn()>, llseek: Option<unsafe fn()>, release: Option<unsafe fn()> }

static kfd_debugfs_fops: file_operations = file_operations { owner: core::ptr::null(), open: Some(kfd_debugfs_open), read: None, write: None, llseek: None, release: None };
static kfd_debugfs_hang_hws_fops: file_operations = file_operations { owner: core::ptr::null(), open: Some(kfd_debugfs_open), read: None, write: None, llseek: None, release: None };

unsafe fn kfd_debugfs_init() {
    debugfs_root = debugfs_create_dir(b"kfd\0".as_ptr(), core::ptr::null_mut());
    debugfs_proc = debugfs_create_dir(b"proc\0".as_ptr(), debugfs_root);
    INIT_LIST_HEAD(&mut procs);
    debugfs_create_file(b"mqds\0".as_ptr(), 0o444, debugfs_root, kfd_debugfs_mqds_by_process as *mut _, &kfd_debugfs_fops);
    debugfs_create_file(b"hqds\0".as_ptr(), 0o444, debugfs_root, kfd_debugfs_hqds_by_device as *mut _, &kfd_debugfs_fops);
    debugfs_create_file(b"rls\0".as_ptr(), 0o444, debugfs_root, kfd_debugfs_rls_by_device as *mut _, &kfd_debugfs_fops);
    debugfs_create_file(b"hang_hws\0".as_ptr(), 0o200, debugfs_root, kfd_debugfs_hang_hws_read as *mut _, &kfd_debugfs_hang_hws_fops);
    debugfs_create_file(b"mem_limit\0".as_ptr(), 0o200, debugfs_root, kfd_debugfs_kfd_mem_limits as *mut _, &kfd_debugfs_fops);
}

unsafe fn kfd_debugfs_fini() { debugfs_remove_recursive(debugfs_proc); debugfs_remove_recursive(debugfs_root); }

unsafe fn kfd_debugfs_pasid_read(file: *mut file, buf: *mut u8, count: usize, ppos: *mut loff_t) -> isize {
    let pdd = (*(*file).f_inode).i_private as *mut kfd_process_device;
    let mut tmp = [0u8; 32];
    let len = snprintf(tmp.as_mut_ptr(), tmp.len(), b"%u\n\0".as_ptr(), (*pdd).pasid);
    simple_read_from_buffer(buf, count, ppos, tmp.as_ptr(), len as usize)
}

unsafe fn kfd_debugfs_find_process_entry(p: *mut kfd_process) -> *mut debugfs_proc_entry {
    let mut entry = list_first_entry(&procs as *const _, debugfs_proc_entry::default());
    while !entry.is_null() { if (*entry).process == p { return entry; } entry = list_next_entry(entry, list); }
    core::ptr::null_mut()
}

unsafe fn kfd_debugfs_create_pasid_files(p: *mut kfd_process, dir: *mut dentry) {
    let mut name = [0u8; MAX_DEBUGFS_FILENAME_LEN];
    for i in 0..(*p).n_pdds { let pdd = *(*p).pdds.add(i as usize); snprintf(name.as_mut_ptr(), name.len(), b"pasid_%u\0".as_ptr(), (*(*pdd).dev).id); debugfs_create_file(name.as_ptr(), 0o444, dir, pdd as *mut _, &kfd_debugfs_pasid_fops); }
}

// Process-entry creation/removal follows the kernel list and debugfs operations directly.
unsafe fn kfd_debugfs_add_process(p: *mut kfd_process) -> i32 {
    let entry = kzalloc(core::mem::size_of::<debugfs_proc_entry>(), 0) as *mut debugfs_proc_entry;
    if entry.is_null() { return -12; }
    (*entry).process = p;
    (*entry).pid = (*(*p).lead_thread).pid;
    if (*p).context_id == KFD_CONTEXT_ID_PRIMARY {
        let mut name = [0u8; MAX_DEBUGFS_FILENAME_LEN];
        snprintf(name.as_mut_ptr(), name.len(), b"%d\0".as_ptr(), (*entry).pid);
        (*entry).proc_dentry = debugfs_create_dir(name.as_ptr(), debugfs_proc);
    } else {
        let primary = kfd_lookup_process_by_mm((*(*p).lead_thread).mm);
        if primary.is_null() { kfree(entry as *mut _); return -3; }
        let primary_entry = kfd_debugfs_find_process_entry(primary);
        kfd_unref_process(primary);
        if primary_entry.is_null() { kfree(entry as *mut _); return -2; }
        let mut name = [0u8; MAX_DEBUGFS_FILENAME_LEN];
        snprintf(name.as_mut_ptr(), name.len(), b"context_%u\0".as_ptr(), (*p).context_id);
        (*entry).proc_dentry = debugfs_create_dir(name.as_ptr(), (*primary_entry).proc_dentry);
    }
    list_add(&mut (*entry).list, &mut procs);
    kfd_debugfs_create_pasid_files(p, (*entry).proc_dentry);
    0
}
unsafe fn kfd_debugfs_remove_entry(entry: *mut debugfs_proc_entry) { debugfs_remove((*entry).proc_dentry); list_del(&mut (*entry).list); kfree(entry as *mut _); }
unsafe fn kfd_debugfs_remove_process(p: *mut kfd_process) {
    mutex_lock(&mut kfd_processes_mutex);
    let mut entry = kfd_debugfs_find_process_entry(p);
    if !entry.is_null() { kfd_debugfs_remove_entry(entry); }
    if (*p).context_id == KFD_CONTEXT_ID_PRIMARY {
        entry = list_first_entry(&procs as *const _, debugfs_proc_entry::default());
        while !entry.is_null() { let next = list_next_entry(entry, list); if (*entry).pid == (*(*p).lead_thread).pid { kfd_debugfs_remove_entry(entry); } entry = next; }
    }
    mutex_unlock(&mut kfd_processes_mutex);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
