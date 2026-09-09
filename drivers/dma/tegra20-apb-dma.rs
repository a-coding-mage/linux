// SPDX-License-Identifier: GPL-2.0-only
/* DMA driver for Nvidia's Tegra20 APB DMA controller. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

/* The Linux kernel definitions referenced by this translation are supplied by
 * the surrounding kernel/Rust compatibility layer. */
extern "C" {
    fn writel(value: u32, address: *mut core::ffi::c_void);
    fn readl(address: *const core::ffi::c_void) -> u32;
    fn udelay(usecs: u32);
}

const fn BIT(n: u32) -> u32 { 1u32 << n }

const TEGRA_APBDMA_GENERAL: u32 = 0x0;
const TEGRA_APBDMA_GENERAL_ENABLE: u32 = BIT(31);
const TEGRA_APBDMA_CONTROL: u32 = 0x010;
const TEGRA_APBDMA_IRQ_MASK: u32 = 0x01c;
const TEGRA_APBDMA_IRQ_MASK_SET: u32 = 0x020;
const TEGRA_APBDMA_CHAN_CSR: u32 = 0x00;
const TEGRA_APBDMA_CSR_ENB: u32 = BIT(31);
const TEGRA_APBDMA_CSR_IE_EOC: u32 = BIT(30);
const TEGRA_APBDMA_CSR_HOLD: u32 = BIT(29);
const TEGRA_APBDMA_CSR_DIR: u32 = BIT(28);
const TEGRA_APBDMA_CSR_ONCE: u32 = BIT(27);
const TEGRA_APBDMA_CSR_FLOW: u32 = BIT(21);
const TEGRA_APBDMA_CSR_REQ_SEL_SHIFT: u32 = 16;
const TEGRA_APBDMA_CSR_REQ_SEL_MASK: u32 = 0x1F;
const TEGRA_APBDMA_CSR_WCOUNT_MASK: u32 = 0xFFFC;
const TEGRA_APBDMA_CHAN_STATUS: u32 = 0x004;
const TEGRA_APBDMA_STATUS_BUSY: u32 = BIT(31);
const TEGRA_APBDMA_STATUS_ISE_EOC: u32 = BIT(30);
const TEGRA_APBDMA_STATUS_HALT: u32 = BIT(29);
const TEGRA_APBDMA_STATUS_PING_PONG: u32 = BIT(28);
const TEGRA_APBDMA_STATUS_COUNT_SHIFT: u32 = 2;
const TEGRA_APBDMA_STATUS_COUNT_MASK: u32 = 0xFFFC;
const TEGRA_APBDMA_CHAN_CSRE: u32 = 0x00C;
const TEGRA_APBDMA_CHAN_CSRE_PAUSE: u32 = BIT(31);
const TEGRA_APBDMA_CHAN_AHBPTR: u32 = 0x010;
const TEGRA_APBDMA_CHAN_AHBSEQ: u32 = 0x14;
const TEGRA_APBDMA_AHBSEQ_INTR_ENB: u32 = BIT(31);
const TEGRA_APBDMA_AHBSEQ_BUS_WIDTH_8: u32 = 0 << 28;
const TEGRA_APBDMA_AHBSEQ_BUS_WIDTH_16: u32 = 1 << 28;
const TEGRA_APBDMA_AHBSEQ_BUS_WIDTH_32: u32 = 2 << 28;
const TEGRA_APBDMA_AHBSEQ_BUS_WIDTH_64: u32 = 3 << 28;
const TEGRA_APBDMA_AHBSEQ_BUS_WIDTH_128: u32 = 4 << 28;
const TEGRA_APBDMA_AHBSEQ_DATA_SWAP: u32 = BIT(27);
const TEGRA_APBDMA_AHBSEQ_BURST_1: u32 = 4 << 24;
const TEGRA_APBDMA_AHBSEQ_BURST_4: u32 = 5 << 24;
const TEGRA_APBDMA_AHBSEQ_BURST_8: u32 = 6 << 24;
const TEGRA_APBDMA_AHBSEQ_DBL_BUF: u32 = BIT(19);
const TEGRA_APBDMA_AHBSEQ_WRAP_SHIFT: u32 = 16;
const TEGRA_APBDMA_AHBSEQ_WRAP_NONE: u32 = 0;
const TEGRA_APBDMA_CHAN_APBPTR: u32 = 0x018;
const TEGRA_APBDMA_CHAN_APBSEQ: u32 = 0x01c;
const TEGRA_APBDMA_APBSEQ_BUS_WIDTH_8: u32 = 0 << 28;
const TEGRA_APBDMA_APBSEQ_BUS_WIDTH_16: u32 = 1 << 28;
const TEGRA_APBDMA_APBSEQ_BUS_WIDTH_32: u32 = 2 << 28;
const TEGRA_APBDMA_APBSEQ_BUS_WIDTH_64: u32 = 3 << 28;
const TEGRA_APBDMA_APBSEQ_BUS_WIDTH_128: u32 = 4 << 28;
const TEGRA_APBDMA_APBSEQ_DATA_SWAP: u32 = BIT(27);
const TEGRA_APBDMA_APBSEQ_WRAP_WORD_1: u32 = 1 << 16;
const TEGRA_APBDMA_CHAN_WCOUNT: u32 = 0x20;
const TEGRA_APBDMA_CHAN_WORD_TRANSFER: u32 = 0x24;
const TEGRA_APBDMA_BURST_COMPLETE_TIME: u32 = 20;
const TEGRA_APBDMA_CHANNEL_BASE_ADD_OFFSET: u32 = 0x1000;
const TEGRA_APBDMA_SLAVE_ID_INVALID: u32 = TEGRA_APBDMA_CSR_REQ_SEL_MASK + 1;

#[repr(C)]
pub struct tegra_dma_chip_data { pub nr_channels: u32, pub channel_reg_size: u32, pub max_dma_count: u32, pub support_channel_pause: bool, pub support_separate_wcount_reg: bool }
#[repr(C)]
pub struct tegra_dma_channel_regs { pub csr: u32, pub ahb_ptr: u32, pub apb_ptr: u32, pub ahb_seq: u32, pub apb_seq: u32, pub wcount: u32 }

/* Kernel structures are intentionally opaque here; their layout is defined by
 * the DMA-engine compatibility layer. */
#[repr(C)] pub struct tegra_dma_sg_req { pub ch_regs: tegra_dma_channel_regs, pub req_len: u32, pub configured: bool, pub last_sg: bool, pub node: [usize; 2], pub dma_desc: *mut tegra_dma_desc, pub words_xferred: u32 }
#[repr(C)] pub struct tegra_dma_desc { pub txd: dma_async_tx_descriptor, pub bytes_requested: u32, pub bytes_transferred: u32, pub dma_status: dma_status, pub node: [usize; 2], pub tx_list: [usize; 2], pub cb_node: [usize; 2], pub cb_count: u32 }
#[repr(C)] pub struct tegra_dma_channel { pub dma_chan: dma_chan, pub name: [u8; 12], pub config_init: bool, pub id: u32, pub chan_addr: *mut core::ffi::c_void, pub busy: bool, pub tdma: *mut tegra_dma, pub cyclic: bool, pub isr_handler: Option<unsafe extern "C" fn(*mut tegra_dma_channel, bool)> }
#[repr(C)] pub struct tegra_dma { pub dma_dev: dma_device, pub dev: *mut device, pub dma_clk: *mut clk, pub rst: *mut reset_control, pub base_addr: *mut core::ffi::c_void, pub chip_data: *const tegra_dma_chip_data, pub global_pause_count: u32 }

/* External kernel types. */
#[repr(C)] pub struct dma_async_tx_descriptor { pub chan: *mut dma_chan, pub flags: u32, pub cookie: i32, pub tx_submit: Option<unsafe extern "C" fn(*mut dma_async_tx_descriptor) -> i32> }
#[repr(C)] pub struct dma_chan { pub device: *mut dma_device }
#[repr(C)] pub struct dma_device { pub dev: *mut device }
#[repr(C)] pub struct device;
#[repr(C)] pub struct clk;
#[repr(C)] pub struct reset_control;
#[repr(C)] pub struct dma_slave_config { pub src_addr: u32, pub dst_addr: u32, pub src_addr_width: u32, pub dst_addr_width: u32, pub src_maxburst: u32, pub dst_maxburst: u32 }
#[repr(C)] #[derive(Copy, Clone)] pub enum dma_status { DMA_COMPLETE, DMA_IN_PROGRESS, DMA_ERROR }

#[inline] unsafe fn tdma_write(tdma: *mut tegra_dma, reg: u32, val: u32) { writel(val, (*tdma).base_addr.add(reg as usize)); }
#[inline] unsafe fn tdc_write(tdc: *mut tegra_dma_channel, reg: u32, val: u32) { writel(val, (*tdc).chan_addr.add(reg as usize)); }
#[inline] unsafe fn tdc_read(tdc: *mut tegra_dma_channel, reg: u32) -> u32 { readl((*tdc).chan_addr.add(reg as usize)) }

#[inline] unsafe fn tegra_dma_prep_wcount(tdc: *mut tegra_dma_channel, regs: *mut tegra_dma_channel_regs, len: u32) { let field = (len.wrapping_sub(4)) & 0xfffc; if (*(*tdc).tdma).chip_data != core::ptr::null() && (*(*(*tdc).tdma).chip_data).support_separate_wcount_reg { (*regs).wcount = field; } else { (*regs).csr |= field; } }

/* Core operations retain the driver's original ordering and register writes. */
#[inline] unsafe fn tegra_dma_start(tdc: *mut tegra_dma_channel, req: *mut tegra_dma_sg_req) { let r=&(*req).ch_regs; tdc_write(tdc,TEGRA_APBDMA_CHAN_CSR,r.csr); tdc_write(tdc,TEGRA_APBDMA_CHAN_APBSEQ,r.apb_seq); tdc_write(tdc,TEGRA_APBDMA_CHAN_APBPTR,r.apb_ptr); tdc_write(tdc,TEGRA_APBDMA_CHAN_AHBSEQ,r.ahb_seq); tdc_write(tdc,TEGRA_APBDMA_CHAN_AHBPTR,r.ahb_ptr); if (*(*(*tdc).tdma).chip_data).support_separate_wcount_reg { tdc_write(tdc,TEGRA_APBDMA_CHAN_WCOUNT,r.wcount); } tdc_write(tdc,TEGRA_APBDMA_CHAN_CSR,r.csr|TEGRA_APBDMA_CSR_ENB); }

#[no_mangle] pub unsafe extern "C" fn tegra_dma_init_hw(tdma: *mut tegra_dma) -> i32 { tdma_write(tdma,TEGRA_APBDMA_GENERAL,TEGRA_APBDMA_GENERAL_ENABLE); tdma_write(tdma,TEGRA_APBDMA_CONTROL,0); tdma_write(tdma,TEGRA_APBDMA_IRQ_MASK_SET,0xffff_ffff); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
