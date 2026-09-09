// SPDX-License-Identifier: GPL-2.0-only
/*
 * Old U-boot compatibility for 85xx
 *
 * Author: Scott Wood <scottwood@freescale.com>
 *
 * Copyright (c) 2007 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the corresponding platform headers.

extern "C" {
    static _dtb_start: u8;
    static mut platform_ops: platform_ops_t;

    fn dt_fixup_memory(start: usize, size: usize);
    fn dt_fixup_mac_address_by_alias(alias: *const u8, address: *const u8);
    fn dt_fixup_cpu_clocks(intfreq: usize, busfreq_div: usize, busfreq: usize);
    fn find_node_by_devtype(previous: *mut core::ffi::c_void, devtype: *const u8)
        -> *mut core::ffi::c_void;
    fn find_node_by_compatible(
        previous: *mut core::ffi::c_void,
        compatible: *const u8,
    ) -> *mut core::ffi::c_void;
    fn setprop(
        node: *mut core::ffi::c_void,
        name: *const u8,
        value: *const core::ffi::c_void,
        size: usize,
    );
    fn get_parent(node: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn fdt_init(dtb: *const u8);
    fn serial_console_init();
}

// Type supplied by the corresponding platform headers.
#[allow(non_camel_case_types)]
type platform_ops_t = crate::platform_ops_t;

// Type supplied by ppcboot.h.
#[allow(non_camel_case_types)]
type bd_t = crate::bd_t;

static mut bd: bd_t = unsafe { core::mem::zeroed() };

unsafe fn platform_fixups() {
    let mut devp: *mut core::ffi::c_void;

    dt_fixup_memory(bd.bi_memstart, bd.bi_memsize);
    dt_fixup_mac_address_by_alias(b"ethernet0\0".as_ptr(), bd.bi_enetaddr.as_ptr());
    dt_fixup_mac_address_by_alias(b"ethernet1\0".as_ptr(), bd.bi_enet1addr.as_ptr());
    dt_fixup_mac_address_by_alias(b"ethernet2\0".as_ptr(), bd.bi_enet2addr.as_ptr());
    dt_fixup_cpu_clocks(bd.bi_intfreq, bd.bi_busfreq / 8, bd.bi_busfreq);

    /* Unfortunately, the specific model number is encoded in the
     * soc node name in existing dts files -- once that is fixed,
     * this can do a simple path lookup.
     */
    devp = find_node_by_devtype(core::ptr::null_mut(), b"soc\0".as_ptr());
    if !devp.is_null() {
        let mut serial: *mut core::ffi::c_void = core::ptr::null_mut();

        setprop(
            devp,
            b"bus-frequency\0".as_ptr(),
            (&bd.bi_busfreq as *const _).cast(),
            core::mem::size_of_val(&bd.bi_busfreq),
        );

        loop {
            serial = find_node_by_devtype(serial, b"serial\0".as_ptr());
            if serial.is_null() {
                break;
            }
            if get_parent(serial) != devp {
                continue;
            }

            setprop(
                serial,
                b"clock-frequency\0".as_ptr(),
                (&bd.bi_busfreq as *const _).cast(),
                core::mem::size_of_val(&bd.bi_busfreq),
            );
        }
    }

    devp = find_node_by_compatible(core::ptr::null_mut(), b"fsl,cpm2-brg\0".as_ptr());
    if !devp.is_null() {
        setprop(
            devp,
            b"clock-frequency\0".as_ptr(),
            (&bd.bi_brgfreq as *const _).cast(),
            core::mem::size_of_val(&bd.bi_brgfreq),
        );
    }
}

pub unsafe fn platform_init(
    _r3: usize,
    _r4: usize,
    _r5: usize,
    _r6: usize,
    _r7: usize,
) {
    // CUBOOT_INIT(); — supplied by cuboot.h as a build-time macro.
    cuboot_init!();
    fdt_init((&_dtb_start as *const u8));
    serial_console_init();
    platform_ops.fixups = Some(platform_fixups);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
