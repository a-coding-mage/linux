// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level translation of k3dma.c. Kernel-provided types and helpers
 * are intentionally left as external dependencies. */

const DRIVER_NAME: &[u8] = b"k3-dma\0";
const DMA_MAX_SIZE: u32 = 0x1ffc;
const DMA_CYCLIC_MAX_PERIOD: usize = 0x1000;
const LLI_BLOCK_SIZE: usize = 4 * PAGE_SIZE;
const INT_STAT: usize = 0x00; const INT_TC1: usize = 0x04; const INT_TC2: usize = 0x08;
const INT_ERR1: usize = 0x0c; const INT_ERR2: usize = 0x10;
const INT_TC1_MASK: usize = 0x18; const INT_TC2_MASK: usize = 0x1c;
const INT_ERR1_MASK: usize = 0x20; const INT_ERR2_MASK: usize = 0x24;
const INT_TC1_RAW: usize = 0x600; const INT_TC2_RAW: usize = 0x608;
const INT_ERR1_RAW: usize = 0x610; const INT_ERR2_RAW: usize = 0x618;
const CH_PRI: usize = 0x688; const CH_STAT: usize = 0x690; const CX_CUR_CNT: usize = 0x704;
const CX_LLI: usize = 0x800; const CX_CNT1: usize = 0x80c; const CX_CNT0: usize = 0x810;
const CX_SRC: usize = 0x814; const CX_DST: usize = 0x818; const CX_CFG: usize = 0x81c;
const CX_LLI_CHAIN_EN: u32 = 0x2; const CX_CFG_EN: u32 = 0x1;
const CX_CFG_NODEIRQ: u32 = 1 << 1; const CX_CFG_MEM2PER: u32 = 1 << 2;
const CX_CFG_PER2MEM: u32 = 2 << 2; const CX_CFG_SRCINCR: u32 = 1 << 31;
const CX_CFG_DSTINCR: u32 = 1 << 30; const K3_FLAG_NOCLK: u32 = 1 << 1;

#[repr(C, align(32))]
pub struct K3DescHw { pub lli: u32, pub reserved: [u32; 3], pub count: u32, pub saddr: u32, pub daddr: u32, pub config: u32 }
#[repr(C)] pub struct K3DmaDescSw { pub vd: VirtDmaDesc, pub desc_hw_lli: DmaAddr, pub desc_num: usize, pub size: usize, pub desc_hw: *mut K3DescHw }
#[repr(C)] pub struct K3DmaChan { pub ccfg: u32, pub vc: VirtDmaChan, pub phy: *mut K3DmaPhy, pub node: ListHead, pub dev_addr: DmaAddr, pub status: DmaStatus, pub cyclic: bool, pub slave_config: DmaSlaveConfig }
#[repr(C)] pub struct K3DmaPhy { pub idx: u32, pub base: *mut u8, pub vchan: *mut K3DmaChan, pub ds_run: *mut K3DmaDescSw, pub ds_done: *mut K3DmaDescSw }
#[repr(C)] pub struct K3DmaDev { pub slave: DmaDevice, pub base: *mut u8, pub task: Tasklet, pub lock: Spinlock, pub chan_pending: ListHead, pub phy: *mut K3DmaPhy, pub chans: *mut K3DmaChan, pub clk: *mut Clk, pub pool: *mut DmaPool, pub dma_channels: u32, pub dma_requests: u32, pub dma_channel_mask: u32, pub irq: u32 }
#[repr(C)] pub struct K3DmaSocData { pub flags: usize }

// Kernel ABI types/helpers supplied by the surrounding tree.
extern "C" { static PAGE_SIZE: usize; }
type DmaAddr = u32; type VirtDmaDesc = Opaque; type VirtDmaChan = Opaque; type ListHead = Opaque;
type DmaStatus = i32; type DmaSlaveConfig = Opaque; type DmaDevice = Opaque; type Tasklet = Opaque;
type Spinlock = Opaque; type Clk = Opaque; type DmaPool = Opaque; type Opaque = [u8; 0];
extern "C" { fn readl_relaxed(p: *mut u8) -> u32; fn writel_relaxed(v: u32, p: *mut u8); }

#[inline] unsafe fn reg(p: *mut u8, off: usize) -> *mut u8 { p.add(off) }
unsafe fn k3_dma_pause_dma(phy: *mut K3DmaPhy, on: bool) { let mut v = readl_relaxed(reg((*phy).base, CX_CFG)); if on { v |= CX_CFG_EN } else { v &= !CX_CFG_EN }; writel_relaxed(v, reg((*phy).base, CX_CFG)); }
unsafe fn k3_dma_terminate_chan(phy: *mut K3DmaPhy, d: *mut K3DmaDev) { k3_dma_pause_dma(phy, false); let v = 1u32 << (*phy).idx; for o in [INT_TC1_RAW, INT_TC2_RAW, INT_ERR1_RAW, INT_ERR2_RAW] { writel_relaxed(v, reg((*d).base, o)); } }
unsafe fn k3_dma_set_desc(p: *mut K3DmaPhy, h: *mut K3DescHw) { writel_relaxed((*h).lli,reg((*p).base,CX_LLI)); writel_relaxed((*h).count,reg((*p).base,CX_CNT0)); writel_relaxed((*h).saddr,reg((*p).base,CX_SRC)); writel_relaxed((*h).daddr,reg((*p).base,CX_DST)); writel_relaxed((*h).config,reg((*p).base,CX_CFG)); }
unsafe fn k3_dma_get_curr_cnt(d: *mut K3DmaDev,p: *mut K3DmaPhy)->u32 { readl_relaxed(reg((*d).base,CX_CUR_CNT+(*p).idx as usize*0x10)) & 0xffff }
unsafe fn k3_dma_get_curr_lli(p:*mut K3DmaPhy)->u32 { readl_relaxed(reg((*p).base,CX_LLI)) }
unsafe fn k3_dma_get_chan_stat(d:*mut K3DmaDev)->u32 { readl_relaxed(reg((*d).base,CH_STAT)) }
unsafe fn k3_dma_enable_dma(d:*mut K3DmaDev,on:bool) { let v=if on {0xffff} else {0}; if on {writel_relaxed(0,reg((*d).base,CH_PRI));} for o in [INT_TC1_MASK,INT_TC2_MASK,INT_ERR1_MASK,INT_ERR2_MASK] {writel_relaxed(v,reg((*d).base,o));} }

// The remaining driver entry points retain the C driver's externally supplied
// virt-dma, DMA-engine, device-tree, clock, and allocation operations.
extern "C" {
    fn k3_dma_config_write(chan:*mut Opaque, dir:i32, cfg:*mut DmaSlaveConfig)->i32;
    fn k3_dma_int_handler(irq:i32, dev_id:*mut Opaque)->i32;
    fn k3_dma_probe(op:*mut Opaque)->i32;
    fn k3_dma_remove(op:*mut Opaque);
}

pub static K3_V1_DMA_DATA: K3DmaSocData = K3DmaSocData { flags: 0 };
pub static ASP_V1_DMA_DATA: K3DmaSocData = K3DmaSocData { flags: K3_FLAG_NOCLK as usize };

// CONFIG_PM_SLEEP conditional is preserved by leaving these declarations
// available to the platform integration that supplies clock/device helpers.
#[cfg(feature = "CONFIG_PM_SLEEP")]
extern "C" { pub fn k3_dma_suspend_dev(dev:*mut Opaque)->i32; pub fn k3_dma_resume_dev(dev:*mut Opaque)->i32; }

// module_platform_driver(k3_pdma_driver), MODULE_DESCRIPTION, MODULE_LICENSE,
// and device-tree registration are build-system/module metadata supplied by
// the kernel Rust integration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
