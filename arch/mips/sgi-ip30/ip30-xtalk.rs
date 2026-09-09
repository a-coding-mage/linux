// SPDX-License-Identifier: GPL-2.0
/*
 * ip30-xtalk.c - Very basic Crosstalk (XIO) detection support.
 *   Copyright (C) 2004-2007 Stanislaw Skowronek <skylark@unaligned.org>
 *   Copyright (C) 2009 Johannes Dickgreber <tanzy@gmx.de>
 *   Copyright (C) 2007, 2014-2016 Joshua Kinard <linux@kumba.dev>
 */

// Linux and architecture headers supplied by the surrounding kernel.

const IP30_WIDGET_XBOW: usize = 0x0; // XBow is always 0
const IP30_WIDGET_HEART: usize = 0x8; // HEART is always 8
const IP30_WIDGET_PCI_BASE: usize = 0xf; // BaseIO PCI is always 15
const XTALK_NODEV: u32 = 0xffff_ffff;
const XBOW_REG_LINK_STAT_0: usize = 0x114;
const XBOW_REG_LINK_BLK_SIZE: usize = 0x40;
const XBOW_REG_LINK_ALIVE: u32 = 0x8000_0000;
const HEART_INTR_ADDR: usize = 0x0000_0080;
const IP30_SWIN_SIZE: usize = 1 << 24;

#[inline]
const fn ip30_swin_base(widget: usize) -> usize {
    0x0000_0000_1000_0000usize | (widget << 24)
}

// IO_BASE, device constants, register types, and kernel interfaces are external dependencies.
extern "C" {
    static IO_BASE: usize;
    fn __raw_readl(addr: *const core::ffi::c_void) -> u32;
    fn platform_device_alloc(name: *const core::ffi::c_char, id: i32) -> *mut platform_device;
    fn platform_device_add_resources(pdev: *mut platform_device, res: *const resource, n: usize) -> i32;
    fn platform_device_add_data(pdev: *mut platform_device, data: *const core::ffi::c_void, size: usize) -> i32;
    fn platform_device_add(pdev: *mut platform_device) -> i32;
    fn platform_device_put(pdev: *mut platform_device);
    fn platform_device_unregister(pdev: *mut platform_device);
    fn kfree(ptr: *mut core::ffi::c_void);
    fn pr_warn(fmt: *const core::ffi::c_char, ...);
    fn pr_info(fmt: *const core::ffi::c_char, ...);
}

#[repr(C)] struct platform_device { _private: [u8; 0] }
#[repr(C)] struct bridge_regs { b_nic: u32 }
#[repr(C)] struct resource { start: usize, end: usize, name: *const core::ffi::c_char, flags: usize }
#[repr(C)] struct sgi_w1_platform_data { dev_id: [core::ffi::c_char; 32] }
#[repr(C)] struct xtalk_bridge_platform_data {
    bridge_addr: usize, intr_addr: usize, nasid: i32, masterwid: i32,
    mem: resource, mem_offset: usize, io: resource, io_offset: usize,
}

const PLATFORM_DEVID_AUTO: i32 = -1;
const IORESOURCE_MEM: usize = 0x0000_0200;
const IORESOURCE_IO: usize = 0x0000_0100;
const BRIDGE_DEVIO0: usize = 0x200000;
const WIDGET_ID: usize = 0x04;
const BRIDGE_WIDGET_PART_NUM: u32 = 0x0;
const XBRIDGE_WIDGET_PART_NUM: u32 = 0x1;
const fn xwidget_part_num(id: u32) -> u32 { (id >> 16) & 0xffff }

unsafe fn bridge_platform_create(widget: i32, masterwid: i32) {
    // The allocator and platform data layout are provided by the kernel.
    let wd = libc::calloc(1, core::mem::size_of::<sgi_w1_platform_data>()) as *mut sgi_w1_platform_data;
    if wd.is_null() { return; }
    let mut w1_res = resource { start: ip30_swin_base(widget as usize) + core::mem::offset_of!(bridge_regs, b_nic), end: 0, name: core::ptr::null(), flags: IORESOURCE_MEM };
    w1_res.end = w1_res.start + 3;
    let pdev_wd = platform_device_alloc(c"sgi_w1".as_ptr(), PLATFORM_DEVID_AUTO);
    if pdev_wd.is_null() { kfree(wd.cast()); return; }
    if platform_device_add_resources(pdev_wd, &w1_res, 1) != 0 || platform_device_add_data(pdev_wd, wd.cast(), core::mem::size_of_val(&*wd)) != 0 || platform_device_add(pdev_wd) != 0 { platform_device_put(pdev_wd); kfree(wd.cast()); return; }
    kfree(wd.cast());
    let bd = libc::calloc(1, core::mem::size_of::<xtalk_bridge_platform_data>()) as *mut xtalk_bridge_platform_data;
    if bd.is_null() { platform_device_unregister(pdev_wd); return; }
    let pdev_bd = platform_device_alloc(c"xtalk-bridge".as_ptr(), PLATFORM_DEVID_AUTO);
    if pdev_bd.is_null() { kfree(bd.cast()); platform_device_unregister(pdev_wd); return; }
    (*bd).bridge_addr = IO_BASE + ip30_swin_base(widget as usize);
    (*bd).intr_addr = HEART_INTR_ADDR; (*bd).nasid = 0; (*bd).masterwid = masterwid;
    (*bd).mem = resource { start: ip30_swin_base(widget as usize)+BRIDGE_DEVIO0, end: ip30_swin_base(widget as usize)+IP30_SWIN_SIZE-1, name: c"Bridge PCI MEM".as_ptr(), flags: IORESOURCE_MEM };
    (*bd).mem_offset = ip30_swin_base(widget as usize);
    (*bd).io = resource { start: ip30_swin_base(widget as usize)+BRIDGE_DEVIO0, end: ip30_swin_base(widget as usize)+IP30_SWIN_SIZE-1, name: c"Bridge PCI IO".as_ptr(), flags: IORESOURCE_IO };
    (*bd).io_offset = ip30_swin_base(widget as usize);
    if platform_device_add_data(pdev_bd, bd.cast(), core::mem::size_of_val(&*bd)) != 0 || platform_device_add(pdev_bd) != 0 { platform_device_put(pdev_bd); kfree(bd.cast()); platform_device_unregister(pdev_wd); return; }
    kfree(bd.cast());
}

unsafe fn xbow_widget_active(wid: i8) -> u32 {
    let addr = IO_BASE + ip30_swin_base(IP30_WIDGET_XBOW) + XBOW_REG_LINK_STAT_0 + XBOW_REG_LINK_BLK_SIZE * ((wid as isize - 8) as usize);
    if __raw_readl(addr as *const _) & XBOW_REG_LINK_ALIVE != 0 { 1 } else { 0 }
}

unsafe fn xtalk_init_widget(wid: i8, masterwid: i8) {
    if xbow_widget_active(wid) == 0 { return; }
    let widget_id = __raw_readl((IO_BASE + ip30_swin_base(wid as usize) + WIDGET_ID) as *const _);
    match xwidget_part_num(widget_id) { BRIDGE_WIDGET_PART_NUM | XBRIDGE_WIDGET_PART_NUM => bridge_platform_create(wid as i32, masterwid as i32), _ => {} }
}

unsafe fn ip30_xtalk_init() -> i32 {
    let mut i = IP30_WIDGET_PCI_BASE as i32;
    while i > IP30_WIDGET_HEART as i32 { xtalk_init_widget(i as i8, IP30_WIDGET_HEART as i8); i -= 1; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
