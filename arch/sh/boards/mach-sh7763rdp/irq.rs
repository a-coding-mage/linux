// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/sh/boards/renesas/sh7763rdp/irq.c
 *
 * Renesas Solutions SH7763RDP Support.
 *
 * Copyright (C) 2008 Renesas Solutions Corp.
 * Copyright (C) 2008  Nobuhiro Iwamatsu <iwamatsu.nobuhiro@renesas.com>
 */

// External symbols supplied by the Linux I/O and initialization dependencies.
extern "C" {
    fn __raw_writel(value: u32, address: u32);
    fn __raw_readl(address: u32) -> u32;
}

const INTC_BASE: u32 = 0xFFD00000;
const INTC_INT2PRI7: u32 = INTC_BASE + 0x4001C;
const INTC_INT2MSKCR: u32 = INTC_BASE + 0x4003C;
const INTC_INT2MSKCR1: u32 = INTC_BASE + 0x400D4;

/*
 * Initialize IRQ setting
 */
pub unsafe extern "C" fn init_sh7763rdp_IRQ() {
    /* GPIO enabled */
    __raw_writel(1u32 << 25, INTC_INT2MSKCR);

    /* enable GPIO interrupts */
    __raw_writel(
        (__raw_readl(INTC_INT2PRI7) & 0xFF00FFFF) | 0x000F0000,
        INTC_INT2PRI7,
    );

    /* USBH enabled */
    __raw_writel(1u32 << 17, INTC_INT2MSKCR1);

    /* GETHER enabled */
    __raw_writel(1u32 << 16, INTC_INT2MSKCR1);

    /* DMAC enabled */
    __raw_writel(1u32 << 8, INTC_INT2MSKCR);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
