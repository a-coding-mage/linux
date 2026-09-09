/* SPDX-License-Identifier: GPL-2.0 */
/*
 *
 * parport.h: ia32-compatible parport initialisation
 *
 * This file should only be included by drivers/parport/parport_pc.c.
 */

fn parport_pc_find_nonpci_ports(autoirq: i32, autodma: i32) -> i32 {
    /* nothing ! */
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
