// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2016-17 IBM Corp.
 */

// C includes and kernel-provided declarations are supplied by the surrounding
// translation environment.

use core::ffi::c_void;

#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct spinlock { _private: [u8; 0] }
#[repr(C)] pub struct ida { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct resource { pub start: u64, pub end: u64 }
#[repr(C)] pub struct platform_device {
    pub dev: device,
    pub name: *const i8,
    pub num_resources: u32,
    pub resource: *mut resource,
}
#[repr(C)] pub struct xive_irq_data { pub trig_page: u64 }
#[repr(C)] pub struct vas_instance {
    pub node: list_head,
    pub ida: ida,
    pub mutex: mutex,
    pub fault_lock: spinlock,
    pub name: *mut i8,
    pub vas_id: i32,
    pub pdev: *mut platform_device,
    pub hvwc_bar_start: u64,
    pub uwc_bar_start: u64,
    pub paste_base_addr: u64,
    pub paste_win_id_shift: u64,
    pub virq: u32,
    pub irq_port: u64,
}
#[repr(C)] pub struct of_device_id { pub compatible: *const i8 }
#[repr(C)] pub struct platform_driver { _private: [u8; 0] }

extern "C" {
    static mut vas_mutex: mutex;
    static mut vas_instances: list_head;
    static mut cpu_vas_id: i32;
    fn request_threaded_irq(irq: u32, handler: unsafe extern "C" fn(), thread_fn: unsafe extern "C" fn(), flags: u32, name: *const i8, dev: *mut c_void) -> i32;
    fn vas_fault_handler();
    fn vas_fault_thread_fn();
    fn vas_setup_fault_window(vinst: *mut vas_instance) -> i32;
    fn free_irq(irq: u32, dev: *mut c_void);
    fn of_property_read_u32(dn: *mut device_node, name: *const i8, value: *mut i32) -> i32;
    fn kzalloc_vas_instance() -> *mut vas_instance;
    fn kasprintf(flags: u32, fmt: *const i8, value: i32) -> *mut i8;
    fn kfree(ptr: *mut c_void);
    fn xive_native_alloc_irq_on_chip(chipid: u32) -> u32;
    fn irq_create_mapping(domain: *mut c_void, hwirq: u32) -> u32;
    fn irq_get_chip_data(virq: u32) -> *mut xive_irq_data;
    fn cpu_to_chip_id(cpu: i32) -> i32;
    fn of_get_ibm_chip_id(dn: *mut device_node) -> i32;
    fn smp_processor_id() -> i32;
    fn vas_instance_init_dbgdir(vinst: *mut vas_instance);
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn platform_driver_unregister(driver: *mut platform_driver);
    fn of_platform_device_create(dn: *mut device_node, data: *mut c_void, parent: *mut c_void) -> *mut platform_device;
    fn for_each_compatible_node(dn: *mut *mut device_node, from: *mut c_void, compatible: *const i8);
}

unsafe fn vas_irq_fault_window_setup(vinst: *mut vas_instance) -> i32 {
    let mut rc = request_threaded_irq((*vinst).virq, vas_fault_handler, vas_fault_thread_fn, 0, (*vinst).name, vinst as *mut c_void);
    if rc != 0 { return rc; }
    rc = vas_setup_fault_window(vinst);
    if rc != 0 { free_irq((*vinst).virq, vinst as *mut c_void); }
    rc
}

unsafe fn init_vas_instance(pdev: *mut platform_device) -> i32 {
    let dn = (*pdev).dev.of_node;
    let mut vasid = 0i32;
    let mut chipid = 0i32;
    if of_property_read_u32(dn, b"ibm,vas-id\0".as_ptr() as *const i8, &mut vasid) != 0 { return -19; }
    if of_property_read_u32(dn, b"ibm,chip-id\0".as_ptr() as *const i8, &mut chipid) != 0 { return -19; }
    if (*pdev).num_resources != 4 { return -19; }
    let vinst = kzalloc_vas_instance();
    if vinst.is_null() { return -12; }
    (*vinst).name = kasprintf(0, b"vas-%d\0".as_ptr() as *const i8, vasid);
    if (*vinst).name.is_null() { kfree(vinst as *mut c_void); return -12; }
    (*vinst).vas_id = vasid; (*vinst).pdev = pdev;
    for cpu in 0..1024 { if cpu_to_chip_id(cpu) == of_get_ibm_chip_id(dn) { cpu_vas_id = vasid; } }
    let hwirq = xive_native_alloc_irq_on_chip(chipid as u32);
    if hwirq == 0 { return -2; }
    (*vinst).virq = irq_create_mapping(core::ptr::null_mut(), hwirq);
    if (*vinst).virq == 0 { return -22; }
    let xd = irq_get_chip_data((*vinst).virq);
    if xd.is_null() { return -22; }
    (*vinst).irq_port = (*xd).trig_page;
    let rc = vas_irq_fault_window_setup(vinst);
    if rc != 0 { (*vinst).virq = 0; }
    vas_instance_init_dbgdir(vinst); dev_set_drvdata(&mut (*pdev).dev, vinst as *mut c_void); 0
}

#[no_mangle] pub unsafe extern "C" fn find_vas_instance(mut vasid: i32) -> *mut vas_instance {
    if vasid == -1 { vasid = cpu_vas_id; }
    core::ptr::null_mut()
}
#[no_mangle] pub unsafe extern "C" fn chip_to_vas_id(chipid: i32) -> i32 {
    for cpu in 0..1024 { if cpu_to_chip_id(cpu) == chipid { return cpu_vas_id; } } -1
}

unsafe fn vas_probe(pdev: *mut platform_device) -> i32 { init_vas_instance(pdev) }
static mut powernv_vas_match: [of_device_id; 2] = [of_device_id { compatible: b"ibm,vas\0".as_ptr() as *const i8 }, of_device_id { compatible: core::ptr::null() }];
static mut vas_driver: platform_driver = platform_driver { _private: [] };
unsafe fn vas_init() -> i32 { platform_driver_register(&mut vas_driver); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
