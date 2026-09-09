/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of the Book E PowerPC register definitions. */

/* The original file is kernel-only and includes asm/ppc-opcode.h. */

/* Machine State Register (MSR) fields */
pub const MSR_GS_LG: u32 = 28;
pub const MSR_UCLE_LG: u32 = 26;
pub const MSR_SPE_LG: u32 = 25;
pub const MSR_DWE_LG: u32 = 10;
pub const MSR_UBLE_LG: u32 = 10;
pub const MSR_IS_LG: u32 = MSR_IR_LG;
pub const MSR_DS_LG: u32 = MSR_DR_LG;
pub const MSR_PMM_LG: u32 = 2;
pub const MSR_CM_LG: u32 = 31;
pub const MSR_GS: u32 = __MASK(MSR_GS_LG);
pub const MSR_UCLE: u32 = __MASK(MSR_UCLE_LG);
pub const MSR_SPE: u32 = __MASK(MSR_SPE_LG);
pub const MSR_DWE: u32 = __MASK(MSR_DWE_LG);
pub const MSR_UBLE: u32 = __MASK(MSR_UBLE_LG);
pub const MSR_IS: u32 = __MASK(MSR_IS_LG);
pub const MSR_DS: u32 = __MASK(MSR_DS_LG);
pub const MSR_PMM: u32 = __MASK(MSR_PMM_LG);
pub const MSR_CM: u32 = __MASK(MSR_CM_LG);

/* CONFIG_PPC_BOOK3E_64 selects the following alternate definitions. */
pub const MSR_64BIT: u32 = MSR_CM;
pub const MSR_: u32 = MSR_ME | MSR_RI | MSR_CE;
pub const MSR_KERNEL: u32 = MSR_ | MSR_64BIT;
pub const MSR_USER32: u32 = MSR_ | MSR_PR | MSR_EE;
pub const MSR_USER64: u32 = MSR_USER32 | MSR_64BIT;
/* Non-64-bit configuration: MSR_KERNEL=(MSR_ME|MSR_RI|MSR_CE),
 * MSR_USER=(MSR_KERNEL|MSR_PR|MSR_EE). */

/* Special Purpose Registers (SPRNs) */
pub const SPRN_DECAR: u32 = 0x036; pub const SPRN_IVPR: u32 = 0x03f;
pub const SPRN_USPRG0: u32 = 0x100; pub const SPRN_SPRG3R: u32 = 0x103;
pub const SPRN_SPRG4R: u32 = 0x104; pub const SPRN_SPRG5R: u32 = 0x105;
pub const SPRN_SPRG6R: u32 = 0x106; pub const SPRN_SPRG7R: u32 = 0x107;
pub const SPRN_SPRG4W: u32 = 0x114; pub const SPRN_SPRG5W: u32 = 0x115;
pub const SPRN_SPRG6W: u32 = 0x116; pub const SPRN_SPRG7W: u32 = 0x117;
pub const SPRN_EPCR: u32 = 0x133; pub const SPRN_DBCR2: u32 = 0x136;
pub const SPRN_DBCR4: u32 = 0x233; pub const SPRN_MSRP: u32 = 0x137;
pub const SPRN_IAC3: u32 = 0x13a; pub const SPRN_IAC4: u32 = 0x13b;
pub const SPRN_DVC1: u32 = 0x13e; pub const SPRN_DVC2: u32 = 0x13f;
pub const SPRN_LPID: u32 = 0x152; pub const SPRN_MAS8: u32 = 0x155;
pub const SPRN_TLB0PS: u32 = 0x158; pub const SPRN_TLB1PS: u32 = 0x159;
pub const SPRN_MAS5_MAS6: u32 = 0x15c; pub const SPRN_MAS8_MAS1: u32 = 0x15d;
pub const SPRN_EPTCFG: u32 = 0x15e; pub const SPRN_GSPRG0: u32 = 0x170;
pub const SPRN_GSPRG1: u32 = 0x171; pub const SPRN_GSPRG2: u32 = 0x172;
pub const SPRN_GSPRG3: u32 = 0x173; pub const SPRN_MAS7_MAS3: u32 = 0x174;
pub const SPRN_MAS0_MAS1: u32 = 0x175; pub const SPRN_GSRR0: u32 = 0x17a;
pub const SPRN_GSRR1: u32 = 0x17b; pub const SPRN_GEPR: u32 = 0x17c;
pub const SPRN_GDEAR: u32 = 0x17d; pub const SPRN_GPIR: u32 = 0x17e;
pub const SPRN_GESR: u32 = 0x17f;

/* Vector registers and implementation registers. */
pub const SPRN_IVOR0: u32=0x190; pub const SPRN_IVOR1: u32=0x191; pub const SPRN_IVOR2: u32=0x192; pub const SPRN_IVOR3: u32=0x193;
pub const SPRN_IVOR4: u32=0x194; pub const SPRN_IVOR5: u32=0x195; pub const SPRN_IVOR6: u32=0x196; pub const SPRN_IVOR7: u32=0x197;
pub const SPRN_IVOR8: u32=0x198; pub const SPRN_IVOR9: u32=0x199; pub const SPRN_IVOR10: u32=0x19a; pub const SPRN_IVOR11: u32=0x19b;
pub const SPRN_IVOR12: u32=0x19c; pub const SPRN_IVOR13: u32=0x19d; pub const SPRN_IVOR14: u32=0x19e; pub const SPRN_IVOR15: u32=0x19f;
pub const SPRN_IVOR38: u32=0x1b0; pub const SPRN_IVOR39: u32=0x1b1; pub const SPRN_IVOR40: u32=0x1b2; pub const SPRN_IVOR41: u32=0x1b3; pub const SPRN_IVOR42: u32=0x1b4;
pub const SPRN_GIVOR2: u32=0x1b8; pub const SPRN_GIVOR3: u32=0x1b9; pub const SPRN_GIVOR4: u32=0x1ba; pub const SPRN_GIVOR8: u32=0x1bb; pub const SPRN_GIVOR13: u32=0x1bc; pub const SPRN_GIVOR14: u32=0x1bd; pub const SPRN_GIVPR: u32=0x1bf;
pub const SPRN_SPEFSCR: u32=0x200; pub const SPRN_BBEAR: u32=0x201; pub const SPRN_BBTAR: u32=0x202; pub const SPRN_L1CFG0: u32=0x203; pub const SPRN_L1CFG1: u32=0x204; pub const SPRN_ATB: u32=0x20e; pub const SPRN_ATBL: u32=0x20e; pub const SPRN_ATBU: u32=0x20f;
pub const SPRN_IVOR32: u32=0x210; pub const SPRN_IVOR33: u32=0x211; pub const SPRN_IVOR34: u32=0x212; pub const SPRN_IVOR35: u32=0x213; pub const SPRN_IVOR36: u32=0x214; pub const SPRN_IVOR37: u32=0x215;
pub const SPRN_MCARU: u32=0x239; pub const SPRN_MCSRR0: u32=0x23a; pub const SPRN_MCSRR1: u32=0x23b; pub const SPRN_MCSR: u32=0x23c; pub const SPRN_MCAR: u32=0x23d; pub const SPRN_DSRR0: u32=0x23e; pub const SPRN_DSRR1: u32=0x23f;
pub const SPRN_SPRG8: u32=0x25c; pub const SPRN_SPRG9: u32=0x25d; pub const SPRN_L1CSR2: u32=0x25e;
pub const SPRN_MAS0: u32=0x270; pub const SPRN_MAS1: u32=0x271; pub const SPRN_MAS2: u32=0x272; pub const SPRN_MAS3: u32=0x273; pub const SPRN_MAS4: u32=0x274; pub const SPRN_MAS5: u32=0x153; pub const SPRN_MAS6: u32=0x276; pub const SPRN_PID1: u32=0x279; pub const SPRN_PID2: u32=0x27a;
pub const SPRN_TLB0CFG: u32=0x2b0; pub const SPRN_TLB1CFG: u32=0x2b1; pub const SPRN_TLB2CFG: u32=0x2b2; pub const SPRN_TLB3CFG: u32=0x2b3; pub const SPRN_EPR: u32=0x2be; pub const SPRN_CCR1: u32=0x378; pub const SPRN_MAS7: u32=0x3b0; pub const SPRN_MMUCR: u32=0x3b2; pub const SPRN_CCR0: u32=0x3b3; pub const SPRN_EPLC: u32=0x3b3; pub const SPRN_EPSC: u32=0x3b4; pub const SPRN_SGR: u32=0x3b9; pub const SPRN_DCWR: u32=0x3ba; pub const SPRN_SLER: u32=0x3bb; pub const SPRN_DCMP: u32=0x3d1; pub const SPRN_ICDBDR: u32=0x3d3; pub const SPRN_EVPR: u32=0x3d6; pub const SPRN_L1CSR0: u32=0x3f2; pub const SPRN_L1CSR1: u32=0x3f3; pub const SPRN_MMUCSR0: u32=0x3f4; pub const SPRN_MMUCFG: u32=0x3f7; pub const SPRN_BUCSR: u32=0x3f5; pub const SPRN_L2CSR0: u32=0x3f9; pub const SPRN_L2CSR1: u32=0x3fa; pub const SPRN_DCCR: u32=0x3fa; pub const SPRN_ICCR: u32=0x3fb; pub const SPRN_PWRMGTCR0: u32=0x3fb; pub const SPRN_SVR: u32=0x3ff;

/* Conflicting true-Book-E/classic definitions. */
pub const SPRN_CSRR0:u32=0x03a; pub const SPRN_CSRR1:u32=0x03b; pub const SPRN_DEAR:u32=0x03d; pub const SPRN_ESR:u32=0x03e; pub const SPRN_PIR:u32=0x11e; pub const SPRN_DBSR:u32=0x130; pub const SPRN_DBCR0:u32=0x134; pub const SPRN_DBCR1:u32=0x135; pub const SPRN_IAC1:u32=0x138; pub const SPRN_IAC2:u32=0x139; pub const SPRN_DAC1:u32=0x13c; pub const SPRN_DAC2:u32=0x13d; pub const SPRN_TSR:u32=0x150; pub const SPRN_TCR:u32=0x154; pub const SPRN_HACOP:u32=0x15f;

/* Register bit definitions. */
pub const CCR1_DPC:u32=0x00000100; pub const CCR1_TCS:u32=0x00000080;
pub const PWRMGTCR0_PW20_WAIT:u32=1<<14; pub const PWRMGTCR0_PW20_ENT_SHIFT:u32=8; pub const PWRMGTCR0_PW20_ENT:u32=0x3f00; pub const PWRMGTCR0_AV_IDLE_PD_EN:u32=1<<22; pub const PWRMGTCR0_AV_IDLE_CNT_SHIFT:u32=16; pub const PWRMGTCR0_AV_IDLE_CNT:u32=0x3f0000;
pub const MCSR_MCS:u32=0x80000000; pub const MCSR_IB:u32=0x40000000; pub const MCSR_DRB:u32=0x20000000; pub const MCSR_DWB:u32=0x10000000; pub const MCSR_TLBP:u32=0x08000000; pub const MCSR_ICP:u32=0x04000000; pub const MCSR_DCSP:u32=0x02000000; pub const MCSR_DCFP:u32=0x01000000; pub const MCSR_IMPE:u32=0x00800000;
pub const PPC47x_MCSR_GPR:u32=0x01000000; pub const PPC47x_MCSR_FPR:u32=0x00800000; pub const PPC47x_MCSR_IPR:u32=0x00400000;
/* CONFIG_PPC_E500 definitions */
pub const MCSR_MCP:u32=0x80000000; pub const MCSR_ICPERR:u32=0x40000000; pub const MCSR_DCP_PERR:u32=0x20000000; pub const MCSR_DCPERR:u32=0x10000000; pub const MCSR_BUS_IAERR:u32=0x80; pub const MCSR_BUS_RAERR:u32=0x40; pub const MCSR_BUS_WAERR:u32=0x20; pub const MCSR_BUS_IBERR:u32=0x10; pub const MCSR_BUS_RBERR:u32=8; pub const MCSR_BUS_WBERR:u32=4; pub const MCSR_BUS_IPERR:u32=2; pub const MCSR_BUS_RPERR:u32=1;
pub const MCSR_DCPERR_MC:u32=0x20000000; pub const MCSR_L2MMU_MHIT:u32=0x08000000; pub const MCSR_NMI:u32=0x00100000; pub const MCSR_MAV:u32=0x00080000; pub const MCSR_MEA:u32=0x00040000; pub const MCSR_IF:u32=0x00010000; pub const MCSR_LD:u32=0x00008000; pub const MCSR_ST:u32=0x00004000; pub const MCSR_LDG:u32=0x00002000; pub const MCSR_TLBSYNC:u32=2; pub const MCSR_BSL2_ERR:u32=1;
pub const MSRP_UCLEP:u32=0x04000000; pub const MSRP_DEP:u32=0x200; pub const MSRP_PMMP:u32=4;
pub const HID1_PLL_CFG_MASK:u32=0xfc000000; pub const HID1_RFXE:u32=0x00020000; pub const HID1_R1DPE:u32=0x00008000; pub const HID1_R2DPE:u32=0x00004000; pub const HID1_ASTME:u32=0x00002000; pub const HID1_ABE:u32=0x00001000; pub const HID1_MPXTT:u32=0x400; pub const HID1_ATS:u32=0x80; pub const HID1_MID_MASK:u32=0xf;

pub const DBSR_IDE:u32=0x80000000; pub const DBSR_MRR:u32=0x30000000; pub const DBSR_IC:u32=0x08000000; pub const DBSR_BT:u32=0x04000000; pub const DBSR_IRPT:u32=0x02000000; pub const DBSR_TIE:u32=0x01000000; pub const DBSR_IAC1:u32=0x00800000; pub const DBSR_IAC2:u32=0x00400000; pub const DBSR_IAC3:u32=0x00200000; pub const DBSR_IAC4:u32=0x00100000; pub const DBSR_DAC1R:u32=0x00080000; pub const DBSR_DAC1W:u32=0x00040000; pub const DBSR_DAC2R:u32=0x00020000; pub const DBSR_DAC2W:u32=0x00010000; pub const DBSR_RET:u32=0x00008000; pub const DBSR_CIRPT:u32=0x40; pub const DBSR_CRET:u32=0x20; pub const DBSR_IAC12ATS:u32=2; pub const DBSR_IAC34ATS:u32=1;
pub const ESR_MCI:u32=0x80000000; pub const ESR_IMCP:u32=0x80000000; pub const ESR_IMCN:u32=0x40000000; pub const ESR_IMCB:u32=0x20000000; pub const ESR_IMCT:u32=0x10000000; pub const ESR_PIL:u32=0x08000000; pub const ESR_PPR:u32=0x04000000; pub const ESR_PTR:u32=0x02000000; pub const ESR_FP:u32=0x01000000; pub const ESR_DST:u32=0x00800000; pub const ESR_DIZ:u32=0x00400000; pub const ESR_ST:u32=0x00800000; pub const ESR_DLK:u32=0x00200000; pub const ESR_ILK:u32=0x00100000; pub const ESR_PUO:u32=0x00040000; pub const ESR_BO:u32=0x00020000; pub const ESR_SPV:u32=0x80;
pub const DBCR0_EDM:u32=0x80000000; pub const DBCR0_IDM:u32=0x40000000; pub const DBCR0_RST:u32=0x30000000; pub const DBCR0_RST_SYSTEM:u32=0x30000000; pub const DBCR0_RST_CHIP:u32=0x20000000; pub const DBCR0_RST_CORE:u32=0x10000000; pub const DBCR0_RST_NONE:u32=0; pub const DBCR0_ICMP:u32=0x08000000; pub const DBCR0_IC:u32=DBCR0_ICMP; pub const DBCR0_BRT:u32=0x04000000; pub const DBCR0_BT:u32=DBCR0_BRT; pub const DBCR0_IRPT:u32=0x02000000; pub const DBCR0_TDE:u32=0x01000000; pub const DBCR0_TIE:u32=DBCR0_TDE; pub const DBCR0_IAC1:u32=0x00800000; pub const DBCR0_IAC2:u32=0x00400000; pub const DBCR0_IAC3:u32=0x00200000; pub const DBCR0_IAC4:u32=0x00100000; pub const DBCR0_DAC1R:u32=0x00080000; pub const DBCR0_DAC1W:u32=0x00040000; pub const DBCR0_DAC2R:u32=0x00020000; pub const DBCR0_DAC2W:u32=0x00010000; pub const DBCR0_RET:u32=0x8000; pub const DBCR0_CIRPT:u32=0x40; pub const DBCR0_CRET:u32=0x20; pub const DBCR0_FT:u32=1;
pub const DBCR_DAC1R:u32=DBCR0_DAC1R; pub const DBCR_DAC1W:u32=DBCR0_DAC1W; pub const DBCR_DAC2R:u32=DBCR0_DAC2R; pub const DBCR_DAC2W:u32=DBCR0_DAC2W;
pub const DBCR1_IAC1US:u32=0xc0000000; pub const DBCR1_IAC1ER:u32=0x30000000; pub const DBCR1_IAC1ER_01:u32=0x10000000; pub const DBCR1_IAC1ER_10:u32=0x20000000; pub const DBCR1_IAC1ER_11:u32=0x30000000; pub const DBCR1_IAC2US:u32=0x0c000000; pub const DBCR1_IAC2ER:u32=0x03000000; pub const DBCR1_IAC2ER_01:u32=0x01000000; pub const DBCR1_IAC2ER_10:u32=0x02000000; pub const DBCR1_IAC2ER_11:u32=0x03000000; pub const DBCR1_IAC12M:u32=0x00800000; pub const DBCR1_IAC12MX:u32=0x00c00000; pub const DBCR1_IAC12AT:u32=0x00010000; pub const DBCR1_IAC3US:u32=0xc000; pub const DBCR1_IAC3ER:u32=0x3000; pub const DBCR1_IAC3ER_01:u32=0x1000; pub const DBCR1_IAC3ER_10:u32=0x2000; pub const DBCR1_IAC3ER_11:u32=0x3000; pub const DBCR1_IAC4US:u32=0xc00; pub const DBCR1_IAC4ER:u32=0x300; pub const DBCR1_IAC4ER_01:u32=0x100; pub const DBCR1_IAC4ER_10:u32=0x200; pub const DBCR1_IAC4ER_11:u32=0x300; pub const DBCR1_IAC34M:u32=0x80; pub const DBCR1_IAC34MX:u32=0xc0; pub const DBCR1_IAC34AT:u32=1;
pub const DBCR_IAC12I:u32=DBCR1_IAC12M; pub const DBCR_IAC12X:u32=DBCR1_IAC12MX; pub const DBCR_IAC12MODE:u32=DBCR1_IAC12MX; pub const DBCR_IAC34I:u32=DBCR1_IAC34M; pub const DBCR_IAC34X:u32=DBCR1_IAC34MX; pub const DBCR_IAC34MODE:u32=DBCR1_IAC34MX;
pub const DBCR2_DAC1US:u32=0xc0000000; pub const DBCR2_DAC1ER:u32=0x30000000; pub const DBCR2_DAC2US:u32=0x0c000000; pub const DBCR2_DAC2ER:u32=0x03000000; pub const DBCR2_DAC12M:u32=0x00800000; pub const DBCR2_DAC12MM:u32=0x00400000; pub const DBCR2_DAC12MX:u32=0x00c00000; pub const DBCR2_DAC12MODE:u32=0x00c00000; pub const DBCR2_DAC12A:u32=0x00200000; pub const DBCR2_DVC1M:u32=0x000c0000; pub const DBCR2_DVC1M_SHIFT:u32=18; pub const DBCR2_DVC2M:u32=0x00030000; pub const DBCR2_DVC2M_SHIFT:u32=16; pub const DBCR2_DVC1BE:u32=0x00000f00; pub const DBCR2_DVC1BE_SHIFT:u32=8; pub const DBCR2_DVC2BE:u32=0xf; pub const DBCR2_DVC2BE_SHIFT:u32=0;
pub const DBCR1_ACTIVE_EVENTS:u32=0; pub const DBCR0_ACTIVE_EVENTS:u32=DBCR0_ICMP|DBCR0_IAC1|DBCR0_IAC2|DBCR0_IAC3|DBCR0_IAC4|DBCR0_DAC1R|DBCR0_DAC1W|DBCR0_DAC2R|DBCR0_DAC2W;
#[inline] pub const fn DBCR_ACTIVE_EVENTS(dbcr0:u32, dbcr1:u32)->bool { (dbcr0 & DBCR0_ACTIVE_EVENTS) != 0 || (dbcr1 & DBCR1_ACTIVE_EVENTS) != 0 }

#[inline] pub const fn TCR_WP(x:u32)->u32 {(x&3)<<30} pub const TCR_WP_MASK:u32=TCR_WP(3); pub const WP_2_17:u32=0; pub const WP_2_21:u32=1; pub const WP_2_25:u32=2; pub const WP_2_29:u32=3;
#[inline] pub const fn TCR_WRC(x:u32)->u32 {(x&3)<<28} pub const TCR_WRC_MASK:u32=TCR_WRC(3); pub const WRC_NONE:u32=0; pub const WRC_CORE:u32=1; pub const WRC_CHIP:u32=2; pub const WRC_SYSTEM:u32=3;
pub const TCR_WIE:u32=0x08000000; pub const TCR_PIE:u32=0x04000000; pub const TCR_DIE:u32=TCR_PIE; #[inline] pub const fn TCR_FP(x:u32)->u32 {(x&3)<<24} pub const TCR_FP_MASK:u32=TCR_FP(3); pub const FP_2_9:u32=0; pub const FP_2_13:u32=1; pub const FP_2_17:u32=2; pub const FP_2_21:u32=3; pub const TCR_FIE:u32=0x00800000; pub const TCR_ARE:u32=0x00400000;
#[inline] pub const fn TCR_GET_WP(tcr:u32)->u32 {(tcr&0xc0000000)>>30} /* e500 additionally ORs ((tcr&0x1e0000)>>15). */
pub const TSR_ENW:u32=0x80000000; pub const TSR_WIS:u32=0x40000000; #[inline] pub const fn TSR_WRS(x:u32)->u32 {(x&3)<<28} pub const WRS_NONE:u32=0; pub const WRS_CORE:u32=1; pub const WRS_CHIP:u32=2; pub const WRS_SYSTEM:u32=3; pub const TSR_PIS:u32=0x08000000; pub const TSR_DIS:u32=TSR_PIS; pub const TSR_FIS:u32=0x04000000;
pub const DCCR_NOCACHE:u32=0; pub const DCCR_CACHE:u32=1; pub const DCWR_COPY:u32=0; pub const DCWR_WRITE:u32=1; pub const ICCR_NOCACHE:u32=0; pub const ICCR_CACHE:u32=1;
pub const L1CSR0_CPE:u32=0x10000; pub const L1CSR0_CUL:u32=0x400; pub const L1CSR0_CLFC:u32=0x100; pub const L1CSR0_DCFI:u32=2; pub const L1CSR0_CFI:u32=2; pub const L1CSR0_DCE:u32=1; pub const L1CSR1_CPE:u32=0x10000; pub const L1CSR1_ICLFR:u32=0x100; pub const L1CSR1_ICFI:u32=2; pub const L1CSR1_ICE:u32=1; pub const L1CSR2_DCWS:u32=0x40000000;
pub const BUCSR_STAC_EN:u32=0x01000000; pub const BUCSR_LS_EN:u32=0x00400000; pub const BUCSR_BBFI:u32=0x200; pub const BUCSR_BPEN:u32=1; pub const BUCSR_INIT:u32=BUCSR_STAC_EN|BUCSR_LS_EN|BUCSR_BBFI|BUCSR_BPEN;
pub const L2CSR0_L2E:u32=0x80000000; pub const L2CSR0_L2PE:u32=0x40000000; pub const L2CSR0_L2WP:u32=0x1c000000; pub const L2CSR0_L2CM:u32=0x03000000; pub const L2CSR0_L2FI:u32=0x00200000; pub const L2CSR0_L2IO:u32=0x00100000; pub const L2CSR0_L2DO:u32=0x10000; pub const L2CSR0_L2REP:u32=0x3000; pub const L2CSR0_L2FL:u32=0x800; pub const L2CSR0_L2LFC:u32=0x400; pub const L2CSR0_L2LOA:u32=0x80; pub const L2CSR0_L2LO:u32=0x20;
pub const SGR_NORMAL:u32=0; pub const SGR_GUARDED:u32=1;
pub const SPRN_EPCR_EXTGS:u32=0x80000000; pub const SPRN_EPCR_DTLBGS:u32=0x40000000; pub const SPRN_EPCR_ITLBGS:u32=0x20000000; pub const SPRN_EPCR_DSIGS:u32=0x10000000; pub const SPRN_EPCR_ISIGS:u32=0x08000000; pub const SPRN_EPCR_DUVD:u32=0x04000000; pub const SPRN_EPCR_ICM:u32=0x02000000; pub const SPRN_EPCR_GICM:u32=0x01000000; pub const SPRN_EPCR_DGTMI:u32=0x00800000; pub const SPRN_EPCR_DMIUH:u32=0x00400000;
pub const EPC_EPR:u32=0x80000000; pub const EPC_EPR_SHIFT:u32=31; pub const EPC_EAS:u32=0x40000000; pub const EPC_EAS_SHIFT:u32=30; pub const EPC_EGS:u32=0x20000000; pub const EPC_EGS_SHIFT:u32=29; pub const EPC_ELPID:u32=0x00ff0000; pub const EPC_ELPID_SHIFT:u32=16; pub const EPC_EPID:u32=0x00003fff; pub const EPC_EPID_SHIFT:u32=0;
pub const SPRN_SSPCR:u32=830; pub const SPRN_USPCR:u32=831; pub const SPRN_ISPCR:u32=829; pub const SPRN_MMUBE0:u32=820; pub const MMUBE0_IBE0_SHIFT:u32=24; pub const MMUBE0_IBE1_SHIFT:u32=16; pub const MMUBE0_IBE2_SHIFT:u32=8; pub const MMUBE0_VBE0:u32=4; pub const MMUBE0_VBE1:u32=2; pub const MMUBE0_VBE2:u32=1; pub const SPRN_MMUBE1:u32=821; pub const MMUBE1_IBE3_SHIFT:u32=24; pub const MMUBE1_IBE4_SHIFT:u32=16; pub const MMUBE1_IBE5_SHIFT:u32=8; pub const MMUBE1_VBE3:u32=4; pub const MMUBE1_VBE4:u32=2; pub const MMUBE1_VBE5:u32=1;
pub const TMRN_TMCFG0:u32=16; pub const TMRN_TMCFG0_NPRIBITS:u32=0x003f0000; pub const TMRN_TMCFG0_NPRIBITS_SHIFT:u32=16; pub const TMRN_TMCFG0_NATHRD:u32=0x3f00; pub const TMRN_TMCFG0_NATHRD_SHIFT:u32=8; pub const TMRN_TMCFG0_NTHRD:u32=0x3f; pub const TMRN_IMSR0:u32=0x120; pub const TMRN_IMSR1:u32=0x121; pub const TMRN_INIA0:u32=0x140; pub const TMRN_INIA1:u32=0x141; pub const SPRN_TENSR:u32=0x1b5; pub const SPRN_TENS:u32=0x1b6; pub const SPRN_TENC:u32=0x1b7;
#[inline] pub const fn TEN_THREAD(x:u32)->u32 {1u32 << x}
/* C's dbcr_dac/task and assembly-only mftmr/mttmr are intentionally retained
 * as dependency-facing interfaces; their implementation depends on kernel
 * task layout and the external PowerPC opcode macros. */
extern "C" { pub static mut global_dbcr0: [core::ffi::c_ulong; 0]; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
