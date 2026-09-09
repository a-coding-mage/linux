// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * arch/powerpc/platforms/83xx/mpc830x_rdb.c
 *
 * Description: MPC830x RDB board specific routines.
 * This file is based on mpc831x_rdb.c
 *
 * Copyright (C) Freescale Semiconductor, Inc. 2009. All rights reserved.
 * Copyright (C) 2010. Ilya Yanok, Emcraft Systems, yanok@emcraft.com
 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/pci.h, linux/of_platform.h, asm/time.h, asm/ipic.h, asm/udbg.h,
// sysdev/fsl_pci.h, sysdev/fsl_soc.h, and mpc83xx.h.

extern "C" {
    fn mpc83xx_setup_arch();
    fn mpc831x_usb_cfg();
    fn mpc83xx_declare_of_platform_devices();
    fn mpc83xx_setup_pci();
    fn mpc83xx_ipic_init_IRQ();
    fn ipic_get_irq() -> i32;
    fn mpc83xx_restart();
    fn mpc83xx_time_init();
    fn udbg_progress();
}

/*
 * Setup the architecture
 */
unsafe fn mpc830x_rdb_setup_arch() {
    mpc83xx_setup_arch();
    mpc831x_usb_cfg();
}

static mut board: [*const u8; 4] = [
    b"MPC8308RDB\0".as_ptr(),
    b"fsl,mpc8308rdb\0".as_ptr(),
    b"denx,mpc8308_p1m\0".as_ptr(),
    core::ptr::null(),
];

// machine_device_initcall(mpc830x_rdb, mpc83xx_declare_of_platform_devices);
// define_machine(mpc830x_rdb) {
#[repr(C)]
pub struct MachineDesc {
    pub name: *const u8,
    pub compatibles: *const *const u8,
    pub setup_arch: unsafe fn(),
    pub discover_phbs: unsafe extern "C" fn(),
    pub init_IRQ: unsafe extern "C" fn(),
    pub get_irq: unsafe extern "C" fn() -> i32,
    pub restart: unsafe extern "C" fn(),
    pub time_init: unsafe extern "C" fn(),
    pub progress: unsafe extern "C" fn(),
}

#[no_mangle]
pub static mut mpc830x_rdb: MachineDesc = MachineDesc {
    name: b"MPC830x RDB\0".as_ptr(),
    compatibles: board.as_ptr(),
    setup_arch: mpc830x_rdb_setup_arch,
    discover_phbs: mpc83xx_setup_pci,
    init_IRQ: mpc83xx_ipic_init_IRQ,
    get_irq: ipic_get_irq,
    restart: mpc83xx_restart,
    time_init: mpc83xx_time_init,
    progress: udbg_progress,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
