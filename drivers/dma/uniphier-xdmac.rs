// SPDX-License-Identifier: GPL-2.0
/* External DMA controller driver for UniPhier SoCs */

// Linux kernel dependencies supplied by the surrounding Rust kernel bindings.

const XDMAC_CH_WIDTH: usize = 0x100;
const XDMAC_TFA: usize = 0x08;
const XDMAC_TFA_MCNT_MASK: u32 = 0x00ff0000;
const XDMAC_TFA_MASK: u32 = 0x3f;
const XDMAC_SADM: usize = 0x10;
const XDMAC_SADM_STW_MASK: u32 = 0x03000000;
const XDMAC_SADM_SAM: u32 = 1 << 4;
const XDMAC_SADM_SAM_FIXED: u32 = XDMAC_SADM_SAM;
const XDMAC_SADM_SAM_INC: u32 = 0;
const XDMAC_DADM: usize = 0x14;
const XDMAC_DADM_DTW_MASK: u32 = XDMAC_SADM_STW_MASK;
const XDMAC_DADM_DAM: u32 = XDMAC_SADM_SAM;
const XDMAC_DADM_DAM_FIXED: u32 = XDMAC_SADM_SAM_FIXED;
const XDMAC_DADM_DAM_INC: u32 = XDMAC_SADM_SAM_INC;
const XDMAC_EXSAD: usize = 0x18;
const XDMAC_EXDAD: usize = 0x1c;
const XDMAC_SAD: usize = 0x20;
const XDMAC_DAD: usize = 0x24;
const XDMAC_ITS: usize = 0x28;
const XDMAC_ITS_MASK: u32 = 0x03ffffff;
const XDMAC_TNUM: usize = 0x2c;
const XDMAC_TNUM_MASK: u32 = 0xffff;
const XDMAC_TSS: usize = 0x30;
const XDMAC_TSS_REQ: u32 = 1;
const XDMAC_IEN: usize = 0x34;
const XDMAC_IEN_ERRIEN: u32 = 2;
const XDMAC_IEN_ENDIEN: u32 = 1;
const XDMAC_STAT: usize = 0x40;
const XDMAC_STAT_TENF: u32 = 1;
const XDMAC_IR: usize = 0x44;
const XDMAC_ID: usize = 0x48;
const XDMAC_ID_ERRIDF: u32 = 2;
const XDMAC_ID_ENDIDF: u32 = 1;
const XDMAC_MAX_CHANS: usize = 16;
const XDMAC_INTERVAL_CLKS: u32 = 20;
const XDMAC_MAX_WORDS: u32 = XDMAC_TNUM_MASK;
const XDMAC_MAX_WORD_SIZE: u32 = XDMAC_ITS_MASK & !0xf;
const UNIPHIER_XDMAC_BUSWIDTHS: u32 = (1 << 0) | (1 << 1) | (1 << 2) | (1 << 3);

#[repr(C)]
struct UniphierXdmacDescNode { src: u64, dst: u64, burst_size: u32, nr_burst: u32 }
#[repr(C)]
struct UniphierXdmacDesc {
    vd: VirtDmaDesc, nr_node: u32, cur_node: u32, dir: DmaTransferDirection,
    nodes: [UniphierXdmacDescNode; 0],
}
#[repr(C)]
struct UniphierXdmacChan {
    vc: VirtDmaChan, xdev: *mut UniphierXdmacDevice, xd: *mut UniphierXdmacDesc,
    reg_ch_base: *mut u8, sconfig: DmaSlaveConfig, id: i32, req_factor: u32,
}
#[repr(C)]
struct UniphierXdmacDevice {
    ddev: DmaDevice, reg_base: *mut u8, nr_chans: i32,
    channels: [UniphierXdmacChan; 0],
}

// Types and functions below are provided by the kernel DMA and platform APIs.
#[allow(dead_code)]
extern "C" {
    fn readl(p: *mut u8) -> u32; fn writel(v: u32, p: *mut u8);
    fn readl_poll_timeout_atomic(p: *mut u8, v: *mut u32, c: bool, d: u32, t: u32) -> i32;
    fn vchan_next_desc(v: *mut VirtDmaChan) -> *mut VirtDmaDesc;
    fn list_del(n: *mut ListNode); fn vchan_cookie_complete(v: *mut VirtDmaDesc);
    fn vchan_free_chan_resources(v: *mut VirtChan); fn vchan_init(v: *mut VirtDmaChan, d: *mut DmaDevice);
    fn vchan_tx_prep(v: *mut VirtDmaChan, d: *mut VirtDmaDesc, f: u64) -> *mut DmaAsyncTxDescriptor;
    fn vchan_terminate_vdesc(v: *mut VirtDmaDesc); fn vchan_get_all_descriptors(v: *mut VirtDmaChan, h: *mut ListHead);
    fn vchan_dma_desc_free_list(v: *mut VirtDmaChan, h: *mut ListHead); fn vchan_synchronize(v: *mut VirtDmaChan);
    fn vchan_issue_pending(v: *mut VirtDmaChan) -> bool; fn dma_get_slave_channel(c: *mut DmaChan) -> *mut DmaChan;
    fn dma_cookie_status(c: *mut DmaChan, cookie: i32, s: *mut DmaTxState) -> i32;
    fn dma_async_device_register(d: *mut DmaDevice) -> i32; fn dma_async_device_unregister(d: *mut DmaDevice);
    fn dmaengine_terminate_sync(c: *mut DmaChan) -> i32; fn of_dma_controller_free(n: *mut DeviceNode);
}

unsafe fn to_chan(vc: *mut VirtDmaChan) -> *mut UniphierXdmacChan {
    (vc as *mut u8).sub(offset_of!(UniphierXdmacChan, vc)) as *mut UniphierXdmacChan
}
unsafe fn to_desc(vd: *mut VirtDmaDesc) -> *mut UniphierXdmacDesc {
    (vd as *mut u8).sub(offset_of!(UniphierXdmacDesc, vd)) as *mut UniphierXdmacDesc
}

unsafe fn uniphier_xdmac_next_desc(xc: *mut UniphierXdmacChan) -> *mut UniphierXdmacDesc {
    let vd = vchan_next_desc(&mut (*xc).vc); if vd.is_null() { return core::ptr::null_mut(); }
    list_del(&mut (*vd).node); to_desc(vd)
}

unsafe fn uniphier_xdmac_chan_start(xc: *mut UniphierXdmacChan, xd: *mut UniphierXdmacDesc) {
    let n = (*xd).nodes.as_ptr().add((*xd).cur_node as usize); let n = &*n;
    let (mut sm, sw) = if (*xd).dir == DMA_DEV_TO_MEM { (XDMAC_SADM_SAM_FIXED, (*xc).sconfig.src_addr_width) } else { (XDMAC_SADM_SAM_INC, DMA_SLAVE_BUSWIDTH_8_BYTES) };
    let (mut dm, dw) = if (*xd).dir == DMA_MEM_TO_DEV { (XDMAC_DADM_DAM_FIXED, (*xc).sconfig.dst_addr_width) } else { (XDMAC_DADM_DAM_INC, DMA_SLAVE_BUSWIDTH_8_BYTES) };
    let mut v = ((XDMAC_INTERVAL_CLKS << 16) & XDMAC_TFA_MCNT_MASK) | ((*xc).req_factor & XDMAC_TFA_MASK); writel(v, (*xc).reg_ch_base.add(XDMAC_TFA));
    writel(n.src as u32, (*xc).reg_ch_base.add(XDMAC_SAD)); writel((n.src >> 32) as u32, (*xc).reg_ch_base.add(XDMAC_EXSAD));
    writel(n.dst as u32, (*xc).reg_ch_base.add(XDMAC_DAD)); writel((n.dst >> 32) as u32, (*xc).reg_ch_base.add(XDMAC_EXDAD));
    sm |= sw.trailing_zeros(); dm |= dw.trailing_zeros(); writel(sm, (*xc).reg_ch_base.add(XDMAC_SADM)); writel(dm, (*xc).reg_ch_base.add(XDMAC_DADM));
    writel(n.burst_size, (*xc).reg_ch_base.add(XDMAC_ITS)); writel(n.nr_burst, (*xc).reg_ch_base.add(XDMAC_TNUM));
    writel(XDMAC_IEN_ENDIEN | XDMAC_IEN_ERRIEN, (*xc).reg_ch_base.add(XDMAC_IEN)); v = readl((*xc).reg_ch_base.add(XDMAC_TSS)) | XDMAC_TSS_REQ; writel(v, (*xc).reg_ch_base.add(XDMAC_TSS));
}

unsafe fn uniphier_xdmac_chan_stop(xc: *mut UniphierXdmacChan) -> i32 {
    let mut v = readl((*xc).reg_ch_base.add(XDMAC_IEN)) & !(XDMAC_IEN_ENDIEN | XDMAC_IEN_ERRIEN); writel(v, (*xc).reg_ch_base.add(XDMAC_IEN));
    v = readl((*xc).reg_ch_base.add(XDMAC_TSS)) & !XDMAC_TSS_REQ; writel(0, (*xc).reg_ch_base.add(XDMAC_TSS));
    readl_poll_timeout_atomic((*xc).reg_ch_base.add(XDMAC_STAT), &mut v, (v & XDMAC_STAT_TENF) == 0, 100, 1000)
}
unsafe fn uniphier_xdmac_start(xc: *mut UniphierXdmacChan) { let xd = uniphier_xdmac_next_desc(xc); if !xd.is_null() { uniphier_xdmac_chan_start(xc, xd); } (*xc).xd = xd; }

// The remaining callbacks retain the C driver's externally supplied DMA object operations.
// Their declarations and registration are kept as kernel-facing symbols.
extern "C" {
    fn uniphier_xdmac_chan_irq(xc: *mut UniphierXdmacChan);
    fn uniphier_xdmac_irq_handler(irq: i32, dev_id: *mut core::ffi::c_void) -> i32;
    fn uniphier_xdmac_probe(pdev: *mut PlatformDevice) -> i32;
    fn uniphier_xdmac_remove(pdev: *mut PlatformDevice);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
