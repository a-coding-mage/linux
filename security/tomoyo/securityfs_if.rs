// SPDX-License-Identifier: GPL-2.0
/*
 * security/tomoyo/securityfs_if.c
 *
 * Copyright (C) 2005-2011  NTT DATA CORPORATION
 */

use core::ffi::{c_char, c_int, c_void};

// Dependencies supplied by the surrounding kernel/TOMOYO translation.
extern "C" {
    static mut tomoyo_enabled: bool;
    static mut tomoyo_kernel_domain: tomoyo_domain_info;
    static current: *mut task_struct;

    fn tomoyo_pathcmp(a: *const tomoyo_path_info, b: *const tomoyo_path_info) -> bool;
    fn memdup_user_nul(buf: *const c_char, count: usize) -> *mut c_char;
    fn tomoyo_normalize_line(data: *mut c_char);
    fn tomoyo_correct_domain(data: *const c_char) -> bool;
    fn tomoyo_read_lock() -> c_int;
    fn tomoyo_fill_path_info(name: *mut tomoyo_path_info);
    fn tomoyo_init_request_info(r: *mut tomoyo_request_info, param: *mut c_void, operation: c_int);
    fn tomoyo_check_acl(
        r: *mut tomoyo_request_info,
        check: Option<unsafe extern "C" fn(*mut tomoyo_request_info, *const tomoyo_acl_info) -> bool>,
    );
    fn tomoyo_assign_domain(data: *const c_char, create: bool) -> *mut tomoyo_domain_info;
    fn tomoyo_task(task: *mut task_struct) -> *mut tomoyo_task;
    fn tomoyo_read_unlock(idx: c_int);
    fn tomoyo_domain() -> *mut tomoyo_domain_info;
    fn tomoyo_open_control(key: u8, file: *mut file) -> isize;
    fn tomoyo_close_control(data: *mut c_void);
    fn tomoyo_poll_control(file: *mut file, wait: *mut poll_table) -> u32;
    fn tomoyo_read_control(data: *mut c_void, buf: *mut c_char, count: usize) -> isize;
    fn tomoyo_write_control(data: *mut c_void, buf: *const c_char, count: usize) -> isize;
    fn securityfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn securityfs_create_file(
        name: *const c_char, mode: u16, parent: *mut dentry, data: *mut c_void,
        fops: *const file_operations,
    );
    fn tomoyo_load_builtin_policy();
    fn noop_llseek(file: *mut file, offset: i64, whence: c_int) -> i64;
    fn strlen(s: *const c_char) -> usize;
    fn copy_to_user(to: *mut c_char, from: *const c_char, count: usize) -> usize;
    fn kfree(ptr: *mut c_void);
}

#[repr(C)] pub struct tomoyo_request_info { pub param: tomoyo_request_param, pub param_type: c_int, pub granted: bool }
#[repr(C)] pub union tomoyo_request_param { pub task: tomoyo_task_param, _bindgen_opaque: [u8; 0] }
#[repr(C)] pub struct tomoyo_task_param { pub domainname: *mut tomoyo_path_info }
#[repr(C)] pub struct tomoyo_acl_info { pub _opaque: [u8; 0] }
#[repr(C)] pub struct tomoyo_task_acl { pub head: tomoyo_acl_info, pub domainname: *mut tomoyo_path_info }
#[repr(C)] pub struct tomoyo_path_info { pub name: *mut c_char }
#[repr(C)] pub struct tomoyo_domain_info { pub domainname: *mut tomoyo_path_info }
#[repr(C)] pub struct tomoyo_task { pub domain_info: *mut tomoyo_domain_info }
#[repr(C)] pub struct task_struct;
#[repr(C)] pub struct inode;
#[repr(C)] pub struct dentry;
#[repr(C)] pub struct poll_table;
#[repr(C)] pub struct file { pub private_data: *mut c_void }
#[repr(C)] pub struct file_operations {
    pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, usize, *mut i64) -> isize>,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, usize, *mut i64) -> isize>,
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub poll: Option<unsafe extern "C" fn(*mut file, *mut poll_table) -> u32>,
    pub llseek: Option<unsafe extern "C" fn(*mut file, i64, c_int) -> i64>,
}

const TOMOYO_EXEC_TMPSIZE: usize = 4096;
const TOMOYO_MAC_FILE_EXECUTE: c_int = 0;
const TOMOYO_TYPE_MANUAL_TASK_ACL: c_int = 0;
const TOMOYO_QUERY: u8 = 0;
const TOMOYO_DOMAINPOLICY: u8 = 1;
const TOMOYO_EXCEPTIONPOLICY: u8 = 2;
const TOMOYO_AUDIT: u8 = 3;
const TOMOYO_PROCESS_STATUS: u8 = 4;
const TOMOYO_STAT: u8 = 5;
const TOMOYO_PROFILE: u8 = 6;
const TOMOYO_MANAGER: u8 = 7;
const TOMOYO_VERSION: u8 = 8;
const ENOMEM: isize = 12;
const EPERM: isize = 1;
const ENOENT: isize = 2;
const EINVAL: isize = 22;
const EFAULT: isize = 14;

unsafe extern "C" fn tomoyo_check_task_acl(r: *mut tomoyo_request_info, ptr: *const tomoyo_acl_info) -> bool {
    let acl = (ptr as *const u8).sub(core::mem::offset_of!(tomoyo_task_acl, head)) as *const tomoyo_task_acl;
    !tomoyo_pathcmp((*r).param.task.domainname, (*acl).domainname)
}

unsafe extern "C" fn tomoyo_write_self(_file: *mut file, buf: *const c_char, count: usize, _ppos: *mut i64) -> isize {
    if count == 0 || count >= TOMOYO_EXEC_TMPSIZE - 10 { return -ENOMEM; }
    let data = memdup_user_nul(buf, count);
    if data.is_null() { return -ENOMEM; }
    tomoyo_normalize_line(data);
    let mut error: isize;
    if tomoyo_correct_domain(data) {
        let idx = tomoyo_read_lock();
        let mut name = tomoyo_path_info { name: data };
        let mut r = tomoyo_request_info { param: tomoyo_request_param { task: tomoyo_task_param { domainname: core::ptr::null_mut() } }, param_type: 0, granted: false };
        tomoyo_fill_path_info(&mut name);
        tomoyo_init_request_info(&mut r, core::ptr::null_mut(), TOMOYO_MAC_FILE_EXECUTE);
        r.param_type = TOMOYO_TYPE_MANUAL_TASK_ACL;
        r.param.task.domainname = &mut name;
        tomoyo_check_acl(&mut r, Some(tomoyo_check_task_acl));
        if !r.granted { error = -EPERM; } else {
            let new_domain = tomoyo_assign_domain(data, true);
            if new_domain.is_null() { error = -ENOENT; } else {
                let s = tomoyo_task(current);
                let old_domain = (*s).domain_info;
                (*s).domain_info = new_domain;
                // Atomic user-count updates are provided by the surrounding translation.
                error = 0;
                let _ = old_domain;
            }
        }
        tomoyo_read_unlock(idx);
    } else { error = -EINVAL; }
    kfree(data as *mut c_void);
    if error != 0 { error } else { count as isize }
}

unsafe extern "C" fn tomoyo_read_self(_file: *mut file, buf: *mut c_char, count: usize, ppos: *mut i64) -> isize {
    let domain = (*(*tomoyo_domain()).domainname).name;
    let mut len = strlen(domain) as i64;
    let pos = *ppos;
    if pos >= len || count == 0 { return 0; }
    len -= pos;
    if (count as i64) < len { len = count as i64; }
    if copy_to_user(buf, domain.add(pos as usize), len as usize) != 0 { return -EFAULT; }
    *ppos += len;
    len as isize
}

static tomoyo_self_operations: file_operations = file_operations { write: Some(tomoyo_write_self), read: Some(tomoyo_read_self), open: None, release: None, poll: None, llseek: None };

unsafe extern "C" fn tomoyo_open(_inode: *mut inode, file: *mut file) -> c_int { tomoyo_open_control(0, file) as c_int }
unsafe extern "C" fn tomoyo_release(_inode: *mut inode, file: *mut file) -> c_int { tomoyo_close_control((*file).private_data); 0 }
unsafe extern "C" fn tomoyo_poll(file: *mut file, wait: *mut poll_table) -> u32 { tomoyo_poll_control(file, wait) }
unsafe extern "C" fn tomoyo_read(file: *mut file, buf: *mut c_char, count: usize, _ppos: *mut i64) -> isize { tomoyo_read_control((*file).private_data, buf, count) }
unsafe extern "C" fn tomoyo_write(file: *mut file, buf: *const c_char, count: usize, _ppos: *mut i64) -> isize { tomoyo_write_control((*file).private_data, buf, count) }
static tomoyo_operations: file_operations = file_operations { open: Some(tomoyo_open), release: Some(tomoyo_release), poll: Some(tomoyo_poll), read: Some(tomoyo_read), write: Some(tomoyo_write), llseek: Some(noop_llseek) };

unsafe fn tomoyo_create_entry(name: *const c_char, mode: u16, parent: *mut dentry, key: u8) { securityfs_create_file(name, mode, parent, key as usize as *mut c_void, &tomoyo_operations); }

#[no_mangle]
pub unsafe extern "C" fn tomoyo_interface_init() -> c_int {
    if !tomoyo_enabled { return 0; }
    let domain = tomoyo_domain();
    if domain != &mut tomoyo_kernel_domain { return 0; }
    let dir = securityfs_create_dir(b"tomoyo\0".as_ptr() as *const c_char, core::ptr::null_mut());
    tomoyo_create_entry(b"query\0".as_ptr() as *const c_char, 0o600, dir, TOMOYO_QUERY);
    tomoyo_create_entry(b"domain_policy\0".as_ptr() as *const c_char, 0o600, dir, TOMOYO_DOMAINPOLICY);
    tomoyo_create_entry(b"exception_policy\0".as_ptr() as *const c_char, 0o600, dir, TOMOYO_EXCEPTIONPOLICY);
    tomoyo_create_entry(b"audit\0".as_ptr() as *const c_char, 0o400, dir, TOMOYO_AUDIT);
    tomoyo_create_entry(b".process_status\0".as_ptr() as *const c_char, 0o600, dir, TOMOYO_PROCESS_STATUS);
    tomoyo_create_entry(b"stat\0".as_ptr() as *const c_char, 0o644, dir, TOMOYO_STAT);
    tomoyo_create_entry(b"profile\0".as_ptr() as *const c_char, 0o600, dir, TOMOYO_PROFILE);
    tomoyo_create_entry(b"manager\0".as_ptr() as *const c_char, 0o600, dir, TOMOYO_MANAGER);
    tomoyo_create_entry(b"version\0".as_ptr() as *const c_char, 0o400, dir, TOMOYO_VERSION);
    securityfs_create_file(b"self_domain\0".as_ptr() as *const c_char, 0o666, dir, core::ptr::null_mut(), &tomoyo_self_operations);
    tomoyo_load_builtin_policy();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
