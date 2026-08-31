/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies:
// #include <linux/list.h>
// #include <linux/acpi.h>
// #include <linux/dax.h>
// #include <cxl.h>

unsafe extern "C" {
    pub static mut hmem_test: bool;
}

#[repr(C)]
pub struct cxl_mock_ops {
    pub list: list_head,
    pub is_mock_adev: Option<unsafe extern "C" fn(dev: *mut acpi_device) -> bool>,
    pub acpi_table_parse_cedt: Option<
        unsafe extern "C" fn(
            id: acpi_cedt_type,
            handler_arg: acpi_tbl_entry_handler_arg,
            arg: *mut core::ffi::c_void,
        ) -> core::ffi::c_int,
    >,
    pub is_mock_bridge: Option<unsafe extern "C" fn(dev: *mut device) -> bool>,
    pub acpi_evaluate_integer: Option<
        unsafe extern "C" fn(
            handle: acpi_handle,
            pathname: acpi_string,
            arguments: *mut acpi_object_list,
            data: *mut core::ffi::c_ulonglong,
        ) -> acpi_status,
    >,
    pub acpi_pci_find_root: Option<unsafe extern "C" fn(handle: acpi_handle) -> *mut acpi_pci_root>,
    pub is_mock_bus: Option<unsafe extern "C" fn(bus: *mut pci_bus) -> bool>,
    pub is_mock_port: Option<unsafe extern "C" fn(dev: *mut device) -> bool>,
    pub is_mock_dev: Option<unsafe extern "C" fn(dev: *mut device) -> bool>,
    pub devm_cxl_switch_port_decoders_setup:
        Option<unsafe extern "C" fn(port: *mut cxl_port) -> core::ffi::c_int>,
    pub devm_cxl_endpoint_decoders_setup:
        Option<unsafe extern "C" fn(port: *mut cxl_port) -> core::ffi::c_int>,
    pub cxl_endpoint_parse_cdat: Option<unsafe extern "C" fn(port: *mut cxl_port)>,
    pub devm_cxl_add_dport_by_dev: Option<
        unsafe extern "C" fn(port: *mut cxl_port, dport_dev: *mut device) -> *mut cxl_dport,
    >,
    pub hmat_get_extended_linear_cache_size: Option<
        unsafe extern "C" fn(
            backing_res: *mut resource,
            nid: core::ffi::c_int,
            cache_size: *mut resource_size_t,
        ) -> core::ffi::c_int,
    >,
    pub walk_hmem_resources:
        Option<unsafe extern "C" fn(host: *mut device, fn_: walk_hmem_fn) -> core::ffi::c_int>,
    pub region_intersects: Option<
        unsafe extern "C" fn(
            start: resource_size_t,
            size: usize,
            flags: core::ffi::c_ulong,
            desc: core::ffi::c_ulong,
        ) -> core::ffi::c_int,
    >,
    pub region_intersects_soft_reserve:
        Option<unsafe extern "C" fn(start: resource_size_t, size: usize) -> core::ffi::c_int>,
}

unsafe extern "C" {
    pub fn hmem_test_init() -> core::ffi::c_int;
    pub fn hmem_test_exit();
    pub fn register_cxl_mock_ops(ops: *mut cxl_mock_ops);
    pub fn unregister_cxl_mock_ops(ops: *mut cxl_mock_ops);
    pub fn get_cxl_mock_ops(index: *mut core::ffi::c_int) -> *mut cxl_mock_ops;
    pub fn put_cxl_mock_ops(index: core::ffi::c_int);
}
