// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/boards/dreamcast/setup.c
 *
 * Hardware support for the Sega Dreamcast.
 *
 * Copyright (c) 2001, 2002 M. R. Brown <mrbrown@linuxdc.org>
 * Copyright (c) 2002, 2003, 2004 Paul Mundt <lethal@linux-sh.org>
 *
 * This file is part of the LinuxDC project (www.linuxdc.org)
 *
 * This file originally bore the message (with enclosed-$):
 *	Id: setup_dc.c,v 1.5 2001/05/24 05:09:16 mrbrown Exp
 *	SEGA Dreamcast support
 */

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    fn __set_io_port_base(base: usize);
    fn systemasic_irq_demux(irq: i32) -> i32;
    fn systemasic_irq_init();
    static P2SEG: usize;
}

// The machine-vector type and its initialization ABI are supplied externally.
extern "C" {
    type sh_machine_vector;
}

unsafe extern "C" fn dreamcast_setup(_cmdline_p: *mut *mut core::ffi::c_char) {
    /* GAPS PCI bridge assumes P2 area relative addresses. */
    __set_io_port_base(P2SEG);
}

#[no_mangle]
static mut mv_dreamcast: sh_machine_vector = sh_machine_vector {
    mv_name: c"Sega Dreamcast".as_ptr(),
    mv_setup: Some(dreamcast_setup),
    mv_irq_demux: Some(systemasic_irq_demux),
    mv_init_irq: Some(systemasic_irq_init),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
