// SPDX-License-Identifier: GPL-2.0
/*
 * idprom.c: Routines to load the idprom into kernel addresses and
 *           interpret the data contained within.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 * Sun3/3x models added by David Monro (davidm@psrg.cs.usyd.edu.au)
 */

// Linux and architecture-specific declarations are supplied by the surrounding kernel.

extern "C" {
    static mut idprom: *mut crate::idprom;
    fn prom_get_idprom(buf: *mut core::ffi::c_char, len: usize);
    fn prom_printf(fmt: *const core::ffi::c_char, ...);
    fn prom_halt() -> !;
    fn pr_info(fmt: *const core::ffi::c_char, ...);
}

static mut idprom_buffer: crate::idprom = unsafe { core::mem::zeroed() };

/* Here is the master table of Sun machines which use some implementation
 * of the Sparc CPU and have a meaningful IDPROM machtype value that we
 * know about.  See asm-sparc/machines.h for empirical constants.
 */
static mut Sun_Machines: [crate::Sun_Machine_Models; crate::NUM_SUN_MACHINES] = [
    crate::Sun_Machine_Models { name: "Sun 3/160 Series\0", id_machtype: crate::SM_SUN3 | crate::SM_3_160 },
    crate::Sun_Machine_Models { name: "Sun 3/50\0", id_machtype: crate::SM_SUN3 | crate::SM_3_50 },
    crate::Sun_Machine_Models { name: "Sun 3/260 Series\0", id_machtype: crate::SM_SUN3 | crate::SM_3_260 },
    crate::Sun_Machine_Models { name: "Sun 3/110 Series\0", id_machtype: crate::SM_SUN3 | crate::SM_3_110 },
    crate::Sun_Machine_Models { name: "Sun 3/60\0", id_machtype: crate::SM_SUN3 | crate::SM_3_60 },
    crate::Sun_Machine_Models { name: "Sun 3/E\0", id_machtype: crate::SM_SUN3 | crate::SM_3_E },
    crate::Sun_Machine_Models { name: "Sun 3/460 Series\0", id_machtype: crate::SM_SUN3X | crate::SM_3_460 },
    crate::Sun_Machine_Models { name: "Sun 3/80\0", id_machtype: crate::SM_SUN3X | crate::SM_3_80 },
];

unsafe fn display_system_type(machtype: u8) {
    let mut i: usize = 0;
    while i < crate::NUM_SUN_MACHINES {
        if Sun_Machines[i].id_machtype == machtype {
            if machtype != (crate::SM_SUN4M_OBP | 0x00) {
                pr_info(b"TYPE: %s\n\0".as_ptr() as *const _, Sun_Machines[i].name.as_ptr());
            }
            return;
        }
        i += 1;
    }

    prom_printf(b"IDPROM: Bogus id_machtype value, 0x%x\n\0".as_ptr() as *const _, machtype as core::ffi::c_uint);
    prom_halt();
}

pub unsafe fn sun3_get_model(model: *mut core::ffi::c_char) {
    let mut i: usize = 0;
    while i < crate::NUM_SUN_MACHINES {
        if Sun_Machines[i].id_machtype == (*idprom).id_machtype {
            crate::strcpy(model, Sun_Machines[i].name.as_ptr());
            return;
        }
        i += 1;
    }
}

/* Calculate the IDPROM checksum (xor of the data bytes). */
unsafe fn calc_idprom_cksum(idprom: *mut crate::idprom) -> u8 {
    let mut cksum: u8 = 0;
    let ptr = idprom as *mut u8;
    let mut i: u8 = 0;
    while i <= 0x0e {
        cksum ^= *ptr.add(i as usize);
        i += 1;
    }
    cksum
}

/* Create a local IDPROM copy, verify integrity, and display information. */
pub unsafe fn idprom_init() {
    prom_get_idprom(
        &mut idprom_buffer as *mut crate::idprom as *mut core::ffi::c_char,
        core::mem::size_of::<crate::idprom>(),
    );

    idprom = &mut idprom_buffer;

    if (*idprom).id_format != 0x01 {
        prom_printf(b"IDPROM: Unknown format type!\n\0".as_ptr() as *const _);
        prom_halt();
    }

    if (*idprom).id_cksum != calc_idprom_cksum(idprom) {
        prom_printf(
            b"IDPROM: Checksum failure (nvram=%x, calc=%x)!\n\0".as_ptr() as *const _,
            (*idprom).id_cksum,
            calc_idprom_cksum(idprom),
        );
        prom_halt();
    }

    display_system_type((*idprom).id_machtype);
    pr_info(b"Ethernet address: %pM\n\0".as_ptr() as *const _, (*idprom).id_ethaddr.as_ptr());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
