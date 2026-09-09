/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Texas Instruments' K3 TI SCI INTA MSI helper
 *
 * Copyright (C) 2018-2019 Texas Instruments Incorporated - https://www.ti.com/
 *	Lokesh Vutla <lokeshvutla@ti.com>
 */

/* Declarations supplied by linux/msi.h and linux/soc/ti/ti_sci_protocol.h. */

#[repr(C)]
pub struct irq_domain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct msi_domain_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ti_sci_resource {
    _private: [u8; 0],
}

extern "C" {
    pub fn ti_sci_inta_msi_create_irq_domain(
        fwnode: *mut fwnode_handle,
        info: *mut msi_domain_info,
        parent: *mut irq_domain,
    ) -> *mut irq_domain;

    pub fn ti_sci_inta_msi_domain_alloc_irqs(
        dev: *mut device,
        res: *mut ti_sci_resource,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
