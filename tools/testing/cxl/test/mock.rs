// SPDX-License-Identifier: GPL-2.0-only
// Copyright(c) 2021 Intel Corporation. All rights reserved.

// Rust translation of testing/cxl/test/mock.c.
// C include dependencies preserved as external declarations:
// linux/libnvdimm.h, linux/rculist.h, linux/device.h, linux/export.h,
// linux/acpi.h, linux/pci.h, cxlmem.h, cxlpci.h, and "mock.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::MaybeUninit;
use core::ptr;

pub type bool_ = bool;
pub type size_t = usize;
pub type resource_size_t = u64;
pub type acpi_status = u32;
pub type acpi_handle = *mut c_void;
pub type acpi_string = *mut c_char;
pub type acpi_cedt_type = c_int;
pub type acpi_tbl_entry_handler_arg = Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> c_int>;
pub type walk_hmem_fn = Option<unsafe extern "C" fn(*mut device, *mut resource) -> c_int>;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct srcu_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpi_device {
    pub fwnode: fwnode_handle,
}

#[repr(C)]
pub struct acpi_object_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpi_pci_root {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub parent: *mut device,
}

#[repr(C)]
pub struct nvdimm_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nvdimm_bus_descriptor {
    pub provider_name: *const c_char,
}

#[repr(C)]
pub struct cxl_port {
    pub uport_dev: *mut device,
}

#[repr(C)]
pub struct cxl_dev_state {
    pub dev: *mut device,
}

#[repr(C)]
pub struct cxl_memdev {
    pub dev: device,
}

#[repr(C)]
pub struct cxl_dport_rcrb {
    pub base: resource_size_t,
}

#[repr(C)]
pub struct cxl_dport {
    pub rcrb: cxl_dport_rcrb,
    pub rch: bool,
}

#[repr(C)]
pub struct cxl_mock_ops {
    pub list: list_head,
    pub is_mock_adev: unsafe extern "C" fn(*mut acpi_device) -> bool,
    pub acpi_table_parse_cedt:
        unsafe extern "C" fn(acpi_cedt_type, acpi_tbl_entry_handler_arg, *mut c_void) -> c_int,
    pub acpi_evaluate_integer: unsafe extern "C" fn(
        acpi_handle,
        acpi_string,
        *mut acpi_object_list,
        *mut u64,
    ) -> acpi_status,
    pub hmat_get_extended_linear_cache_size:
        unsafe extern "C" fn(*mut resource, c_int, *mut resource_size_t) -> c_int,
    pub acpi_pci_find_root: unsafe extern "C" fn(acpi_handle) -> *mut acpi_pci_root,
    pub is_mock_dev: unsafe extern "C" fn(*mut device) -> bool,
    pub is_mock_port: unsafe extern "C" fn(*mut device) -> bool,
    pub devm_cxl_switch_port_decoders_setup: unsafe extern "C" fn(*mut cxl_port) -> c_int,
    pub devm_cxl_endpoint_decoders_setup: unsafe extern "C" fn(*mut cxl_port) -> c_int,
    pub devm_cxl_add_dport_by_dev:
        unsafe extern "C" fn(*mut cxl_port, *mut device) -> *mut cxl_dport,
    pub cxl_endpoint_parse_cdat: unsafe extern "C" fn(*mut cxl_port),
    pub region_intersects:
        unsafe extern "C" fn(resource_size_t, size_t, c_ulong, c_ulong) -> c_int,
    pub region_intersects_soft_reserve: unsafe extern "C" fn(resource_size_t, size_t) -> c_int,
    pub walk_hmem_resources: unsafe extern "C" fn(*mut device, walk_hmem_fn) -> c_int,
}

static mut mock: list_head = list_head {
    next: ptr::null_mut(),
    prev: ptr::null_mut(),
};

static mut cxl_mock_srcu: MaybeUninit<srcu_struct> = MaybeUninit::uninit();

const CXL_RESOURCE_NONE: resource_size_t = !0;
const cxl_test: &[u8] = b"cxl_test\0";
const hmem_platform_1: &[u8] = b"hmem_platform.1\0";

extern "C" {
    fn list_add_rcu(new: *mut list_head, head: *mut list_head);
    fn list_del_rcu(entry: *mut list_head);
    fn srcu_read_lock(sp: *mut srcu_struct) -> c_int;
    fn srcu_read_unlock(sp: *mut srcu_struct, idx: c_int);
    fn synchronize_srcu(sp: *mut srcu_struct);
    fn is_acpi_device_node(fwnode: *const fwnode_handle) -> bool;
    fn acpi_table_parse_cedt(
        id: acpi_cedt_type,
        handler_arg: acpi_tbl_entry_handler_arg,
        arg: *mut c_void,
    ) -> c_int;
    fn acpi_evaluate_integer(
        handle: acpi_handle,
        pathname: acpi_string,
        arguments: *mut acpi_object_list,
        data: *mut u64,
    ) -> acpi_status;
    fn hmat_get_extended_linear_cache_size(
        backing_res: *mut resource,
        nid: c_int,
        cache_size: *mut resource_size_t,
    ) -> c_int;
    fn acpi_pci_find_root(handle: acpi_handle) -> *mut acpi_pci_root;
    fn nvdimm_bus_register(
        dev: *mut device,
        nd_desc: *mut nvdimm_bus_descriptor,
    ) -> *mut nvdimm_bus;
    fn devm_cxl_switch_port_decoders_setup(port: *mut cxl_port) -> c_int;
    fn devm_cxl_endpoint_decoders_setup(port: *mut cxl_port) -> c_int;
    fn cxl_await_media_ready(cxlds: *mut cxl_dev_state) -> c_int;
    fn devm_cxl_add_dport(
        port: *mut cxl_port,
        dport_dev: *mut device,
        port_id: c_int,
        component_reg_phys: resource_size_t,
    ) -> *mut cxl_dport;
    fn devm_cxl_add_rch_dport(
        port: *mut cxl_port,
        dport_dev: *mut device,
        port_id: c_int,
        rcrb: resource_size_t,
    ) -> *mut cxl_dport;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn to_cxl_memdev(dev: *mut device) -> *mut cxl_memdev;
    fn cxl_endpoint_parse_cdat(port: *mut cxl_port);
    fn devm_cxl_add_dport_by_dev(port: *mut cxl_port, dport_dev: *mut device) -> *mut cxl_dport;
    fn region_intersects(
        start: resource_size_t,
        size: size_t,
        flags: c_ulong,
        desc: c_ulong,
    ) -> c_int;
    fn region_intersects_soft_reserve(start: resource_size_t, size: size_t) -> c_int;
    fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    fn dev_name(dev: *const device) -> *const c_char;
    fn walk_hmem_resources(host: *mut device, fn_: walk_hmem_fn) -> c_int;
}

unsafe fn list_first_or_null_rcu_mock(head: *mut list_head) -> *mut cxl_mock_ops {
    if (*head).next.is_null() || (*head).next == head {
        ptr::null_mut()
    } else {
        (*head).next as *mut cxl_mock_ops
    }
}

#[no_mangle]
pub unsafe extern "C" fn register_cxl_mock_ops(ops: *mut cxl_mock_ops) {
    list_add_rcu(&mut (*ops).list, &mut mock);
}
// EXPORT_SYMBOL_GPL(register_cxl_mock_ops);

#[no_mangle]
pub unsafe extern "C" fn unregister_cxl_mock_ops(ops: *mut cxl_mock_ops) {
    list_del_rcu(&mut (*ops).list);
    synchronize_srcu(cxl_mock_srcu.as_mut_ptr());
}
// EXPORT_SYMBOL_GPL(unregister_cxl_mock_ops);

#[no_mangle]
pub unsafe extern "C" fn get_cxl_mock_ops(index: *mut c_int) -> *mut cxl_mock_ops {
    *index = srcu_read_lock(cxl_mock_srcu.as_mut_ptr());
    list_first_or_null_rcu_mock(&mut mock)
}
// EXPORT_SYMBOL_GPL(get_cxl_mock_ops);

#[no_mangle]
pub unsafe extern "C" fn put_cxl_mock_ops(index: c_int) {
    srcu_read_unlock(cxl_mock_srcu.as_mut_ptr(), index);
}
// EXPORT_SYMBOL_GPL(put_cxl_mock_ops);

#[no_mangle]
pub unsafe extern "C" fn __wrap_is_acpi_device_node(fwnode: *const fwnode_handle) -> bool {
    let adev = fwnode as *mut acpi_device;
    let mut index: c_int = 0;
    let ops = get_cxl_mock_ops(&mut index);
    let mut retval = false;

    if !ops.is_null() {
        retval = ((*ops).is_mock_adev)(adev);
    }

    if !retval {
        retval = is_acpi_device_node(fwnode);
    }

    put_cxl_mock_ops(index);
    retval
}
// EXPORT_SYMBOL(__wrap_is_acpi_device_node);

#[no_mangle]
pub unsafe extern "C" fn __wrap_acpi_table_parse_cedt(
    id: acpi_cedt_type,
    handler_arg: acpi_tbl_entry_handler_arg,
    arg: *mut c_void,
) -> c_int {
    let mut index: c_int = 0;
    let rc: c_int;
    let ops = get_cxl_mock_ops(&mut index);

    if !ops.is_null() {
        rc = ((*ops).acpi_table_parse_cedt)(id, handler_arg, arg);
    } else {
        rc = acpi_table_parse_cedt(id, handler_arg, arg);
    }

    put_cxl_mock_ops(index);
    rc
}
// EXPORT_SYMBOL_NS_GPL(__wrap_acpi_table_parse_cedt, "ACPI");

#[no_mangle]
pub unsafe extern "C" fn __wrap_acpi_evaluate_integer(
    handle: acpi_handle,
    pathname: acpi_string,
    arguments: *mut acpi_object_list,
    data: *mut u64,
) -> acpi_status {
    let mut index: c_int = 0;
    let ops = get_cxl_mock_ops(&mut index);
    let status: acpi_status;

    if !ops.is_null() {
        status = ((*ops).acpi_evaluate_integer)(handle, pathname, arguments, data);
    } else {
        status = acpi_evaluate_integer(handle, pathname, arguments, data);
    }
    put_cxl_mock_ops(index);
    status
}
// EXPORT_SYMBOL(__wrap_acpi_evaluate_integer);

#[no_mangle]
pub unsafe extern "C" fn __wrap_hmat_get_extended_linear_cache_size(
    backing_res: *mut resource,
    nid: c_int,
    cache_size: *mut resource_size_t,
) -> c_int {
    let mut index: c_int = 0;
    let rc: c_int;
    let ops = get_cxl_mock_ops(&mut index);

    if !ops.is_null() {
        rc = ((*ops).hmat_get_extended_linear_cache_size)(backing_res, nid, cache_size);
    } else {
        rc = hmat_get_extended_linear_cache_size(backing_res, nid, cache_size);
    }

    put_cxl_mock_ops(index);
    rc
}
// EXPORT_SYMBOL_GPL(__wrap_hmat_get_extended_linear_cache_size);

#[no_mangle]
pub unsafe extern "C" fn __wrap_acpi_pci_find_root(handle: acpi_handle) -> *mut acpi_pci_root {
    let mut index: c_int = 0;
    let root: *mut acpi_pci_root;
    let ops = get_cxl_mock_ops(&mut index);

    if !ops.is_null() {
        root = ((*ops).acpi_pci_find_root)(handle);
    } else {
        root = acpi_pci_find_root(handle);
    }

    put_cxl_mock_ops(index);
    root
}
// EXPORT_SYMBOL_GPL(__wrap_acpi_pci_find_root);

#[no_mangle]
pub unsafe extern "C" fn __wrap_nvdimm_bus_register(
    dev: *mut device,
    nd_desc: *mut nvdimm_bus_descriptor,
) -> *mut nvdimm_bus {
    let mut index: c_int = 0;
    let ops = get_cxl_mock_ops(&mut index);

    if !ops.is_null() && ((*ops).is_mock_dev)((*(*dev).parent).parent) {
        (*nd_desc).provider_name = cxl_test.as_ptr() as *const c_char;
    }
    put_cxl_mock_ops(index);

    nvdimm_bus_register(dev, nd_desc)
}
// EXPORT_SYMBOL_GPL(__wrap_nvdimm_bus_register);

#[no_mangle]
pub unsafe extern "C" fn __wrap_devm_cxl_switch_port_decoders_setup(port: *mut cxl_port) -> c_int {
    let mut rc: c_int;
    let mut index: c_int = 0;
    let ops = get_cxl_mock_ops(&mut index);

    if !ops.is_null() && ((*ops).is_mock_port)((*port).uport_dev) {
        rc = ((*ops).devm_cxl_switch_port_decoders_setup)(port);
    } else {
        rc = devm_cxl_switch_port_decoders_setup(port);
    }
    put_cxl_mock_ops(index);

    rc
}
// EXPORT_SYMBOL_NS_GPL(__wrap_devm_cxl_switch_port_decoders_setup, "CXL");

#[no_mangle]
pub unsafe extern "C" fn __wrap_devm_cxl_endpoint_decoders_setup(port: *mut cxl_port) -> c_int {
    let mut rc: c_int;
    let mut index: c_int = 0;
    let ops = get_cxl_mock_ops(&mut index);

    if !ops.is_null() && ((*ops).is_mock_port)((*port).uport_dev) {
        rc = ((*ops).devm_cxl_endpoint_decoders_setup)(port);
    } else {
        rc = devm_cxl_endpoint_decoders_setup(port);
    }
    put_cxl_mock_ops(index);

    rc
}
// EXPORT_SYMBOL_NS_GPL(__wrap_devm_cxl_endpoint_decoders_setup, "CXL");

#[no_mangle]
pub unsafe extern "C" fn __wrap_cxl_await_media_ready(cxlds: *mut cxl_dev_state) -> c_int {
    let mut rc: c_int;
    let mut index: c_int = 0;
    let ops = get_cxl_mock_ops(&mut index);

    if !ops.is_null() && ((*ops).is_mock_dev)((*cxlds).dev) {
        rc = 0;
    } else {
        rc = cxl_await_media_ready(cxlds);
    }
    put_cxl_mock_ops(index);

    rc
}
// EXPORT_SYMBOL_NS_GPL(__wrap_cxl_await_media_ready, "CXL");

#[no_mangle]
pub unsafe extern "C" fn __wrap_devm_cxl_add_rch_dport(
    port: *mut cxl_port,
    dport_dev: *mut device,
    port_id: c_int,
    rcrb: resource_size_t,
) -> *mut cxl_dport {
    let mut index: c_int = 0;
    let mut dport: *mut cxl_dport;
    let ops = get_cxl_mock_ops(&mut index);

    if !ops.is_null() && ((*ops).is_mock_port)(dport_dev) {
        dport = devm_cxl_add_dport(port, dport_dev, port_id, CXL_RESOURCE_NONE);
        if !IS_ERR(dport as *const c_void) {
            (*dport).rcrb.base = rcrb;
            (*dport).rch = true;
        }
    } else {
        dport = devm_cxl_add_rch_dport(port, dport_dev, port_id, rcrb);
    }
    put_cxl_mock_ops(index);

    dport
}
// EXPORT_SYMBOL_NS_GPL(__wrap_devm_cxl_add_rch_dport, "CXL");

#[no_mangle]
pub unsafe extern "C" fn __wrap_cxl_endpoint_parse_cdat(port: *mut cxl_port) {
    let mut index: c_int = 0;
    let ops = get_cxl_mock_ops(&mut index);
    let cxlmd = to_cxl_memdev((*port).uport_dev);

    if !ops.is_null() && ((*ops).is_mock_dev)((*cxlmd).dev.parent) {
        ((*ops).cxl_endpoint_parse_cdat)(port);
    } else {
        cxl_endpoint_parse_cdat(port);
    }
    put_cxl_mock_ops(index);
}
// EXPORT_SYMBOL_NS_GPL(__wrap_cxl_endpoint_parse_cdat, "CXL");

#[no_mangle]
pub unsafe extern "C" fn __wrap_devm_cxl_add_dport_by_dev(
    port: *mut cxl_port,
    dport_dev: *mut device,
) -> *mut cxl_dport {
    let mut index: c_int = 0;
    let ops = get_cxl_mock_ops(&mut index);
    let dport: *mut cxl_dport;

    if !ops.is_null() && ((*ops).is_mock_port)((*port).uport_dev) {
        dport = ((*ops).devm_cxl_add_dport_by_dev)(port, dport_dev);
    } else {
        dport = devm_cxl_add_dport_by_dev(port, dport_dev);
    }
    put_cxl_mock_ops(index);

    dport
}
// EXPORT_SYMBOL_NS_GPL(__wrap_devm_cxl_add_dport_by_dev, "CXL");

#[no_mangle]
pub unsafe extern "C" fn __wrap_region_intersects(
    start: resource_size_t,
    size: size_t,
    flags: c_ulong,
    desc: c_ulong,
) -> c_int {
    let mut rc: c_int = -1;
    let mut index: c_int = 0;
    let ops = get_cxl_mock_ops(&mut index);

    if !ops.is_null() {
        rc = ((*ops).region_intersects)(start, size, flags, desc);
    }
    if rc < 0 {
        rc = region_intersects(start, size, flags, desc);
    }
    put_cxl_mock_ops(index);

    rc
}
// EXPORT_SYMBOL_GPL(__wrap_region_intersects);

#[no_mangle]
pub unsafe extern "C" fn __wrap_region_intersects_soft_reserve(
    start: resource_size_t,
    size: size_t,
) -> c_int {
    let mut rc: c_int = -1;
    let mut index: c_int = 0;
    let ops = get_cxl_mock_ops(&mut index);

    if !ops.is_null() {
        rc = ((*ops).region_intersects_soft_reserve)(start, size);
    }
    if rc < 0 {
        rc = region_intersects_soft_reserve(start, size);
    }
    put_cxl_mock_ops(index);

    rc
}
// EXPORT_SYMBOL_GPL(__wrap_region_intersects_soft_reserve);

#[no_mangle]
pub unsafe extern "C" fn __wrap_walk_hmem_resources(host: *mut device, fn_: walk_hmem_fn) -> c_int {
    let mut index: c_int = 0;
    let mut rc: c_int = 0;
    let is_mock = strcmp(dev_name(host), hmem_platform_1.as_ptr() as *const c_char) == 0;
    let ops = get_cxl_mock_ops(&mut index);

    if is_mock {
        if !ops.is_null() {
            rc = ((*ops).walk_hmem_resources)(host, fn_);
        }
    } else {
        rc = walk_hmem_resources(host, fn_);
    }
    put_cxl_mock_ops(index);
    rc
}
// EXPORT_SYMBOL_GPL(__wrap_walk_hmem_resources);

// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("cxl_test: emulation module");
// MODULE_IMPORT_NS("ACPI");
// MODULE_IMPORT_NS("CXL");
