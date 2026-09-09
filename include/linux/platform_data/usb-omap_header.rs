/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * usb-omap.h - Platform data for the various OMAP USB IPs
 *
 * Copyright (C) 2012 Texas Instruments Incorporated - https://www.ti.com
 */

pub const OMAP3_HS_USB_PORTS: usize = 3;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usbhs_omap_port_mode {
    OMAP_USBHS_PORT_MODE_UNUSED,
    OMAP_EHCI_PORT_MODE_PHY,
    OMAP_EHCI_PORT_MODE_TLL,
    OMAP_EHCI_PORT_MODE_HSIC,
    OMAP_OHCI_PORT_MODE_PHY_6PIN_DATSE0,
    OMAP_OHCI_PORT_MODE_PHY_6PIN_DPDM,
    OMAP_OHCI_PORT_MODE_PHY_3PIN_DATSE0,
    OMAP_OHCI_PORT_MODE_PHY_4PIN_DPDM,
    OMAP_OHCI_PORT_MODE_TLL_6PIN_DATSE0,
    OMAP_OHCI_PORT_MODE_TLL_6PIN_DPDM,
    OMAP_OHCI_PORT_MODE_TLL_3PIN_DATSE0,
    OMAP_OHCI_PORT_MODE_TLL_4PIN_DPDM,
    OMAP_OHCI_PORT_MODE_TLL_2PIN_DATSE0,
    OMAP_OHCI_PORT_MODE_TLL_2PIN_DPDM,
}

#[repr(C)]
pub struct usbtll_omap_platform_data {
    pub port_mode: [usbhs_omap_port_mode; OMAP3_HS_USB_PORTS],
}

#[repr(C)]
pub struct ehci_hcd_omap_platform_data {
    pub port_mode: [usbhs_omap_port_mode; OMAP3_HS_USB_PORTS],
    pub reset_gpio_port: [::core::ffi::c_int; OMAP3_HS_USB_PORTS],
    pub regulator: [*mut regulator; OMAP3_HS_USB_PORTS],
    pub phy_reset: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct ohci_hcd_omap_platform_data {
    pub port_mode: [usbhs_omap_port_mode; OMAP3_HS_USB_PORTS],
    pub es2_compatibility: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct usbhs_omap_platform_data {
    pub nports: ::core::ffi::c_int,
    pub port_mode: [usbhs_omap_port_mode; OMAP3_HS_USB_PORTS],
    pub reset_gpio_port: [::core::ffi::c_int; OMAP3_HS_USB_PORTS],
    pub regulator: [*mut regulator; OMAP3_HS_USB_PORTS],
    pub ehci_data: *mut ehci_hcd_omap_platform_data,
    pub ohci_data: *mut ohci_hcd_omap_platform_data,
    /* OMAP3 <= ES2.1 have a single ulpi bypass control bit */
    pub single_ulpi_bypass: ::core::ffi::c_uint,
    pub es2_compatibility: ::core::ffi::c_uint,
    pub phy_reset: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct omap_musb_board_data {
    pub interface_type: u8,
    pub mode: u8,
    pub power: u16,
    pub extvbus: ::core::ffi::c_uint,
    pub set_phy_power: Option<unsafe extern "C" fn(on: u8)>,
    pub clear_irq: Option<unsafe extern "C" fn()>,
    pub set_mode: Option<unsafe extern "C" fn(mode: u8)>,
    pub reset: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum musb_interface {
    MUSB_INTERFACE_ULPI,
    MUSB_INTERFACE_UTMI,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
