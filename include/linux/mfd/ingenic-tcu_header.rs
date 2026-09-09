/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Header file for the Ingenic JZ47xx TCU driver
 */

// Dependency intent: Linux bitops BIT() is represented directly below.

pub const TCU_REG_WDT_TDR: u32 = 0x00;
pub const TCU_REG_WDT_TCER: u32 = 0x04;
pub const TCU_REG_WDT_TCNT: u32 = 0x08;
pub const TCU_REG_WDT_TCSR: u32 = 0x0c;
pub const TCU_REG_TER: u32 = 0x10;
pub const TCU_REG_TESR: u32 = 0x14;
pub const TCU_REG_TECR: u32 = 0x18;
pub const TCU_REG_TSR: u32 = 0x1c;
pub const TCU_REG_TFR: u32 = 0x20;
pub const TCU_REG_TFSR: u32 = 0x24;
pub const TCU_REG_TFCR: u32 = 0x28;
pub const TCU_REG_TSSR: u32 = 0x2c;
pub const TCU_REG_TMR: u32 = 0x30;
pub const TCU_REG_TMSR: u32 = 0x34;
pub const TCU_REG_TMCR: u32 = 0x38;
pub const TCU_REG_TSCR: u32 = 0x3c;
pub const TCU_REG_TDFR0: u32 = 0x40;
pub const TCU_REG_TDHR0: u32 = 0x44;
pub const TCU_REG_TCNT0: u32 = 0x48;
pub const TCU_REG_TCSR0: u32 = 0x4c;
pub const TCU_REG_OST_DR: u32 = 0xe0;
pub const TCU_REG_OST_CNTL: u32 = 0xe4;
pub const TCU_REG_OST_CNTH: u32 = 0xe8;
pub const TCU_REG_OST_TCSR: u32 = 0xec;
pub const TCU_REG_TSTR: u32 = 0xf0;
pub const TCU_REG_TSTSR: u32 = 0xf4;
pub const TCU_REG_TSTCR: u32 = 0xf8;
pub const TCU_REG_OST_CNTHBUF: u32 = 0xfc;

pub const TCU_TCSR_RESERVED_BITS: u32 = 0x3f;
pub const TCU_TCSR_PARENT_CLOCK_MASK: u32 = 0x07;
pub const TCU_TCSR_PRESCALE_LSB: u32 = 3;
pub const TCU_TCSR_PRESCALE_MASK: u32 = 0x38;

pub const TCU_TCSR_PWM_SD: u32 = 1u32 << 9; // 0: Shutdown gracefully 1: abruptly
pub const TCU_TCSR_PWM_INITL_HIGH: u32 = 1u32 << 8; // Sets the initial output level
pub const TCU_TCSR_PWM_EN: u32 = 1u32 << 7; // PWM pin output enable

pub const TCU_WDT_TCER_TCEN: u32 = 1u32 << 0; // Watchdog timer enable

pub const TCU_CHANNEL_STRIDE: u32 = 0x10;

#[inline]
pub const fn TCU_REG_TDFRc(c: u32) -> u32 {
    TCU_REG_TDFR0 + c * TCU_CHANNEL_STRIDE
}

#[inline]
pub const fn TCU_REG_TDHRc(c: u32) -> u32 {
    TCU_REG_TDHR0 + c * TCU_CHANNEL_STRIDE
}

#[inline]
pub const fn TCU_REG_TCNTc(c: u32) -> u32 {
    TCU_REG_TCNT0 + c * TCU_CHANNEL_STRIDE
}

#[inline]
pub const fn TCU_REG_TCSRc(c: u32) -> u32 {
    TCU_REG_TCSR0 + c * TCU_CHANNEL_STRIDE
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
