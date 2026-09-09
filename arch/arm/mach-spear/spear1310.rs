// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-spear13xx/spear1310.c
 *
 * SPEAr1310 machine source file
 *
 * Copyright (C) 2012 ST Microelectronics
 * Viresh Kumar <vireshk@kernel.org>
 */

// #define pr_fmt(fmt) "SPEAr1310: " fmt
// External Linux kernel declarations and constants are supplied by the
// corresponding kernel headers and machine sources.

use core::ffi::c_void;

const SPEAR1310_RAS_GRP1_BASE: usize = 0xD800_0000;
const VA_SPEAR1310_RAS_GRP1_BASE: usize = 0xFA00_0000;

extern "C" {
    fn platform_device_register_simple(
        name: *const core::ffi::c_char,
        id: i32,
        res: *const c_void,
        num: usize,
    ) -> *mut c_void;
    fn __phys_to_pfn(phys: usize) -> usize;
    fn iotable_init(desc: *mut MapDesc, nr: usize);
    fn spear13xx_map_io();
    fn spear13xx_smp_ops();
    fn spear13xx_timer_init();
    fn spear_restart(mode: i32, cmd: *const core::ffi::c_char);
}

#[repr(C)]
pub struct MapDesc {
    pub virtual_: usize,
    pub pfn: usize,
    pub length: usize,
    pub type_: u32,
}

const SZ_16M: usize = 16 * 1024 * 1024;
const MT_DEVICE: u32 = 0;

unsafe fn spear1310_dt_init() {
    let name = b"spear-cpufreq\0";
    platform_device_register_simple(name.as_ptr() as *const core::ffi::c_char, -1, core::ptr::null(), 0);
}

static SPEAR1310_DT_BOARD_COMPAT: [*const core::ffi::c_char; 3] = [
    b"st,spear1310\0".as_ptr() as *const core::ffi::c_char,
    b"st,spear1310-evb\0".as_ptr() as *const core::ffi::c_char,
    core::ptr::null(),
];

/*
 * Following will create 16MB static virtual/physical mappings
 * PHYSICAL        VIRTUAL
 * 0xD8000000      0xFA000000
 */
#[no_mangle]
pub static mut spear1310_io_desc: [MapDesc; 1] = [MapDesc {
    virtual_: VA_SPEAR1310_RAS_GRP1_BASE,
    pfn: 0,
    length: SZ_16M,
    type_: MT_DEVICE,
}];

unsafe fn spear1310_map_io() {
    spear1310_io_desc[0].pfn = __phys_to_pfn(SPEAR1310_RAS_GRP1_BASE);
    iotable_init(spear1310_io_desc.as_mut_ptr(), spear1310_io_desc.len());
    spear13xx_map_io();
}

/* DT_MACHINE_START(SPEAR1310_DT, "ST SPEAr1310 SoC with Flattened Device Tree") */
#[repr(C)]
pub struct MachineDesc {
    pub smp: unsafe extern "C" fn(),
    pub map_io: unsafe fn(),
    pub init_time: unsafe extern "C" fn(),
    pub init_machine: unsafe fn(),
    pub restart: unsafe extern "C" fn(i32, *const core::ffi::c_char),
    pub dt_compat: *const *const core::ffi::c_char,
}

#[no_mangle]
pub static SPEAR1310_DT: MachineDesc = MachineDesc {
    smp: spear13xx_smp_ops,
    map_io: spear1310_map_io,
    init_time: spear13xx_timer_init,
    init_machine: spear1310_dt_init,
    restart: spear_restart,
    dt_compat: SPEAR1310_DT_BOARD_COMPAT.as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
