// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018-2023 Intel Corporation
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;

type u8 = core::ffi::c_uchar;
type u32 = c_uint;
type size_t = usize;
type ssize_t = isize;
type loff_t = i64;
type umode_t = u16;
type gfp_t = c_uint;

const EINVAL: c_int = 22;
const EFAULT: c_int = 14;
const ENOMEM: c_int = 12;

extern "C" {
    static SOF_IPC4_DEBUG_SLOT_TELEMETRY: u32;
    static SOF_IPC4_DEBUG_SLOT_SIZE: size_t;
    static SOF_DFSENTRY_TYPE_IOMEM: c_int;
    static SOF_DEBUGFS_ACCESS_ALWAYS: c_int;
    static GFP_KERNEL: gfp_t;

    static simple_open: unsafe extern "C" fn(
        inode: *mut c_void,
        file: *mut file,
    ) -> c_int;

    fn sof_ipc4_find_debug_slot_offset_by_type(
        sdev: *mut snd_sof_dev,
        type_: u32,
    ) -> size_t;
    fn kzalloc(size: size_t, flags: gfp_t) -> *mut c_void;
    fn kfree(ptr: *const c_void);
    fn memcpy_fromio(to: *mut c_void, from: *const c_void, count: size_t);
    fn copy_to_user(to: *mut c_char, from: *const c_void, n: size_t) -> size_t;
    fn devm_kzalloc(dev: *mut c_void, size: size_t, flags: gfp_t) -> *mut c_void;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn debugfs_create_file(
        name: *const c_char,
        mode: umode_t,
        parent: *mut c_void,
        data: *mut c_void,
        fops: *const file_operations,
    ) -> *mut c_void;
}

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct file {
    private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_sof_dev {
    dev: *mut c_void,
    bar: [*mut u8; 8],
    mailbox_bar: usize,
    dfsentry_list: list_head,
    debugfs_root: *mut c_void,
}

#[repr(C)]
pub struct snd_sof_dfsentry {
    list: list_head,
    type_: c_int,
    size: size_t,
    access_type: c_int,
    sdev: *mut snd_sof_dev,
}

#[repr(C)]
pub struct file_operations {
    open: Option<unsafe extern "C" fn(*mut c_void, *mut file) -> c_int>,
    read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
}

unsafe fn sof_ipc4_query_exception_address(sdev: *mut snd_sof_dev) -> *mut c_void {
    let type_: u32 = SOF_IPC4_DEBUG_SLOT_TELEMETRY;
    let telemetry_slot_offset: size_t;
    let offset: u32;

    telemetry_slot_offset = sof_ipc4_find_debug_slot_offset_by_type(sdev, type_);
    if telemetry_slot_offset == 0 {
        return core::ptr::null_mut();
    }

    /* skip the first separator magic number */
    offset = (telemetry_slot_offset + size_of::<u32>()) as u32;

    (*sdev).bar[(*sdev).mailbox_bar].add(offset as usize) as *mut c_void
}

unsafe extern "C" fn sof_telemetry_entry_read(
    file: *mut file,
    buffer: *mut c_char,
    mut count: size_t,
    ppos: *mut loff_t,
) -> ssize_t {
    let dfse: *mut snd_sof_dfsentry = (*file).private_data as *mut snd_sof_dfsentry;
    let sdev: *mut snd_sof_dev = (*dfse).sdev;
    let io_addr: *mut c_void;
    let pos: loff_t = *ppos;
    let size_ret: size_t;
    let buf: *mut u8;

    if pos < 0 {
        return -(EINVAL as ssize_t);
    }
    /* skip the first separator magic number */
    if pos as size_t >= SOF_IPC4_DEBUG_SLOT_SIZE - 4 || count == 0 {
        return 0;
    }
    if count > SOF_IPC4_DEBUG_SLOT_SIZE - 4 - pos as size_t {
        count = SOF_IPC4_DEBUG_SLOT_SIZE - 4 - pos as size_t;
    }

    io_addr = sof_ipc4_query_exception_address(sdev);
    if io_addr.is_null() {
        return -(EFAULT as ssize_t);
    }

    buf = kzalloc(SOF_IPC4_DEBUG_SLOT_SIZE - 4, GFP_KERNEL) as *mut u8;
    if buf.is_null() {
        return -(ENOMEM as ssize_t);
    }

    memcpy_fromio(buf as *mut c_void, io_addr, SOF_IPC4_DEBUG_SLOT_SIZE - 4);
    size_ret = copy_to_user(buffer, buf.add(pos as usize) as *const c_void, count);
    if size_ret != 0 {
        kfree(buf as *const c_void);
        return -(EFAULT as ssize_t);
    }

    *ppos = pos + count as loff_t;
    kfree(buf as *const c_void);

    count as ssize_t
}

static sof_telemetry_fops: file_operations = file_operations {
    open: Some(simple_open),
    read: Some(sof_telemetry_entry_read),
};

#[no_mangle]
pub unsafe extern "C" fn sof_ipc4_create_exception_debugfs_node(sdev: *mut snd_sof_dev) {
    let dfse: *mut snd_sof_dfsentry;

    dfse = devm_kzalloc(
        (*sdev).dev,
        size_of::<snd_sof_dfsentry>(),
        GFP_KERNEL,
    ) as *mut snd_sof_dfsentry;
    if dfse.is_null() {
        return;
    }

    (*dfse).type_ = SOF_DFSENTRY_TYPE_IOMEM;
    (*dfse).size = SOF_IPC4_DEBUG_SLOT_SIZE - 4;
    (*dfse).access_type = SOF_DEBUGFS_ACCESS_ALWAYS;
    (*dfse).sdev = sdev;

    list_add(&mut (*dfse).list, &mut (*sdev).dfsentry_list);

    debugfs_create_file(
        b"exception\0".as_ptr() as *const c_char,
        0o444,
        (*sdev).debugfs_root,
        dfse as *mut c_void,
        &sof_telemetry_fops,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
