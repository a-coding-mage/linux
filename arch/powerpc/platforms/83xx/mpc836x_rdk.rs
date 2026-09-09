// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * MPC8360E-RDK board file.
 *
 * Copyright (c) 2006  Freescale Semiconductor, Inc.
 * Copyright (c) 2007-2008  MontaVista Software, Inc.
 *
 * Author: Anton Vorontsov <avorontsov@ru.mvista.com>
 */

// Translated dependencies supplied by the surrounding kernel environment.

extern "C" {
    fn mpc83xx_declare_of_platform_devices() -> i32;
    fn mpc83xx_setup_arch();
    fn mpc83xx_setup_pci();
    fn mpc83xx_ipic_init_IRQ();
    fn ipic_get_irq() -> i32;
    fn mpc83xx_restart();
    fn mpc83xx_time_init();
    fn udbg_progress(message: *const u8, hex: u32);
}

// Equivalent of machine_device_initcall(mpc836x_rdk,
// mpc83xx_declare_of_platform_devices).
#[used]
#[cfg_attr(target_os = "none", link_section = ".initcall")]
static MPC836X_RDK_DEVICE_INITCALL: unsafe extern "C" fn() -> i32 =
    mpc83xx_declare_of_platform_devices;

unsafe fn mpc836x_rdk_setup_arch() {
    mpc83xx_setup_arch();
}

#[repr(C)]
pub struct MachineDesc {
    pub name: *const u8,
    pub compatible: *const u8,
    pub setup_arch: unsafe fn(),
    pub discover_phbs: unsafe extern "C" fn(),
    pub init_irq: unsafe extern "C" fn(),
    pub get_irq: unsafe extern "C" fn() -> i32,
    pub restart: unsafe extern "C" fn(),
    pub time_init: unsafe extern "C" fn(),
    pub progress: unsafe extern "C" fn(*const u8, u32),
}

// Equivalent of define_machine(mpc836x_rdk).
#[no_mangle]
pub static mut mpc836x_rdk: MachineDesc = MachineDesc {
    name: b"MPC836x RDK\0".as_ptr(),
    compatible: b"fsl,mpc8360rdk\0".as_ptr(),
    setup_arch: mpc836x_rdk_setup_arch,
    discover_phbs: mpc83xx_setup_pci,
    init_irq: mpc83xx_ipic_init_IRQ,
    get_irq: ipic_get_irq,
    restart: mpc83xx_restart,
    time_init: mpc83xx_time_init,
    progress: udbg_progress,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
