// SPDX-License-Identifier: GPL-2.0
/*
 * Test managed platform driver
 */

use core::ffi::c_void;

// Dependencies supplied by the surrounding kernel/KUnit translation.
#[repr(C)] pub struct kunit { _private: [u8; 0] }
#[repr(C)] pub struct kunit_resource { pub data: *mut c_void, pub free: Option<unsafe extern "C" fn(*mut kunit_resource)> }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct platform_device_info { _private: [u8; 0] }
#[repr(C)] pub struct platform_driver { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int> }
#[repr(C)] pub struct bus_type { _private: [u8; 0] }

type c_int = i32;
type c_ulong = usize;

const ENOMEM: c_int = 12;
const GFP_KERNEL: u32 = 0;
const BUS_NOTIFY_BOUND_DRIVER: c_ulong = 0;
const NOTIFY_DONE: c_int = 0;
const NOTIFY_OK: c_int = 1;

unsafe extern "C" {
    fn platform_device_alloc(name: *const i8, id: c_int) -> *mut platform_device;
    fn platform_device_put(pdev: *mut platform_device);
    fn platform_device_add(pdev: *mut platform_device) -> c_int;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn platform_device_register_full(info: *const platform_device_info) -> *mut platform_device;
    fn platform_driver_register(drv: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(drv: *mut platform_driver);
    fn kunit_alloc_resource(test: *mut kunit, init: unsafe extern "C" fn(*mut kunit_resource, *mut c_void) -> c_int, exit: unsafe extern "C" fn(*mut kunit_resource), flags: u32, context: *mut c_void) -> *mut platform_device;
    fn kunit_find_resource(test: *mut kunit, m: unsafe extern "C" fn(*mut kunit, *mut kunit_resource, *mut c_void) -> bool, data: *mut c_void) -> *mut kunit_resource;
    fn kunit_put_resource(res: *mut kunit_resource);
    fn kunit_add_action_or_reset(test: *mut kunit, action: unsafe extern "C" fn(*mut c_void), data: *mut c_void) -> c_int;
    fn kunit_remove_action(test: *mut kunit, action: unsafe extern "C" fn(*mut c_void), data: *mut c_void);
    fn kunit_kzalloc(test: *mut kunit, size: usize, flags: u32) -> *mut c_void;
    fn kunit_kfree(test: *mut kunit, ptr: *mut c_void);
    fn bus_register_notifier(bus: *mut bus_type, nb: *mut notifier_block) -> c_int;
    fn bus_unregister_notifier(bus: *mut bus_type, nb: *mut notifier_block);
    fn device_lock(dev: *mut device);
    fn device_unlock(dev: *mut device);
    fn device_is_bound(dev: *mut device) -> bool;
    fn complete(x: *mut completion);
    static mut platform_bus_type: bus_type;
}

#[repr(C)]
struct kunit_platform_device_alloc_params { name: *const i8, id: c_int }

unsafe extern "C" fn kunit_platform_device_alloc_init(res: *mut kunit_resource, context: *mut c_void) -> c_int {
    let params = &*(context as *const kunit_platform_device_alloc_params);
    let pdev = platform_device_alloc(params.name, params.id);
    if pdev.is_null() { return -ENOMEM; }
    (*res).data = pdev.cast();
    0
}

unsafe extern "C" fn kunit_platform_device_alloc_exit(res: *mut kunit_resource) {
    platform_device_put((*res).data.cast());
}

pub unsafe extern "C" fn kunit_platform_device_alloc(test: *mut kunit, name: *const i8, id: c_int) -> *mut platform_device {
    let mut params = kunit_platform_device_alloc_params { name, id };
    kunit_alloc_resource(test, kunit_platform_device_alloc_init, kunit_platform_device_alloc_exit, GFP_KERNEL, (&mut params as *mut _).cast())
}

unsafe extern "C" fn kunit_platform_device_add_exit(res: *mut kunit_resource) { platform_device_unregister((*res).data.cast()); }

unsafe extern "C" fn kunit_platform_device_alloc_match(_test: *mut kunit, res: *mut kunit_resource, match_data: *mut c_void) -> bool {
    (*res).data == match_data && (*res).free == Some(kunit_platform_device_alloc_exit)
}

unsafe extern "C" fn platform_device_unregister_wrapper(data: *mut c_void) { platform_device_unregister(data.cast()); }

pub unsafe extern "C" fn kunit_platform_device_add(test: *mut kunit, pdev: *mut platform_device) -> c_int {
    let mut ret = platform_device_add(pdev);
    if ret != 0 { return ret; }
    let res = kunit_find_resource(test, kunit_platform_device_alloc_match, pdev.cast());
    if !res.is_null() {
        (*res).free = Some(kunit_platform_device_add_exit);
        kunit_put_resource(res);
    } else {
        ret = kunit_add_action_or_reset(test, platform_device_unregister_wrapper, pdev.cast());
        if ret != 0 { return ret; }
    }
    0
}

pub unsafe extern "C" fn kunit_platform_device_register_full(test: *mut kunit, pdevinfo: *const platform_device_info) -> *mut platform_device {
    let pdev = platform_device_register_full(pdevinfo);
    if pdev.is_null() { return pdev; }
    let ret = kunit_add_action_or_reset(test, platform_device_unregister_wrapper, pdev.cast());
    if ret != 0 { return (-(ret as isize)) as *mut platform_device; }
    pdev
}

unsafe extern "C" fn kunit_platform_device_add_match(_test: *mut kunit, res: *mut kunit_resource, match_data: *mut c_void) -> bool {
    (*res).data == match_data && (*res).free == Some(kunit_platform_device_add_exit)
}

pub unsafe extern "C" fn kunit_platform_device_unregister(test: *mut kunit, pdev: *mut platform_device) {
    let res = kunit_find_resource(test, kunit_platform_device_add_match, pdev.cast());
    if !res.is_null() { (*res).free = None; kunit_put_resource(res); }
    else { kunit_remove_action(test, platform_device_unregister_wrapper, pdev.cast()); }
    platform_device_unregister(pdev);
}

#[repr(C)] struct kunit_platform_device_probe_nb { x: *mut completion, dev: *mut device, nb: notifier_block }

unsafe extern "C" fn kunit_platform_device_probe_notify(nb: *mut notifier_block, event: c_ulong, data: *mut c_void) -> c_int {
    let knb = (nb as *mut u8).sub(core::mem::offset_of!(kunit_platform_device_probe_nb, nb)) as *mut kunit_platform_device_probe_nb;
    let dev = data as *mut device;
    if event != BUS_NOTIFY_BOUND_DRIVER || (*knb).dev != dev { return NOTIFY_DONE; }
    complete((*knb).x); NOTIFY_OK
}

unsafe extern "C" fn kunit_platform_device_probe_nb_remove(nb: *mut c_void) { bus_unregister_notifier(&mut platform_bus_type, nb.cast()); }

pub unsafe extern "C" fn kunit_platform_device_prepare_wait_for_probe(test: *mut kunit, pdev: *mut platform_device, x: *mut completion) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let knb = kunit_kzalloc(test, core::mem::size_of::<kunit_platform_device_probe_nb>(), GFP_KERNEL) as *mut kunit_platform_device_probe_nb;
    if knb.is_null() { return -ENOMEM; }
    (*knb).nb.notifier_call = Some(kunit_platform_device_probe_notify); (*knb).dev = dev; (*knb).x = x;
    device_lock(dev); let bound = device_is_bound(dev);
    if bound { device_unlock(dev); complete(x); kunit_kfree(test, knb.cast()); return 0; }
    bus_register_notifier(&mut platform_bus_type, &mut (*knb).nb); device_unlock(dev);
    kunit_add_action_or_reset(test, kunit_platform_device_probe_nb_remove, (&mut (*knb).nb).cast())
}

unsafe extern "C" fn platform_driver_unregister_wrapper(data: *mut c_void) { platform_driver_unregister(data.cast()); }

pub unsafe extern "C" fn kunit_platform_driver_register(test: *mut kunit, drv: *mut platform_driver) -> c_int {
    let ret = platform_driver_register(drv); if ret != 0 { return ret; }
    kunit_add_action_or_reset(test, platform_driver_unregister_wrapper, drv.cast())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
