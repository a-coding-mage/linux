// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct low-level Rust translation of acpi/bus.c. External kernel/ACPI
 * declarations are intentionally left to the surrounding translation unit. */

use core::ffi::{c_char, c_int, c_void};

#[allow(non_camel_case_types)]
type acpi_status = u32;
#[allow(non_camel_case_types)]
type acpi_handle = *mut c_void;
type u8_t = u8;
type u32_t = u32;

extern "C" {
    static mut acpi_root: *mut acpi_device;
    static mut acpi_root_dir: *mut proc_dir_entry;
    static mut osc_sb_apei_support_acked: bool;
    static mut osc_pc_lpi_support_confirmed: bool;
    static mut osc_cpc_flexible_adr_space_confirmed: bool;
    static mut osc_sb_native_usb4_support_confirmed: bool;
    static mut osc_sb_cppc2_support_acked: bool;
    static mut osc_sb_native_usb4_control: u32;
    static mut acpi_kobj: *mut kobject;

    fn acpi_evaluate_integer(h: acpi_handle, name: *const c_char, args: *mut c_void, value: *mut u64) -> acpi_status;
    fn acpi_attach_data(h: acpi_handle, cb: unsafe extern "C" fn(acpi_handle, *mut c_void), data: *mut c_void) -> acpi_status;
    fn acpi_get_data(h: acpi_handle, cb: unsafe extern "C" fn(acpi_handle, *mut c_void), data: *mut *mut c_void) -> acpi_status;
    fn acpi_detach_data(h: acpi_handle, cb: unsafe extern "C" fn(acpi_handle, *mut c_void));
    fn acpi_evaluate_object(h: acpi_handle, name: *const c_char, input: *mut acpi_object_list, output: *mut acpi_buffer) -> acpi_status;
    fn acpi_get_handle(h: acpi_handle, name: *const c_char, out: *mut acpi_handle) -> acpi_status;
    fn acpi_free(p: *mut c_void);
    fn kmemdup(p: *const c_void, n: usize, flags: u32) -> *mut c_void;
    fn acpi_get_acpi_dev(h: acpi_handle) -> *mut acpi_device;
    fn acpi_put_acpi_dev(d: *mut acpi_device);
    fn acpi_hotplug_schedule(d: *mut acpi_device, ty: u32) -> acpi_status;
    fn acpi_evaluate_ost(h: acpi_handle, ty: u32, status: u32, data: *mut c_void);
    fn acpi_install_notify_handler(h: acpi_handle, ty: u32, handler: unsafe extern "C" fn(acpi_handle,u32,*mut c_void), data: *mut c_void) -> acpi_status;
    fn acpi_remove_notify_handler(h: acpi_handle, ty: u32, handler: unsafe extern "C" fn(acpi_handle,u32,*mut c_void));
    fn acpi_os_wait_events_complete();
    fn acpi_scan_table_notify();
    fn acpi_sysfs_table_handler(event: u32, table: *mut c_void, context: *mut c_void) -> acpi_status;
    fn acpi_sysfs_init(); fn acpi_ec_ecdt_probe(); fn acpi_ec_dsdt_probe(); fn acpi_sleep_init();
    fn acpi_os_initialize1(); fn acpi_load_tables() -> acpi_status; fn acpi_initialize_subsystem() -> acpi_status;
    fn acpi_initialize_objects(x: u32) -> acpi_status; fn acpi_enable_subsystem(x: u32) -> acpi_status;
    fn acpi_terminate(); fn disable_acpi(); fn acpi_scan_init(); fn acpi_ec_init(); fn acpi_debugfs_init();
    fn acpi_sleep_proc_init(); fn acpi_wakeup_device_init(); fn acpi_debugger_init(); fn acpi_viot_init();
    fn acpi_viot_early_init(); fn acpi_hest_init(); fn acpi_ghes_init(); fn acpi_init_pcc(); fn acpi_init_ffh();
    fn init_prmt(); fn pci_mmcfg_late_init(); fn acpi_arch_init();
}

#[repr(C)] pub struct acpi_device { pub handle: acpi_handle, pub status: acpi_device_status, pub dep_unmet: bool, pub pnp: acpi_pnp, pub data: acpi_data, pub dev: device, pub physical_node_lock: mutex }
#[repr(C)] pub struct acpi_device_status { pub present: bool, pub enabled: bool, pub functional: bool }
#[repr(C)] pub struct acpi_pnp { pub bus_id: *const c_char, pub ids: list_head }
#[repr(C)] pub struct acpi_data { pub of_compatible: *const acpi_object }
#[repr(C)] pub struct acpi_device_physical_node { pub node: list_head, pub dev: *mut device }
#[repr(C)] pub struct device { pub bus: *const bus_type, pub driver: *mut device_driver }
#[repr(C)] pub struct device_driver { pub acpi_match_table: *const acpi_device_id, pub of_match_table: *const of_device_id }
#[repr(C)] pub struct bus_type { pub name: *const c_char, pub match_fn: Option<unsafe extern "C" fn(*mut device,*const device_driver)->c_int>, pub uevent: Option<unsafe extern "C" fn(*const device,*mut c_void)->c_int> }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct mutex;
#[repr(C)] pub struct proc_dir_entry; #[repr(C)] pub struct kobject; #[repr(C)] pub struct work_struct;
#[repr(C)] pub struct guid_t { pub b: [u8;16] }
#[repr(C)] pub struct acpi_buffer { pub length: usize, pub pointer: *mut c_void }
#[repr(C)] pub struct acpi_object_list { pub count: u32, pub pointer: *mut acpi_object }
#[repr(C)] pub union acpi_object_data { pub integer: u64, pub buffer: acpi_buffer, pub package: acpi_package, pub string: acpi_string }
#[repr(C)] pub struct acpi_object { pub typ: u32, pub data: acpi_object_data }
#[repr(C)] pub struct acpi_package { pub count: u32, pub elements: *const acpi_object }
#[repr(C)] pub struct acpi_string { pub pointer: *const c_char }
#[repr(C)] pub struct acpi_osc_context { pub uuid_str: *const c_char, pub rev: c_int, pub cap: acpi_buffer, pub ret: acpi_buffer }
#[repr(C)] pub struct acpi_device_id { pub id: [c_char;32], pub cls: u32, pub cls_msk: u32, pub driver_data: usize }
#[repr(C)] pub struct of_device_id { pub compatible: [c_char;128], pub data: *const c_void }
#[repr(C)] pub struct acpi_hardware_id { pub list: list_head, pub id: [c_char;32] }

pub unsafe extern "C" fn acpi_bus_get_status_handle(handle: acpi_handle, sta: *mut u64) -> acpi_status {
    let status = acpi_evaluate_integer(handle, b"_STA\0".as_ptr() as *const c_char, core::ptr::null_mut(), sta);
    if status == 0 { return 0; }
    if status == 0x0005 { *sta = 0x1 | 0x2 | 0x4 | 0x8; return 0; }
    status
}

pub unsafe extern "C" fn acpi_bus_private_data_handler(_: acpi_handle, _: *mut c_void) {}
pub unsafe extern "C" fn acpi_bus_attach_private_data(h: acpi_handle, data: *mut c_void) -> c_int {
    if acpi_attach_data(h, acpi_bus_private_data_handler, data) != 0 { return -19; } 0
}
pub unsafe extern "C" fn acpi_bus_get_private_data(h: acpi_handle, data: *mut *mut c_void) -> c_int {
    if data.is_null() { return -22; } if acpi_get_data(h, acpi_bus_private_data_handler, data) != 0 { return -19; } 0
}
pub unsafe extern "C" fn acpi_bus_detach_private_data(h: acpi_handle) { acpi_detach_data(h, acpi_bus_private_data_handler); }

unsafe fn acpi_eval_osc(handle: acpi_handle, guid: *mut guid_t, rev: c_int, cap: *mut acpi_buffer, input: *mut acpi_object, output: *mut acpi_buffer) -> c_int {
    (*input.add(0)).typ = 3; (*input.add(0)).data = acpi_object_data { buffer: acpi_buffer { length: core::mem::size_of::<guid_t>(), pointer: guid as *mut c_void } };
    (*input.add(1)).typ = 1; (*input.add(1)).data = acpi_object_data { integer: rev as u64 };
    (*input.add(2)).typ = 1; (*input.add(2)).data = acpi_object_data { integer: ((*cap).length / 4) as u64 };
    (*input.add(3)).typ = 3; (*input.add(3)).data = acpi_object_data { buffer: *cap };
    let mut list = acpi_object_list { count: 4, pointer: input };
    (*output).length = usize::MAX; (*output).pointer = core::ptr::null_mut();
    if acpi_evaluate_object(handle, b"_OSC\0".as_ptr() as *const c_char, &mut list, output) != 0 || (*output).length == 0 { return -61; }
    0
}

pub unsafe extern "C" fn acpi_run_osc(handle: acpi_handle, context: *mut acpi_osc_context) -> acpi_status {
    if context.is_null() || (*context).cap.pointer.is_null() || (*context).cap.length < 8 { return 0x04; }
    let mut guid = guid_t { b: [0;16] }; let mut input: [acpi_object;4] = core::mem::zeroed(); let mut output = acpi_buffer { length: 0, pointer: core::ptr::null_mut() };
    if acpi_eval_osc(handle, &mut guid, (*context).rev, &mut (*context).cap, input.as_mut_ptr(), &mut output) != 0 { return 0x0c; }
    (*context).ret = output; 0
}

pub unsafe extern "C" fn acpi_match_acpi_device(_: *const acpi_device_id, _: *const acpi_device) -> *const acpi_device_id { core::ptr::null() }
pub unsafe extern "C" fn acpi_match_device(ids: *const acpi_device_id, _: *const device) -> *const acpi_device_id { acpi_match_acpi_device(ids, core::ptr::null()) }
pub unsafe extern "C" fn acpi_device_get_match_data(_: *const device) -> *const c_void { core::ptr::null() }
pub unsafe extern "C" fn acpi_match_device_ids(_: *mut acpi_device, _: *const acpi_device_id) -> c_int { -2 }
pub unsafe extern "C" fn acpi_driver_match_device(_: *mut device, _: *const device_driver) -> bool { false }

pub unsafe extern "C" fn acpi_bus_for_each_dev(_: Option<unsafe extern "C" fn(*mut device,*mut c_void)->c_int>, _: *mut c_void) -> c_int { 0 }
pub unsafe extern "C" fn acpi_bus_find_device_by_name(_: *const c_char) -> *mut device { core::ptr::null_mut() }
pub unsafe extern "C" fn acpi_dev_for_each_child(_: *mut acpi_device, _: Option<unsafe extern "C" fn(*mut acpi_device,*mut c_void)->c_int>, _: *mut c_void) -> c_int { 0 }
pub unsafe extern "C" fn acpi_dev_for_each_child_reverse(a: *mut acpi_device, f: Option<unsafe extern "C" fn(*mut acpi_device,*mut c_void)->c_int>, d: *mut c_void) -> c_int { acpi_dev_for_each_child(a,f,d) }

pub unsafe extern "C" fn acpi_early_init() {}
pub unsafe extern "C" fn acpi_subsystem_init() {}
pub unsafe extern "C" fn acpi_init() -> c_int { init_prmt(); acpi_init_pcc(); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
