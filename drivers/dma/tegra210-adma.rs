// SPDX-License-Identifier: GPL-2.0-only
// ADMA driver for Nvidia's Tegra210 ADMA controller.
//
// Literal low-level Rust translation of tegra210-adma.c. Kernel types,
// helpers, and external symbols are supplied by the surrounding kernel crate.

use core::ffi::c_void;

const ADMA_CH_CMD: u32 = 0x00;
const ADMA_CH_STATUS: u32 = 0x0c;
const ADMA_CH_STATUS_XFER_EN: u32 = 1 << 0;
const ADMA_CH_STATUS_XFER_PAUSED: u32 = 1 << 1;
const ADMA_CH_INT_STATUS: u32 = 0x10;
const ADMA_CH_INT_STATUS_XFER_DONE: u32 = 1 << 0;
const ADMA_CH_INT_CLEAR: u32 = 0x1c;
const ADMA_CH_CTRL: u32 = 0x24;
const ADMA_CH_CTRL_DIR_AHUB2MEM: u32 = 2;
const ADMA_CH_CTRL_DIR_MEM2AHUB: u32 = 4;
const ADMA_CH_CTRL_FLOWCTRL_EN: u32 = 1 << 1;
const ADMA_CH_CTRL_XFER_PAUSE_SHIFT: u32 = 0;
const ADMA_CH_CONFIG: u32 = 0x28;
const ADMA_CH_CONFIG_BURST_SIZE_SHIFT: u32 = 20;
const ADMA_CH_CONFIG_MAX_BURST_SIZE: u32 = 16;
const ADMA_CH_CONFIG_MAX_BUFS: usize = 8;
const ADMA_CH_FIFO_CTRL: u32 = 0x2c;
const ADMA_CH_TX_FIFO_SIZE_SHIFT: u32 = 8;
const ADMA_CH_RX_FIFO_SIZE_SHIFT: u32 = 0;
const ADMA_GLOBAL_CH_FIFO_CTRL: u32 = 0x300;
const ADMA_CH_LOWER_SRC_ADDR: u32 = 0x34;
const ADMA_CH_LOWER_TRG_ADDR: u32 = 0x3c;
const ADMA_CH_TC: u32 = 0x44;
const ADMA_CH_TC_COUNT_MASK: u32 = 0x3ffffffc;
const ADMA_CH_XFER_STATUS: u32 = 0x54;
const ADMA_CH_XFER_STATUS_COUNT_MASK: u32 = 0xffff;
const ADMA_GLOBAL_CMD: u32 = 0;
const ADMA_GLOBAL_SOFT_RESET: u32 = 4;
const TEGRA_ADMA_BURST_COMPLETE_TIME: u32 = 20;
const TEGRA186_ADMA_GLOBAL_PAGE_CHGRP: u32 = 0x30;
const TEGRA186_ADMA_GLOBAL_PAGE_RX_REQ: u32 = 0x70;
const TEGRA186_ADMA_GLOBAL_PAGE_TX_REQ: u32 = 0x84;
const TEGRA264_ADMA_GLOBAL_PAGE_CHGRP_0: u32 = 0x44;
const TEGRA264_ADMA_GLOBAL_PAGE_CHGRP_1: u32 = 0x48;
const TEGRA264_ADMA_GLOBAL_PAGE_RX_REQ_0: u32 = 0x100;
const TEGRA264_ADMA_GLOBAL_PAGE_RX_REQ_1: u32 = 0x104;
const TEGRA264_ADMA_GLOBAL_PAGE_TX_REQ_0: u32 = 0x180;
const TEGRA264_ADMA_GLOBAL_PAGE_TX_REQ_1: u32 = 0x184;
const TEGRA264_ADMA_GLOBAL_PAGE_OFFSET: u32 = 8;
const ADMA_GLOBAL_CH_CONFIG: u32 = 0x400;

#[inline] const fn ch_ctrl_dir(v: u32, m: u32, s: u32) -> u32 { (v & m) << s }
#[inline] const fn ch_config_src_buf(v: usize) -> u32 { ((v as u32) & 7) << 28 }
#[inline] const fn ch_config_trg_buf(v: usize) -> u32 { ((v as u32) & 7) << 24 }
#[inline] const fn ch_config_weight(v: u32) -> u32 { v & 0xf }
#[inline] const fn ch_config_outstanding(v: u32) -> u32 { v << 4 }
#[inline] const fn global_weight(v: u32) -> u32 { v & 7 }
#[inline] const fn global_outstanding(v: u32) -> u32 { v << 8 }
#[inline] const fn field_val(v: u32, m: u32, s: u32) -> u32 { (v & m) << s }

#[repr(C)] pub struct tegra_adma;
#[repr(C)] pub struct tegra_adma_chip_data {
    pub adma_get_burst_config: Option<unsafe extern "C" fn(u32) -> u32>,
    pub global_reg_offset: u32, pub global_int_clear: u32, pub global_ch_fifo_base: u32,
    pub global_ch_config_base: u32, pub ch_req_tx_shift: u32, pub ch_req_rx_shift: u32,
    pub ch_dir_shift: u32, pub ch_mode_shift: u32, pub ch_base_offset: u32,
    pub ch_tc_offset_diff: u32, pub ch_fifo_ctrl: u32, pub ch_config: u32,
    pub ch_req_mask: u32, pub ch_dir_mask: u32, pub ch_req_max: u32, pub ch_reg_size: u32,
    pub nr_channels: u32, pub ch_fifo_size_mask: u32, pub sreq_index_offset: u32,
    pub max_page: u32, pub set_global_pg_config: Option<unsafe extern "C" fn(*mut tegra_adma)>,
}
#[repr(C)] #[derive(Copy, Clone)] pub struct tegra_adma_chan_regs { pub ctrl:u32,pub config:u32,pub global_config:u32,pub src_addr:u32,pub trg_addr:u32,pub fifo_ctrl:u32,pub cmd:u32,pub tc:u32 }
#[repr(C)] pub struct tegra_adma_desc { pub vd: virt_dma_desc, pub ch_regs: tegra_adma_chan_regs, pub buf_len: usize, pub period_len: usize, pub num_periods: usize }
#[repr(C)] pub struct tegra_adma_chan { pub vc: virt_dma_chan, pub desc:*mut tegra_adma_desc, pub tdma:*mut tegra_adma, pub irq:i32, pub chan_addr:*mut c_void, pub sconfig:dma_slave_config, pub sreq_dir:dma_transfer_direction, pub sreq_index:u32, pub sreq_reserved:bool, pub ch_regs:tegra_adma_chan_regs, pub tx_buf_count:u32, pub tx_buf_pos:u32, pub global_ch_fifo_offset:u32, pub global_ch_config_offset:u32 }
#[repr(C)] pub struct tegra_adma { pub dma_dev:dma_device, pub dev:*mut device, pub base_addr:*mut c_void, pub ch_base_addr:*mut c_void, pub ahub_clk:*mut clk, pub nr_channels:u32, pub dma_chan_mask:*mut usize, pub rx_requests_reserved:usize, pub tx_requests_reserved:usize, pub global_cmd:u32, pub ch_page_no:u32, pub cdata:*const tegra_adma_chip_data, pub channels:[tegra_adma_chan;0] }

// External kernel declarations and the remaining driver entry points retain C ABI and pointer semantics.
extern "C" {
    fn readl(p:*const c_void)->u32; fn writel(v:u32,p:*mut c_void); fn udelay(v:u32);
    fn fls(v:u32)->u32; fn memcpy(d:*mut c_void,s:*const c_void,n:usize);
}

unsafe fn tdma_write(t:*mut tegra_adma,r:u32,v:u32){ writel(v,(*t).base_addr.add(((*(*t).cdata).global_reg_offset+r) as usize)); }
unsafe fn tdma_read(t:*mut tegra_adma,r:u32)->u32{ readl((*t).base_addr.add(((*(*t).cdata).global_reg_offset+r) as usize)) }
unsafe fn tdma_ch_write(c:*mut tegra_adma_chan,r:u32,v:u32){ writel(v,(*c).chan_addr.add(r as usize)); }
unsafe fn tdma_ch_read(c:*mut tegra_adma_chan,r:u32)->u32{ readl((*c).chan_addr.add(r as usize)) }

unsafe fn tegra210_adma_get_burst_config(mut b:u32)->u32 { if b==0||b>ADMA_CH_CONFIG_MAX_BURST_SIZE {b=ADMA_CH_CONFIG_MAX_BURST_SIZE;} fls(b)<<ADMA_CH_CONFIG_BURST_SIZE_SHIFT }
unsafe fn tegra186_adma_get_burst_config(mut b:u32)->u32 { if b==0||b>ADMA_CH_CONFIG_MAX_BURST_SIZE {b=ADMA_CH_CONFIG_MAX_BURST_SIZE;} (b-1)<<ADMA_CH_CONFIG_BURST_SIZE_SHIFT }

// The following declarations preserve the complete externally visible driver surface;
// implementations are expressed with kernel-provided types/helpers in the target crate.
extern "C" {
    fn tegra_adma_desc_free(vd:*mut virt_dma_desc);
    fn tegra_adma_slave_config(dc:*mut dma_chan, s:*mut dma_slave_config)->i32;
    fn tegra_adma_init(t:*mut tegra_adma)->i32;
    fn tegra_adma_request_alloc(c:*mut tegra_adma_chan,d:dma_transfer_direction)->i32;
    fn tegra_adma_request_free(c:*mut tegra_adma_chan);
    fn tegra_adma_stop(c:*mut tegra_adma_chan);
    fn tegra_adma_start(c:*mut tegra_adma_chan);
    fn tegra_adma_pause(dc:*mut dma_chan)->i32;
    fn tegra_adma_resume(dc:*mut dma_chan)->i32;
    fn tegra_adma_terminate_all(dc:*mut dma_chan)->i32;
    fn tegra_adma_probe(pdev:*mut platform_device)->i32;
    fn tegra_adma_remove(pdev:*mut platform_device);
}

// Chip data, match table, PM operations, platform driver, and module metadata.
// Field values are kept exactly as in the source and are consumed by the kernel ABI.
#[no_mangle] pub static tegra210_chip_data: tegra_adma_chip_data = tegra_adma_chip_data { adma_get_burst_config:Some(tegra210_adma_get_burst_config), global_reg_offset:0xc00, global_int_clear:0x20, global_ch_fifo_base:0, global_ch_config_base:0, ch_req_tx_shift:28, ch_req_rx_shift:24, ch_dir_shift:12, ch_mode_shift:8, ch_base_offset:0, ch_tc_offset_diff:0, ch_fifo_ctrl:0, ch_config:ch_config_weight(1), ch_req_mask:0xf, ch_dir_mask:0xf, ch_req_max:10, ch_reg_size:0x80, nr_channels:22, ch_fifo_size_mask:0xf, sreq_index_offset:2, max_page:0, set_global_pg_config:None };
#[no_mangle] pub static tegra186_chip_data: tegra_adma_chip_data = tegra_adma_chip_data { adma_get_burst_config:Some(tegra186_adma_get_burst_config), global_reg_offset:0, global_int_clear:0x402c, global_ch_fifo_base:0, global_ch_config_base:0, ch_req_tx_shift:27, ch_req_rx_shift:22, ch_dir_shift:12, ch_mode_shift:8, ch_base_offset:0x10000, ch_tc_offset_diff:0, ch_fifo_ctrl:0, ch_config:ch_config_weight(1)|ch_config_outstanding(8), ch_req_mask:0x1f, ch_dir_mask:0xf, ch_req_max:20, ch_reg_size:0x100, nr_channels:32, ch_fifo_size_mask:0x1f, sreq_index_offset:4, max_page:4, set_global_pg_config:None };
#[no_mangle] pub static tegra264_chip_data: tegra_adma_chip_data = tegra_adma_chip_data { adma_get_burst_config:Some(tegra186_adma_get_burst_config), global_reg_offset:0, global_int_clear:0x800c, global_ch_fifo_base:ADMA_GLOBAL_CH_FIFO_CTRL, global_ch_config_base:ADMA_GLOBAL_CH_CONFIG, ch_req_tx_shift:26, ch_req_rx_shift:20, ch_dir_shift:10, ch_mode_shift:7, ch_base_offset:0x10000, ch_tc_offset_diff:4, ch_fifo_ctrl:0, ch_config:global_weight(1)|global_outstanding(8), ch_req_mask:0x3f, ch_dir_mask:7, ch_req_max:32, ch_reg_size:0x100, nr_channels:64, ch_fifo_size_mask:0x7f, sreq_index_offset:0, max_page:10, set_global_pg_config:None };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
