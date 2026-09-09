/*
 * MPC8xx Internal Memory Map
 * Copyright (c) 1997 Dan Malek (dmalek@jlc.net)
 *
 * Rust translation of the C header. C conditional guards and includes are
 * intentionally represented only by this comment; these items are kernel-side.
 */

use core::ffi::c_char;

pub type uint = u32;
pub type ushort = u16;
pub type u_char = u8;

#[repr(C)]
pub struct sysconf8xx_t { pub sc_siumcr:uint, pub sc_sypcr:uint, pub sc_swt:uint, pub res1:[c_char;2], pub sc_swsr:ushort, pub sc_sipend:uint, pub sc_simask:uint, pub sc_siel:uint, pub sc_sivec:uint, pub sc_tesr:uint, pub res2:[c_char;0xc], pub sc_sdcr:uint, pub res3:[c_char;0x4c] }
#[repr(C)]
pub struct pcmconf8xx_t { pub pcmc_pbr0:uint, pub pcmc_por0:uint, pub pcmc_pbr1:uint, pub pcmc_por1:uint, pub pcmc_pbr2:uint, pub pcmc_por2:uint, pub pcmc_pbr3:uint, pub pcmc_por3:uint, pub pcmc_pbr4:uint, pub pcmc_por4:uint, pub pcmc_pbr5:uint, pub pcmc_por5:uint, pub pcmc_pbr6:uint, pub pcmc_por6:uint, pub pcmc_pbr7:uint, pub pcmc_por7:uint, pub res1:[c_char;0x20], pub pcmc_pgcra:uint, pub pcmc_pgcrb:uint, pub pcmc_pscr:uint, pub res2:[c_char;4], pub pcmc_pipr:uint, pub res3:[c_char;4], pub pcmc_per:uint, pub res4:[c_char;4] }
#[repr(C)]
pub struct memctl8xx_t { pub memc_br0:uint,pub memc_or0:uint,pub memc_br1:uint,pub memc_or1:uint,pub memc_br2:uint,pub memc_or2:uint,pub memc_br3:uint,pub memc_or3:uint,pub memc_br4:uint,pub memc_or4:uint,pub memc_br5:uint,pub memc_or5:uint,pub memc_br6:uint,pub memc_or6:uint,pub memc_br7:uint,pub memc_or7:uint,pub res1:[c_char;0x24],pub memc_mar:uint,pub memc_mcr:uint,pub res2:[c_char;4],pub memc_mamr:uint,pub memc_mbmr:uint,pub memc_mstat:ushort,pub memc_mptpr:ushort,pub memc_mdr:uint,pub res3:[c_char;0x80] }

pub const BR_BA_MSK:uint=0xffff8000; pub const BR_AT_MSK:uint=0x00007000; pub const BR_PS_MSK:uint=0x00000c00; pub const BR_PS_32:uint=0; pub const BR_PS_16:uint=0x800; pub const BR_PS_8:uint=0x400; pub const BR_PARE:uint=0x200; pub const BR_WP:uint=0x100; pub const BR_MS_MSK:uint=0xc0; pub const BR_MS_GPCM:uint=0; pub const BR_MS_UPMA:uint=0x80; pub const BR_MS_UPMB:uint=0xc0; pub const BR_V:uint=1;
pub const OR_AM_MSK:uint=0xffff8000; pub const OR_ATM_MSK:uint=0x7000; pub const OR_CSNT_SAM:uint=0x800; pub const OR_ACS_MSK:uint=0x600; pub const OR_ACS_DIV1:uint=0; pub const OR_ACS_DIV4:uint=0x400; pub const OR_ACS_DIV2:uint=0x600; pub const OR_G5LA:uint=0x400; pub const OR_G5LS:uint=0x200; pub const OR_BI:uint=0x100; pub const OR_SCY_MSK:uint=0xf0; pub const OR_SCY_0_CLK:uint=0; pub const OR_SCY_1_CLK:uint=0x10; pub const OR_SCY_2_CLK:uint=0x20; pub const OR_SCY_3_CLK:uint=0x30; pub const OR_SCY_4_CLK:uint=0x40; pub const OR_SCY_5_CLK:uint=0x50; pub const OR_SCY_6_CLK:uint=0x60; pub const OR_SCY_7_CLK:uint=0x70; pub const OR_SCY_8_CLK:uint=0x80; pub const OR_SCY_9_CLK:uint=0x90; pub const OR_SCY_10_CLK:uint=0xa0; pub const OR_SCY_11_CLK:uint=0xb0; pub const OR_SCY_12_CLK:uint=0xc0; pub const OR_SCY_13_CLK:uint=0xd0; pub const OR_SCY_14_CLK:uint=0xe0; pub const OR_SCY_15_CLK:uint=0xf0; pub const OR_SETA:uint=8; pub const OR_TRLX:uint=4; pub const OR_EHTR:uint=2;

#[repr(C)] pub struct sit8xx_t { pub sit_tbscr:ushort,pub res0:[c_char;2],pub sit_tbreff0:uint,pub sit_tbreff1:uint,pub res1:[c_char;0x14],pub sit_rtcsc:ushort,pub res2:[c_char;2],pub sit_rtc:uint,pub sit_rtsec:uint,pub sit_rtcal:uint,pub res3:[c_char;0x10],pub sit_piscr:ushort,pub res4:[c_char;2],pub sit_pitc:uint,pub sit_pitr:uint,pub res5:[c_char;0x34] }
pub const TBSCR_TBIRQ_MASK:ushort=0xff00; pub const TBSCR_REFA:ushort=0x80; pub const TBSCR_REFB:ushort=0x40; pub const TBSCR_REFAE:ushort=8; pub const TBSCR_REFBE:ushort=4; pub const TBSCR_TBF:ushort=2; pub const TBSCR_TBE:ushort=1;
pub const RTCSC_RTCIRQ_MASK:ushort=0xff00; pub const RTCSC_SEC:ushort=0x80; pub const RTCSC_ALR:ushort=0x40; pub const RTCSC_38K:ushort=0x10; pub const RTCSC_SIE:ushort=8; pub const RTCSC_ALE:ushort=4; pub const RTCSC_RTF:ushort=2; pub const RTCSC_RTE:ushort=1;
pub const PISCR_PIRQ_MASK:ushort=0xff00; pub const PISCR_PS:ushort=0x80; pub const PISCR_PIE:ushort=4; pub const PISCR_PTF:ushort=2; pub const PISCR_PTE:ushort=1;

#[repr(C)] pub struct car8xx_t { pub car_sccr:uint,pub car_plprcr:uint,pub car_rsr:uint,pub res:[c_char;0x74] }
#[repr(C)] pub struct sitk8xx_t { pub sitk_tbscrk:uint,pub sitk_tbreff0k:uint,pub sitk_tbreff1k:uint,pub sitk_tbk:uint,pub res1:[c_char;0x10],pub sitk_rtcsck:uint,pub sitk_rtck:uint,pub sitk_rtseck:uint,pub sitk_rtcalk:uint,pub res2:[c_char;0x10],pub sitk_piscrk:uint,pub sitk_pitck:uint,pub res3:[c_char;0x38] }
#[repr(C)] pub struct cark8xx_t { pub cark_sccrk:uint,pub cark_plprcrk:uint,pub cark_rsrk:uint,pub res:[c_char;0x474] }
pub const KAPWR_KEY:uint=0x55ccaa33;

#[repr(C)] pub struct vid823_t { pub vid_vccr:ushort,pub res1:ushort,pub vid_vsr:u_char,pub res2:u_char,pub vid_vcmr:u_char,pub res3:u_char,pub vid_vbcb:uint,pub res4:uint,pub vid_vfcr0:uint,pub vid_vfaa0:uint,pub vid_vfba0:uint,pub vid_vfcr1:uint,pub vid_vfaa1:uint,pub vid_vfba1:uint,pub res5:[u_char;0x18] }
#[repr(C)] pub struct lcd823_t { pub lcd_lccr:uint,pub lcd_lchcr:uint,pub lcd_lcvcr:uint,pub res1:[c_char;4],pub lcd_lcfaa:uint,pub lcd_lcfba:uint,pub lcd_lcsr:c_char,pub res2:[c_char;7] }
#[repr(C)] pub struct i2c8xx_t { pub i2c_i2mod:u_char,pub res1:[c_char;3],pub i2c_i2add:u_char,pub res2:[c_char;3],pub i2c_i2brg:u_char,pub res3:[c_char;3],pub i2c_i2com:u_char,pub res4:[c_char;3],pub i2c_i2cer:u_char,pub res5:[c_char;3],pub i2c_i2cmr:u_char,pub res6:[c_char;0x8b] }
#[repr(C)] pub struct sdma8xx_t { pub res1:[c_char;4],pub sdma_sdar:uint,pub sdma_sdsr:u_char,pub res3:[c_char;3],pub sdma_sdmr:u_char,pub res4:[c_char;3],pub sdma_idsr1:u_char,pub res5:[c_char;3],pub sdma_idmr1:u_char,pub res6:[c_char;3],pub sdma_idsr2:u_char,pub res7:[c_char;3],pub sdma_idmr2:u_char,pub res8:[c_char;0x13] }
#[repr(C)] pub struct cpic8xx_t { pub cpic_civr:ushort,pub res:[c_char;0xe],pub cpic_cicr:uint,pub cpic_cipr:uint,pub cpic_cimr:uint,pub cpic_cisr:uint }
#[repr(C)] pub struct iop8xx_t { pub iop_padir:ushort,pub iop_papar:ushort,pub iop_paodr:ushort,pub iop_padat:ushort,pub res1:[c_char;8],pub iop_pcdir:ushort,pub iop_pcpar:ushort,pub iop_pcso:ushort,pub iop_pcdat:ushort,pub iop_pcint:ushort,pub res2:[c_char;6],pub iop_pddir:ushort,pub iop_pdpar:ushort,pub res3:[c_char;2],pub iop_pddat:ushort,pub utmode:uint,pub res4:[c_char;4] }
#[repr(C)] pub struct cpmtimer8xx_t { pub cpmt_tgcr:ushort,pub res1:[c_char;0xe],pub cpmt_tmr1:ushort,pub cpmt_tmr2:ushort,pub cpmt_trr1:ushort,pub cpmt_trr2:ushort,pub cpmt_tcr1:ushort,pub cpmt_tcr2:ushort,pub cpmt_tcn1:ushort,pub cpmt_tcn2:ushort,pub cpmt_tmr3:ushort,pub cpmt_tmr4:ushort,pub cpmt_trr3:ushort,pub cpmt_trr4:ushort,pub cpmt_tcr3:ushort,pub cpmt_tcr4:ushort,pub cpmt_tcn3:ushort,pub cpmt_tcn4:ushort,pub cpmt_ter1:ushort,pub cpmt_ter2:ushort,pub cpmt_ter3:ushort,pub cpmt_ter4:ushort,pub res2:[c_char;8] }

#[repr(C)] pub struct scc_t { pub scc_gsmrl:uint,pub scc_gsmrh:uint,pub scc_psmr:ushort,pub res1:[c_char;2],pub scc_todr:ushort,pub scc_dsr:ushort,pub scc_scce:ushort,pub res2:[c_char;2],pub scc_sccm:ushort,pub res3:c_char,pub scc_sccs:u_char,pub res4:[c_char;8] }
#[repr(C)] pub struct smc_t { pub res1:[c_char;2],pub smc_smcmr:ushort,pub res2:[c_char;2],pub smc_smce:u_char,pub res3:[c_char;3],pub smc_smcm:u_char,pub res4:[c_char;5] }
#[repr(C)] pub struct fec_t { pub fec_addr_low:uint,pub fec_addr_high:ushort,pub res1:ushort,pub fec_grp_hash_table_high:uint,pub fec_grp_hash_table_low:uint,pub fec_r_des_start:uint,pub fec_x_des_start:uint,pub fec_r_buff_size:uint,pub res2:[uint;9],pub fec_ecntrl:uint,pub fec_ievent:uint,pub fec_imask:uint,pub fec_ivec:uint,pub fec_r_des_active:uint,pub fec_x_des_active:uint,pub res3:[uint;10],pub fec_mii_data:uint,pub fec_mii_speed:uint,pub res4:[uint;17],pub fec_r_bound:uint,pub fec_r_fstart:uint,pub res5:[uint;6],pub fec_x_fstart:uint,pub res6:[uint;17],pub fec_fun_code:uint,pub res7:[uint;3],pub fec_r_cntrl:uint,pub fec_r_hash:uint,pub res8:[uint;14],pub fec_x_cntrl:uint,pub res9:[uint;0x1e] }
#[repr(C)] pub union fec_lcd { pub fl_un_fec:fec_t, pub fl_un_cmap:[u_char;0x200] }

#[repr(C)] pub struct cpm8xx_t {
 pub cp_cpcr:ushort,pub res1:[u_char;2],pub cp_rccr:ushort,pub res2:u_char,pub cp_rmds:u_char,pub res3:[u_char;4],pub cp_cpmcr1:ushort,pub cp_cpmcr2:ushort,pub cp_cpmcr3:ushort,pub cp_cpmcr4:ushort,pub res4:[u_char;2],pub cp_rter:ushort,pub res5:[u_char;2],pub cp_rtmr:ushort,pub res6:[u_char;0x14],
 pub cp_brgc1:uint,pub cp_brgc2:uint,pub cp_brgc3:uint,pub cp_brgc4:uint,
 pub cp_scc:[scc_t;4],pub cp_smc:[smc_t;2],
 pub cp_spmode:ushort,pub res7:[u_char;4],pub cp_spie:u_char,pub res8:[u_char;3],pub cp_spim:u_char,pub res9:[u_char;2],pub cp_spcom:u_char,pub res10:[u_char;2],
 pub res11:[u_char;2],pub cp_pipc:ushort,pub res12:[u_char;2],pub cp_ptpr:ushort,pub cp_pbdir:uint,pub cp_pbpar:uint,pub res13:[u_char;2],pub cp_pbodr:ushort,pub cp_pbdat:uint,
 pub cp_pedir:uint,pub cp_pepar:uint,pub cp_peso:uint,pub cp_peodr:uint,pub cp_pedat:uint,pub cp_cptr:uint,
 pub cp_simode:uint,pub cp_sigmr:u_char,pub res15:u_char,pub cp_sistr:u_char,pub cp_sicmr:u_char,pub res16:[u_char;4],pub cp_sicr:uint,pub cp_sirp:uint,pub res17:[u_char;0xc],pub cp_vcram:[u_char;0x100],pub cp_siram:[u_char;0x200],pub fl_un:fec_lcd,pub res18:[c_char;0xE00],pub cp_fec2:fec_t,pub cp_dpmem:[u_char;0x1C00],pub cp_dparam:[u_char;0x400]
}

#[repr(C)] pub struct immap_t { pub im_siu_conf:sysconf8xx_t,pub im_pcmcia:pcmconf8xx_t,pub im_memctl:memctl8xx_t,pub im_sit:sit8xx_t,pub im_clkrst:car8xx_t,pub im_sitk:sitk8xx_t,pub im_clkrstk:cark8xx_t,pub im_vid:vid823_t,pub im_lcd:lcd823_t,pub im_i2c:i2c8xx_t,pub im_sdma:sdma8xx_t,pub im_cpic:cpic8xx_t,pub im_ioport:iop8xx_t,pub im_cpmtimer:cpmtimer8xx_t,pub im_cpm:cpm8xx_t }

// C macros: cp_fec = fl_un.fl_un_fec; lcd_cmap = fl_un.fl_un_cmap; cp_fec1 = cp_fec.
// The kernel-provided __iomem annotation has no Rust equivalent.
unsafe extern "C" { pub static mut mpc8xx_immr: *mut immap_t; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
