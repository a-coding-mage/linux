// SPDX-License-Identifier: GPL-2.0-only
/*
 * Old U-boot compatibility for MPC5200
 *
 * Author: Grant Likely <grant.likely@secretlab.ca>
 *
 * Copyright (c) 2007 Secret Lab Technologies Ltd.
 * Copyright (c) 2007 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the surrounding platform implementation.

const TARGET_PPC_MPC52XX: () = ();

static mut bd: bd_t = unsafe { core::mem::zeroed() };

unsafe fn platform_fixups() {
    let mut soc: *mut core::ffi::c_void;
    let mut reg: *mut core::ffi::c_void;
    let div: i32;
    let mut sysfreq: u32;

    dt_fixup_memory(bd.bi_memstart, bd.bi_memsize);
    dt_fixup_mac_addresses(bd.bi_enetaddr.as_ptr());
    dt_fixup_cpu_clocks(bd.bi_intfreq, bd.bi_busfreq / 4, bd.bi_busfreq);

    /* Unfortunately, the specific model number is encoded in the
     * soc node name in existing dts files -- once that is fixed,
     * this can do a simple path lookup.
     */
    soc = find_node_by_devtype(core::ptr::null_mut(), b"soc\0".as_ptr() as *const _);
    if soc.is_null() {
        soc = find_node_by_compatible(
            core::ptr::null_mut(),
            b"fsl,mpc5200-immr\0".as_ptr() as *const _,
        );
    }
    if soc.is_null() {
        soc = find_node_by_compatible(
            core::ptr::null_mut(),
            b"fsl,mpc5200b-immr\0".as_ptr() as *const _,
        );
    }
    if !soc.is_null() {
        setprop(
            soc,
            b"bus-frequency\0".as_ptr() as *const _,
            &bd.bi_ipbfreq as *const _ as *const core::ffi::c_void,
            core::mem::size_of_val(&bd.bi_ipbfreq),
        );

        if !dt_xlate_reg(
            soc,
            0,
            &mut reg as *mut _ as *mut core::ffi::c_void,
            core::ptr::null_mut(),
        ) {
            return;
        }
        div = if (in_8(reg.add(0x204)) & 0x0020) != 0 { 8 } else { 4 };
        sysfreq = bd.bi_busfreq.wrapping_mul(div as u32);
        setprop(
            soc,
            b"system-frequency\0".as_ptr() as *const _,
            &sysfreq as *const _ as *const core::ffi::c_void,
            core::mem::size_of_val(&sysfreq),
        );
    }
}

pub unsafe fn platform_init(
    r3: c_ulong,
    r4: c_ulong,
    r5: c_ulong,
    r6: c_ulong,
    r7: c_ulong,
) {
    let _ = (r3, r4, r5, r6, r7);
    // CUBOOT_INIT(); -- initialization macro supplied by the surrounding build.
    cuboot_init();
    fdt_init(_dtb_start);
    serial_console_init();
    platform_ops.fixups = Some(platform_fixups);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
