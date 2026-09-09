/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2007-2009 ST-Ericsson AB
 *
 * ABX500 core access functions.
 * The abx500 interface is used for the Analog Baseband chips.
 *
 * Author: Mattias Wallin <mattias.wallin@stericsson.com>
 * Author: Mattias Nilsson <mattias.i.nilsson@stericsson.com>
 * Author: Bengt Jonsson <bengt.g.jonsson@stericsson.com>
 * Author: Rickard Andersson <rickard.andersson@stericsson.com>
 */

// Dependency supplied externally: linux/regulator/machine.h

use core::ffi::c_int;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

/**
 * struct abx500_init_setting
 * Initial value of the registers for driver to use during setup.
 */
#[repr(C)]
pub struct abx500_init_settings {
    pub bank: u8,
    pub reg: u8,
    pub setting: u8,
}

unsafe extern "C" {
    pub fn abx500_set_register_interruptible(
        dev: *mut device,
        bank: u8,
        reg: u8,
        value: u8,
    ) -> c_int;
    pub fn abx500_get_register_interruptible(
        dev: *mut device,
        bank: u8,
        reg: u8,
        value: *mut u8,
    ) -> c_int;
    pub fn abx500_get_register_page_interruptible(
        dev: *mut device,
        bank: u8,
        first_reg: u8,
        regvals: *mut u8,
        numregs: u8,
    ) -> c_int;
    pub fn abx500_set_register_page_interruptible(
        dev: *mut device,
        bank: u8,
        first_reg: u8,
        regvals: *mut u8,
        numregs: u8,
    ) -> c_int;
    /**
     * abx500_mask_and_set_register_inerruptible() - Modifies selected bits of a
     * target register
     *
     * @dev: The AB sub device.
     * @bank: The i2c bank number.
     * @bitmask: The bit mask to use.
     * @bitvalues: The new bit values.
     *
     * Updates the value of an AB register:
     * value -> ((value & ~bitmask) | (bitvalues & bitmask))
     */
    pub fn abx500_mask_and_set_register_interruptible(
        dev: *mut device,
        bank: u8,
        reg: u8,
        bitmask: u8,
        bitvalues: u8,
    ) -> c_int;
    pub fn abx500_get_chip_id(dev: *mut device) -> c_int;
    pub fn abx500_event_registers_startup_state_get(dev: *mut device, event: *mut u8) -> c_int;
    pub fn abx500_startup_irq_enabled(dev: *mut device, irq: u32) -> c_int;
}

#[repr(C)]
pub struct abx500_ops {
    pub get_chip_id: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub get_register: Option<unsafe extern "C" fn(*mut device, u8, u8, *mut u8) -> c_int>,
    pub set_register: Option<unsafe extern "C" fn(*mut device, u8, u8, u8) -> c_int>,
    pub get_register_page:
        Option<unsafe extern "C" fn(*mut device, u8, u8, *mut u8, u8) -> c_int>,
    pub set_register_page:
        Option<unsafe extern "C" fn(*mut device, u8, u8, *mut u8, u8) -> c_int>,
    pub mask_and_set_register: Option<unsafe extern "C" fn(*mut device, u8, u8, u8, u8) -> c_int>,
    pub event_registers_startup_state_get: Option<unsafe extern "C" fn(*mut device, *mut u8) -> c_int>,
    pub startup_irq_enabled: Option<unsafe extern "C" fn(*mut device, u32) -> c_int>,
    pub dump_all_banks: Option<unsafe extern "C" fn(*mut device)>,
}

unsafe extern "C" {
    pub fn abx500_register_ops(core_dev: *mut device, ops: *mut abx500_ops) -> c_int;
    pub fn abx500_remove_ops(dev: *mut device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
