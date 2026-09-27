// SPDX-License-Identifier: GPL-2.0-only
/*
 * Old U-boot compatibility for 8xx
 *
 * Author: Scott Wood <scottwood@freescale.com>
 *
 * Copyright (c) 2007 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the surrounding repository:
// ops.h, stdio.h, cuboot.h, and ppcboot.h (with TARGET_8xx and TARGET_HAS_ETH1).

#[allow(non_camel_case_types)]
extern "C" {
    static mut _dtb_start: u8;

    fn dt_fixup_memory(memstart: core::ffi::c_ulong, memsize: core::ffi::c_ulong);
    fn dt_fixup_mac_addresses(enetaddr: *mut u8, enet1addr: *mut u8);
    fn dt_fixup_cpu_clocks(
        intfreq: core::ffi::c_ulong,
        busfreq_div16: core::ffi::c_ulong,
        busfreq: core::ffi::c_ulong,
    );
    fn finddevice(path: *const core::ffi::c_char) -> *mut core::ffi::c_void;
    fn setprop(
        node: *mut core::ffi::c_void,
        name: *const core::ffi::c_char,
        value: *const core::ffi::c_void,
        len: core::ffi::c_int,
    );
    fn fdt_init(dtb: *mut u8);
    fn serial_console_init();

    static mut platform_ops: PlatformOps;
}

#[repr(C)]
#[allow(non_camel_case_types)]
struct bd_t {
    bi_memstart: core::ffi::c_ulong,
    bi_memsize: core::ffi::c_ulong,
    bi_enetaddr: [u8; 6],
    bi_enet1addr: [u8; 6],
    bi_intfreq: core::ffi::c_ulong,
    bi_busfreq: core::ffi::c_ulong,
}

#[repr(C)]
struct PlatformOps {
    fixups: Option<unsafe extern "C" fn()>,
}

static mut bd: bd_t = bd_t {
    bi_memstart: 0,
    bi_memsize: 0,
    bi_enetaddr: [0; 6],
    bi_enet1addr: [0; 6],
    bi_intfreq: 0,
    bi_busfreq: 0,
};

unsafe extern "C" fn platform_fixups() {
    let mut node: *mut core::ffi::c_void;

    dt_fixup_memory(bd.bi_memstart, bd.bi_memsize);
    dt_fixup_mac_addresses(bd.bi_enetaddr.as_mut_ptr(), bd.bi_enet1addr.as_mut_ptr());
    dt_fixup_cpu_clocks(bd.bi_intfreq, bd.bi_busfreq / 16, bd.bi_busfreq);

    node = finddevice(b"/soc/cpm\0".as_ptr() as *const core::ffi::c_char);
    if !node.is_null() {
        setprop(
            node,
            b"clock-frequency\0".as_ptr() as *const core::ffi::c_char,
            &bd.bi_busfreq as *const _ as *const core::ffi::c_void,
            4,
        );
    }

    node = finddevice(b"/soc/cpm/brg\0".as_ptr() as *const core::ffi::c_char);
    if !node.is_null() {
        setprop(
            node,
            b"clock-frequency\0".as_ptr() as *const core::ffi::c_char,
            &bd.bi_busfreq as *const _ as *const core::ffi::c_void,
            4,
        );
    }
}

pub unsafe extern "C" fn platform_init(
    r3: core::ffi::c_ulong,
    r4: core::ffi::c_ulong,
    r5: core::ffi::c_ulong,
    r6: core::ffi::c_ulong,
    r7: core::ffi::c_ulong,
) {
    let _ = (r3, r4, r5, r6, r7);
    // CUBOOT_INIT is provided as a macro by cuboot.h.
    CUBOOT_INIT!();
    fdt_init(&_dtb_start as *const u8 as *mut u8);
    serial_console_init();
    platform_ops.fixups = Some(platform_fixups);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
