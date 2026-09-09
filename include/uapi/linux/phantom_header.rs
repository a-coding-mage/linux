/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 *  Copyright (C) 2005-2007 Jiri Slaby <jirislaby@gmail.com>
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 */

/* PHN_(G/S)ET_REG param */
#[repr(C)]
pub struct phm_reg {
    pub reg: u32,
    pub value: u32,
}

/* PHN_(G/S)ET_REGS param */
#[repr(C)]
pub struct phm_regs {
    pub count: u32,
    pub mask: u32,
    pub values: [u32; 8],
}

pub const PH_IOC_MAGIC: u8 = b'p';
pub const PHN_GET_REG: _IOWR_TYPE = _IOWR(PH_IOC_MAGIC, 0, *mut phm_reg);
pub const PHN_SET_REG: _IOW_TYPE = _IOW(PH_IOC_MAGIC, 1, *mut phm_reg);
pub const PHN_GET_REGS: _IOWR_TYPE = _IOWR(PH_IOC_MAGIC, 2, *mut phm_regs);
pub const PHN_SET_REGS: _IOW_TYPE = _IOW(PH_IOC_MAGIC, 3, *mut phm_regs);
/* this ioctl tells the driver, that the caller is not OpenHaptics and might
 * use improved registers update (no more phantom switchoffs when using
 * libphantom) */
pub const PHN_NOT_OH: _IO_TYPE = _IO(PH_IOC_MAGIC, 4);
pub const PHN_GETREG: _IOWR_TYPE = _IOWR(PH_IOC_MAGIC, 5, phm_reg);
pub const PHN_SETREG: _IOW_TYPE = _IOW(PH_IOC_MAGIC, 6, phm_reg);
pub const PHN_GETREGS: _IOWR_TYPE = _IOWR(PH_IOC_MAGIC, 7, phm_regs);
pub const PHN_SETREGS: _IOW_TYPE = _IOW(PH_IOC_MAGIC, 8, phm_regs);

pub const PHN_CONTROL: u32 = 0x6; /* control byte in iaddr space */
pub const PHN_CTL_AMP: u32 = 0x1; /*   switch after torques change */
pub const PHN_CTL_BUT: u32 = 0x2; /*   is button switched */
pub const PHN_CTL_IRQ: u32 = 0x10; /*   is irq enabled */

pub const PHN_ZERO_FORCE: u32 = 2048; /* zero torque on motor */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
