// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Freescale 83xx USB SOC setup code
 *
 * Copyright (C) 2007 Freescale Semiconductor, Inc.
 * Author: Li Yang
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem::MaybeUninit;

// Linux headers and local headers provide these declarations and constants.

unsafe extern "C" {
    fn of_find_compatible_node(
        from: *mut device_node,
        ty: *mut device_node,
        compatible: *const c_char,
    ) -> *mut device_node;
    fn of_get_property(
        node: *mut device_node,
        name: *const c_char,
        lenp: *mut c_int,
    ) -> *const c_void;
    fn of_get_parent(node: *mut device_node) -> *mut device_node;
    fn of_device_is_compatible(node: *mut device_node, compatible: *const c_char) -> c_int;
    fn of_node_put(node: *mut device_node);
    fn of_address_to_resource(node: *mut device_node, index: c_int, resource: *mut resource)
        -> c_int;
    fn get_immrbase() -> usize;
    fn ioremap(addr: usize, size: usize) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn resource_size(resource: *const resource) -> usize;
    fn clrsetbits_be32(addr: *mut c_void, clear: u32, set: u32);
    fn out_be32(addr: *mut c_void, value: u32);
    fn strcmp(left: *const c_char, right: *const c_char) -> c_int;
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: usize,
}

unsafe extern "C" {
    fn pr_warn(format: *const c_char, ...);
}

// __init
pub unsafe fn mpc831x_usb_cfg() -> c_int {
    let mut temp: u32;
    let mut immap: *mut c_void;
    let mut usb_regs: *mut c_void;
    let mut np: *mut device_node = core::ptr::null_mut();
    let mut immr_node: *mut device_node = core::ptr::null_mut();
    let prop: *const c_void;
    let mut res = MaybeUninit::<resource>::uninit();
    let mut ret: c_int = 0;

    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        b"fsl-usb2-dr\0".as_ptr() as *const c_char,
    );
    if np.is_null() {
        return -ENODEV;
    }
    prop = of_get_property(np, b"phy_type\0".as_ptr() as *const c_char, core::ptr::null_mut());

    // Map IMMR space for pin and clock settings
    immap = ioremap(get_immrbase(), 0x1000);
    if immap.is_null() {
        of_node_put(np);
        return -ENOMEM;
    }

    // Configure clock
    immr_node = of_get_parent(np);
    if !immr_node.is_null()
        && (of_device_is_compatible(immr_node, b"fsl,mpc8315-immr\0".as_ptr() as *const c_char)
            != 0
            || of_device_is_compatible(
                immr_node,
                b"fsl,mpc8308-immr\0".as_ptr() as *const c_char,
            ) != 0)
    {
        clrsetbits_be32(
            immap.add(MPC83XX_SCCR_OFFS),
            MPC8315_SCCR_USB_MASK,
            MPC8315_SCCR_USB_DRCM_01,
        );
    } else {
        clrsetbits_be32(
            immap.add(MPC83XX_SCCR_OFFS),
            MPC83XX_SCCR_USB_MASK,
            MPC83XX_SCCR_USB_DRCM_11,
        );
    }

    // Configure pin mux for ULPI.  There is no pin mux for UTMI
    if !prop.is_null() && strcmp(prop as *const c_char, b"ulpi\0".as_ptr() as *const c_char) == 0 {
        if of_device_is_compatible(immr_node, b"fsl,mpc8308-immr\0".as_ptr() as *const c_char) != 0 {
            clrsetbits_be32(
                immap.add(MPC83XX_SICRH_OFFS),
                MPC8308_SICRH_USB_MASK,
                MPC8308_SICRH_USB_ULPI,
            );
        } else if of_device_is_compatible(
            immr_node,
            b"fsl,mpc8315-immr\0".as_ptr() as *const c_char,
        ) != 0 {
            clrsetbits_be32(
                immap.add(MPC83XX_SICRL_OFFS),
                MPC8315_SICRL_USB_MASK,
                MPC8315_SICRL_USB_ULPI,
            );
            clrsetbits_be32(
                immap.add(MPC83XX_SICRH_OFFS),
                MPC8315_SICRH_USB_MASK,
                MPC8315_SICRH_USB_ULPI,
            );
        } else {
            clrsetbits_be32(
                immap.add(MPC83XX_SICRL_OFFS),
                MPC831X_SICRL_USB_MASK,
                MPC831X_SICRL_USB_ULPI,
            );
            clrsetbits_be32(
                immap.add(MPC83XX_SICRH_OFFS),
                MPC831X_SICRH_USB_MASK,
                MPC831X_SICRH_USB_ULPI,
            );
        }
    }

    iounmap(immap);
    of_node_put(immr_node);

    // Map USB SOC space
    ret = of_address_to_resource(np, 0, res.as_mut_ptr());
    if ret != 0 {
        of_node_put(np);
        return ret;
    }
    usb_regs = ioremap((*res.as_ptr()).start, resource_size(res.as_ptr()));

    // Using on-chip PHY
    if !prop.is_null()
        && (strcmp(prop as *const c_char, b"utmi_wide\0".as_ptr() as *const c_char) == 0
            || strcmp(prop as *const c_char, b"utmi\0".as_ptr() as *const c_char) == 0)
    {
        let refsel: u32;
        if of_device_is_compatible(immr_node, b"fsl,mpc8308-immr\0".as_ptr() as *const c_char) != 0 {
            iounmap(usb_regs);
            of_node_put(np);
            return ret;
        }
        if of_device_is_compatible(immr_node, b"fsl,mpc8315-immr\0".as_ptr() as *const c_char) != 0 {
            refsel = CONTROL_REFSEL_24MHZ;
        } else {
            refsel = CONTROL_REFSEL_48MHZ;
        }
        // Set UTMI_PHY_EN and REFSEL
        out_be32(
            usb_regs.add(FSL_USB2_CONTROL_OFFS),
            CONTROL_UTMI_PHY_EN | refsel,
        );
    // Using external UPLI PHY
    } else if !prop.is_null() && strcmp(prop as *const c_char, b"ulpi\0".as_ptr() as *const c_char) == 0 {
        // Set PHY_CLK_SEL to ULPI
        temp = CONTROL_PHY_CLK_SEL_ULPI;
        // CONFIG_USB_OTG: Set OTG_PORT when enabled by the build.
        if of_device_is_compatible(immr_node, b"fsl,mpc8308-immr\0".as_ptr() as *const c_char) == 0 {
            let dr_mode = of_get_property(np, b"dr_mode\0".as_ptr() as *const c_char, core::ptr::null_mut());
            if !dr_mode.is_null() && strcmp(dr_mode as *const c_char, b"otg\0".as_ptr() as *const c_char) == 0 {
                temp |= CONTROL_OTG_PORT;
            }
        }
        out_be32(usb_regs.add(FSL_USB2_CONTROL_OFFS), temp);
    } else {
        pr_warn(b"831x USB PHY type not supported\n\0".as_ptr() as *const c_char);
        ret = -EINVAL;
    }

    iounmap(usb_regs);
    of_node_put(np);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
