/* SPDX-License-Identifier: GPL-2.0 */
/* Driver for the Synopsys DesignWare AHB DMA Controller */

// External Linux/kernel types and helpers are supplied by other translated dependencies.

pub const DW_DMA_MAX_NR_REQUESTS: usize = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dw_dma_fc { DW_DMA_FC_D_M2M, DW_DMA_FC_D_M2P, DW_DMA_FC_D_P2M, DW_DMA_FC_D_P2P, DW_DMA_FC_P_P2M, DW_DMA_FC_SP_P2P, DW_DMA_FC_P_M2P, DW_DMA_FC_DP_P2P }

#[repr(C)]
pub struct dw_dma_chan_regs {
    pub SAR: u32, pub __pad_SAR: u32, pub DAR: u32, pub __pad_DAR: u32,
    pub LLP: u32, pub __pad_LLP: u32, pub CTL_LO: u32, pub CTL_HI: u32,
    pub SSTAT: u32, pub __pad_SSTAT: u32, pub DSTAT: u32, pub __pad_DSTAT: u32,
    pub SSTATAR: u32, pub __pad_SSTATAR: u32, pub DSTATAR: u32, pub __pad_DSTATAR: u32,
    pub CFG_LO: u32, pub CFG_HI: u32, pub SGR: u32, pub __pad_SGR: u32,
    pub DSR: u32, pub __pad_DSR: u32,
}

#[repr(C)]
pub struct dw_dma_irq_regs { pub XFER:u32, pub __pad_XFER:u32, pub BLOCK:u32, pub __pad_BLOCK:u32, pub SRC_TRAN:u32, pub __pad_SRC_TRAN:u32, pub DST_TRAN:u32, pub __pad_DST_TRAN:u32, pub ERROR:u32, pub __pad_ERROR:u32 }

#[repr(C)]
pub struct dw_dma_regs {
    pub CHAN: [dw_dma_chan_regs; DW_DMA_MAX_NR_CHANNELS],
    pub RAW: dw_dma_irq_regs, pub STATUS: dw_dma_irq_regs, pub MASK: dw_dma_irq_regs, pub CLEAR: dw_dma_irq_regs,
    pub STATUS_INT:u32, pub __pad_STATUS_INT:u32, pub REQ_SRC:u32, pub __pad_REQ_SRC:u32, pub REQ_DST:u32, pub __pad_REQ_DST:u32,
    pub SGL_REQ_SRC:u32, pub __pad_SGL_REQ_SRC:u32, pub SGL_REQ_DST:u32, pub __pad_SGL_REQ_DST:u32, pub LAST_SRC:u32, pub __pad_LAST_SRC:u32,
    pub LAST_DST:u32, pub __pad_LAST_DST:u32, pub CFG:u32, pub __pad_CFG:u32, pub CH_EN:u32, pub __pad_CH_EN:u32, pub ID:u32, pub __pad_ID:u32,
    pub TEST:u32, pub __pad_TEST:u32, pub CLASS_PRIORITY0:u32, pub __pad_CLASS_PRIORITY0:u32, pub CLASS_PRIORITY1:u32, pub __pad_CLASS_PRIORITY1:u32,
    pub __reserved:u32, pub DWC_PARAMS:[u32; DW_DMA_MAX_NR_CHANNELS], pub MULTI_BLK_TYPE:u32, pub MAX_BLK_SIZE:u32, pub DW_PARAMS:u32,
    pub COMP_TYPE:u32, pub COMP_VERSION:u32, pub FIFO_PARTITION0:u32, pub __pad_FIFO_PARTITION0:u32, pub FIFO_PARTITION1:u32, pub __pad_FIFO_PARTITION1:u32,
    pub SAI_ERR:u32, pub __pad_SAI_ERR:u32, pub GLOBAL_CFG:u32, pub __pad_GLOBAL_CFG:u32,
}

pub const DW_PARAMS_NR_CHAN:u32=8; pub const DW_PARAMS_NR_MASTER:u32=11;
pub const DW_PARAMS_DATA_WIDTH1:u32=15; pub const DW_PARAMS_DATA_WIDTH2:u32=17; pub const DW_PARAMS_DATA_WIDTH3:u32=19; pub const DW_PARAMS_DATA_WIDTH4:u32=21; pub const DW_PARAMS_EN:u32=28;
#[inline] pub const fn DW_PARAMS_DATA_WIDTH(n:u32)->u32 { 15+2*n }
pub const DWC_PARAMS_MBLK_EN:u32=11; pub const DWC_PARAMS_HC_LLP:u32=13; pub const DWC_PARAMS_MSIZE:u32=16;

#[repr(C)] #[derive(Copy,Clone)] pub enum dw_dma_msize { DW_DMA_MSIZE_1, DW_DMA_MSIZE_4, DW_DMA_MSIZE_8, DW_DMA_MSIZE_16, DW_DMA_MSIZE_32, DW_DMA_MSIZE_64, DW_DMA_MSIZE_128, DW_DMA_MSIZE_256 }
#[inline] pub const fn DWC_LLP_LMS(x:u32)->u32{x&3} #[inline] pub const fn DWC_LLP_LOC(x:u32)->u32{x&!3}

pub const DWC_CTLL_INT_EN:u32=1<<0; pub const DWC_CTLL_DST_INC:u32=0<<7; pub const DWC_CTLL_DST_DEC:u32=1<<7; pub const DWC_CTLL_DST_FIX:u32=2<<7; pub const DWC_CTLL_SRC_INC:u32=0<<9; pub const DWC_CTLL_SRC_DEC:u32=1<<9; pub const DWC_CTLL_SRC_FIX:u32=2<<9; pub const DWC_CTLL_S_GATH_EN:u32=1<<17; pub const DWC_CTLL_D_SCAT_EN:u32=1<<18; pub const DWC_CTLL_FC_M2M:u32=0<<20; pub const DWC_CTLL_FC_M2P:u32=1<<20; pub const DWC_CTLL_FC_P2M:u32=2<<20; pub const DWC_CTLL_FC_P2P:u32=3<<20; pub const DWC_CTLL_LLP_D_EN:u32=1<<27; pub const DWC_CTLL_LLP_S_EN:u32=1<<28;
#[inline] pub const fn DWC_CTLL_DST_WIDTH(n:u32)->u32{n<<1} #[inline] pub const fn DWC_CTLL_SRC_WIDTH(n:u32)->u32{n<<4} #[inline] pub const fn DWC_CTLL_DST_MSIZE(n:u32)->u32{n<<11} #[inline] pub const fn DWC_CTLL_SRC_MSIZE(n:u32)->u32{n<<14} #[inline] pub const fn DWC_CTLL_FC(n:u32)->u32{n<<20} #[inline] pub const fn DWC_CTLL_DMS(n:u32)->u32{n<<23} #[inline] pub const fn DWC_CTLL_SMS(n:u32)->u32{n<<25}

pub const DWC_CTLH_BLOCK_TS_MASK:u32=0xfff; pub const DWC_CTLH_DONE:u32=1<<12; #[inline] pub const fn DWC_CTLH_BLOCK_TS(x:u32)->u32{x&DWC_CTLH_BLOCK_TS_MASK}
pub const DWC_CFGL_CH_PRIOR_MASK:u32=0x7<<5; pub const DWC_CFGL_CH_SUSP:u32=1<<8; pub const DWC_CFGL_FIFO_EMPTY:u32=1<<9; pub const DWC_CFGL_HS_DST:u32=1<<10; pub const DWC_CFGL_HS_SRC:u32=1<<11; pub const DWC_CFGL_LOCK_CH_XFER:u32=0<<12; pub const DWC_CFGL_LOCK_CH_BLOCK:u32=1<<12; pub const DWC_CFGL_LOCK_CH_XACT:u32=2<<12; pub const DWC_CFGL_LOCK_BUS_XFER:u32=0<<14; pub const DWC_CFGL_LOCK_BUS_BLOCK:u32=1<<14; pub const DWC_CFGL_LOCK_BUS_XACT:u32=2<<14; pub const DWC_CFGL_LOCK_CH:u32=1<<15; pub const DWC_CFGL_LOCK_BUS:u32=1<<16; pub const DWC_CFGL_HS_DST_POL:u32=1<<18; pub const DWC_CFGL_HS_SRC_POL:u32=1<<19; pub const DWC_CFGL_RELOAD_SAR:u32=1<<30; pub const DWC_CFGL_RELOAD_DAR:u32=1<<31;
#[inline] pub const fn DWC_CFGL_CH_PRIOR(x:u32)->u32{x<<5} #[inline] pub const fn DWC_CFGL_MAX_BURST(x:u32)->u32{x<<20}
pub const DWC_CFGH_FCMODE:u32=1; pub const DWC_CFGH_FIFO_MODE:u32=2; pub const DWC_CFGH_PROTCTL_DATA:u32=0; pub const DWC_CFGH_PROTCTL_PRIV:u32=1<<2; pub const DWC_CFGH_PROTCTL_BUFFER:u32=2<<2; pub const DWC_CFGH_PROTCTL_CACHE:u32=4<<2; pub const DWC_CFGH_DS_UPD_EN:u32=1<<5; pub const DWC_CFGH_SS_UPD_EN:u32=1<<6; pub const DW_CFG_DMA_EN:u32=1;
#[inline] pub const fn DWC_CFGH_PROTCTL(x:u32)->u32{x<<2} #[inline] pub const fn DWC_CFGH_SRC_PER(x:u32)->u32{x<<7} #[inline] pub const fn DWC_CFGH_DST_PER(x:u32)->u32{x<<11} #[inline] pub const fn DWC_SGR_SGI(x:u32)->u32{x} #[inline] pub const fn DWC_SGR_SGC(x:u32)->u32{x<<20} #[inline] pub const fn DWC_DSR_DSI(x:u32)->u32{x} #[inline] pub const fn DWC_DSR_DSC(x:u32)->u32{x<<20}

#[repr(C)] #[derive(Copy,Clone)] pub enum idma32_msize { IDMA32_MSIZE_1, IDMA32_MSIZE_2, IDMA32_MSIZE_4, IDMA32_MSIZE_8, IDMA32_MSIZE_16, IDMA32_MSIZE_32 }
pub const IDMA32C_CTLH_BLOCK_TS_MASK:u32=0x1ffff; pub const IDMA32C_CTLH_DONE:u32=1<<17; #[inline] pub const fn IDMA32C_CTLH_BLOCK_TS(x:u32)->u32{x&IDMA32C_CTLH_BLOCK_TS_MASK}
pub const IDMA32C_CFGL_DST_BURST_ALIGN:u32=1; pub const IDMA32C_CFGL_SRC_BURST_ALIGN:u32=1<<1; pub const IDMA32C_CFGL_CH_DRAIN:u32=1<<10; pub const IDMA32C_CFGL_DST_OPT_BL:u32=1<<20; pub const IDMA32C_CFGL_SRC_OPT_BL:u32=1<<21;
#[inline] pub const fn IDMA32C_CFGH_SRC_PER(x:u32)->u32{x} #[inline] pub const fn IDMA32C_CFGH_DST_PER(x:u32)->u32{x<<4} #[inline] pub const fn IDMA32C_CFGH_RD_ISSUE_THD(x:u32)->u32{x<<8} #[inline] pub const fn IDMA32C_CFGH_RW_ISSUE_THD(x:u32)->u32{x<<18} #[inline] pub const fn IDMA32C_CFGH_SRC_PER_EXT(x:u32)->u32{x<<28} #[inline] pub const fn IDMA32C_CFGH_DST_PER_EXT(x:u32)->u32{x<<30} #[inline] pub const fn IDMA32C_FP_PSIZE_CH0(x:u32)->u32{x} #[inline] pub const fn IDMA32C_FP_PSIZE_CH1(x:u32)->u32{x<<13} pub const IDMA32C_FP_UPDATE:u32=1<<26;

#[repr(C)] #[derive(Copy,Clone)] pub enum dw_dmac_flags { DW_DMA_IS_CYCLIC=0, DW_DMA_IS_SOFT_LLP=1, DW_DMA_IS_PAUSED=2, DW_DMA_IS_INITIALIZED=3 }

// The following structs retain kernel-provided field types and helpers verbatim in Rust form.
#[repr(C)] pub struct dw_dma_chan { pub chan: dma_chan, pub ch_regs:*mut dw_dma_chan_regs, pub mask:u8, pub priority:u8, pub direction:dma_transfer_direction, pub tx_node_active:*mut list_head, pub lock:spinlock_t, pub flags: c_ulong, pub active_list:list_head, pub queue:list_head, pub descs_allocated:c_uint, pub block_size:c_uint, pub nollp:bool, pub max_burst:u32, pub dws:dw_dma_slave, pub dma_sconfig:dma_slave_config }
#[inline] pub unsafe fn __dwc_regs(dwc:*mut dw_dma_chan)->*mut dw_dma_chan_regs { (*dwc).ch_regs }
#[macro_export] macro_rules! channel_readl { ($dwc:expr,$name:ident) => { readl(unsafe { &(*$crate::__dwc_regs($dwc)).$name }) }; }
#[macro_export] macro_rules! channel_writel { ($dwc:expr,$name:ident,$val:expr) => { writel($val, unsafe { &mut (*$crate::__dwc_regs($dwc)).$name }) }; }
#[inline] pub unsafe fn to_dw_dma_chan(chan:*mut dma_chan)->*mut dw_dma_chan { container_of(chan, dw_dma_chan, chan) }
#[repr(C)] pub struct dw_dma { pub dma:dma_device, pub name:[c_char;20], pub regs:*mut dw_dma_regs, pub desc_pool:*mut dma_pool, pub tasklet:tasklet_struct, pub chan:*mut dw_dma_chan, pub all_chan_mask:u8, pub in_use:u8, pub initialize_chan:Option<unsafe extern "C" fn(*mut dw_dma_chan)>, pub suspend_chan:Option<unsafe extern "C" fn(*mut dw_dma_chan,bool)>, pub resume_chan:Option<unsafe extern "C" fn(*mut dw_dma_chan,bool)>, pub prepare_ctllo:Option<unsafe extern "C" fn(*mut dw_dma_chan)->u32>, pub bytes2block:Option<unsafe extern "C" fn(*mut dw_dma_chan,usize,c_uint,*mut usize)->u32>, pub block2bytes:Option<unsafe extern "C" fn(*mut dw_dma_chan,u32,u32)->usize>, pub set_device_name:Option<unsafe extern "C" fn(*mut dw_dma,c_int)>, pub disable:Option<unsafe extern "C" fn(*mut dw_dma)>, pub enable:Option<unsafe extern "C" fn(*mut dw_dma)>, pub pdata:*mut dw_dma_platform_data }
#[inline] pub unsafe fn __dw_regs(dw:*mut dw_dma)->*mut dw_dma_regs{(*dw).regs} #[inline] pub unsafe fn to_dw_dma(ddev:*mut dma_device)->*mut dw_dma{container_of(ddev,dw_dma,dma)}
#[macro_export] macro_rules! dma_readl { ($dw:expr,$name:ident) => { readl(unsafe { &(*$crate::__dw_regs($dw)).$name }) }; }
#[macro_export] macro_rules! dma_writel { ($dw:expr,$name:ident,$val:expr) => { writel($val, unsafe { &mut (*$crate::__dw_regs($dw)).$name }) }; }
#[macro_export] macro_rules! idma32_readq { ($dw:expr,$name:ident) => { hi_lo_readq(unsafe { &(*$crate::__dw_regs($dw)).$name }) }; }
#[macro_export] macro_rules! idma32_writeq { ($dw:expr,$name:ident,$val:expr) => { hi_lo_writeq($val, unsafe { &mut (*$crate::__dw_regs($dw)).$name }) }; }
#[macro_export] macro_rules! channel_set_bit { ($dw:expr,$reg:ident,$mask:expr) => { dma_writel!($dw,$reg,(($mask)<<8)|($mask)) }; }
#[macro_export] macro_rules! channel_clear_bit { ($dw:expr,$reg:ident,$mask:expr) => { dma_writel!($dw,$reg,(($mask)<<8)|0) }; }
#[repr(C)] pub struct dw_lli { pub sar:__le32,pub dar:__le32,pub llp:__le32,pub ctllo:__le32,pub ctlhi:__le32,pub sstat:__le32,pub dstat:__le32 }
#[repr(C)] pub struct dw_desc { pub lli:dw_lli,pub desc_node:list_head,pub tx_list:list_head,pub txd:dma_async_tx_descriptor,pub len:usize,pub total_len:usize,pub residue:u32 }
#[macro_export] macro_rules! lli_set { ($d:expr,$reg:ident,$v:expr) => { $d.lli.$reg |= cpu_to_le32($v) }; }
#[macro_export] macro_rules! lli_clear { ($d:expr,$reg:ident,$v:expr) => { $d.lli.$reg &= !cpu_to_le32($v) }; }
#[macro_export] macro_rules! lli_read { ($d:expr,$reg:ident) => { le32_to_cpu($d.lli.$reg) }; }
#[macro_export] macro_rules! lli_write { ($d:expr,$reg:ident,$v:expr) => { $d.lli.$reg = cpu_to_le32($v) }; }
#[macro_export] macro_rules! to_dw_desc { ($h:expr) => { list_entry!($h,dw_desc,desc_node) }; }
#[inline] pub unsafe fn txd_to_dw_desc(txd:*mut dma_async_tx_descriptor)->*mut dw_desc{container_of(txd,dw_desc,txd)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
