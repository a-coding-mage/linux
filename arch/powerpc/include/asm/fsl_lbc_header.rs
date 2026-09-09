/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Freescale Local Bus Controller */

// C header dependencies are supplied externally.

#[repr(C)]
pub struct fsl_lbc_bank {
    pub br: __be32,
    pub or_: __be32,
}

pub const BR_BA: u32 = 0xFFFF8000;
pub const BR_BA_SHIFT: u32 = 15;
pub const BR_PS: u32 = 0x00001800;
pub const BR_PS_SHIFT: u32 = 11;
pub const BR_PS_8: u32 = 0x00000800;
pub const BR_PS_16: u32 = 0x00001000;
pub const BR_PS_32: u32 = 0x00001800;
pub const BR_DECC: u32 = 0x00000600;
pub const BR_DECC_SHIFT: u32 = 9;
pub const BR_DECC_OFF: u32 = 0x00000000;
pub const BR_DECC_CHK: u32 = 0x00000200;
pub const BR_DECC_CHK_GEN: u32 = 0x00000400;
pub const BR_WP: u32 = 0x00000100;
pub const BR_WP_SHIFT: u32 = 8;
pub const BR_MSEL: u32 = 0x000000E0;
pub const BR_MSEL_SHIFT: u32 = 5;
pub const BR_MS_GPCM: u32 = 0x00000000;
pub const BR_MS_FCM: u32 = 0x00000020;
pub const BR_MS_SDRAM: u32 = 0x00000060;
pub const BR_MS_UPMA: u32 = 0x00000080;
pub const BR_MS_UPMB: u32 = 0x000000A0;
pub const BR_MS_UPMC: u32 = 0x000000C0;
pub const BR_V: u32 = 0x00000001;
pub const BR_V_SHIFT: u32 = 0;
pub const BR_RES: u32 = !(BR_BA | BR_PS | BR_DECC | BR_WP | BR_MSEL | BR_V);

pub const OR0: u32 = 0x5004; pub const OR1: u32 = 0x500C;
pub const OR2: u32 = 0x5014; pub const OR3: u32 = 0x501C;
pub const OR4: u32 = 0x5024; pub const OR5: u32 = 0x502C;
pub const OR6: u32 = 0x5034; pub const OR7: u32 = 0x503C;
pub const OR_FCM_AM: u32 = 0xFFFF8000; pub const OR_FCM_AM_SHIFT: u32 = 15;
pub const OR_FCM_BCTLD: u32 = 0x00001000; pub const OR_FCM_BCTLD_SHIFT: u32 = 12;
pub const OR_FCM_PGS: u32 = 0x00000400; pub const OR_FCM_PGS_SHIFT: u32 = 10;
pub const OR_FCM_CSCT: u32 = 0x00000200; pub const OR_FCM_CSCT_SHIFT: u32 = 9;
pub const OR_FCM_CST: u32 = 0x00000100; pub const OR_FCM_CST_SHIFT: u32 = 8;
pub const OR_FCM_CHT: u32 = 0x00000080; pub const OR_FCM_CHT_SHIFT: u32 = 7;
pub const OR_FCM_SCY: u32 = 0x00000070; pub const OR_FCM_SCY_SHIFT: u32 = 4;
pub const OR_FCM_SCY_1: u32 = 0x10; pub const OR_FCM_SCY_2: u32 = 0x20;
pub const OR_FCM_SCY_3: u32 = 0x30; pub const OR_FCM_SCY_4: u32 = 0x40;
pub const OR_FCM_SCY_5: u32 = 0x50; pub const OR_FCM_SCY_6: u32 = 0x60;
pub const OR_FCM_SCY_7: u32 = 0x70; pub const OR_FCM_RST: u32 = 0x08;
pub const OR_FCM_RST_SHIFT: u32 = 3; pub const OR_FCM_TRLX: u32 = 0x04;
pub const OR_FCM_TRLX_SHIFT: u32 = 2; pub const OR_FCM_EHTR: u32 = 0x02;
pub const OR_FCM_EHTR_SHIFT: u32 = 1; pub const OR_GPCM_AM: u32 = 0xFFFF8000;
pub const OR_GPCM_AM_SHIFT: u32 = 15;

#[repr(C)]
pub struct fsl_lbc_regs {
    pub bank: [fsl_lbc_bank; 12], pub res0: [u8; 0x8], pub mar: __be32,
    pub res1: [u8; 0x4], pub mamr: __be32, pub mbmr: __be32, pub mcmr: __be32,
    pub res2: [u8; 0x8], pub mrtpr: __be32, pub mdr: __be32, pub res3: [u8; 0x4],
    pub lsor: __be32, pub lsdmr: __be32, pub res4: [u8; 0x8], pub lurt: __be32,
    pub lsrt: __be32, pub res5: [u8; 0x8], pub ltesr: __be32, pub ltedr: __be32,
    pub lteir: __be32, pub lteatr: __be32, pub ltear: __be32, pub lteccr: __be32,
    pub res6: [u8; 0x8], pub lbcr: __be32, pub lcrr: __be32, pub res7: [u8; 0x8],
    pub fmr: __be32, pub fir: __be32, pub fcr: __be32, pub fbar: __be32,
    pub fpar: __be32, pub fbcr: __be32,
}

pub const MxMR_OP_NO: u32 = 0 << 28; pub const MxMR_OP_WA: u32 = 1 << 28;
pub const MxMR_OP_RA: u32 = 2 << 28; pub const MxMR_OP_RP: u32 = 3 << 28;
pub const MxMR_MAD: u32 = 0x3f;
pub const LTESR_BM: u32 = 0x80000000; pub const LTESR_FCT: u32 = 0x40000000;
pub const LTESR_PAR: u32 = 0x20000000; pub const LTESR_WP: u32 = 0x04000000;
pub const LTESR_ATMW: u32 = 0x00800000; pub const LTESR_ATMR: u32 = 0x00400000;
pub const LTESR_CS: u32 = 0x00080000; pub const LTESR_UPM: u32 = 0x2;
pub const LTESR_CC: u32 = 0x1;
pub const LTESR_NAND_MASK: u32 = LTESR_FCT | LTESR_PAR | LTESR_CC;
pub const LTESR_MASK: u32 = LTESR_BM | LTESR_FCT | LTESR_PAR | LTESR_WP | LTESR_ATMW | LTESR_ATMR | LTESR_CS | LTESR_UPM | LTESR_CC;
pub const LTESR_CLEAR: u32 = 0xFFFFFFFF; pub const LTECCR_CLEAR: u32 = 0xFFFFFFFF;
pub const LTESR_STATUS: u32 = LTESR_MASK; pub const LTEIR_ENABLE: u32 = LTESR_MASK;
pub const LTEDR_ENABLE: u32 = 0;

pub const LBCR_LDIS: u32 = 0x80000000; pub const LBCR_LDIS_SHIFT: u32 = 31;
pub const LBCR_BCTLC: u32 = 0x00C00000; pub const LBCR_BCTLC_SHIFT: u32 = 22;
pub const LBCR_AHD: u32 = 0x00200000; pub const LBCR_LPBSE: u32 = 0x00020000;
pub const LBCR_LPBSE_SHIFT: u32 = 17; pub const LBCR_EPAR: u32 = 0x00010000;
pub const LBCR_EPAR_SHIFT: u32 = 16; pub const LBCR_BMT: u32 = 0x0000FF00;
pub const LBCR_BMT_SHIFT: u32 = 8; pub const LBCR_BMTPS: u32 = 0xF;
pub const LBCR_BMTPS_SHIFT: u32 = 0; pub const LBCR_INIT: u32 = 0x00040000;
pub const LCRR_DBYP: u32 = 0x80000000; pub const LCRR_DBYP_SHIFT: u32 = 31;
pub const LCRR_BUFCMDC: u32 = 0x30000000; pub const LCRR_BUFCMDC_SHIFT: u32 = 28;
pub const LCRR_ECL: u32 = 0x03000000; pub const LCRR_ECL_SHIFT: u32 = 24;
pub const LCRR_EADC: u32 = 0x00030000; pub const LCRR_EADC_SHIFT: u32 = 16;
pub const LCRR_CLKDIV: u32 = 0xF; pub const LCRR_CLKDIV_SHIFT: u32 = 0;
pub const FMR_CWTO: u32 = 0x0000F000; pub const FMR_CWTO_SHIFT: u32 = 12;
pub const FMR_BOOT: u32 = 0x800; pub const FMR_ECCM: u32 = 0x100;
pub const FMR_AL: u32 = 0x30; pub const FMR_AL_SHIFT: u32 = 4; pub const FMR_OP: u32 = 3; pub const FMR_OP_SHIFT: u32 = 0;
pub const FIR_OP0: u32 = 0xF0000000; pub const FIR_OP0_SHIFT: u32 = 28;
pub const FIR_OP1: u32 = 0x0F000000; pub const FIR_OP1_SHIFT: u32 = 24;
pub const FIR_OP2: u32 = 0x00F00000; pub const FIR_OP2_SHIFT: u32 = 20;
pub const FIR_OP3: u32 = 0x000F0000; pub const FIR_OP3_SHIFT: u32 = 16;
pub const FIR_OP4: u32 = 0x0000F000; pub const FIR_OP4_SHIFT: u32 = 12;
pub const FIR_OP5: u32 = 0x00000F00; pub const FIR_OP5_SHIFT: u32 = 8;
pub const FIR_OP6: u32 = 0xF0; pub const FIR_OP6_SHIFT: u32 = 4; pub const FIR_OP7: u32 = 0xF; pub const FIR_OP7_SHIFT: u32 = 0;
pub const FIR_OP_NOP: u32 = 0; pub const FIR_OP_CA: u32 = 1; pub const FIR_OP_PA: u32 = 2; pub const FIR_OP_UA: u32 = 3;
pub const FIR_OP_CM0: u32 = 4; pub const FIR_OP_CM1: u32 = 5; pub const FIR_OP_CM2: u32 = 6; pub const FIR_OP_CM3: u32 = 7;
pub const FIR_OP_WB: u32 = 8; pub const FIR_OP_WS: u32 = 9; pub const FIR_OP_RB: u32 = 10; pub const FIR_OP_RS: u32 = 11;
pub const FIR_OP_CW0: u32 = 12; pub const FIR_OP_CW1: u32 = 13; pub const FIR_OP_RBW: u32 = 14; pub const FIR_OP_RSW: u32 = 14;
pub const FCR_CMD0: u32 = 0xFF000000; pub const FCR_CMD0_SHIFT: u32 = 24; pub const FCR_CMD1: u32 = 0x00FF0000; pub const FCR_CMD1_SHIFT: u32 = 16;
pub const FCR_CMD2: u32 = 0xFF00; pub const FCR_CMD2_SHIFT: u32 = 8; pub const FCR_CMD3: u32 = 0xFF; pub const FCR_CMD3_SHIFT: u32 = 0;
pub const FBAR_BLK: u32 = 0xFFFFFF; pub const FPAR_SP_PI: u32 = 0x7C00; pub const FPAR_SP_PI_SHIFT: u32 = 10;
pub const FPAR_SP_MS: u32 = 0x200; pub const FPAR_SP_CI: u32 = 0x1FF; pub const FPAR_SP_CI_SHIFT: u32 = 0;
pub const FPAR_LP_PI: u32 = 0x3F000; pub const FPAR_LP_PI_SHIFT: u32 = 12; pub const FPAR_LP_MS: u32 = 0x800;
pub const FPAR_LP_CI: u32 = 0x7FF; pub const FPAR_LP_CI_SHIFT: u32 = 0; pub const FBCR_BC: u32 = 0xFFF;

#[repr(C)]
pub struct fsl_upm { pub mxmr: *mut __be32, pub width: i32 }

extern "C" {
    pub fn fsl_lbc_addr(addr_base: phys_addr_t) -> u32;
    pub fn fsl_lbc_find(addr_base: phys_addr_t) -> i32;
    pub fn fsl_upm_find(addr_base: phys_addr_t, upm: *mut fsl_upm) -> i32;
    pub fn fsl_upm_run_pattern(upm: *mut fsl_upm, io_base: *mut core::ffi::c_void, mar: u32);
    pub static mut fsl_lbc_ctrl_dev: *mut fsl_lbc_ctrl;
}

#[inline]
pub unsafe fn fsl_upm_start_pattern(upm: *mut fsl_upm, pat_offset: u8) {
    clrsetbits_be32((*upm).mxmr, MxMR_MAD, MxMR_OP_RP | pat_offset as u32);
}

#[inline]
pub unsafe fn fsl_upm_end_pattern(upm: *mut fsl_upm) {
    clrbits32((*upm).mxmr, MxMR_OP_RP);
    while in_be32((*upm).mxmr) & MxMR_OP_RP != 0 { cpu_relax(); }
}

#[repr(C)]
pub struct fsl_lbc_ctrl {
    pub dev: *mut device, pub regs: *mut fsl_lbc_regs, pub irq: [i32; 2],
    pub irq_wait: wait_queue_head_t, pub lock: spinlock_t, pub nand: *mut core::ffi::c_void,
    pub irq_status: u32,
    // Preserved from CONFIG_SUSPEND: included only when that build-time condition is enabled.
    #[cfg(CONFIG_SUSPEND)] pub saved_regs: *mut fsl_lbc_regs,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
