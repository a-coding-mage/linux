// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2008 PIKA Technologies
 *   Sean MacLennan <smaclennan@pikatech.com>
 */

// C dependencies: ops.h, 4xx.h, cuboot.h, stdio.h, and ppcboot.h.
// The TARGET_4xx and TARGET_44x build-time definitions are preserved here.

const TARGET_4XX: () = ();
const TARGET_44X: () = ();

static mut bd: bd_t = unsafe { core::mem::zeroed() };

unsafe extern "C" {
    fn ibm440ep_fixup_clocks(clock: u32, baud: u32, bus: u32);
    fn ibm4xx_sdram_fixup_memsize();
    fn ibm4xx_fixup_ebc_ranges(path: *const core::ffi::c_char);
    fn dt_fixup_mac_address_by_alias(
        alias: *const core::ffi::c_char,
        address: *const u8,
    );
    fn ibm44x_dbcr_reset();
    fn fdt_init(dtb: *const u8);
    fn serial_console_init();
    fn cuboot_init();
}

extern "C" {
    static _dtb_start: u8;
    static mut platform_ops: platform_ops_t;
}

unsafe fn warp_fixups() {
    ibm440ep_fixup_clocks(66000000, 11059200, 50000000);
    ibm4xx_sdram_fixup_memsize();
    ibm4xx_fixup_ebc_ranges(b"/plb/opb/ebc\0".as_ptr() as *const core::ffi::c_char);
    dt_fixup_mac_address_by_alias(
        b"ethernet0\0".as_ptr() as *const core::ffi::c_char,
        (*core::ptr::addr_of!(bd)).bi_enetaddr.as_ptr(),
    );
}

#[no_mangle]
pub unsafe extern "C" fn platform_init(
    r3: c_ulong,
    r4: c_ulong,
    r5: c_ulong,
    r6: c_ulong,
    r7: c_ulong,
) {
    let _ = (r3, r4, r5, r6, r7);

    cuboot_init();

    platform_ops.fixups = Some(warp_fixups);
    platform_ops.exit = Some(ibm44x_dbcr_reset);
    fdt_init(core::ptr::addr_of!(_dtb_start));
    serial_console_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
