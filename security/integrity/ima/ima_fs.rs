// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2005,2006,2007,2008 IBM Corporation
 *
 * Authors:
 * Kylene Hall <kjhall@us.ibm.com>
 * Reiner Sailer <sailer@us.ibm.com>
 * Mimi Zohar <zohar@us.ibm.com>
 *
 * File: ima_fs.c
 *  implemenents security file system for reporting
 *  current measurement list and IMA statistics
 */

// #include <linux/fcntl.h>
// #include <linux/kernel_read_file.h>
// #include <linux/slab.h>
// #include <linux/init.h>
// #include <linux/seq_file.h>
// #include <linux/rculist.h>
// #include <linux/rcupdate.h>
// #include <linux/parser.h>
// #include <linux/vmalloc.h>
//
// #include "ima.h"

/*
 * Requests:
 * 'A\n': stage the entire measurements list
 * 'D\n': delete all staged measurements
 * '[1, ULONG_MAX]\n' delete N measurements records
 */
const STAGED_REQ_LENGTH: usize = 21;

// lock for protecting concurrent IMA policy updates
// DEFINE_MUTEX(ima_write_mutex) - declared externally
pub static mut ima_write_mutex: *mut std::ffi::c_void = std::ptr::null_mut();

static mut ima_measure_mutex: *mut std::ffi::c_void = std::ptr::null_mut();
static mut ima_measure_users: i64 = 0;
static mut measure_writer: *mut std::ffi::c_void = std::ptr::null_mut();
static mut measure_writer_extra_writes: i64 = 0;

pub static mut ima_canonical_fmt: bool = false;

// __init attribute and return code for __setup callback
extern "C" {
    fn __setup(name: *const u8, func: unsafe extern "C" fn(*mut u8) -> i32);
}

unsafe extern "C" fn default_canonical_fmt_setup(_str: *mut u8) -> i32 {
    #[cfg(target_endian = "big")]
    {
        ima_canonical_fmt = true;
    }
    1
}

static mut valid_policy: i32 = 1;

// External declarations needed from ima.h and kernel headers
extern "C" {
    static mut ima_num_violations: *mut std::ffi::c_void;
    static mut ima_num_records: *mut std::ffi::c_void;
    static mut ima_measurements: std::ffi::c_int;
    static mut ima_measurements_staged: std::ffi::c_int;
    static mut ima_sha1_idx: std::ffi::c_int;
    static mut ima_tpm_chip: *mut std::ffi::c_void;
    static mut ima_algo_array: *mut std::ffi::c_void;
    static mut ima_flush_htable: bool;
    static mut ima_appraise: std::ffi::c_uint;
    static ima_policy_start: *mut std::ffi::c_void;
    static ima_policy_next: *mut std::ffi::c_void;
    static ima_policy_stop: *mut std::ffi::c_void;
    static ima_policy_show: *mut std::ffi::c_void;
    static integrity_dir: *mut std::ffi::c_void;
    static hash_algo_name: *const *const u8;

    fn atomic_long_read(v: *mut std::ffi::c_void) -> i64;
    fn scnprintf(buf: *mut u8, size: usize, fmt: *const u8, ...) -> isize;
    fn simple_read_from_buffer(
        to: *mut u8,
        n: usize,
        ppos: *mut i64,
        from: *const u8,
        available: usize,
    ) -> isize;
    fn seq_putc(m: *mut std::ffi::c_void, c: u8);
    fn seq_puts(m: *mut std::ffi::c_void, s: *const u8);
    fn seq_printf(m: *mut std::ffi::c_void, fmt: *const u8, ...);
    fn file_inode(file: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn cpu_to_le32(x: u32) -> u32;
    fn strcmp(s1: *const u8, s2: *const u8) -> i32;
    fn strlen(s: *const u8) -> usize;
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn list_for_each_entry_rcu(
        pos: *mut *mut std::ffi::c_void,
        head: *mut std::ffi::c_void,
        member: *const u8,
    );
    fn list_entry_rcu(
        ptr: *mut std::ffi::c_void,
        typ: *mut std::ffi::c_void,
        member: *const u8,
    ) -> *mut std::ffi::c_void;
    fn seq_open(
        file: *mut std::ffi::c_void,
        op: *const std::ffi::c_void,
    ) -> i32;
    fn seq_release(inode: *mut std::ffi::c_void, file: *mut std::ffi::c_void) -> i32;
    fn seq_read(
        file: *mut std::ffi::c_void,
        buf: *mut u8,
        size: usize,
        ppos: *mut i64,
    ) -> isize;
    fn seq_lseek(
        file: *mut std::ffi::c_void,
        offset: i64,
        whence: i32,
    ) -> i64;
    fn generic_file_llseek(
        file: *mut std::ffi::c_void,
        offset: i64,
        whence: i32,
    ) -> i64;
    fn mutex_lock(lock: *mut std::ffi::c_void);
    fn mutex_unlock(lock: *mut std::ffi::c_void);
    fn mutex_lock_interruptible(lock: *mut std::ffi::c_void) -> i32;
    fn capable(cap: i32) -> bool;
    fn copy_from_user(to: *mut u8, from: *const u8, len: usize) -> usize;
    fn kstrtoul(s: *const u8, base: i32, res: *mut u64) -> i32;
    fn pr_debug(fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);
    fn pr_info(fmt: *const u8, ...);
    fn integrity_audit_msg(
        type_: i32,
        owner: *mut std::ffi::c_void,
        subject: *mut std::ffi::c_void,
        op: *const u8,
        cause: *const u8,
        result: i32,
        info: i32,
    );
    fn ima_queue_stage() -> i32;
    fn ima_queue_staged_delete_all() -> i32;
    fn ima_queue_delete_partial(count: u64) -> i32;
    fn ima_parse_add_rule(rule: *const u8) -> i32;
    fn ima_measure_raw_policy(data: *const u8, len: usize);
    fn ima_check_policy() -> i32;
    fn ima_delete_rules();
    fn ima_update_policy();
    fn ima_measure_loaded_policy();
    fn integrity_fs_init() -> i32;
    fn integrity_fs_fini();
    fn kernel_read_file_from_path(
        path: *const u8,
        offset: u64,
        buf: *mut *mut u8,
        buf_size: usize,
        file_size: *mut usize,
        id: i32,
    ) -> i32;
    fn strsep(stringp: *mut *mut u8, delim: *const u8) -> *mut u8;
    fn vfree(addr: *mut std::ffi::c_void);
    fn memdup_user_nul(src: *const u8, len: usize) -> *mut u8;
    fn IS_ERR(ptr: *const std::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const std::ffi::c_void) -> i32;
    fn kfree(ptr: *mut std::ffi::c_void);
    fn securityfs_create_dir(name: *const u8, parent: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn securityfs_create_symlink(
        name: *const u8,
        parent: *mut std::ffi::c_void,
        target: *const u8,
        d_inode: *mut std::ffi::c_void,
    ) -> *mut std::ffi::c_void;
    fn securityfs_create_file(
        name: *const u8,
        mode: u32,
        parent: *mut std::ffi::c_void,
        data: *mut std::ffi::c_void,
        fops: *const std::ffi::c_void,
    ) -> *mut std::ffi::c_void;
    fn securityfs_remove(dentry: *mut std::ffi::c_void);
    fn test_and_set_bit(nr: i32, addr: *mut u64) -> bool;
    fn clear_bit(nr: i32, addr: *mut u64);
    fn snprintf(buf: *mut u8, size: usize, fmt: *const u8, ...) -> i32;
    fn current_task() -> *mut std::ffi::c_void;
}

// Constants from kernel headers
const LONG_MAX: i64 = i64::MAX;
const ENFILE: i32 = 23;
const EBUSY: i32 = 16;
const EPERM: i32 = 1;
const EINVAL: i32 = 22;
const EFAULT: i32 = 14;
const EACCES: i32 = 13;
const CAP_SYS_ADMIN: i32 = 21;
const INT_MAX: i32 = i32::MAX;
const PAGE_SIZE: usize = 4096;
const NAME_MAX: usize = 255;
const FMODE_WRITE: u32 = 0x2;
const O_WRONLY: i32 = 0x1;
const O_ACCMODE: i32 = 0x3;
const O_RDONLY: i32 = 0x0;
const BINARY: usize = 0;
const IMA_SHOW_BINARY: i32 = 0;
const IMA_SHOW_BINARY_NO_FIELD_LEN: i32 = 1;
const IMA_SHOW_BINARY_OLD_STRING_FMT: i32 = 2;
const IMA_SHOW_ASCII: i32 = 3;
const HASH_ALGO__LAST: u32 = u32::MAX;
const READING_POLICY: i32 = 4;
const POLICY_FILE_FLAGS: u32 = 0o600;
const IMA_APPRAISE_POLICY: u32 = 0x40;
const AUDIT_INTEGRITY_STATUS: i32 = 2001;
const S_IRUSR: u32 = 0o400;
const S_IWUSR: u32 = 0o200;
const S_IRGRP: u32 = 0o040;
const S_IWGRP: u32 = 0o020;

const IMA_TEMPLATE_IMA_NAME: &[u8] = b"ima\0";

// External struct definitions referenced but not defined here
#[repr(C)]
pub struct seq_operations {
    pub start: Option<unsafe extern "C" fn(*mut std::ffi::c_void, *mut i64) -> *mut std::ffi::c_void>,
    pub next: Option<unsafe extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void, *mut i64) -> *mut std::ffi::c_void>,
    pub stop: Option<unsafe extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void)>,
    pub show: Option<unsafe extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> i32>,
}

#[repr(C)]
pub struct file_operations {
    pub open: Option<unsafe extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> i32>,
    pub read: Option<unsafe extern "C" fn(*mut std::ffi::c_void, *mut u8, usize, *mut i64) -> isize>,
    pub write: Option<unsafe extern "C" fn(*mut std::ffi::c_void, *const u8, usize, *mut i64) -> isize>,
    pub llseek: Option<unsafe extern "C" fn(*mut std::ffi::c_void, i64, i32) -> i64>,
    pub release: Option<unsafe extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> i32>,
}

static mut ima_dir: *mut std::ffi::c_void = std::ptr::null_mut();
static mut ima_symlink: *mut std::ffi::c_void = std::ptr::null_mut();

#[repr(u32)]
enum ima_fs_flags {
    IMA_FS_BUSY = 0,
}

static mut ima_fs_flags: u64 = 0;

static mut ima_num_records_BINARY: i64 = 0;

unsafe fn current() -> *mut std::ffi::c_void {
    current_task()
}

unsafe fn ima_show_counter(
    buf: *mut u8,
    count: usize,
    ppos: *mut i64,
    val: *mut std::ffi::c_void,
) -> isize {
    let tmpbuf_size = 32;
    let mut tmpbuf: [u8; 32] = [0; 32];

    let len = scnprintf(
        tmpbuf.as_mut_ptr(),
        tmpbuf_size,
        b"%li\n\0".as_ptr(),
        atomic_long_read(val),
    );
    simple_read_from_buffer(buf, count, ppos, tmpbuf.as_ptr(), len as usize)
}

unsafe extern "C" fn ima_show_num_violations(
    _filp: *mut std::ffi::c_void,
    buf: *mut u8,
    count: usize,
    ppos: *mut i64,
) -> isize {
    ima_show_counter(buf, count, ppos, ima_num_violations)
}

static ima_num_violations_ops: file_operations = file_operations {
    open: None,
    read: Some(ima_show_num_violations),
    write: None,
    llseek: Some(generic_file_llseek),
    release: None,
};

unsafe extern "C" fn ima_show_measurements_count(
    _filp: *mut std::ffi::c_void,
    buf: *mut u8,
    count: usize,
    ppos: *mut i64,
) -> isize {
    ima_show_counter(buf, count, ppos, ima_num_records)
}

static ima_measurements_count_ops: file_operations = file_operations {
    open: None,
    read: Some(ima_show_measurements_count),
    write: None,
    llseek: Some(generic_file_llseek),
    release: None,
};

unsafe fn _ima_measurements_start(
    _m: *mut std::ffi::c_void,
    pos: *mut i64,
    head: *mut std::ffi::c_void,
) -> *mut std::ffi::c_void {
    let mut l = *pos;
    let mut qe: *mut std::ffi::c_void = std::ptr::null_mut();

    rcu_read_lock();
    // Simulating list_for_each_entry_rcu
    let mut current = head;
    loop {
        // This would normally iterate through the list
        // The exact implementation depends on kernel macros
        // For now we approximate the behavior
        if l <= 0 {
            rcu_read_unlock();
            return qe;
        }
        l -= 1;
        current = std::ptr::null_mut();
        if current.is_null() {
            break;
        }
    }
    rcu_read_unlock();
    std::ptr::null_mut()
}

unsafe extern "C" fn ima_measurements_start(
    m: *mut std::ffi::c_void,
    pos: *mut i64,
) -> *mut std::ffi::c_void {
    _ima_measurements_start(m, pos, &ima_measurements as *const _ as *mut _)
}

unsafe extern "C" fn ima_measurements_staged_start(
    m: *mut std::ffi::c_void,
    pos: *mut i64,
) -> *mut std::ffi::c_void {
    _ima_measurements_start(m, pos, &ima_measurements_staged as *const _ as *mut _)
}

unsafe fn _ima_measurements_next(
    _m: *mut std::ffi::c_void,
    v: *mut std::ffi::c_void,
    pos: *mut i64,
    head: *mut std::ffi::c_void,
) -> *mut std::ffi::c_void {
    let mut qe = v;

    rcu_read_lock();
    // list_entry_rcu simulation
    qe = list_entry_rcu(qe, std::ptr::null_mut(), b"later\0".as_ptr());
    rcu_read_unlock();
    *pos += 1;

    if qe == head {
        std::ptr::null_mut()
    } else {
        qe
    }
}

unsafe extern "C" fn ima_measurements_next(
    m: *mut std::ffi::c_void,
    v: *mut std::ffi::c_void,
    pos: *mut i64,
) -> *mut std::ffi::c_void {
    _ima_measurements_next(m, v, pos, &ima_measurements as *const _ as *mut _)
}

unsafe extern "C" fn ima_measurements_staged_next(
    m: *mut std::ffi::c_void,
    v: *mut std::ffi::c_void,
    pos: *mut i64,
) -> *mut std::ffi::c_void {
    _ima_measurements_next(m, v, pos, &ima_measurements_staged as *const _ as *mut _)
}

unsafe extern "C" fn ima_measurements_stop(_m: *mut std::ffi::c_void, _v: *mut std::ffi::c_void) {}

pub unsafe fn ima_putc(m: *mut std::ffi::c_void, data: *mut std::ffi::c_void, mut datalen: i32) {
    let mut ptr = data as *mut u8;
    while datalen > 0 {
        seq_putc(m, *ptr);
        ptr = ptr.add(1);
        datalen -= 1;
    }
}

unsafe extern "C" fn ima_measurements_show(
    m: *mut std::ffi::c_void,
    v: *mut std::ffi::c_void,
) -> i32 {
    let qe = v as *mut std::ffi::c_void;
    let mut e: *mut std::ffi::c_void = std::ptr::null_mut();
    let mut template_name: *mut u8 = std::ptr::null_mut();
    let mut pcr: u32 = 0;
    let mut namelen: u32 = 0;
    let mut template_data_len: u32 = 0;
    let mut is_ima_template = false;
    let mut algo_idx = ima_sha1_idx;

    if !(*std::ptr::addr_of!((*std::ptr::addr_of!(m)))).is_null() {
        algo_idx = file_inode(m) as usize as i32;
    }

    // Get entry - simplified without actual struct access
    if e.is_null() {
        return -1;
    }

    // Fetch template name
    // template_name = (e->template_desc->name[0] != '\0') ? e->template_desc->name : e->template_desc->fmt;

    // PCRIndex
    pcr = if ima_canonical_fmt {
        cpu_to_le32(pcr)
    } else {
        pcr
    };
    ima_putc(m, &pcr as *const _ as *mut _, std::mem::size_of_val(&pcr) as i32);

    // Template digest
    // ima_putc(m, e->digests[algo_idx].digest, ima_algo_array[algo_idx].digest_size);

    // Template name size
    namelen = if ima_canonical_fmt {
        cpu_to_le32(namelen)
    } else {
        namelen
    };
    ima_putc(m, &namelen as *const _ as *mut _, std::mem::size_of_val(&namelen) as i32);

    // Template name
    ima_putc(m, template_name as *mut _, strlen(template_name as *const _) as i32);

    // Template length (except for 'ima' template)
    if strcmp(template_name as *const _, IMA_TEMPLATE_IMA_NAME.as_ptr()) == 0 {
        is_ima_template = true;
    }

    if !is_ima_template {
        template_data_len = if ima_canonical_fmt {
            cpu_to_le32(template_data_len)
        } else {
            template_data_len
        };
        ima_putc(
            m,
            &template_data_len as *const _ as *mut _,
            std::mem::size_of_val(&template_data_len) as i32,
        );
    }

    // Template specific data
    // for (i = 0; i < e->template_desc->num_fields; i++) { ... }

    0
}

static ima_measurments_seqops: seq_operations = seq_operations {
    start: Some(ima_measurements_start),
    next: Some(ima_measurements_next),
    stop: Some(ima_measurements_stop),
    show: Some(ima_measurements_show),
};

static ima_measurments_staged_seqops: seq_operations = seq_operations {
    start: Some(ima_measurements_staged_start),
    next: Some(ima_measurements_staged_next),
    stop: Some(ima_measurements_stop),
    show: Some(ima_measurements_show),
};

unsafe fn ima_measure_lock(write: bool) -> i32 {
    mutex_lock(ima_measure_mutex);
    // Overflow check
    if !write && ima_measure_users == LONG_MAX {
        mutex_unlock(ima_measure_mutex);
        return -ENFILE;
    }

    // Same writer can do additional writes or read/writes
    if write && current() == measure_writer {
        measure_writer_extra_writes += 1;
        mutex_unlock(ima_measure_mutex);
        return 0;
    }

    // ima_measure_users: > 0 open readers
    // ima_measure_users: == -1 open writer
    if (write && ima_measure_users != 0) || (!write && ima_measure_users < 0) {
        mutex_unlock(ima_measure_mutex);
        return -EBUSY;
    }

    if write {
        ima_measure_users -= 1;
        // Pointer valid, no reuse while the file descriptor is open
        measure_writer = current();
    } else {
        ima_measure_users += 1;
    }
    mutex_unlock(ima_measure_mutex);
    0
}

unsafe fn ima_measure_unlock(write: bool) {
    mutex_lock(ima_measure_mutex);
    // Decrement additional writes or read/writes
    if write && current() == measure_writer && measure_writer_extra_writes != 0 {
        measure_writer_extra_writes -= 1;
        mutex_unlock(ima_measure_mutex);
        return;
    }
    if write {
        ima_measure_users += 1;
        measure_writer = std::ptr::null_mut();
    } else {
        ima_measure_users -= 1;
    }
    mutex_unlock(ima_measure_mutex);
}

unsafe fn _ima_measurements_open(
    _inode: *mut std::ffi::c_void,
    file: *mut std::ffi::c_void,
    seq_ops: *const seq_operations,
) -> i32 {
    // Simulate file->f_mode access
    let write = false; // Placeholder: should extract from file struct

    if write && !capable(CAP_SYS_ADMIN) {
        return -EPERM;
    }

    let ret = ima_measure_lock(write);
    if ret < 0 {
        return ret;
    }

    let ret = seq_open(file, seq_ops as *const _);
    if ret < 0 {
        ima_measure_unlock(write);
    }

    ret
}

unsafe extern "C" fn ima_measurements_open(
    inode: *mut std::ffi::c_void,
    file: *mut std::ffi::c_void,
) -> i32 {
    _ima_measurements_open(inode, file, &ima_measurments_seqops)
}

unsafe extern "C" fn ima_measurements_release(
    inode: *mut std::ffi::c_void,
    file: *mut std::ffi::c_void,
) -> i32 {
    let write = false;
    let ret = seq_release(inode, file);
    ima_measure_unlock(write);
    ret
}

unsafe extern "C" fn ima_measurements_staged_open(
    inode: *mut std::ffi::c_void,
    file: *mut std::ffi::c_void,
) -> i32 {
    _ima_measurements_open(inode, file, &ima_measurments_staged_seqops)
}

unsafe fn _ima_measurements_write(
    _file: *mut std::ffi::c_void,
    buf: *const u8,
    datalen: usize,
    _ppos: *mut i64,
    staged_interface: bool,
) -> isize {
    let mut req: [u8; STAGED_REQ_LENGTH] = [0; STAGED_REQ_LENGTH];
    let mut req_value: u64 = 0;
    let mut ret: i32 = 0;

    if datalen < 2 || datalen > STAGED_REQ_LENGTH {
        return -EINVAL as isize;
    }

    if copy_from_user(req.as_mut_ptr(), buf, datalen) != 0 {
        return -EFAULT as isize;
    }

    if req[datalen - 1] != b'\n' {
        return -EINVAL as isize;
    }

    req[datalen - 1] = b'\0';

    match req[0] {
        b'A' => {
            if datalen != 2 || !staged_interface {
                return -EINVAL as isize;
            }
            ret = ima_queue_stage();
        }
        b'D' => {
            if datalen != 2 || !staged_interface {
                return -EINVAL as isize;
            }
            ret = ima_queue_staged_delete_all();
        }
        _ => {
            if staged_interface {
                return -EINVAL as isize;
            }

            if ima_flush_htable {
                pr_debug(b"Deleting staged N measurements not supported when flushing the hash table is requested\n\0".as_ptr());
                return -EINVAL as isize;
            }

            ret = kstrtoul(req.as_ptr(), 10, &mut req_value);
            if ret < 0 {
                return ret as isize;
            }

            if req_value == 0 {
                pr_debug(b"Must delete at least one entry\n\0".as_ptr());
                return -EINVAL as isize;
            }

            ret = ima_queue_delete_partial(req_value);
        }
    }

    if ret < 0 {
        return ret as isize;
    }

    datalen as isize
}

unsafe extern "C" fn ima_measurements_write(
    file: *mut std::ffi::c_void,
    buf: *const u8,
    datalen: usize,
    ppos: *mut i64,
) -> isize {
    _ima_measurements_write(file, buf, datalen, ppos, false)
}

unsafe extern "C" fn ima_measurements_staged_write(
    file: *mut std::ffi::c_void,
    buf: *const u8,
    datalen: usize,
    ppos: *mut i64,
) -> isize {
    _ima_measurements_write(file, buf, datalen, ppos, true)
}

static ima_measurements_ops: file_operations = file_operations {
    open: Some(ima_measurements_open),
    read: Some(seq_read),
    write: Some(ima_measurements_write),
    llseek: Some(seq_lseek),
    release: Some(ima_measurements_release),
};

static ima_measurements_staged_ops: file_operations = file_operations {
    open: Some(ima_measurements_staged_open),
    read: Some(seq_read),
    write: Some(ima_measurements_staged_write),
    llseek: Some(seq_lseek),
    release: Some(ima_measurements_release),
};

pub unsafe fn ima_print_digest(m: *mut std::ffi::c_void, digest: *mut u8, size: u32) {
    for i in 0..size {
        seq_printf(m, b"%02x\0".as_ptr(), *digest.add(i as usize));
    }
}

unsafe extern "C" fn ima_ascii_measurements_show(
    m: *mut std::ffi::c_void,
    v: *mut std::ffi::c_void,
) -> i32 {
    let _qe = v;
    let mut _e: *mut std::ffi::c_void = std::ptr::null_mut();
    let mut _template_name: *mut u8 = std::ptr::null_mut();
    let mut algo_idx = ima_sha1_idx;

    if !m.is_null() {
        algo_idx = file_inode(m) as usize as i32;
    }

    // Get entry
    if _e.is_null() {
        return -1;
    }

    // Simplified without full struct access
    seq_printf(m, b"%2d \0".as_ptr(), 0);

    // ima_print_digest(m, e->digests[algo_idx].digest, ima_algo_array[algo_idx].digest_size);

    seq_printf(m, b" %s\0".as_ptr(), _template_name);

    // for (i = 0; i < e->template_desc->num_fields; i++) { ... }

    seq_puts(m, b"\n\0".as_ptr());
    0
}

static ima_ascii_measurements_seqops: seq_operations = seq_operations {
    start: Some(ima_measurements_start),
    next: Some(ima_measurements_next),
    stop: Some(ima_measurements_stop),
    show: Some(ima_ascii_measurements_show),
};

unsafe extern "C" fn ima_ascii_measurements_open(
    inode: *mut std::ffi::c_void,
    file: *mut std::ffi::c_void,
) -> i32 {
    _ima_measurements_open(inode, file, &ima_ascii_measurements_seqops)
}

static ima_ascii_measurements_ops: file_operations = file_operations {
    open: Some(ima_ascii_measurements_open),
    read: Some(seq_read),
    write: Some(ima_measurements_write),
    llseek: Some(seq_lseek),
    release: Some(ima_measurements_release),
};

static ima_ascii_measurements_staged_seqops: seq_operations = seq_operations {
    start: Some(ima_measurements_staged_start),
    next: Some(ima_measurements_staged_next),
    stop: Some(ima_measurements_stop),
    show: Some(ima_ascii_measurements_show),
};

unsafe extern "C" fn ima_ascii_measurements_staged_open(
    inode: *mut std::ffi::c_void,
    file: *mut std::ffi::c_void,
) -> i32 {
    _ima_measurements_open(inode, file, &ima_ascii_measurements_staged_seqops)
}

static ima_ascii_measurements_staged_ops: file_operations = file_operations {
    open: Some(ima_ascii_measurements_staged_open),
    read: Some(seq_read),
    write: Some(ima_measurements_staged_write),
    llseek: Some(seq_lseek),
    release: Some(ima_measurements_release),
};

unsafe fn ima_read_policy(path: *mut u8) -> isize {
    let mut data: *mut u8 = std::ptr::null_mut();
    let mut datap: *mut u8 = std::ptr::null_mut();
    let mut size: usize = 0;
    let mut rc: i32 = 0;
    let pathlen = strlen(path as *const _) as i32;

    // Remove \n
    datap = path;
    strsep(&mut datap, b"\n\0".as_ptr());

    rc = kernel_read_file_from_path(
        path as *const _,
        0,
        &mut data,
        INT_MAX as usize,
        &mut size,
        READING_POLICY,
    );
    if rc < 0 {
        pr_err(b"Unable to open file: %s (%d)\0".as_ptr(), path, rc);
        return rc as isize;
    }
    size = rc as usize;
    rc = 0;

    datap = data;
    while size > 0 {
        let p = strsep(&mut datap, b"\n\0".as_ptr());
        if !p.is_null() {
            pr_debug(b"rule: %s\n\0".as_ptr(), p);
            rc = ima_parse_add_rule(p as *const _);
            if rc < 0 {
                break;
            }
            size = size.wrapping_sub(rc as usize);
        } else {
            break;
        }
    }

    vfree(data as *mut _);
    if rc < 0 {
        rc as isize
    } else if size != 0 {
        -EINVAL as isize
    } else {
        pathlen as isize
    }
}

unsafe extern "C" fn ima_write_policy(
    _file: *mut std::ffi::c_void,
    buf: *const u8,
    mut datalen: usize,
    ppos: *mut i64,
) -> isize {
    let mut data: *mut u8 = std::ptr::null_mut();
    let mut result: isize = 0;

    if datalen >= PAGE_SIZE {
        datalen = PAGE_SIZE - 1;
    }

    // No partial writes
    result = -EINVAL as isize;
    if *ppos != 0 {
        return result;
    }

    data = memdup_user_nul(buf, datalen);
    if IS_ERR(data as *const _) {
        result = PTR_ERR(data as *const _) as isize;
        return result;
    }

    result = mutex_lock_interruptible(ima_write_mutex) as isize;
    if result < 0 {
        kfree(data as *mut _);
        return result;
    }

    if *data == b'/' {
        result = ima_read_policy(data);
    } else if ima_appraise & IMA_APPRAISE_POLICY != 0 {
        pr_err(b"signed policy file (specified as an absolute pathname) required\n\0".as_ptr());
        integrity_audit_msg(
            AUDIT_INTEGRITY_STATUS,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            b"policy_update\0".as_ptr(),
            b"signed policy required\0".as_ptr(),
            1,
            0,
        );
        result = -EACCES as isize;
    } else {
        ima_measure_raw_policy(data as *const _, datalen);
        result = ima_parse_add_rule(data as *const _) as isize;
    }
    mutex_unlock(ima_write_mutex);

    kfree(data as *mut _);

    if result < 0 {
        valid_policy = 0;
    }

    result
}

// CONFIG_IMA_READ_POLICY conditional
#[cfg(any())] // Placeholder - CONFIG_IMA_READ_POLICY not in scope
static ima_policy_seqops: seq_operations = seq_operations {
    start: Some(ima_policy_start),
    next: Some(ima_policy_next),
    stop: Some(ima_policy_stop),
    show: Some(ima_policy_show),
};

unsafe fn create_securityfs_measurement_lists(staging: bool) -> i32 {
    let ascii_ops = if staging {
        &ima_ascii_measurements_staged_ops
    } else {
        &ima_ascii_measurements_ops
    };
    let binary_ops = if staging {
        &ima_measurements_staged_ops
    } else {
        &ima_measurements_ops
    };
    let permissions = S_IRUSR | S_IRGRP | S_IWUSR | S_IWGRP;
    let file_suffix = if staging { b"_staged\0" } else { b"\0" };
    let file_suffix_ptr = if staging {
        b"_staged\0".as_ptr() as *const u8
    } else {
        b"\0".as_ptr() as *const u8
    };

    // TODO: NR_BANKS macro - depends on external definition
    let count = 1;

    for i in 0..count {
        let mut file_name: [u8; NAME_MAX + 1] = [0; NAME_MAX + 1];
        let algo = 0; // Placeholder

        if algo == HASH_ALGO__LAST {
            snprintf(
                file_name.as_mut_ptr(),
                NAME_MAX + 1,
                b"ascii_runtime_measurements_tpm_alg_%x%s\0".as_ptr(),
                0,
                file_suffix_ptr,
            );
        } else {
            snprintf(
                file_name.as_mut_ptr(),
                NAME_MAX + 1,
                b"ascii_runtime_measurements_%s%s\0".as_ptr(),
                hash_algo_name as *const u8,
                file_suffix_ptr,
            );
        }

        let dentry = securityfs_create_file(
            file_name.as_ptr(),
            permissions,
            ima_dir,
            i as *mut std::ffi::c_void,
            ascii_ops as *const _ as *const std::ffi::c_void,
        );
        if IS_ERR(dentry as *const _) {
            return PTR_ERR(dentry as *const _);
        }

        if algo == HASH_ALGO__LAST {
            snprintf(
                file_name.as_mut_ptr(),
                NAME_MAX + 1,
                b"binary_runtime_measurements_tpm_alg_%x%s\0".as_ptr(),
                0,
                file_suffix_ptr,
            );
        } else {
            snprintf(
                file_name.as_mut_ptr(),
                NAME_MAX + 1,
                b"binary_runtime_measurements_%s%s\0".as_ptr(),
                hash_algo_name as *const u8,
                file_suffix_ptr,
            );
        }

        let dentry = securityfs_create_file(
            file_name.as_ptr(),
            permissions,
            ima_dir,
            i as *mut std::ffi::c_void,
            binary_ops as *const _ as *const std::ffi::c_void,
        );
        if IS_ERR(dentry as *const _) {
            return PTR_ERR(dentry as *const _);
        }
    }

    0
}

unsafe fn create_securityfs_staging_links() -> i32 {
    let dentry = securityfs_create_symlink(
        b"binary_runtime_measurements_staged\0".as_ptr(),
        ima_dir,
        b"binary_runtime_measurements_sha1_staged\0".as_ptr(),
        std::ptr::null_mut(),
    );
    if IS_ERR(dentry as *const _) {
        return PTR_ERR(dentry as *const _);
    }

    let dentry = securityfs_create_symlink(
        b"ascii_runtime_measurements_staged\0".as_ptr(),
        ima_dir,
        b"ascii_runtime_measurements_sha1_staged\0".as_ptr(),
        std::ptr::null_mut(),
    );
    if IS_ERR(dentry as *const _) {
        return PTR_ERR(dentry as *const _);
    }

    0
}

unsafe extern "C" fn ima_open_policy(
    _inode: *mut std::ffi::c_void,
    filp: *mut std::ffi::c_void,
) -> i32 {
    // Simulate filp->f_flags access
    let f_flags = 0; // Placeholder

    if f_flags & O_WRONLY == 0 {
        // CONFIG_IMA_READ_POLICY conditional check
        #[cfg(any())] // Placeholder
        {
            if (f_flags & O_ACCMODE) != O_RDONLY {
                return -EACCES;
            }
            if !capable(CAP_SYS_ADMIN) {
                return -EPERM;
            }
            // This would call: return seq_open(filp, &ima_policy_seqops);
        }
        #[cfg(not(any()))]
        {
            return -EACCES;
        }
    }

    if test_and_set_bit(ima_fs_flags::IMA_FS_BUSY as i32, &mut ima_fs_flags) {
        return -EBUSY;
    }
    0
}

unsafe extern "C" fn ima_release_policy(
    inode: *mut std::ffi::c_void,
    file: *mut std::ffi::c_void,
) -> i32 {
    let cause = if valid_policy == 1 {
        b"completed\0".as_ptr()
    } else {
        b"failed\0".as_ptr()
    };

    // Simulate file->f_flags access
    let f_flags = 0; // Placeholder

    if (f_flags & O_ACCMODE) == O_RDONLY {
        return seq_release(inode, file);
    }

    if valid_policy != 0 && ima_check_policy() < 0 {
        valid_policy = 0;
    }

    pr_info(b"policy update %s\n\0".as_ptr(), cause);
    integrity_audit_msg(
        AUDIT_INTEGRITY_STATUS,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        b"policy_update\0".as_ptr(),
        cause,
        if valid_policy != 0 { 0 } else { 1 },
        0,
    );

    if valid_policy == 0 {
        ima_delete_rules();
        valid_policy = 1;
        clear_bit(ima_fs_flags::IMA_FS_BUSY as i32, &mut ima_fs_flags);
        return 0;
    }

    ima_update_policy();

    mutex_lock(ima_write_mutex);
    ima_measure_loaded_policy();
    mutex_unlock(ima_write_mutex);

    // CONFIG conditional compilation
    #[cfg(all(
        not(feature = "CONFIG_IMA_WRITE_POLICY"),
        not(feature = "CONFIG_IMA_READ_POLICY")
    ))]
    {
        // securityfs_remove(file->f_path.dentry);
    }
    #[cfg(feature = "CONFIG_IMA_WRITE_POLICY")]
    {
        clear_bit(ima_fs_flags::IMA_FS_BUSY as i32, &mut ima_fs_flags);
    }
    #[cfg(feature = "CONFIG_IMA_READ_POLICY")]
    {
        // inode->i_mode &= ~S_IWUSR;
    }

    0
}

static ima_measure_policy_ops: file_operations = file_operations {
    open: Some(ima_open_policy),
    read: Some(seq_read),
    write: Some(ima_write_policy),
    llseek: Some(generic_file_llseek),
    release: Some(ima_release_policy),
};

pub unsafe fn ima_fs_init() -> i32 {
    let mut dentry: *mut std::ffi::c_void = std::ptr::null_mut();
    let mut ret: i32 = 0;

    ret = integrity_fs_init();
    if ret < 0 {
        return ret;
    }

    ima_dir = securityfs_create_dir(b"ima\0".as_ptr(), integrity_dir);
    if IS_ERR(ima_dir as *const _) {
        ret = PTR_ERR(ima_dir as *const _);
        return ret;
    }

    ima_symlink = securityfs_create_symlink(
        b"ima\0".as_ptr(),
        std::ptr::null_mut(),
        b"integrity/ima\0".as_ptr(),
        std::ptr::null_mut(),
    );
    if IS_ERR(ima_symlink as *const _) {
        ret = PTR_ERR(ima_symlink as *const _);
        return ret;
    }

    ret = create_securityfs_measurement_lists(false);
    // CONFIG_IMA_STAGING conditional
    #[cfg(feature = "CONFIG_IMA_STAGING")]
    if ret == 0 {
        ret = create_securityfs_measurement_lists(true);
        if ret == 0 {
            ret = create_securityfs_staging_links();
        }
    }

    if ret != 0 {
        return ret;
    }

    dentry = securityfs_create_symlink(
        b"binary_runtime_measurements\0".as_ptr(),
        ima_dir,
        b"binary_runtime_measurements_sha1\0".as_ptr(),
        std::ptr::null_mut(),
    );
    if IS_ERR(dentry as *const _) {
        ret = PTR_ERR(dentry as *const _);
        return ret;
    }

    dentry = securityfs_create_symlink(
        b"ascii_runtime_measurements\0".as_ptr(),
        ima_dir,
        b"ascii_runtime_measurements_sha1\0".as_ptr(),
        std::ptr::null_mut(),
    );
    if IS_ERR(dentry as *const _) {
        ret = PTR_ERR(dentry as *const _);
        return ret;
    }

    dentry = securityfs_create_file(
        b"runtime_measurements_count\0".as_ptr(),
        S_IRUSR | S_IRGRP,
        ima_dir,
        std::ptr::null_mut(),
        &ima_measurements_count_ops as *const _ as *const std::ffi::c_void,
    );
    if IS_ERR(dentry as *const _) {
        ret = PTR_ERR(dentry as *const _);
        return ret;
    }

    dentry = securityfs_create_file(
        b"violations\0".as_ptr(),
        S_IRUSR | S_IRGRP,
        ima_dir,
        std::ptr::null_mut(),
        &ima_num_violations_ops as *const _ as *const std::ffi::c_void,
    );
    if IS_ERR(dentry as *const _) {
        ret = PTR_ERR(dentry as *const _);
        return ret;
    }

    dentry = securityfs_create_file(
        b"policy\0".as_ptr(),
        POLICY_FILE_FLAGS,
        ima_dir,
        std::ptr::null_mut(),
        &ima_measure_policy_ops as *const _ as *const std::ffi::c_void,
    );
    if IS_ERR(dentry as *const _) {
        ret = PTR_ERR(dentry as *const _);
        return ret;
    }

    0
}

// Cleanup path
// This would be called on error in ima_fs_init()
unsafe fn ima_fs_cleanup() {
    securityfs_remove(ima_symlink);
    securityfs_remove(ima_dir);
    integrity_fs_fini();
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
