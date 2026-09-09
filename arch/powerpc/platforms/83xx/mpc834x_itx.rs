// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * arch/powerpc/platforms/83xx/mpc834x_itx.c
 *
 * MPC834x ITX board specific routines
 *
 * Maintainer: Kumar Gala <galak@kernel.crashing.org>
 */

use core::ffi::{c_char, c_int, c_void};

// Linux and architecture headers provide these declarations and types.
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

extern "C" {
    fn mpc83xx_declare_of_platform_devices();
    fn of_platform_bus_probe(
        bus: *mut c_void,
        matches: *const of_device_id,
        parent: *mut c_void,
    ) -> c_int;
    fn mpc83xx_setup_arch();
    fn mpc834x_usb_cfg();
    fn mpc83xx_setup_pci();
    fn mpc83xx_ipic_init_IRQ();
    fn ipic_get_irq() -> c_int;
    fn mpc83xx_restart();
    fn mpc83xx_time_init();
    fn udbg_progress(message: *const c_char, value: c_int);
}

static MPC834X_ITX_IDS: [of_device_id; 2] = [
    of_device_id {
        compatible: b"fsl,pq2pro-localbus\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

unsafe fn mpc834x_itx_declare_of_platform_devices() -> c_int {
    mpc83xx_declare_of_platform_devices();
    of_platform_bus_probe(
        core::ptr::null_mut(),
        MPC834X_ITX_IDS.as_ptr(),
        core::ptr::null_mut(),
    )
}

// Corresponds to machine_device_initcall(mpc834x_itx,
// mpc834x_itx_declare_of_platform_devices).

/* ************************************************************************
 *
 * Setup the architecture
 *
 */
unsafe fn mpc834x_itx_setup_arch() {
    mpc83xx_setup_arch();

    mpc834x_usb_cfg();
}

// Corresponds to the PowerPC define_machine(mpc834x_itx) declaration.
#[repr(C)]
pub struct machine_desc {
    pub name: *const c_char,
    pub compatible: *const c_char,
    pub setup_arch: Option<unsafe extern "C" fn()>,
    pub discover_phbs: Option<unsafe extern "C" fn()>,
    pub init_irq: Option<unsafe extern "C" fn()>,
    pub get_irq: Option<unsafe extern "C" fn() -> c_int>,
    pub restart: Option<unsafe extern "C" fn()>,
    pub time_init: Option<unsafe extern "C" fn()>,
    pub progress: Option<unsafe extern "C" fn(*const c_char, c_int)>,
}

#[no_mangle]
pub static MPC834X_ITX: machine_desc = machine_desc {
    name: b"MPC834x ITX\0".as_ptr() as *const c_char,
    compatible: b"MPC834xMITX\0".as_ptr() as *const c_char,
    setup_arch: Some(mpc834x_itx_setup_arch),
    discover_phbs: Some(mpc83xx_setup_pci),
    init_irq: Some(mpc83xx_ipic_init_IRQ),
    get_irq: Some(ipic_get_irq),
    restart: Some(mpc83xx_restart),
    time_init: Some(mpc83xx_time_init),
    progress: Some(udbg_progress),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
