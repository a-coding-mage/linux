/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Definitions for the SCOOP interface found on various Sharp PDAs
 *
 *  Copyright (c) 2004 Richard Purdie
 */

pub const SCOOP_MCR: u16 = 0x00;
pub const SCOOP_CDR: u16 = 0x04;
pub const SCOOP_CSR: u16 = 0x08;
pub const SCOOP_CPR: u16 = 0x0C;
pub const SCOOP_CCR: u16 = 0x10;
pub const SCOOP_IRR: u16 = 0x14;
pub const SCOOP_IRM: u16 = 0x14;
pub const SCOOP_IMR: u16 = 0x18;
pub const SCOOP_ISR: u16 = 0x1C;
pub const SCOOP_GPCR: u16 = 0x20;
pub const SCOOP_GPWR: u16 = 0x24;
pub const SCOOP_GPRR: u16 = 0x28;

pub const SCOOP_CPR_OUT: u16 = 1 << 7;
pub const SCOOP_CPR_SD_3V: u16 = 1 << 2;
pub const SCOOP_CPR_CF_XV: u16 = 1 << 1;
pub const SCOOP_CPR_CF_3V: u16 = 1 << 0;

pub const SCOOP_GPCR_PA22: u16 = 1 << 12;
pub const SCOOP_GPCR_PA21: u16 = 1 << 11;
pub const SCOOP_GPCR_PA20: u16 = 1 << 10;
pub const SCOOP_GPCR_PA19: u16 = 1 << 9;
pub const SCOOP_GPCR_PA18: u16 = 1 << 8;
pub const SCOOP_GPCR_PA17: u16 = 1 << 7;
pub const SCOOP_GPCR_PA16: u16 = 1 << 6;
pub const SCOOP_GPCR_PA15: u16 = 1 << 5;
pub const SCOOP_GPCR_PA14: u16 = 1 << 4;
pub const SCOOP_GPCR_PA13: u16 = 1 << 3;
pub const SCOOP_GPCR_PA12: u16 = 1 << 2;
pub const SCOOP_GPCR_PA11: u16 = 1 << 1;

#[repr(C)]
pub struct scoop_config {
    pub io_out: u16,
    pub io_dir: u16,
    pub suspend_clr: u16,
    pub suspend_set: u16,
    pub gpio_base: i32,
}

/* Structure for linking scoop devices to PCMCIA sockets */
#[repr(C)]
pub struct scoop_pcmcia_dev {
    pub dev: *mut device, /* Pointer to this socket's scoop device */
    pub irq: i32,         /* irq for socket */
    pub cd_irq: i32,
    pub cd_irq_str: *const core::ffi::c_char,
    pub keep_vs: u8,
    pub keep_rd: u8,
}

#[repr(C)]
pub struct scoop_pcmcia_config {
    pub devs: *mut scoop_pcmcia_dev,
    pub num_devs: i32,
    pub power_ctrl: Option<unsafe extern "C" fn(scoop: *mut device, cpr: u16, nr: i32)>,
}

/* Opaque type supplied by the including environment. */
pub enum device {}

extern "C" {
    pub static mut platform_scoop_config: *mut scoop_pcmcia_config;

    pub fn reset_scoop(dev: *mut device);
    pub fn read_scoop_reg(dev: *mut device, reg: u16) -> u16;
    pub fn write_scoop_reg(dev: *mut device, reg: u16, data: u16);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
