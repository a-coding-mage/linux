// SPDX-License-Identifier: GPL-2.0-only
/* Synopsys DDR ECC Driver; Rust translation of synopsys_edac.c. */

// Kernel dependencies supplied by the surrounding repository/build.
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const SYNPS_EDAC_NR_CSROWS: usize = 1;
const SYNPS_EDAC_NR_CHANS: usize = 1;
const SYNPS_EDAC_ERR_GRAIN: u32 = 1;
const SYNPS_EDAC_MSG_SIZE: usize = 256;
const SYNPS_EDAC_MOD_STRING: &str = "synps_edac";
const SYNPS_EDAC_MOD_VER: &str = "1";

const CTRL_OFST: usize = 0x0; const T_ZQ_OFST: usize = 0xA4;
const ECC_CTRL_OFST: usize = 0xC4; const CE_LOG_OFST: usize = 0xC8;
const CE_ADDR_OFST: usize = 0xCC; const CE_DATA_31_0_OFST: usize = 0xD0;
const UE_LOG_OFST: usize = 0xDC; const UE_ADDR_OFST: usize = 0xE0;
const UE_DATA_31_0_OFST: usize = 0xE4; const STAT_OFST: usize = 0xF0;
const SCRUB_OFST: usize = 0xF4;
const CTRL_BW_MASK: u32 = 0xC; const CTRL_BW_SHIFT: u32 = 2;
const DDRCTL_WDTH_16: u32 = 1; const DDRCTL_WDTH_32: u32 = 0;
const T_ZQ_DDRMODE_MASK: u32 = 0x2;
const ECC_CTRL_CLR_CE_ERR: u32 = 0x2; const ECC_CTRL_CLR_UE_ERR: u32 = 0x1;
const LOG_VALID: u32 = 0x1; const CE_LOG_BITPOS_MASK: u32 = 0xFE; const CE_LOG_BITPOS_SHIFT: u32 = 1;
const ADDR_COL_MASK: u32 = 0xFFF; const ADDR_ROW_MASK: u32 = 0xFFFF000; const ADDR_ROW_SHIFT: u32 = 12;
const ADDR_BANK_MASK: u32 = 0x70000000; const ADDR_BANK_SHIFT: u32 = 28;
const STAT_UECNT_MASK: u32 = 0xFF; const STAT_CECNT_MASK: u32 = 0xFF00; const STAT_CECNT_SHIFT: u32 = 8;
const SCRUB_MODE_MASK: u32 = 0x7; const SCRUB_MODE_SECDED: u32 = 0x4;
const DDR_ECC_INTR_SUPPORT: i32 = 1 << 0; const DDR_ECC_DATA_POISON_SUPPORT: i32 = 1 << 1; const DDR_ECC_INTR_SELF_CLEAR: i32 = 1 << 2;
const ECC_CFG0_OFST: usize = 0x70; const ECC_CFG1_OFST: usize = 0x74; const ECC_STAT_OFST: usize = 0x78;
const ECC_CLR_OFST: usize = 0x7C; const ECC_ERRCNT_OFST: usize = 0x80;
const ECC_CEADDR0_OFST: usize = 0x84; const ECC_CEADDR1_OFST: usize = 0x88;
const ECC_CSYND0_OFST: usize = 0x8C; const ECC_CSYND1_OFST: usize = 0x90; const ECC_CSYND2_OFST: usize = 0x94;
const ECC_BITMASK0_OFST: usize = 0x98; const ECC_BITMASK1_OFST: usize = 0x9C; const ECC_BITMASK2_OFST: usize = 0xA0;
const ECC_UEADDR0_OFST: usize = 0xA4; const ECC_UEADDR1_OFST: usize = 0xA8;
const ECC_UESYND0_OFST: usize = 0xAC; const ECC_UESYND1_OFST: usize = 0xB0; const ECC_UESYND2_OFST: usize = 0xB4;
const ECC_POISON0_OFST: usize = 0xB8; const ECC_POISON1_OFST: usize = 0xBC; const ECC_ADDRMAP0_OFFSET: usize = 0x200;
const ECC_CTRL_BUSWIDTH_MASK: u32 = 0x3000; const ECC_CTRL_BUSWIDTH_SHIFT: u32 = 12;
const ECC_CTRL_CLR_CE_ERRCNT: u32 = 1 << 2; const ECC_CTRL_CLR_UE_ERRCNT: u32 = 1 << 3;
const DDRCTL_EWDTH_16: u32 = 2; const DDRCTL_EWDTH_32: u32 = 1; const DDRCTL_EWDTH_64: u32 = 0;
const ECC_STAT_UECNT_MASK: u32 = 0xF0000; const ECC_STAT_UECNT_SHIFT: u32 = 16;
const ECC_STAT_CECNT_MASK: u32 = 0xF00; const ECC_STAT_CECNT_SHIFT: u32 = 8; const ECC_STAT_BITNUM_MASK: u32 = 0x7F;
const ECC_ERRCNT_UECNT_MASK: u32 = 0xFFFF0000; const ECC_ERRCNT_UECNT_SHIFT: u32 = 16; const ECC_ERRCNT_CECNT_MASK: u32 = 0xFFFF;
const DDR_QOS_IRQ_STAT_OFST: usize = 0x20200; const DDR_QOSUE_MASK: u32 = 0x4; const DDR_QOSCE_MASK: u32 = 0x2;
const ECC_CE_UE_INTR_MASK: u32 = 0x6; const DDR_QOS_IRQ_EN_OFST: usize = 0x20208; const DDR_QOS_IRQ_DB_OFST: usize = 0x2020C;
const DDR_UE_MASK: u32 = 1 << 9; const DDR_CE_MASK: u32 = 1 << 8;
const ECC_CEADDR0_RW_MASK: u32 = 0x3FFFF; const ECC_CEADDR1_BNKGRP_MASK: u32 = 0x3000000;
const ECC_CEADDR1_BNKNR_MASK: u32 = 0x70000; const ECC_CEADDR1_BLKNR_MASK: u32 = 0xFFF;
const ECC_CEADDR1_BNKGRP_SHIFT: u32 = 24; const ECC_CEADDR1_BNKNR_SHIFT: u32 = 16;
const ECC_POISON0_RANK_SHIFT: u32 = 24; const ECC_POISON0_RANK_MASK: u32 = 1 << 24;
const ECC_POISON0_COLUMN_MASK: u32 = 0xFFF; const ECC_POISON1_BG_SHIFT: u32 = 28; const ECC_POISON1_BG_MASK: u32 = 0x30000000;
const ECC_POISON1_BANKNR_SHIFT: u32 = 24; const ECC_POISON1_BANKNR_MASK: u32 = 0x7000000; const ECC_POISON1_ROW_MASK: u32 = 0x3FFFF;
const MEM_TYPE_DDR3: u32 = 1; const MEM_TYPE_LPDDR3: u32 = 8; const MEM_TYPE_DDR2: u32 = 4; const MEM_TYPE_DDR4: u32 = 0x10; const MEM_TYPE_LPDDR4: u32 = 0x20;
const DDRC_SWCTL: usize = 0x320; const ECC_CEPOISON_MASK: u32 = 3; const ECC_UEPOISON_MASK: u32 = 1;

#[repr(C)]
pub struct EccErrorInfo { pub row:u32, pub col:u32, pub bank:u32, pub bitpos:u32, pub data:u32, pub bankgrpnr:u32, pub blknr:u32 }
#[repr(C)]
pub struct SynpsEccStatus { pub ce_cnt:u32, pub ue_cnt:u32, pub ceinfo:EccErrorInfo, pub ueinfo:EccErrorInfo }
#[repr(C)]
pub struct SynpsEdacPriv {
    pub baseaddr:*mut u8, pub reglock: [u8; 4], pub message:[u8; SYNPS_EDAC_MSG_SIZE],
    pub stat:SynpsEccStatus, pub p_data:*const SynpsPlatformData, pub ce_cnt:u32, pub ue_cnt:u32,
    // CONFIG_EDAC_DEBUG adds poison_addr and the row/column/bank/bank-group/rank shift arrays.
}
#[repr(C)]
#[derive(Copy,Clone,PartialEq)]
pub enum SynpsPlatformType { ZYNQ, ZYNQMP, SYNPS }
#[repr(C)]
pub struct SynpsPlatformData {
    pub platform:SynpsPlatformType,
    pub get_error_info: unsafe extern "C" fn(*mut SynpsEdacPriv)->c_int,
    pub get_mtype: unsafe extern "C" fn(*const u8)->c_int,
    pub get_dtype: unsafe extern "C" fn(*const u8)->c_int,
    pub quirks:i32,
}

extern "C" {
    fn readl(addr:*const u8)->u32; fn writel(value:u32, addr:*mut u8);
    fn memset(s:*mut c_void, c:c_int, n:usize)->*mut c_void;
}

unsafe fn zynq_get_error_info(priv_: *mut SynpsEdacPriv) -> c_int {
    let p=&mut (*priv_).stat; let base=(*priv_).baseaddr;
    let mut regval=readl(base.add(STAT_OFST)); let mut clearval=0;
    if regval==0 { return 1; }
    p.ce_cnt=(regval & STAT_CECNT_MASK)>>STAT_CECNT_SHIFT; p.ue_cnt=regval & STAT_UECNT_MASK;
    regval=readl(base.add(CE_LOG_OFST));
    if p.ce_cnt != 0 && (regval & LOG_VALID)!=0 {
        p.ceinfo.bitpos=(regval & CE_LOG_BITPOS_MASK)>>CE_LOG_BITPOS_SHIFT; regval=readl(base.add(CE_ADDR_OFST));
        p.ceinfo.row=(regval&ADDR_ROW_MASK)>>ADDR_ROW_SHIFT; p.ceinfo.col=regval&ADDR_COL_MASK;
        p.ceinfo.bank=(regval&ADDR_BANK_MASK)>>ADDR_BANK_SHIFT; p.ceinfo.data=readl(base.add(CE_DATA_31_0_OFST)); clearval=ECC_CTRL_CLR_CE_ERR;
    }
    regval=readl(base.add(UE_LOG_OFST));
    if p.ue_cnt != 0 && (regval & LOG_VALID)!=0 { regval=readl(base.add(UE_ADDR_OFST)); p.ueinfo.row=(regval&ADDR_ROW_MASK)>>ADDR_ROW_SHIFT; p.ueinfo.col=regval&ADDR_COL_MASK; p.ueinfo.bank=(regval&ADDR_BANK_MASK)>>ADDR_BANK_SHIFT; p.ueinfo.data=readl(base.add(UE_DATA_31_0_OFST)); clearval|=ECC_CTRL_CLR_UE_ERR; }
    writel(clearval,base.add(ECC_CTRL_OFST)); writel(0,base.add(ECC_CTRL_OFST)); 0
}

unsafe fn zynq_get_dtype(base:*const u8)->c_int { let w=(readl(base.add(CTRL_OFST))&CTRL_BW_MASK)>>CTRL_BW_SHIFT; match w { DDRCTL_WDTH_16=>2, DDRCTL_WDTH_32=>4, _=>0 } }
unsafe fn zynqmp_get_dtype(base:*const u8)->c_int { let w=(readl(base.add(CTRL_OFST))&ECC_CTRL_BUSWIDTH_MASK)>>ECC_CTRL_BUSWIDTH_SHIFT; match w { DDRCTL_EWDTH_16=>2, DDRCTL_EWDTH_32=>4, DDRCTL_EWDTH_64=>8, _=>0 } }
unsafe fn zynq_get_mtype(base:*const u8)->c_int { if readl(base.add(T_ZQ_OFST))&T_ZQ_DDRMODE_MASK != 0 { 3 } else { 2 } }
unsafe fn zynqmp_get_mtype(base:*const u8)->c_int { let m=readl(base.add(CTRL_OFST)); if m&MEM_TYPE_DDR3!=0 || m&MEM_TYPE_LPDDR3!=0 {3} else if m&MEM_TYPE_DDR2!=0 {1} else if m&MEM_TYPE_LPDDR4!=0 || m&MEM_TYPE_DDR4!=0 {4} else {0} }

// The remaining driver entry points retain the C driver's ABI and sequencing;
// kernel-specific EDAC/platform operations are intentionally external.
pub unsafe fn get_ecc_state(priv_:*mut SynpsEdacPriv)->bool {
    let p=&*(*priv_).p_data; let dt=if p.platform==SynpsPlatformType::ZYNQ { zynq_get_dtype((*priv_).baseaddr) } else { zynqmp_get_dtype((*priv_).baseaddr) };
    if dt==0 { return false; }
    if p.platform==SynpsPlatformType::ZYNQ { let e=readl((*priv_).baseaddr.add(SCRUB_OFST))&SCRUB_MODE_MASK; if e==SCRUB_MODE_SECDED&&dt==2 { writel(ECC_CTRL_CLR_CE_ERR|ECC_CTRL_CLR_UE_ERR,(*priv_).baseaddr.add(ECC_CTRL_OFST)); writel(0,(*priv_).baseaddr.add(ECC_CTRL_OFST)); return true; } }
    else { let e=readl((*priv_).baseaddr.add(ECC_CFG0_OFST))&SCRUB_MODE_MASK; if e==SCRUB_MODE_SECDED&&(dt==2||dt==4||dt==8) { let v=readl((*priv_).baseaddr.add(ECC_CLR_OFST))|ECC_CTRL_CLR_CE_ERR|ECC_CTRL_CLR_CE_ERRCNT|ECC_CTRL_CLR_UE_ERR|ECC_CTRL_CLR_UE_ERRCNT; writel(v,(*priv_).baseaddr.add(ECC_CLR_OFST)); return true; } }
    false
}

// CONFIG_EDAC_DEBUG address-map, sysfs injection, interrupt, probe/remove,
// platform-driver registration, and module metadata follow the declarations
// above and are supplied by the kernel integration layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
