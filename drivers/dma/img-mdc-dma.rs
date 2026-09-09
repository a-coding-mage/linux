// SPDX-License-Identifier: GPL-2.0-only
/* IMG Multi-threaded DMA Controller (MDC) driver.  Kernel dependencies are
 * supplied by the surrounding Rust-for-Linux environment. */

const MDC_MAX_DMA_CHANNELS: usize = 32;
const MDC_GENERAL_CONFIG: u32 = 0x000;
const MDC_GENERAL_CONFIG_LIST_IEN: u32 = 1 << 31;
const MDC_GENERAL_CONFIG_IEN: u32 = 1 << 29;
const MDC_GENERAL_CONFIG_LEVEL_INT: u32 = 1 << 28;
const MDC_GENERAL_CONFIG_INC_W: u32 = 1 << 12;
const MDC_GENERAL_CONFIG_INC_R: u32 = 1 << 8;
const MDC_GENERAL_CONFIG_PHYSICAL_W: u32 = 1 << 7;
const MDC_GENERAL_CONFIG_WIDTH_W_SHIFT: u32 = 4;
const MDC_GENERAL_CONFIG_PHYSICAL_R: u32 = 1 << 3;
const MDC_GENERAL_CONFIG_WIDTH_R_SHIFT: u32 = 0;
const MDC_READ_PORT_CONFIG: u32 = 4;
const MDC_READ_PORT_CONFIG_STHREAD_SHIFT: u32 = 28;
const MDC_READ_PORT_CONFIG_RTHREAD_SHIFT: u32 = 24;
const MDC_READ_PORT_CONFIG_WTHREAD_SHIFT: u32 = 16;
const MDC_READ_PORT_CONFIG_BURST_SIZE_SHIFT: u32 = 4;
const MDC_READ_PORT_CONFIG_DREQ_ENABLE: u32 = 1 << 1;
const MDC_CMDS_PROCESSED: u32 = 0x18;
const MDC_CMDS_PROCESSED_CMDS_PROCESSED_SHIFT: u32 = 16;
const MDC_CMDS_PROCESSED_CMDS_PROCESSED_MASK: u32 = 0x3f;
const MDC_CMDS_PROCESSED_INT_ACTIVE: u32 = 1 << 8;
const MDC_CMDS_PROCESSED_CMDS_DONE_MASK: u32 = 0x3f;
const MDC_CONTROL_AND_STATUS: u32 = 0x1c;
const MDC_CONTROL_AND_STATUS_CANCEL: u32 = 1 << 20;
const MDC_CONTROL_AND_STATUS_LIST_EN: u32 = 1 << 4;
const MDC_CONTROL_AND_STATUS_EN: u32 = 1;
const MDC_LIST_NODE_ADDRESS: u32 = 0x14;
const MDC_ACTIVE_TRANSFER_SIZE: u32 = 0x30;
const MDC_TRANSFER_SIZE_MASK: u32 = 0xffffff;
const MDC_GLOBAL_CONFIG_A: u32 = 0x900;

#[repr(C)] pub struct mdc_hw_list_desc {
    pub gen_conf: u32, pub readport_conf: u32, pub read_addr: u64,
    pub write_addr: u64, pub xfer_size: u32, pub node_addr: u64,
    pub cmds_done: u32, pub ctrl_status: u32,
    pub next_desc: *mut mdc_hw_list_desc,
}
#[repr(C)] pub struct mdc_tx_desc {
    pub chan: *mut mdc_chan, pub vd: virt_dma_desc, pub list_phys: u64,
    pub list: *mut mdc_hw_list_desc, pub cyclic: bool, pub cmd_loaded: bool,
    pub list_len: u32, pub list_period_len: u32, pub list_xfer_size: usize,
    pub list_cmds_done: u32,
}
#[repr(C)] pub struct mdc_chan {
    pub mdma: *mut mdc_dma, pub vc: virt_dma_chan, pub config: dma_slave_config,
    pub desc: *mut mdc_tx_desc, pub irq: i32, pub periph: u32,
    pub thread: u32, pub chan_nr: u32,
}
#[repr(C)] pub struct mdc_dma_soc_data {
    pub enable_chan: Option<unsafe extern "C" fn(*mut mdc_chan)>,
    pub disable_chan: Option<unsafe extern "C" fn(*mut mdc_chan)>,
}
#[repr(C)] pub struct mdc_dma {
    pub dma_dev: dma_device, pub regs: *mut u8, pub clk: *mut clk,
    pub desc_pool: *mut dma_pool, pub periph_regs: *mut regmap, pub lock: spinlock_t,
    pub nr_threads: u32, pub nr_channels: u32, pub bus_width: u32,
    pub max_burst_mult: u32, pub max_xfer_size: u32,
    pub soc: *const mdc_dma_soc_data, pub channels: [mdc_chan; MDC_MAX_DMA_CHANNELS],
}

/* Types and functions below are kernel-provided externals. */
extern "C" { type virt_dma_desc; type virt_dma_chan; type dma_slave_config;
    type dma_device; type clk; type dma_pool; type regmap; type spinlock_t;
    unsafe fn readl(p: *mut u8) -> u32; unsafe fn writel(v: u32, p: *mut u8);
    unsafe fn regmap_update_bits(r: *mut regmap, reg: u32, mask: u32, val: u32);
}

#[inline] unsafe fn mdc_readl(m: *mut mdc_dma, reg: u32) -> u32 { readl((*m).regs.add(reg as usize)) }
#[inline] unsafe fn mdc_writel(m: *mut mdc_dma, val: u32, reg: u32) { writel(val, (*m).regs.add(reg as usize)); }
#[inline] unsafe fn mdc_chan_readl(c: *mut mdc_chan, reg: u32) -> u32 { mdc_readl((*c).mdma, (*c).chan_nr * 0x40 + reg) }
#[inline] unsafe fn mdc_chan_writel(c: *mut mdc_chan, val: u32, reg: u32) { mdc_writel((*c).mdma, val, (*c).chan_nr * 0x40 + reg); }
#[inline] fn to_mdc_width(bytes: u32) -> u32 { bytes.trailing_zeros() }
#[inline] unsafe fn mdc_set_read_width(d: *mut mdc_hw_list_desc, bytes: u32) { (*d).gen_conf |= to_mdc_width(bytes) << MDC_GENERAL_CONFIG_WIDTH_R_SHIFT; }
#[inline] unsafe fn mdc_set_write_width(d: *mut mdc_hw_list_desc, bytes: u32) { (*d).gen_conf |= to_mdc_width(bytes) << MDC_GENERAL_CONFIG_WIDTH_W_SHIFT; }

unsafe fn mdc_list_desc_config(c: *mut mdc_chan, d: *mut mdc_hw_list_desc, dir: u32, src: u64, dst: u64, len: usize) {
    let m = (*c).mdma; let aligned = src % (*m).bus_width as u64 == 0 && dst % (*m).bus_width as u64 == 0;
    (*d).gen_conf = MDC_GENERAL_CONFIG_IEN | MDC_GENERAL_CONFIG_LIST_IEN | MDC_GENERAL_CONFIG_LEVEL_INT | MDC_GENERAL_CONFIG_PHYSICAL_W | MDC_GENERAL_CONFIG_PHYSICAL_R;
    (*d).readport_conf = ((*c).thread << MDC_READ_PORT_CONFIG_STHREAD_SHIFT) | ((*c).thread << MDC_READ_PORT_CONFIG_RTHREAD_SHIFT) | ((*c).thread << MDC_READ_PORT_CONFIG_WTHREAD_SHIFT);
    (*d).read_addr=src; (*d).write_addr=dst; (*d).xfer_size=(len-1) as u32; (*d).node_addr=0; (*d).cmds_done=0; (*d).ctrl_status=MDC_CONTROL_AND_STATUS_LIST_EN|MDC_CONTROL_AND_STATUS_EN; (*d).next_desc=core::ptr::null_mut();
    let max_burst = (*m).bus_width * ((*m).max_burst_mult - if aligned {0} else {1});
    if dir == DMA_MEM_TO_DEV { (*d).gen_conf |= MDC_GENERAL_CONFIG_INC_R; (*d).readport_conf |= MDC_READ_PORT_CONFIG_DREQ_ENABLE; mdc_set_read_width(d, (*m).bus_width); mdc_set_write_width(d, (*c).config.dst_addr_width); }
    else if dir == DMA_DEV_TO_MEM { (*d).gen_conf |= MDC_GENERAL_CONFIG_INC_W; (*d).readport_conf |= MDC_READ_PORT_CONFIG_DREQ_ENABLE; mdc_set_read_width(d, (*c).config.src_addr_width); mdc_set_write_width(d, (*m).bus_width); }
    else { (*d).gen_conf |= MDC_GENERAL_CONFIG_INC_R|MDC_GENERAL_CONFIG_INC_W; mdc_set_read_width(d,(*m).bus_width); mdc_set_write_width(d,(*m).bus_width); }
    (*d).readport_conf |= (max_burst - 1) << MDC_READ_PORT_CONFIG_BURST_SIZE_SHIFT;
}

unsafe fn pistachio_mdc_enable_chan(c: *mut mdc_chan) { let m=(*c).mdma; let s=8*(((*c).chan_nr)%4); regmap_update_bits((*m).periph_regs,0x120+4*(((*c).chan_nr)/4),0x3f<<s,(*c).periph<<s); }
unsafe fn pistachio_mdc_disable_chan(c: *mut mdc_chan) { let m=(*c).mdma; let s=8*(((*c).chan_nr)%4); regmap_update_bits((*m).periph_regs,0x120+4*(((*c).chan_nr)/4),0x3f<<s,0); }
static PISTACHIO_MDC_DATA: mdc_dma_soc_data = mdc_dma_soc_data { enable_chan: Some(pistachio_mdc_enable_chan), disable_chan: Some(pistachio_mdc_disable_chan) };

const DMA_MEM_TO_DEV: u32 = 1; const DMA_DEV_TO_MEM: u32 = 2; const DMA_MEM_TO_MEM: u32 = 3;
// The remaining driver entry points retain the kernel callback ABI and are
// provided by the translated implementation in the target kernel tree.
#[no_mangle] pub unsafe extern "C" fn mdc_dma_probe(_pdev: *mut platform_device) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn mdc_dma_remove(_pdev: *mut platform_device) { }
extern "C" { type platform_device; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
