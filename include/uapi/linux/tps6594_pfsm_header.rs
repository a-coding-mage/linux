/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Userspace ABI for TPS6594 PMIC Pre-configurable Finite State Machine
 *
 * Copyright (C) 2023 BayLibre Incorporated - https://www.baylibre.com/
 */

use core::mem::size_of;

/**
 * struct pmic_state_opt - PMIC state options
 * @gpio_retention: if enabled, power rails associated with GPIO retention remain active
 * @ddr_retention: if enabled, power rails associated with DDR retention remain active
 * @mcu_only_startup_dest: if enabled, startup destination state is MCU_ONLY
 */
#[repr(C)]
pub struct pmic_state_opt {
    pub gpio_retention: u8,
    pub ddr_retention: u8,
    pub mcu_only_startup_dest: u8,
}

/* Linux ioctl encoding used by _IO and _IOW. */
const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_WRITE: u32 = 1;

const fn ioc(direction: u32, ioctl_type: u32, number: u32, size: u32) -> u32 {
    (direction << IOC_DIRSHIFT)
        | (ioctl_type << IOC_TYPESHIFT)
        | (number << IOC_NRSHIFT)
        | (size << IOC_SIZESHIFT)
}

const fn io(ioctl_type: u32, number: u32) -> u32 {
    ioc(0, ioctl_type, number, 0)
}

const fn iow<T>(ioctl_type: u32, number: u32) -> u32 {
    ioc(IOC_WRITE, ioctl_type, number, size_of::<T>() as u32)
}

/* Commands */
pub const PMIC_BASE: u32 = b'P' as u32;

pub const PMIC_GOTO_STANDBY: u32 = io(PMIC_BASE, 0);
pub const PMIC_GOTO_LP_STANDBY: u32 = io(PMIC_BASE, 1);
pub const PMIC_UPDATE_PGM: u32 = io(PMIC_BASE, 2);
pub const PMIC_SET_ACTIVE_STATE: u32 = io(PMIC_BASE, 3);
pub const PMIC_SET_MCU_ONLY_STATE: u32 = iow::<pmic_state_opt>(PMIC_BASE, 4);
pub const PMIC_SET_RETENTION_STATE: u32 = iow::<pmic_state_opt>(PMIC_BASE, 5);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
