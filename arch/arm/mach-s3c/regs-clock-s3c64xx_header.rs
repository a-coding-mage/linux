/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2008 Openmoko, Inc.
 * Copyright 2008 Simtec Electronics
 *	Ben Dooks <ben@simtec.co.uk>
 *	http://armlinux.simtec.co.uk/
 *
 * S3C64XX clock register definitions
 */

/*
 * FIXME: Remove remaining definitions
 */

/// Equivalent to the C `S3C_CLKREG(x)` macro.
#[macro_export]
macro_rules! S3C_CLKREG {
    ($x:expr) => {
        S3C_VA_SYS + ($x)
    };
}

pub const S3C_PCLK_GATE: usize = S3C_CLKREG!(0x34);
pub const S3C6410_CLK_SRC2: usize = S3C_CLKREG!(0x10C);
pub const S3C_MEM_SYS_CFG: usize = S3C_CLKREG!(0x120);

/* PCLK GATE Registers */
pub const S3C_CLKCON_PCLK_UART3: u32 = 1 << 4;
pub const S3C_CLKCON_PCLK_UART2: u32 = 1 << 3;
pub const S3C_CLKCON_PCLK_UART1: u32 = 1 << 2;
pub const S3C_CLKCON_PCLK_UART0: u32 = 1 << 1;

/* MEM_SYS_CFG */
pub const MEM_SYS_CFG_INDEP_CF: u32 = 0x4000;
pub const MEM_SYS_CFG_EBI_FIX_PRI_CFCON: u32 = 0x30;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
