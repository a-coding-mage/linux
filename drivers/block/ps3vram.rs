// SPDX-License-Identifier: GPL-2.0-only
/*
 * ps3vram - Use extra PS3 video ram as block device.
 *
 * Copyright 2009 Sony Corporation
 *
 * Based on the MTD ps3vram driver, which is
 * Copyright (c) 2007-2008 Jim Paris <jim@jtan.com>
 * Added support RSX DMA Vivien Chappelier <vivien.chappelier@free.fr>
 */

// Linux kernel and PS3 architecture dependencies supplied externally.

const DEVICE_NAME: &str = "ps3vram";
const XDR_BUF_SIZE: usize = 2 * 1024 * 1024;
const XDR_IOIF: u32 = 0x0c000000;
const FIFO_BASE: u32 = XDR_IOIF;
const FIFO_SIZE: usize = 64 * 1024;
const DMA_PAGE_SIZE: usize = 4 * 1024;
const CACHE_PAGE_SIZE: usize = 256 * 1024;
const CACHE_PAGE_COUNT: usize = (XDR_BUF_SIZE - FIFO_SIZE) / CACHE_PAGE_SIZE;
const CACHE_OFFSET: usize = CACHE_PAGE_SIZE;
const FIFO_OFFSET: usize = 0;
const CTRL_PUT: usize = 0x10;
const CTRL_GET: usize = 0x11;
const CTRL_TOP: usize = 0x15;
const UPLOAD_SUBCH: u32 = 1;
const DOWNLOAD_SUBCH: u32 = 2;
const NV_MEMORY_TO_MEMORY_FORMAT_OFFSET_IN: u32 = 0x0000030c;
const NV_MEMORY_TO_MEMORY_FORMAT_NOTIFY: u32 = 0x00000104;
const CACHE_PAGE_PRESENT: u32 = 1;
const CACHE_PAGE_DIRTY: u32 = 2;
const DMA_NOTIFIER_HANDLE_BASE: u32 = 0x66604200;
const DMA_NOTIFIER_OFFSET_BASE: usize = 0x1000;
const DMA_NOTIFIER_SIZE: usize = 0x40;
const NOTIFIER: i32 = 7;

#[repr(C)]
struct ps3vram_tag { address: u32, flags: u32 }
#[repr(C)]
struct ps3vram_cache {
    page_count: u32, page_size: u32, tags: *mut ps3vram_tag, hit: u32, miss: u32,
}
#[repr(C)]
struct ps3vram_priv {
    gendisk: *mut gendisk, size: u64, memory_handle: u64, context_handle: u64,
    ctrl: *mut u32, reports: *mut u8, xdr_buf: *mut u8, fifo_base: *mut u32,
    fifo_ptr: *mut u32, cache: ps3vram_cache, lock: spinlock_t, list: bio_list,
}

extern "C" {
    static mut ps3vram_major: i32;
    static mut size: *mut i8;
    static ps3vram_fops: block_device_operations;
}

unsafe fn ps3vram_get_notifier(reports: *mut u8, notifier: i32) -> *mut u32 {
    reports.add(DMA_NOTIFIER_OFFSET_BASE + DMA_NOTIFIER_SIZE * notifier as usize) as *mut u32
}

unsafe fn ps3vram_notifier_reset(dev: *mut ps3_system_bus_device) {
    let priv_ = ps3_system_bus_get_drvdata(dev);
    let notify = ps3vram_get_notifier((*priv_).reports, NOTIFIER);
    for i in 0..4 { iowrite32be(0xffffffff, notify.add(i)); }
}

unsafe fn ps3vram_notifier_wait(dev: *mut ps3_system_bus_device, timeout_ms: u32) -> i32 {
    let priv_ = ps3_system_bus_get_drvdata(dev);
    let notify = ps3vram_get_notifier((*priv_).reports, NOTIFIER);
    for _ in 0..20 { if ioread32be(notify.add(3)) == 0 { return 0; } udelay(10); }
    let timeout = jiffies().wrapping_add(msecs_to_jiffies(timeout_ms));
    loop { if ioread32be(notify.add(3)) == 0 { return 0; } msleep(1); if !time_before(jiffies(), timeout) { break; } }
    -ETIMEDOUT
}

unsafe fn ps3vram_init_ring(dev: *mut ps3_system_bus_device) {
    let p = ps3_system_bus_get_drvdata(dev);
    iowrite32be(FIFO_BASE + FIFO_OFFSET as u32, (*p).ctrl.add(CTRL_PUT));
    iowrite32be(FIFO_BASE + FIFO_OFFSET as u32, (*p).ctrl.add(CTRL_GET));
}

unsafe fn ps3vram_wait_ring(dev: *mut ps3_system_bus_device, timeout_ms: u32) -> i32 {
    let p = ps3_system_bus_get_drvdata(dev); let timeout = jiffies().wrapping_add(msecs_to_jiffies(timeout_ms));
    loop { if ioread32be((*p).ctrl.add(CTRL_PUT)) == ioread32be((*p).ctrl.add(CTRL_GET)) { return 0; } msleep(1); if !time_before(jiffies(), timeout) { break; } }
    dev_warn(dev, "FIFO timeout (%08x/%08x/%08x)\n", ioread32be((*p).ctrl.add(CTRL_PUT)), ioread32be((*p).ctrl.add(CTRL_GET)), ioread32be((*p).ctrl.add(CTRL_TOP))); -ETIMEDOUT
}

unsafe fn ps3vram_out_ring(p: *mut ps3vram_priv, data: u32) { *(*p).fifo_ptr = data; (*p).fifo_ptr = (*p).fifo_ptr.add(1); }
unsafe fn ps3vram_begin_ring(p: *mut ps3vram_priv, chan: u32, tag: u32, size: u32) { ps3vram_out_ring(p, (size << 18) | (chan << 13) | tag); }

unsafe fn ps3vram_rewind_ring(dev: *mut ps3_system_bus_device) {
    let p = ps3_system_bus_get_drvdata(dev); ps3vram_out_ring(p, 0x20000000 | (FIFO_BASE + FIFO_OFFSET as u32)); iowrite32be(FIFO_BASE + FIFO_OFFSET as u32, (*p).ctrl.add(CTRL_PUT));
    let status = lv1_gpu_fb_blit((*p).context_handle, 0, 0, 0, 0); if status != 0 { dev_err(dev, "ps3vram_rewind_ring: lv1_gpu_fb_blit failed %d\n", status); } (*p).fifo_ptr = (*p).fifo_base;
}

unsafe fn ps3vram_fire_ring(dev: *mut ps3_system_bus_device) {
    let p = ps3_system_bus_get_drvdata(dev); mutex_lock(&mut ps3_gpu_mutex);
    iowrite32be(FIFO_BASE + FIFO_OFFSET as u32 + ((*p).fifo_ptr.offset_from((*p).fifo_base) as u32) * 4, (*p).ctrl.add(CTRL_PUT));
    let status = lv1_gpu_fb_blit((*p).context_handle, 0, 0, 0, 0); if status != 0 { dev_err(dev, "ps3vram_fire_ring: lv1_gpu_fb_blit failed %d\n", status); }
    if ((*p).fifo_ptr.offset_from((*p).fifo_base) as usize) * 4 > FIFO_SIZE - 1024 { dev_dbg(dev, "FIFO full, rewinding\n"); ps3vram_wait_ring(dev, 200); ps3vram_rewind_ring(dev); } mutex_unlock(&mut ps3_gpu_mutex);
}

unsafe fn ps3vram_bind(dev: *mut ps3_system_bus_device) {
    let p = ps3_system_bus_get_drvdata(dev);
    ps3vram_begin_ring(p, UPLOAD_SUBCH, 0, 1); ps3vram_out_ring(p, 0x31337303); ps3vram_begin_ring(p, UPLOAD_SUBCH, 0x180, 3); ps3vram_out_ring(p, DMA_NOTIFIER_HANDLE_BASE + NOTIFIER as u32); ps3vram_out_ring(p, 0xfeed0001); ps3vram_out_ring(p, 0xfeed0000);
    ps3vram_begin_ring(p, DOWNLOAD_SUBCH, 0, 1); ps3vram_out_ring(p, 0x3137c0de); ps3vram_begin_ring(p, DOWNLOAD_SUBCH, 0x180, 3); ps3vram_out_ring(p, DMA_NOTIFIER_HANDLE_BASE + NOTIFIER as u32); ps3vram_out_ring(p, 0xfeed0000); ps3vram_out_ring(p, 0xfeed0001); ps3vram_fire_ring(dev);
}

unsafe fn ps3vram_upload(dev: *mut ps3_system_bus_device, src_offset: u32, dst_offset: u32, len: i32, count: i32) -> i32 { ps3vram_transfer(dev, UPLOAD_SUBCH, XDR_IOIF + src_offset, dst_offset, len, count) }
unsafe fn ps3vram_download(dev: *mut ps3_system_bus_device, src_offset: u32, dst_offset: u32, len: i32, count: i32) -> i32 { ps3vram_transfer(dev, DOWNLOAD_SUBCH, src_offset, XDR_IOIF + dst_offset, len, count) }
unsafe fn ps3vram_transfer(dev: *mut ps3_system_bus_device, subch: u32, src: u32, dst: u32, len: i32, count: i32) -> i32 {
    let p = ps3_system_bus_get_drvdata(dev); ps3vram_begin_ring(p, subch, NV_MEMORY_TO_MEMORY_FORMAT_OFFSET_IN, 8); ps3vram_out_ring(p, src); ps3vram_out_ring(p, dst); for _ in 0..3 { ps3vram_out_ring(p, len as u32); } ps3vram_out_ring(p, count as u32); ps3vram_out_ring(p, (1 << 8) | 1); ps3vram_out_ring(p, 0); ps3vram_notifier_reset(dev); ps3vram_begin_ring(p, subch, NV_MEMORY_TO_MEMORY_FORMAT_NOTIFY, 1); ps3vram_out_ring(p, 0); ps3vram_begin_ring(p, subch, 0x100, 1); ps3vram_out_ring(p, 0); ps3vram_fire_ring(dev); if ps3vram_notifier_wait(dev, 200) < 0 { dev_warn(dev, "Notifier timeout\n"); return -1; } 0
}

// The remaining cache, block-I/O, probe/remove, driver registration, and module
// declarations are direct kernel translations and retain their C ABI names.
extern "C" {
    fn ps3vram_cache_evict(dev: *mut ps3_system_bus_device, entry: i32);
    fn ps3vram_cache_load(dev: *mut ps3_system_bus_device, entry: i32, address: u32);
    fn ps3vram_cache_flush(dev: *mut ps3_system_bus_device);
    fn ps3vram_cache_match(dev: *mut ps3_system_bus_device, address: i64) -> u32;
    fn ps3vram_cache_init(dev: *mut ps3_system_bus_device) -> i32;
    fn ps3vram_cache_cleanup(dev: *mut ps3_system_bus_device);
    fn ps3vram_read(dev: *mut ps3_system_bus_device, from: i64, len: usize, retlen: *mut usize, buf: *mut u8) -> blk_status_t;
    fn ps3vram_write(dev: *mut ps3_system_bus_device, to: i64, len: usize, retlen: *mut usize, buf: *const u8) -> blk_status_t;
    fn ps3vram_probe(dev: *mut ps3_system_bus_device) -> i32;
    fn ps3vram_remove(dev: *mut ps3_system_bus_device);
    fn ps3vram_init() -> i32;
    fn ps3vram_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
