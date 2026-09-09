/* SPDX-License-Identifier: (GPL-2.0 OR MIT) */
/*
 * Hirschmann Hellcreek TSN switch platform data.
 *
 * Copyright (C) 2020 Linutronix GmbH
 * Author Kurt Kanzenbach <kurt@linutronix.de>
 */

// Original dependency: #include <linux/types.h>

#[repr(C)]
pub struct hellcreek_platform_data {
    pub name: *const core::ffi::c_char, // Switch name
    pub num_ports: i32,                 // Amount of switch ports
    pub is_100_mbits: i32,              // Is it configured to 100 or 1000 mbit/s
    pub qbv_support: i32,               // Qbv support on front TSN ports
    pub qbv_on_cpu_port: i32,            // Qbv support on the CPU port
    pub qbu_support: i32,               // Qbu support on front TSN ports
    pub module_id: u16,                 // Module identificaton
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
