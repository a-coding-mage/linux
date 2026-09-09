/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: linux/device.h and linux/of.h.

pub struct of_phandle_args;
pub struct reserved_mem_ops;
pub struct resource;

#[repr(C)]
pub struct reserved_mem {
    pub name: *const ::core::ffi::c_char,
    pub ops: *const reserved_mem_ops,
    pub base: phys_addr_t,
    pub size: phys_addr_t,
    pub priv_: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct reserved_mem_ops {
    pub node_validate:
        Option<unsafe extern "C" fn(fdt_node: ::core::ffi::c_ulong, align: *mut phys_addr_t) -> ::core::ffi::c_int>,
    pub node_fixup: Option<unsafe extern "C" fn(
        fdt_node: ::core::ffi::c_ulong,
        base: phys_addr_t,
        size: phys_addr_t,
    ) -> ::core::ffi::c_int>,
    pub node_init: Option<unsafe extern "C" fn(
        fdt_node: ::core::ffi::c_ulong,
        rmem: *mut reserved_mem,
    ) -> ::core::ffi::c_int>,
    pub device_init: Option<unsafe extern "C" fn(
        rmem: *mut reserved_mem,
        dev: *mut device,
    ) -> ::core::ffi::c_int>,
    pub device_release:
        Option<unsafe extern "C" fn(rmem: *mut reserved_mem, dev: *mut device)>,
}

// Build-time condition: CONFIG_OF_RESERVED_MEM selects declarations below;
// otherwise the inline stubs are used.

#[cfg(CONFIG_OF_RESERVED_MEM)]
#[macro_export]
macro_rules! RESERVEDMEM_OF_DECLARE {
    ($name:ident, $compat:expr, $ops:expr) => {
        // Corresponds to _OF_DECLARE(reservedmem, ...).
        $crate::_OF_DECLARE!(reservedmem, $name, $compat, $ops, reserved_mem_ops);
    };
}

#[cfg(CONFIG_OF_RESERVED_MEM)]
extern "C" {
    pub fn of_reserved_mem_device_init_by_idx(
        dev: *mut device,
        np: *mut device_node,
        idx: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn of_reserved_mem_device_init_by_name(
        dev: *mut device,
        np: *mut device_node,
        name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn of_reserved_mem_device_release(dev: *mut device);
    pub fn devm_of_reserved_mem_device_init(dev: *mut device) -> ::core::ffi::c_int;
    pub fn of_reserved_mem_lookup(np: *mut device_node) -> *mut reserved_mem;
    pub fn of_reserved_mem_region_to_resource(
        np: *const device_node,
        idx: ::core::ffi::c_uint,
        res: *mut resource,
    ) -> ::core::ffi::c_int;
    pub fn of_reserved_mem_region_to_resource_byname(
        np: *const device_node,
        name: *const ::core::ffi::c_char,
        res: *mut resource,
    ) -> ::core::ffi::c_int;
    pub fn of_reserved_mem_region_count(np: *const device_node) -> ::core::ffi::c_int;
}

#[cfg(not(CONFIG_OF_RESERVED_MEM))]
#[macro_export]
macro_rules! RESERVEDMEM_OF_DECLARE {
    ($name:ident, $compat:expr, $ops:expr) => {
        // Corresponds to _OF_DECLARE_STUB(reservedmem, ...).
    };
}

#[cfg(not(CONFIG_OF_RESERVED_MEM))]
pub unsafe fn of_reserved_mem_device_init_by_idx(
    _dev: *mut device,
    _np: *mut device_node,
    _idx: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    -ENOSYS
}

#[cfg(not(CONFIG_OF_RESERVED_MEM))]
pub unsafe fn of_reserved_mem_device_init_by_name(
    _dev: *mut device,
    _np: *mut device_node,
    _name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    -ENOSYS
}

#[cfg(not(CONFIG_OF_RESERVED_MEM))]
pub unsafe fn of_reserved_mem_device_release(_pdev: *mut device) {}

#[cfg(not(CONFIG_OF_RESERVED_MEM))]
pub unsafe fn devm_of_reserved_mem_device_init(_dev: *mut device) -> ::core::ffi::c_int {
    -EOPNOTSUPP
}

#[cfg(not(CONFIG_OF_RESERVED_MEM))]
pub unsafe fn of_reserved_mem_lookup(_np: *mut device_node) -> *mut reserved_mem {
    ::core::ptr::null_mut()
}

#[cfg(not(CONFIG_OF_RESERVED_MEM))]
pub unsafe fn of_reserved_mem_region_to_resource(
    _np: *const device_node,
    _idx: ::core::ffi::c_uint,
    _res: *mut resource,
) -> ::core::ffi::c_int {
    -ENOSYS
}

#[cfg(not(CONFIG_OF_RESERVED_MEM))]
pub unsafe fn of_reserved_mem_region_to_resource_byname(
    _np: *const device_node,
    _name: *const ::core::ffi::c_char,
    _res: *mut resource,
) -> ::core::ffi::c_int {
    -ENOSYS
}

#[cfg(not(CONFIG_OF_RESERVED_MEM))]
pub unsafe fn of_reserved_mem_region_count(_np: *const device_node) -> ::core::ffi::c_int {
    0
}

/**
 * of_reserved_mem_device_init() - assign reserved memory region to given device
 * @dev: Pointer to the device to configure
 *
 * This function assigns respective DMA-mapping operations based on the first
 * reserved memory region specified by 'memory-region' property in device tree
 * node of the given device.
 *
 * Returns error code or zero on success.
 */
#[inline]
pub unsafe fn of_reserved_mem_device_init(dev: *mut device) -> ::core::ffi::c_int {
    of_reserved_mem_device_init_by_idx(dev, (*dev).of_node, 0)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
