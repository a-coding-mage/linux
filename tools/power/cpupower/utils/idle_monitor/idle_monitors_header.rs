/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  (C) 2010,2011       Thomas Renninger <trenn@suse.de>, Novell Inc.
 *
 *  Based on the idea from Michael Matz <matz@suse.de>
 */

// C dependency intent:
// #define DEF(x) extern struct cpuidle_monitor x ##_monitor;
// #include "idle_monitors.def"
// #undef DEF
//
// The included .def file expands to extern declarations for symbols named
// `<x>_monitor` with type `struct cpuidle_monitor`.

unsafe extern "C" {
    pub static mut all_monitors: [*mut cpuidle_monitor; 0];
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
