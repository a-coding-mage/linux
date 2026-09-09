/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  NXP (Philips) SCC+++(SCN+++) serial driver
 *
 *  Copyright (C) 2012 Alexander Shiyan <shc_work@mail.ru>
 *
 *  Based on sc26xx.c, by Thomas Bogendörfer (tsbogend@alpha.franken.de)
 */

pub const SCCNXP_MAX_UARTS: usize = 2;

/* Output lines */
pub const LINE_OP0: u32 = 1;
pub const LINE_OP1: u32 = 2;
pub const LINE_OP2: u32 = 3;
pub const LINE_OP3: u32 = 4;
pub const LINE_OP4: u32 = 5;
pub const LINE_OP5: u32 = 6;
pub const LINE_OP6: u32 = 7;
pub const LINE_OP7: u32 = 8;

/* Input lines */
pub const LINE_IP0: u32 = 9;
pub const LINE_IP1: u32 = 10;
pub const LINE_IP2: u32 = 11;
pub const LINE_IP3: u32 = 12;
pub const LINE_IP4: u32 = 13;
pub const LINE_IP5: u32 = 14;
pub const LINE_IP6: u32 = 15;

/* Signals */
pub const DTR_OP: u32 = 0; /* DTR */
pub const RTS_OP: u32 = 4; /* RTS */
pub const DSR_IP: u32 = 8; /* DSR */
pub const CTS_IP: u32 = 12; /* CTS */
pub const DCD_IP: u32 = 16; /* DCD */
pub const RNG_IP: u32 = 20; /* RNG */

pub const DIR_OP: u32 = 24; /* Special signal for control RS-485.
                             * Goes high when transmit,
                             * then goes low.
                             */

/* Routing control signal 'sig' to line 'line' */
#[inline]
pub const fn MCTRL_SIG(sig: u32, line: u32) -> u32 {
    line << sig
}

/*
 * Example board initialization data:
 *
 * static struct resource sc2892_resources[] = {
 *  DEFINE_RES_MEM(UART_PHYS_START, 0x10),
 *  DEFINE_RES_IRQ(IRQ_EXT2),
 * };
 *
 * static struct sccnxp_pdata sc2892_info = {
 *  .mctrl_cfg[0]  = MCTRL_SIG(DIR_OP, LINE_OP0),
 *  .mctrl_cfg[1]  = MCTRL_SIG(DIR_OP, LINE_OP1),
 * };
 *
 * static struct platform_device sc2892 = {
 *  .name   = "sc2892",
 *  .id = -1,
 *  .resource   = sc2892_resources,
 *  .num_resources  = ARRAY_SIZE(sc2892_resources),
 *  .dev = {
 *      .platform_data  = &sc2892_info,
 *  },
 * };
 */

/* SCCNXP platform data structure */
#[repr(C)]
pub struct sccnxp_pdata {
    /* Shift for A0 line */
    pub reg_shift: u8,
    /* Modem control lines configuration */
    pub mctrl_cfg: [u32; SCCNXP_MAX_UARTS],
    /* Timer value for polling mode (usecs) */
    pub poll_time_us: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
