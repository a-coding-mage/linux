/* SPDX-License-Identifier: GPL-2.0 */
/*
 * bbc.h: Defines for BootBus Controller found on UltraSPARC-III
 *        systems.
 *
 * Copyright (C) 2000 David S. Miller (davem@redhat.com)
 */

/* Register sizes are indicated by "B" (Byte, 1-byte),
 * "H" (Half-word, 2 bytes), "W" (Word, 4 bytes) or
 * "Q" (Quad, 8 bytes) inside brackets.
 */

pub const BBC_AID: u32 = 0x00; /* [B] Agent ID */
pub const BBC_DEVP: u32 = 0x01; /* [B] Device Present */
pub const BBC_ARB: u32 = 0x02; /* [B] Arbitration */
pub const BBC_QUIESCE: u32 = 0x03; /* [B] Quiesce */
pub const BBC_WDACTION: u32 = 0x04; /* [B] Watchdog Action */
pub const BBC_SPG: u32 = 0x06; /* [B] Soft POR Gen */
pub const BBC_SXG: u32 = 0x07; /* [B] Soft XIR Gen */
pub const BBC_PSRC: u32 = 0x08; /* [W] POR Source */
pub const BBC_XSRC: u32 = 0x0c; /* [B] XIR Source */
pub const BBC_CSC: u32 = 0x0d; /* [B] Clock Synthesizers Control */
pub const BBC_ES_CTRL: u32 = 0x0e; /* [H] Energy Star Control */
pub const BBC_ES_ACT: u32 = 0x10; /* [W] E* Assert Change Time */
pub const BBC_ES_DACT: u32 = 0x14; /* [B] E* De-Assert Change Time */
pub const BBC_ES_DABT: u32 = 0x15; /* [B] E* De-Assert Bypass Time */
pub const BBC_ES_ABT: u32 = 0x16; /* [H] E* Assert Bypass Time */
pub const BBC_ES_PST: u32 = 0x18; /* [W] E* PLL Settle Time */
pub const BBC_ES_FSL: u32 = 0x1c; /* [W] E* Frequency Switch Latency */
pub const BBC_EBUST: u32 = 0x20; /* [Q] EBUS Timing */
pub const BBC_JTAG_CMD: u32 = 0x28; /* [W] JTAG+ Command */
pub const BBC_JTAG_CTRL: u32 = 0x2c; /* [B] JTAG+ Control */
pub const BBC_I2C_SEL: u32 = 0x2d; /* [B] I2C Selection */
pub const BBC_I2C_0_S1: u32 = 0x2e; /* [B] I2C ctrlr-0 reg S1 */
pub const BBC_I2C_0_S0: u32 = 0x2f; /* [B] I2C ctrlr-0 regs S0,S0',S2,S3 */
pub const BBC_I2C_1_S1: u32 = 0x30; /* [B] I2C ctrlr-1 reg S1 */
pub const BBC_I2C_1_S0: u32 = 0x31; /* [B] I2C ctrlr-1 regs S0,S0',S2,S3 */
pub const BBC_KBD_BEEP: u32 = 0x32; /* [B] Keyboard Beep */
pub const BBC_KBD_BCNT: u32 = 0x34; /* [W] Keyboard Beep Counter */

pub const BBC_REGS_SIZE: u32 = 0x40;

/* There is a 2K scratch ram area at offset 0x80000 but I doubt
 * we will use it for anything.
 */

/* Agent ID register.  This register shows the Safari Agent ID
 * for the processors.  The value returned depends upon which
 * cpu is reading the register.
 */
pub const BBC_AID_ID: u32 = 0x07;
pub const BBC_AID_RESV: u32 = 0xf8;

/* Device Present register.  One can determine which cpus are actually
 * present in the machine by interrogating this register.
 */
pub const BBC_DEVP_CPU0: u32 = 0x01;
pub const BBC_DEVP_CPU1: u32 = 0x02;
pub const BBC_DEVP_CPU2: u32 = 0x04;
pub const BBC_DEVP_CPU3: u32 = 0x08;
pub const BBC_DEVP_RESV: u32 = 0xf0;

/* Arbitration register.  This register is used to block access to
 * the BBC from a particular cpu.
 */
pub const BBC_ARB_CPU0: u32 = 0x01;
pub const BBC_ARB_CPU1: u32 = 0x02;
pub const BBC_ARB_CPU2: u32 = 0x04;
pub const BBC_ARB_CPU3: u32 = 0x08;
pub const BBC_ARB_RESV: u32 = 0xf0;

/* Quiesce register.  Bus and BBC segments for cpus can be disabled
 * with this register, ie. for hot plugging.
 */
pub const BBC_QUIESCE_S02: u32 = 0x01;
pub const BBC_QUIESCE_S13: u32 = 0x02;
pub const BBC_QUIESCE_B02: u32 = 0x04;
pub const BBC_QUIESCE_B13: u32 = 0x08;
pub const BBC_QUIESCE_FD0: u32 = 0x10;
pub const BBC_QUIESCE_FD1: u32 = 0x20;
pub const BBC_QUIESCE_FD2: u32 = 0x40;
pub const BBC_QUIESCE_FD3: u32 = 0x80;

/* Watchdog Action register.  When the watchdog device timer expires
 * a line is enabled to the BBC.  The action BBC takes when this line
 * is asserted can be controlled by this regiser.
 */
pub const BBC_WDACTION_RST: u32 = 0x01;
pub const BBC_WDACTION_RESV: u32 = 0xfe;

/* Soft_POR_GEN register. */
pub const BBC_SPG_CPU0: u32 = 0x01;
pub const BBC_SPG_CPU1: u32 = 0x02;
pub const BBC_SPG_CPU2: u32 = 0x04;
pub const BBC_SPG_CPU3: u32 = 0x08;
pub const BBC_SPG_CPUALL: u32 = 0x10;
pub const BBC_SPG_RESV: u32 = 0xe0;

/* Soft_XIR_GEN register. */
pub const BBC_SXG_CPU0: u32 = 0x01;
pub const BBC_SXG_CPU1: u32 = 0x02;
pub const BBC_SXG_CPU2: u32 = 0x04;
pub const BBC_SXG_CPU3: u32 = 0x08;
pub const BBC_SXG_RESV: u32 = 0xf0;

/* POR Source register. */
pub const BBC_PSRC_SPG0: u32 = 0x0001;
pub const BBC_PSRC_SPG1: u32 = 0x0002;
pub const BBC_PSRC_SPG2: u32 = 0x0004;
pub const BBC_PSRC_SPG3: u32 = 0x0008;
pub const BBC_PSRC_SPGSYS: u32 = 0x0010;
pub const BBC_PSRC_JTAG: u32 = 0x0020;
pub const BBC_PSRC_BUTTON: u32 = 0x0040;
pub const BBC_PSRC_PWRUP: u32 = 0x0080;
pub const BBC_PSRC_FE0: u32 = 0x0100;
pub const BBC_PSRC_FE1: u32 = 0x0200;
pub const BBC_PSRC_FE2: u32 = 0x0400;
pub const BBC_PSRC_FE3: u32 = 0x0800;
pub const BBC_PSRC_FE4: u32 = 0x1000;
pub const BBC_PSRC_FE5: u32 = 0x2000;
pub const BBC_PSRC_FE6: u32 = 0x4000;
pub const BBC_PSRC_SYNTH: u32 = 0x8000;
pub const BBC_PSRC_WDT: u32 = 0x10000;
pub const BBC_PSRC_RSC: u32 = 0x20000;

/* XIR Source register. */
pub const BBC_XSRC_SXG0: u32 = 0x01;
pub const BBC_XSRC_SXG1: u32 = 0x02;
pub const BBC_XSRC_SXG2: u32 = 0x04;
pub const BBC_XSRC_SXG3: u32 = 0x08;
pub const BBC_XSRC_JTAG: u32 = 0x10;
pub const BBC_XSRC_W_OR_B: u32 = 0x20;
pub const BBC_XSRC_RESV: u32 = 0xc0;

/* Clock Synthesizers Control register. */
pub const BBC_CSC_SLOAD: u32 = 0x01;
pub const BBC_CSC_SDATA: u32 = 0x02;
pub const BBC_CSC_SCLOCK: u32 = 0x04;
pub const BBC_CSC_RESV: u32 = 0x78;
pub const BBC_CSC_RST: u32 = 0x80;

/* Energy Star Control register. */
pub const BBC_ES_CTRL_1_1: u32 = 0x01;
pub const BBC_ES_CTRL_1_2: u32 = 0x02;
pub const BBC_ES_CTRL_1_32: u32 = 0x20;
pub const BBC_ES_RESV: u32 = 0xdc;

/* Energy Star timing registers. */
pub const BBC_ES_ACT_VAL: u32 = 0xff;
pub const BBC_ES_ABT_VAL: u32 = 0xffff;
pub const BBC_ES_PST_VAL: u32 = 0xffffffff;
pub const BBC_ES_FSL_VAL: u32 = 0xffffffff;

/* Keyboard Beep control register. */
pub const BBC_KBD_BEEP_ENABLE: u32 = 0x01;
pub const BBC_KBD_BEEP_RESV: u32 = 0xfe;

/* Keyboard Beep Counter register. */
pub const BBC_KBD_BCNT_BITS: u32 = 0x0007fc00;
pub const BBC_KBC_BCNT_RESV: u32 = 0xfff803ff;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
