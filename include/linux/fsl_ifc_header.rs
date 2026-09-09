/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Freescale Integrated Flash Controller
 *
 * Copyright 2011 Freescale Semiconductor, Inc
 * Author: Dipen Dudhat <dipen.dudhat@freescale.com>
 */

// Linux dependencies supplied by the surrounding translation unit.
use core::ffi::c_void;

pub const FSL_IFC_BANK_COUNT: usize = 8;
pub const FSL_IFC_VERSION_MASK: u32 = 0x0F0F0000;
pub const FSL_IFC_VERSION_1_0_0: u32 = 0x01000000;
pub const FSL_IFC_VERSION_1_1_0: u32 = 0x01010000;
pub const FSL_IFC_VERSION_2_0_0: u32 = 0x02000000;
pub const PGOFFSET_64K: usize = 64 * 1024;
pub const PGOFFSET_4K: usize = 4 * 1024;

macro_rules! c { ($($n:ident = $v:expr),* $(,)?) => { $(pub const $n: u32 = $v;)* }; }
c! {
 CSPR_BA=0xFFFF0000, CSPR_BA_SHIFT=16, CSPR_PORT_SIZE=0x00000180, CSPR_PORT_SIZE_SHIFT=7,
 CSPR_PORT_SIZE_8=0x80, CSPR_PORT_SIZE_16=0x100, CSPR_PORT_SIZE_32=0x180, CSPR_WP=0x40,
 CSPR_WP_SHIFT=6, CSPR_MSEL=6, CSPR_MSEL_SHIFT=1, CSPR_MSEL_NOR=0, CSPR_MSEL_NAND=2,
 CSPR_MSEL_GPCM=4, CSPR_V=1, CSPR_V_SHIFT=0, IFC_AMASK_MASK=0xFFFF0000, IFC_AMASK_SHIFT=16,
 CSOR_NAND_ECC_ENC_EN=0x80000000, CSOR_NAND_ECC_MODE_MASK=0x30000000, CSOR_NAND_ECC_MODE_4=0,
 CSOR_NAND_ECC_MODE_8=0x10000000, CSOR_NAND_ECC_DEC_EN=0x04000000, CSOR_NAND_RAL_MASK=0x01800000,
 CSOR_NAND_RAL_SHIFT=20, CSOR_NAND_RAL_1=0, CSOR_NAND_RAL_2=0x00800000, CSOR_NAND_RAL_3=0x01000000,
 CSOR_NAND_RAL_4=0x01800000, CSOR_NAND_PGS_MASK=0x00180000, CSOR_NAND_PGS_SHIFT=16,
 CSOR_NAND_PGS_512=0, CSOR_NAND_PGS_2K=0x80000, CSOR_NAND_PGS_4K=0x100000, CSOR_NAND_PGS_8K=0x180000,
 CSOR_NAND_SPRZ_MASK=0xE000, CSOR_NAND_SPRZ_SHIFT=13, CSOR_NAND_SPRZ_16=0, CSOR_NAND_SPRZ_64=0x2000,
 CSOR_NAND_SPRZ_128=0x4000, CSOR_NAND_SPRZ_210=0x6000, CSOR_NAND_SPRZ_218=0x8000, CSOR_NAND_SPRZ_224=0xA000,
 CSOR_NAND_SPRZ_CSOR_EXT=0xC000, CSOR_NAND_PB_MASK=0x700, CSOR_NAND_PB_SHIFT=8,
 CSOR_NAND_TRHZ_MASK=0x1C, CSOR_NAND_TRHZ_SHIFT=2, CSOR_NAND_TRHZ_20=0, CSOR_NAND_TRHZ_40=4,
 CSOR_NAND_TRHZ_60=8, CSOR_NAND_TRHZ_80=0xC, CSOR_NAND_TRHZ_100=0x10, CSOR_NAND_BCTLD=1,
 CSOR_NOR_ADM_SHFT_MODE_EN=0x80000000, CSOR_NOR_PGRD_EN=0x10000000, CSOR_NOR_AVD_TGL_PGM_EN=0x01000000,
 CSOR_NOR_ADM_MASK=0x3E000, CSOR_NOR_ADM_SHIFT_SHIFT=13, CSOR_NOR_NOR_MODE_AYSNC_NOR=0,
 CSOR_NOR_NOR_MODE_AVD_NOR=0x20, CSOR_NOR_TRHZ_MASK=0x1C, CSOR_NOR_TRHZ_SHIFT=2, CSOR_NOR_TRHZ_20=0,
 CSOR_NOR_TRHZ_40=4, CSOR_NOR_TRHZ_60=8, CSOR_NOR_TRHZ_80=0xC, CSOR_NOR_TRHZ_100=0x10, CSOR_NOR_BCTLD=1,
 CSOR_GPCM_GPMODE_NORMAL=0, CSOR_GPCM_GPMODE_ASIC=0x80000000, CSOR_GPCM_PARITY_EVEN=0x40000000,
 CSOR_GPCM_PAR_EN=0x20000000, CSOR_GPCM_GPTO_MASK=0x0F000000, CSOR_GPCM_GPTO_SHIFT=24,
 CSOR_GPCM_RGETA_EXT=0x80000, CSOR_GPCM_WGETA_EXT=0x40000, CSOR_GPCM_ADM_MASK=0x3E000,
 CSOR_GPCM_ADM_SHIFT_SHIFT=13, CSOR_GPCM_GAPERRD_MASK=0x180, CSOR_GPCM_GAPERRD_SHIFT=7,
 CSOR_GPCM_TRHZ_MASK=0x1C, CSOR_GPCM_TRHZ_20=0, CSOR_GPCM_TRHZ_40=4, CSOR_GPCM_TRHZ_60=8,
 CSOR_GPCM_TRHZ_80=0xC, CSOR_GPCM_TRHZ_100=0x10, CSOR_GPCM_BCTLD=1,
 IFC_RB_STAT_READY_CS0=0x80000000, IFC_RB_STAT_READY_CS1=0x40000000, IFC_RB_STAT_READY_CS2=0x20000000,
 IFC_RB_STAT_READY_CS3=0x10000000, IFC_GCR_MASK=0x8000F800, IFC_GCR_SOFT_RST_ALL=0x80000000,
 IFC_GCR_TBCTL_TRN_TIME=0xF800, IFC_GCR_TBCTL_TRN_TIME_SHIFT=11, IFC_CM_EVTER_STAT_CSER=0x80000000,
 IFC_CM_EVTER_EN_CSEREN=0x80000000, IFC_CM_EVTER_INTR_EN_CSERIREN=0x80000000,
 IFC_CM_ERATTR0_ERTYP_READ=0x80000000, IFC_CM_ERATTR0_ERAID=0x0FF00000, IFC_CM_ERATTR0_ERAID_SHIFT=20,
 IFC_CM_ERATTR0_ESRCID=0xFF00, IFC_CM_ERATTR0_ESRCID_SHIFT=8, IFC_CCR_MASK=0x0F0F8800,
 IFC_CCR_CLK_DIV_MASK=0x0F000000, IFC_CCR_CLK_DIV_SHIFT=24, IFC_CCR_CLK_DLY_MASK=0xF0000,
 IFC_CCR_CLK_DLY_SHIFT=16, IFC_CCR_INV_CLK_EN=0x8000, IFC_CCR_FB_IFC_CLK_SEL=0x800,
 IFC_CSR_CLK_STAT_STABLE=0x80000000, IFC_NAND_NCFGR_BOOT=0x80000000, IFC_NAND_NCFGR_SRAM_INIT_EN=0x20000000,
 IFC_NAND_NCFGR_ADDR_MODE_RC0=0, IFC_NAND_NCFGR_ADDR_MODE_RC1=0x400000, IFC_NAND_NCFGR_NUM_LOOP_MASK=0xF000,
 IFC_NAND_NCFGR_NUM_LOOP_SHIFT=12, IFC_NAND_NCFGR_NUM_WAIT_MASK=0xFF, IFC_NAND_NCFGR_NUM_WAIT_SHIFT=0,
 IFC_NAND_COL_MS=0x80000000, IFC_NAND_COL_CA_MASK=0xFFF, IFC_NAND_BC=0x1FF,
 IFC_NAND_CSEL=0x0C000000, IFC_NAND_CSEL_SHIFT=26, IFC_NAND_CSEL_CS0=0, IFC_NAND_CSEL_CS1=0x04000000,
 IFC_NAND_CSEL_CS2=0x08000000, IFC_NAND_CSEL_CS3=0x0C000000, IFC_NAND_SEQ_STRT_FIR_STRT=0x80000000,
 IFC_NAND_SEQ_STRT_AUTO_ERS=0x00800000, IFC_NAND_SEQ_STRT_AUTO_PGM=0x00100000, IFC_NAND_SEQ_STRT_AUTO_CPB=0x20000,
 IFC_NAND_SEQ_STRT_AUTO_RD=0x4000, IFC_NAND_SEQ_STRT_AUTO_STAT_RD=0x800, IFC_NAND_EVTER_STAT_OPC=0x80000000,
 IFC_NAND_EVTER_STAT_FTOER=0x08000000, IFC_NAND_EVTER_STAT_WPER=0x04000000, IFC_NAND_EVTER_STAT_ECCER=0x02000000,
 IFC_NAND_EVTER_STAT_RCW_DN=0x8000, IFC_NAND_EVTER_STAT_BOOT_DN=0x4000, IFC_NAND_EVTER_STAT_BBI_SRCH_SE=0x800,
 PGRDCMPL_EVT_STAT_MASK=0xFFFF0000, IFC_NAND_EVTER_EN_OPC_EN=0x80000000, IFC_NAND_EVTER_EN_PGRDCMPL_EN=0x20000000,
 IFC_NAND_EVTER_EN_FTOER_EN=0x08000000, IFC_NAND_EVTER_EN_WPER_EN=0x04000000, IFC_NAND_EVTER_EN_ECCER_EN=0x02000000,
 IFC_NAND_ERATTR0_MASK=0x0C080000, IFC_NAND_ERATTR0_ERCS_CS0=0, IFC_NAND_ERATTR0_ERCS_CS1=0x04000000,
 IFC_NAND_ERATTR0_ERCS_CS2=0x08000000, IFC_NAND_ERATTR0_ERCS_CS3=0x0C000000, IFC_NAND_ERATTR0_ERTTYPE_READ=0x80000,
 IFC_NAND_NFSR_RS0=0xFF000000, IFC_NAND_NFSR_RS1=0x00FF0000, IFC_NAND_AUTOBOOT_TRGR_RCW_LD=0x80000000,
 IFC_NAND_AUTOBOOT_TRGR_BOOT_LD=0x20000000, IFC_NAND_MDR_RDATA0=0xFF000000, IFC_NAND_MDR_RDATA1=0x00FF0000,
 IFC_NOR_EVTER_STAT_OPC_NOR=0x80000000, IFC_NOR_EVTER_STAT_WPER=0x04000000, IFC_NOR_EVTER_STAT_STOER=0x01000000,
 IFC_NOR_EVTER_EN_OPCEN_NOR=0x80000000, IFC_NOR_EVTER_EN_WPEREN=0x04000000, IFC_NOR_EVTER_EN_STOEREN=0x01000000,
 IFC_NOR_ERATTR0_ERSRCID=0xFF000000, IFC_NOR_ERATTR0_ERAID=0x000FF000, IFC_NOR_ERATTR0_ERCS_CS0=0,
 IFC_NOR_ERATTR0_ERCS_CS1=0x10, IFC_NOR_ERATTR0_ERCS_CS2=0x20, IFC_NOR_ERATTR0_ERCS_CS3=0x30, IFC_NOR_ERATTR0_ERTYPE_READ=1,
 IFC_NOR_ERATTR2_ER_NUM_PHASE_EXP=0xF0000, IFC_NOR_ERATTR2_ER_NUM_PHASE_PER=0xF00, IFC_NORCR_MASK=0x0F0F0000,
 IFC_NORCR_NUM_PHASE_MASK=0x0F000000, IFC_NORCR_NUM_PHASE_SHIFT=24, IFC_NORCR_STOCNT_MASK=0x000F0000,
 IFC_NORCR_STOCNT_SHIFT=16, IFC_GPCM_EVTER_STAT_TOER=0x04000000, IFC_GPCM_EVTER_STAT_PER=0x01000000,
 IFC_GPCM_EVTER_EN_TOER_EN=0x04000000, IFC_GPCM_EVTER_EN_PER_EN=0x01000000, IFC_GPCM_ERATTR0_ERSRCID=0xFF000000,
 IFC_GPCM_ERATTR0_ERAID=0x000FF000, IFC_GPCM_ERATTR0_ERCS_CS0=0, IFC_GPCM_ERATTR0_ERCS_CS1=0x40,
 IFC_GPCM_ERATTR0_ERCS_CS2=0x80, IFC_GPCM_ERATTR0_ERCS_CS3=0xC0, IFC_GPCM_ERATTR0_ERTYPE_READ=1,
 IFC_GPCM_ERATTR2_PERR_BEAT=0xC00, IFC_GPCM_ERATTR2_PERR_BYTE=0xF0, IFC_GPCM_ERATTR2_PERR_DATA_PHASE=1,
 IFC_GPCM_STAT_BSY=0x80000000
}

pub const fn ifc_amask(n: u32) -> u32 { IFC_AMASK_MASK << (ilog2(n) - IFC_AMASK_SHIFT) }
pub const fn csor_nand_pb(n: u32) -> u32 { (ilog2(n) - 5) << 8 }
pub const fn csor_gpcm_gpto(n: u32) -> u32 { (ilog2(n) - 8) << 24 }
pub const fn ifc_ccr_clk_div(n: u32) -> u32 { (n - 1) << 24 }
pub const fn ifc_norcr_num_phase(n: u32) -> u32 { (n - 1) << 24 }
pub const fn ifc_norcr_stocnt(n: u32) -> u32 { (ilog2(n) - 8) << 16 }
pub const fn ifc_nand_ncr_ftocnt(n: u32) -> u32 { (ilog2(n) - 8) << 25 }
pub const fn pgrdcmpl_evt_stat_section_sp(n: u32) -> u32 { 1 << (31 - n) }
pub const fn pgrdcmpl_evt_stat_lp_2k(n: u32) -> u32 { 0xF << (28 - n * 4) }
pub const fn pgrdcmpl_evt_stat_lp_4k(n: u32) -> u32 { 0xFF << (24 - n * 8) }
pub const fn ilog2(_n: u32) -> u32 { /* supplied by the Linux dependency */ 0 }

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum IfcNandFirOpcodes { Nop, Ca0, Ca1, Ca2, Ca3, Ra0, Ra1, Ra2, Ra3, Cmd0, Cmd1, Cmd2, Cmd3, Cmd4, Cmd5, Cmd6, Cmd7, Cw0, Cw1, Cw2, Cw3, Cw4, Cw5, Cw6, Cw7, Wbcd, Rbcd, Btrd, Rdstat, Nwait, Wfr, Sbrd, Ua, Rb }

type Be32 = u32; type U32 = u32;
#[repr(C)] pub struct FslIfcNand { pub ncfgr: Be32, pub res1:[U32;4], pub nand_fcr0:Be32, pub nand_fcr1:Be32, pub res2:[U32;8], pub row0:Be32, pub res3:U32, pub col0:Be32, pub res4:U32, pub row1:Be32, pub res5:U32, pub col1:Be32, pub res6:U32, pub row2:Be32, pub res7:U32, pub col2:Be32, pub res8:U32, pub row3:Be32, pub res9:U32, pub col3:Be32, pub res10:[U32;0x24], pub nand_fbcr:Be32, pub res11:U32, pub nand_fir0:Be32, pub nand_fir1:Be32, pub nand_fir2:Be32, pub res12:[U32;0x10], pub nand_csel:Be32, pub res13:U32, pub nandseq_strt:Be32, pub res14:U32, pub nand_evter_stat:Be32, pub res15:U32, pub pgrdcmpl_evt_stat:Be32, pub res16:[U32;2], pub nand_evter_en:Be32, pub res17:[U32;2], pub nand_evter_intr_en:Be32, pub nand_vol_addr_stat:Be32, pub res18:U32, pub nand_erattr0:Be32, pub nand_erattr1:Be32, pub res19:[U32;0x10], pub nand_fsr:Be32, pub res20:U32, pub nand_eccstat:[Be32;8], pub res21:[U32;0x1c], pub nanndcr:Be32, pub res22:[U32;2], pub nand_autoboot_trgr:Be32, pub res23:U32, pub nand_mdr:Be32, pub res24:[U32;0x1C], pub nand_dll_lowcfg0:Be32, pub nand_dll_lowcfg1:Be32, pub res25:U32, pub nand_dll_lowstat:Be32, pub res26:[U32;0x3c] }
#[repr(C)] pub struct FslIfcNor { pub nor_evter_stat:Be32, pub res1:[U32;2], pub nor_evter_en:Be32, pub res2:[U32;2], pub nor_evter_intr_en:Be32, pub res3:[U32;2], pub nor_erattr0:Be32, pub nor_erattr1:Be32, pub nor_erattr2:Be32, pub res4:[U32;4], pub norcr:Be32, pub res5:[U32;0xEF] }
#[repr(C)] pub struct FslIfcGpcm { pub gpcm_evter_stat:Be32, pub res1:[U32;2], pub gpcm_evter_en:Be32, pub res2:[U32;2], pub gpcm_evter_intr_en:Be32, pub res3:[U32;2], pub gpcm_erattr0:Be32, pub gpcm_erattr1:Be32, pub gpcm_erattr2:Be32, pub gpcm_stat:Be32 }
#[repr(C)] pub struct FslIfcRuntime { pub ifc_nand:FslIfcNand, pub ifc_nor:FslIfcNor, pub ifc_gpcm:FslIfcGpcm }

extern "C" { pub fn convert_ifc_address(addr_base: usize) -> u32; pub fn fsl_ifc_find(addr_base: usize) -> i32; }
// The controller, device, I/O, locking, and wait-queue types are supplied by Linux dependencies.
#[repr(C)] pub struct FslIfcGlobal {
 pub ifc_rev:Be32, pub res1:[U32;2],
 pub cspr_cs:[FslIfcCspr;FSL_IFC_BANK_COUNT], pub res3:[U32;0xd],
 pub amask_cs:[FslIfcAmask;FSL_IFC_BANK_COUNT], pub res5:[U32;0xc],
 pub csor_cs:[FslIfcCsor;FSL_IFC_BANK_COUNT], pub res7:[U32;0xc],
 pub ftim_cs:[FslIfcFtim;FSL_IFC_BANK_COUNT], pub res9:[U32;0x30],
 pub rb_stat:Be32, pub rb_map:Be32, pub wb_map:Be32, pub ifc_gcr:Be32, pub res10:[U32;2],
 pub cm_evter_stat:Be32, pub res11:[U32;2], pub cm_evter_en:Be32, pub res12:[U32;2],
 pub cm_evter_intr_en:Be32, pub res13:[U32;2], pub cm_erattr0:Be32, pub cm_erattr1:Be32,
 pub res14:[U32;2], pub ifc_ccr:Be32, pub ifc_csr:Be32, pub ddr_ccr_low:Be32
}
#[repr(C)] pub struct FslIfcCspr { pub cspr_ext:Be32, pub cspr:Be32, pub res2:U32 }
#[repr(C)] pub struct FslIfcAmask { pub amask:Be32, pub res4:[U32;2] }
#[repr(C)] pub struct FslIfcCsor { pub csor:Be32, pub csor_ext:Be32, pub res6:U32 }
#[repr(C)] pub struct FslIfcFtim { pub ftim:[Be32;4], pub res8:[U32;8] }
#[repr(C)] pub struct FslIfcCtrl { pub dev:*mut c_void, pub gregs:*mut FslIfcGlobal, pub rregs:*mut FslIfcRuntime, pub irq:i32, pub nand_irq:i32, pub lock:c_void, pub nand:*mut c_void, pub version:i32, pub banks:i32, pub nand_stat:U32, pub nand_wait:c_void, pub little_endian:bool }
extern "C" { pub static mut fsl_ifc_ctrl_dev:*mut FslIfcCtrl; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
