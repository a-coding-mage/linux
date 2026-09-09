// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 Imagination Technologies
 * Authors:  Will Thomas, James Hartley
 *
 * Interface structure taken from omap-sham driver
 *
 * C kernel dependencies are intentionally left as external Rust symbols.
 */

const CR_RESET: u32 = 0;
const CR_RESET_SET: u32 = 1;
const CR_RESET_UNSET: u32 = 0;
const CR_MESSAGE_LENGTH_H: u32 = 0x4;
const CR_MESSAGE_LENGTH_L: u32 = 0x8;
const CR_CONTROL: u32 = 0xc;
const CR_CONTROL_BYTE_ORDER_3210: u32 = 0;
const CR_CONTROL_BYTE_ORDER_0123: u32 = 1;
const CR_CONTROL_BYTE_ORDER_2310: u32 = 2;
const CR_CONTROL_BYTE_ORDER_1032: u32 = 3;
const CR_CONTROL_BYTE_ORDER_SHIFT: u32 = 8;
const CR_CONTROL_ALGO_MD5: u32 = 0;
const CR_CONTROL_ALGO_SHA1: u32 = 1;
const CR_CONTROL_ALGO_SHA224: u32 = 2;
const CR_CONTROL_ALGO_SHA256: u32 = 3;
const CR_INTSTAT: u32 = 0x10;
const CR_INTENAB: u32 = 0x14;
const CR_INTCLEAR: u32 = 0x18;
const CR_INT_RESULTS_AVAILABLE: u32 = 1 << 0;
const CR_INT_NEW_RESULTS_SET: u32 = 1 << 1;
const CR_INT_RESULT_READ_ERR: u32 = 1 << 2;
const CR_INT_MESSAGE_WRITE_ERROR: u32 = 1 << 3;
const CR_INT_STATUS: u32 = 1 << 8;
const CR_RESULT_QUEUE: u32 = 0x1c;
const CR_RSD0: u32 = 0x40;
const CR_CORE_REV: u32 = 0x50;
const CR_CORE_DES1: u32 = 0x60;
const CR_CORE_DES2: u32 = 0x70;

const DRIVER_FLAGS_BUSY: u64 = 1 << 0;
const DRIVER_FLAGS_FINAL: u64 = 1 << 1;
const DRIVER_FLAGS_DMA_ACTIVE: u64 = 1 << 2;
const DRIVER_FLAGS_OUTPUT_READY: u64 = 1 << 3;
const DRIVER_FLAGS_INIT: u64 = 1 << 4;
const DRIVER_FLAGS_CPU: u64 = 1 << 5;
const DRIVER_FLAGS_DMA_READY: u64 = 1 << 6;
const DRIVER_FLAGS_ERROR: u64 = 1 << 7;
const DRIVER_FLAGS_SG: u64 = 1 << 8;
const DRIVER_FLAGS_SHA1: u64 = 1 << 18;
const DRIVER_FLAGS_SHA224: u64 = 1 << 19;
const DRIVER_FLAGS_SHA256: u64 = 1 << 20;
const DRIVER_FLAGS_MD5: u64 = 1 << 21;
const IMG_HASH_QUEUE_LENGTH: usize = 20;
const IMG_HASH_DMA_BURST: u32 = 4;
const IMG_HASH_DMA_THRESHOLD: usize = 64;
#[cfg(target_endian = "little")]
const IMG_HASH_BYTE_ORDER: u32 = CR_CONTROL_BYTE_ORDER_3210;
#[cfg(not(target_endian = "little"))]
const IMG_HASH_BYTE_ORDER: u32 = CR_CONTROL_BYTE_ORDER_0123;

// External kernel types and functions are supplied by the surrounding translation.
extern "C" {
    fn readl_relaxed(addr: *mut core::ffi::c_void) -> u32;
    fn writel_relaxed(value: u32, addr: *mut core::ffi::c_void);
}

#[repr(C)]
struct img_hash_dev {
    list: list_head,
    dev: *mut device,
    hash_clk: *mut clk,
    sys_clk: *mut clk,
    io_base: *mut core::ffi::c_void,
    bus_addr: phys_addr_t,
    cpu_addr: *mut core::ffi::c_void,
    lock: spinlock_t,
    err: i32,
    done_task: tasklet_struct,
    dma_task: tasklet_struct,
    flags: u64,
    queue: crypto_queue,
    req: *mut ahash_request,
    dma_lch: *mut dma_chan,
}

#[repr(C)]
struct img_hash_request_ctx {
    hdev: *mut img_hash_dev,
    digest: [u8; SHA256_DIGEST_SIZE],
    flags: u64,
    digsize: usize,
    dma_addr: dma_addr_t,
    dma_ct: usize,
    sgfirst: *mut scatterlist,
    sg: *mut scatterlist,
    nents: usize,
    offset: usize,
    total: u32,
    sent: usize,
    op: u64,
    bufcnt: usize,
    fallback_req: ahash_request,
    buffer: [u8; IMG_HASH_DMA_THRESHOLD],
}

#[repr(C)]
struct img_hash_ctx { hdev: *mut img_hash_dev, flags: u64, fallback: *mut crypto_ahash }

#[repr(C)]
struct img_hash_drv { dev_list: list_head, lock: spinlock_t }

static mut img_hash: img_hash_drv = img_hash_drv {
    dev_list: unsafe { core::mem::zeroed() },
    lock: unsafe { core::mem::zeroed() },
};

unsafe fn img_hash_read(hdev: *mut img_hash_dev, offset: u32) -> u32 {
    readl_relaxed((*hdev).io_base.add(offset as usize))
}
unsafe fn img_hash_write(hdev: *mut img_hash_dev, offset: u32, value: u32) {
    writel_relaxed(value, (*hdev).io_base.add(offset as usize));
}
unsafe fn img_hash_read_result_queue(hdev: *mut img_hash_dev) -> u32 {
    img_hash_read(hdev, CR_RESULT_QUEUE).to_be()
}

unsafe fn img_hash_start(hdev: *mut img_hash_dev, dma: bool) {
    let ctx = ahash_request_ctx((*hdev).req);
    let mut cr = IMG_HASH_BYTE_ORDER << CR_CONTROL_BYTE_ORDER_SHIFT;
    if (*ctx).flags & DRIVER_FLAGS_MD5 != 0 { cr |= CR_CONTROL_ALGO_MD5; }
    else if (*ctx).flags & DRIVER_FLAGS_SHA1 != 0 { cr |= CR_CONTROL_ALGO_SHA1; }
    else if (*ctx).flags & DRIVER_FLAGS_SHA224 != 0 { cr |= CR_CONTROL_ALGO_SHA224; }
    else if (*ctx).flags & DRIVER_FLAGS_SHA256 != 0 { cr |= CR_CONTROL_ALGO_SHA256; }
    img_hash_write(hdev, CR_CONTROL, cr);
    if !dma { let _ = img_hash_read(hdev, CR_CONTROL); }
}

unsafe fn img_hash_xmit_cpu(hdev: *mut img_hash_dev, buf: *const u8, length: usize, final_: i32) -> i32 {
    if final_ != 0 { (*hdev).flags |= DRIVER_FLAGS_FINAL; }
    let len32 = (length + 3) / 4;
    let buffer = buf as *const u32;
    for count in 0..len32 { writel_relaxed(*buffer.add(count), (*hdev).cpu_addr); }
    -115
}

unsafe fn img_hash_dma_callback(data: *mut core::ffi::c_void) {
    let hdev = data as *mut img_hash_dev;
    let ctx = ahash_request_ctx((*hdev).req);
    if (*ctx).bufcnt != 0 {
        img_hash_xmit_cpu(hdev, (*ctx).buffer.as_ptr(), (*ctx).bufcnt, 0);
        (*ctx).bufcnt = 0;
    }
    if !(*ctx).sg.is_null() { tasklet_schedule(&mut (*hdev).dma_task); }
}

// The remaining driver entry points retain the original kernel call structure.
// External helper declarations allow the source-level behavior to remain literal.
unsafe fn img_hash_finish(req: *mut ahash_request) -> i32 {
    let ctx = ahash_request_ctx(req);
    if (*req).result.is_null() { return -22; }
    core::ptr::copy_nonoverlapping((*ctx).digest.as_ptr(), (*req).result, (*ctx).digsize);
    0
}

unsafe fn img_hash_copy_hash(req: *mut ahash_request) {
    let ctx = ahash_request_ctx(req);
    let hash = (*ctx).digest.as_mut_ptr() as *mut u32;
    let mut i = (*ctx).digsize / 4;
    while i != 0 { i -= 1; *hash.add(i) = img_hash_read_result_queue((*ctx).hdev); }
}

unsafe fn img_hash_finish_req(req: *mut ahash_request, mut err: i32) {
    let ctx = ahash_request_ctx(req);
    let hdev = (*ctx).hdev;
    if err == 0 {
        img_hash_copy_hash(req);
        if (*hdev).flags & DRIVER_FLAGS_FINAL != 0 { err = img_hash_finish(req); }
    } else { (*ctx).flags |= DRIVER_FLAGS_ERROR; }
    (*hdev).flags &= !(DRIVER_FLAGS_DMA_READY | DRIVER_FLAGS_OUTPUT_READY | DRIVER_FLAGS_CPU | DRIVER_FLAGS_BUSY | DRIVER_FLAGS_FINAL);
    if !(*req).base.complete.is_none() { ahash_request_complete(req, err); }
}

// Kernel registration, DMA, queue, IRQ, probe/remove, PM, and algorithm descriptors
// are declarations below because their definitions depend on the kernel translation.
extern "C" {
    fn img_hash_write_via_dma(hdev: *mut img_hash_dev) -> i32;
    fn img_hash_write_via_cpu(hdev: *mut img_hash_dev) -> i32;
    fn img_hash_process_data(hdev: *mut img_hash_dev) -> i32;
    fn img_hash_hw_init(hdev: *mut img_hash_dev) -> i32;
    fn img_hash_handle_queue(hdev: *mut img_hash_dev, req: *mut ahash_request) -> i32;
    fn img_hash_init(req: *mut ahash_request) -> i32;
    fn img_hash_update(req: *mut ahash_request) -> i32;
    fn img_hash_final(req: *mut ahash_request) -> i32;
    fn img_hash_finup(req: *mut ahash_request) -> i32;
    fn img_hash_digest(req: *mut ahash_request) -> i32;
    fn img_hash_probe(pdev: *mut platform_device) -> i32;
    fn img_hash_remove(pdev: *mut platform_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
