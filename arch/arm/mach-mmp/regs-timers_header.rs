/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *   Timers Module
 */

pub const TMR_CCR: u32 = 0x0000;
pub const fn TMR_TN_MM(n: u32, m: u32) -> u32 {
    0x0004 + (n << 3) + ((n + m) << 2)
}
pub const fn TMR_CR(n: u32) -> u32 {
    0x0028 + (n << 2)
}
pub const fn TMR_SR(n: u32) -> u32 {
    0x0034 + (n << 2)
}
pub const fn TMR_IER(n: u32) -> u32 {
    0x0040 + (n << 2)
}
pub const fn TMR_PLVR(n: u32) -> u32 {
    0x004c + (n << 2)
}
pub const fn TMR_PLCR(n: u32) -> u32 {
    0x0058 + (n << 2)
}
pub const TMR_WMER: u32 = 0x0064;
pub const TMR_WMR: u32 = 0x0068;
pub const TMR_WVR: u32 = 0x006c;
pub const TMR_WSR: u32 = 0x0070;
pub const fn TMR_ICR(n: u32) -> u32 {
    0x0074 + (n << 2)
}
pub const TMR_WICR: u32 = 0x0080;
pub const TMR_CER: u32 = 0x0084;
pub const TMR_CMR: u32 = 0x0088;
pub const fn TMR_ILR(n: u32) -> u32 {
    0x008c + (n << 2)
}
pub const TMR_WCR: u32 = 0x0098;
pub const TMR_WFAR: u32 = 0x009c;
pub const TMR_WSAR: u32 = 0x00A0;
pub const fn TMR_CVWR(n: u32) -> u32 {
    0x00A4 + (n << 2)
}

pub const fn TMR_CCR_CS_0(x: u32) -> u32 {
    (x & 0x3) << 0
}
pub const fn TMR_CCR_CS_1(x: u32) -> u32 {
    (x & 0x7) << 2
}
pub const fn TMR_CCR_CS_2(x: u32) -> u32 {
    (x & 0x3) << 5
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
