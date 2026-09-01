// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Functions for the OPL4 proc file
 * Copyright (c) 2003 by Clemens Ladisch <clemens@ladisch.de>
 */

// C dependencies: "opl4_local.h", <linux/vmalloc.h>, <linux/export.h>, <sound/info.h>

use core::ffi::{c_char, c_int, c_ushort, c_void};

type ssize_t = isize;
type size_t = usize;
type loff_t = i64;

const EBUSY: c_int = 16;
const ENOMEM: c_int = 12;
const EFAULT: c_int = 14;
const OPL3_HW_OPL4_ML: c_int = 0; // external constant value supplied by opl4_local.h in the full build
const SNDRV_INFO_CONTENT_DATA: c_int = 0; // external constant value supplied by sound/info.h in the full build

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub proc_root: *mut snd_info_entry,
}

#[repr(C)]
pub struct snd_opl4 {
    pub card: *mut snd_card,
    pub access_mutex: mutex,
    pub memory_access: c_int,
    pub hardware: c_int,
    pub proc_entry: *mut snd_info_entry,
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub union snd_info_entry_c {
    pub ops: *const snd_info_entry_ops,
}

#[repr(C)]
pub struct snd_info_entry {
    pub private_data: *mut c_void,
    pub mode: c_ushort,
    pub size: size_t,
    pub content: c_int,
    pub c: snd_info_entry_c,
    pub module: *mut module,
}

#[repr(C)]
pub struct snd_info_entry_ops {
    pub open: Option<
        unsafe extern "C" fn(
            entry: *mut snd_info_entry,
            mode: c_ushort,
            file_private_data: *mut *mut c_void,
        ) -> c_int,
    >,
    pub release: Option<
        unsafe extern "C" fn(
            entry: *mut snd_info_entry,
            mode: c_ushort,
            file_private_data: *mut c_void,
        ) -> c_int,
    >,
    pub read: Option<
        unsafe extern "C" fn(
            entry: *mut snd_info_entry,
            file_private_data: *mut c_void,
            file: *mut file,
            _buf: *mut c_char,
            count: size_t,
            pos: loff_t,
        ) -> ssize_t,
    >,
    pub write: Option<
        unsafe extern "C" fn(
            entry: *mut snd_info_entry,
            file_private_data: *mut c_void,
            file: *mut file,
            _buf: *const c_char,
            count: size_t,
            pos: loff_t,
        ) -> ssize_t,
    >,
}

extern "C" {
    static mut THIS_MODULE: *mut module;

    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn vmalloc(size: size_t) -> *mut c_void;
    fn vfree(addr: *mut c_void);
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: size_t) -> size_t;
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: size_t) -> size_t;
    fn snd_opl4_read_memory(opl4: *mut snd_opl4, buf: *mut c_char, pos: loff_t, count: size_t);
    fn snd_opl4_write_memory(opl4: *mut snd_opl4, buf: *mut c_char, pos: loff_t, count: size_t);
    fn snd_info_create_card_entry(
        card: *mut snd_card,
        name: *const c_char,
        parent: *mut snd_info_entry,
    ) -> *mut snd_info_entry;
    fn snd_info_free_entry(entry: *mut snd_info_entry);
}

struct MutexGuard {
    lock: *mut mutex,
}

impl MutexGuard {
    unsafe fn new(lock: *mut mutex) -> Self {
        mutex_lock(lock);
        Self { lock }
    }
}

impl Drop for MutexGuard {
    fn drop(&mut self) {
        unsafe {
            mutex_unlock(self.lock);
        }
    }
}

unsafe extern "C" fn snd_opl4_mem_proc_open(
    entry: *mut snd_info_entry,
    _mode: c_ushort,
    _file_private_data: *mut *mut c_void,
) -> c_int {
    let opl4 = (*entry).private_data as *mut snd_opl4;

    let _guard = MutexGuard::new(&mut (*opl4).access_mutex);
    if (*opl4).memory_access != 0 {
        return -EBUSY;
    }
    (*opl4).memory_access += 1;
    0
}

unsafe extern "C" fn snd_opl4_mem_proc_release(
    entry: *mut snd_info_entry,
    _mode: c_ushort,
    _file_private_data: *mut c_void,
) -> c_int {
    let opl4 = (*entry).private_data as *mut snd_opl4;

    let _guard = MutexGuard::new(&mut (*opl4).access_mutex);
    (*opl4).memory_access -= 1;
    0
}

unsafe extern "C" fn snd_opl4_mem_proc_read(
    entry: *mut snd_info_entry,
    _file_private_data: *mut c_void,
    _file: *mut file,
    _buf: *mut c_char,
    count: size_t,
    pos: loff_t,
) -> ssize_t {
    let opl4 = (*entry).private_data as *mut snd_opl4;
    let buf: *mut c_char;

    buf = vmalloc(count) as *mut c_char;
    if buf.is_null() {
        return -ENOMEM as ssize_t;
    }
    snd_opl4_read_memory(opl4, buf, pos, count);
    if copy_to_user(_buf as *mut c_void, buf as *const c_void, count) != 0 {
        vfree(buf as *mut c_void);
        return -EFAULT as ssize_t;
    }
    vfree(buf as *mut c_void);
    count as ssize_t
}

unsafe extern "C" fn snd_opl4_mem_proc_write(
    entry: *mut snd_info_entry,
    _file_private_data: *mut c_void,
    _file: *mut file,
    _buf: *const c_char,
    count: size_t,
    pos: loff_t,
) -> ssize_t {
    let opl4 = (*entry).private_data as *mut snd_opl4;
    let buf: *mut c_char;

    buf = vmalloc(count) as *mut c_char;
    if buf.is_null() {
        return -ENOMEM as ssize_t;
    }
    if copy_from_user(buf as *mut c_void, _buf as *const c_void, count) != 0 {
        vfree(buf as *mut c_void);
        return -EFAULT as ssize_t;
    }
    snd_opl4_write_memory(opl4, buf, pos, count);
    vfree(buf as *mut c_void);
    count as ssize_t
}

static snd_opl4_mem_proc_ops: snd_info_entry_ops = snd_info_entry_ops {
    open: Some(snd_opl4_mem_proc_open),
    release: Some(snd_opl4_mem_proc_release),
    read: Some(snd_opl4_mem_proc_read),
    write: Some(snd_opl4_mem_proc_write),
};

#[no_mangle]
pub unsafe extern "C" fn snd_opl4_create_proc(opl4: *mut snd_opl4) -> c_int {
    let entry: *mut snd_info_entry;

    entry = snd_info_create_card_entry(
        (*opl4).card,
        b"opl4-mem\0".as_ptr() as *const c_char,
        (*(*opl4).card).proc_root,
    );
    if !entry.is_null() {
        if (*opl4).hardware < OPL3_HW_OPL4_ML {
            /* OPL4 can access 4 MB external ROM/SRAM */
            (*entry).mode |= 0o200;
            (*entry).size = 4 * 1024 * 1024;
        } else {
            /* OPL4-ML has 1 MB internal ROM */
            (*entry).size = 1 * 1024 * 1024;
        }
        (*entry).content = SNDRV_INFO_CONTENT_DATA;
        (*entry).c.ops = &snd_opl4_mem_proc_ops;
        (*entry).module = THIS_MODULE;
        (*entry).private_data = opl4 as *mut c_void;
    }
    (*opl4).proc_entry = entry;
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_opl4_free_proc(opl4: *mut snd_opl4) {
    snd_info_free_entry((*opl4).proc_entry);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
