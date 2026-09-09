// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2007, 2008 Freescale Semiconductor, Inc. All rights reserved.
 *
 * Author: John Rigby, <jrigby@freescale.com>, Thur Mar 29 2007
 *
 * Description:
 * MPC5121 ADS board setup
 */

// Linux and architecture headers from the original implementation provide
// the declarations referenced below.

use core::ffi::c_char;

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct MachineDesc {
    pub name: *const c_char,
    pub compatible: *const c_char,
    pub probe: Option<unsafe extern "C" fn() -> i32>,
    pub setup_arch: Option<unsafe extern "C" fn()>,
    pub discover_phbs: Option<unsafe extern "C" fn()>,
    pub init: Option<unsafe extern "C" fn()>,
    pub init_IRQ: Option<unsafe extern "C" fn()>,
    pub get_irq: Option<unsafe extern "C" fn() -> i32>,
    pub restart: Option<unsafe extern "C" fn()>,
}

extern "C" {
    fn mpc5121_ads_cpld_map();
    fn mpc512x_setup_arch();
    fn mpc83xx_add_bridge(np: *mut DeviceNode);
    fn mpc512x_init_IRQ();
    fn mpc5121_ads_cpld_pic_init();
    fn mpc512x_init_early();
    fn mpc512x_init();
    fn ipic_get_irq() -> i32;
    fn mpc512x_restart();
    fn printk(fmt: *const c_char, ...) -> i32;
}

// CONFIG_PCI controls this declaration and the PCI setup body in the kernel
// build, as in the original preprocessor conditional.
#[cfg(feature = "CONFIG_PCI")]
extern "C" {
    fn for_each_compatible_node(
        np: *mut *mut DeviceNode,
        type_: *const c_char,
        compatible: *const c_char,
    );
}

unsafe extern "C" fn mpc5121_ads_setup_arch() {
    static MESSAGE: &[u8] = b"MPC5121 ADS board from Freescale Semiconductor\0";
    printk(MESSAGE.as_ptr() as *const c_char);
    /*
     * cpld regs are needed early
     */
    mpc5121_ads_cpld_map();

    mpc512x_setup_arch();
}

unsafe extern "C" fn mpc5121_ads_setup_pci() {
    #[cfg(feature = "CONFIG_PCI")]
    {
        let mut np: *mut DeviceNode = core::ptr::null_mut();

        // for_each_compatible_node(np, "pci", "fsl,mpc5121-pci")
        for_each_compatible_node(
            &mut np,
            b"pci\0".as_ptr() as *const c_char,
            b"fsl,mpc5121-pci\0".as_ptr() as *const c_char,
        );
        mpc83xx_add_bridge(np);
    }
}

unsafe extern "C" fn mpc5121_ads_init_IRQ() {
    mpc512x_init_IRQ();
    mpc5121_ads_cpld_pic_init();
}

/*
 * Called very early, MMU is off, device-tree isn't unflattened
 */
unsafe extern "C" fn mpc5121_ads_probe() -> i32 {
    mpc512x_init_early();

    1
}

#[no_mangle]
pub static mut mpc5121_ads: MachineDesc = MachineDesc {
    name: b"MPC5121 ADS\0".as_ptr() as *const c_char,
    compatible: b"fsl,mpc5121ads\0".as_ptr() as *const c_char,
    probe: Some(mpc5121_ads_probe),
    setup_arch: Some(mpc5121_ads_setup_arch),
    discover_phbs: Some(mpc5121_ads_setup_pci),
    init: Some(mpc512x_init),
    init_IRQ: Some(mpc5121_ads_init_IRQ),
    get_irq: Some(ipic_get_irq),
    restart: Some(mpc512x_restart),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
