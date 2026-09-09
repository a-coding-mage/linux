// SPDX-License-Identifier: GPL-2.0
/* devices.c: Initial scan of the prom device tree for important
 *             Sparc device nodes which we need to find.
 *
 * This is based on the sparc64 version, but sun4m doesn't always use
 * the hardware MIDs, so be careful.
 *
 * Copyright (C) 1996 David S. Miller (davem@caip.rutgers.edu)
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn cpu_mid_prop() -> *mut u8 {
    if sparc_cpu_model == sun4d {
        return b"cpu-id\0".as_ptr() as *mut u8;
    }
    b"mid\0".as_ptr() as *mut u8
}

unsafe fn check_cpu_node(
    nd: phandle,
    cur_inst: *mut i32,
    compare: unsafe extern "C" fn(phandle, i32, *mut core::ffi::c_void) -> i32,
    compare_arg: *mut core::ffi::c_void,
    prom_node: *mut phandle,
    mid: *mut i32,
) -> i32 {
    if compare(nd, *cur_inst, compare_arg) == 0 {
        if !prom_node.is_null() {
            *prom_node = nd;
        }
        if !mid.is_null() {
            *mid = prom_getintdefault(nd, cpu_mid_prop(), 0);
            if sparc_cpu_model == sun4m {
                *mid &= 3;
            }
        }
        return 0;
    }

    *cur_inst += 1;

    -ENODEV
}

unsafe fn __cpu_find_by(
    compare: unsafe extern "C" fn(phandle, i32, *mut core::ffi::c_void) -> i32,
    compare_arg: *mut core::ffi::c_void,
    prom_node: *mut phandle,
    mid: *mut i32,
) -> i32 {
    let mut dp: *mut device_node;
    let mut cur_inst: i32 = 0;

    // Preserves the C for_each_node_by_type(dp, "cpu") iteration macro.
    for_each_node_by_type!(dp, b"cpu\0".as_ptr() as *const u8) {
        let err = check_cpu_node((*dp).phandle, &mut cur_inst, compare, compare_arg, prom_node, mid);
        if err == 0 {
            of_node_put(dp);
            return 0;
        }
    }

    -ENODEV
}

unsafe extern "C" fn cpu_instance_compare(
    _nd: phandle,
    instance: i32,
    arg: *mut core::ffi::c_void,
) -> i32 {
    let desired_instance = arg as isize as i32;

    if instance == desired_instance {
        return 0;
    }
    -ENODEV
}

pub unsafe extern "C" fn cpu_find_by_instance(
    instance: i32,
    prom_node: *mut phandle,
    mid: *mut i32,
) -> i32 {
    __cpu_find_by(
        cpu_instance_compare,
        instance as isize as *mut core::ffi::c_void,
        prom_node,
        mid,
    )
}

unsafe extern "C" fn cpu_mid_compare(
    nd: phandle,
    _instance: i32,
    arg: *mut core::ffi::c_void,
) -> i32 {
    let desired_mid = arg as isize as i32;
    let this_mid = prom_getintdefault(nd, cpu_mid_prop(), 0);

    if this_mid == desired_mid
        || (sparc_cpu_model == sun4m && (this_mid & 3) == desired_mid)
    {
        return 0;
    }
    -ENODEV
}

pub unsafe extern "C" fn cpu_find_by_mid(mid: i32, prom_node: *mut phandle) -> i32 {
    __cpu_find_by(
        cpu_mid_compare,
        mid as isize as *mut core::ffi::c_void,
        prom_node,
        core::ptr::null_mut(),
    )
}

/* sun4m uses truncated mids since we base the cpuid on the ttable/irqset
 * address (0-3).  This gives us the true hardware mid, which might have
 * some other bits set.  On 4d hardware and software mids are the same.
 */
pub unsafe extern "C" fn cpu_get_hwmid(prom_node: phandle) -> i32 {
    prom_getintdefault(prom_node, cpu_mid_prop(), -ENODEV)
}

pub unsafe extern "C" fn device_scan() {
    printk(KERN_NOTICE, b"Booting Linux...\0".as_ptr());

    // The CONFIG_SMP build-time condition is preserved from the source.
    #[cfg(not(CONFIG_SMP))]
    {
        let mut cpu_node: phandle = 0;
        let err = cpu_find_by_instance(0, &mut cpu_node, core::ptr::null_mut());
        if err != 0 {
            /* Probably a sun4e, Sun is trying to trick us ;-) */
            prom_printf(b"No cpu nodes, cannot continue\0".as_ptr());
            prom_halt();
        }
        cpu_data(0).clock_tick = prom_getintdefault(
            cpu_node,
            b"clock-frequency\0".as_ptr() as *mut u8,
            0,
        );
    }

    auxio_probe();
    auxio_power_probe();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
