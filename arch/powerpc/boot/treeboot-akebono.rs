// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright © 2013 Tony Breeds IBM Corporation
 * Copyright © 2013 Alistair Popple IBM Corporation
 *
 * Based on earlier code:
 *   Copyright (C) Paul Mackerras 1997.
 *
 *   Matt Porter <mporter@kernel.crashing.org>
 *   Copyright 2002-2005 MontaVista Software Inc.
 *
 *   Eugene Surovegin <eugene.surovegin@zultys.com> or <ebs@ebshome.net>
 *   Copyright (c) 2003, 2004 Zultys Technologies
 *
 *    Copyright 2007 David Gibson, IBM Corporation.
 *    Copyright 2010 Ben. Herrenschmidt, IBM Corporation.
 *    Copyright © 2011 David Kleikamp IBM Corporation
 */

// Dependencies supplied by the surrounding platform implementation.
// BSS_STACK(4096);

const SPRN_PIR: u32 = 0x11e; // Processor Identification Register
const USERDATA_LEN: usize = 256; // Length of userdata passed in by PIBS
const MAX_RANKS: u32 = 0x4;
const DDR3_MR0CF: u32 = 0x8001_0011;
const CCTL0_MCO2: u32 = 0x8000_080f;
const CCTL0_MCO3: u32 = 0x8000_0810;
const CCTL0_MCO4: u32 = 0x8000_0811;
const CCTL0_MCO5: u32 = 0x8000_0812;
const CCTL0_MCO6: u32 = 0x8000_0813;

static mut ibm_akebono_memsize: u64 = 0;
static mut mac_addr: u64 = 0;

unsafe fn ibm_akebono_detect_memsize() -> u64 {
    let mut reg: u32;
    let mut i: u32 = 0;
    let mut memsize: u64 = 0;

    while i < MAX_RANKS {
        reg = mfdcrx(DDR3_MR0CF.wrapping_add(i));

        if (reg & 1) == 0 {
            i = i.wrapping_add(1);
            continue;
        }

        reg &= 0x0000_f000;
        reg >>= 12;
        memsize = memsize.wrapping_add(0x800000u64 << reg);
        i = i.wrapping_add(1);
    }

    memsize
}

unsafe fn ibm_akebono_fixups() {
    let emac: *mut core::ffi::c_void;
    let reg: u32;

    dt_fixup_memory(0u64, ibm_akebono_memsize);

    /* Fixup the SD timeout frequency */
    mtdcrx(CCTL0_MCO4, 0x1);

    /* Disable SD high-speed mode (which seems to be broken) */
    reg = mfdcrx(CCTL0_MCO2) & !0x2;
    mtdcrx(CCTL0_MCO2, reg);

    /* Set the MAC address */
    emac = finddevice(c"/plb/opb/ethernet".as_ptr() as *const _);
    if (emac as usize > 0 {
        if mac_addr != 0 {
            setprop(
                emac,
                c"local-mac-address".as_ptr() as *const _,
                ((&mac_addr as *const u64 as *const u8).add(2)),
                6,
            );
        }
    }
}

pub unsafe fn platform_init(userdata: *mut i8) {
    let end_of_ram: usize;
    let avail_ram: usize;
    let pir_reg: u32;
    let mut node: i32;
    let mut size: i32;
    let timebase: *const u32;
    let mut len: i32;
    let mut i: i32;
    let userdata_len: i32;
    let mut end: *mut i8;

    *userdata.add(USERDATA_LEN - 1) = 0;
    userdata_len = strlen(userdata);
    i = 0;
    while i < userdata_len - 15 {
        if strncmp(userdata.add(i as usize), c"local-mac-addr=".as_ptr() as *const i8, 15) == 0 {
            if i > 0 && *userdata.add((i - 1) as usize) != b' ' as i8 {
                /* We've only found a substring ending
                 * with local-mac-addr so this isn't
                 * our mac address. */
                i += 1;
                continue;
            }

            mac_addr = strtoull(userdata.add((i + 15) as usize), &mut end, 16);

            /* Remove the "local-mac-addr=<...>" from the kernel
             * command line, including the tailing space if
             * present. */
            if *end == b' ' as i8 {
                end = end.add(1);
            }

            len = end as isize as i32 - userdata.add(i as usize) as isize as i32;
            memmove(
                userdata.add(i as usize),
                end,
                (userdata_len - (len + i) + 1) as usize,
            );
            break;
        }
        i += 1;
    }

    loader_info.cmdline = userdata;
    loader_info.cmdline_len = 256;

    ibm_akebono_memsize = ibm_akebono_detect_memsize();
    if (ibm_akebono_memsize >> 32) != 0 {
        end_of_ram = usize::MAX;
    } else {
        end_of_ram = ibm_akebono_memsize as usize;
    }
    avail_ram = end_of_ram.wrapping_sub(_end as usize);

    simple_alloc_init(_end, avail_ram, 128, 64);
    platform_ops.fixups = Some(ibm_akebono_fixups);
    platform_ops.exit = Some(ibm44x_dbcr_reset);
    pir_reg = mfspr(SPRN_PIR);

    /* Make sure FDT blob is sane */
    if fdt_check_header(_dtb_start) != 0 {
        fatal(c"Invalid device tree blob\n".as_ptr() as *const i8);
    }

    node = fdt_node_offset_by_prop_value(
        _dtb_start,
        -1,
        c"device_type".as_ptr() as *const i8,
        c"cpu".as_ptr() as *const i8,
        core::mem::size_of::<[u8; 4]>(),
    );
    if node < 0 {
        fatal(c"Cannot find cpu node\n".as_ptr() as *const i8);
    }
    timebase = fdt_getprop(
        _dtb_start,
        node,
        c"timebase-frequency".as_ptr() as *const i8,
        &mut size,
    );
    if !timebase.is_null() && size == 4 {
        timebase_period_ns = 1_000_000_000u32 / *timebase;
    }

    fdt_set_boot_cpuid_phys(_dtb_start, pir_reg);
    fdt_init(_dtb_start);

    serial_console_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
