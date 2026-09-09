/* SPDX-License-Identifier: GPL-2.0 */

/*
 * linux/include/asm-sh/sh7763drp.h
 *
 * Copyright (C) 2008 Renesas Solutions
 * Copyright (C) 2008 Nobuhiro Iwamatsu <iwamatsu.nobuhiro@renesas.com>
 */

/* Dependency: <asm/addrspace.h> */

/* clock control */
pub const MSTPCR1: u32 = 0xFFC80038;

/* PORT */
pub const PORT_PSEL0: u32 = 0xFFEF0070;
pub const PORT_PSEL1: u32 = 0xFFEF0072;
pub const PORT_PSEL2: u32 = 0xFFEF0074;
pub const PORT_PSEL3: u32 = 0xFFEF0076;
pub const PORT_PSEL4: u32 = 0xFFEF0078;

pub const PORT_PACR: u32 = 0xFFEF0000;
pub const PORT_PCCR: u32 = 0xFFEF0004;
pub const PORT_PFCR: u32 = 0xFFEF000A;
pub const PORT_PGCR: u32 = 0xFFEF000C;
pub const PORT_PHCR: u32 = 0xFFEF000E;
pub const PORT_PICR: u32 = 0xFFEF0010;
pub const PORT_PJCR: u32 = 0xFFEF0012;
pub const PORT_PKCR: u32 = 0xFFEF0014;
pub const PORT_PLCR: u32 = 0xFFEF0016;
pub const PORT_PMCR: u32 = 0xFFEF0018;
pub const PORT_PNCR: u32 = 0xFFEF001A;

/* FPGA */
pub const CPLD_BOARD_ID_ERV_REG: u32 = 0xB1000000;
pub const CPLD_CPLD_CMD_REG: u32 = 0xB1000006;

/*
 * USB SH7763RDP board can use Host only.
 */
pub const USB_USBHSC: u32 = 0xFFEC80f0;

/* arch/sh/boards/renesas/sh7763rdp/irq.c */
unsafe extern "C" {
    pub fn init_sh7763rdp_IRQ();
    pub fn sh7763rdp_irq_demux(irq: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

/* __IO_PREFIX sh7763rdp; dependency: <asm/io_generic.h> */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
