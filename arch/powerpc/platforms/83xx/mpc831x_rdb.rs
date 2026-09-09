// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * arch/powerpc/platforms/83xx/mpc831x_rdb.c
 *
 * Description: MPC831x RDB board specific routines.
 * This file is based on mpc834x_sys.c
 * Author: Lo Wlison <r43300@freescale.com>
 *
 * Copyright (C) Freescale Semiconductor, Inc. 2006. All rights reserved.
 */

use core::ffi::{c_char, c_int, c_void};

// External declarations supplied by the kernel and mpc83xx support code.
extern "C" {
    fn mpc83xx_setup_arch();
    fn mpc831x_usb_cfg();
    fn mpc83xx_declare_of_platform_devices() -> c_int;
    fn mpc83xx_setup_pci() -> c_int;
    fn mpc83xx_ipic_init_IRQ();
    fn ipic_get_irq() -> c_int;
    fn mpc83xx_restart(cmd: *const c_char);
    fn mpc83xx_time_init();
    fn udbg_progress(message: *const c_char, value: c_int);
}

/*
 * Setup the architecture
 */
unsafe extern "C" fn mpc831x_rdb_setup_arch() {
    mpc83xx_setup_arch();
    mpc831x_usb_cfg();
}

static BOARD: [*const c_char; 3] = [
    b"MPC8313ERDB\0".as_ptr() as *const c_char,
    b"fsl,mpc8315erdb\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// Corresponds to: machine_device_initcall(mpc831x_rdb,
// mpc83xx_declare_of_platform_devices);

#[repr(C)]
pub struct MachineDesc {
    pub name: *const c_char,
    pub compatibles: *const *const c_char,
    pub setup_arch: unsafe extern "C" fn(),
    pub discover_phbs: unsafe extern "C" fn() -> c_int,
    pub init_IRQ: unsafe extern "C" fn(),
    pub get_irq: unsafe extern "C" fn() -> c_int,
    pub restart: unsafe extern "C" fn(*const c_char),
    pub time_init: unsafe extern "C" fn(),
    pub progress: unsafe extern "C" fn(*const c_char, c_int),
}

// Translation of define_machine(mpc831x_rdb) { ... }.
#[no_mangle]
pub static mpc831x_rdb: MachineDesc = MachineDesc {
    name: b"MPC831x RDB\0".as_ptr() as *const c_char,
    compatibles: BOARD.as_ptr(),
    setup_arch: mpc831x_rdb_setup_arch,
    discover_phbs: mpc83xx_setup_pci,
    init_IRQ: mpc83xx_ipic_init_IRQ,
    get_irq: ipic_get_irq,
    restart: mpc83xx_restart,
    time_init: mpc83xx_time_init,
    progress: udbg_progress,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
