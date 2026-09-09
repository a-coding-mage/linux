/* SPDX-License-Identifier: GPL-2.0 */
/*
 * parport.h: platform-specific PC-style parport initialisation
 *
 * Copyright (C) 1999, 2000  Tim Waugh <tim@cyberelk.demon.co.uk>
 *
 * This file should only be included by drivers/parport/parport_pc.c.
 *
 * RZ: for use with Q40 and other ISA machines
 */

// C header guard: _ASM_M68K_PARPORT_H

// The C header undefines any prior insl/outsl definitions.
#[macro_export]
macro_rules! insl {
    ($port:expr, $buf:expr, $len:expr) => {
        isa_insb($port, $buf, ($len) << 2)
    };
}

#[macro_export]
macro_rules! outsl {
    ($port:expr, $buf:expr, $len:expr) => {
        isa_outsb($port, $buf, ($len) << 2)
    };
}

/* no dma, or IRQ autoprobing */
extern "C" {
    fn parport_pc_find_isa_ports(autoirq: ::core::ffi::c_int, autodma: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
}

unsafe fn parport_pc_find_nonpci_ports(
    autoirq: ::core::ffi::c_int,
    autodma: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !MACH_IS_Q40 {
        return 0; /* count=0 */
    }
    parport_pc_find_isa_ports(PARPORT_IRQ_NONE, PARPORT_DMA_NONE)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
