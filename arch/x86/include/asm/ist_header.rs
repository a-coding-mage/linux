/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Include file for the interface to IST BIOS
 * Copyright 2002 Andy Grover <andrew.grover@intel.com>
 */

// Dependency provided by <uapi/asm/ist.h>.

extern "C" {
    pub static mut ist_info: ist_info;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
