// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Freescale 83xx USB SOC setup code
 *
 * Copyright (C) 2007 Freescale Semiconductor, Inc.
 * Author: Li Yang
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

// External declarations supplied by the kernel and platform dependencies.
extern "C" {
    fn get_immrbase() -> c_ulong;
    fn ioremap(offset: c_ulong, size: usize) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn in_be32(addr: *const u32) -> u32;
    fn out_be32(addr: *mut u32, value: u32);
    fn of_find_compatible_node(
        from: *mut device_node,
        typ: *const c_char,
        compatible: *const c_char,
    ) -> *mut device_node;
    fn of_get_property(
        node: *const device_node,
        name: *const c_char,
        lenp: *mut c_int,
    ) -> *const c_void;
    fn of_node_put(node: *mut device_node);
    fn strcmp(lhs: *const c_char, rhs: *const c_char) -> c_int;
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

extern "C" {
    static __errno_placeholder: c_int;
}

// Platform constants supplied by mpc83xx.h.
extern "C" {
    static MPC83XX_SCCR_OFFS: usize;
    static MPC83XX_SICRL_OFFS: usize;
    static MPC83XX_SICRH_OFFS: usize;
    static MPC83XX_SCCR_USB_MASK: u32;
    static MPC834X_SICRL_USB_MASK: u32;
    static MPC834X_SICRH_USB_UTMI: u32;
    static MPC83XX_SCCR_USB_DRCM_11: u32;
    static MPC834X_SICRL_USB0: u32;
    static MPC834X_SICRL_USB1: u32;
    static MPC834X_SICRH_USB_UTMI: u32;
    static MPC83XX_SCCR_USB_MPHCM_11: u32;
}

unsafe extern "C" {
    fn pr_warn(fmt: *const c_char, ...);
}

#[no_mangle]
pub unsafe extern "C" fn mpc834x_usb_cfg() -> c_int {
    let mut sccr: c_ulong;
    let mut sicrl: c_ulong;
    let mut sicrh: c_ulong;
    let immap = ioremap(get_immrbase(), 0x1000);
    let mut np: *mut device_node = core::ptr::null_mut();
    let mut port0_is_dr: c_int = 0;
    let mut port1_is_dr: c_int = 0;
    let mut prop: *const c_void;
    let mut dr_mode: *const c_void;

    if immap.is_null() {
        return -12; // -ENOMEM
    }

    // Read registers
    // Note: DR and MPH must use the same clock setting in SCCR
    sccr = (in_be32(immap.add(MPC83XX_SCCR_OFFS as usize) as *const u32)
        & !MPC83XX_SCCR_USB_MASK as u32) as c_ulong;
    sicrl = (in_be32(immap.add(MPC83XX_SICRL_OFFS as usize) as *const u32)
        & !MPC834X_SICRL_USB_MASK as u32) as c_ulong;
    sicrh = (in_be32(immap.add(MPC83XX_SICRH_OFFS as usize) as *const u32)
        & !MPC834X_SICRH_USB_UTMI as u32) as c_ulong;

    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), c"fsl-usb2-dr".as_ptr());
    if !np.is_null() {
        sccr |= MPC83XX_SCCR_USB_DRCM_11 as c_ulong; // 1:3

        prop = of_get_property(np, c"phy_type".as_ptr(), core::ptr::null_mut());
        port1_is_dr = 1;
        if !prop.is_null()
            && (strcmp(prop as *const c_char, c"utmi".as_ptr()) == 0
                || strcmp(prop as *const c_char, c"utmi_wide".as_ptr()) == 0)
        {
            sicrl |= (MPC834X_SICRL_USB0 | MPC834X_SICRL_USB1) as c_ulong;
            sicrh |= MPC834X_SICRH_USB_UTMI as c_ulong;
            port0_is_dr = 1;
        } else if !prop.is_null() && strcmp(prop as *const c_char, c"serial".as_ptr()) == 0 {
            dr_mode = of_get_property(np, c"dr_mode".as_ptr(), core::ptr::null_mut());
            if !dr_mode.is_null() && strcmp(dr_mode as *const c_char, c"otg".as_ptr()) == 0 {
                sicrl |= (MPC834X_SICRL_USB0 | MPC834X_SICRL_USB1) as c_ulong;
                port0_is_dr = 1;
            } else {
                sicrl |= MPC834X_SICRL_USB1 as c_ulong;
            }
        } else if !prop.is_null() && strcmp(prop as *const c_char, c"ulpi".as_ptr()) == 0 {
            sicrl |= MPC834X_SICRL_USB1 as c_ulong;
        } else {
            pr_warn(c"834x USB PHY type not supported\n".as_ptr());
        }
        of_node_put(np);
    }
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), c"fsl-usb2-mph".as_ptr());
    if !np.is_null() {
        sccr |= MPC83XX_SCCR_USB_MPHCM_11 as c_ulong; // 1:3

        prop = of_get_property(np, c"port0".as_ptr(), core::ptr::null_mut());
        if !prop.is_null() {
            if port0_is_dr != 0 {
                pr_warn(c"834x USB port0 can't be used by both DR and MPH!\n".as_ptr());
            }
            sicrl &= !(MPC834X_SICRL_USB0 as c_ulong);
        }
        prop = of_get_property(np, c"port1".as_ptr(), core::ptr::null_mut());
        if !prop.is_null() {
            if port1_is_dr != 0 {
                pr_warn(c"834x USB port1 can't be used by both DR and MPH!\n".as_ptr());
            }
            sicrl &= !(MPC834X_SICRL_USB1 as c_ulong);
        }
        of_node_put(np);
    }

    // Write back
    out_be32(immap.add(MPC83XX_SCCR_OFFS as usize) as *mut u32, sccr as u32);
    out_be32(immap.add(MPC83XX_SICRL_OFFS as usize) as *mut u32, sicrl as u32);
    out_be32(immap.add(MPC83XX_SICRH_OFFS as usize) as *mut u32, sicrh as u32);

    iounmap(immap);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
