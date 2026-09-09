/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Faithful Rust translation of bcm1480_regs.h; dependencies from included headers remain external. */
#![allow(non_upper_case_globals, dead_code)]

/* SPDX-License-Identifier: GPL-2.0-or-later */
/*  *********************************************************************
    *  BCM1255/BCM1280/BCM1455/BCM1480 Board Support Package
    *
    *  Register Definitions			File: bcm1480_regs.h
    *
    *  This module contains the addresses of the on-chip peripherals
    *  on the BCM1280 and BCM1480.
    *
    *  BCM1480 specification level:  1X55_1X80-UM100-D4 (11/24/03)
    *
    *********************************************************************
    *
    *  Copyright 2000,2001,2002,2003
    *  Broadcom Corporation. All rights reserved.
    *
    ********************************************************************* */



/*  *********************************************************************
    *  Pull in the BCM1250's registers since a great deal of the 1480's
    *  functions are the same as the BCM1250.
    ********************************************************************* */



/*  *********************************************************************
    *  Some general notes:
    *
    *  Register addresses are grouped by function and follow the order
    *  of the User Manual.
    *
    *  For the most part, when there is more than one peripheral
    *  of the same type on the SOC, the constants below will be
    *  offsets from the base of each peripheral.  For example,
    *  the MAC registers are described as offsets from the first
    *  MAC register, and there will be a MAC_REGISTER() macro
    *  to calculate the base address of a given MAC.
    *
    *  The information in this file is based on the BCM1X55/BCM1X80
    *  User Manual, Document 1X55_1X80-UM100-R, 22/12/03.
    *
    *  This file is basically a "what's new" header file.  Since the
    *  BCM1250 and the new BCM1480 (and derivatives) share many common
    *  features, this file contains only what's new or changed from
    *  the 1250.  (above, you can see that we include the 1250 symbols
    *  to get the base functionality).
    *
    *  In software, be sure to use the correct symbols, particularly
    *  for blocks that are different between the two chip families.
    *  All BCM1480-specific symbols have _BCM1480_ in their names,
    *  and all BCM1250-specific and "base" functions that are common in
    *  both chips have no special names (this is for compatibility with
    *  older include files).  Therefore, if you're working with the
    *  SCD, which is very different on each chip, A_SCD_xxx implies
    *  the BCM1250 version and A_BCM1480_SCD_xxx implies the BCM1480
    *  version.
    ********************************************************************* */


/*  *********************************************************************
    * Memory Controller Registers (Section 6)
    ********************************************************************* */

pub const A_BCM1480_MC_BASE_0: u64 = 0x0010050000;
pub const A_BCM1480_MC_BASE_1: u64 = 0x0010051000;
pub const A_BCM1480_MC_BASE_2: u64 = 0x0010052000;
pub const A_BCM1480_MC_BASE_3: u64 = 0x0010053000;
pub const BCM1480_MC_REGISTER_SPACING: u64 = 0x1000;

#[inline]
pub const fn A_BCM1480_MC_BASE(ctlid: u64) -> u64 { (A_BCM1480_MC_BASE_0+(ctlid)*BCM1480_MC_REGISTER_SPACING) }
#[inline]
pub const fn A_BCM1480_MC_REGISTER(ctlid, : u64) -> u64 { reg)    (A_BCM1480_MC_BASE(ctlid)+(reg)) }

pub const R_BCM1480_MC_CONFIG: u64 = 0x0000000100;
pub const R_BCM1480_MC_CS_START: u64 = 0x0000000120;
pub const R_BCM1480_MC_CS_END: u64 = 0x0000000140;
pub const S_BCM1480_MC_CS_STARTEND: u64 = 24;

pub const R_BCM1480_MC_CS01_ROW0: u64 = 0x0000000180;
pub const R_BCM1480_MC_CS01_ROW1: u64 = 0x00000001A0;
pub const R_BCM1480_MC_CS23_ROW0: u64 = 0x0000000200;
pub const R_BCM1480_MC_CS23_ROW1: u64 = 0x0000000220;
pub const R_BCM1480_MC_CS01_COL0: u64 = 0x0000000280;
pub const R_BCM1480_MC_CS01_COL1: u64 = 0x00000002A0;
pub const R_BCM1480_MC_CS23_COL0: u64 = 0x0000000300;
pub const R_BCM1480_MC_CS23_COL1: u64 = 0x0000000320;

pub const R_BCM1480_MC_CSX_BASE: u64 = 0x0000000180;
pub const R_BCM1480_MC_CSX_ROW0: u64 = 0x0000000000   /* relative to CSX_BASE */;
pub const R_BCM1480_MC_CSX_ROW1: u64 = 0x0000000020   /* relative to CSX_BASE */;
pub const R_BCM1480_MC_CSX_COL0: u64 = 0x0000000100   /* relative to CSX_BASE */;
pub const R_BCM1480_MC_CSX_COL1: u64 = 0x0000000120   /* relative to CSX_BASE */;
pub const BCM1480_MC_CSX_SPACING: u64 = 0x0000000080   /* CS23 relative to CS01 */;

pub const R_BCM1480_MC_CS01_BA: u64 = 0x0000000380;
pub const R_BCM1480_MC_CS23_BA: u64 = 0x00000003A0;
pub const R_BCM1480_MC_DRAMCMD: u64 = 0x0000000400;
pub const R_BCM1480_MC_DRAMMODE: u64 = 0x0000000420;
pub const R_BCM1480_MC_CLOCK_CFG: u64 = 0x0000000440;
pub const R_BCM1480_MC_MCLK_CFG: u64 = R_BCM1480_MC_CLOCK_CFG;
pub const R_BCM1480_MC_TEST_DATA: u64 = 0x0000000480;
pub const R_BCM1480_MC_TEST_ECC: u64 = 0x00000004A0;
pub const R_BCM1480_MC_TIMING1: u64 = 0x00000004C0;
pub const R_BCM1480_MC_TIMING2: u64 = 0x00000004E0;
pub const R_BCM1480_MC_DLL_CFG: u64 = 0x0000000500;
pub const R_BCM1480_MC_DRIVE_CFG: u64 = 0x0000000520;

// Conditional source block: SIBYTE_HDR_FEATURE(1480, PASS2).
#[cfg(feature = "bcm1480_pass2")]
pub const R_BCM1480_MC_ODT: u64 = 0x0000000460;
pub const R_BCM1480_MC_ECC_STATUS: u64 = 0x0000000540;

/* Global registers (single instance) */
#define A_BCM1480_MC_GLB_CONFIG		    0x0010054100
#define A_BCM1480_MC_GLB_INTLV		    0x0010054120
#define A_BCM1480_MC_GLB_ECC_STATUS	    0x0010054140
#define A_BCM1480_MC_GLB_ECC_ADDR	    0x0010054160
#define A_BCM1480_MC_GLB_ECC_CORRECT	    0x0010054180
#define A_BCM1480_MC_GLB_PERF_CNT_CONTROL   0x00100541A0

/*  *********************************************************************
    * L2 Cache Control Registers (Section 5)
    ********************************************************************* */

pub const A_BCM1480_L2_BASE: u64 = 0x0010040000;

pub const A_BCM1480_L2_READ_TAG: u64 = 0x0010040018;
pub const A_BCM1480_L2_ECC_TAG: u64 = 0x0010040038;
pub const A_BCM1480_L2_MISC0_VALUE: u64 = 0x0010040058;
pub const A_BCM1480_L2_MISC1_VALUE: u64 = 0x0010040078;
pub const A_BCM1480_L2_MISC2_VALUE: u64 = 0x0010040098;
pub const A_BCM1480_L2_MISC_CONFIG: u64 = 0x0010040040	/* x040 */;
pub const A_BCM1480_L2_CACHE_DISABLE: u64 = 0x0010040060	/* x060 */;
#[inline]
pub const fn A_BCM1480_L2_MAKECACHEDISABLE(x: u64) -> u64 { (A_BCM1480_L2_CACHE_DISABLE | (((x)&0xF) << 12)) }
pub const A_BCM1480_L2_WAY_ENABLE_3_0: u64 = 0x0010040080	/* x080 */;
pub const A_BCM1480_L2_WAY_ENABLE_7_4: u64 = 0x00100400A0	/* x0A0 */;
#[inline]
pub const fn A_BCM1480_L2_MAKE_WAY_ENABLE_LO(x: u64) -> u64 { (A_BCM1480_L2_WAY_ENABLE_3_0 | (((x)&0xF) << 12)) }
#[inline]
pub const fn A_BCM1480_L2_MAKE_WAY_ENABLE_HI(x: u64) -> u64 { (A_BCM1480_L2_WAY_ENABLE_7_4 | (((x)&0xF) << 12)) }
#[inline]
pub const fn A_BCM1480_L2_MAKE_WAY_DISABLE_LO(x: u64) -> u64 { (A_BCM1480_L2_WAY_ENABLE_3_0 | (((~x)&0xF) << 12)) }
#[inline]
pub const fn A_BCM1480_L2_MAKE_WAY_DISABLE_HI(x: u64) -> u64 { (A_BCM1480_L2_WAY_ENABLE_7_4 | (((~x)&0xF) << 12)) }
pub const A_BCM1480_L2_WAY_LOCAL_3_0: u64 = 0x0010040100	/* x100 */;
pub const A_BCM1480_L2_WAY_LOCAL_7_4: u64 = 0x0010040120	/* x120 */;
pub const A_BCM1480_L2_WAY_REMOTE_3_0: u64 = 0x0010040140	/* x140 */;
pub const A_BCM1480_L2_WAY_REMOTE_7_4: u64 = 0x0010040160	/* x160 */;
pub const A_BCM1480_L2_WAY_AGENT_3_0: u64 = 0x00100400C0	/* xxC0 */;
pub const A_BCM1480_L2_WAY_AGENT_7_4: u64 = 0x00100400E0	/* xxE0 */;
#[inline]
pub const fn A_BCM1480_L2_WAY_ENABLE(A, : u64) -> u64 { banks)   (A | (((~(banks))&0x0F) << 8)) }
pub const A_BCM1480_L2_BANK_BASE: u64 = 0x00D0300000;
#[inline]
pub const fn A_BCM1480_L2_BANK_ADDRESS(b: u64) -> u64 { (A_BCM1480_L2_BANK_BASE | (((b)&0x7)<<17)) }
pub const A_BCM1480_L2_MGMT_TAG_BASE: u64 = 0x00D0000000;


/*  *********************************************************************
    * PCI-X Interface Registers (Section 7)
    ********************************************************************* */

pub const A_BCM1480_PCI_BASE: u64 = 0x0010061400;

pub const A_BCM1480_PCI_RESET: u64 = 0x0010061400;
pub const A_BCM1480_PCI_DLL: u64 = 0x0010061500;

pub const A_BCM1480_PCI_TYPE00_HEADER: u64 = 0x002E000000;

/*  *********************************************************************
    * Ethernet MAC Registers (Section 11) and DMA Registers (Section 10.6)
    ********************************************************************* */

/* No register changes with Rev.C BCM1250, but one additional MAC */

#define A_BCM1480_MAC_BASE_2	    0x0010066000

#ifndef A_MAC_BASE_2
#define A_MAC_BASE_2		    A_BCM1480_MAC_BASE_2
#endif

#define A_BCM1480_MAC_BASE_3	    0x0010067000
#define A_MAC_BASE_3		    A_BCM1480_MAC_BASE_3

#define R_BCM1480_MAC_DMA_OODPKTLOST	    0x00000038

#ifndef R_MAC_DMA_OODPKTLOST
#define R_MAC_DMA_OODPKTLOST	    R_BCM1480_MAC_DMA_OODPKTLOST
#endif


/*  *********************************************************************
    * DUART Registers (Section 14)
    ********************************************************************* */

/* No significant differences from BCM1250, two DUARTs */

/*  Conventions, per user manual:
 *     DUART	generic, channels A,B,C,D
 *     DUART0	implementing channels A,B
 *     DUART1	inplementing channels C,D
 */

pub const BCM1480_DUART_NUM_PORTS: u64 = 4;

pub const A_BCM1480_DUART0: u64 = 0x0010060000;
pub const A_BCM1480_DUART1: u64 = 0x0010060400;
#[inline]
pub const fn A_BCM1480_DUART(chan: u64) -> u64 { ((((chan)&2) == 0)? A_BCM1480_DUART0 : A_BCM1480_DUART1) }

pub const BCM1480_DUART_CHANREG_SPACING: u64 = 0x100;
#[inline]
pub const fn A_BCM1480_DUART_CHANREG(chan, : u64) -> u64 { reg)				\ }
	(A_BCM1480_DUART(chan) +					\
	 BCM1480_DUART_CHANREG_SPACING * (((chan) & 1) + 1) + (reg))
#[inline]
pub const fn A_BCM1480_DUART_CTRLREG(chan, : u64) -> u64 { reg)				\ }
	(A_BCM1480_DUART(chan) +					\
	 BCM1480_DUART_CHANREG_SPACING * 3 + (reg))

pub const DUART_IMRISR_SPACING: u64 = 0x20;
pub const DUART_INCHNG_SPACING: u64 = 0x10;

#[inline]
pub const fn R_BCM1480_DUART_IMRREG(chan: u64) -> u64 { \ }
	(R_DUART_IMR_A + ((chan) & 1) * DUART_IMRISR_SPACING)
#[inline]
pub const fn R_BCM1480_DUART_ISRREG(chan: u64) -> u64 { \ }
	(R_DUART_ISR_A + ((chan) & 1) * DUART_IMRISR_SPACING)
#[inline]
pub const fn R_BCM1480_DUART_INCHREG(chan: u64) -> u64 { \ }
	(R_DUART_IN_CHNG_A + ((chan) & 1) * DUART_INCHNG_SPACING)

#[inline]
pub const fn A_BCM1480_DUART_IMRREG(chan: u64) -> u64 { \ }
	(A_BCM1480_DUART_CTRLREG((chan), R_BCM1480_DUART_IMRREG(chan)))
#[inline]
pub const fn A_BCM1480_DUART_ISRREG(chan: u64) -> u64 { \ }
	(A_BCM1480_DUART_CTRLREG((chan), R_BCM1480_DUART_ISRREG(chan)))

#[inline]
pub const fn A_BCM1480_DUART_IN_PORT(chan: u64) -> u64 { \ }
	(A_BCM1480_DUART_CTRLREG((chan), R_DUART_IN_PORT))

/*
 * These constants are the absolute addresses.
 */

pub const A_BCM1480_DUART_MODE_REG_1_C: u64 = 0x0010060400;
pub const A_BCM1480_DUART_MODE_REG_2_C: u64 = 0x0010060410;
pub const A_BCM1480_DUART_STATUS_C: u64 = 0x0010060420;
pub const A_BCM1480_DUART_CLK_SEL_C: u64 = 0x0010060430;
pub const A_BCM1480_DUART_FULL_CTL_C: u64 = 0x0010060440;
pub const A_BCM1480_DUART_CMD_C: u64 = 0x0010060450;
pub const A_BCM1480_DUART_RX_HOLD_C: u64 = 0x0010060460;
pub const A_BCM1480_DUART_TX_HOLD_C: u64 = 0x0010060470;
pub const A_BCM1480_DUART_OPCR_C: u64 = 0x0010060480;
pub const A_BCM1480_DUART_AUX_CTRL_C: u64 = 0x0010060490;

pub const A_BCM1480_DUART_MODE_REG_1_D: u64 = 0x0010060500;
pub const A_BCM1480_DUART_MODE_REG_2_D: u64 = 0x0010060510;
pub const A_BCM1480_DUART_STATUS_D: u64 = 0x0010060520;
pub const A_BCM1480_DUART_CLK_SEL_D: u64 = 0x0010060530;
pub const A_BCM1480_DUART_FULL_CTL_D: u64 = 0x0010060540;
pub const A_BCM1480_DUART_CMD_D: u64 = 0x0010060550;
pub const A_BCM1480_DUART_RX_HOLD_D: u64 = 0x0010060560;
pub const A_BCM1480_DUART_TX_HOLD_D: u64 = 0x0010060570;
pub const A_BCM1480_DUART_OPCR_D: u64 = 0x0010060580;
pub const A_BCM1480_DUART_AUX_CTRL_D: u64 = 0x0010060590;

pub const A_BCM1480_DUART_INPORT_CHNG_CD: u64 = 0x0010060600;
pub const A_BCM1480_DUART_AUX_CTRL_CD: u64 = 0x0010060610;
pub const A_BCM1480_DUART_ISR_C: u64 = 0x0010060620;
pub const A_BCM1480_DUART_IMR_C: u64 = 0x0010060630;
pub const A_BCM1480_DUART_ISR_D: u64 = 0x0010060640;
pub const A_BCM1480_DUART_IMR_D: u64 = 0x0010060650;
pub const A_BCM1480_DUART_OUT_PORT_CD: u64 = 0x0010060660;
pub const A_BCM1480_DUART_OPCR_CD: u64 = 0x0010060670;
pub const A_BCM1480_DUART_IN_PORT_CD: u64 = 0x0010060680;
pub const A_BCM1480_DUART_ISR_CD: u64 = 0x0010060690;
pub const A_BCM1480_DUART_IMR_CD: u64 = 0x00100606A0;
pub const A_BCM1480_DUART_SET_OPR_CD: u64 = 0x00100606B0;
pub const A_BCM1480_DUART_CLEAR_OPR_CD: u64 = 0x00100606C0;
pub const A_BCM1480_DUART_INPORT_CHNG_C: u64 = 0x00100606D0;
pub const A_BCM1480_DUART_INPORT_CHNG_D: u64 = 0x00100606E0;


/*  *********************************************************************
    * Generic Bus Registers (Section 15) and PCMCIA Registers (Section 16)
    ********************************************************************* */

pub const A_BCM1480_IO_PCMCIA_CFG_B: u64 = 0x0010061A58;
pub const A_BCM1480_IO_PCMCIA_STATUS_B: u64 = 0x0010061A68;

/*  *********************************************************************
    * GPIO Registers (Section 17)
    ********************************************************************* */

/* One additional GPIO register, placed _before_ the BCM1250's GPIO block base */

#define A_BCM1480_GPIO_INT_ADD_TYPE	    0x0010061A78
#define R_BCM1480_GPIO_INT_ADD_TYPE	    (-8)

#define A_GPIO_INT_ADD_TYPE	A_BCM1480_GPIO_INT_ADD_TYPE
#define R_GPIO_INT_ADD_TYPE	R_BCM1480_GPIO_INT_ADD_TYPE

/*  *********************************************************************
    * SMBus Registers (Section 18)
    ********************************************************************* */

/* No changes from BCM1250 */

/*  *********************************************************************
    * Timer Registers (Sections 4.6)
    ********************************************************************* */

/* BCM1480 has two additional watchdogs */

/* Watchdog timers */

#define A_BCM1480_SCD_WDOG_2		    0x0010022050
#define A_BCM1480_SCD_WDOG_3		    0x0010022150

#define BCM1480_SCD_NUM_WDOGS		    4

#define A_BCM1480_SCD_WDOG_BASE(w)	 (A_BCM1480_SCD_WDOG_0+((w)&2)*0x1000 + ((w)&1)*0x100)
#define A_BCM1480_SCD_WDOG_REGISTER(w, r) (A_BCM1480_SCD_WDOG_BASE(w) + (r))

#define A_BCM1480_SCD_WDOG_INIT_2	0x0010022050
#define A_BCM1480_SCD_WDOG_CNT_2	0x0010022058
#define A_BCM1480_SCD_WDOG_CFG_2	0x0010022060

#define A_BCM1480_SCD_WDOG_INIT_3	0x0010022150
#define A_BCM1480_SCD_WDOG_CNT_3	0x0010022158
#define A_BCM1480_SCD_WDOG_CFG_3	0x0010022160

/* BCM1480 has two additional compare registers */

#define A_BCM1480_SCD_ZBBUS_CYCLE_COUNT		A_SCD_ZBBUS_CYCLE_COUNT
#define A_BCM1480_SCD_ZBBUS_CYCLE_CP_BASE	0x0010020C00
#define A_BCM1480_SCD_ZBBUS_CYCLE_CP0		A_SCD_ZBBUS_CYCLE_CP0
#define A_BCM1480_SCD_ZBBUS_CYCLE_CP1		A_SCD_ZBBUS_CYCLE_CP1
#define A_BCM1480_SCD_ZBBUS_CYCLE_CP2		0x0010020C10
#define A_BCM1480_SCD_ZBBUS_CYCLE_CP3		0x0010020C18

/*  *********************************************************************
    * System Control Registers (Section 4.2)
    ********************************************************************* */

/* Scratch register in different place */

#define A_BCM1480_SCD_SCRATCH		0x100200A0

/*  *********************************************************************
    * System Address Trap Registers (Section 4.9)
    ********************************************************************* */

/* No changes from BCM1250 */

/*  *********************************************************************
    * System Interrupt Mapper Registers (Sections 4.3-4.5)
    ********************************************************************* */

pub const A_BCM1480_IMR_CPU0_BASE: u64 = 0x0010020000;
pub const A_BCM1480_IMR_CPU1_BASE: u64 = 0x0010022000;
pub const A_BCM1480_IMR_CPU2_BASE: u64 = 0x0010024000;
pub const A_BCM1480_IMR_CPU3_BASE: u64 = 0x0010026000;
pub const BCM1480_IMR_REGISTER_SPACING: u64 = 0x2000;
pub const BCM1480_IMR_REGISTER_SPACING_SHIFT: u64 = 13;

#[inline]
pub const fn A_BCM1480_IMR_MAPPER(cpu: u64) -> u64 { (A_BCM1480_IMR_CPU0_BASE+(cpu)*BCM1480_IMR_REGISTER_SPACING) }
#[inline]
pub const fn A_BCM1480_IMR_REGISTER(cpu, : u64) -> u64 { reg) (A_BCM1480_IMR_MAPPER(cpu)+(reg)) }

/* Most IMR registers are 128 bits, implemented as non-contiguous
   64-bit registers high (_H) and low (_L) */
pub const BCM1480_IMR_HL_SPACING: u64 = 0x1000;

pub const R_BCM1480_IMR_INTERRUPT_DIAG_H: u64 = 0x0010;
pub const R_BCM1480_IMR_LDT_INTERRUPT_H: u64 = 0x0018;
pub const R_BCM1480_IMR_LDT_INTERRUPT_CLR_H: u64 = 0x0020;
pub const R_BCM1480_IMR_INTERRUPT_MASK_H: u64 = 0x0028;
pub const R_BCM1480_IMR_INTERRUPT_TRACE_H: u64 = 0x0038;
pub const R_BCM1480_IMR_INTERRUPT_SOURCE_STATUS_H: u64 = 0x0040;
pub const R_BCM1480_IMR_LDT_INTERRUPT_SET: u64 = 0x0048;
pub const R_BCM1480_IMR_MAILBOX_0_CPU: u64 = 0x00C0;
pub const R_BCM1480_IMR_MAILBOX_0_SET_CPU: u64 = 0x00C8;
pub const R_BCM1480_IMR_MAILBOX_0_CLR_CPU: u64 = 0x00D0;
pub const R_BCM1480_IMR_MAILBOX_1_CPU: u64 = 0x00E0;
pub const R_BCM1480_IMR_MAILBOX_1_SET_CPU: u64 = 0x00E8;
pub const R_BCM1480_IMR_MAILBOX_1_CLR_CPU: u64 = 0x00F0;
pub const R_BCM1480_IMR_INTERRUPT_STATUS_BASE_H: u64 = 0x0100;
pub const BCM1480_IMR_INTERRUPT_STATUS_COUNT: u64 = 8;
pub const R_BCM1480_IMR_INTERRUPT_MAP_BASE_H: u64 = 0x0200;
pub const BCM1480_IMR_INTERRUPT_MAP_COUNT: u64 = 64;

pub const R_BCM1480_IMR_INTERRUPT_DIAG_L: u64 = 0x1010;
pub const R_BCM1480_IMR_LDT_INTERRUPT_L: u64 = 0x1018;
pub const R_BCM1480_IMR_LDT_INTERRUPT_CLR_L: u64 = 0x1020;
pub const R_BCM1480_IMR_INTERRUPT_MASK_L: u64 = 0x1028;
pub const R_BCM1480_IMR_INTERRUPT_TRACE_L: u64 = 0x1038;
pub const R_BCM1480_IMR_INTERRUPT_SOURCE_STATUS_L: u64 = 0x1040;
pub const R_BCM1480_IMR_INTERRUPT_STATUS_BASE_L: u64 = 0x1100;
pub const R_BCM1480_IMR_INTERRUPT_MAP_BASE_L: u64 = 0x1200;

pub const A_BCM1480_IMR_ALIAS_MAILBOX_CPU0_BASE: u64 = 0x0010028000;
pub const A_BCM1480_IMR_ALIAS_MAILBOX_CPU1_BASE: u64 = 0x0010028100;
pub const A_BCM1480_IMR_ALIAS_MAILBOX_CPU2_BASE: u64 = 0x0010028200;
pub const A_BCM1480_IMR_ALIAS_MAILBOX_CPU3_BASE: u64 = 0x0010028300;
pub const BCM1480_IMR_ALIAS_MAILBOX_SPACING: u64 = 0100;

#[inline]
pub const fn A_BCM1480_IMR_ALIAS_MAILBOX(cpu: u64) -> u64 { (A_BCM1480_IMR_ALIAS_MAILBOX_CPU0_BASE + \ }
					(cpu)*BCM1480_IMR_ALIAS_MAILBOX_SPACING)
#[inline]
pub const fn A_BCM1480_IMR_ALIAS_MAILBOX_REGISTER(cpu, : u64) -> u64 { reg) (A_BCM1480_IMR_ALIAS_MAILBOX(cpu)+(reg)) }

pub const R_BCM1480_IMR_ALIAS_MAILBOX_0: u64 = 0x0000;
pub const R_BCM1480_IMR_ALIAS_MAILBOX_0_SET: u64 = 0x0008;

/*
 * these macros work together to build the address of a mailbox
 * register, e.g., A_BCM1480_MAILBOX_REGISTER(0,R_BCM1480_IMR_MAILBOX_SET,2)
 * for mbox_0_set_cpu2 returns 0x00100240C8
 */
pub const R_BCM1480_IMR_MAILBOX_CPU: u64 = 0x00;
pub const R_BCM1480_IMR_MAILBOX_SET: u64 = 0x08;
pub const R_BCM1480_IMR_MAILBOX_CLR: u64 = 0x10;
pub const R_BCM1480_IMR_MAILBOX_NUM_SPACING: u64 = 0x20;
#[inline]
pub const fn A_BCM1480_MAILBOX_REGISTER(num, : u64) -> u64 { reg, cpu) \ }
    (A_BCM1480_IMR_CPU0_BASE + \
     (num * R_BCM1480_IMR_MAILBOX_NUM_SPACING) + \
     (cpu * BCM1480_IMR_REGISTER_SPACING) + \
     (R_BCM1480_IMR_MAILBOX_0_CPU + reg))

/*  *********************************************************************
    * System Performance Counter Registers (Section 4.7)
    ********************************************************************* */

/* BCM1480 has four more performance counter registers, and two control
   registers. */

pub const A_BCM1480_SCD_PERF_CNT_BASE: u64 = 0x00100204C0;

pub const A_BCM1480_SCD_PERF_CNT_CFG0: u64 = 0x00100204C0;
pub const A_BCM1480_SCD_PERF_CNT_CFG_0: u64 = A_BCM1480_SCD_PERF_CNT_CFG0;
pub const A_BCM1480_SCD_PERF_CNT_CFG1: u64 = 0x00100204C8;
pub const A_BCM1480_SCD_PERF_CNT_CFG_1: u64 = A_BCM1480_SCD_PERF_CNT_CFG1;

pub const A_BCM1480_SCD_PERF_CNT_0: u64 = A_SCD_PERF_CNT_0;
pub const A_BCM1480_SCD_PERF_CNT_1: u64 = A_SCD_PERF_CNT_1;
pub const A_BCM1480_SCD_PERF_CNT_2: u64 = A_SCD_PERF_CNT_2;
pub const A_BCM1480_SCD_PERF_CNT_3: u64 = A_SCD_PERF_CNT_3;

pub const A_BCM1480_SCD_PERF_CNT_4: u64 = 0x00100204F0;
pub const A_BCM1480_SCD_PERF_CNT_5: u64 = 0x00100204F8;
pub const A_BCM1480_SCD_PERF_CNT_6: u64 = 0x0010020500;
pub const A_BCM1480_SCD_PERF_CNT_7: u64 = 0x0010020508;

pub const BCM1480_SCD_NUM_PERF_CNT: u64 = 8;
pub const BCM1480_SCD_PERF_CNT_SPACING: u64 = 8;
#[inline]
pub const fn A_BCM1480_SCD_PERF_CNT(n: u64) -> u64 { (A_SCD_PERF_CNT_0+(n*BCM1480_SCD_PERF_CNT_SPACING)) }

/*  *********************************************************************
    * System Bus Watcher Registers (Section 4.8)
    ********************************************************************* */


/* Same as 1250 except BUS_ERR_STATUS_DEBUG is in a different place. */

#define A_BCM1480_BUS_ERR_STATUS_DEBUG	    0x00100208D8

/*  *********************************************************************
    * System Debug Controller Registers (Section 19)
    ********************************************************************* */

/* Same as 1250 */

/*  *********************************************************************
    * System Trace Unit Registers (Sections 4.10)
    ********************************************************************* */

/* Same as 1250 */

/*  *********************************************************************
    * Data Mover DMA Registers (Section 10.7)
    ********************************************************************* */

/* Same as 1250 */


/*  *********************************************************************
    * HyperTransport Interface Registers (Section 8)
    ********************************************************************* */

pub const BCM1480_HT_NUM_PORTS: u64 = 3;
pub const BCM1480_HT_PORT_SPACING: u64 = 0x800;
#[inline]
pub const fn A_BCM1480_HT_PORT_HEADER(x: u64) -> u64 { (A_BCM1480_HT_PORT0_HEADER + ((x)*BCM1480_HT_PORT_SPACING)) }

pub const A_BCM1480_HT_PORT0_HEADER: u64 = 0x00FE000000;
pub const A_BCM1480_HT_PORT1_HEADER: u64 = 0x00FE000800;
pub const A_BCM1480_HT_PORT2_HEADER: u64 = 0x00FE001000;
pub const A_BCM1480_HT_TYPE00_HEADER: u64 = 0x00FE002000;


/*  *********************************************************************
    * Node Controller Registers (Section 9)
    ********************************************************************* */

pub const A_BCM1480_NC_BASE: u64 = 0x00DFBD0000;

pub const A_BCM1480_NC_RLD_FIELD: u64 = 0x00DFBD0000;
pub const A_BCM1480_NC_RLD_TRIGGER: u64 = 0x00DFBD0020;
pub const A_BCM1480_NC_RLD_BAD_ERROR: u64 = 0x00DFBD0040;
pub const A_BCM1480_NC_RLD_COR_ERROR: u64 = 0x00DFBD0060;
pub const A_BCM1480_NC_RLD_ECC_STATUS: u64 = 0x00DFBD0080;
pub const A_BCM1480_NC_RLD_WAY_ENABLE: u64 = 0x00DFBD00A0;
pub const A_BCM1480_NC_RLD_RANDOM_LFSR: u64 = 0x00DFBD00C0;

pub const A_BCM1480_NC_INTERRUPT_STATUS: u64 = 0x00DFBD00E0;
pub const A_BCM1480_NC_INTERRUPT_ENABLE: u64 = 0x00DFBD0100;
pub const A_BCM1480_NC_TIMEOUT_COUNTER: u64 = 0x00DFBD0120;
pub const A_BCM1480_NC_TIMEOUT_COUNTER_SEL: u64 = 0x00DFBD0140;

pub const A_BCM1480_NC_CREDIT_STATUS_REG0: u64 = 0x00DFBD0200;
pub const A_BCM1480_NC_CREDIT_STATUS_REG1: u64 = 0x00DFBD0220;
pub const A_BCM1480_NC_CREDIT_STATUS_REG2: u64 = 0x00DFBD0240;
pub const A_BCM1480_NC_CREDIT_STATUS_REG3: u64 = 0x00DFBD0260;
pub const A_BCM1480_NC_CREDIT_STATUS_REG4: u64 = 0x00DFBD0280;
pub const A_BCM1480_NC_CREDIT_STATUS_REG5: u64 = 0x00DFBD02A0;
pub const A_BCM1480_NC_CREDIT_STATUS_REG6: u64 = 0x00DFBD02C0;
pub const A_BCM1480_NC_CREDIT_STATUS_REG7: u64 = 0x00DFBD02E0;
pub const A_BCM1480_NC_CREDIT_STATUS_REG8: u64 = 0x00DFBD0300;
pub const A_BCM1480_NC_CREDIT_STATUS_REG9: u64 = 0x00DFBD0320;
pub const A_BCM1480_NC_CREDIT_STATUS_REG10: u64 = 0x00DFBE0000;
pub const A_BCM1480_NC_CREDIT_STATUS_REG11: u64 = 0x00DFBE0020;
pub const A_BCM1480_NC_CREDIT_STATUS_REG12: u64 = 0x00DFBE0040;

pub const A_BCM1480_NC_SR_TIMEOUT_COUNTER: u64 = 0x00DFBE0060;
pub const A_BCM1480_NC_SR_TIMEOUT_COUNTER_SEL: u64 = 0x00DFBE0080;


/*  *********************************************************************
    * H&R Block Configuration Registers (Section 12.4)
    ********************************************************************* */

pub const A_BCM1480_HR_BASE_0: u64 = 0x00DF820000;
pub const A_BCM1480_HR_BASE_1: u64 = 0x00DF8A0000;
pub const A_BCM1480_HR_BASE_2: u64 = 0x00DF920000;
pub const BCM1480_HR_REGISTER_SPACING: u64 = 0x80000;

#[inline]
pub const fn A_BCM1480_HR_BASE(idx: u64) -> u64 { (A_BCM1480_HR_BASE_0 + ((idx)*BCM1480_HR_REGISTER_SPACING)) }
#[inline]
pub const fn A_BCM1480_HR_REGISTER(idx, : u64) -> u64 { reg)	     (A_BCM1480_HR_BASE(idx) + (reg)) }

pub const R_BCM1480_HR_CFG: u64 = 0x0000000000;

pub const R_BCM1480_HR_MAPPING: u64 = 0x0000010010;

pub const BCM1480_HR_RULE_SPACING: u64 = 0x0000000010;
pub const BCM1480_HR_NUM_RULES: u64 = 16;
pub const BCM1480_HR_OP_OFFSET: u64 = 0x0000000100;
pub const BCM1480_HR_TYPE_OFFSET: u64 = 0x0000000108;
#[inline]
pub const fn R_BCM1480_HR_RULE_OP(idx: u64) -> u64 { (BCM1480_HR_OP_OFFSET + ((idx)*BCM1480_HR_RULE_SPACING)) }
#[inline]
pub const fn R_BCM1480_HR_RULE_TYPE(idx: u64) -> u64 { (BCM1480_HR_TYPE_OFFSET + ((idx)*BCM1480_HR_RULE_SPACING)) }

pub const BCM1480_HR_LEAF_SPACING: u64 = 0x0000000010;
pub const BCM1480_HR_NUM_LEAVES: u64 = 10;
pub const BCM1480_HR_LEAF_OFFSET: u64 = 0x0000000300;
#[inline]
pub const fn R_BCM1480_HR_HA_LEAF0(idx: u64) -> u64 { (BCM1480_HR_LEAF_OFFSET + ((idx)*BCM1480_HR_LEAF_SPACING)) }

pub const R_BCM1480_HR_EX_LEAF0: u64 = 0x00000003A0;

pub const BCM1480_HR_PATH_SPACING: u64 = 0x0000000010;
pub const BCM1480_HR_NUM_PATHS: u64 = 16;
pub const BCM1480_HR_PATH_OFFSET: u64 = 0x0000000600;
#[inline]
pub const fn R_BCM1480_HR_PATH(idx: u64) -> u64 { (BCM1480_HR_PATH_OFFSET + ((idx)*BCM1480_HR_PATH_SPACING)) }

pub const R_BCM1480_HR_PATH_DEFAULT: u64 = 0x0000000700;

pub const BCM1480_HR_ROUTE_SPACING: u64 = 8;
pub const BCM1480_HR_NUM_ROUTES: u64 = 512;
pub const BCM1480_HR_ROUTE_OFFSET: u64 = 0x0000001000;
#[inline]
pub const fn R_BCM1480_HR_RT_WORD(idx: u64) -> u64 { (BCM1480_HR_ROUTE_OFFSET + ((idx)*BCM1480_HR_ROUTE_SPACING)) }


/* checked to here - ehs */
/*  *********************************************************************
    * Packet Manager DMA Registers (Section 12.5)
    ********************************************************************* */

pub const A_BCM1480_PM_BASE: u64 = 0x0010056000;

pub const A_BCM1480_PMI_LCL_0: u64 = 0x0010058000;
pub const A_BCM1480_PMO_LCL_0: u64 = 0x001005C000;
pub const A_BCM1480_PMI_OFFSET_0: u64 = (A_BCM1480_PMI_LCL_0 - A_BCM1480_PM_BASE);
pub const A_BCM1480_PMO_OFFSET_0: u64 = (A_BCM1480_PMO_LCL_0 - A_BCM1480_PM_BASE);

pub const BCM1480_PM_LCL_REGISTER_SPACING: u64 = 0x100;
pub const BCM1480_PM_NUM_CHANNELS: u64 = 32;

#[inline]
pub const fn A_BCM1480_PMI_LCL_BASE(idx: u64) -> u64 { (A_BCM1480_PMI_LCL_0 + ((idx)*BCM1480_PM_LCL_REGISTER_SPACING)) }
#[inline]
pub const fn A_BCM1480_PMI_LCL_REGISTER(idx, : u64) -> u64 { reg)	 (A_BCM1480_PMI_LCL_BASE(idx) + (reg)) }
#[inline]
pub const fn A_BCM1480_PMO_LCL_BASE(idx: u64) -> u64 { (A_BCM1480_PMO_LCL_0 + ((idx)*BCM1480_PM_LCL_REGISTER_SPACING)) }
#[inline]
pub const fn A_BCM1480_PMO_LCL_REGISTER(idx, : u64) -> u64 { reg)	 (A_BCM1480_PMO_LCL_BASE(idx) + (reg)) }

pub const BCM1480_PM_INT_PACKING: u64 = 8;
pub const BCM1480_PM_INT_FUNCTION_SPACING: u64 = 0x40;
pub const BCM1480_PM_INT_NUM_FUNCTIONS: u64 = 3;

/*
 * DMA channel registers relative to A_BCM1480_PMI_LCL_BASE(n) and A_BCM1480_PMO_LCL_BASE(n)
 */

pub const R_BCM1480_PM_BASE_SIZE: u64 = 0x0000000000;
pub const R_BCM1480_PM_CNT: u64 = 0x0000000008;
pub const R_BCM1480_PM_PFCNT: u64 = 0x0000000010;
pub const R_BCM1480_PM_LAST: u64 = 0x0000000018;
pub const R_BCM1480_PM_PFINDX: u64 = 0x0000000020;
pub const R_BCM1480_PM_INT_WMK: u64 = 0x0000000028;
pub const R_BCM1480_PM_CONFIG0: u64 = 0x0000000030;
pub const R_BCM1480_PM_LOCALDEBUG: u64 = 0x0000000078;
pub const R_BCM1480_PM_CACHEABILITY: u64 = 0x0000000080   /* PMI only */;
pub const R_BCM1480_PM_INT_CNFG: u64 = 0x0000000088;
pub const R_BCM1480_PM_DESC_MERGE_TIMER: u64 = 0x0000000090;
pub const R_BCM1480_PM_LOCALDEBUG_PIB: u64 = 0x00000000F8   /* PMI only */;
pub const R_BCM1480_PM_LOCALDEBUG_POB: u64 = 0x00000000F8   /* PMO only */;

/*
 * Global Registers (Not Channelized)
 */

pub const A_BCM1480_PMI_GLB_0: u64 = 0x0010056000;
pub const A_BCM1480_PMO_GLB_0: u64 = 0x0010057000;

/*
 * PM to TX Mapping Register relative to A_BCM1480_PMI_GLB_0 and A_BCM1480_PMO_GLB_0
 */

pub const R_BCM1480_PM_PMO_MAPPING: u64 = 0x00000008C8   /* PMO only */;

pub const A_BCM1480_PM_PMO_MAPPING: u64 = (A_BCM1480_PMO_GLB_0 + R_BCM1480_PM_PMO_MAPPING);

/*
 * Interrupt mapping registers
 */


pub const A_BCM1480_PMI_INT_0: u64 = 0x0010056800;
#[inline]
pub const fn A_BCM1480_PMI_INT(q: u64) -> u64 { (A_BCM1480_PMI_INT_0 + ((q>>8)<<8)) }
pub const A_BCM1480_PMI_INT_OFFSET_0: u64 = (A_BCM1480_PMI_INT_0 - A_BCM1480_PM_BASE);
pub const A_BCM1480_PMO_INT_0: u64 = 0x0010057800;
#[inline]
pub const fn A_BCM1480_PMO_INT(q: u64) -> u64 { (A_BCM1480_PMO_INT_0 + ((q>>8)<<8)) }
pub const A_BCM1480_PMO_INT_OFFSET_0: u64 = (A_BCM1480_PMO_INT_0 - A_BCM1480_PM_BASE);

/*
 * Interrupt registers relative to A_BCM1480_PMI_INT_0 and A_BCM1480_PMO_INT_0
 */

pub const R_BCM1480_PM_INT_ST: u64 = 0x0000000000;
pub const R_BCM1480_PM_INT_MSK: u64 = 0x0000000040;
pub const R_BCM1480_PM_INT_CLR: u64 = 0x0000000080;
pub const R_BCM1480_PM_MRGD_INT: u64 = 0x00000000C0;

/*
 * Debug registers (global)
 */

pub const A_BCM1480_PM_GLOBALDEBUGMODE_PMI: u64 = 0x0010056000;
pub const A_BCM1480_PM_GLOBALDEBUG_PID: u64 = 0x00100567F8;
pub const A_BCM1480_PM_GLOBALDEBUG_PIB: u64 = 0x0010056FF8;
pub const A_BCM1480_PM_GLOBALDEBUGMODE_PMO: u64 = 0x0010057000;
pub const A_BCM1480_PM_GLOBALDEBUG_POD: u64 = 0x00100577F8;
pub const A_BCM1480_PM_GLOBALDEBUG_POB: u64 = 0x0010057FF8;

/*  *********************************************************************
    *  Switch performance counters
    ********************************************************************* */

pub const A_BCM1480_SWPERF_CFG: u64 = 0xdfb91800;
pub const A_BCM1480_SWPERF_CNT0: u64 = 0xdfb91880;
pub const A_BCM1480_SWPERF_CNT1: u64 = 0xdfb91888;
pub const A_BCM1480_SWPERF_CNT2: u64 = 0xdfb91890;
pub const A_BCM1480_SWPERF_CNT3: u64 = 0xdfb91898;


/*  *********************************************************************
    *  Switch Trace Unit
    ********************************************************************* */

pub const A_BCM1480_SWTRC_MATCH_CONTROL_0: u64 = 0xDFB91000;
pub const A_BCM1480_SWTRC_MATCH_DATA_VALUE_0: u64 = 0xDFB91100;
pub const A_BCM1480_SWTRC_MATCH_DATA_MASK_0: u64 = 0xDFB91108;
pub const A_BCM1480_SWTRC_MATCH_TAG_VALUE_0: u64 = 0xDFB91200;
pub const A_BCM1480_SWTRC_MATCH_TAG_MAKS_0: u64 = 0xDFB91208;
pub const A_BCM1480_SWTRC_EVENT_0: u64 = 0xDFB91300;
pub const A_BCM1480_SWTRC_SEQUENCE_0: u64 = 0xDFB91400;

pub const A_BCM1480_SWTRC_CFG: u64 = 0xDFB91500;
pub const A_BCM1480_SWTRC_READ: u64 = 0xDFB91508;

pub const A_BCM1480_SWDEBUG_SCHEDSTOP: u64 = 0xDFB92000;

#[inline]
pub const fn A_BCM1480_SWTRC_MATCH_CONTROL(x: u64) -> u64 { (A_BCM1480_SWTRC_MATCH_CONTROL_0 + ((x)*8)) }
#[inline]
pub const fn A_BCM1480_SWTRC_EVENT(x: u64) -> u64 { (A_BCM1480_SWTRC_EVENT_0 + ((x)*8)) }
#[inline]
pub const fn A_BCM1480_SWTRC_SEQUENCE(x: u64) -> u64 { (A_BCM1480_SWTRC_SEQUENCE_0 + ((x)*8)) }

#[inline]
pub const fn A_BCM1480_SWTRC_MATCH_DATA_VALUE(x: u64) -> u64 { (A_BCM1480_SWTRC_MATCH_DATA_VALUE_0 + ((x)*16)) }
#[inline]
pub const fn A_BCM1480_SWTRC_MATCH_DATA_MASK(x: u64) -> u64 { (A_BCM1480_SWTRC_MATCH_DATA_MASK_0 + ((x)*16)) }
#[inline]
pub const fn A_BCM1480_SWTRC_MATCH_TAG_VALUE(x: u64) -> u64 { (A_BCM1480_SWTRC_MATCH_TAG_VALUE_0 + ((x)*16)) }
#[inline]
pub const fn A_BCM1480_SWTRC_MATCH_TAG_MASK(x: u64) -> u64 { (A_BCM1480_SWTRC_MATCH_TAG_MASK_0 + ((x)*16)) }



/*  *********************************************************************
    *  High-Speed Port Registers (Section 13)
    ********************************************************************* */

pub const A_BCM1480_HSP_BASE_0: u64 = 0x00DF810000;
pub const A_BCM1480_HSP_BASE_1: u64 = 0x00DF890000;
pub const A_BCM1480_HSP_BASE_2: u64 = 0x00DF910000;
pub const BCM1480_HSP_REGISTER_SPACING: u64 = 0x80000;

#[inline]
pub const fn A_BCM1480_HSP_BASE(idx: u64) -> u64 { (A_BCM1480_HSP_BASE_0 + ((idx)*BCM1480_HSP_REGISTER_SPACING)) }
#[inline]
pub const fn A_BCM1480_HSP_REGISTER(idx, : u64) -> u64 { reg)     (A_BCM1480_HSP_BASE(idx) + (reg)) }

pub const R_BCM1480_HSP_RX_SPI4_CFG_0: u64 = 0x0000000000;
pub const R_BCM1480_HSP_RX_SPI4_CFG_1: u64 = 0x0000000008;
pub const R_BCM1480_HSP_RX_SPI4_DESKEW_OVERRIDE: u64 = 0x0000000010;
pub const R_BCM1480_HSP_RX_SPI4_DESKEW_DATAPATH: u64 = 0x0000000018;
pub const R_BCM1480_HSP_RX_SPI4_PORT_INT_EN: u64 = 0x0000000020;
pub const R_BCM1480_HSP_RX_SPI4_PORT_INT_STATUS: u64 = 0x0000000028;

pub const R_BCM1480_HSP_RX_SPI4_CALENDAR_0: u64 = 0x0000000200;
pub const R_BCM1480_HSP_RX_SPI4_CALENDAR_1: u64 = 0x0000000208;

pub const R_BCM1480_HSP_RX_PLL_CNFG: u64 = 0x0000000800;
pub const R_BCM1480_HSP_RX_CALIBRATION: u64 = 0x0000000808;
pub const R_BCM1480_HSP_RX_TEST: u64 = 0x0000000810;
pub const R_BCM1480_HSP_RX_DIAG_DETAILS: u64 = 0x0000000818;
pub const R_BCM1480_HSP_RX_DIAG_CRC_0: u64 = 0x0000000820;
pub const R_BCM1480_HSP_RX_DIAG_CRC_1: u64 = 0x0000000828;
pub const R_BCM1480_HSP_RX_DIAG_HTCMD: u64 = 0x0000000830;
pub const R_BCM1480_HSP_RX_DIAG_PKTCTL: u64 = 0x0000000838;

pub const R_BCM1480_HSP_RX_VIS_FLCTRL_COUNTER: u64 = 0x0000000870;

pub const R_BCM1480_HSP_RX_PKT_RAMALLOC_0: u64 = 0x0000020020;
pub const R_BCM1480_HSP_RX_PKT_RAMALLOC_1: u64 = 0x0000020028;
pub const R_BCM1480_HSP_RX_PKT_RAMALLOC_2: u64 = 0x0000020030;
pub const R_BCM1480_HSP_RX_PKT_RAMALLOC_3: u64 = 0x0000020038;
pub const R_BCM1480_HSP_RX_PKT_RAMALLOC_4: u64 = 0x0000020040;
pub const R_BCM1480_HSP_RX_PKT_RAMALLOC_5: u64 = 0x0000020048;
pub const R_BCM1480_HSP_RX_PKT_RAMALLOC_6: u64 = 0x0000020050;
pub const R_BCM1480_HSP_RX_PKT_RAMALLOC_7: u64 = 0x0000020058;
#[inline]
pub const fn R_BCM1480_HSP_RX_PKT_RAMALLOC(idx: u64) -> u64 { (R_BCM1480_HSP_RX_PKT_RAMALLOC_0 + 8*(idx)) }

/* XXX Following registers were shuffled.  Renamed/renumbered per errata. */
#define R_BCM1480_HSP_RX_HT_RAMALLOC_0	    0x0000020078
#define R_BCM1480_HSP_RX_HT_RAMALLOC_1	    0x0000020080
#define R_BCM1480_HSP_RX_HT_RAMALLOC_2	    0x0000020088
#define R_BCM1480_HSP_RX_HT_RAMALLOC_3	    0x0000020090
#define R_BCM1480_HSP_RX_HT_RAMALLOC_4	    0x0000020098
#define R_BCM1480_HSP_RX_HT_RAMALLOC_5	    0x00000200A0

#define R_BCM1480_HSP_RX_SPI_WATERMARK_0      0x00000200B0
#define R_BCM1480_HSP_RX_SPI_WATERMARK_1      0x00000200B8
#define R_BCM1480_HSP_RX_SPI_WATERMARK_2      0x00000200C0
#define R_BCM1480_HSP_RX_SPI_WATERMARK_3      0x00000200C8
#define R_BCM1480_HSP_RX_SPI_WATERMARK_4      0x00000200D0
#define R_BCM1480_HSP_RX_SPI_WATERMARK_5      0x00000200D8
#define R_BCM1480_HSP_RX_SPI_WATERMARK_6      0x00000200E0
#define R_BCM1480_HSP_RX_SPI_WATERMARK_7      0x00000200E8
#define R_BCM1480_HSP_RX_SPI_WATERMARK(idx)   (R_BCM1480_HSP_RX_SPI_WATERMARK_0 + 8*(idx))

#define R_BCM1480_HSP_RX_VIS_CMDQ_0	      0x00000200F0
#define R_BCM1480_HSP_RX_VIS_CMDQ_1	      0x00000200F8
#define R_BCM1480_HSP_RX_VIS_CMDQ_2	      0x0000020100
#define R_BCM1480_HSP_RX_RAM_READCTL	      0x0000020108
#define R_BCM1480_HSP_RX_RAM_READWINDOW	      0x0000020110
#define R_BCM1480_HSP_RX_RF_READCTL	      0x0000020118
#define R_BCM1480_HSP_RX_RF_READWINDOW	      0x0000020120

#define R_BCM1480_HSP_TX_SPI4_CFG_0	      0x0000040000
#define R_BCM1480_HSP_TX_SPI4_CFG_1	      0x0000040008
#define R_BCM1480_HSP_TX_SPI4_TRAINING_FMT    0x0000040010

#define R_BCM1480_HSP_TX_PKT_RAMALLOC_0	      0x0000040020
#define R_BCM1480_HSP_TX_PKT_RAMALLOC_1	      0x0000040028
#define R_BCM1480_HSP_TX_PKT_RAMALLOC_2	      0x0000040030
#define R_BCM1480_HSP_TX_PKT_RAMALLOC_3	      0x0000040038
#define R_BCM1480_HSP_TX_PKT_RAMALLOC_4	      0x0000040040
#define R_BCM1480_HSP_TX_PKT_RAMALLOC_5	      0x0000040048
#define R_BCM1480_HSP_TX_PKT_RAMALLOC_6	      0x0000040050
#define R_BCM1480_HSP_TX_PKT_RAMALLOC_7	      0x0000040058
#define R_BCM1480_HSP_TX_PKT_RAMALLOC(idx)    (R_BCM1480_HSP_TX_PKT_RAMALLOC_0 + 8*(idx))
#define R_BCM1480_HSP_TX_NPC_RAMALLOC	      0x0000040078
#define R_BCM1480_HSP_TX_RSP_RAMALLOC	      0x0000040080
#define R_BCM1480_HSP_TX_PC_RAMALLOC	      0x0000040088
#define R_BCM1480_HSP_TX_HTCC_RAMALLOC_0      0x0000040090
#define R_BCM1480_HSP_TX_HTCC_RAMALLOC_1      0x0000040098
#define R_BCM1480_HSP_TX_HTCC_RAMALLOC_2      0x00000400A0

#define R_BCM1480_HSP_TX_PKT_RXPHITCNT_0      0x00000400B0
#define R_BCM1480_HSP_TX_PKT_RXPHITCNT_1      0x00000400B8
#define R_BCM1480_HSP_TX_PKT_RXPHITCNT_2      0x00000400C0
#define R_BCM1480_HSP_TX_PKT_RXPHITCNT_3      0x00000400C8
#define R_BCM1480_HSP_TX_PKT_RXPHITCNT(idx)   (R_BCM1480_HSP_TX_PKT_RXPHITCNT_0 + 8*(idx))
#define R_BCM1480_HSP_TX_HTIO_RXPHITCNT	      0x00000400D0
#define R_BCM1480_HSP_TX_HTCC_RXPHITCNT	      0x00000400D8

#define R_BCM1480_HSP_TX_PKT_TXPHITCNT_0      0x00000400E0
#define R_BCM1480_HSP_TX_PKT_TXPHITCNT_1      0x00000400E8
#define R_BCM1480_HSP_TX_PKT_TXPHITCNT_2      0x00000400F0
#define R_BCM1480_HSP_TX_PKT_TXPHITCNT_3      0x00000400F8
#define R_BCM1480_HSP_TX_PKT_TXPHITCNT(idx)   (R_BCM1480_HSP_TX_PKT_TXPHITCNT_0 + 8*(idx))
#define R_BCM1480_HSP_TX_HTIO_TXPHITCNT	      0x0000040100
#define R_BCM1480_HSP_TX_HTCC_TXPHITCNT	      0x0000040108

#define R_BCM1480_HSP_TX_SPI4_CALENDAR_0      0x0000040200
#define R_BCM1480_HSP_TX_SPI4_CALENDAR_1      0x0000040208

#define R_BCM1480_HSP_TX_PLL_CNFG	      0x0000040800
#define R_BCM1480_HSP_TX_CALIBRATION	      0x0000040808
#define R_BCM1480_HSP_TX_TEST		      0x0000040810

#define R_BCM1480_HSP_TX_VIS_CMDQ_0	      0x0000040840
#define R_BCM1480_HSP_TX_VIS_CMDQ_1	      0x0000040848
#define R_BCM1480_HSP_TX_VIS_CMDQ_2	      0x0000040850
#define R_BCM1480_HSP_TX_RAM_READCTL	      0x0000040860
#define R_BCM1480_HSP_TX_RAM_READWINDOW	      0x0000040868
#define R_BCM1480_HSP_TX_RF_READCTL	      0x0000040870
#define R_BCM1480_HSP_TX_RF_READWINDOW	      0x0000040878

#define R_BCM1480_HSP_TX_SPI4_PORT_INT_STATUS 0x0000040880
#define R_BCM1480_HSP_TX_SPI4_PORT_INT_EN     0x0000040888

#define R_BCM1480_HSP_TX_NEXT_ADDR_BASE 0x000040400
#define R_BCM1480_HSP_TX_NEXT_ADDR_REGISTER(x)	(R_BCM1480_HSP_TX_NEXT_ADDR_BASE+ 8*(x))



/*  *********************************************************************
    *  Physical Address Map (Table 10 and Figure 7)
    ********************************************************************* */

pub const A_BCM1480_PHYS_MEMORY_0: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_MEMORY_SIZE: u64 = _SB_MAKE64((256*1024*1024));
pub const A_BCM1480_PHYS_SYSTEM_CTL: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_IO_SYSTEM: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_GENBUS: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_GENBUS_END: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_PCI_MISC_MATCH_BYTES: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_PCI_IACK_MATCH_BYTES: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_PCI_IO_MATCH_BYTES: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_PCI_CFG_MATCH_BYTES: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_PCI_OMAP_MATCH_BYTES: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_PCI_MEM_MATCH_BYTES: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_HT_MEM_MATCH_BYTES: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_HT_MEM_MATCH_BITS: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_MEMORY_1: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_MEMORY_2: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_PCI_MISC_MATCH_BITS: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_PCI_IACK_MATCH_BITS: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_PCI_IO_MATCH_BITS: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_PCI_CFG_MATCH_BITS: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_PCI_OMAP_MATCH_BITS: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_PCI_MEM_MATCH_BITS: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_MEMORY_3: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_L2_CACHE_TEST: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_HT_SPECIAL_MATCH_BYTES: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_HT_IO_MATCH_BYTES: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_HT_CFG_MATCH_BYTES: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_HS_SUBSYS: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_HT_SPECIAL_MATCH_BITS: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_HT_IO_MATCH_BITS: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_HT_CFG_MATCH_BITS: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_MEMORY_EXP: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_MEMORY_EXP_SIZE: u64 = _SB_MAKE64((508*1024*1024*1024));
pub const A_BCM1480_PHYS_PCI_UPPER: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_HT_UPPER_MATCH_BYTES: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_HT_UPPER_MATCH_BITS: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_HT_NODE_ALIAS: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_HT_FULLACCESS: u64 = ($1 as u64);


/*  *********************************************************************
    *  L2 Cache as RAM (Table 54)
    ********************************************************************* */

pub const A_BCM1480_PHYS_L2CACHE_WAY_SIZE: u64 = ($1 as u64);
pub const BCM1480_PHYS_L2CACHE_NUM_WAYS: u64 = 8;
pub const A_BCM1480_PHYS_L2CACHE_TOTAL_SIZE: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_L2CACHE_WAY0: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_L2CACHE_WAY1: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_L2CACHE_WAY2: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_L2CACHE_WAY3: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_L2CACHE_WAY4: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_L2CACHE_WAY5: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_L2CACHE_WAY6: u64 = ($1 as u64);
pub const A_BCM1480_PHYS_L2CACHE_WAY7: u64 = ($1 as u64);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
