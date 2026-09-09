/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Interface for Surface ACPI Notify (SAN) driver.
 *
 * Provides access to discrete GPU notifications sent from ACPI via the SAN
 * driver, which are not handled by this driver directly.
 *
 * Copyright (C) 2019-2020 Maximilian Luz <luzmaximilian@gmail.com>
 */

// C dependencies: <linux/notifier.h>, <linux/types.h>

/**
 * struct san_dgpu_event - Discrete GPU ACPI event.
 * @category: Category of the event.
 * @target:   Target ID of the event source.
 * @command:  Command ID of the event.
 * @instance: Instance ID of the event source.
 * @length:   Length of the event's payload data (in bytes).
 * @payload:  Pointer to the event's payload data.
 */
#[repr(C)]
pub struct san_dgpu_event {
    pub category: u8,
    pub target: u8,
    pub command: u8,
    pub instance: u8,
    pub length: u16,
    pub payload: *mut u8,
}

// External types supplied by the Linux kernel dependencies.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

extern "C" {
    pub fn san_client_link(client: *mut device) -> core::ffi::c_int;
    pub fn san_dgpu_notifier_register(nb: *mut notifier_block) -> core::ffi::c_int;
    pub fn san_dgpu_notifier_unregister(nb: *mut notifier_block) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
