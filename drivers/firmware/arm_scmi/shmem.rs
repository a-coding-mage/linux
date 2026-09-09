// SPDX-License-Identifier: GPL-2.0
/*
 * For transport using shared mem structure.
 *
 * Copyright (C) 2019-2024 ARM Ltd.
 */

use core::ffi::{c_char, c_void};

pub const SCMI_SHMEM_LAYOUT_OVERHEAD: usize = 24;
pub const SCMI_SHMEM_CHAN_STAT_CHANNEL_ERROR: u32 = 1 << 1;
pub const SCMI_SHMEM_CHAN_STAT_CHANNEL_FREE: u32 = 1 << 0;
pub const SCMI_SHMEM_FLAG_INTR_ENABLED: u32 = 1 << 0;

#[repr(C)]
pub struct scmi_shared_mem {
    pub reserved: u32,
    pub channel_status: u32,
    pub reserved1: [u32; 2],
    pub flags: u32,
    pub length: u32,
    pub msg_header: u32,
    pub msg_payload: [u8; 0],
}

#[repr(C)]
pub struct scmi_shmem_io_ops {
    pub fromio: Option<unsafe extern "C" fn(*mut c_void, *const c_void, usize)>,
    pub toio: Option<unsafe extern "C" fn(*mut c_void, *const c_void, usize)>,
}

pub type shmem_copy_fromio_t = unsafe extern "C" fn(*mut c_void, *const c_void, usize);
pub type shmem_copy_toio_t = unsafe extern "C" fn(*mut c_void, *const c_void, usize);

#[repr(C)]
pub struct scmi_xfer_hdr {
    pub poll_completion: bool,
    pub seq: u16,
    pub status: u32,
}

#[repr(C)]
pub struct scmi_xfer_buf {
    pub buf: *mut c_void,
    pub len: usize,
}

#[repr(C)]
pub struct scmi_xfer {
    pub hdr: scmi_xfer_hdr,
    pub tx: scmi_xfer_buf,
    pub rx: scmi_xfer_buf,
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct scmi_chan_info {
    pub dev: *mut device,
    pub rx_timeout_ms: u64,
    pub max_msg_size: usize,
}

#[repr(C)]
pub struct resource {
    pub start: usize,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct scmi_shared_mem_operations {
    pub tx_prepare: Option<unsafe extern "C" fn(*mut scmi_shared_mem, *mut scmi_xfer, *mut scmi_chan_info, shmem_copy_toio_t)>,
    pub read_header: Option<unsafe extern "C" fn(*mut scmi_shared_mem) -> u32>,
    pub fetch_response: Option<unsafe extern "C" fn(*mut scmi_shared_mem, *mut scmi_xfer, shmem_copy_fromio_t)>,
    pub fetch_notification: Option<unsafe extern "C" fn(*mut scmi_shared_mem, usize, *mut scmi_xfer, shmem_copy_fromio_t)>,
    pub clear_channel: Option<unsafe extern "C" fn(*mut scmi_shared_mem)>,
    pub poll_done: Option<unsafe extern "C" fn(*mut scmi_shared_mem, *mut scmi_xfer) -> bool>,
    pub channel_free: Option<unsafe extern "C" fn(*mut scmi_shared_mem) -> bool>,
    pub channel_intr_enabled: Option<unsafe extern "C" fn(*mut scmi_shared_mem) -> bool>,
    pub setup_iomap: Option<unsafe extern "C" fn(*mut scmi_chan_info, *mut device, bool, *mut resource, *mut *mut scmi_shmem_io_ops) -> *mut c_void>,
}

extern "C" {
    fn ioread32(addr: *const u32) -> u32;
    fn iowrite32(value: u32, addr: *mut u32);
    fn __ioread32_copy(to: *mut c_void, from: *const c_void, count: usize);
    fn __iowrite32_copy(to: *mut c_void, from: *const c_void, count: usize);
    fn memcpy_fromio(to: *mut c_void, from: *const c_void, count: usize);
    fn memcpy_toio(to: *mut c_void, from: *const c_void, count: usize);
    fn ktime_get() -> i64;
    fn ktime_add_ms(ktime: i64, ms: u64) -> i64;
    fn ktime_after(a: i64, b: i64) -> bool;
    fn pack_scmi_header(hdr: *const scmi_xfer_hdr) -> u32;
    fn msg_xtract_token(header: u32) -> u16;
    fn of_parse_phandle(node: *mut device_node, name: *const c_char, index: i32) -> *mut device_node;
    fn of_device_is_compatible(node: *mut device_node, compatible: *const c_char) -> bool;
    fn of_address_to_resource(node: *mut device_node, index: i32, res: *mut resource) -> i32;
    fn resource_size(res: *const resource) -> usize;
    fn devm_ioremap(dev: *mut device, start: usize, size: usize) -> *mut c_void;
    fn of_property_read_u32(node: *mut device_node, name: *const c_char, value: *mut u32) -> i32;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn warn_on(condition: bool);
    fn warn_on_once(condition: bool);
}

unsafe extern "C" fn shmem_memcpy_fromio32(to: *mut c_void, from: *const c_void, count: usize) {
    warn_on((from as usize) % 4 != 0 || (to as usize) % 4 != 0 || count % 4 != 0);
    __ioread32_copy(to, from, count / 4);
}

unsafe extern "C" fn shmem_memcpy_toio32(to: *mut c_void, from: *const c_void, count: usize) {
    warn_on((to as usize) % 4 != 0 || (from as usize) % 4 != 0 || count % 4 != 0);
    __iowrite32_copy(to, from, count / 4);
}

static mut SHMEM_IO_OPS32: scmi_shmem_io_ops = scmi_shmem_io_ops {
    fromio: Some(shmem_memcpy_fromio32),
    toio: Some(shmem_memcpy_toio32),
};

unsafe extern "C" fn shmem_memcpy_fromio(to: *mut c_void, from: *const c_void, count: usize) {
    memcpy_fromio(to, from, count);
}

unsafe extern "C" fn shmem_memcpy_toio(to: *mut c_void, from: *const c_void, count: usize) {
    memcpy_toio(to, from, count);
}

static mut SHMEM_IO_OPS_DEFAULT: scmi_shmem_io_ops = scmi_shmem_io_ops {
    fromio: Some(shmem_memcpy_fromio),
    toio: Some(shmem_memcpy_toio),
};

unsafe extern "C" fn shmem_tx_prepare(shmem: *mut scmi_shared_mem, xfer: *mut scmi_xfer, cinfo: *mut scmi_chan_info, copy_toio: shmem_copy_toio_t) {
    let stop = ktime_add_ms(ktime_get(), 2 * (*cinfo).rx_timeout_ms);
    while (ioread32(&(*shmem).channel_status) & SCMI_SHMEM_CHAN_STAT_CHANNEL_FREE == 0 && !ktime_after(ktime_get(), stop)) {}
    if ioread32(&(*shmem).channel_status) & SCMI_SHMEM_CHAN_STAT_CHANNEL_FREE == 0 {
        warn_on_once(true);
        dev_err((*cinfo).dev, b"Timeout waiting for a free TX channel !\0".as_ptr() as *const c_char);
        return;
    }
    iowrite32(0, &mut (*shmem).channel_status);
    iowrite32(if (*xfer).hdr.poll_completion { 0 } else { SCMI_SHMEM_FLAG_INTR_ENABLED }, &mut (*shmem).flags);
    iowrite32(8 + (*xfer).tx.len as u32, &mut (*shmem).length);
    iowrite32(pack_scmi_header(&(*xfer).hdr), &mut (*shmem).msg_header);
    if !(*xfer).tx.buf.is_null() { copy_toio((*shmem).msg_payload.as_mut_ptr() as *mut c_void, (*xfer).tx.buf, (*xfer).tx.len); }
}

unsafe extern "C" fn shmem_read_header(shmem: *mut scmi_shared_mem) -> u32 { ioread32(&(*shmem).msg_header) }

unsafe extern "C" fn shmem_fetch_response(shmem: *mut scmi_shared_mem, xfer: *mut scmi_xfer, copy_fromio: shmem_copy_fromio_t) {
    let len = ioread32(&(*shmem).length) as usize;
    (*xfer).hdr.status = ioread32((*shmem).msg_payload.as_ptr() as *const u32);
    (*xfer).rx.len = core::cmp::min((*xfer).rx.len, if len > 8 { len - 8 } else { 0 });
    copy_fromio((*xfer).rx.buf, (*shmem).msg_payload.as_ptr().add(4) as *const c_void, (*xfer).rx.len);
}

unsafe extern "C" fn shmem_fetch_notification(shmem: *mut scmi_shared_mem, max_len: usize, xfer: *mut scmi_xfer, copy_fromio: shmem_copy_fromio_t) {
    let len = ioread32(&(*shmem).length) as usize;
    (*xfer).rx.len = core::cmp::min(max_len, if len > 4 { len - 4 } else { 0 });
    copy_fromio((*xfer).rx.buf, (*shmem).msg_payload.as_ptr() as *const c_void, (*xfer).rx.len);
}

unsafe extern "C" fn shmem_clear_channel(shmem: *mut scmi_shared_mem) { iowrite32(SCMI_SHMEM_CHAN_STAT_CHANNEL_FREE, &mut (*shmem).channel_status); }

unsafe extern "C" fn shmem_poll_done(shmem: *mut scmi_shared_mem, xfer: *mut scmi_xfer) -> bool {
    if (*xfer).hdr.seq != msg_xtract_token(ioread32(&(*shmem).msg_header)) { return false; }
    ioread32(&(*shmem).channel_status) & (SCMI_SHMEM_CHAN_STAT_CHANNEL_ERROR | SCMI_SHMEM_CHAN_STAT_CHANNEL_FREE) != 0
}

unsafe extern "C" fn shmem_channel_free(shmem: *mut scmi_shared_mem) -> bool { ioread32(&(*shmem).channel_status) & SCMI_SHMEM_CHAN_STAT_CHANNEL_FREE != 0 }
unsafe extern "C" fn shmem_channel_intr_enabled(shmem: *mut scmi_shared_mem) -> bool { ioread32(&(*shmem).flags) & SCMI_SHMEM_FLAG_INTR_ENABLED != 0 }

unsafe extern "C" fn shmem_setup_iomap(cinfo: *mut scmi_chan_info, dev: *mut device, tx: bool, res: *mut resource, ops: *mut *mut scmi_shmem_io_ops) -> *mut c_void {
    let idx = if tx { 0 } else { 1 };
    let cdev = (*cinfo).dev;
    let shmem = of_parse_phandle((*cdev).of_node, b"shmem\0".as_ptr() as *const c_char, idx);
    if shmem.is_null() { return (-19isize) as *mut c_void; }
    if !of_device_is_compatible(shmem, b"arm,scmi-shmem\0".as_ptr() as *const c_char) { return (-6isize) as *mut c_void; }
    let mut local_res = resource { start: 0 };
    let target_res = if res.is_null() { &mut local_res } else { &mut *res };
    let ret = of_address_to_resource(shmem, 0, target_res);
    if ret != 0 { dev_err(cdev, b"failed to get SCMI shared memory\n\0".as_ptr() as *const c_char); return ret as isize as *mut c_void; }
    let size = resource_size(target_res);
    if (*cinfo).max_msg_size + SCMI_SHMEM_LAYOUT_OVERHEAD > size { dev_err(dev, b"misconfigured SCMI shared memory\0".as_ptr() as *const c_char); return (-28isize) as *mut c_void; }
    let addr = devm_ioremap(dev, target_res.start, size);
    if addr.is_null() { dev_err(dev, b"failed to ioremap SCMI shared memory\n\0".as_ptr() as *const c_char); return (-99isize) as *mut c_void; }
    let mut reg_io_width = 0u32;
    of_property_read_u32(shmem, b"reg-io-width\0".as_ptr() as *const c_char, &mut reg_io_width);
    *ops = if reg_io_width == 4 { &mut SHMEM_IO_OPS32 } else { &mut SHMEM_IO_OPS_DEFAULT };
    addr
}

pub fn scmi_shared_mem_operations_get() -> *const scmi_shared_mem_operations { &SCMI_SHMEM_OPS }

static SCMI_SHMEM_OPS: scmi_shared_mem_operations = scmi_shared_mem_operations {
    tx_prepare: Some(shmem_tx_prepare), read_header: Some(shmem_read_header), fetch_response: Some(shmem_fetch_response), fetch_notification: Some(shmem_fetch_notification), clear_channel: Some(shmem_clear_channel), poll_done: Some(shmem_poll_done), channel_free: Some(shmem_channel_free), channel_intr_enabled: Some(shmem_channel_intr_enabled), setup_iomap: Some(shmem_setup_iomap),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
