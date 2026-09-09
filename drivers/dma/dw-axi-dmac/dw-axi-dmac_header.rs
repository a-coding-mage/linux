/* SPDX-License-Identifier: GPL-2.0 */
// (C) 2017-2018 Synopsys, Inc. (www.synopsys.com)
// Synopsys DesignWare AXI DMA Controller driver.
// Author: Eugeniy Paltsev <Eugeniy.Paltsev@synopsys.com>
//
// Linux dependencies and the C header guard are intentionally omitted; the
// referenced kernel types and helpers are supplied by other translation units.

pub const DMAC_MAX_CHANNELS: usize = 32;
pub const DMAC_MAX_MASTERS: u32 = 2;
pub const DMAC_MAX_BLK_SIZE: u32 = 0x200000;

#[repr(C)]
pub struct dw_axi_dma_hcfg {
    pub nr_channels: u32,
    pub nr_masters: u32,
    pub m_data_width: u32,
    pub block_size: [u32; DMAC_MAX_CHANNELS],
    pub priority: [u32; DMAC_MAX_CHANNELS],
    pub axi_rw_burst_len: u32,
    pub reg_map_8_channels: bool,
    pub restrict_axi_burst_len: bool,
    pub use_cfg2: bool,
    pub use_handshake_as_channel_number: bool,
}

#[repr(C)]
pub struct axi_dma_chan {
    pub chip: *mut axi_dma_chip,
    pub chan_regs: *mut core::ffi::c_void,
    pub id: u8,
    pub hw_handshake_num: u8,
    pub descs_allocated: atomic_t,
    pub desc_pool: *mut dma_pool,
    pub vc: virt_dma_chan,
    pub desc: *mut axi_dma_desc,
    pub config: dma_slave_config,
    pub direction: dma_transfer_direction,
    pub cyclic: bool,
    pub is_paused: bool,
}

#[repr(C)]
pub struct dw_axi_dma {
    pub dma: dma_device,
    pub hdata: *mut dw_axi_dma_hcfg,
    pub dma_parms: device_dma_parameters,
    pub chan: *mut axi_dma_chan,
}

#[repr(C)]
pub struct axi_dma_chip {
    pub dev: *mut device,
    pub irq: [i32; DMAC_MAX_CHANNELS],
    pub regs: *mut core::ffi::c_void,
    pub apb_regs: *mut core::ffi::c_void,
    pub core_clk: *mut clk,
    pub cfgr_clk: *mut clk,
    pub dw: *mut dw_axi_dma,
}

#[repr(C, packed)]
pub struct axi_dma_lli {
    pub sar: __le64, pub dar: __le64,
    pub block_ts_lo: __le32, pub block_ts_hi: __le32,
    pub llp: __le64,
    pub ctl_lo: __le32, pub ctl_hi: __le32,
    pub sstat: __le32, pub dstat: __le32,
    pub status_lo: __le32, pub status_hi: __le32,
    pub reserved_lo: __le32, pub reserved_hi: __le32,
}

#[repr(C)]
pub struct axi_dma_hw_desc { pub lli: *mut axi_dma_lli, pub llp: dma_addr_t, pub len: u32 }

#[repr(C)]
pub struct axi_dma_desc {
    pub hw_desc: *mut axi_dma_hw_desc,
    pub vd: virt_dma_desc,
    pub chan: *mut axi_dma_chan,
    pub completed_blocks: u32,
    pub length: u32,
    pub period_len: u32,
    pub nr_hw_descs: u32,
}

#[repr(C)]
pub struct axi_dma_chan_config {
    pub dst_multblk_type: u8, pub src_multblk_type: u8,
    pub dst_per: u8, pub src_per: u8, pub tt_fc: u8, pub prior: u8,
    pub hs_sel_dst: u8, pub hs_sel_src: u8,
}

#[inline]
pub unsafe fn dchan2dev(dchan: *mut dma_chan) -> *mut device { &mut (*(*dchan).dev).device }
#[inline]
pub unsafe fn chan2dev(chan: *mut axi_dma_chan) -> *mut device { &mut (*(*(*chan).vc.chan).dev).device }
#[inline]
pub unsafe fn vd_to_axi_desc(vd: *mut virt_dma_desc) -> *mut axi_dma_desc { container_of!(vd, axi_dma_desc, vd) }
#[inline]
pub unsafe fn vc_to_axi_dma_chan(vc: *mut virt_dma_chan) -> *mut axi_dma_chan { container_of!(vc, axi_dma_chan, vc) }
#[inline]
pub unsafe fn dchan_to_axi_dma_chan(dchan: *mut dma_chan) -> *mut axi_dma_chan { vc_to_axi_dma_chan(to_virt_chan(dchan)) }

pub const COMMON_REG_LEN: u32 = 0x100; pub const CHAN_REG_LEN: u32 = 0x100;
pub const DMAC_ID: u32 = 0x000; pub const DMAC_COMPVER: u32 = 0x008; pub const DMAC_CFG: u32 = 0x010;
pub const DMAC_CHEN: u32 = 0x018; pub const DMAC_CHEN_L: u32 = 0x018; pub const DMAC_CHEN_H: u32 = 0x01c;
pub const DMAC_CHSUSPREG: u32 = 0x020; pub const DMAC_CHABORTREG: u32 = 0x028; pub const DMAC_INTSTATUS: u32 = 0x030;
pub const DMAC_COMMON_INTCLEAR: u32 = 0x038; pub const DMAC_COMMON_INTSTATUS_ENA: u32 = 0x040;
pub const DMAC_COMMON_INTSIGNAL_ENA: u32 = 0x048; pub const DMAC_COMMON_INTSTATUS: u32 = 0x050; pub const DMAC_RESET: u32 = 0x058;

pub const CH_SAR:u32=0x000; pub const CH_DAR:u32=0x008; pub const CH_BLOCK_TS:u32=0x010; pub const CH_CTL:u32=0x018;
pub const CH_CTL_L:u32=0x018; pub const CH_CTL_H:u32=0x01c; pub const CH_CFG:u32=0x020; pub const CH_CFG_L:u32=0x020; pub const CH_CFG_H:u32=0x024;
pub const CH_LLP:u32=0x028; pub const CH_STATUS:u32=0x030; pub const CH_SWHSSRC:u32=0x038; pub const CH_SWHSDST:u32=0x040;
pub const CH_BLK_TFR_RESUMEREQ:u32=0x048; pub const CH_AXI_ID:u32=0x050; pub const CH_AXI_QOS:u32=0x058; pub const CH_SSTAT:u32=0x060; pub const CH_DSTAT:u32=0x068;
pub const CH_SSTATAR:u32=0x070; pub const CH_DSTATAR:u32=0x078; pub const CH_INTSTATUS_ENA:u32=0x080; pub const CH_INTSTATUS:u32=0x088; pub const CH_INTSIGNAL_ENA:u32=0x090; pub const CH_INTCLEAR:u32=0x098;

pub const DMAC_APB_CFG:u32=0; pub const DMAC_APB_STAT:u32=4; pub const DMAC_APB_DEBUG_STAT_0:u32=8; pub const DMAC_APB_DEBUG_STAT_1:u32=0xc;
pub const DMAC_APB_HW_HS_SEL_0:u32=0x10; pub const DMAC_APB_HW_HS_SEL_1:u32=0x14; pub const DMAC_APB_LPI:u32=0x18; pub const DMAC_APB_BYTE_WR_CH_EN:u32=0x1c; pub const DMAC_APB_HALFWORD_WR_CH_EN:u32=0x20;
pub const UNUSED_CHANNEL:u32=0x3f; pub const DMA_APB_HS_SEL_BIT_SIZE:u32=8; pub const DMA_APB_HS_SEL_MASK:u32=0xff; pub const MAX_BLOCK_SIZE:u32=0x1000; pub const DMA_REG_MAP_CH_REF:u32=8;

pub const DMAC_EN_POS:u32=0; pub const DMAC_EN_MASK:u32=1<<DMAC_EN_POS; pub const INT_EN_POS:u32=1; pub const INT_EN_MASK:u32=1<<INT_EN_POS;
pub const DMAC_CHAN_EN_SHIFT:u32=0; pub const DMAC_CHAN_EN_WE_SHIFT:u32=8; pub const DMAC_CHAN_SUSP_SHIFT:u32=16; pub const DMAC_CHAN_SUSP_WE_SHIFT:u32=24;
pub const DMAC_CHAN_EN2_WE_SHIFT:u32=16; pub const DMAC_CHAN_BLOCK_SHIFT:u32=32; pub const DMAC_CHAN_16:u32=16; pub const DMAC_CHAN_SUSP2_SHIFT:u32=0; pub const DMAC_CHAN_SUSP2_WE_SHIFT:u32=16;
pub const CH_CTL_H_ARLEN_EN:u32=1<<6; pub const CH_CTL_H_ARLEN_POS:u32=7; pub const CH_CTL_H_AWLEN_EN:u32=1<<15; pub const CH_CTL_H_AWLEN_POS:u32=16;

pub const DWAXIDMAC_ARWLEN_1:u32=0; pub const DWAXIDMAC_ARWLEN_2:u32=1; pub const DWAXIDMAC_ARWLEN_4:u32=3; pub const DWAXIDMAC_ARWLEN_8:u32=7; pub const DWAXIDMAC_ARWLEN_16:u32=15; pub const DWAXIDMAC_ARWLEN_32:u32=31; pub const DWAXIDMAC_ARWLEN_64:u32=63; pub const DWAXIDMAC_ARWLEN_128:u32=127; pub const DWAXIDMAC_ARWLEN_256:u32=255; pub const DWAXIDMAC_ARWLEN_MIN:u32=DWAXIDMAC_ARWLEN_1; pub const DWAXIDMAC_ARWLEN_MAX:u32=DWAXIDMAC_ARWLEN_256;
pub const CH_CTL_H_LLI_LAST:u32=1<<30; pub const CH_CTL_H_LLI_VALID:u32=1<<31; pub const CH_CTL_L_LAST_WRITE_EN:u32=1<<30; pub const CH_CTL_L_DST_MSIZE_POS:u32=18; pub const CH_CTL_L_SRC_MSIZE_POS:u32=14;

pub const CH_CTL_L_DST_WIDTH_POS:u32=11; pub const CH_CTL_L_SRC_WIDTH_POS:u32=8; pub const CH_CTL_L_DST_INC_POS:u32=6; pub const CH_CTL_L_SRC_INC_POS:u32=4;
pub const DWAXIDMAC_CH_CTL_L_INC:u32=0; pub const DWAXIDMAC_CH_CTL_L_NOINC:u32=1; pub const CH_CTL_L_DST_MAST:u32=1<<2; pub const CH_CTL_L_SRC_MAST:u32=1;
pub const CH_CFG_H_PRIORITY_POS:u32=17; pub const CH_CFG_H_DST_PER_POS:u32=12; pub const CH_CFG_H_SRC_PER_POS:u32=7; pub const CH_CFG_H_HS_SEL_DST_POS:u32=4; pub const CH_CFG_H_HS_SEL_SRC_POS:u32=3; pub const DWAXIDMAC_HS_SEL_HW:u32=0; pub const DWAXIDMAC_HS_SEL_SW:u32=1; pub const CH_CFG_H_TT_FC_POS:u32=0;
pub const CH_CFG_L_DST_MULTBLK_TYPE_POS:u32=2; pub const CH_CFG_L_SRC_MULTBLK_TYPE_POS:u32=0; pub const CH_CFG2_L_SRC_PER_POS:u32=4; pub const CH_CFG2_L_DST_PER_POS:u32=11; pub const CH_CFG2_H_TT_FC_POS:u32=0; pub const CH_CFG2_H_HS_SEL_SRC_POS:u32=3; pub const CH_CFG2_H_HS_SEL_DST_POS:u32=4; pub const CH_CFG2_H_PRIORITY_POS:u32=20;

// Enumerations retain their C discriminants.
pub const DWAXIDMAC_BURST_TRANS_LEN_1:u32=0; pub const DWAXIDMAC_BURST_TRANS_LEN_4:u32=1; pub const DWAXIDMAC_BURST_TRANS_LEN_8:u32=2; pub const DWAXIDMAC_BURST_TRANS_LEN_16:u32=3; pub const DWAXIDMAC_BURST_TRANS_LEN_32:u32=4; pub const DWAXIDMAC_BURST_TRANS_LEN_64:u32=5; pub const DWAXIDMAC_BURST_TRANS_LEN_128:u32=6; pub const DWAXIDMAC_BURST_TRANS_LEN_256:u32=7; pub const DWAXIDMAC_BURST_TRANS_LEN_512:u32=8; pub const DWAXIDMAC_BURST_TRANS_LEN_1024:u32=9;
pub const DWAXIDMAC_TRANS_WIDTH_8:u32=0; pub const DWAXIDMAC_TRANS_WIDTH_16:u32=1; pub const DWAXIDMAC_TRANS_WIDTH_32:u32=2; pub const DWAXIDMAC_TRANS_WIDTH_64:u32=3; pub const DWAXIDMAC_TRANS_WIDTH_128:u32=4; pub const DWAXIDMAC_TRANS_WIDTH_256:u32=5; pub const DWAXIDMAC_TRANS_WIDTH_512:u32=6; pub const DWAXIDMAC_TRANS_WIDTH_MAX:u32=6;
pub const DWAXIDMAC_TT_FC_MEM_TO_MEM_DMAC:u32=0; pub const DWAXIDMAC_TT_FC_MEM_TO_PER_DMAC:u32=1; pub const DWAXIDMAC_TT_FC_PER_TO_MEM_DMAC:u32=2; pub const DWAXIDMAC_TT_FC_PER_TO_PER_DMAC:u32=3; pub const DWAXIDMAC_TT_FC_PER_TO_MEM_SRC:u32=4; pub const DWAXIDMAC_TT_FC_PER_TO_PER_SRC:u32=5; pub const DWAXIDMAC_TT_FC_MEM_TO_PER_DST:u32=6; pub const DWAXIDMAC_TT_FC_PER_TO_PER_DST:u32=7;
pub const DWAXIDMAC_MBLK_TYPE_CONTIGUOUS:u32=0; pub const DWAXIDMAC_MBLK_TYPE_RELOAD:u32=1; pub const DWAXIDMAC_MBLK_TYPE_SHADOW_REG:u32=2; pub const DWAXIDMAC_MBLK_TYPE_LL:u32=3;

pub const DWAXIDMAC_IRQ_NONE:u32=0;
pub const DWAXIDMAC_IRQ_BLOCK_TRF:u32=1<<0; pub const DWAXIDMAC_IRQ_DMA_TRF:u32=1<<1; pub const DWAXIDMAC_IRQ_SRC_TRAN:u32=1<<3; pub const DWAXIDMAC_IRQ_DST_TRAN:u32=1<<4;
pub const DWAXIDMAC_IRQ_SRC_DEC_ERR:u32=1<<5; pub const DWAXIDMAC_IRQ_DST_DEC_ERR:u32=1<<6; pub const DWAXIDMAC_IRQ_SRC_SLV_ERR:u32=1<<7; pub const DWAXIDMAC_IRQ_DST_SLV_ERR:u32=1<<8;
pub const DWAXIDMAC_IRQ_LLI_RD_DEC_ERR:u32=1<<9; pub const DWAXIDMAC_IRQ_LLI_WR_DEC_ERR:u32=1<<10; pub const DWAXIDMAC_IRQ_LLI_RD_SLV_ERR:u32=1<<11; pub const DWAXIDMAC_IRQ_LLI_WR_SLV_ERR:u32=1<<12;
pub const DWAXIDMAC_IRQ_INVALID_ERR:u32=1<<13; pub const DWAXIDMAC_IRQ_MULTIBLKTYPE_ERR:u32=1<<14; pub const DWAXIDMAC_IRQ_DEC_ERR:u32=1<<16; pub const DWAXIDMAC_IRQ_WR2RO_ERR:u32=1<<17; pub const DWAXIDMAC_IRQ_RD2RWO_ERR:u32=1<<18; pub const DWAXIDMAC_IRQ_WRONCHEN_ERR:u32=1<<19; pub const DWAXIDMAC_IRQ_SHADOWREG_ERR:u32=1<<20; pub const DWAXIDMAC_IRQ_WRONHOLD_ERR:u32=1<<21;
pub const DWAXIDMAC_IRQ_LOCK_CLEARED:u32=1<<27; pub const DWAXIDMAC_IRQ_SRC_SUSPENDED:u32=1<<28; pub const DWAXIDMAC_IRQ_SUSPENDED:u32=1<<29; pub const DWAXIDMAC_IRQ_DISABLED:u32=1<<30; pub const DWAXIDMAC_IRQ_ABORTED:u32=1<<31;
pub const DWAXIDMAC_IRQ_ALL_ERR:u32=0x003f0000 | 0x00007fe0; pub const DWAXIDMAC_IRQ_ALL:u32=0xffff_ffff;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
