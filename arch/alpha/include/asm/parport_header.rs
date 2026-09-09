/* SPDX-License-Identifier: GPL-2.0 */
/*
 * parport.h: platform-specific PC-style parport initialisation
 *
 * Copyright (C) 1999, 2000  Tim Waugh <tim@cyberelk.demon.co.uk>
 *
 * This file should only be included by drivers/parport/parport_pc.c.
 */

// Header guard: _ASM_AXP_PARPORT_H

unsafe extern "C" {
    fn parport_pc_find_isa_ports(autoirq: i32, autodma: i32) -> i32;
}

unsafe fn parport_pc_find_nonpci_ports(autoirq: i32, autodma: i32) -> i32 {
    parport_pc_find_isa_ports(autoirq, autodma)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
