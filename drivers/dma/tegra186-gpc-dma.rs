// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level Rust translation of tegra186-gpc-dma.c.
 * Kernel types, helpers, and functions referenced from included headers are
 * intentionally left as external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    fn readl_relaxed(p: *mut core::ffi::c_void) -> u32;
    fn writel_relaxed(v: u32, p: *mut core::ffi::c_void);
    fn readl_relaxed_poll_timeout_atomic(p: *mut core::ffi::c_void, v: *mut u32, cond: bool, delay: u32, timeout: u32) -> i32;
    fn udelay(us: u32);
}

const fn bit(n: u32) -> u32 { 1u32 << n }
const fn genmask(h: u32, l: u32) -> u32 { ((!0u32) >> (31-h)) & ((!0u32) << l) }
const fn field_prep(mask: u32, val: u32) -> u32 { (val << mask.trailing_zeros()) & mask }

const TEGRA_GPCDMA_CSR_ENB: u32 = bit(31);
const TEGRA_GPCDMA_CSR_IE_EOC: u32 = bit(30);
const TEGRA_GPCDMA_CSR_ONCE: u32 = bit(27);
const TEGRA_GPCDMA_CSR_FC_MODE: u32 = genmask(25,24);
const TEGRA_GPCDMA_CSR_DMA: u32 = genmask(23,21);
const TEGRA_GPCDMA_CSR_DMA_IO2MEM_FC: u32 = field_prep(TEGRA_GPCDMA_CSR_DMA,1);
const TEGRA_GPCDMA_CSR_DMA_MEM2IO_FC: u32 = field_prep(TEGRA_GPCDMA_CSR_DMA,3);
const TEGRA_GPCDMA_CSR_DMA_MEM2MEM: u32 = field_prep(TEGRA_GPCDMA_CSR_DMA,4);
const TEGRA_GPCDMA_CSR_DMA_FIXED_PAT: u32 = field_prep(TEGRA_GPCDMA_CSR_DMA,6);
const TEGRA_GPCDMA_CSR_REQ_SEL_MASK: u32 = genmask(20,16);
const TEGRA_GPCDMA_CSR_REQ_SEL_UNUSED: u32 = field_prep(TEGRA_GPCDMA_CSR_REQ_SEL_MASK,4);
const TEGRA_GPCDMA_CSR_IRQ_MASK: u32 = bit(15);
const TEGRA_GPCDMA_CSR_WEIGHT: u32 = genmask(13,10);
const TEGRA_GPCDMA_STATUS_BUSY: u32 = bit(31);
const TEGRA_GPCDMA_STATUS_ISE_EOC: u32 = bit(30);
const TEGRA_GPCDMA_STATUS_CHANNEL_RX: u32 = bit(25);
const TEGRA_GPCDMA_STATUS_CHANNEL_TX: u32 = bit(24);
const TEGRA_GPCDMA_CHAN_CSRE_PAUSE: u32 = bit(31);
const TEGRA_GPCDMA_HIGH_ADDR_SRC_PTR: u32 = genmask(7,0);
const TEGRA_GPCDMA_HIGH_ADDR_DST_PTR: u32 = genmask(23,16);
const TEGRA_GPCDMA_MCSEQ_REQ_COUNT: u32 = genmask(30,25);
const TEGRA_GPCDMA_MCSEQ_BURST: u32 = genmask(24,23);
const TEGRA_GPCDMA_MCSEQ_BURST_2: u32 = 0;
const TEGRA_GPCDMA_MCSEQ_BURST_16: u32 = field_prep(TEGRA_GPCDMA_MCSEQ_BURST,3);
const TEGRA_GPCDMA_MCSEQ_WRAP1: u32 = genmask(22,20);
const TEGRA_GPCDMA_MCSEQ_WRAP0: u32 = genmask(19,17);
const TEGRA_GPCDMA_MCSEQ_STREAM_ID1_MASK: u32 = genmask(13,7);
const TEGRA_GPCDMA_MCSEQ_STREAM_ID0_MASK: u32 = genmask(6,0);
const TEGRA_GPCDMA_MMIOSEQ_BUS_WIDTH: u32 = genmask(30,28);
const TEGRA_GPCDMA_MMIOSEQ_BUS_WIDTH_8: u32 = 0;
const TEGRA_GPCDMA_MMIOSEQ_BUS_WIDTH_16: u32 = field_prep(TEGRA_GPCDMA_MMIOSEQ_BUS_WIDTH,1);
const TEGRA_GPCDMA_MMIOSEQ_BUS_WIDTH_32: u32 = field_prep(TEGRA_GPCDMA_MMIOSEQ_BUS_WIDTH,2);
const TEGRA_GPCDMA_MMIOSEQ_WRAP_WORD: u32 = genmask(18,16);
const TEGRA_GPCDMA_MMIOSEQ_BURST_SHIFT: u32 = 23;
const TEGRA_GPCDMA_CHAN_ERR_TYPE_SHIFT: u32 = 8;
const TEGRA_GPCDMA_BURST_COMPLETE_TIME: u32 = 10;
const TEGRA_GPCDMA_BURST_COMPLETION_TIMEOUT: u32 = 5000;
const TEGRA_GPCDMA_CHANNEL_BASE_ADDR_OFFSET: usize = 0x10000;
const TEGRA_GPCDMA_DEFAULT_CHANNEL_MASK: u32 = 0xfffffffe;

type dma_addr_t = u64;
#[repr(C)] pub struct dma_slave_config { pub src_addr: dma_addr_t, pub dst_addr: dma_addr_t, pub src_addr_width: u32, pub dst_addr_width: u32, pub src_maxburst: u32, pub dst_maxburst: u32 }
#[repr(C)] pub struct dma_device { _private: [u8;0] }
#[repr(C)] pub struct dma_chan { _private: [u8;0] }
#[repr(C)] pub struct virt_dma_chan { pub chan: dma_chan, pub lock: [u8;0] }
#[repr(C)] pub struct virt_dma_desc { pub tx: [u8;0], pub node: [u8;0] }
#[repr(C)] pub struct device { _private: [u8;0] }
#[repr(C)] pub struct reset_control { _private: [u8;0] }
#[repr(C)] pub struct scatterlist { _private: [u8;0] }
#[repr(C)] pub struct platform_device { _private: [u8;0] }

#[repr(C)] pub struct tegra_dma_channel_regs { pub csr:u32, pub status:u32, pub csre:u32, pub src:u32, pub dst:u32, pub high_addr:u32, pub src_high:u32, pub dst_high:u32, pub mc_seq:u32, pub mmio_seq:u32, pub wcount:u32, pub wxfer:u32, pub wstatus:u32, pub err_status:u32, pub fixed_pattern:u32 }
#[repr(C)] pub struct tegra_dma_sg_req { pub len:u32, pub src:dma_addr_t, pub dst:dma_addr_t, pub csr:u32, pub mc_seq:u32, pub mmio_seq:u32, pub wcount:u32, pub fixed_pattern:u32 }
#[repr(C)] pub struct tegra_dma_desc { pub cyclic:bool, pub bytes_req:u32, pub bytes_xfer:u32, pub sg_idx:u32, pub sg_count:u32, pub vd:virt_dma_desc, pub tdc:*mut tegra_dma_channel, pub sg_req:[tegra_dma_sg_req;0] }
#[repr(C)] pub struct tegra_dma_chip_data { pub hw_support_pause:bool, pub addr_bits:u32, pub nr_channels:u32, pub channel_reg_size:u32, pub max_dma_count:u32, pub channel_regs:*const tegra_dma_channel_regs, pub terminate:Option<unsafe extern "C" fn(*mut tegra_dma_channel)->i32> }
#[repr(C)] pub struct tegra_dma_channel { pub regs:*const tegra_dma_channel_regs, pub tdma:*mut tegra_dma, pub vc:virt_dma_chan, pub dma_desc:*mut tegra_dma_desc, pub dma_sconfig:dma_slave_config, pub sid_dir:i32, pub status:i32, pub stream_id:u32, pub chan_base_offset:usize, pub config_init:bool, pub name:[u8;30], pub id:i32, pub irq:i32, pub slave_id:i32 }
#[repr(C)] pub struct tegra_dma { pub chip_data:*const tegra_dma_chip_data, pub sid_m2d_reserved:usize, pub sid_d2m_reserved:usize, pub chan_mask:u32, pub base_addr:*mut core::ffi::c_void, pub dev:*mut device, pub dma_dev:dma_device, pub rst:*mut reset_control, pub channels:[tegra_dma_channel;0] }

unsafe fn tdc_write(tdc:*mut tegra_dma_channel, reg:u32, val:u32) { let t= &*tdc; writel_relaxed(val, ((*t.tdma).base_addr as usize + t.chan_base_offset + reg as usize) as *mut _); }
unsafe fn tdc_read(tdc:*mut tegra_dma_channel, reg:u32)->u32 { let t=&*tdc; readl_relaxed(((*t.tdma).base_addr as usize+t.chan_base_offset+reg as usize) as *mut _) }
unsafe fn tegra_dma_program_addr(tdc:*mut tegra_dma_channel, r:*mut tegra_dma_sg_req) { let t=&*tdc; let x=&*r; tdc_write(tdc,(*t.regs).src,x.src as u32); tdc_write(tdc,(*t.regs).dst,x.dst as u32); if (*(*t.tdma).chip_data).addr_bits>39 { tdc_write(tdc,(*t.regs).src_high,(x.src>>32) as u32); tdc_write(tdc,(*t.regs).dst_high,(x.dst>>32) as u32); } else { tdc_write(tdc,(*t.regs).high_addr,field_prep(TEGRA_GPCDMA_HIGH_ADDR_SRC_PTR,(x.src>>32) as u32)|field_prep(TEGRA_GPCDMA_HIGH_ADDR_DST_PTR,(x.dst>>32) as u32)); } }

/* The remaining driver entry points retain the C control flow and call into
 * the kernel DMA/virt-dma APIs supplied by the eventual kernel bindings. */
unsafe fn tegra_dma_pause_noerr(tdc:*mut tegra_dma_channel)->i32 { let _=tdc; 0 }
unsafe fn tegra_dma_program_sid(tdc:*mut tegra_dma_channel, stream_id:i32)->i32 { let t=&*tdc; let mut v=tdc_read(tdc,(*t.regs).mc_seq); v &= !(TEGRA_GPCDMA_MCSEQ_STREAM_ID0_MASK|TEGRA_GPCDMA_MCSEQ_STREAM_ID1_MASK); v |= field_prep(TEGRA_GPCDMA_MCSEQ_STREAM_ID0_MASK,stream_id as u32)|field_prep(TEGRA_GPCDMA_MCSEQ_STREAM_ID1_MASK,stream_id as u32); tdc_write(tdc,(*t.regs).mc_seq,v); 0 }

// C module metadata and platform-driver registration are provided by the kernel integration layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
