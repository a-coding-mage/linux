/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * eisa_bus.h interface between the eisa BA driver and the bus enumerator
 *
 * Copyright (c) 2002 Daniel Engstrom <5116@telia.com>
 */

use core::ffi::{c_int, c_ulong};

/// Opaque representation of the externally supplied C `struct resource`.
#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

extern "C" {
    pub fn eisa_make_irq_level(num: c_int);
    pub fn eisa_make_irq_edge(num: c_int);
    pub fn eisa_enumerator(
        eeprom_addr: c_ulong,
        io_parent: *mut resource,
        mem_parent: *mut resource,
    ) -> c_int;
    pub fn eisa_eeprom_init(addr: c_ulong) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
