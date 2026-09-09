// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * arch/powerpc/platforms/83xx/mpc837x_rdb.c
 *
 * Copyright (C) 2007 Freescale Semiconductor, Inc. All rights reserved.
 *
 * MPC837x RDB board specific routines
 */

// C dependencies supplied by other translation units/headers.
use core::ffi::{c_char, c_void};

extern "C" {
    fn get_immrbase() -> usize;
    fn ioremap(phys_addr: usize, size: usize) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn clrsetbits_be32(addr: *mut c_void, clear: u32, set: u32);
    fn mpc83xx_setup_arch();
    fn mpc837x_usb_cfg();
    fn mpc83xx_declare_of_platform_devices();
    fn mpc83xx_setup_pci();
    fn mpc83xx_ipic_init_IRQ();
    fn ipic_get_irq() -> i32;
    fn mpc83xx_restart();
    fn mpc83xx_time_init();
    fn udbg_progress();
    fn warn_on(condition: i32);
}

const MPC83XX_SICRL_OFFS: usize = 0;
const MPC83XX_SICRH_OFFS: usize = 0;
const MPC837X_SICRL_USBB_MASK: u32 = 0;
const MPC837X_SICRL_SD: u32 = 0;
const MPC837X_SICRH_SPI_MASK: u32 = 0;
const MPC837X_SICRH_SD: u32 = 0;

unsafe fn mpc837x_rdb_sd_cfg() {
    let im = ioremap(get_immrbase(), 0x1000);
    if im.is_null() {
        warn_on(1);
        return;
    }

    /*
     * On RDB boards (in contrast to MDS) USBB pins are used for SD only,
     * so we can safely mux them away from the USB block.
     */
    clrsetbits_be32(
        (im as *mut u8).add(MPC83XX_SICRL_OFFS) as *mut c_void,
        MPC837X_SICRL_USBB_MASK,
        MPC837X_SICRL_SD,
    );
    clrsetbits_be32(
        (im as *mut u8).add(MPC83XX_SICRH_OFFS) as *mut c_void,
        MPC837X_SICRH_SPI_MASK,
        MPC837X_SICRH_SD,
    );
    iounmap(im);
}

/* ************************************************************************
 *
 * Setup the architecture
 *
 */
unsafe fn mpc837x_rdb_setup_arch() {
    mpc83xx_setup_arch();
    mpc837x_usb_cfg();
    mpc837x_rdb_sd_cfg();
}

// machine_device_initcall(mpc837x_rdb, mpc83xx_declare_of_platform_devices);

static BOARD: [*const c_char; 5] = [
    b"fsl,mpc8377rdb\0".as_ptr() as *const c_char,
    b"fsl,mpc8378rdb\0".as_ptr() as *const c_char,
    b"fsl,mpc8379rdb\0".as_ptr() as *const c_char,
    b"fsl,mpc8377wlan\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

#[repr(C)]
pub struct MachineDesc {
    pub name: *const c_char,
    pub compatibles: *const *const c_char,
    pub setup_arch: Option<unsafe extern "C" fn()>,
    pub discover_phbs: Option<unsafe extern "C" fn()>,
    pub init_irq: Option<unsafe extern "C" fn()>,
    pub get_irq: Option<unsafe extern "C" fn() -> i32>,
    pub restart: Option<unsafe extern "C" fn()>,
    pub time_init: Option<unsafe extern "C" fn()>,
    pub progress: Option<unsafe extern "C" fn()>,
}

#[no_mangle]
pub static mut MPC837X_RDB: MachineDesc = MachineDesc {
    name: b"MPC837x RDB/WLAN\0".as_ptr() as *const c_char,
    compatibles: BOARD.as_ptr(),
    setup_arch: Some(mpc837x_rdb_setup_arch),
    discover_phbs: Some(mpc83xx_setup_pci),
    init_irq: Some(mpc83xx_ipic_init_IRQ),
    get_irq: Some(ipic_get_irq),
    restart: Some(mpc83xx_restart),
    time_init: Some(mpc83xx_time_init),
    progress: Some(udbg_progress),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
