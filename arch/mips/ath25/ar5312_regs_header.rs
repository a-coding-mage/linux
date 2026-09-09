/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2003 Atheros Communications, Inc.,  All Rights Reserved.
 * Copyright (C) 2006 Imre Kaloz <kaloz@openwrt.org>
 * Copyright (C) 2006 Felix Fietkau <nbd@openwrt.org>
 */

/* IRQs */
pub const AR5312_IRQ_WLAN0: u32 = MIPS_CPU_IRQ_BASE + 2; /* C0_CAUSE: 0x0400 */
pub const AR5312_IRQ_ENET0: u32 = MIPS_CPU_IRQ_BASE + 3; /* C0_CAUSE: 0x0800 */
pub const AR5312_IRQ_ENET1: u32 = MIPS_CPU_IRQ_BASE + 4; /* C0_CAUSE: 0x1000 */
pub const AR5312_IRQ_WLAN1: u32 = MIPS_CPU_IRQ_BASE + 5; /* C0_CAUSE: 0x2000 */
pub const AR5312_IRQ_MISC: u32 = MIPS_CPU_IRQ_BASE + 6; /* C0_CAUSE: 0x4000 */

/* Miscellaneous interrupts, which share IP6. */
pub const AR5312_MISC_IRQ_TIMER: u32 = 0;
pub const AR5312_MISC_IRQ_AHB_PROC: u32 = 1;
pub const AR5312_MISC_IRQ_AHB_DMA: u32 = 2;
pub const AR5312_MISC_IRQ_GPIO: u32 = 3;
pub const AR5312_MISC_IRQ_UART0: u32 = 4;
pub const AR5312_MISC_IRQ_UART0_DMA: u32 = 5;
pub const AR5312_MISC_IRQ_WATCHDOG: u32 = 6;
pub const AR5312_MISC_IRQ_LOCAL: u32 = 7;
pub const AR5312_MISC_IRQ_SPI: u32 = 8;
pub const AR5312_MISC_IRQ_COUNT: u32 = 9;

/* Address Map */
pub const AR5312_WLAN0_BASE: u32 = 0x18000000;
pub const AR5312_ENET0_BASE: u32 = 0x18100000;
pub const AR5312_ENET1_BASE: u32 = 0x18200000;
pub const AR5312_SDRAMCTL_BASE: u32 = 0x18300000;
pub const AR5312_SDRAMCTL_SIZE: u32 = 0x00000010;
pub const AR5312_FLASHCTL_BASE: u32 = 0x18400000;
pub const AR5312_FLASHCTL_SIZE: u32 = 0x00000010;
pub const AR5312_WLAN1_BASE: u32 = 0x18500000;
pub const AR5312_UART0_BASE: u32 = 0x1c000000; /* UART MMR */
pub const AR5312_GPIO_BASE: u32 = 0x1c002000;
pub const AR5312_GPIO_SIZE: u32 = 0x00000010;
pub const AR5312_RST_BASE: u32 = 0x1c003000;
pub const AR5312_RST_SIZE: u32 = 0x00000100;
pub const AR5312_FLASH_BASE: u32 = 0x1e000000;
pub const AR5312_FLASH_SIZE: u32 = 0x00800000;

/* Need these defines to determine true number of ethernet MACs */
pub const AR5312_AR5312_REV2: u32 = 0x0052; /* AR5312 WMAC (AP31) */
pub const AR5312_AR5312_REV7: u32 = 0x0057; /* AR5312 WMAC (AP30-040) */
pub const AR5312_AR2313_REV8: u32 = 0x0058; /* AR2313 WMAC (AP43-030) */

/* Reset/Timer Block Address Map */
pub const AR5312_TIMER: u32 = 0x0000; /* countdown timer */
pub const AR5312_RELOAD: u32 = 0x0004; /* timer reload value */
pub const AR5312_WDT_CTRL: u32 = 0x0008; /* watchdog cntrl */
pub const AR5312_WDT_TIMER: u32 = 0x000c; /* watchdog timer */
pub const AR5312_ISR: u32 = 0x0010; /* Intr Status Reg */
pub const AR5312_IMR: u32 = 0x0014; /* Intr Mask Reg */
pub const AR5312_RESET: u32 = 0x0020;
pub const AR5312_CLOCKCTL1: u32 = 0x0064;
pub const AR5312_SCRATCH: u32 = 0x006c;
pub const AR5312_PROCADDR: u32 = 0x0070;
pub const AR5312_PROC1: u32 = 0x0074;
pub const AR5312_DMAADDR: u32 = 0x0078;
pub const AR5312_DMA1: u32 = 0x007c;
pub const AR5312_ENABLE: u32 = 0x0080; /* interface enb */
pub const AR5312_REV: u32 = 0x0090; /* revision */

pub const AR5312_WDT_CTRL_IGNORE: u32 = 0x00000000; /* ignore expiration */
pub const AR5312_WDT_CTRL_NMI: u32 = 0x00000001;
pub const AR5312_WDT_CTRL_RESET: u32 = 0x00000002;

pub const AR5312_ISR_TIMER: u32 = 0x00000001;
pub const AR5312_ISR_AHBPROC: u32 = 0x00000002;
pub const AR5312_ISR_AHBDMA: u32 = 0x00000004;
pub const AR5312_ISR_GPIO: u32 = 0x00000008;
pub const AR5312_ISR_UART0: u32 = 0x00000010;
pub const AR5312_ISR_UART0DMA: u32 = 0x00000020;
pub const AR5312_ISR_WD: u32 = 0x00000040;
pub const AR5312_ISR_LOCAL: u32 = 0x00000080;

pub const AR5312_RESET_SYSTEM: u32 = 0x00000001; /* cold reset full system */
pub const AR5312_RESET_PROC: u32 = 0x00000002; /* cold reset MIPS core */
pub const AR5312_RESET_WLAN0: u32 = 0x00000004; /* cold reset WLAN MAC/BB */
pub const AR5312_RESET_EPHY0: u32 = 0x00000008; /* cold reset ENET0 phy */
pub const AR5312_RESET_EPHY1: u32 = 0x00000010; /* cold reset ENET1 phy */
pub const AR5312_RESET_ENET0: u32 = 0x00000020; /* cold reset ENET0 MAC */
pub const AR5312_RESET_ENET1: u32 = 0x00000040; /* cold reset ENET1 MAC */
pub const AR5312_RESET_UART0: u32 = 0x00000100; /* cold reset UART0 */
pub const AR5312_RESET_WLAN1: u32 = 0x00000200; /* cold reset WLAN MAC/BB */
pub const AR5312_RESET_APB: u32 = 0x00000400; /* cold reset APB ar5312 */
pub const AR5312_RESET_WARM_PROC: u32 = 0x00001000; /* warm reset MIPS core */
pub const AR5312_RESET_WARM_WLAN0_MAC: u32 = 0x00002000; /* warm reset WLAN0 MAC */
pub const AR5312_RESET_WARM_WLAN0_BB: u32 = 0x00004000; /* warm reset WLAN0 BB */
pub const AR5312_RESET_NMI: u32 = 0x00010000; /* send an NMI to the CPU */
pub const AR5312_RESET_WARM_WLAN1_MAC: u32 = 0x00020000; /* warm reset WLAN1 MAC */
pub const AR5312_RESET_WARM_WLAN1_BB: u32 = 0x00040000; /* warm reset WLAN1 BB */
pub const AR5312_RESET_LOCAL_BUS: u32 = 0x00080000; /* reset local bus */
pub const AR5312_RESET_WDOG: u32 = 0x00100000; /* last reset was a wdt */
pub const AR5312_RESET_WMAC0_BITS: u32 = AR5312_RESET_WLAN0 | AR5312_RESET_WARM_WLAN0_MAC | AR5312_RESET_WARM_WLAN0_BB;
pub const AR5312_RESET_WMAC1_BITS: u32 = AR5312_RESET_WLAN1 | AR5312_RESET_WARM_WLAN1_MAC | AR5312_RESET_WARM_WLAN1_BB;

pub const AR5312_CLOCKCTL1_PREDIVIDE_MASK: u32 = 0x00000030;
pub const AR5312_CLOCKCTL1_PREDIVIDE_SHIFT: u32 = 4;
pub const AR5312_CLOCKCTL1_MULTIPLIER_MASK: u32 = 0x00001f00;
pub const AR5312_CLOCKCTL1_MULTIPLIER_SHIFT: u32 = 8;
pub const AR5312_CLOCKCTL1_DOUBLER_MASK: u32 = 0x00010000;
/* Duplicate C definitions above are intentionally represented once in Rust. */

pub const AR2313_CLOCKCTL1_PREDIVIDE_MASK: u32 = 0x00003000;
pub const AR2313_CLOCKCTL1_PREDIVIDE_SHIFT: u32 = 12;
pub const AR2313_CLOCKCTL1_MULTIPLIER_MASK: u32 = 0x001f0000;
pub const AR2313_CLOCKCTL1_MULTIPLIER_SHIFT: u32 = 16;
pub const AR2313_CLOCKCTL1_DOUBLER_MASK: u32 = 0x00000000;

pub const AR5312_ENABLE_WLAN0: u32 = 0x00000001;
pub const AR5312_ENABLE_ENET0: u32 = 0x00000002;
pub const AR5312_ENABLE_ENET1: u32 = 0x00000004;
pub const AR5312_ENABLE_UART_AND_WLAN1_PIO: u32 = 0x00000008; /* UART & WLAN1 PIO */
pub const AR5312_ENABLE_WLAN1_DMA: u32 = 0x00000010; /* WLAN1 DMAs */
pub const AR5312_ENABLE_WLAN1: u32 = AR5312_ENABLE_UART_AND_WLAN1_PIO | AR5312_ENABLE_WLAN1_DMA;

pub const AR5312_REV_WMAC_MAJ: u32 = 0x0000f000;
pub const AR5312_REV_WMAC_MAJ_S: u32 = 12;
pub const AR5312_REV_WMAC_MIN: u32 = 0x00000f00;
pub const AR5312_REV_WMAC_MIN_S: u32 = 8;
pub const AR5312_REV_MAJ: u32 = 0x000000f0;
pub const AR5312_REV_MAJ_S: u32 = 4;
pub const AR5312_REV_MIN: u32 = 0x0000000f;
pub const AR5312_REV_MIN_S: u32 = 0;
pub const AR5312_REV_CHIP: u32 = AR5312_REV_MAJ | AR5312_REV_MIN;
pub const AR5312_REV_MAJ_AR5312: u32 = 0x4;
pub const AR5312_REV_MAJ_AR2313: u32 = 0x5;
pub const AR5312_REV_MIN_DUAL: u32 = 0x0; /* Dual WLAN version */
pub const AR5312_REV_MIN_SINGLE: u32 = 0x1; /* Single WLAN version */

/* ARM Flash Controller -- 3 flash banks with either x8 or x16 devices */
pub const AR5312_FLASHCTL0: u32 = 0x0000;
pub const AR5312_FLASHCTL1: u32 = 0x0004;
pub const AR5312_FLASHCTL2: u32 = 0x0008;
pub const AR5312_FLASHCTL_IDCY: u32 = 0x0000000f; /* Idle cycle turnaround time */
pub const AR5312_FLASHCTL_IDCY_S: u32 = 0;
pub const AR5312_FLASHCTL_WST1: u32 = 0x000003e0; /* Wait state 1 */
pub const AR5312_FLASHCTL_WST1_S: u32 = 5;
pub const AR5312_FLASHCTL_RBLE: u32 = 0x00000400; /* Read byte lane enable */
pub const AR5312_FLASHCTL_WST2: u32 = 0x0000f800; /* Wait state 2 */
pub const AR5312_FLASHCTL_WST2_S: u32 = 11;
pub const AR5312_FLASHCTL_AC: u32 = 0x00070000; /* Flash addr check (added) */
pub const AR5312_FLASHCTL_AC_S: u32 = 16;
pub const AR5312_FLASHCTL_AC_128K: u32 = 0x00000000;
pub const AR5312_FLASHCTL_AC_256K: u32 = 0x00010000;
pub const AR5312_FLASHCTL_AC_512K: u32 = 0x00020000;
pub const AR5312_FLASHCTL_AC_1M: u32 = 0x00030000;
pub const AR5312_FLASHCTL_AC_2M: u32 = 0x00040000;
pub const AR5312_FLASHCTL_AC_4M: u32 = 0x00050000;
pub const AR5312_FLASHCTL_AC_8M: u32 = 0x00060000;
pub const AR5312_FLASHCTL_AC_RES: u32 = 0x00070000; /* 16MB is not supported */
pub const AR5312_FLASHCTL_E: u32 = 0x00080000; /* Flash bank enable (added) */
pub const AR5312_FLASHCTL_BUSERR: u32 = 0x01000000; /* Bus transfer error flag */
pub const AR5312_FLASHCTL_WPERR: u32 = 0x02000000; /* Write protect error flag */
pub const AR5312_FLASHCTL_WP: u32 = 0x04000000; /* Write protect */
pub const AR5312_FLASHCTL_BM: u32 = 0x08000000; /* Burst mode */
pub const AR5312_FLASHCTL_MW: u32 = 0x30000000; /* Mem width */
pub const AR5312_FLASHCTL_MW8: u32 = 0x00000000; /* Mem width x8 */
pub const AR5312_FLASHCTL_MW16: u32 = 0x10000000; /* Mem width x16 */
pub const AR5312_FLASHCTL_MW32: u32 = 0x20000000; /* Mem width x32 (not supp) */
pub const AR5312_FLASHCTL_ATNR: u32 = 0x00000000; /* Access == no retry */
pub const AR5312_FLASHCTL_ATR: u32 = 0x80000000; /* Access == retry every */
pub const AR5312_FLASHCTL_ATR4: u32 = 0xc0000000; /* Access == retry every 4 */

/* ARM SDRAM Controller -- just enough to determine memory size */
pub const AR5312_MEM_CFG1: u32 = 0x0004;
pub const AR5312_MEM_CFG1_AC0_M: u32 = 0x00000700; /* bank 0: SDRAM addr check */
pub const AR5312_MEM_CFG1_AC0_S: u32 = 8;
pub const AR5312_MEM_CFG1_AC1_M: u32 = 0x00007000; /* bank 1: SDRAM addr check */
pub const AR5312_MEM_CFG1_AC1_S: u32 = 12;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
