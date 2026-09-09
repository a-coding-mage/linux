// SPDX-License-Identifier: GPL-2.0-only
/*
 * Old U-boot compatibility for 824x
 *
 * Copyright (c) 2007 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the surrounding PowerPC boot environment:
// ops.h, stdio.h, cuboot.h, and ppcboot.h.
// The C build defines TARGET_824x before including ppcboot.h.

const TARGET_824X: bool = true;

static mut bd: bd_t = unsafe { core::mem::zeroed() };

unsafe fn platform_fixups() {
    let mut soc: *mut core::ffi::c_void;

    dt_fixup_memory((*core::ptr::addr_of!(bd)).bi_memstart,
                    (*core::ptr::addr_of!(bd)).bi_memsize);
    dt_fixup_mac_addresses((*core::ptr::addr_of!(bd)).bi_enetaddr);
    dt_fixup_cpu_clocks((*core::ptr::addr_of!(bd)).bi_intfreq,
                        (*core::ptr::addr_of!(bd)).bi_busfreq / 4,
                        (*core::ptr::addr_of!(bd)).bi_busfreq);

    soc = find_node_by_devtype(core::ptr::null_mut(), "soc");
    if !soc.is_null() {
        let mut serial: *mut core::ffi::c_void = core::ptr::null_mut();

        setprop(soc, "bus-frequency",
                core::ptr::addr_of!((*core::ptr::addr_of!(bd)).bi_busfreq).cast(),
                core::mem::size_of_val(&(*core::ptr::addr_of!(bd)).bi_busfreq));

        loop {
            serial = find_node_by_devtype(serial, "serial");
            if serial.is_null() {
                break;
            }
            if get_parent(serial) != soc {
                continue;
            }

            setprop(serial, "clock-frequency",
                    core::ptr::addr_of!((*core::ptr::addr_of!(bd)).bi_busfreq).cast(),
                    core::mem::size_of_val(&(*core::ptr::addr_of!(bd)).bi_busfreq));
        }
    }
}

pub unsafe fn platform_init(
    _r3: c_ulong,
    _r4: c_ulong,
    _r5: c_ulong,
    _r6: c_ulong,
    _r7: c_ulong,
) {
    CUBOOT_INIT!();
    fdt_init(_dtb_start);
    serial_console_init();
    platform_ops.fixups = Some(platform_fixups);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
