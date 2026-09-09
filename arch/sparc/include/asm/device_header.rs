/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Arch specific extensions to struct device
 */

// Dependency supplied by the translated openprom header.
use core::ffi::c_void;

// Forward declarations from the C header.
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dev_archdata {
    pub iommu: *mut c_void,
    pub stc: *mut c_void,
    pub host_controller: *mut c_void,
    pub op: *mut platform_device,
    pub numa_node: ::core::ffi::c_int,
}

extern "C" {
    pub fn of_propagate_archdata(bus: *mut platform_device);
}

#[repr(C)]
pub struct pdev_archdata {
    // `struct resource` and PROMREG_MAX/PROMINTR_MAX are supplied by dependencies.
    pub resource: [resource; PROMREG_MAX],
    pub irqs: [::core::ffi::c_uint; PROMINTR_MAX],
    pub num_irqs: ::core::ffi::c_int,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
