#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// Faithful low-level translation boundary for cxl/core/port.c.
//
// This implementation is Linux-kernel code. The referenced kernel and CXL
// types, globals, macros, and functions are intentionally left as external
// dependencies, as in the source translation contract.

#[cfg(any())]
mod translated {
    // The original implementation is enabled by the target kernel binding.
    // Its declarations retain C ABI/layout semantics through the bindings.
    include!("port.c");
}

// External declarations corresponding to the source-level exported API.
// Concrete definitions are supplied by the surrounding CXL kernel bindings.
extern "C" {
    pub fn cxl_num_decoders_committed(port: *mut c_void) -> i32;
    pub fn is_endpoint_decoder(dev: *mut c_void) -> bool;
    pub fn is_root_decoder(dev: *mut c_void) -> bool;
    pub fn is_switch_decoder(dev: *mut c_void) -> bool;
    pub fn is_cxl_port(dev: *const c_void) -> bool;
    pub fn to_cxl_port(dev: *const c_void) -> *mut c_void;
    pub fn parent_port_of(port: *mut c_void) -> *mut c_void;
    pub fn cxl_port_setup_regs(port: *mut c_void, component_reg_phys: u64) -> i32;
    pub fn devm_cxl_add_port(
        host: *mut c_void,
        uport_dev: *mut c_void,
        component_reg_phys: u64,
        parent_dport: *mut c_void,
    ) -> *mut c_void;
    pub fn devm_cxl_add_root(host: *mut c_void) -> *mut c_void;
    pub fn cxl_port_to_pci_bus(port: *mut c_void) -> *mut c_void;
    pub fn devm_cxl_register_pci_bus(
        host: *mut c_void,
        uport_dev: *mut c_void,
        bus: *mut c_void,
    ) -> i32;
    pub fn find_cxl_root(port: *mut c_void) -> *mut c_void;
    pub fn devm_cxl_add_dport(
        port: *mut c_void,
        dport_dev: *mut c_void,
        port_id: i32,
        component_reg_phys: u64,
    ) -> *mut c_void;
    pub fn devm_cxl_add_rch_dport(
        port: *mut c_void,
        dport_dev: *mut c_void,
        port_id: i32,
        rcrb: u64,
    ) -> *mut c_void;
    pub fn devm_cxl_enumerate_ports(cxlmd: *mut c_void) -> i32;
    pub fn cxl_pci_find_port(pdev: *mut c_void, dport: *mut *mut c_void) -> *mut c_void;
    pub fn cxl_mem_find_port(cxlmd: *mut c_void, dport: *mut *mut c_void) -> *mut c_void;
    pub fn cxl_root_decoder_alloc(port: *mut c_void, nr_targets: u32) -> *mut c_void;
    pub fn cxl_switch_decoder_alloc(port: *mut c_void, nr_targets: u32) -> *mut c_void;
    pub fn cxl_endpoint_decoder_alloc(port: *mut c_void) -> *mut c_void;
    pub fn cxl_decoder_add_locked(cxld: *mut c_void) -> i32;
    pub fn cxl_decoder_add(cxld: *mut c_void) -> i32;
    pub fn cxl_decoder_autoremove(host: *mut c_void, cxld: *mut c_void) -> i32;
    pub fn __cxl_driver_register(cxl_drv: *mut c_void, owner: *mut c_void, modname: *const i8) -> i32;
    pub fn cxl_driver_unregister(cxl_drv: *mut c_void);
    pub fn cxl_bus_rescan();
    pub fn cxl_bus_drain();
    pub fn schedule_cxl_memdev_detach(cxlmd: *mut c_void) -> bool;
    pub fn cxl_endpoint_get_perf_coordinates(port: *mut c_void, coord: *mut c_void) -> i32;
    pub fn cxl_port_get_switch_dport_bandwidth(port: *mut c_void, coord: *mut c_void) -> i32;
    pub fn cxl_debugfs_create_dir(dir: *const i8) -> *mut c_void;
}

use core::ffi::c_void;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
