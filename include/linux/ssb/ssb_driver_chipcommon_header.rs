/* SPDX-License-Identifier: GPL-2.0-only */
// FIXME: empty C macro preserved: #define LINUX_SSB_CHIPCO_H_
/* SonicsSiliconBackplane CHIPCOMMON core hardware definitions
 *
 * The chipcommon core provides chip identification, SB control,
 * jtag, 0/1/2 uarts, clock frequency control, a watchdog interrupt timer,
 * gpio interface, extbus, and support for serial and parallel flashes.
 *
 * Copyright 2005, Broadcom Corporation
 * Copyright 2006, Michael Buesch <m@bues.ch>
 */
/** ChipCommon core registers. **/
pub const SSB_CHIPCO_CHIPID		0x0000: u32 = ;
pub const SSB_CHIPCO_IDMASK		0x0000FFFF: u32 = ;
pub const SSB_CHIPCO_REVMASK		0x000F0000: u32 = ;
pub const SSB_CHIPCO_REVSHIFT		16: u32 = ;
pub const SSB_CHIPCO_PACKMASK		0x00F00000: u32 = ;
pub const SSB_CHIPCO_PACKSHIFT		20: u32 = ;
pub const SSB_CHIPCO_NRCORESMASK		0x0F000000: u32 = ;
pub const SSB_CHIPCO_NRCORESSHIFT	24: u32 = ;
pub const SSB_CHIPCO_CAP	 		0x0004		/* Capabilities */: u32 = ;
pub const SSB_CHIPCO_CAP_NRUART		0x00000003	/* # of UARTs */: u32 = ;
pub const SSB_CHIPCO_CAP_MIPSEB		0x00000004	/* MIPS in BigEndian Mode */: u32 = ;
pub const SSB_CHIPCO_CAP_UARTCLK		0x00000018	/* UART clock select */: u32 = ;
pub const SSB_CHIPCO_CAP_UARTCLK_INT	0x00000008	/* UARTs are driven by internal divided clock */: u32 = ;
pub const SSB_CHIPCO_CAP_UARTGPIO	0x00000020	/* UARTs on GPIO 15-12 */: u32 = ;
pub const SSB_CHIPCO_CAP_EXTBUS		0x000000C0	/* External buses present */: u32 = ;
pub const SSB_CHIPCO_CAP_FLASHT		0x00000700	/* Flash Type */: u32 = ;
pub const SSB_CHIPCO_FLASHT_NONE	0x00000000	/* No flash */: u32 = ;
pub const SSB_CHIPCO_FLASHT_STSER	0x00000100	/* ST serial flash */: u32 = ;
pub const SSB_CHIPCO_FLASHT_ATSER	0x00000200	/* Atmel serial flash */: u32 = ;
pub const SSB_CHIPCO_FLASHT_PARA	0x00000700	/* Parallel flash */: u32 = ;
pub const SSB_CHIPCO_CAP_PLLT		0x00038000	/* PLL Type */: u32 = ;
pub const SSB_PLLTYPE_NONE		0x00000000: u32 = ;
pub const SSB_PLLTYPE_1			0x00010000	/* 48Mhz base, 3 dividers */: u32 = ;
pub const SSB_PLLTYPE_2			0x00020000	/* 48Mhz, 4 dividers */: u32 = ;
pub const SSB_PLLTYPE_3			0x00030000	/* 25Mhz, 2 dividers */: u32 = ;
pub const SSB_PLLTYPE_4			0x00008000	/* 48Mhz, 4 dividers */: u32 = ;
pub const SSB_PLLTYPE_5			0x00018000	/* 25Mhz, 4 dividers */: u32 = ;
pub const SSB_PLLTYPE_6			0x00028000	/* 100/200 or 120/240 only */: u32 = ;
pub const SSB_PLLTYPE_7			0x00038000	/* 25Mhz, 4 dividers */: u32 = ;
pub const SSB_CHIPCO_CAP_PCTL		0x00040000	/* Power Control */: u32 = ;
pub const SSB_CHIPCO_CAP_OTPS		0x00380000	/* OTP size */: u32 = ;
pub const SSB_CHIPCO_CAP_OTPS_SHIFT	19: u32 = ;
pub const SSB_CHIPCO_CAP_OTPS_BASE	5: u32 = ;
pub const SSB_CHIPCO_CAP_JTAGM		0x00400000	/* JTAG master present */: u32 = ;
pub const SSB_CHIPCO_CAP_BROM		0x00800000	/* Internal boot ROM active */: u32 = ;
pub const SSB_CHIPCO_CAP_64BIT		0x08000000	/* 64-bit Backplane */: u32 = ;
pub const SSB_CHIPCO_CAP_PMU		0x10000000	/* PMU available (rev >= 20) */: u32 = ;
pub const SSB_CHIPCO_CAP_ECI		0x20000000	/* ECI available (rev >= 20) */: u32 = ;
pub const SSB_CHIPCO_CAP_SPROM		0x40000000	/* SPROM present */: u32 = ;
pub const SSB_CHIPCO_CORECTL		0x0008: u32 = ;
pub const SSB_CHIPCO_CORECTL_UARTCLK0	0x00000001	/* Drive UART with internal clock */: u32 = ;
pub const SSB_CHIPCO_CORECTL_SE		0x00000002	/* sync clk out enable (corerev >= 3) */: u32 = ;
pub const SSB_CHIPCO_CORECTL_UARTCLKEN	0x00000008	/* UART clock enable (rev >= 21) */: u32 = ;
pub const SSB_CHIPCO_BIST			0x000C: u32 = ;
pub const SSB_CHIPCO_OTPS			0x0010		/* OTP status */: u32 = ;
pub const SSB_CHIPCO_OTPS_PROGFAIL	0x80000000: u32 = ;
pub const SSB_CHIPCO_OTPS_PROTECT	0x00000007: u32 = ;
pub const SSB_CHIPCO_OTPS_HW_PROTECT	0x00000001: u32 = ;
pub const SSB_CHIPCO_OTPS_SW_PROTECT	0x00000002: u32 = ;
pub const SSB_CHIPCO_OTPS_CID_PROTECT	0x00000004: u32 = ;
pub const SSB_CHIPCO_OTPC			0x0014		/* OTP control */: u32 = ;
pub const SSB_CHIPCO_OTPC_RECWAIT	0xFF000000: u32 = ;
pub const SSB_CHIPCO_OTPC_PROGWAIT	0x00FFFF00: u32 = ;
pub const SSB_CHIPCO_OTPC_PRW_SHIFT	8: u32 = ;
pub const SSB_CHIPCO_OTPC_MAXFAIL	0x00000038: u32 = ;
pub const SSB_CHIPCO_OTPC_VSEL		0x00000006: u32 = ;
pub const SSB_CHIPCO_OTPC_SELVL		0x00000001: u32 = ;
pub const SSB_CHIPCO_OTPP			0x0018		/* OTP prog */: u32 = ;
pub const SSB_CHIPCO_OTPP_COL		0x000000FF: u32 = ;
pub const SSB_CHIPCO_OTPP_ROW		0x0000FF00: u32 = ;
pub const SSB_CHIPCO_OTPP_ROW_SHIFT	8: u32 = ;
pub const SSB_CHIPCO_OTPP_READERR	0x10000000: u32 = ;
pub const SSB_CHIPCO_OTPP_VALUE		0x20000000: u32 = ;
pub const SSB_CHIPCO_OTPP_READ		0x40000000: u32 = ;
pub const SSB_CHIPCO_OTPP_START		0x80000000: u32 = ;
pub const SSB_CHIPCO_OTPP_BUSY		0x80000000: u32 = ;
pub const SSB_CHIPCO_IRQSTAT		0x0020: u32 = ;
pub const SSB_CHIPCO_IRQMASK		0x0024: u32 = ;
pub const SSB_CHIPCO_IRQ_GPIO		0x00000001	/* gpio intr */: u32 = ;
pub const SSB_CHIPCO_IRQ_EXT		0x00000002	/* ro: ext intr pin (corerev >= 3) */: u32 = ;
pub const SSB_CHIPCO_IRQ_WDRESET		0x80000000	/* watchdog reset occurred */: u32 = ;
pub const SSB_CHIPCO_CHIPCTL		0x0028		/* Rev >= 11 only */: u32 = ;
pub const SSB_CHIPCO_CHIPSTAT		0x002C		/* Rev >= 11 only */: u32 = ;
pub const SSB_CHIPCO_JCMD			0x0030		/* Rev >= 10 only */: u32 = ;
pub const SSB_CHIPCO_JCMD_START		0x80000000: u32 = ;
pub const SSB_CHIPCO_JCMD_BUSY		0x80000000: u32 = ;
pub const SSB_CHIPCO_JCMD_PAUSE		0x40000000: u32 = ;
pub const SSB_CHIPCO_JCMD0_ACC_MASK	0x0000F000: u32 = ;
pub const SSB_CHIPCO_JCMD0_ACC_IRDR	0x00000000: u32 = ;
pub const SSB_CHIPCO_JCMD0_ACC_DR	0x00001000: u32 = ;
pub const SSB_CHIPCO_JCMD0_ACC_IR	0x00002000: u32 = ;
pub const SSB_CHIPCO_JCMD0_ACC_RESET	0x00003000: u32 = ;
pub const SSB_CHIPCO_JCMD0_ACC_IRPDR	0x00004000: u32 = ;
pub const SSB_CHIPCO_JCMD0_ACC_PDR	0x00005000: u32 = ;
pub const SSB_CHIPCO_JCMD0_IRW_MASK	0x00000F00: u32 = ;
pub const SSB_CHIPCO_JCMD_ACC_MASK	0x000F0000	/* Changes for corerev 11 */: u32 = ;
pub const SSB_CHIPCO_JCMD_ACC_IRDR	0x00000000: u32 = ;
pub const SSB_CHIPCO_JCMD_ACC_DR		0x00010000: u32 = ;
pub const SSB_CHIPCO_JCMD_ACC_IR		0x00020000: u32 = ;
pub const SSB_CHIPCO_JCMD_ACC_RESET	0x00030000: u32 = ;
pub const SSB_CHIPCO_JCMD_ACC_IRPDR	0x00040000: u32 = ;
pub const SSB_CHIPCO_JCMD_ACC_PDR	0x00050000: u32 = ;
pub const SSB_CHIPCO_JCMD_IRW_MASK	0x00001F00: u32 = ;
pub const SSB_CHIPCO_JCMD_IRW_SHIFT	8: u32 = ;
pub const SSB_CHIPCO_JCMD_DRW_MASK	0x0000003F: u32 = ;
pub const SSB_CHIPCO_JIR			0x0034		/* Rev >= 10 only */: u32 = ;
pub const SSB_CHIPCO_JDR			0x0038		/* Rev >= 10 only */: u32 = ;
pub const SSB_CHIPCO_JCTL			0x003C		/* Rev >= 10 only */: u32 = ;
pub const SSB_CHIPCO_JCTL_FORCE_CLK	4		/* Force clock */: u32 = ;
pub const SSB_CHIPCO_JCTL_EXT_EN		2		/* Enable external targets */: u32 = ;
pub const SSB_CHIPCO_JCTL_EN		1		/* Enable Jtag master */: u32 = ;
pub const SSB_CHIPCO_FLASHCTL		0x0040: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_START	0x80000000: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_BUSY	SSB_CHIPCO_FLASHCTL_START: u32 = ;
pub const SSB_CHIPCO_FLASHADDR		0x0044: u32 = ;
pub const SSB_CHIPCO_FLASHDATA		0x0048: u32 = ;
pub const SSB_CHIPCO_BCAST_ADDR		0x0050: u32 = ;
pub const SSB_CHIPCO_BCAST_DATA		0x0054: u32 = ;
pub const SSB_CHIPCO_GPIOPULLUP		0x0058		/* Rev >= 20 only */: u32 = ;
pub const SSB_CHIPCO_GPIOPULLDOWN		0x005C		/* Rev >= 20 only */: u32 = ;
pub const SSB_CHIPCO_GPIOIN		0x0060: u32 = ;
pub const SSB_CHIPCO_GPIOOUT		0x0064: u32 = ;
pub const SSB_CHIPCO_GPIOOUTEN		0x0068: u32 = ;
pub const SSB_CHIPCO_GPIOCTL		0x006C: u32 = ;
pub const SSB_CHIPCO_GPIOPOL		0x0070: u32 = ;
pub const SSB_CHIPCO_GPIOIRQ		0x0074: u32 = ;
pub const SSB_CHIPCO_WATCHDOG		0x0080: u32 = ;
pub const SSB_CHIPCO_GPIOTIMER		0x0088		/* LED powersave (corerev >= 16) */: u32 = ;
pub const SSB_CHIPCO_GPIOTIMER_OFFTIME	0x0000FFFF: u32 = ;
pub const SSB_CHIPCO_GPIOTIMER_OFFTIME_SHIFT	0: u32 = ;
pub const SSB_CHIPCO_GPIOTIMER_ONTIME	0xFFFF0000: u32 = ;
pub const SSB_CHIPCO_GPIOTIMER_ONTIME_SHIFT	16: u32 = ;
pub const SSB_CHIPCO_GPIOTOUTM		0x008C		/* LED powersave (corerev >= 16) */: u32 = ;
pub const SSB_CHIPCO_CLOCK_N		0x0090: u32 = ;
pub const SSB_CHIPCO_CLOCK_SB		0x0094: u32 = ;
pub const SSB_CHIPCO_CLOCK_PCI		0x0098: u32 = ;
pub const SSB_CHIPCO_CLOCK_M2		0x009C: u32 = ;
pub const SSB_CHIPCO_CLOCK_MIPS		0x00A0: u32 = ;
pub const SSB_CHIPCO_CLKDIV		0x00A4		/* Rev >= 3 only */: u32 = ;
pub const SSB_CHIPCO_CLKDIV_SFLASH	0x0F000000: u32 = ;
pub const SSB_CHIPCO_CLKDIV_SFLASH_SHIFT	24: u32 = ;
pub const SSB_CHIPCO_CLKDIV_OTP		0x000F0000: u32 = ;
pub const SSB_CHIPCO_CLKDIV_OTP_SHIFT	16: u32 = ;
pub const SSB_CHIPCO_CLKDIV_JTAG		0x00000F00: u32 = ;
pub const SSB_CHIPCO_CLKDIV_JTAG_SHIFT	8: u32 = ;
pub const SSB_CHIPCO_CLKDIV_UART		0x000000FF: u32 = ;
pub const SSB_CHIPCO_PLLONDELAY		0x00B0		/* Rev >= 4 only */: u32 = ;
pub const SSB_CHIPCO_FREFSELDELAY		0x00B4		/* Rev >= 4 only */: u32 = ;
pub const SSB_CHIPCO_SLOWCLKCTL		0x00B8		/* 6 <= Rev <= 9 only */: u32 = ;
pub const SSB_CHIPCO_SLOWCLKCTL_SRC	0x00000007	/* slow clock source mask */: u32 = ;
pub const SSB_CHIPCO_SLOWCLKCTL_SRC_LPO		0x00000000	/* source of slow clock is LPO */: u32 = ;
pub const SSB_CHIPCO_SLOWCLKCTL_SRC_XTAL	0x00000001	/* source of slow clock is crystal */: u32 = ;
pub const SSB_CHIPCO_SLOECLKCTL_SRC_PCI		0x00000002	/* source of slow clock is PCI */: u32 = ;
pub const SSB_CHIPCO_SLOWCLKCTL_LPOFREQ	0x00000200	/* LPOFreqSel, 1: 160Khz, 0: 32KHz */: u32 = ;
pub const SSB_CHIPCO_SLOWCLKCTL_LPOPD	0x00000400	/* LPOPowerDown, 1: LPO is disabled, 0: LPO is enabled */: u32 = ;
pub const SSB_CHIPCO_SLOWCLKCTL_FSLOW	0x00000800	/* ForceSlowClk, 1: sb/cores running on slow clock, 0: power logic control */: u32 = ;
pub const SSB_CHIPCO_SLOWCLKCTL_IPLL	0x00001000	/* IgnorePllOffReq, 1/0: power logic ignores/honors PLL clock disable requests from core */: u32 = ;
pub const SSB_CHIPCO_SLOWCLKCTL_ENXTAL	0x00002000	/* XtalControlEn, 1/0: power logic does/doesn't disable crystal when appropriate */: u32 = ;
pub const SSB_CHIPCO_SLOWCLKCTL_XTALPU	0x00004000	/* XtalPU (RO), 1/0: crystal running/disabled */: u32 = ;
pub const SSB_CHIPCO_SLOWCLKCTL_CLKDIV	0xFFFF0000	/* ClockDivider (SlowClk = 1/(4+divisor)) */: u32 = ;
pub const SSB_CHIPCO_SLOWCLKCTL_CLKDIV_SHIFT	16: u32 = ;
pub const SSB_CHIPCO_SYSCLKCTL		0x00C0		/* Rev >= 3 only */: u32 = ;
pub const SSB_CHIPCO_SYSCLKCTL_IDLPEN	0x00000001	/* ILPen: Enable Idle Low Power */: u32 = ;
pub const SSB_CHIPCO_SYSCLKCTL_ALPEN	0x00000002	/* ALPen: Enable Active Low Power */: u32 = ;
pub const SSB_CHIPCO_SYSCLKCTL_PLLEN	0x00000004	/* ForcePLLOn */: u32 = ;
pub const SSB_CHIPCO_SYSCLKCTL_FORCEALP	0x00000008	/* Force ALP (or HT if ALPen is not set */: u32 = ;
pub const SSB_CHIPCO_SYSCLKCTL_FORCEHT	0x00000010	/* Force HT */: u32 = ;
pub const SSB_CHIPCO_SYSCLKCTL_CLKDIV	0xFFFF0000	/* ClkDiv  (ILP = 1/(4+divisor)) */: u32 = ;
pub const SSB_CHIPCO_SYSCLKCTL_CLKDIV_SHIFT	16: u32 = ;
pub const SSB_CHIPCO_CLKSTSTR		0x00C4		/* Rev >= 3 only */: u32 = ;
pub const SSB_CHIPCO_PCMCIA_CFG		0x0100: u32 = ;
pub const SSB_CHIPCO_PCMCIA_MEMWAIT	0x0104: u32 = ;
pub const SSB_CHIPCO_PCMCIA_ATTRWAIT	0x0108: u32 = ;
pub const SSB_CHIPCO_PCMCIA_IOWAIT	0x010C: u32 = ;
pub const SSB_CHIPCO_IDE_CFG		0x0110: u32 = ;
pub const SSB_CHIPCO_IDE_MEMWAIT		0x0114: u32 = ;
pub const SSB_CHIPCO_IDE_ATTRWAIT		0x0118: u32 = ;
pub const SSB_CHIPCO_IDE_IOWAIT		0x011C: u32 = ;
pub const SSB_CHIPCO_PROG_CFG		0x0120: u32 = ;
pub const SSB_CHIPCO_PROG_WAITCNT		0x0124: u32 = ;
pub const SSB_CHIPCO_FLASH_CFG		0x0128: u32 = ;
pub const SSB_CHIPCO_FLASH_WAITCNT	0x012C: u32 = ;
pub const SSB_CHIPCO_CLKCTLST		0x01E0 /* Clock control and status (rev >= 20) */: u32 = ;
pub const SSB_CHIPCO_CLKCTLST_FORCEALP	0x00000001 /* Force ALP request */: u32 = ;
pub const SSB_CHIPCO_CLKCTLST_FORCEHT	0x00000002 /* Force HT request */: u32 = ;
pub const SSB_CHIPCO_CLKCTLST_FORCEILP	0x00000004 /* Force ILP request */: u32 = ;
pub const SSB_CHIPCO_CLKCTLST_HAVEALPREQ	0x00000008 /* ALP available request */: u32 = ;
pub const SSB_CHIPCO_CLKCTLST_HAVEHTREQ	0x00000010 /* HT available request */: u32 = ;
pub const SSB_CHIPCO_CLKCTLST_HWCROFF	0x00000020 /* Force HW clock request off */: u32 = ;
pub const SSB_CHIPCO_CLKCTLST_HAVEALP	0x00010000 /* ALP available */: u32 = ;
pub const SSB_CHIPCO_CLKCTLST_HAVEHT	0x00020000 /* HT available */: u32 = ;
pub const SSB_CHIPCO_CLKCTLST_4328A0_HAVEHT	0x00010000 /* 4328a0 has reversed bits */: u32 = ;
pub const SSB_CHIPCO_CLKCTLST_4328A0_HAVEALP	0x00020000 /* 4328a0 has reversed bits */: u32 = ;
pub const SSB_CHIPCO_HW_WORKAROUND	0x01E4 /* Hardware workaround (rev >= 20) */: u32 = ;
pub const SSB_CHIPCO_UART0_DATA		0x0300: u32 = ;
pub const SSB_CHIPCO_UART0_IMR		0x0304: u32 = ;
pub const SSB_CHIPCO_UART0_FCR		0x0308: u32 = ;
pub const SSB_CHIPCO_UART0_LCR		0x030C: u32 = ;
pub const SSB_CHIPCO_UART0_MCR		0x0310: u32 = ;
pub const SSB_CHIPCO_UART0_LSR		0x0314: u32 = ;
pub const SSB_CHIPCO_UART0_MSR		0x0318: u32 = ;
pub const SSB_CHIPCO_UART0_SCRATCH	0x031C: u32 = ;
pub const SSB_CHIPCO_UART1_DATA		0x0400: u32 = ;
pub const SSB_CHIPCO_UART1_IMR		0x0404: u32 = ;
pub const SSB_CHIPCO_UART1_FCR		0x0408: u32 = ;
pub const SSB_CHIPCO_UART1_LCR		0x040C: u32 = ;
pub const SSB_CHIPCO_UART1_MCR		0x0410: u32 = ;
pub const SSB_CHIPCO_UART1_LSR		0x0414: u32 = ;
pub const SSB_CHIPCO_UART1_MSR		0x0418: u32 = ;
pub const SSB_CHIPCO_UART1_SCRATCH	0x041C: u32 = ;
/* PMU registers (rev >= 20) */
pub const SSB_CHIPCO_PMU_CTL			0x0600 /* PMU control */: u32 = ;
pub const SSB_CHIPCO_PMU_CTL_ILP_DIV		0xFFFF0000 /* ILP div mask */: u32 = ;
pub const SSB_CHIPCO_PMU_CTL_ILP_DIV_SHIFT	16: u32 = ;
pub const SSB_CHIPCO_PMU_CTL_PLL_UPD		0x00000400: u32 = ;
pub const SSB_CHIPCO_PMU_CTL_NOILPONW		0x00000200 /* No ILP on wait */: u32 = ;
pub const SSB_CHIPCO_PMU_CTL_HTREQEN		0x00000100 /* HT req enable */: u32 = ;
pub const SSB_CHIPCO_PMU_CTL_ALPREQEN		0x00000080 /* ALP req enable */: u32 = ;
pub const SSB_CHIPCO_PMU_CTL_XTALFREQ		0x0000007C /* Crystal freq */: u32 = ;
pub const SSB_CHIPCO_PMU_CTL_XTALFREQ_SHIFT	2: u32 = ;
pub const SSB_CHIPCO_PMU_CTL_ILPDIVEN		0x00000002 /* ILP div enable */: u32 = ;
pub const SSB_CHIPCO_PMU_CTL_LPOSEL		0x00000001 /* LPO sel */: u32 = ;
pub const SSB_CHIPCO_PMU_CAP			0x0604 /* PMU capabilities */: u32 = ;
pub const SSB_CHIPCO_PMU_CAP_REVISION		0x000000FF /* Revision mask */: u32 = ;
pub const SSB_CHIPCO_PMU_STAT			0x0608 /* PMU status */: u32 = ;
pub const SSB_CHIPCO_PMU_STAT_INTPEND		0x00000040 /* Interrupt pending */: u32 = ;
pub const SSB_CHIPCO_PMU_STAT_SBCLKST		0x00000030 /* Backplane clock status? */: u32 = ;
pub const SSB_CHIPCO_PMU_STAT_HAVEALP		0x00000008 /* ALP available */: u32 = ;
pub const SSB_CHIPCO_PMU_STAT_HAVEHT		0x00000004 /* HT available */: u32 = ;
pub const SSB_CHIPCO_PMU_STAT_RESINIT		0x00000003 /* Res init */: u32 = ;
pub const SSB_CHIPCO_PMU_RES_STAT			0x060C /* PMU res status */: u32 = ;
pub const SSB_CHIPCO_PMU_RES_PEND			0x0610 /* PMU res pending */: u32 = ;
pub const SSB_CHIPCO_PMU_TIMER			0x0614 /* PMU timer */: u32 = ;
pub const SSB_CHIPCO_PMU_MINRES_MSK		0x0618 /* PMU min res mask */: u32 = ;
pub const SSB_CHIPCO_PMU_MAXRES_MSK		0x061C /* PMU max res mask */: u32 = ;
pub const SSB_CHIPCO_PMU_RES_TABSEL		0x0620 /* PMU res table sel */: u32 = ;
pub const SSB_CHIPCO_PMU_RES_DEPMSK		0x0624 /* PMU res dep mask */: u32 = ;
pub const SSB_CHIPCO_PMU_RES_UPDNTM		0x0628 /* PMU res updown timer */: u32 = ;
pub const SSB_CHIPCO_PMU_RES_TIMER		0x062C /* PMU res timer */: u32 = ;
pub const SSB_CHIPCO_PMU_CLKSTRETCH		0x0630 /* PMU clockstretch */: u32 = ;
pub const SSB_CHIPCO_PMU_WATCHDOG			0x0634 /* PMU watchdog */: u32 = ;
pub const SSB_CHIPCO_PMU_RES_REQTS		0x0640 /* PMU res req timer sel */: u32 = ;
pub const SSB_CHIPCO_PMU_RES_REQT			0x0644 /* PMU res req timer */: u32 = ;
pub const SSB_CHIPCO_PMU_RES_REQM			0x0648 /* PMU res req mask */: u32 = ;
pub const SSB_CHIPCO_CHIPCTL_ADDR			0x0650: u32 = ;
pub const SSB_CHIPCO_CHIPCTL_DATA			0x0654: u32 = ;
pub const SSB_CHIPCO_REGCTL_ADDR			0x0658: u32 = ;
pub const SSB_CHIPCO_REGCTL_DATA			0x065C: u32 = ;
pub const SSB_CHIPCO_PLLCTL_ADDR			0x0660: u32 = ;
pub const SSB_CHIPCO_PLLCTL_DATA			0x0664: u32 = ;
/** PMU PLL registers */
/* PMU rev 0 PLL registers */
pub const SSB_PMU0_PLLCTL0			0: u32 = ;
pub const SSB_PMU0_PLLCTL0_PDIV_MSK		0x00000001: u32 = ;
pub const SSB_PMU0_PLLCTL0_PDIV_FREQ		25000 /* kHz */: u32 = ;
pub const SSB_PMU0_PLLCTL1			1: u32 = ;
pub const SSB_PMU0_PLLCTL1_WILD_IMSK		0xF0000000 /* Wild int mask (low nibble) */: u32 = ;
pub const SSB_PMU0_PLLCTL1_WILD_IMSK_SHIFT	28: u32 = ;
pub const SSB_PMU0_PLLCTL1_WILD_FMSK		0x0FFFFF00 /* Wild frac mask */: u32 = ;
pub const SSB_PMU0_PLLCTL1_WILD_FMSK_SHIFT	8: u32 = ;
pub const SSB_PMU0_PLLCTL1_STOPMOD		0x00000040 /* Stop mod */: u32 = ;
pub const SSB_PMU0_PLLCTL2			2: u32 = ;
pub const SSB_PMU0_PLLCTL2_WILD_IMSKHI		0x0000000F /* Wild int mask (high nibble) */: u32 = ;
pub const SSB_PMU0_PLLCTL2_WILD_IMSKHI_SHIFT	0: u32 = ;
/* PMU rev 1 PLL registers */
pub const SSB_PMU1_PLLCTL0			0: u32 = ;
pub const SSB_PMU1_PLLCTL0_P1DIV			0x00F00000 /* P1 div */: u32 = ;
pub const SSB_PMU1_PLLCTL0_P1DIV_SHIFT		20: u32 = ;
pub const SSB_PMU1_PLLCTL0_P2DIV			0x0F000000 /* P2 div */: u32 = ;
pub const SSB_PMU1_PLLCTL0_P2DIV_SHIFT		24: u32 = ;
pub const SSB_PMU1_PLLCTL1			1: u32 = ;
pub const SSB_PMU1_PLLCTL1_M1DIV			0x000000FF /* M1 div */: u32 = ;
pub const SSB_PMU1_PLLCTL1_M1DIV_SHIFT		0: u32 = ;
pub const SSB_PMU1_PLLCTL1_M2DIV			0x0000FF00 /* M2 div */: u32 = ;
pub const SSB_PMU1_PLLCTL1_M2DIV_SHIFT		8: u32 = ;
pub const SSB_PMU1_PLLCTL1_M3DIV			0x00FF0000 /* M3 div */: u32 = ;
pub const SSB_PMU1_PLLCTL1_M3DIV_SHIFT		16: u32 = ;
pub const SSB_PMU1_PLLCTL1_M4DIV			0xFF000000 /* M4 div */: u32 = ;
pub const SSB_PMU1_PLLCTL1_M4DIV_SHIFT		24: u32 = ;
pub const SSB_PMU1_PLLCTL2			2: u32 = ;
pub const SSB_PMU1_PLLCTL2_M5DIV			0x000000FF /* M5 div */: u32 = ;
pub const SSB_PMU1_PLLCTL2_M5DIV_SHIFT		0: u32 = ;
pub const SSB_PMU1_PLLCTL2_M6DIV			0x0000FF00 /* M6 div */: u32 = ;
pub const SSB_PMU1_PLLCTL2_M6DIV_SHIFT		8: u32 = ;
pub const SSB_PMU1_PLLCTL2_NDIVMODE		0x000E0000 /* NDIV mode */: u32 = ;
pub const SSB_PMU1_PLLCTL2_NDIVMODE_SHIFT	17: u32 = ;
pub const SSB_PMU1_PLLCTL2_NDIVINT		0x1FF00000 /* NDIV int */: u32 = ;
pub const SSB_PMU1_PLLCTL2_NDIVINT_SHIFT		20: u32 = ;
pub const SSB_PMU1_PLLCTL3			3: u32 = ;
pub const SSB_PMU1_PLLCTL3_NDIVFRAC		0x00FFFFFF /* NDIV frac */: u32 = ;
pub const SSB_PMU1_PLLCTL3_NDIVFRAC_SHIFT	0: u32 = ;
pub const SSB_PMU1_PLLCTL4			4: u32 = ;
pub const SSB_PMU1_PLLCTL5			5: u32 = ;
pub const SSB_PMU1_PLLCTL5_CLKDRV		0xFFFFFF00 /* clk drv */: u32 = ;
pub const SSB_PMU1_PLLCTL5_CLKDRV_SHIFT		8: u32 = ;
/* BCM4312 PLL resource numbers. */
pub const SSB_PMURES_4312_SWITCHER_BURST		0: u32 = ;
pub const SSB_PMURES_4312_SWITCHER_PWM    	1: u32 = ;
pub const SSB_PMURES_4312_PA_REF_LDO		2: u32 = ;
pub const SSB_PMURES_4312_CORE_LDO_BURST		3: u32 = ;
pub const SSB_PMURES_4312_CORE_LDO_PWM		4: u32 = ;
pub const SSB_PMURES_4312_RADIO_LDO		5: u32 = ;
pub const SSB_PMURES_4312_ILP_REQUEST		6: u32 = ;
pub const SSB_PMURES_4312_BG_FILTBYP		7: u32 = ;
pub const SSB_PMURES_4312_TX_FILTBYP		8: u32 = ;
pub const SSB_PMURES_4312_RX_FILTBYP		9: u32 = ;
pub const SSB_PMURES_4312_XTAL_PU			10: u32 = ;
pub const SSB_PMURES_4312_ALP_AVAIL		11: u32 = ;
pub const SSB_PMURES_4312_BB_PLL_FILTBYP		12: u32 = ;
pub const SSB_PMURES_4312_RF_PLL_FILTBYP		13: u32 = ;
pub const SSB_PMURES_4312_HT_AVAIL		14: u32 = ;
/* BCM4325 PLL resource numbers. */
pub const SSB_PMURES_4325_BUCK_BOOST_BURST	0: u32 = ;
pub const SSB_PMURES_4325_CBUCK_BURST		1: u32 = ;
pub const SSB_PMURES_4325_CBUCK_PWM		2: u32 = ;
pub const SSB_PMURES_4325_CLDO_CBUCK_BURST	3: u32 = ;
pub const SSB_PMURES_4325_CLDO_CBUCK_PWM		4: u32 = ;
pub const SSB_PMURES_4325_BUCK_BOOST_PWM		5: u32 = ;
pub const SSB_PMURES_4325_ILP_REQUEST		6: u32 = ;
pub const SSB_PMURES_4325_ABUCK_BURST		7: u32 = ;
pub const SSB_PMURES_4325_ABUCK_PWM		8: u32 = ;
pub const SSB_PMURES_4325_LNLDO1_PU		9: u32 = ;
pub const SSB_PMURES_4325_LNLDO2_PU		10: u32 = ;
pub const SSB_PMURES_4325_LNLDO3_PU		11: u32 = ;
pub const SSB_PMURES_4325_LNLDO4_PU		12: u32 = ;
pub const SSB_PMURES_4325_XTAL_PU			13: u32 = ;
pub const SSB_PMURES_4325_ALP_AVAIL		14: u32 = ;
pub const SSB_PMURES_4325_RX_PWRSW_PU		15: u32 = ;
pub const SSB_PMURES_4325_TX_PWRSW_PU		16: u32 = ;
pub const SSB_PMURES_4325_RFPLL_PWRSW_PU		17: u32 = ;
pub const SSB_PMURES_4325_LOGEN_PWRSW_PU		18: u32 = ;
pub const SSB_PMURES_4325_AFE_PWRSW_PU		19: u32 = ;
pub const SSB_PMURES_4325_BBPLL_PWRSW_PU		20: u32 = ;
pub const SSB_PMURES_4325_HT_AVAIL		21: u32 = ;
/* BCM4328 PLL resource numbers. */
pub const SSB_PMURES_4328_EXT_SWITCHER_PWM	0: u32 = ;
pub const SSB_PMURES_4328_BB_SWITCHER_PWM		1: u32 = ;
pub const SSB_PMURES_4328_BB_SWITCHER_BURST	2: u32 = ;
pub const SSB_PMURES_4328_BB_EXT_SWITCHER_BURST	3: u32 = ;
pub const SSB_PMURES_4328_ILP_REQUEST		4: u32 = ;
pub const SSB_PMURES_4328_RADIO_SWITCHER_PWM	5: u32 = ;
pub const SSB_PMURES_4328_RADIO_SWITCHER_BURST	6: u32 = ;
pub const SSB_PMURES_4328_ROM_SWITCH		7: u32 = ;
pub const SSB_PMURES_4328_PA_REF_LDO		8: u32 = ;
pub const SSB_PMURES_4328_RADIO_LDO		9: u32 = ;
pub const SSB_PMURES_4328_AFE_LDO			10: u32 = ;
pub const SSB_PMURES_4328_PLL_LDO			11: u32 = ;
pub const SSB_PMURES_4328_BG_FILTBYP		12: u32 = ;
pub const SSB_PMURES_4328_TX_FILTBYP		13: u32 = ;
pub const SSB_PMURES_4328_RX_FILTBYP		14: u32 = ;
pub const SSB_PMURES_4328_XTAL_PU			15: u32 = ;
pub const SSB_PMURES_4328_XTAL_EN			16: u32 = ;
pub const SSB_PMURES_4328_BB_PLL_FILTBYP		17: u32 = ;
pub const SSB_PMURES_4328_RF_PLL_FILTBYP		18: u32 = ;
pub const SSB_PMURES_4328_BB_PLL_PU		19: u32 = ;
/* BCM5354 PLL resource numbers. */
pub const SSB_PMURES_5354_EXT_SWITCHER_PWM	0: u32 = ;
pub const SSB_PMURES_5354_BB_SWITCHER_PWM		1: u32 = ;
pub const SSB_PMURES_5354_BB_SWITCHER_BURST	2: u32 = ;
pub const SSB_PMURES_5354_BB_EXT_SWITCHER_BURST	3: u32 = ;
pub const SSB_PMURES_5354_ILP_REQUEST		4: u32 = ;
pub const SSB_PMURES_5354_RADIO_SWITCHER_PWM	5: u32 = ;
pub const SSB_PMURES_5354_RADIO_SWITCHER_BURST	6: u32 = ;
pub const SSB_PMURES_5354_ROM_SWITCH		7: u32 = ;
pub const SSB_PMURES_5354_PA_REF_LDO		8: u32 = ;
pub const SSB_PMURES_5354_RADIO_LDO		9: u32 = ;
pub const SSB_PMURES_5354_AFE_LDO			10: u32 = ;
pub const SSB_PMURES_5354_PLL_LDO			11: u32 = ;
pub const SSB_PMURES_5354_BG_FILTBYP		12: u32 = ;
pub const SSB_PMURES_5354_TX_FILTBYP		13: u32 = ;
pub const SSB_PMURES_5354_RX_FILTBYP		14: u32 = ;
pub const SSB_PMURES_5354_XTAL_PU			15: u32 = ;
pub const SSB_PMURES_5354_XTAL_EN			16: u32 = ;
pub const SSB_PMURES_5354_BB_PLL_FILTBYP		17: u32 = ;
pub const SSB_PMURES_5354_RF_PLL_FILTBYP		18: u32 = ;
pub const SSB_PMURES_5354_BB_PLL_PU		19: u32 = ;
/** Chip specific Chip-Status register contents. */
pub const SSB_CHIPCO_CHST_4322_SPROM_EXISTS	0x00000040 /* SPROM present */: u32 = ;
pub const SSB_CHIPCO_CHST_4325_SPROM_OTP_SEL	0x00000003: u32 = ;
pub const SSB_CHIPCO_CHST_4325_DEFCIS_SEL		0 /* OTP is powered up, use def. CIS, no SPROM */: u32 = ;
pub const SSB_CHIPCO_CHST_4325_SPROM_SEL		1 /* OTP is powered up, SPROM is present */: u32 = ;
pub const SSB_CHIPCO_CHST_4325_OTP_SEL		2 /* OTP is powered up, no SPROM */: u32 = ;
pub const SSB_CHIPCO_CHST_4325_OTP_PWRDN		3 /* OTP is powered down, SPROM is present */: u32 = ;
pub const SSB_CHIPCO_CHST_4325_SDIO_USB_MODE	0x00000004: u32 = ;
pub const SSB_CHIPCO_CHST_4325_SDIO_USB_MODE_SHIFT  2: u32 = ;
pub const SSB_CHIPCO_CHST_4325_RCAL_VALID		0x00000008: u32 = ;
pub const SSB_CHIPCO_CHST_4325_RCAL_VALID_SHIFT	3: u32 = ;
pub const SSB_CHIPCO_CHST_4325_RCAL_VALUE		0x000001F0: u32 = ;
pub const SSB_CHIPCO_CHST_4325_RCAL_VALUE_SHIFT	4: u32 = ;
pub const SSB_CHIPCO_CHST_4325_PMUTOP_2B 		0x00000200 /* 1 for 2b, 0 for to 2a */: u32 = ;
/** Macros to determine SPROM presence based on Chip-Status register. */
/** Clockcontrol masks and values **/
/* SSB_CHIPCO_CLOCK_N */
pub const SSB_CHIPCO_CLK_N1		0x0000003F	/* n1 control */: u32 = ;
pub const SSB_CHIPCO_CLK_N2		0x00003F00	/* n2 control */: u32 = ;
pub const SSB_CHIPCO_CLK_N2_SHIFT		8: u32 = ;
pub const SSB_CHIPCO_CLK_PLLC		0x000F0000	/* pll control */: u32 = ;
pub const SSB_CHIPCO_CLK_PLLC_SHIFT	16: u32 = ;
/* SSB_CHIPCO_CLOCK_SB/PCI/UART */
pub const SSB_CHIPCO_CLK_M1		0x0000003F	/* m1 control */: u32 = ;
pub const SSB_CHIPCO_CLK_M2		0x00003F00	/* m2 control */: u32 = ;
pub const SSB_CHIPCO_CLK_M2_SHIFT		8: u32 = ;
pub const SSB_CHIPCO_CLK_M3		0x003F0000	/* m3 control */: u32 = ;
pub const SSB_CHIPCO_CLK_M3_SHIFT		16: u32 = ;
pub const SSB_CHIPCO_CLK_MC		0x1F000000	/* mux control */: u32 = ;
pub const SSB_CHIPCO_CLK_MC_SHIFT		24: u32 = ;
/* N3M Clock control magic field values */
pub const SSB_CHIPCO_CLK_F6_2		0x02		/* A factor of 2 in */: u32 = ;
pub const SSB_CHIPCO_CLK_F6_3		0x03		/* 6-bit fields like */: u32 = ;
pub const SSB_CHIPCO_CLK_F6_4		0x05		/* N1, M1 or M3 */: u32 = ;
pub const SSB_CHIPCO_CLK_F6_5		0x09: u32 = ;
pub const SSB_CHIPCO_CLK_F6_6		0x11: u32 = ;
pub const SSB_CHIPCO_CLK_F6_7		0x21: u32 = ;
pub const SSB_CHIPCO_CLK_F5_BIAS		5		/* 5-bit fields get this added */: u32 = ;
pub const SSB_CHIPCO_CLK_MC_BYPASS	0x08: u32 = ;
pub const SSB_CHIPCO_CLK_MC_M1		0x04: u32 = ;
pub const SSB_CHIPCO_CLK_MC_M1M2		0x02: u32 = ;
pub const SSB_CHIPCO_CLK_MC_M1M2M3	0x01: u32 = ;
pub const SSB_CHIPCO_CLK_MC_M1M3		0x11: u32 = ;
/* Type 2 Clock control magic field values */
pub const SSB_CHIPCO_CLK_T2_BIAS		2		/* n1, n2, m1 & m3 bias */: u32 = ;
pub const SSB_CHIPCO_CLK_T2M2_BIAS	3		/* m2 bias */: u32 = ;
pub const SSB_CHIPCO_CLK_T2MC_M1BYP	1: u32 = ;
pub const SSB_CHIPCO_CLK_T2MC_M2BYP	2: u32 = ;
pub const SSB_CHIPCO_CLK_T2MC_M3BYP	4: u32 = ;
/* Type 6 Clock control magic field values */
pub const SSB_CHIPCO_CLK_T6_MMASK		1		/* bits of interest in m */: u32 = ;
pub const SSB_CHIPCO_CLK_T6_M0		120000000	/* sb clock for m = 0 */: u32 = ;
pub const SSB_CHIPCO_CLK_T6_M1		100000000	/* sb clock for m = 1 */: u32 = ;
/* Common clock base */
pub const SSB_CHIPCO_CLK_BASE1		24000000	/* Half the clock freq */: u32 = ;
pub const SSB_CHIPCO_CLK_BASE2		12500000	/* Alternate crystal on some PLL's */: u32 = ;
/* Clock control values for 200Mhz in 5350 */
pub const SSB_CHIPCO_CLK_5350_N		0x0311: u32 = ;
pub const SSB_CHIPCO_CLK_5350_M		0x04020009: u32 = ;
/** Bits in the config registers **/
pub const SSB_CHIPCO_CFG_EN		0x0001		/* Enable */: u32 = ;
pub const SSB_CHIPCO_CFG_EXTM		0x000E		/* Extif Mode */: u32 = ;
pub const SSB_CHIPCO_CFG_EXTM_ASYNC	0x0002		/* Async/Parallel flash */: u32 = ;
pub const SSB_CHIPCO_CFG_EXTM_SYNC	0x0004		/* Synchronous */: u32 = ;
pub const SSB_CHIPCO_CFG_EXTM_PCMCIA	0x0008		/* PCMCIA */: u32 = ;
pub const SSB_CHIPCO_CFG_EXTM_IDE	0x000A		/* IDE */: u32 = ;
pub const SSB_CHIPCO_CFG_DS16		0x0010		/* Data size, 0=8bit, 1=16bit */: u32 = ;
pub const SSB_CHIPCO_CFG_CLKDIV		0x0060		/* Sync: Clock divisor */: u32 = ;
pub const SSB_CHIPCO_CFG_CLKEN		0x0080		/* Sync: Clock enable */: u32 = ;
pub const SSB_CHIPCO_CFG_BSTRO		0x0100		/* Sync: Size/Bytestrobe */: u32 = ;
/** Flash-specific control/status values */
/* flashcontrol opcodes for ST flashes */
pub const SSB_CHIPCO_FLASHCTL_ST_WREN	0x0006		/* Write Enable */: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_ST_WRDIS	0x0004		/* Write Disable */: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_ST_RDSR	0x0105		/* Read Status Register */: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_ST_WRSR	0x0101		/* Write Status Register */: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_ST_READ	0x0303		/* Read Data Bytes */: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_ST_PP	0x0302		/* Page Program */: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_ST_SE	0x02D8		/* Sector Erase */: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_ST_BE	0x00C7		/* Bulk Erase */: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_ST_DP	0x00B9		/* Deep Power-down */: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_ST_RES	0x03AB		/* Read Electronic Signature */: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_ST_CSA	0x1000		/* Keep chip select asserted */: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_ST_SSE	0x0220		/* Sub-sector Erase */: u32 = ;
/* Status register bits for ST flashes */
pub const SSB_CHIPCO_FLASHSTA_ST_WIP	0x01		/* Write In Progress */: u32 = ;
pub const SSB_CHIPCO_FLASHSTA_ST_WEL	0x02		/* Write Enable Latch */: u32 = ;
pub const SSB_CHIPCO_FLASHSTA_ST_BP	0x1C		/* Block Protect */: u32 = ;
pub const SSB_CHIPCO_FLASHSTA_ST_BP_SHIFT	2: u32 = ;
pub const SSB_CHIPCO_FLASHSTA_ST_SRWD	0x80		/* Status Register Write Disable */: u32 = ;
/* flashcontrol opcodes for Atmel flashes */
pub const SSB_CHIPCO_FLASHCTL_AT_READ		0x07E8: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_AT_PAGE_READ	0x07D2: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF1_READ	/* FIXME */: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF2_READ	/* FIXME */: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_AT_STATUS		0x01D7: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF1_WRITE	0x0384: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF2_WRITE	0x0387: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF1_ERASE_PRGM	0x0283	/* Erase program */: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF2_ERASE_PRGM	0x0286	/* Erase program */: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF1_PROGRAM	0x0288: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF2_PROGRAM	0x0289: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_AT_PAGE_ERASE	0x0281: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_AT_BLOCK_ERASE	0x0250: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF1_WRER_PRGM	0x0382	/* Write erase program */: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF2_WRER_PRGM	0x0385	/* Write erase program */: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF1_LOAD	0x0253: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF2_LOAD	0x0255: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF1_COMPARE	0x0260: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF2_COMPARE	0x0261: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF1_REPROGRAM	0x0258: u32 = ;
pub const SSB_CHIPCO_FLASHCTL_AT_BUF2_REPROGRAM	0x0259: u32 = ;
/* Status register bits for Atmel flashes */
pub const SSB_CHIPCO_FLASHSTA_AT_READY	0x80: u32 = ;
pub const SSB_CHIPCO_FLASHSTA_AT_MISMATCH	0x40: u32 = ;
pub const SSB_CHIPCO_FLASHSTA_AT_ID	0x38: u32 = ;
pub const SSB_CHIPCO_FLASHSTA_AT_ID_SHIFT	3: u32 = ;
/** OTP **/
/* OTP regions */
pub const SSB_CHIPCO_OTP_HW_REGION	SSB_CHIPCO_OTPS_HW_PROTECT: u32 = ;
pub const SSB_CHIPCO_OTP_SW_REGION	SSB_CHIPCO_OTPS_SW_PROTECT: u32 = ;
pub const SSB_CHIPCO_OTP_CID_REGION	SSB_CHIPCO_OTPS_CID_PROTECT: u32 = ;
/* OTP regions (Byte offsets from otp size) */
pub const SSB_CHIPCO_OTP_SWLIM_OFF	(-8): u32 = ;
pub const SSB_CHIPCO_OTP_CIDBASE_OFF	0: u32 = ;
pub const SSB_CHIPCO_OTP_CIDLIM_OFF	8: u32 = ;
/* Predefined OTP words (Word offset from otp size) */
pub const SSB_CHIPCO_OTP_BOUNDARY_OFF	(-4): u32 = ;
pub const SSB_CHIPCO_OTP_HWSIGN_OFF	(-3): u32 = ;
pub const SSB_CHIPCO_OTP_SWSIGN_OFF	(-2): u32 = ;
pub const SSB_CHIPCO_OTP_CIDSIGN_OFF	(-1): u32 = ;
pub const SSB_CHIPCO_OTP_CID_OFF		0: u32 = ;
pub const SSB_CHIPCO_OTP_PKG_OFF		1: u32 = ;
pub const SSB_CHIPCO_OTP_FID_OFF		2: u32 = ;
pub const SSB_CHIPCO_OTP_RSV_OFF		3: u32 = ;
pub const SSB_CHIPCO_OTP_LIM_OFF		4: u32 = ;
pub const SSB_CHIPCO_OTP_SIGNATURE	0x578A: u32 = ;
pub const SSB_CHIPCO_OTP_MAGIC		0x4E56: u32 = ;
/* Data for the PMU, if available.
 * Check availability with ((struct ssb_chipcommon)->capabilities & SSB_CHIPCO_CAP_PMU)
 */
/* Register access */
/* Chipcommon GPIO pin access. */
/* PMU support */
// C preprocessor conditional: CONFIG_SSB_SERIAL.
#[repr(C)] pub struct ssb_device { _private: [u8; 0] }
#[repr(C)] pub struct ssb_serial_port { _private: [u8; 0] }
#[repr(C)] pub struct ssb_chipcommon_pmu { pub rev: u8, pub crystalfreq: u32 }
#[repr(C)] pub struct ssb_chipcommon { pub dev:*mut ssb_device,pub capabilities:u32,pub status:u32,pub fast_pwrup_delay:u16,pub gpio_lock:spinlock_t,pub pmu:ssb_chipcommon_pmu,pub ticks_per_ms:u32,pub max_timer_ms:u32 }
pub type spinlock_t=core::ffi::c_void;
#[inline] pub unsafe fn ssb_chipco_available(cc:*mut ssb_chipcommon)->bool { !(*cc).dev.is_null() }
extern "C" { pub fn ssb_read32(dev:*mut ssb_device,offset:u32)->u32; pub fn ssb_write32(dev:*mut ssb_device,offset:u32,val:u32); pub fn ssb_chipcommon_init(cc:*mut ssb_chipcommon); pub fn ssb_chipco_suspend(cc:*mut ssb_chipcommon); pub fn ssb_chipco_resume(cc:*mut ssb_chipcommon); pub fn ssb_chipco_get_clockcpu(cc:*mut ssb_chipcommon,plltype:*mut u32,n:*mut u32,m:*mut u32); pub fn ssb_chipco_get_clockcontrol(cc:*mut ssb_chipcommon,plltype:*mut u32,n:*mut u32,m:*mut u32); pub fn ssb_chipco_timing_init(cc:*mut ssb_chipcommon,ns_per_cycle:core::ffi::c_ulong); pub fn ssb_chipco_set_clockmode(cc:*mut ssb_chipcommon,mode:ssb_clkmode); pub fn ssb_chipco_watchdog_timer_set(cc:*mut ssb_chipcommon,ticks:u32)->u32; pub fn ssb_chipco_irq_mask(cc:*mut ssb_chipcommon,mask:u32,value:u32); pub fn ssb_chipco_irq_status(cc:*mut ssb_chipcommon,mask:u32)->u32; pub fn ssb_pmu_init(cc:*mut ssb_chipcommon); pub fn ssb_pmu_set_ldo_voltage(cc:*mut ssb_chipcommon,id:ssb_pmu_ldo_volt_id,voltage:u32); pub fn ssb_pmu_set_ldo_paref(cc:*mut ssb_chipcommon,on:bool); pub fn ssb_pmu_spuravoid_pllupdate(cc:*mut ssb_chipcommon,spuravoid:i32); }
#[repr(C)] pub enum ssb_clkmode { SSB_CLKMODE_SLOW,SSB_CLKMODE_FAST,SSB_CLKMODE_DYNAMIC }
#[repr(C)] pub enum ssb_pmu_ldo_volt_id { LDO_PAREF=0,LDO_VOLT1,LDO_VOLT2,LDO_VOLT3 }
#[inline] pub unsafe fn chipco_read32(cc:*mut ssb_chipcommon,o:u32)->u32 { ssb_read32((*cc).dev,o) }
#[inline] pub unsafe fn chipco_write32(cc:*mut ssb_chipcommon,o:u32,v:u32) { ssb_write32((*cc).dev,o,v) }
#[inline] pub unsafe fn chipco_mask32(cc:*mut ssb_chipcommon,o:u32,m:u32) { chipco_write32(cc,o,chipco_read32(cc,o)&m) }
#[inline] pub unsafe fn chipco_set32(cc:*mut ssb_chipcommon,o:u32,s:u32) { chipco_write32(cc,o,chipco_read32(cc,o)|s) }
#[inline] pub unsafe fn chipco_maskset32(cc:*mut ssb_chipcommon,o:u32,m:u32,s:u32) { chipco_write32(cc,o,(chipco_read32(cc,o)&m)|s) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
