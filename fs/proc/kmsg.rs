// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/proc/kmsg.c
 *
 *  Copyright (C) 1992  by Linus Torvalds
 *
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    pub f_flags: i32,
}

#[repr(C)]
pub struct poll_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct proc_ops {
    pub proc_flags: u32,
    pub proc_read: Option<unsafe extern "C" fn(*mut file, *mut u8, usize, *mut i64) -> isize>,
    pub proc_poll: Option<unsafe extern "C" fn(*mut file, *mut poll_table) -> u32>,
    pub proc_open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> i32>,
    pub proc_release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> i32>,
    pub proc_lseek: Option<unsafe extern "C" fn(*mut file, i64, i32) -> i64>,
}

extern "C" {
    static mut log_wait: wait_queue_head_t;

    fn do_syslog(action: i32, buf: *mut u8, len: usize, source: i32) -> isize;
    fn poll_wait(file: *mut file, wait_address: *mut wait_queue_head_t, wait: *mut poll_table);
    fn generic_file_llseek(file: *mut file, offset: i64, whence: i32) -> i64;
    fn proc_create(name: *const u8, mode: u32, parent: *mut inode, ops: *const proc_ops) -> *mut u8;
}

// Values supplied by the Linux kernel headers.
extern "C" {
    static SYSLOG_ACTION_OPEN: i32;
    static SYSLOG_ACTION_CLOSE: i32;
    static SYSLOG_ACTION_SIZE_UNREAD: i32;
    static SYSLOG_ACTION_READ: i32;
    static SYSLOG_FROM_PROC: i32;
    static O_NONBLOCK: i32;
    static EAGAIN: i32;
    static EPOLLIN: u32;
    static EPOLLRDNORM: u32;
    static PROC_ENTRY_PERMANENT: u32;
    static S_IRUSR: u32;
}

unsafe extern "C" fn kmsg_open(_inode: *mut inode, _file: *mut file) -> i32 {
    do_syslog(SYSLOG_ACTION_OPEN, core::ptr::null_mut(), 0, SYSLOG_FROM_PROC) as i32
}

unsafe extern "C" fn kmsg_release(_inode: *mut inode, _file: *mut file) -> i32 {
    let _ = do_syslog(SYSLOG_ACTION_CLOSE, core::ptr::null_mut(), 0, SYSLOG_FROM_PROC);
    0
}

unsafe extern "C" fn kmsg_read(file: *mut file, buf: *mut u8, count: usize, _ppos: *mut i64) -> isize {
    if ((*file).f_flags & O_NONBLOCK) != 0
        && do_syslog(SYSLOG_ACTION_SIZE_UNREAD, core::ptr::null_mut(), 0, SYSLOG_FROM_PROC) == 0
    {
        return -(EAGAIN as isize);
    }
    do_syslog(SYSLOG_ACTION_READ, buf, count, SYSLOG_FROM_PROC)
}

unsafe extern "C" fn kmsg_poll(file: *mut file, wait: *mut poll_table) -> u32 {
    poll_wait(file, &raw mut log_wait, wait);
    if do_syslog(SYSLOG_ACTION_SIZE_UNREAD, core::ptr::null_mut(), 0, SYSLOG_FROM_PROC) != 0 {
        return EPOLLIN | EPOLLRDNORM;
    }
    0
}

static kmsg_proc_ops: proc_ops = proc_ops {
    proc_flags: PROC_ENTRY_PERMANENT,
    proc_read: Some(kmsg_read),
    proc_poll: Some(kmsg_poll),
    proc_open: Some(kmsg_open),
    proc_release: Some(kmsg_release),
    proc_lseek: Some(generic_file_llseek),
};

unsafe extern "C" fn proc_kmsg_init() -> i32 {
    proc_create(b"kmsg\0".as_ptr(), S_IRUSR, core::ptr::null_mut(), &kmsg_proc_ops);
    0
}

// fs_initcall(proc_kmsg_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
