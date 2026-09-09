// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * memory mapped NVRAM
 *
 * (C) Copyright IBM Corp. 2005
 *
 * Authors : Utz Bacher <utz.bacher@de.ibm.com>
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

extern "C" {
    fn of_find_node_by_type(from: *mut device_node, typ: *const c_char) -> *mut device_node;
    fn of_find_compatible_node(
        from: *mut device_node,
        typ: *const c_char,
        compatible: *const c_char,
    ) -> *mut device_node;
    fn of_address_to_resource(np: *mut device_node, index: c_int, r: *mut resource) -> c_int;
    fn of_node_put(np: *mut device_node);
    fn ioremap(addr: c_ulong, size: c_ulong) -> *mut c_void;
    fn ioread8(addr: *mut c_void) -> u8;
    fn iowrite8(value: u8, addr: *mut c_void);
    fn memcpy_fromio(to: *mut c_char, from: *mut c_void, count: usize);
    fn memcpy_toio(to: *mut c_void, from: *const c_char, count: usize);
    fn printk(fmt: *const c_char, ...) -> c_int;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    static mut mmio_nvram_lock: spinlock_t;
    static mut ppc_md: ppc_machine_desc;
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: c_ulong,
    pub end: c_ulong,
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ppc_machine_desc {
    pub nvram_read_val: Option<unsafe extern "C" fn(c_int) -> u8>,
    pub nvram_write_val: Option<unsafe extern "C" fn(c_int, u8)>,
    pub nvram_read: Option<unsafe extern "C" fn(*mut c_char, usize, *mut c_long) -> isize>,
    pub nvram_write: Option<unsafe extern "C" fn(*mut c_char, usize, *mut c_long) -> isize>,
    pub nvram_size: Option<unsafe extern "C" fn() -> isize>,
}

static mut mmio_nvram_start: *mut c_void = core::ptr::null_mut();
static mut mmio_nvram_len: c_long = 0;

unsafe fn mmio_nvram_read(buf: *mut c_char, mut count: usize, index: *mut c_long) -> isize {
    let mut flags: c_ulong = 0;

    if *index >= mmio_nvram_len {
        return 0;
    }
    if *index + count as c_long > mmio_nvram_len {
        count = (mmio_nvram_len - *index) as usize;
    }

    spin_lock_irqsave(&mut mmio_nvram_lock, &mut flags);
    memcpy_fromio(buf, (mmio_nvram_start as *mut u8).add(*index as usize) as *mut c_void, count);
    spin_unlock_irqrestore(&mut mmio_nvram_lock, flags);

    *index += count as c_long;
    count as isize
}

unsafe fn mmio_nvram_read_val(addr: c_int) -> u8 {
    let mut flags: c_ulong = 0;
    if addr as c_long >= mmio_nvram_len {
        return 0xff;
    }
    spin_lock_irqsave(&mut mmio_nvram_lock, &mut flags);
    let val = ioread8((mmio_nvram_start as *mut u8).add(addr as usize) as *mut c_void);
    spin_unlock_irqrestore(&mut mmio_nvram_lock, flags);
    val
}

unsafe fn mmio_nvram_write(buf: *mut c_char, mut count: usize, index: *mut c_long) -> isize {
    let mut flags: c_ulong = 0;
    if *index >= mmio_nvram_len {
        return 0;
    }
    if *index + count as c_long > mmio_nvram_len {
        count = (mmio_nvram_len - *index) as usize;
    }
    spin_lock_irqsave(&mut mmio_nvram_lock, &mut flags);
    memcpy_toio((mmio_nvram_start as *mut u8).add(*index as usize) as *mut c_void, buf, count);
    spin_unlock_irqrestore(&mut mmio_nvram_lock, flags);
    *index += count as c_long;
    count as isize
}

unsafe fn mmio_nvram_write_val(addr: c_int, val: u8) {
    let mut flags: c_ulong = 0;
    if (addr as c_long) < mmio_nvram_len {
        spin_lock_irqsave(&mut mmio_nvram_lock, &mut flags);
        iowrite8(val, (mmio_nvram_start as *mut u8).add(addr as usize) as *mut c_void);
        spin_unlock_irqrestore(&mut mmio_nvram_lock, flags);
    }
}

unsafe fn mmio_nvram_get_size() -> isize {
    mmio_nvram_len as isize
}

pub unsafe extern "C" fn mmio_nvram_init() -> c_int {
    let mut nvram_node = of_find_node_by_type(core::ptr::null_mut(), b"nvram\0".as_ptr() as *const c_char);
    if nvram_node.is_null() {
        nvram_node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"nvram\0".as_ptr() as *const c_char);
    }
    if nvram_node.is_null() {
        printk(b"nvram: no node found in device-tree\n\0".as_ptr() as *const c_char);
        return -19;
    }

    let mut r = resource { start: 0, end: 0, _private: [] };
    let mut ret = of_address_to_resource(nvram_node, 0, &mut r);
    if ret != 0 {
        printk(b"nvram: failed to get address (err %d)\n\0".as_ptr() as *const c_char, ret);
        of_node_put(nvram_node);
        return ret;
    }
    let nvram_addr = r.start;
    mmio_nvram_len = r.end - r.start + 1;
    if mmio_nvram_len == 0 || nvram_addr == 0 {
        printk(b"nvram: address or length is 0\n\0".as_ptr() as *const c_char);
        ret = -5;
        of_node_put(nvram_node);
        return ret;
    }
    mmio_nvram_start = ioremap(nvram_addr, mmio_nvram_len as c_ulong);
    if mmio_nvram_start.is_null() {
        printk(b"nvram: failed to ioremap\n\0".as_ptr() as *const c_char);
        ret = -12;
        of_node_put(nvram_node);
        return ret;
    }
    printk(b"mmio NVRAM, %luk at 0x%lx mapped to %p\n\0".as_ptr() as *const c_char,
           (mmio_nvram_len as c_ulong) >> 10, nvram_addr, mmio_nvram_start);
    ppc_md.nvram_read_val = Some(mmio_nvram_read_val);
    ppc_md.nvram_write_val = Some(mmio_nvram_write_val);
    ppc_md.nvram_read = Some(mmio_nvram_read);
    ppc_md.nvram_write = Some(mmio_nvram_write);
    ppc_md.nvram_size = Some(mmio_nvram_get_size);
    of_node_put(nvram_node);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
