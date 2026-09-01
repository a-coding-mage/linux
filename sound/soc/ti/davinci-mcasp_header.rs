/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ALSA SoC McASP Audio Layer for TI DAVINCI processor
 *
 * MCASP related definitions
 *
 * Author: Nirmal Pandey <n-pandey@ti.com>,
 *         Suresh Rajashekara <suresh.r@ti.com>
 *         Steve Chen <schen@.mvista.com>
 *
 * Copyright:   (C) 2009 MontaVista Software, Inc., <source@mvista.com>
 * Copyright:   (C) 2009  Texas Instruments, India
 */

/*
 * McASP register definitions
 */
pub const DAVINCI_MCASP_PID_REG: u32 = 0x00;
pub const DAVINCI_MCASP_PWREMUMGT_REG: u32 = 0x04;

pub const DAVINCI_MCASP_PFUNC_REG: u32 = 0x10;
pub const DAVINCI_MCASP_PDIR_REG: u32 = 0x14;
pub const DAVINCI_MCASP_PDOUT_REG: u32 = 0x18;
pub const DAVINCI_MCASP_PDSET_REG: u32 = 0x1c;

pub const DAVINCI_MCASP_PDCLR_REG: u32 = 0x20;

pub const DAVINCI_MCASP_TLGC_REG: u32 = 0x30;
pub const DAVINCI_MCASP_TLMR_REG: u32 = 0x34;

pub const DAVINCI_MCASP_GBLCTL_REG: u32 = 0x44;
pub const DAVINCI_MCASP_AMUTE_REG: u32 = 0x48;
pub const DAVINCI_MCASP_LBCTL_REG: u32 = 0x4c;

pub const DAVINCI_MCASP_TXDITCTL_REG: u32 = 0x50;

pub const DAVINCI_MCASP_GBLCTLR_REG: u32 = 0x60;
pub const DAVINCI_MCASP_RXMASK_REG: u32 = 0x64;
pub const DAVINCI_MCASP_RXFMT_REG: u32 = 0x68;
pub const DAVINCI_MCASP_RXFMCTL_REG: u32 = 0x6c;

pub const DAVINCI_MCASP_ACLKRCTL_REG: u32 = 0x70;
pub const DAVINCI_MCASP_AHCLKRCTL_REG: u32 = 0x74;
pub const DAVINCI_MCASP_RXTDM_REG: u32 = 0x78;
pub const DAVINCI_MCASP_EVTCTLR_REG: u32 = 0x7c;

pub const DAVINCI_MCASP_RXSTAT_REG: u32 = 0x80;
pub const DAVINCI_MCASP_RXTDMSLOT_REG: u32 = 0x84;
pub const DAVINCI_MCASP_RXCLKCHK_REG: u32 = 0x88;
pub const DAVINCI_MCASP_REVTCTL_REG: u32 = 0x8c;

pub const DAVINCI_MCASP_GBLCTLX_REG: u32 = 0xa0;
pub const DAVINCI_MCASP_TXMASK_REG: u32 = 0xa4;
pub const DAVINCI_MCASP_TXFMT_REG: u32 = 0xa8;
pub const DAVINCI_MCASP_TXFMCTL_REG: u32 = 0xac;

pub const DAVINCI_MCASP_ACLKXCTL_REG: u32 = 0xb0;
pub const DAVINCI_MCASP_AHCLKXCTL_REG: u32 = 0xb4;
pub const DAVINCI_MCASP_TXTDM_REG: u32 = 0xb8;
pub const DAVINCI_MCASP_EVTCTLX_REG: u32 = 0xbc;

pub const DAVINCI_MCASP_TXSTAT_REG: u32 = 0xc0;
pub const DAVINCI_MCASP_TXTDMSLOT_REG: u32 = 0xc4;
pub const DAVINCI_MCASP_TXCLKCHK_REG: u32 = 0xc8;
pub const DAVINCI_MCASP_XEVTCTL_REG: u32 = 0xcc;

/* Left(even TDM Slot) Channel Status Register File */
pub const DAVINCI_MCASP_DITCSRA_REG: u32 = 0x100;
/* Right(odd TDM slot) Channel Status Register File */
pub const DAVINCI_MCASP_DITCSRB_REG: u32 = 0x118;
/* Left(even TDM slot) User Data Register File */
pub const DAVINCI_MCASP_DITUDRA_REG: u32 = 0x130;
/* Right(odd TDM Slot) User Data Register File */
pub const DAVINCI_MCASP_DITUDRB_REG: u32 = 0x148;

/* Serializer n Control Register */
pub const DAVINCI_MCASP_XRSRCTL_BASE_REG: u32 = 0x180;
pub const fn DAVINCI_MCASP_XRSRCTL_REG(n: u32) -> u32 {
    DAVINCI_MCASP_XRSRCTL_BASE_REG + (n << 2)
}

/* Transmit Buffer for Serializer n */
pub const fn DAVINCI_MCASP_TXBUF_REG(n: u32) -> u32 {
    0x200 + (n << 2)
}
/* Receive Buffer for Serializer n */
pub const fn DAVINCI_MCASP_RXBUF_REG(n: u32) -> u32 {
    0x280 + (n << 2)
}

/* McASP FIFO Registers */
pub const DAVINCI_MCASP_V2_AFIFO_BASE: u32 = 0x1010;
pub const DAVINCI_MCASP_V3_AFIFO_BASE: u32 = 0x1000;

/* FIFO register offsets from AFIFO base */
pub const MCASP_WFIFOCTL_OFFSET: u32 = 0x0;
pub const MCASP_WFIFOSTS_OFFSET: u32 = 0x4;
pub const MCASP_RFIFOCTL_OFFSET: u32 = 0x8;
pub const MCASP_RFIFOSTS_OFFSET: u32 = 0xc;

pub const fn BIT(n: u32) -> u32 {
    1u32 << n
}

/*
 * DAVINCI_MCASP_PWREMUMGT_REG - Power Down and Emulation Management
 *     Register Bits
 */
pub const MCASP_FREE: u32 = BIT(0);
pub const MCASP_SOFT: u32 = BIT(1);

/*
 * DAVINCI_MCASP_PFUNC_REG - Pin Function / GPIO Enable Register Bits
 * DAVINCI_MCASP_PDIR_REG - Pin Direction Register Bits
 * DAVINCI_MCASP_PDOUT_REG - Pin output in GPIO mode
 * DAVINCI_MCASP_PDSET_REG - Pin input in GPIO mode
 */
pub const fn PIN_BIT_AXR(n: u32) -> u32 {
    n
}
pub const PIN_BIT_AMUTE: u32 = 25;
pub const PIN_BIT_ACLKX: u32 = 26;
pub const PIN_BIT_AHCLKX: u32 = 27;
pub const PIN_BIT_AFSX: u32 = 28;
pub const PIN_BIT_ACLKR: u32 = 29;
pub const PIN_BIT_AHCLKR: u32 = 30;
pub const PIN_BIT_AFSR: u32 = 31;

/*
 * DAVINCI_MCASP_TXDITCTL_REG - Transmit DIT Control Register Bits
 */
pub const DITEN: u32 = BIT(0); /* Transmit DIT mode enable/disable */
pub const VA: u32 = BIT(2);
pub const VB: u32 = BIT(3);

/*
 * DAVINCI_MCASP_TXFMT_REG - Transmit Bitstream Format Register Bits
 */
pub const fn TXROT(val: u32) -> u32 {
    val
}
pub const TXSEL: u32 = BIT(3);
pub const fn TXSSZ(val: u32) -> u32 {
    val << 4
}
pub const fn TXPBIT(val: u32) -> u32 {
    val << 8
}
pub const fn TXPAD(val: u32) -> u32 {
    val << 13
}
pub const TXORD: u32 = BIT(15);
pub const fn FSXDLY(val: u32) -> u32 {
    val << 16
}

/*
 * DAVINCI_MCASP_RXFMT_REG - Receive Bitstream Format Register Bits
 */
pub const fn RXROT(val: u32) -> u32 {
    val
}
pub const RXSEL: u32 = BIT(3);
pub const fn RXSSZ(val: u32) -> u32 {
    val << 4
}
pub const fn RXPBIT(val: u32) -> u32 {
    val << 8
}
pub const fn RXPAD(val: u32) -> u32 {
    val << 13
}
pub const RXORD: u32 = BIT(15);
pub const fn FSRDLY(val: u32) -> u32 {
    val << 16
}

/*
 * DAVINCI_MCASP_TXFMCTL_REG -  Transmit Frame Control Register Bits
 */
pub const FSXPOL: u32 = BIT(0);
pub const AFSXE: u32 = BIT(1);
pub const FSXDUR: u32 = BIT(4);
pub const fn FSXMOD(val: u32) -> u32 {
    val << 7
}

/*
 * DAVINCI_MCASP_RXFMCTL_REG - Receive Frame Control Register Bits
 */
pub const FSRPOL: u32 = BIT(0);
pub const AFSRE: u32 = BIT(1);
pub const FSRDUR: u32 = BIT(4);
pub const fn FSRMOD(val: u32) -> u32 {
    val << 7
}

/*
 * DAVINCI_MCASP_ACLKXCTL_REG - Transmit Clock Control Register Bits
 */
pub const fn ACLKXDIV(val: u32) -> u32 {
    val
}
pub const ACLKXE: u32 = BIT(5);
pub const TX_ASYNC: u32 = BIT(6);
pub const ACLKXPOL: u32 = BIT(7);
pub const ACLKXDIV_MASK: u32 = 0x1f;

/*
 * DAVINCI_MCASP_ACLKRCTL_REG Receive Clock Control Register Bits
 */
pub const fn ACLKRDIV(val: u32) -> u32 {
    val
}
pub const ACLKRE: u32 = BIT(5);
pub const RX_ASYNC: u32 = BIT(6);
pub const ACLKRPOL: u32 = BIT(7);
pub const ACLKRDIV_MASK: u32 = 0x1f;

/*
 * DAVINCI_MCASP_AHCLKXCTL_REG - High Frequency Transmit Clock Control
 *     Register Bits
 */
pub const fn AHCLKXDIV(val: u32) -> u32 {
    val
}
pub const AHCLKXPOL: u32 = BIT(14);
pub const AHCLKXE: u32 = BIT(15);
pub const AHCLKXDIV_MASK: u32 = 0xfff;

/*
 * DAVINCI_MCASP_AHCLKRCTL_REG - High Frequency Receive Clock Control
 *     Register Bits
 */
pub const fn AHCLKRDIV(val: u32) -> u32 {
    val
}
pub const AHCLKRPOL: u32 = BIT(14);
pub const AHCLKRE: u32 = BIT(15);
pub const AHCLKRDIV_MASK: u32 = 0xfff;

/*
 * DAVINCI_MCASP_XRSRCTL_BASE_REG -  Serializer Control Register Bits
 */
pub const fn MODE(val: u32) -> u32 {
    val
}
pub const DISMOD_3STATE: u32 = 0x0;
pub const DISMOD_LOW: u32 = 0x2 << 2;
pub const DISMOD_HIGH: u32 = 0x3 << 2;
pub const fn DISMOD_VAL(x: u32) -> u32 {
    x << 2
}
pub const DISMOD_MASK: u32 = DISMOD_HIGH;
pub const TXSTATE: u32 = BIT(4);
pub const RXSTATE: u32 = BIT(5);
pub const SRMOD_MASK: u32 = 3;
pub const SRMOD_INACTIVE: u32 = 0;

/*
 * DAVINCI_MCASP_LBCTL_REG - Loop Back Control Register Bits
 */
pub const LBEN: u32 = BIT(0);
pub const LBORD: u32 = BIT(1);
pub const fn LBGENMODE(val: u32) -> u32 {
    val << 2
}

/*
 * DAVINCI_MCASP_TXTDMSLOT_REG - Transmit TDM Slot Register configuration
 */
pub const fn TXTDMS(n: u32) -> u32 {
    1u32 << n
}

/*
 * DAVINCI_MCASP_RXTDMSLOT_REG - Receive TDM Slot Register configuration
 */
pub const fn RXTDMS(n: u32) -> u32 {
    1u32 << n
}

/*
 * DAVINCI_MCASP_GBLCTL_REG -  Global Control Register Bits
 */
pub const RXCLKRST: u32 = BIT(0); /* Receiver Clock Divider Reset */
pub const RXHCLKRST: u32 = BIT(1); /* Receiver High Frequency Clock Divider */
pub const RXSERCLR: u32 = BIT(2); /* Receiver Serializer Clear */
pub const RXSMRST: u32 = BIT(3); /* Receiver State Machine Reset */
pub const RXFSRST: u32 = BIT(4); /* Frame Sync Generator Reset */
pub const TXCLKRST: u32 = BIT(8); /* Transmitter Clock Divider Reset */
pub const TXHCLKRST: u32 = BIT(9); /* Transmitter High Frequency Clock Divider*/
pub const TXSERCLR: u32 = BIT(10); /* Transmit Serializer Clear */
pub const TXSMRST: u32 = BIT(11); /* Transmitter State Machine Reset */
pub const TXFSRST: u32 = BIT(12); /* Frame Sync Generator Reset */

/*
 * DAVINCI_MCASP_TXSTAT_REG - Transmitter Status Register Bits
 * DAVINCI_MCASP_RXSTAT_REG - Receiver Status Register Bits
 */
pub const XRERR: u32 = BIT(8); /* Transmit/Receive error */
pub const XRDATA: u32 = BIT(5); /* Transmit/Receive data ready */

/*
 * DAVINCI_MCASP_AMUTE_REG -  Mute Control Register Bits
 */
pub const fn MUTENA(val: u32) -> u32 {
    val
}
pub const MUTEINPOL: u32 = BIT(2);
pub const MUTEINENA: u32 = BIT(3);
pub const MUTEIN: u32 = BIT(4);
pub const MUTER: u32 = BIT(5);
pub const MUTEX: u32 = BIT(6);
pub const MUTEFSR: u32 = BIT(7);
pub const MUTEFSX: u32 = BIT(8);
pub const MUTEBADCLKR: u32 = BIT(9);
pub const MUTEBADCLKX: u32 = BIT(10);
pub const MUTERXDMAERR: u32 = BIT(11);
pub const MUTETXDMAERR: u32 = BIT(12);

/*
 * DAVINCI_MCASP_REVTCTL_REG - Receiver DMA Event Control Register bits
 */
pub const RXDATADMADIS: u32 = BIT(0);

/*
 * DAVINCI_MCASP_XEVTCTL_REG - Transmitter DMA Event Control Register bits
 */
pub const TXDATADMADIS: u32 = BIT(0);

/*
 * DAVINCI_MCASP_EVTCTLR_REG - Receiver Interrupt Control Register Bits
 */
pub const ROVRN: u32 = BIT(0);

/*
 * DAVINCI_MCASP_EVTCTLX_REG - Transmitter Interrupt Control Register Bits
 */
pub const XUNDRN: u32 = BIT(0);

/*
 * DAVINCI_MCASP_W[R]FIFOCTL - Write/Read FIFO Control Register bits
 */
pub const FIFO_ENABLE: u32 = BIT(16);
pub const NUMEVT_MASK: u32 = 0xFF << 8;
pub const fn NUMEVT(x: u32) -> u32 {
    (x & 0xFF) << 8
}
pub const NUMDMA_MASK: u32 = 0xFF;

/* Source of High-frequency transmit/receive clock */
pub const MCASP_CLK_HCLK_AHCLK: u32 = 0; /* AHCLKX/R */
pub const MCASP_CLK_HCLK_AUXCLK: u32 = 1; /* Internal functional clock */
pub const MCASP_CLK_HCLK_AHCLK_TXONLY: u32 = 2; /* AHCLKX for TX only */
pub const MCASP_CLK_HCLK_AHCLK_RXONLY: u32 = 3; /* AHCLKR for RX only */
pub const MCASP_CLK_HCLK_AUXCLK_TXONLY: u32 = 4; /* AUXCLK for TX only */
pub const MCASP_CLK_HCLK_AUXCLK_RXONLY: u32 = 5; /* AUXCLK for RX only */

/* clock divider IDs */
pub const MCASP_CLKDIV_AUXCLK: u32 = 0; /* HCLK divider from AUXCLK */
pub const MCASP_CLKDIV_BCLK: u32 = 1; /* BCLK divider from HCLK */
pub const MCASP_CLKDIV_BCLK_FS_RATIO: u32 = 2; /* to set BCLK FS ration */
pub const MCASP_CLKDIV_AUXCLK_TXONLY: u32 = 3; /* AUXCLK divider for TX only */
pub const MCASP_CLKDIV_AUXCLK_RXONLY: u32 = 4; /* AUXCLK divider for RX only */
pub const MCASP_CLKDIV_BCLK_TXONLY: u32 = 5; /* BCLK divider for TX only */
pub const MCASP_CLKDIV_BCLK_RXONLY: u32 = 6; /* BCLK divider for RX only */
pub const MCASP_CLKDIV_BCLK_FS_RATIO_TXONLY: u32 = 7; /* BCLK/FS ratio for TX only */
pub const MCASP_CLKDIV_BCLK_FS_RATIO_RXONLY: u32 = 8; /* BCLK/FS ratio for RX only*/

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
