/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Contains register definitions common to PowerPC 8xx CPUs.  Notice
 */

/* Cache control on the MPC8xx is provided through some additional
 * special purpose registers.
 */
pub const SPRN_IC_CST: u32 = 560; /* Instruction cache control/status */
pub const SPRN_IC_ADR: u32 = 561; /* Address needed for some commands */
pub const SPRN_IC_DAT: u32 = 562; /* Read-only data register */
pub const SPRN_DC_CST: u32 = 568; /* Data cache control/status */
pub const SPRN_DC_ADR: u32 = 569; /* Address needed for some commands */
pub const SPRN_DC_DAT: u32 = 570; /* Read-only data register */

/* Misc Debug */
pub const SPRN_DPDR: u32 = 630;
pub const SPRN_MI_CAM: u32 = 816;
pub const SPRN_MI_RAM0: u32 = 817;
pub const SPRN_MI_RAM1: u32 = 818;
pub const SPRN_MD_CAM: u32 = 824;
pub const SPRN_MD_RAM0: u32 = 825;
pub const SPRN_MD_RAM1: u32 = 826;

/* Special MSR manipulation registers */
pub const SPRN_EIE: u32 = 80; /* External interrupt enable (EE=1, RI=1) */
pub const SPRN_EID: u32 = 81; /* External interrupt disable (EE=0, RI=1) */
pub const SPRN_NRI: u32 = 82; /* Non recoverable interrupt (EE=0, RI=0) */

/* Debug registers */
pub const SPRN_CMPA: u32 = 144;
pub const SPRN_COUNTA: u32 = 150;
pub const SPRN_CMPE: u32 = 152;
pub const SPRN_CMPF: u32 = 153;
pub const SPRN_LCTRL1: u32 = 156;
pub const LCTRL1_CTE_GT: u32 = 0xc0000000;
pub const LCTRL1_CTF_LT: u32 = 0x14000000;
pub const LCTRL1_CRWE_RW: u32 = 0x00000000;
pub const LCTRL1_CRWE_RO: u32 = 0x00040000;
pub const LCTRL1_CRWE_WO: u32 = 0x000c0000;
pub const LCTRL1_CRWF_RW: u32 = 0x00000000;
pub const LCTRL1_CRWF_RO: u32 = 0x00010000;
pub const LCTRL1_CRWF_WO: u32 = 0x00030000;
pub const SPRN_LCTRL2: u32 = 157;
pub const LCTRL2_LW0EN: u32 = 0x80000000;
pub const LCTRL2_LW0LA_E: u32 = 0x00000000;
pub const LCTRL2_LW0LA_F: u32 = 0x04000000;
pub const LCTRL2_LW0LA_EandF: u32 = 0x08000000;
pub const LCTRL2_LW0LADC: u32 = 0x02000000;
pub const LCTRL2_SLW0EN: u32 = 0x00000002;
/* Defined only when CONFIG_PPC_8xx is enabled in the C build. */
pub const SPRN_ICTRL: u32 = 158;
pub const SPRN_BAR: u32 = 159;

/* Commands.  Only the first few are available to the instruction cache.
 */
pub const IDC_ENABLE: u32 = 0x02000000; /* Cache enable */
pub const IDC_DISABLE: u32 = 0x04000000; /* Cache disable */
pub const IDC_LDLCK: u32 = 0x06000000; /* Load and lock */
pub const IDC_UNLINE: u32 = 0x08000000; /* Unlock line */
pub const IDC_UNALL: u32 = 0x0a000000; /* Unlock all */
pub const IDC_INVALL: u32 = 0x0c000000; /* Invalidate all */

pub const DC_FLINE: u32 = 0x0e000000; /* Flush data cache line */
pub const DC_SFWT: u32 = 0x01000000; /* Set forced writethrough mode */
pub const DC_CFWT: u32 = 0x03000000; /* Clear forced writethrough mode */
pub const DC_SLES: u32 = 0x05000000; /* Set little endian swap mode */
pub const DC_CLES: u32 = 0x07000000; /* Clear little endian swap mode */

/* Status.
 */
pub const IDC_ENABLED: u32 = 0x80000000; /* Cache is enabled */
pub const IDC_CERR1: u32 = 0x00200000; /* Cache error 1 */
pub const IDC_CERR2: u32 = 0x00100000; /* Cache error 2 */
pub const IDC_CERR3: u32 = 0x00080000; /* Cache error 3 */

pub const DC_DFWT: u32 = 0x40000000; /* Data cache is forced write through */
pub const DC_LES: u32 = 0x20000000; /* Caches are little endian mode */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
