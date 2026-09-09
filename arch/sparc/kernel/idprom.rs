// SPDX-License-Identifier: GPL-2.0
/*
 * idprom.c: Routines to load the idprom into kernel addresses and
 *           interpret the data contained within.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 */

// Dependencies supplied by the kernel headers:
// linux/kernel.h, linux/types.h, linux/init.h, linux/export.h,
// linux/etherdevice.h, asm/oplib.h, and asm/idprom.h.

use core::ptr;

pub static mut idprom: *mut idprom = ptr::null_mut();

static mut idprom_buffer: idprom = unsafe { core::mem::zeroed() };

#[cfg(CONFIG_SPARC32)]
mod sparc32 {
    use super::*;

    // Fun with Sun released architectures.  Constants and the model type are
    // supplied by asm/machines.h.
    static mut Sun_Machines: [Sun_Machine_Models; 5] = [
        Sun_Machine_Models {
            name: "Leon3 System-on-a-Chip",
            id_machtype: M_LEON | M_LEON3_SOC,
        },
        Sun_Machine_Models {
            name: "Sun4m SparcSystem600",
            id_machtype: SM_SUN4M | SM_4M_SS60,
        },
        Sun_Machine_Models {
            name: "Sun4m SparcStation10/20",
            id_machtype: SM_SUN4M | SM_4M_SS50,
        },
        Sun_Machine_Models {
            name: "Sun4m SparcStation5",
            id_machtype: SM_SUN4M | SM_4M_SS40,
        },
        Sun_Machine_Models {
            name: "Sun4M OBP based system",
            id_machtype: SM_SUN4M_OBP | 0x0,
        },
    ];

    unsafe fn display_system_type(machtype: u8) {
        let mut sysname = [0i8; 128];

        for i in 0..Sun_Machines.len() {
            if Sun_Machines[i].id_machtype == machtype {
                if machtype != (SM_SUN4M_OBP | 0x00)
                    || prom_getproperty(
                        prom_root_node,
                        b"banner-name\0".as_ptr() as *const i8,
                        sysname.as_mut_ptr(),
                        sysname.len(),
                    ) <= 0
                {
                    printk(KERN_WARNING, b"TYPE: %s\n\0".as_ptr() as *const i8, Sun_Machines[i].name);
                } else {
                    printk(KERN_WARNING, b"TYPE: %s\n\0".as_ptr() as *const i8, sysname.as_ptr());
                }
                return;
            }
        }

        prom_printf(
            b"IDPROM: Warning, bogus id_machtype value, 0x%x\n\0".as_ptr() as *const i8,
            machtype,
        );
    }
}

#[cfg(not(CONFIG_SPARC32))]
unsafe fn display_system_type(_machtype: u8) {}

pub unsafe fn arch_get_platform_mac_address() -> *mut u8 {
    (*idprom).id_ethaddr.as_mut_ptr()
}

/* Calculate the IDPROM checksum (xor of the data bytes). */
unsafe fn calc_idprom_cksum(idprom: *mut idprom) -> u8 {
    let mut cksum: u8 = 0;
    let ptr = idprom as *mut u8;

    for i in 0..=0x0e {
        cksum ^= ptr.add(i).read();
    }

    cksum
}

/* Create a local IDPROM copy, verify integrity, and display information. */
pub unsafe fn idprom_init() {
    prom_get_idprom(
        (&mut idprom_buffer as *mut idprom) as *mut i8,
        core::mem::size_of::<idprom>(),
    );

    idprom = &mut idprom_buffer;

    if (*idprom).id_format != 0x01 {
        prom_printf(b"IDPROM: Warning, unknown format type!\n\0".as_ptr() as *const i8);
    }

    if (*idprom).id_cksum != calc_idprom_cksum(idprom) {
        prom_printf(
            b"IDPROM: Warning, checksum failure (nvram=%x, calc=%x)!\n\0".as_ptr()
                as *const i8,
            (*idprom).id_cksum,
            calc_idprom_cksum(idprom),
        );
    }

    display_system_type((*idprom).id_machtype);

    printk(
        KERN_WARNING,
        b"Ethernet address: %pM\n\0".as_ptr() as *const i8,
        (*idprom).id_ethaddr.as_ptr(),
    );
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
