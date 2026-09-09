// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Freescale 83xx USB SOC setup code
 *
 * Copyright (C) 2007 Freescale Semiconductor, Inc.
 * Author: Li Yang
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

extern "C" {
    fn of_find_compatible_node(
        from: *mut device_node,
        typ: *const c_char,
        compatible: *const c_char,
    ) -> *mut device_node;
    fn of_device_is_available(node: *const device_node) -> c_int;
    fn of_node_put(node: *mut device_node);
    fn of_get_property(
        node: *const device_node,
        name: *const c_char,
        length: *mut c_int,
    ) -> *const c_void;
    fn strcmp(lhs: *const c_char, rhs: *const c_char) -> c_int;
    fn pr_warn(format: *const c_char, ...);
    fn get_immrbase() -> usize;
    fn ioremap(offset: usize, size: usize) -> *mut c_void;
    fn iounmap(address: *mut c_void);
    fn clrsetbits_be32(address: *mut c_void, clear: u32, set: u32);
}

// Constants supplied by mpc83xx.h.
extern "C" {
    static MPC83XX_SCCR_OFFS: usize;
    static MPC837X_SCCR_USB_DRCM_11: u32;
    static MPC83XX_SICRL_OFFS: usize;
    static MPC837X_SICRL_USB_MASK: u32;
    static MPC837X_SICRL_USB_ULPI: u32;
}

pub unsafe fn mpc837x_usb_cfg() -> c_int {
    let mut immap: *mut c_void;
    let mut np: *mut device_node = core::ptr::null_mut();
    let prop: *const c_void;
    let ret: c_int = 0;

    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"fsl-usb2-dr\0".as_ptr() as *const c_char,
    );
    if np.is_null() || of_device_is_available(np) == 0 {
        of_node_put(np);
        return -19; // -ENODEV
    }
    prop = of_get_property(
        np,
        b"phy_type\0".as_ptr() as *const c_char,
        core::ptr::null_mut(),
    );

    if prop.is_null()
        || (strcmp(prop as *const c_char, b"ulpi\0".as_ptr() as *const c_char) != 0
            && strcmp(prop as *const c_char, b"serial\0".as_ptr() as *const c_char) != 0)
    {
        pr_warn(b"837x USB PHY type not supported\n\0".as_ptr() as *const c_char);
        of_node_put(np);
        return -22; // -EINVAL
    }

    /* Map IMMR space for pin and clock settings */
    immap = ioremap(get_immrbase(), 0x1000);
    if immap.is_null() {
        of_node_put(np);
        return -12; // -ENOMEM
    }

    /* Configure clock */
    clrsetbits_be32(
        immap.add(MPC83XX_SCCR_OFFS),
        MPC837X_SCCR_USB_DRCM_11,
        MPC837X_SCCR_USB_DRCM_11,
    );

    /* Configure pin mux for ULPI/serial */
    clrsetbits_be32(
        immap.add(MPC83XX_SICRL_OFFS),
        MPC837X_SICRL_USB_MASK,
        MPC837X_SICRL_USB_ULPI,
    );

    iounmap(immap);
    of_node_put(np);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
