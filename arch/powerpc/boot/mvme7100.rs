// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Motload compatibility for the Emerson/Artesyn MVME7100
 *
 * Copyright 2016 Elettra-Sincrotrone Trieste S.C.p.A.
 *
 * Author: Alessio Igor Bogani <alessio.bogani@elettra.eu>
 */

// Dependencies supplied by the corresponding C headers:
// ops.h, stdio.h, cuboot.h, and ppcboot.h.
// Build-time target conditions: TARGET_86xx, TARGET_HAS_ETH1,
// TARGET_HAS_ETH2, and TARGET_HAS_ETH3.

static mut bd: bd_t = unsafe { core::mem::zeroed() };

// BSS_STACK(16384);

unsafe fn mvme7100_fixups() {
    let mut devp: *mut core::ffi::c_void;
    let busfreq: libc::c_ulong = bd.bi_busfreq * 1000000;

    dt_fixup_cpu_clocks(bd.bi_intfreq * 1000000, busfreq / 4, busfreq);

    devp = finddevice(c"/soc@f1000000".as_ptr() as *const _);
    if !devp.is_null() {
        setprop(
            devp,
            c"bus-frequency".as_ptr() as *const _,
            &busfreq as *const _ as *const _,
            core::mem::size_of_val(&busfreq),
        );
    }

    devp = finddevice(c"/soc/serial@4500".as_ptr() as *const _);
    if !devp.is_null() {
        setprop(
            devp,
            c"clock-frequency".as_ptr() as *const _,
            &busfreq as *const _ as *const _,
            core::mem::size_of_val(&busfreq),
        );
    }

    dt_fixup_memory(bd.bi_memstart, bd.bi_memsize);

    dt_fixup_mac_address_by_alias(c"ethernet0".as_ptr() as *const _, bd.bi_enetaddr);
    dt_fixup_mac_address_by_alias(c"ethernet1".as_ptr() as *const _, bd.bi_enet1addr);
    dt_fixup_mac_address_by_alias(c"ethernet2".as_ptr() as *const _, bd.bi_enet2addr);
    dt_fixup_mac_address_by_alias(c"ethernet3".as_ptr() as *const _, bd.bi_enet3addr);
}

pub unsafe extern "C" fn platform_init(
    _r3: libc::c_ulong,
    _r4: libc::c_ulong,
    _r5: libc::c_ulong,
    _r6: libc::c_ulong,
    _r7: libc::c_ulong,
) {
    // CUBOOT_INIT();
    fdt_init(_dtb_start);
    serial_console_init();
    platform_ops.fixups = Some(mvme7100_fixups);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
