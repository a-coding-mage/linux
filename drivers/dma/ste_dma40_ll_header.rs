/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) ST-Ericsson SA 2007-2010
 * Author: Per Friden <per.friden@stericsson.com> for ST-Ericsson SA
 * Author: Jonas Aaberg <jonas.aberg@stericsson.com> for ST-Ericsson SA
 */

/* Translated from ste_dma40_ll.h. */

pub const D40_DREG_PCBASE: u32 = 0x400;
pub const D40_DREG_PCDELTA: u32 = 8 * 4;
pub const D40_LLI_ALIGN: u32 = 16;
pub const D40_LCPA_CHAN_SIZE: u32 = 32;
pub const D40_LCPA_CHAN_DST_DELTA: u32 = 16;
pub const D40_GROUP_SIZE: u32 = 8;

#[inline] pub const fn D40_TYPE_TO_GROUP(type_: u32) -> u32 { type_ / 16 }
#[inline] pub const fn D40_TYPE_TO_EVENT(type_: u32) -> u32 { type_ % 16 }
#[inline] pub const fn D40_PHYS_TO_GROUP(phys: u32) -> u32 { (phys & (D40_GROUP_SIZE - 1)) / 2 }

pub const D40_SREG_CFG_MST_POS:u32=15; pub const D40_SREG_CFG_TIM_POS:u32=14;
pub const D40_SREG_CFG_EIM_POS:u32=13; pub const D40_SREG_CFG_LOG_INCR_POS:u32=12;
pub const D40_SREG_CFG_PHY_PEN_POS:u32=12; pub const D40_SREG_CFG_PSIZE_POS:u32=10;
pub const D40_SREG_CFG_ESIZE_POS:u32=8; pub const D40_SREG_CFG_PRI_POS:u32=7;
pub const D40_SREG_CFG_LBE_POS:u32=6; pub const D40_SREG_CFG_LOG_GIM_POS:u32=5;
pub const D40_SREG_CFG_LOG_MFU_POS:u32=4; pub const D40_SREG_CFG_PHY_TM_POS:u32=4;
pub const D40_SREG_CFG_PHY_EVTL_POS:u32=0;
pub const D40_SREG_ELEM_PHY_ECNT_POS:u32=16; pub const D40_SREG_ELEM_PHY_EIDX_POS:u32=0;
pub const D40_SREG_ELEM_PHY_ECNT_MASK:u32=0xFFFF<<D40_SREG_ELEM_PHY_ECNT_POS;
pub const D40_SREG_LNK_PHY_TCP_POS:u32=0; pub const D40_SREG_LNK_PHY_LMP_POS:u32=1; pub const D40_SREG_LNK_PHY_PRE_POS:u32=2;
pub const D40_SREG_LNK_PHYS_LNK_MASK:u32=0xFFFFFFF8;
pub const D40_SREG_ELEM_LOG_ECNT_POS:u32=16; pub const D40_SREG_ELEM_LOG_LIDX_POS:u32=8;
pub const D40_SREG_ELEM_LOG_LOS_POS:u32=1; pub const D40_SREG_ELEM_LOG_TCP_POS:u32=0;
pub const D40_SREG_ELEM_LOG_LIDX_MASK:u32=0xFF<<D40_SREG_ELEM_LOG_LIDX_POS;
#[inline] pub const fn D40_EVENTLINE_POS(i:u32)->u32 { 2*i }
#[inline] pub const fn D40_EVENTLINE_MASK(i:u32)->u32 { 0x3<<D40_EVENTLINE_POS(i) }
pub const D40_MEM_LCSP0_ECNT_POS:u32=16; pub const D40_MEM_LCSP0_SPTR_POS:u32=0;
pub const D40_MEM_LCSP0_ECNT_MASK:u32=0xFFFF<<16; pub const D40_MEM_LCSP0_SPTR_MASK:u32=0xFFFF;
pub const D40_MEM_LCSP1_SPTR_POS:u32=16; pub const D40_MEM_LCSP1_SCFG_MST_POS:u32=15;
pub const D40_MEM_LCSP1_SCFG_TIM_POS:u32=14; pub const D40_MEM_LCSP1_SCFG_EIM_POS:u32=13;
pub const D40_MEM_LCSP1_SCFG_INCR_POS:u32=12; pub const D40_MEM_LCSP1_SCFG_PSIZE_POS:u32=10;
pub const D40_MEM_LCSP1_SCFG_ESIZE_POS:u32=8; pub const D40_MEM_LCSP1_SLOS_POS:u32=1; pub const D40_MEM_LCSP1_STCP_POS:u32=0;
pub const D40_MEM_LCSP1_SPTR_MASK:u32=0xFFFF<<16; pub const D40_MEM_LCSP1_SCFG_TIM_MASK:u32=1<<14;
pub const D40_MEM_LCSP1_SCFG_INCR_MASK:u32=1<<12; pub const D40_MEM_LCSP1_SCFG_PSIZE_MASK:u32=3<<10;
pub const D40_MEM_LCSP1_SLOS_MASK:u32=0x7F<<1; pub const D40_MEM_LCSP1_STCP_MASK:u32=1;
pub const D40_MEM_LCSP2_ECNT_POS:u32=16; pub const D40_MEM_LCSP2_ECNT_MASK:u32=0xFFFF<<16;
pub const D40_MEM_LCSP3_DCFG_MST_POS:u32=15; pub const D40_MEM_LCSP3_DCFG_TIM_POS:u32=14; pub const D40_MEM_LCSP3_DCFG_EIM_POS:u32=13;
pub const D40_MEM_LCSP3_DCFG_INCR_POS:u32=12; pub const D40_MEM_LCSP3_DCFG_PSIZE_POS:u32=10; pub const D40_MEM_LCSP3_DCFG_ESIZE_POS:u32=8;
pub const D40_MEM_LCSP3_DLOS_POS:u32=1; pub const D40_MEM_LCSP3_DTCP_POS:u32=0; pub const D40_MEM_LCSP3_DLOS_MASK:u32=0x7F<<1; pub const D40_MEM_LCSP3_DTCP_MASK:u32=1;

/* Standard channel parameter register offsets and DMA register offsets. */
pub const D40_CHAN_REG_SSCFG:u32=0; pub const D40_CHAN_REG_SSELT:u32=4; pub const D40_CHAN_REG_SSPTR:u32=8; pub const D40_CHAN_REG_SSLNK:u32=0xC;
pub const D40_CHAN_REG_SDCFG:u32=0x10; pub const D40_CHAN_REG_SDELT:u32=0x14; pub const D40_CHAN_REG_SDPTR:u32=0x18; pub const D40_CHAN_REG_SDLNK:u32=0x1C;
pub const D40_DREG_GCC:u32=0; pub const D40_DREG_GCC_ENA:u32=1; pub const D40_DREG_GCC_ENABLE_ALL:u32=0x3ff01; pub const D40_DREG_GCC_EVTGRP_POS:u32=8; pub const D40_DREG_GCC_SRC:u32=0; pub const D40_DREG_GCC_DST:u32=1;
#[inline] pub const fn D40_DREG_GCC_EVTGRP_ENA(x:u32,y:u32)->u32 { 1 << (D40_DREG_GCC_EVTGRP_POS+2*x+y) }

/* DMA register map. */
pub const D40_DREG_PRTYP:u32=4; pub const D40_DREG_PRSME:u32=8; pub const D40_DREG_PRSMO:u32=0xC; pub const D40_DREG_PRMSE:u32=0x10; pub const D40_DREG_PRMSO:u32=0x14; pub const D40_DREG_PRMOE:u32=0x18; pub const D40_DREG_PRMOO:u32=0x1C;
pub const D40_DREG_PRMO_PCHAN_BASIC:u32=1; pub const D40_DREG_PRMO_PCHAN_MODULO:u32=2; pub const D40_DREG_PRMO_PCHAN_DOUBLE_DST:u32=3; pub const D40_DREG_PRMO_LCHAN_SRC_PHY_DST_LOG:u32=1; pub const D40_DREG_PRMO_LCHAN_SRC_LOG_DST_PHY:u32=2; pub const D40_DREG_PRMO_LCHAN_SRC_LOG_DST_LOG:u32=3;
pub const D40_DREG_LCPA:u32=0x20; pub const D40_DREG_LCLA:u32=0x24;

/* The remaining register offsets are kept as a literal register-map table. */
pub const D40_DREG_SSEG1:u32=0x30; pub const D40_DREG_SSEG2:u32=0x34; pub const D40_DREG_SSEG3:u32=0x38; pub const D40_DREG_SSEG4:u32=0x3C;
pub const D40_DREG_SCEG1:u32=0x40; pub const D40_DREG_SCEG2:u32=0x44; pub const D40_DREG_SCEG3:u32=0x48; pub const D40_DREG_SCEG4:u32=0x4C;
pub const D40_DREG_ACTIVE:u32=0x50; pub const D40_DREG_ACTIVO:u32=0x54; pub const D40_DREG_CIDMOD:u32=0x58; pub const D40_DREG_TCIDV:u32=0x5C; pub const D40_DREG_PCMIS:u32=0x60; pub const D40_DREG_PCICR:u32=0x64; pub const D40_DREG_PCTIS:u32=0x68; pub const D40_DREG_PCEIS:u32=0x6C;
pub const D40_DREG_SPCMIS:u32=0x70; pub const D40_DREG_SPCICR:u32=0x74; pub const D40_DREG_SPCTIS:u32=0x78; pub const D40_DREG_SPCEIS:u32=0x7C;
pub const D40_DREG_LCMIS0:u32=0x80; pub const D40_DREG_LCMIS1:u32=0x84; pub const D40_DREG_LCMIS2:u32=0x88; pub const D40_DREG_LCMIS3:u32=0x8C; pub const D40_DREG_LCICR0:u32=0x90; pub const D40_DREG_LCICR1:u32=0x94; pub const D40_DREG_LCICR2:u32=0x98; pub const D40_DREG_LCICR3:u32=0x9C;
pub const D40_DREG_LCTIS0:u32=0xA0; pub const D40_DREG_LCTIS1:u32=0xA4; pub const D40_DREG_LCTIS2:u32=0xA8; pub const D40_DREG_LCTIS3:u32=0xAC; pub const D40_DREG_LCEIS0:u32=0xB0; pub const D40_DREG_LCEIS1:u32=0xB4; pub const D40_DREG_LCEIS2:u32=0xB8; pub const D40_DREG_LCEIS3:u32=0xBC;
pub const D40_DREG_SLCMIS1:u32=0xC0; pub const D40_DREG_SLCMIS2:u32=0xC4; pub const D40_DREG_SLCMIS3:u32=0xC8; pub const D40_DREG_SLCMIS4:u32=0xCC; pub const D40_DREG_SLCICR1:u32=0xD0; pub const D40_DREG_SLCICR2:u32=0xD4; pub const D40_DREG_SLCICR3:u32=0xD8; pub const D40_DREG_SLCICR4:u32=0xDC;
pub const D40_DREG_SLCTIS1:u32=0xE0; pub const D40_DREG_SLCTIS2:u32=0xE4; pub const D40_DREG_SLCTIS3:u32=0xE8; pub const D40_DREG_SLCTIS4:u32=0xEC; pub const D40_DREG_SLCEIS1:u32=0xF0; pub const D40_DREG_SLCEIS2:u32=0xF4; pub const D40_DREG_SLCEIS3:u32=0xF8; pub const D40_DREG_SLCEIS4:u32=0xFC;
pub const D40_DREG_FSESS1:u32=0x100; pub const D40_DREG_FSESS2:u32=0x104; pub const D40_DREG_FSEBS1:u32=0x108; pub const D40_DREG_FSEBS2:u32=0x10C;
pub const D40_DREG_PSEG1:u32=0x110; pub const D40_DREG_PSEG2:u32=0x114; pub const D40_DREG_PSEG3:u32=0x118; pub const D40_DREG_PSEG4:u32=0x11C; pub const D40_DREG_PCEG1:u32=0x120; pub const D40_DREG_PCEG2:u32=0x124; pub const D40_DREG_PCEG3:u32=0x128; pub const D40_DREG_PCEG4:u32=0x12C; pub const D40_DREG_RSEG1:u32=0x130; pub const D40_DREG_RSEG2:u32=0x134; pub const D40_DREG_RSEG3:u32=0x138; pub const D40_DREG_RSEG4:u32=0x13C; pub const D40_DREG_RCEG1:u32=0x140; pub const D40_DREG_RCEG2:u32=0x144; pub const D40_DREG_RCEG3:u32=0x148; pub const D40_DREG_RCEG4:u32=0x14C; pub const D40_DREG_PREFOT:u32=0x15C; pub const D40_DREG_EXTCFG:u32=0x160;
pub const D40_DREG_CPSEG1:u32=0x200; pub const D40_DREG_CPSEG2:u32=0x204; pub const D40_DREG_CPSEG3:u32=0x208; pub const D40_DREG_CPSEG4:u32=0x20C; pub const D40_DREG_CPSEG5:u32=0x210; pub const D40_DREG_CPCEG1:u32=0x220; pub const D40_DREG_CPCEG2:u32=0x224; pub const D40_DREG_CPCEG3:u32=0x228; pub const D40_DREG_CPCEG4:u32=0x22C; pub const D40_DREG_CPCEG5:u32=0x230; pub const D40_DREG_CRSEG1:u32=0x240; pub const D40_DREG_CRSEG2:u32=0x244; pub const D40_DREG_CRSEG3:u32=0x248; pub const D40_DREG_CRSEG4:u32=0x24C; pub const D40_DREG_CRSEG5:u32=0x250; pub const D40_DREG_CRCEG1:u32=0x260; pub const D40_DREG_CRCEG2:u32=0x264; pub const D40_DREG_CRCEG3:u32=0x268; pub const D40_DREG_CRCEG4:u32=0x26C; pub const D40_DREG_CRCEG5:u32=0x270;
pub const D40_DREG_CFSESS1:u32=0x280; pub const D40_DREG_CFSESS2:u32=0x284; pub const D40_DREG_CFSESS3:u32=0x288; pub const D40_DREG_CFSEBS1:u32=0x290; pub const D40_DREG_CFSEBS2:u32=0x294; pub const D40_DREG_CFSEBS3:u32=0x298;
pub const D40_DREG_CLCMIS1:u32=0x300; pub const D40_DREG_CLCMIS2:u32=0x304; pub const D40_DREG_CLCMIS3:u32=0x308; pub const D40_DREG_CLCMIS4:u32=0x30C; pub const D40_DREG_CLCMIS5:u32=0x310; pub const D40_DREG_CLCICR1:u32=0x320; pub const D40_DREG_CLCICR2:u32=0x324; pub const D40_DREG_CLCICR3:u32=0x328; pub const D40_DREG_CLCICR4:u32=0x32C; pub const D40_DREG_CLCICR5:u32=0x330; pub const D40_DREG_CLCTIS1:u32=0x340; pub const D40_DREG_CLCTIS2:u32=0x344; pub const D40_DREG_CLCTIS3:u32=0x348; pub const D40_DREG_CLCTIS4:u32=0x34C; pub const D40_DREG_CLCTIS5:u32=0x350; pub const D40_DREG_CLCEIS1:u32=0x360; pub const D40_DREG_CLCEIS2:u32=0x364; pub const D40_DREG_CLCEIS3:u32=0x368; pub const D40_DREG_CLCEIS4:u32=0x36C; pub const D40_DREG_CLCEIS5:u32=0x370; pub const D40_DREG_CPCMIS:u32=0x380; pub const D40_DREG_CPCICR:u32=0x384; pub const D40_DREG_CPCTIS:u32=0x388; pub const D40_DREG_CPCEIS:u32=0x38C;
pub const D40_DREG_SCCIDA1:u32=0xE80; pub const D40_DREG_SCCIDA2:u32=0xE90; pub const D40_DREG_SCCIDA3:u32=0xEA0; pub const D40_DREG_SCCIDA4:u32=0xEB0; pub const D40_DREG_SCCIDA5:u32=0xEC0; pub const D40_DREG_SCCIDB1:u32=0xE84; pub const D40_DREG_SCCIDB2:u32=0xE94; pub const D40_DREG_SCCIDB3:u32=0xEA4; pub const D40_DREG_SCCIDB4:u32=0xEB4; pub const D40_DREG_SCCIDB5:u32=0xEC4; pub const D40_DREG_PRSCCIDA:u32=0xF80; pub const D40_DREG_PRSCCIDB:u32=0xF84; pub const D40_DREG_STFU:u32=0xFC8; pub const D40_DREG_ICFG:u32=0xFCC; pub const D40_DREG_PERIPHID0:u32=0xFE0; pub const D40_DREG_PERIPHID1:u32=0xFE4; pub const D40_DREG_PERIPHID2:u32=0xFE8; pub const D40_DREG_PERIPHID3:u32=0xFEC; pub const D40_DREG_CELLID0:u32=0xFF0; pub const D40_DREG_CELLID1:u32=0xFF4; pub const D40_DREG_CELLID2:u32=0xFF8; pub const D40_DREG_CELLID3:u32=0xFFC;

#[repr(C, align(16))]
pub struct d40_phy_lli { pub reg_cfg:u32, pub reg_elt:u32, pub reg_ptr:u32, pub reg_lnk:u32 }
#[repr(C)] pub struct d40_phy_lli_bidir { pub src:*mut d40_phy_lli, pub dst:*mut d40_phy_lli }
#[repr(C, align(8))] pub struct d40_log_lli { pub lcsp02:u32, pub lcsp13:u32 }
#[repr(C)] pub struct d40_log_lli_bidir { pub src:*mut d40_log_lli, pub dst:*mut d40_log_lli }
#[repr(C)] pub struct d40_log_lli_full { pub lcsp0:u32, pub lcsp1:u32, pub lcsp2:u32, pub lcsp3:u32 }
#[repr(C)] pub struct d40_def_lcsp { pub lcsp3:u32, pub lcsp1:u32 }

#[repr(u32)] pub enum d40_lli_flags { LLI_ADDR_INC=1<<0, LLI_TERM_INT=1<<1, LLI_CYCLIC=1<<2, LLI_LAST_LINK=1<<3 }

extern "C" {
    pub fn d40_phy_cfg(cfg:*mut stedma40_chan_cfg, src_cfg:*mut u32, dst_cfg:*mut u32);
    pub fn d40_log_cfg(cfg:*mut stedma40_chan_cfg, lcsp1:*mut u32, lcsp2:*mut u32);
    pub fn d40_phy_sg_to_lli(sg:*mut scatterlist, sg_len:i32, target:dma_addr_t, lli:*mut d40_phy_lli, lli_phys:dma_addr_t, reg_cfg:u32, info:*mut stedma40_half_channel_info, otherinfo:*mut stedma40_half_channel_info, flags:usize)->i32;
    pub fn d40_log_sg_to_lli(sg:*mut scatterlist, sg_len:i32, dev_addr:dma_addr_t, lli_sg:*mut d40_log_lli, lcsp13:u32, data_width1:u32, data_width2:u32)->i32;
    pub fn d40_log_lli_lcpa_write(lcpa:*mut d40_log_lli_full, lli_dst:*mut d40_log_lli, lli_src:*mut d40_log_lli, next:i32, flags:u32);
    pub fn d40_log_lli_lcla_write(lcla:*mut d40_log_lli, lli_dst:*mut d40_log_lli, lli_src:*mut d40_log_lli, next:i32, flags:u32);
}

/* dma_addr_t, stedma40_chan_cfg, scatterlist, and stedma40_half_channel_info
 * are supplied by the surrounding translation unit. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
