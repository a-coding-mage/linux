// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-spear13xx/spear1340.c
 *
 * SPEAr1340 machine source file
 *
 * Copyright (C) 2012 ST Microelectronics
 * Viresh Kumar <vireshk@kernel.org>
 */

// #define pr_fmt(fmt) "SPEAR1340: " fmt

use core::ffi::{c_char, c_void};

extern "C" {
    fn platform_device_register_simple(
        name: *const c_char,
        id: i32,
        res: *const c_void,
        num: u32,
    ) -> *mut c_void;

    static smp_ops_spear13xx_smp_ops: *const c_void;
    fn spear13xx_map_io();
    fn spear13xx_timer_init();
    fn spear_restart();
}

unsafe fn spear1340_dt_init() {
    // platform_device_register_simple("spear-cpufreq", -1, NULL, 0);
    static DEVICE_NAME: &[u8] = b"spear-cpufreq\0";
    platform_device_register_simple(
        DEVICE_NAME.as_ptr() as *const c_char,
        -1,
        core::ptr::null(),
        0,
    );
}

static SPEAR1340_DT_BOARD_COMPAT: [*const c_char; 3] = [
    b"st,spear1340\0".as_ptr() as *const c_char,
    b"st,spear1340-evb\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// Translation of DT_MACHINE_START(SPEAR1340_DT,
//     "ST SPEAr1340 SoC with Flattened Device Tree") ... MACHINE_END.
#[repr(C)]
pub struct MachineDesc {
    pub name: *const c_char,
    pub smp: *const c_void,
    pub map_io: Option<unsafe extern "C" fn()>,
    pub init_time: Option<unsafe extern "C" fn()>,
    pub init_machine: Option<unsafe fn()>,
    pub restart: Option<unsafe extern "C" fn()>,
    pub dt_compat: *const *const c_char,
}

#[no_mangle]
pub static SPEAR1340_DT: MachineDesc = MachineDesc {
    name: b"ST SPEAr1340 SoC with Flattened Device Tree\0".as_ptr()
        as *const c_char,
    smp: unsafe { smp_ops_spear13xx_smp_ops },
    map_io: Some(spear13xx_map_io),
    init_time: Some(spear13xx_timer_init),
    init_machine: Some(spear1340_dt_init),
    restart: Some(spear_restart),
    dt_compat: SPEAR1340_DT_BOARD_COMPAT.as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
