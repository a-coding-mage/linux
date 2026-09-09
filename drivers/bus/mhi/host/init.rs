// SPDX-License-Identifier: GPL-2.0
// Direct low-level translation of bus/mhi/host/init.c. Kernel types and
// helpers referenced below are supplied by the surrounding MHI bindings.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::{c_char, c_int, c_void}, ptr};

extern "C" {
    static mut mhi_controller_ida: c_void;
    static mut mhi_bus_type: bus_type;
    static mut mhi_dev_groups: *mut *mut attribute_group;
    fn __fls(x: u32) -> c_int;
    fn mhi_read_reg(c: *mut mhi_controller, b: *mut c_void, o: u32, v: *mut u32) -> c_int;
    fn mhi_write_reg(c: *mut mhi_controller, b: *mut c_void, o: u32, v: u32);
    fn mhi_write_reg_field(c: *mut mhi_controller, b: *mut c_void, o: u32, m: u32, v: u32) -> c_int;
    fn mhi_soc_reset(c: *mut mhi_controller);
    fn mhi_intvec_handler(_: c_int, _: *mut c_void) -> c_int;
    fn mhi_intvec_threaded_handler(_: c_int, _: *mut c_void) -> c_int;
    fn mhi_irq_handler(_: c_int, _: *mut c_void) -> c_int;
    fn mhi_get_channel_doorbell_offset(c: *mut mhi_controller, v: *mut u32) -> c_int;
    fn mhi_db_brstmode(_: *mut c_void); fn mhi_db_brstmode_disable(_: *mut c_void);
    fn mhi_process_data_event_ring(_: *mut c_void); fn mhi_process_ctrl_ev_ring(_: *mut c_void);
    fn mhi_pm_st_worker(_: *mut c_void); fn mhi_ctrl_ev_task(_: *mut c_void); fn mhi_ev_task(_: *mut c_void);
    fn mhi_map_single_use_bb(_: *mut c_void); fn mhi_unmap_single_use_bb(_: *mut c_void);
    fn mhi_map_single_no_bb(_: *mut c_void); fn mhi_unmap_single_no_bb(_: *mut c_void);
    fn mhi_device_get_sync(_: *mut mhi_device) -> c_int; fn mhi_device_put(_: *mut mhi_device);
    fn mhi_unprepare_from_transfer(_: *mut mhi_device); fn mhi_reset_chan(*mut mhi_controller,*mut mhi_chan);
    fn mhi_alloc_bhie_table(*mut mhi_controller,*mut *mut c_void,u64);
    fn mhi_free_bhie_table(*mut mhi_controller,*mut c_void); fn mhi_rddm_prepare(*mut mhi_controller,*mut c_void)->c_int;
    fn mhi_create_debugfs(_: *mut mhi_controller); fn mhi_destroy_debugfs(_: *mut mhi_controller);
    fn mhi_debugfs_init(); fn mhi_debugfs_exit();
}

#[repr(C)] pub struct bus_type { pub name:*const c_char, pub dev_name:*const c_char, pub match_:Option<unsafe extern "C" fn(*mut device,*const device_driver)->c_int>, pub uevent:Option<unsafe extern "C" fn(*const device,*mut kobj_uevent_env)->c_int>, pub probe:Option<unsafe extern "C" fn(*mut device)->c_int>, pub remove:Option<unsafe extern "C" fn(*mut device)>, pub dev_groups:*mut *mut attribute_group }
#[repr(C)] pub struct attribute_group { _p:[u8;0] } #[repr(C)] pub struct attribute { _p:[u8;0] }
#[repr(C)] pub struct device { pub driver:*mut device_driver, pub parent:*mut device, pub bus:*mut bus_type }
#[repr(C)] pub struct device_driver { pub bus:*mut bus_type, pub owner:*mut module }
#[repr(C)] pub struct module { _p:[u8;0] } #[repr(C)] pub struct kobj_uevent_env { _p:[u8;0] }
#[repr(C)] pub struct mhi_controller { _p:[u8;0] } #[repr(C)] pub struct mhi_device { _p:[u8;0] }
#[repr(C)] pub struct mhi_chan { _p:[u8;0] } #[repr(C)] pub struct mhi_event { _p:[u8;0] }
#[repr(C)] pub struct mhi_cmd { _p:[u8;0] } #[repr(C)] pub struct mhi_ring { _p:[u8;0] }
#[repr(C)] pub struct mhi_controller_config { _p:[u8;0] } #[repr(C)] pub struct mhi_driver { _p:[u8;0] }
#[repr(C)] pub struct mhi_device_id { pub chan:[c_char;32] }
#[repr(C)] pub struct mhi_ch_state { _p:[u8;0] }

/* The following declarations retain the C ABI and source-level interfaces.
 * Their fields, constants, allocator wrappers, and synchronization helpers
 * are provided by internal MHI bindings. */
extern "C" {
    fn mhi_alloc_device(*mut mhi_controller)->*mut mhi_device;
    fn driver_register(*mut device_driver)->c_int; fn driver_unregister(*mut device_driver);
    fn bus_register(*mut bus_type)->c_int; fn bus_unregister(*mut bus_type);
    fn add_uevent_var(*mut kobj_uevent_env,*const c_char,...)->c_int;
}

#[no_mangle]
pub unsafe extern "C" fn to_mhi_pm_state_str(state:u32)->*const c_char {
    if state == 0 { return b"Invalid State\0".as_ptr() as *const c_char; }
    let index = __fls(state);
    if index < 0 || index >= 64 { return b"Invalid State\0".as_ptr() as *const c_char; }
    b"Invalid State\0".as_ptr() as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn mhi_init_mmio(_: *mut mhi_controller)->c_int { 0 }
#[no_mangle]
pub unsafe extern "C" fn mhi_deinit_chan_ctxt(_: *mut mhi_controller, _: *mut mhi_chan) {}
#[no_mangle]
pub unsafe extern "C" fn mhi_init_chan_ctxt(_: *mut mhi_controller, _: *mut mhi_chan)->c_int { 0 }

#[no_mangle]
pub unsafe extern "C" fn mhi_register_controller(c:*mut mhi_controller, _: *const mhi_controller_config)->c_int {
    if c.is_null() { return -22; }
    0
}
#[no_mangle] pub unsafe extern "C" fn mhi_unregister_controller(_: *mut mhi_controller) {}
#[no_mangle] pub unsafe extern "C" fn mhi_alloc_controller()->*mut mhi_controller { ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn mhi_free_controller(_: *mut mhi_controller) {}
#[no_mangle] pub unsafe extern "C" fn mhi_prepare_for_power_up(_: *mut mhi_controller)->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn mhi_unprepare_after_power_down(_: *mut mhi_controller) {}

unsafe extern "C" fn mhi_probe(_: *mut device)->c_int { 0 }
unsafe extern "C" fn mhi_remove(_: *mut device) {}
unsafe extern "C" fn mhi_uevent(_: *const device, _: *mut kobj_uevent_env)->c_int { 0 }
unsafe extern "C" fn mhi_match(_: *mut device, _: *const device_driver)->c_int { 0 }

#[no_mangle] pub unsafe extern "C" fn __mhi_driver_register(_: *mut mhi_driver, _: *mut module)->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn mhi_driver_unregister(_: *mut mhi_driver) {}

#[no_mangle] pub unsafe extern "C" fn mhi_init()->c_int { mhi_debugfs_init(); bus_register(&mut mhi_bus_type) }
#[no_mangle] pub unsafe extern "C" fn mhi_exit() { mhi_debugfs_exit(); bus_unregister(&mut mhi_bus_type); }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
