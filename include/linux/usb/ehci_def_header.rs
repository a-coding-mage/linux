// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (c) 2001-2002 by David Brownell
 */

// Dependency: linux/usb/ehci-dbgp.h

/* EHCI register interface, corresponds to EHCI Revision 0.95 specification */

#[repr(C)]
pub struct ehci_caps {
    pub hc_capbase: u32,
    pub hcs_params: u32,
    pub hcc_params: u32,
    pub portroute: [u8; 8],
}

#[inline]
pub fn HC_LENGTH(ehci: *const ehci_caps, p: u32) -> u32 {
    (0x00ffu32 & (p >> (if ehci_big_endian_capbase(ehci) { 24 } else { 0 })))
}
#[inline]
pub fn HC_VERSION(ehci: *const ehci_caps, p: u32) -> u32 {
    (0xffffu32 & (p >> (if ehci_big_endian_capbase(ehci) { 0 } else { 16 })))
}
#[inline] pub const fn HCS_DEBUG_PORT(p: u32) -> u32 { (p >> 20) & 0xf }
#[inline] pub const fn HCS_INDICATOR(p: u32) -> u32 { p & (1 << 16) }
#[inline] pub const fn HCS_N_CC(p: u32) -> u32 { (p >> 12) & 0xf }
#[inline] pub const fn HCS_N_PCC(p: u32) -> u32 { (p >> 8) & 0xf }
#[inline] pub const fn HCS_PORTROUTED(p: u32) -> u32 { p & (1 << 7) }
#[inline] pub const fn HCS_PPC(p: u32) -> u32 { p & (1 << 4) }
#[inline] pub const fn HCS_N_PORTS(p: u32) -> u32 { p & 0xf }
pub const HCS_N_PORTS_MAX: usize = 15;
#[inline] pub const fn HCC_32FRAME_PERIODIC_LIST(p: u32) -> u32 { p & (1 << 19) }
#[inline] pub const fn HCC_PER_PORT_CHANGE_EVENT(p: u32) -> u32 { p & (1 << 18) }
#[inline] pub const fn HCC_LPM(p: u32) -> u32 { p & (1 << 17) }
#[inline] pub const fn HCC_HW_PREFETCH(p: u32) -> u32 { p & (1 << 16) }
#[inline] pub const fn HCC_EXT_CAPS(p: u32) -> u32 { (p >> 8) & 0xff }
#[inline] pub const fn HCC_ISOC_CACHE(p: u32) -> u32 { p & (1 << 7) }
#[inline] pub const fn HCC_ISOC_THRES(p: u32) -> u32 { (p >> 4) & 0x7 }
#[inline] pub const fn HCC_CANPARK(p: u32) -> u32 { p & (1 << 2) }
#[inline] pub const fn HCC_PGM_FRAMELISTLEN(p: u32) -> u32 { p & (1 << 1) }
#[inline] pub const fn HCC_64BIT_ADDR(p: u32) -> u32 { p & 1 }

#[repr(C)]
pub struct ehci_regs {
    pub command: u32,
    pub status: u32,
    pub intr_enable: u32,
    pub frame_index: u32,
    pub segment: u32,
    pub frame_list: u32,
    pub async_next: u32,
    pub reserved1: [u32; 2],
    pub txfill_tuning: u32,
    pub reserved2: [u32; 6],
    pub configured_flag: u32,
    pub port_union: ehci_port_union,
    pub hostpc_union: ehci_hostpc_union,
    pub reserved5: [u32; 2],
    pub usbmode_ex: u32,
}

#[repr(C)]
pub union ehci_port_union {
    pub port_status: [u32; HCS_N_PORTS_MAX],
    pub mode: ehci_port_mode,
}
#[repr(C)] pub struct ehci_port_mode { pub reserved3: [u32; 9], pub usbmode: u32 }
#[repr(C)]
pub union ehci_hostpc_union {
    pub hostpc_struct: ehci_hostpc_struct,
    pub brcm_insnreg: [u32; 4],
}
#[repr(C)] pub struct ehci_hostpc_struct { pub reserved4: u32, pub hostpc: [u32; HCS_N_PORTS_MAX] }

pub const CMD_HIRD: u32 = 0xf << 24;
pub const CMD_PPCEE: u32 = 1 << 15; pub const CMD_FSP: u32 = 1 << 14;
pub const CMD_ASPE: u32 = 1 << 13; pub const CMD_PSPE: u32 = 1 << 12;
pub const CMD_PARK: u32 = 1 << 11;
#[inline] pub const fn CMD_PARK_CNT(c: u32) -> u32 { (c >> 8) & 3 }
pub const CMD_LRESET: u32 = 1 << 7; pub const CMD_IAAD: u32 = 1 << 6;
pub const CMD_ASE: u32 = 1 << 5; pub const CMD_PSE: u32 = 1 << 4;
pub const CMD_RESET: u32 = 1 << 1; pub const CMD_RUN: u32 = 1;
pub const STS_PPCE_MASK: u32 = 0xff << 16; pub const STS_ASS: u32 = 1 << 15;
pub const STS_PSS: u32 = 1 << 14; pub const STS_RECL: u32 = 1 << 13; pub const STS_HALT: u32 = 1 << 12;
pub const STS_IAA: u32 = 1 << 5; pub const STS_FATAL: u32 = 1 << 4; pub const STS_FLR: u32 = 1 << 3;
pub const STS_PCD: u32 = 1 << 2; pub const STS_ERR: u32 = 1 << 1; pub const STS_INT: u32 = 1;
pub const TXFIFO_DEFAULT: u32 = 8 << 16; pub const FLAG_CF: u32 = 1;

pub const PORTSC_SUSPEND_STS_ACK: u32 = 0; pub const PORTSC_SUSPEND_STS_NYET: u32 = 1;
pub const PORTSC_SUSPEND_STS_STALL: u32 = 2; pub const PORTSC_SUSPEND_STS_ERR: u32 = 3;
pub const PORT_DEV_ADDR: u32 = 0x7f << 25; pub const PORT_SSTS: u32 = 3 << 23;
pub const PORT_WKOC_E: u32 = 1 << 22; pub const PORT_WKDISC_E: u32 = 1 << 21; pub const PORT_WKCONN_E: u32 = 1 << 20;
#[inline] pub const fn PORT_TEST(x: u32) -> u32 { (x & 0xf) << 16 }
pub const PORT_TEST_PKT: u32 = PORT_TEST(4); pub const PORT_TEST_FORCE: u32 = PORT_TEST(5);
pub const PORT_LED_OFF: u32 = 0; pub const PORT_LED_AMBER: u32 = 1 << 14; pub const PORT_LED_GREEN: u32 = 2 << 14; pub const PORT_LED_MASK: u32 = 3 << 14;
pub const PORT_OWNER: u32 = 1 << 13; pub const PORT_POWER: u32 = 1 << 12;
#[inline] pub const fn PORT_USB11(x: u32) -> bool { (x & (3 << 10)) == (1 << 10) }
pub const PORT_LS_MASK: u32 = 3 << 10; pub const PORT_LPM: u32 = 1 << 9; pub const PORT_RESET: u32 = 1 << 8;
pub const PORT_SUSPEND: u32 = 1 << 7; pub const PORT_RESUME: u32 = 1 << 6; pub const PORT_OCC: u32 = 1 << 5; pub const PORT_OC: u32 = 1 << 4;
pub const PORT_PEC: u32 = 1 << 3; pub const PORT_PE: u32 = 1 << 2; pub const PORT_CSC: u32 = 1 << 1; pub const PORT_CONNECT: u32 = 1;
pub const PORT_RWC_BITS: u32 = PORT_CSC | PORT_PEC | PORT_OCC;
pub const USBMODE_SDIS: u32 = 1 << 3; pub const USBMODE_BE: u32 = 1 << 2; pub const USBMODE_CM_HC: u32 = 3; pub const USBMODE_CM_IDLE: u32 = 0;
pub const HOSTPC_PHCD: u32 = 1 << 22; pub const HOSTPC_PSPD: u32 = 3 << 25;
pub const USBMODE_EX_VBPS: u32 = 1 << 5; pub const USBMODE_EX_HC: u32 = 3;

extern "C" { fn ehci_big_endian_capbase(ehci: *const ehci_caps) -> bool; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
