/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Copyright (C) 2007 Nobuhiro Iwamatsu
 *
 * NTT COMWARE L-BOX RE2 support
 */

/* Dependency: linux/sh_intc.h */

pub const IRQ_CF1: _ = evt2irq(0x320); /* CF1 */
pub const IRQ_CF0: _ = evt2irq(0x340); /* CF0 */
pub const IRQ_INTD: _ = evt2irq(0x360); /* INTD */
pub const IRQ_ETH1: _ = evt2irq(0x380); /* Ether1 */
pub const IRQ_ETH0: _ = evt2irq(0x3a0); /* Ether0 */
pub const IRQ_INTA: _ = evt2irq(0x3c0); /* INTA */

unsafe extern "C" {
    pub fn init_lboxre2_IRQ();
}

/* __IO_PREFIX lboxre2 */
/* Dependency: asm/io_generic.h */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
