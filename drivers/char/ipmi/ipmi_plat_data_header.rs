/* SPDX-License-Identifier: GPL-2.0+ */

/*
 * Generic code to add IPMI platform devices.
 */

// Dependency supplied by the Linux IPMI declarations.

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ipmi_plat_interface_type {
    IPMI_PLAT_IF_SI,
    IPMI_PLAT_IF_SSIF,
}

#[repr(C)]
pub struct ipmi_plat_data {
    pub iftype: ipmi_plat_interface_type,
    pub r#type: ::kernel::ffi::c_uint, /* si_type for si, SI_INVALID for others */
    pub space: ::kernel::ffi::c_uint, /* addr_space for si, intf# for ssif. */
    pub addr: ::kernel::ffi::c_ulong,
    pub regspacing: ::kernel::ffi::c_uint,
    pub regsize: ::kernel::ffi::c_uint,
    pub regshift: ::kernel::ffi::c_uint,
    pub irq: ::kernel::ffi::c_uint,
    pub slave_addr: ::kernel::ffi::c_uint,
    pub addr_source: ipmi_addr_src,
}

extern "C" {
    pub fn ipmi_platform_add(
        name: *const ::kernel::ffi::c_char,
        inst: ::kernel::ffi::c_uint,
        p: *mut ipmi_plat_data,
    ) -> *mut platform_device;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
