/* SPDX-License-Identifier: GPL-2.0+ */
/* Rust translation of fsl-edma-common.h. External kernel types and helpers are
 * intentionally left as dependencies supplied by other translation units. */

pub const EDMA_CR_EDBG: u32 = 1 << 1;
pub const EDMA_CR_ERCA: u32 = 1 << 2;
pub const EDMA_CR_ERGA: u32 = 1 << 3;
pub const EDMA_CR_HOE: u32 = 1 << 4;
pub const EDMA_CR_HALT: u32 = 1 << 5;
pub const EDMA_CR_CLM: u32 = 1 << 6;
pub const EDMA_CR_EMLM: u32 = 1 << 7;
pub const EDMA_CR_ECX: u32 = 1 << 16;
pub const EDMA_CR_CX: u32 = 1 << 17;

pub const EDMA_TCD_ITER_MASK: u32 = (1 << 15) - 1;
pub const EDMA_TCD_CSR_START: u32 = 1 << 0;
pub const EDMA_TCD_CSR_INT_MAJOR: u32 = 1 << 1;
pub const EDMA_TCD_CSR_INT_HALF: u32 = 1 << 2;
pub const EDMA_TCD_CSR_D_REQ: u32 = 1 << 3;
pub const EDMA_TCD_CSR_E_SG: u32 = 1 << 4;
pub const EDMA_TCD_CSR_E_LINK: u32 = 1 << 5;
pub const EDMA_TCD_CSR_ACTIVE: u32 = 1 << 6;
pub const EDMA_TCD_CSR_DONE: u32 = 1 << 7;

pub const EDMAMUX_CHCFG_DIS: u32 = 0x0;
pub const EDMAMUX_CHCFG_ENBL: u32 = 0x80;
pub const DMAMUX_NR: usize = 2;
pub const EDMA_TCD: u32 = 0x1000;

pub const FSL_EDMA_DRV_HAS_DMACLK: u32 = 1 << 0;
pub const FSL_EDMA_DRV_MUX_SWAP: u32 = 1 << 1;
pub const FSL_EDMA_DRV_CONFIG32: u32 = 1 << 2;
pub const FSL_EDMA_DRV_WRAP_IO: u32 = 1 << 3;
pub const FSL_EDMA_DRV_EDMA64: u32 = 1 << 4;
pub const FSL_EDMA_DRV_HAS_PD: u32 = 1 << 5;
pub const FSL_EDMA_DRV_HAS_CHCLK: u32 = 1 << 6;
pub const FSL_EDMA_DRV_HAS_CHMUX: u32 = 1 << 7;
pub const FSL_EDMA_DRV_MEM_REMOTE: u32 = 1 << 8;
pub const FSL_EDMA_DRV_SPLIT_REG: u32 = 1 << 9;
pub const FSL_EDMA_DRV_BUS_8BYTE: u32 = 1 << 10;
pub const FSL_EDMA_DRV_DEV_TO_DEV: u32 = 1 << 11;
pub const FSL_EDMA_DRV_ALIGN_64BYTE: u32 = 1 << 12;
pub const FSL_EDMA_DRV_CLEAR_DONE_E_SG: u32 = 1 << 13;
pub const FSL_EDMA_DRV_CLEAR_DONE_E_LINK: u32 = 1 << 14;
pub const FSL_EDMA_DRV_TCD64: u32 = 1 << 15;
pub const FSL_EDMA_DRV_ERRIRQ_SHARE: u32 = 1 << 16;
pub const FSL_EDMA_DRV_EDMA3: u32 = FSL_EDMA_DRV_SPLIT_REG | FSL_EDMA_DRV_BUS_8BYTE | FSL_EDMA_DRV_DEV_TO_DEV | FSL_EDMA_DRV_ALIGN_64BYTE | FSL_EDMA_DRV_CLEAR_DONE_E_SG | FSL_EDMA_DRV_CLEAR_DONE_E_LINK;
pub const FSL_EDMA_DRV_EDMA4: u32 = FSL_EDMA_DRV_SPLIT_REG | FSL_EDMA_DRV_BUS_8BYTE | FSL_EDMA_DRV_DEV_TO_DEV | FSL_EDMA_DRV_ALIGN_64BYTE | FSL_EDMA_DRV_CLEAR_DONE_E_SG | FSL_EDMA_DRV_CLEAR_DONE_E_LINK;

#[inline] pub const fn EDMA_SEEI_SEEI(x:u32)->u32 { x & 0x1f }
#[inline] pub const fn EDMA_CEEI_CEEI(x:u32)->u32 { x & 0x1f }
#[inline] pub const fn EDMA_CINT_CINT(x:u32)->u32 { x & 0x1f }
#[inline] pub const fn EDMA_CERR_CERR(x:u32)->u32 { x & 0x1f }
#[inline] pub const fn EDMA_TCD_ATTR_DSIZE(x:u32)->u32 { x & 7 }
#[inline] pub const fn EDMA_TCD_ATTR_DMOD(x:u32)->u32 { (x & 31) << 3 }
#[inline] pub const fn EDMA_TCD_ATTR_SSIZE(x:u32)->u32 { (x & 7) << 8 }
#[inline] pub const fn EDMA_TCD_ATTR_SMOD(x:u32)->u32 { (x & 31) << 11 }
#[inline] pub const fn EDMA_TCD_CITER_CITER(x:u32)->u32 { x & EDMA_TCD_ITER_MASK }
#[inline] pub const fn EDMA_TCD_BITER_BITER(x:u32)->u32 { x & EDMA_TCD_ITER_MASK }
#[inline] pub const fn EDMA_V3_TCD_NBYTES_MLOFF_NBYTES(x:u32)->u32 { x & 0x3ff }
#[inline] pub const fn EDMA_V3_TCD_NBYTES_MLOFF(x:u32)->u32 { x << 10 }
pub const EDMA_V3_TCD_NBYTES_DMLOE:u32=1<<30; pub const EDMA_V3_TCD_NBYTES_SMLOE:u32=1<<31;
#[inline] pub const fn EDMAMUX_CHCFG_SOURCE(x:u32)->u32 { x & 0x3f }
pub const FSL_EDMA_BUSWIDTHS:u32=(1<<0)|(1<<1)|(1<<2)|(1<<3);
pub const EDMA_V3_CH_SBR_RD:u32=1<<22; pub const EDMA_V3_CH_SBR_WR:u32=1<<21;
pub const EDMA_V3_CH_CSR_ERQ:u32=1; pub const EDMA_V3_CH_CSR_EARQ:u32=2; pub const EDMA_V3_CH_CSR_EEI:u32=4; pub const EDMA_V3_CH_CSR_DONE:u32=1<<30; pub const EDMA_V3_CH_CSR_ACTIVE:u32=1<<31; pub const EDMA_V3_CH_ES_ERR:u32=1<<31; pub const EDMA_V3_MP_ES_VLD:u32=1<<31;
pub const EDMA_V3_CH_ERR_DBE:u32=1<<0; pub const EDMA_V3_CH_ERR_SBE:u32=1<<1; pub const EDMA_V3_CH_ERR_SGE:u32=1<<2; pub const EDMA_V3_CH_ERR_NCE:u32=1<<3; pub const EDMA_V3_CH_ERR_DOE:u32=1<<4; pub const EDMA_V3_CH_ERR_DAE:u32=1<<5; pub const EDMA_V3_CH_ERR_SOE:u32=1<<6; pub const EDMA_V3_CH_ERR_SAE:u32=1<<7; pub const EDMA_V3_CH_ERR_ECX:u32=1<<8; pub const EDMA_V3_CH_ERR_UCE:u32=1<<9; pub const EDMA_V3_CH_ERR:u32=1<<31;

#[repr(C)] pub enum fsl_edma_pm_state { RUNNING = 0, SUSPENDED }
#[repr(C)] pub struct fsl_edma_hw_tcd { pub saddr: u32, pub soff: u16, pub attr: u16, pub nbytes: u32, pub slast: u32, pub daddr: u32, pub doff: u16, pub citer: u16, pub dlast_sga: u32, pub csr: u16, pub biter: u16 }
#[repr(C, packed)] pub struct fsl_edma_hw_tcd64 { pub saddr: u64, pub soff: u16, pub attr: u16, pub nbytes: u32, pub slast: u64, pub daddr: u64, pub dlast_sga: u64, pub doff: u16, pub citer: u16, pub csr: u16, pub biter: u16 }
#[repr(C, packed)] pub union fsl_edma3_ch_reg { pub tcd: fsl_edma_hw_tcd, pub tcd64: fsl_edma_hw_tcd64, pub ch_csr: u32 }

#[repr(C)] pub struct edma_regs { pub cr:*mut core::ffi::c_void, pub es:*mut core::ffi::c_void, pub erqh:*mut core::ffi::c_void, pub erql:*mut core::ffi::c_void, pub eeih:*mut core::ffi::c_void, pub eeil:*mut core::ffi::c_void, pub seei:*mut core::ffi::c_void, pub ceei:*mut core::ffi::c_void, pub serq:*mut core::ffi::c_void, pub cerq:*mut core::ffi::c_void, pub cint:*mut core::ffi::c_void, pub cerr:*mut core::ffi::c_void, pub ssrt:*mut core::ffi::c_void, pub cdne:*mut core::ffi::c_void, pub inth:*mut core::ffi::c_void, pub intl:*mut core::ffi::c_void, pub errh:*mut core::ffi::c_void, pub errl:*mut core::ffi::c_void }
#[repr(C)] pub struct fsl_edma_sw_tcd { pub ptcd: u64, pub vtcd:*mut core::ffi::c_void }

/* The following structures retain the source fields; referenced kernel types
 * are external dependencies. */
#[repr(C)] pub struct fsl_edma_chan { pub vchan: virt_dma_chan, pub status: dma_status, pub pm_state: fsl_edma_pm_state, pub edma:*mut fsl_edma_engine, pub edesc:*mut fsl_edma_desc, pub cfg: dma_slave_config, pub attr:u32, pub is_sw:bool, pub tcd_pool:*mut dma_pool, pub dma_dev_addr:u64, pub dma_dev_size:u32, pub dma_dir:dma_data_direction, pub chan_name:[i8;32], pub errirq_name:[i8;36], pub tcd:*mut core::ffi::c_void, pub mux_addr:*mut core::ffi::c_void, pub real_count:u32, pub issue_worker: work_struct, pub pdev:*mut platform_device, pub pd_dev:*mut device, pub pd_dev_link:*mut device_link, pub srcid:u32, pub clk:*mut clk, pub priority:i32, pub hw_chanid:i32, pub txirq:i32, pub errirq:i32, pub irq_handler: Option<unsafe extern "C" fn(i32,*mut core::ffi::c_void)->irqreturn_t>, pub errirq_handler: Option<unsafe extern "C" fn(i32,*mut core::ffi::c_void)->irqreturn_t>, pub is_rxchan:bool, pub is_remote:bool, pub is_multi_fifo:bool }
#[repr(C)] pub struct fsl_edma_desc { pub vdesc: virt_dma_desc, pub echan:*mut fsl_edma_chan, pub iscyclic:bool, pub dirn:dma_transfer_direction, pub n_tcds:u32, pub tcd:[fsl_edma_sw_tcd;0] }
#[repr(C)] pub struct fsl_edma_drvdata { pub dmamuxs:u32, pub chreg_off:u32, pub chreg_space_sz:u32, pub flags:u32, pub mux_off:u32, pub mux_skip:u32, pub setup_irq: Option<unsafe extern "C" fn(*mut platform_device,*mut fsl_edma_engine)->i32> }
#[repr(C)] pub struct fsl_edma_engine { pub dma_dev:dma_device, pub membase:*mut core::ffi::c_void, pub muxbase:[*mut core::ffi::c_void;DMAMUX_NR], pub muxclk:[*mut clk;DMAMUX_NR], pub dmaclk:*mut clk, pub fsl_edma_mutex:mutex, pub drvdata:*const fsl_edma_drvdata, pub n_chans:u32, pub txirq:i32, pub txirq_16_31:i32, pub errirq:i32, pub big_endian:bool, pub regs:edma_regs, pub chan_masked:u64, pub chans:[fsl_edma_chan;0] }

#[inline] pub unsafe fn fsl_edma_drvflags(c:*mut fsl_edma_chan)->u32 { (*(*c).edma).drvdata.as_ref().unwrap().flags }

#[inline] pub unsafe fn fsl_edma_err_chan_handler(c:*mut fsl_edma_chan) { (*c).status = DMA_ERROR; }

extern "C" { pub fn fsl_edma_tx_chan_handler(*mut fsl_edma_chan); pub fn fsl_edma_disable_request(*mut fsl_edma_chan); pub fn fsl_edma_chan_mux(*mut fsl_edma_chan,u32,bool); pub fn fsl_edma_free_desc(*mut virt_dma_desc); pub fn fsl_edma_terminate_all(*mut dma_chan)->i32; pub fn fsl_edma_pause(*mut dma_chan)->i32; pub fn fsl_edma_resume(*mut dma_chan)->i32; pub fn fsl_edma_slave_config(*mut dma_chan,*mut dma_slave_config)->i32; pub fn fsl_edma_tx_status(*mut dma_chan,u32,*mut dma_tx_state)->dma_status; pub fn fsl_edma_prep_dma_cyclic(*mut dma_chan,u64,usize,usize,dma_transfer_direction,usize)->*mut dma_async_tx_descriptor; pub fn fsl_edma_prep_slave_sg(*mut dma_chan,*mut scatterlist,u32,dma_transfer_direction,usize,*mut core::ffi::c_void)->*mut dma_async_tx_descriptor; pub fn fsl_edma_prep_memcpy(*mut dma_chan,u64,u64,usize,usize)->*mut dma_async_tx_descriptor; pub fn fsl_edma_xfer_desc(*mut fsl_edma_chan); pub fn fsl_edma_issue_pending(*mut dma_chan); pub fn fsl_edma_alloc_chan_resources(*mut dma_chan)->i32; pub fn fsl_edma_free_chan_resources(*mut dma_chan); pub fn fsl_edma_cleanup_vchan(*mut dma_device); pub fn fsl_edma_setup_regs(*mut fsl_edma_engine); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
