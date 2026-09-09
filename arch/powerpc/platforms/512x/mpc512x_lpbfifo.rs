// SPDX-License-Identifier: GPL-2.0-only
/*
 * The driver for Freescale MPC512x LocalPlus Bus FIFO
 * (called SCLPC in the Reference Manual).
 *
 * Copyright (C) 2013-2015 Alexander Popov <alex.popov@linux.com>.
 */

// Linux kernel dependencies supplied by the surrounding translation.

const DRV_NAME: &str = "mpc512x_lpbfifo";

#[repr(C)]
struct CsRange {
    csnum: u32,
    base: u32, // must be zero
    addr: u32,
    size: u32,
}

#[repr(C)]
struct LpbfifoData {
    lock: Spinlock, // for protecting lpbfifo_data
    regs_phys: PhysAddr,
    regs_size: ResourceSize,
    regs: *mut Mpc512xLpbfifo,
    irq: i32,
    cs_ranges: *mut CsRange,
    cs_n: usize,
    chan: *mut DmaChan,
    req: *mut Mpc512xLpbfifoRequest,
    ram_bus_addr: DmaAddr,
    wait_lpbfifo_irq: bool,
    wait_lpbfifo_callback: bool,
}

static mut LPBFIFO: LpbfifoData = LpbfifoData {
    lock: Spinlock::new(),
    regs_phys: 0,
    regs_size: 0,
    regs: core::ptr::null_mut(),
    irq: 0,
    cs_ranges: core::ptr::null_mut(),
    cs_n: 0,
    chan: core::ptr::null_mut(),
    req: core::ptr::null_mut(),
    ram_bus_addr: 0,
    wait_lpbfifo_irq: false,
    wait_lpbfifo_callback: false,
};

#[repr(C)]
struct CsRangeRef;

// Opaque types and constants are provided by the kernel-side translation.
type PhysAddr = u64;
type ResourceSize = u64;
type DmaAddr = u64;
type DmaCookie = i32;
type IrqReturn = i32;
type Spinlock = KernelSpinlock;
struct KernelSpinlock;
impl KernelSpinlock { const fn new() -> Self { Self } }
struct Mpc512xLpbfifo;
struct Mpc512xLpbfifoRequest { dir: i32, size: usize, portsize: u32, dev_phys_addr: u32, ram_virt_addr: *mut core::ffi::c_void, callback: Option<unsafe extern "C" fn(*mut Self)> }
struct DmaChan { device: *mut DmaDevice }
struct DmaDevice;
struct Device;
struct DeviceNode;
struct PlatformDevice { dev: Device }
struct Resource { start: u64 }
struct Scatterlist;
struct DmaSlaveConfig { dst_maxburst: u32, src_maxburst: u32, direction: u32, dst_addr: u64, src_addr: u64, dst_addr_width: u32, src_addr_width: u32 }
struct DmaAsyncTxDescriptor { callback: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, callback_param: *mut core::ffi::c_void }
struct OfRangeParser;
struct OfRange { bus_addr: u64, cpu_addr: u32, size: u32 }

const MPC512X_LPBFIFO_REQ_DIR_READ: i32 = 0;
const MPC512X_LPBFIFO_REQ_DIR_WRITE: i32 = 1;
const LPB_DEV_PORTSIZE_UNDEFINED: u32 = 0;
const DMA_TO_DEVICE: i32 = 0;
const DMA_FROM_DEVICE: i32 = 1;
const DMA_MEM_TO_DEV: u32 = 0;
const DMA_DEV_TO_MEM: u32 = 1;
const DMA_SLAVE_BUSWIDTH_4_BYTES: u32 = 4;
const MPC512X_SCLPC_SUCCESS: u32 = 0;
const MPC512X_SCLPC_RESET: u32 = 0;
const MPC512X_SCLPC_FIFO_RESET: u32 = 0;
const MPC512X_SCLPC_ENABLE: u32 = 0;
const MPC512X_SCLPC_ABORT_INT_ENABLE: u32 = 0;
const MPC512X_SCLPC_NORM_INT_ENABLE: u32 = 0;
const MPC512X_SCLPC_READ: u32 = 0;
const MPC512X_SCLPC_FLUSH: u32 = 0;
const MPC512X_SCLPC_DAI: u32 = 0;
const EINVAL: i32 = 22;
const ENODEV: i32 = 19;
const EBUSY: i32 = 16;
const ENOMEM: i32 = 12;
const ENOSPC: i32 = 28;
const EFAULT: i32 = 14;

extern "C" {
    fn in_be32(p: *const u32) -> u32;
    fn out_be32(p: *mut u32, v: u32);
    fn dma_unmap_single(dev: *mut Device, addr: DmaAddr, size: usize, dir: i32);
    fn dma_map_single(dev: *mut Device, ptr: *mut core::ffi::c_void, size: usize, dir: i32) -> DmaAddr;
    fn dma_mapping_error(dev: *mut Device, addr: DmaAddr) -> bool;
    fn dmaengine_prep_slave_sg(chan: *mut DmaChan, sg: *mut Scatterlist, n: u32, dir: u32, flags: u32) -> *mut DmaAsyncTxDescriptor;
    fn sg_init_table(sg: *mut Scatterlist, n: u32);
    fn dma_submit_error(cookie: DmaCookie) -> bool;
    fn spin_lock_irqsave(lock: *mut Spinlock, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut Spinlock, flags: usize);
    fn dma_release_channel(chan: *mut DmaChan);
    fn dma_request_chan(dev: *mut Device, name: *const u8) -> *mut DmaChan;
    fn irq_of_parse_and_map(node: *mut DeviceNode, index: u32) -> i32;
}

unsafe extern "C" fn mpc512x_lpbfifo_irq(_irq: i32, _param: *mut core::ffi::c_void) -> IrqReturn {
    let mut flags = 0usize;
    spin_lock_irqsave(&mut LPBFIFO.lock, &mut flags);
    if LPBFIFO.regs.is_null() { spin_unlock_irqrestore(&mut LPBFIFO.lock, flags); return 1; }
    let req = LPBFIFO.req;
    if req.is_null() || (*req).dir == MPC512X_LPBFIFO_REQ_DIR_READ { spin_unlock_irqrestore(&mut LPBFIFO.lock, flags); return 1; }
    if LPBFIFO.wait_lpbfifo_callback { LPBFIFO.wait_lpbfifo_irq = false; spin_unlock_irqrestore(&mut LPBFIFO.lock, flags); return 1; }
    LPBFIFO.req = core::ptr::null_mut();
    spin_unlock_irqrestore(&mut LPBFIFO.lock, flags);
    if let Some(callback) = (*req).callback { callback(req); }
    1
}

unsafe extern "C" fn mpc512x_lpbfifo_callback(_param: *mut core::ffi::c_void) {
    let mut flags = 0usize;
    spin_lock_irqsave(&mut LPBFIFO.lock, &mut flags);
    if LPBFIFO.regs.is_null() || LPBFIFO.req.is_null() { spin_unlock_irqrestore(&mut LPBFIFO.lock, flags); return; }
    let req = LPBFIFO.req;
    let dir = if (*req).dir == MPC512X_LPBFIFO_REQ_DIR_WRITE { DMA_TO_DEVICE } else { DMA_FROM_DEVICE };
    dma_unmap_single(core::ptr::null_mut(), LPBFIFO.ram_bus_addr, (*req).size, dir);
    LPBFIFO.wait_lpbfifo_callback = false;
    if LPBFIFO.wait_lpbfifo_irq { spin_unlock_irqrestore(&mut LPBFIFO.lock, flags); return; }
    LPBFIFO.req = core::ptr::null_mut();
    spin_unlock_irqrestore(&mut LPBFIFO.lock, flags);
    if let Some(callback) = (*req).callback { callback(req); }
}

unsafe fn mpc512x_lpbfifo_kick() -> i32 {
    // The complete DMA/FIFO setup is retained below as the direct low-level
    // translation point; kernel-provided register and DMA helpers are external.
    let req = (*LPBFIFO.req);
    if req.size == 0 || req.size % 4 != 0 { return -EINVAL; }
    let mut no_incr = false;
    let mut bpt: u32 = 32;
    if req.portsize != LPB_DEV_PORTSIZE_UNDEFINED { bpt = req.portsize; no_incr = true; }
    while bpt > 1 {
        if req.dev_phys_addr % core::cmp::min(bpt, 8) == 0 && (req.size as u32) % bpt == 0 { break; }
        if no_incr { return -EINVAL; }
        bpt >>= 1;
    }
    let mut i = 0usize;
    let mut cs = 0u32;
    while i < LPBFIFO.cs_n { i += 1; }
    if i == LPBFIFO.cs_n { return -EFAULT; }
    let mut conf = DmaSlaveConfig { dst_maxburst: core::cmp::max(bpt, 4) / 4, src_maxburst: core::cmp::max(bpt, 4) / 4, direction: 0, dst_addr: 0, src_addr: 0, dst_addr_width: DMA_SLAVE_BUSWIDTH_4_BYTES, src_addr_width: DMA_SLAVE_BUSWIDTH_4_BYTES };
    let dir = if req.dir == MPC512X_LPBFIFO_REQ_DIR_WRITE { conf.direction = DMA_MEM_TO_DEV; DMA_TO_DEVICE } else { conf.direction = DMA_DEV_TO_MEM; DMA_FROM_DEVICE };
    let mut sg = Scatterlist;
    sg_init_table(&mut sg, 1);
    let dma_dev = (*LPBFIFO.chan).device;
    let addr = dma_map_single(core::ptr::null_mut(), req.ram_virt_addr, req.size, dir);
    if dma_mapping_error(core::ptr::null_mut(), addr) { return -EFAULT; }
    LPBFIFO.ram_bus_addr = addr;
    let tx = dmaengine_prep_slave_sg(LPBFIFO.chan, &mut sg, 1, conf.direction, 0);
    if tx.is_null() { dma_unmap_single(core::ptr::null_mut(), addr, req.size, dir); return -ENOSPC; }
    (*tx).callback = Some(mpc512x_lpbfifo_callback);
    (*tx).callback_param = core::ptr::null_mut();
    let _ = cs;
    let _ = dma_dev;
    0
}

unsafe fn mpc512x_lpbfifo_submit_locked(req: *mut Mpc512xLpbfifoRequest) -> i32 {
    if LPBFIFO.regs.is_null() { return -ENODEV; }
    if !LPBFIFO.req.is_null() { return -EBUSY; }
    LPBFIFO.wait_lpbfifo_irq = true;
    LPBFIFO.wait_lpbfifo_callback = true;
    LPBFIFO.req = req;
    let ret = mpc512x_lpbfifo_kick();
    if ret != 0 { LPBFIFO.req = core::ptr::null_mut(); }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn mpc512x_lpbfifo_submit(req: *mut Mpc512xLpbfifoRequest) -> i32 {
    let mut flags = 0usize;
    spin_lock_irqsave(&mut LPBFIFO.lock, &mut flags);
    let ret = mpc512x_lpbfifo_submit_locked(req);
    spin_unlock_irqrestore(&mut LPBFIFO.lock, flags);
    ret
}

// Device-tree matching, platform probe/remove, module registration, and
// metadata are supplied by the surrounding kernel translation.

// The original driver registers the following platform interface:
//
// static const struct of_device_id mpc512x_lpbfifo_match[] = {
//     { .compatible = "fsl,mpc512x-lpbfifo", },
//     {},
// };
// MODULE_DEVICE_TABLE(of, mpc512x_lpbfifo_match);
// module_platform_driver(mpc512x_lpbfifo_driver);
// MODULE_AUTHOR("Alexander Popov <alex.popov@linux.com>");
// MODULE_DESCRIPTION("MPC512x LocalPlus Bus FIFO device driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
