// SPDX-License-Identifier: GPL-2.0-only
/*
 * Old U-boot compatibility for 85xx
 *
 * Author: Scott Wood <scottwood@freescale.com>
 *
 * Copyright (c) 2007 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the corresponding platform headers:
// ops.h, stdio.h, cuboot.h, and ppcboot.h (TARGET_85xx, TARGET_HAS_ETH3).

extern "C" {
    static mut _dtb_start: core::ffi::c_void;

    fn dt_fixup_memory(memstart: u64, memsize: u64);
    fn dt_fixup_mac_address_by_alias(alias: *const core::ffi::c_char,
                                      address: *const u8);
    fn dt_fixup_cpu_clocks(intfreq: u64, busfreq_div8: u64, busfreq: u64);
    fn find_node_by_devtype(node: *mut core::ffi::c_void,
                            devtype: *const core::ffi::c_char)
                            -> *mut core::ffi::c_void;
    fn setprop(node: *mut core::ffi::c_void,
               name: *const core::ffi::c_char,
               value: *const core::ffi::c_void,
               length: usize);
    fn get_parent(node: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn fdt_init(dtb: *mut core::ffi::c_void);
    fn serial_console_init();
}

extern "C" {
    static mut platform_ops: PlatformOps;
}

#[repr(C)]
struct PlatformOps {
    fixups: Option<unsafe extern "C" fn()>,
}

static mut bd: bd_t = unsafe { core::mem::zeroed() };

unsafe extern "C" fn platform_fixups() {
    let mut soc: *mut core::ffi::c_void;

    dt_fixup_memory(bd.bi_memstart, bd.bi_memsize);
    dt_fixup_mac_address_by_alias(
        b"ethernet0\0".as_ptr() as *const core::ffi::c_char,
        bd.bi_enetaddr.as_ptr(),
    );
    dt_fixup_mac_address_by_alias(
        b"ethernet1\0".as_ptr() as *const core::ffi::c_char,
        bd.bi_enet1addr.as_ptr(),
    );
    dt_fixup_mac_address_by_alias(
        b"ethernet2\0".as_ptr() as *const core::ffi::c_char,
        bd.bi_enet2addr.as_ptr(),
    );
    dt_fixup_mac_address_by_alias(
        b"ethernet3\0".as_ptr() as *const core::ffi::c_char,
        bd.bi_enet3addr.as_ptr(),
    );
    dt_fixup_cpu_clocks(bd.bi_intfreq, bd.bi_busfreq / 8, bd.bi_busfreq);

    /* Unfortunately, the specific model number is encoded in the
     * soc node name in existing dts files -- once that is fixed,
     * this can do a simple path lookup.
     */
    soc = find_node_by_devtype(
        core::ptr::null_mut(),
        b"soc\0".as_ptr() as *const core::ffi::c_char,
    );
    if !soc.is_null() {
        let mut serial: *mut core::ffi::c_void = core::ptr::null_mut();

        setprop(
            soc,
            b"bus-frequency\0".as_ptr() as *const core::ffi::c_char,
            &bd.bi_busfreq as *const _ as *const core::ffi::c_void,
            core::mem::size_of_val(&bd.bi_busfreq),
        );

        loop {
            serial = find_node_by_devtype(
                serial,
                b"serial\0".as_ptr() as *const core::ffi::c_char,
            );
            if serial.is_null() {
                break;
            }
            if get_parent(serial) != soc {
                continue;
            }

            setprop(
                serial,
                b"clock-frequency\0".as_ptr() as *const core::ffi::c_char,
                &bd.bi_busfreq as *const _ as *const core::ffi::c_void,
                core::mem::size_of_val(&bd.bi_busfreq),
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn platform_init(
    r3: core::ffi::c_ulong,
    r4: core::ffi::c_ulong,
    r5: core::ffi::c_ulong,
    r6: core::ffi::c_ulong,
    r7: core::ffi::c_ulong,
) {
    // CUBOOT_INIT();
    let _ = (r3, r4, r5, r6, r7);
    fdt_init(&_dtb_start as *const _ as *mut core::ffi::c_void);
    serial_console_init();
    platform_ops.fixups = Some(platform_fixups);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
