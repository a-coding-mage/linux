// SPDX-License-Identifier: GPL-2.0-only
/*
 * Old U-boot compatibility for 83xx
 *
 * Author: Scott Wood <scottwood@freescale.com>
 *
 * Copyright (c) 2007 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the surrounding platform sources:
// ops.h, stdio.h, cuboot.h, and ppcboot.h (TARGET_83xx).

use core::ffi::{c_char, c_void};

extern "C" {
    static mut bd: bd_t;
    static _dtb_start: c_void;

    fn dt_fixup_memory(memstart: usize, memsize: usize);
    fn dt_fixup_mac_address_by_alias(alias: *const c_char, address: *const u8);
    fn dt_fixup_cpu_clocks(intfreq: usize, busfreq_div4: usize, busfreq: usize);
    fn find_node_by_devtype(node: *mut c_void, devtype: *const c_char) -> *mut c_void;
    fn setprop(node: *mut c_void, name: *const c_char, value: *const c_void, size: usize);
    fn get_parent(node: *mut c_void) -> *mut c_void;
    fn fdt_init(dtb_start: *const c_void);
    fn serial_console_init();
    fn cuboot_init();
    static mut platform_ops: platform_ops_t;
}

// `bd_t` and `platform_ops_t` are defined by the translated platform headers.
#[repr(C)]
pub struct bd_t {
    pub bi_memstart: usize,
    pub bi_memsize: usize,
    pub bi_enetaddr: [u8; 6],
    pub bi_enet1addr: [u8; 6],
    pub bi_intfreq: usize,
    pub bi_busfreq: usize,
}

#[repr(C)]
pub struct platform_ops_t {
    pub fixups: Option<unsafe extern "C" fn()>,
}

unsafe fn platform_fixups() {
    let mut soc: *mut c_void;

    dt_fixup_memory(bd.bi_memstart, bd.bi_memsize);
    dt_fixup_mac_address_by_alias(b"ethernet0\0".as_ptr() as *const c_char, bd.bi_enetaddr.as_ptr());
    dt_fixup_mac_address_by_alias(b"ethernet1\0".as_ptr() as *const c_char, bd.bi_enet1addr.as_ptr());
    dt_fixup_cpu_clocks(bd.bi_intfreq, bd.bi_busfreq / 4, bd.bi_busfreq);

    /* Unfortunately, the specific model number is encoded in the
     * soc node name in existing dts files -- once that is fixed,
     * this can do a simple path lookup.
     */
    soc = find_node_by_devtype(core::ptr::null_mut(), b"soc\0".as_ptr() as *const c_char);
    if !soc.is_null() {
        let mut serial: *mut c_void = core::ptr::null_mut();

        setprop(
            soc,
            b"bus-frequency\0".as_ptr() as *const c_char,
            &bd.bi_busfreq as *const usize as *const c_void,
            core::mem::size_of_val(&bd.bi_busfreq),
        );

        loop {
            serial = find_node_by_devtype(serial, b"serial\0".as_ptr() as *const c_char);
            if serial.is_null() {
                break;
            }
            if get_parent(serial) != soc {
                continue;
            }

            setprop(
                serial,
                b"clock-frequency\0".as_ptr() as *const c_char,
                &bd.bi_busfreq as *const usize as *const c_void,
                core::mem::size_of_val(&bd.bi_busfreq),
            );
        }
    }
}

pub unsafe extern "C" fn platform_init(
    _r3: usize,
    _r4: usize,
    _r5: usize,
    _r6: usize,
    _r7: usize,
) {
    cuboot_init();
    fdt_init(core::ptr::addr_of!(_dtb_start));
    serial_console_init();
    platform_ops.fixups = Some(platform_fixups);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
