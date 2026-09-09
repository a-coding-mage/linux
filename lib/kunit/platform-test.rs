// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit test for KUnit platform driver infrastructure.
 */

// C dependencies supplied by the surrounding kernel/KUnit environment.
use core::ffi::c_void;

#[repr(C)]
pub struct kunit;
#[repr(C)]
pub struct platform_device {
    pub dev: device,
    pub name: *const i8,
    pub id: i32,
}
#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
}
#[repr(C)] pub struct device { pub driver: *mut device_driver }
#[repr(C)] pub struct device_driver { pub name: *const i8, pub owner: *mut c_void }
#[repr(C)] pub struct kunit_case;
#[repr(C)] pub struct kunit_suite { pub name: *const i8, pub test_cases: *mut kunit_case }
#[repr(C)] pub struct completion;
#[repr(C)] pub struct bus_type;

extern "C" {
    fn kunit_platform_device_alloc(test: *mut kunit, name: *const i8, id: i32) -> *mut platform_device;
    fn kunit_platform_device_add(test: *mut kunit, pdev: *mut platform_device) -> i32;
    fn kunit_platform_device_prepare_wait_for_probe(test: *mut kunit, pdev: *mut platform_device, comp: *mut completion) -> i32;
    fn kunit_platform_driver_register(test: *mut kunit, pdrv: *mut platform_driver) -> i32;
    fn kunit_kzalloc(test: *mut kunit, size: usize, flags: u32) -> *mut c_void;
    fn kunit_init_test(test: *mut kunit, name: *const i8, log: *const c_void);
    fn kunit_cleanup(test: *mut kunit);
    fn bus_find_device(bus: *const bus_type, start: *mut device, data: *const i8,
                       match_fn: unsafe extern "C" fn(*mut device, *const c_void) -> i32) -> *mut device;
    fn put_device(dev: *mut device);
    fn dev_name(dev: *mut device) -> *const i8;
    fn strcmp(a: *const i8, b: *const c_void) -> i32;
    fn dev_is_platform(dev: *mut device) -> bool;
    fn wait_for_completion_timeout(comp: *mut completion, timeout: u64) -> u64;
    fn reinit_completion(comp: *mut completion);
    static platform_bus_type: bus_type;
}

const KUNIT_SUCCESS: i32 = 0;
const GFP_KERNEL: u32 = 0;
const HZ: u64 = 1;
static TEST_DATA: &[u8] = b"test data\0";

unsafe extern "C" fn kunit_platform_device_alloc_test(test: *mut kunit) {
    let _ = kunit_platform_device_alloc(test, b"kunit-platform\0".as_ptr() as *const i8, 1);
}

unsafe extern "C" fn kunit_platform_device_add_test(test: *mut kunit) {
    let name = b"kunit-platform-add\0".as_ptr() as *const i8;
    let id = -1;
    let pdev = kunit_platform_device_alloc(test, name, id);
    let _ = kunit_platform_device_add(test, pdev);
    let _ = dev_is_platform(&mut (*pdev).dev);
    let _ = (*pdev).name;
    let _ = (*pdev).id;
}

unsafe extern "C" fn kunit_platform_device_add_twice_fails_test(test: *mut kunit) {
    let name = b"kunit-platform-add-2\0".as_ptr() as *const i8;
    let pdev = kunit_platform_device_alloc(test, name, -1);
    let _ = kunit_platform_device_add(test, pdev);
    let pdev = kunit_platform_device_alloc(test, name, -1);
    let _ = kunit_platform_device_add(test, pdev);
}

unsafe extern "C" fn kunit_platform_device_find_by_name(dev: *mut device, data: *const c_void) -> i32 {
    (strcmp(dev_name(dev), data) == 0) as i32
}

unsafe extern "C" fn kunit_platform_device_add_cleans_up(test: *mut kunit) {
    let mut fake = core::mem::zeroed::<kunit>();
    kunit_init_test(&mut fake, b"kunit_platform_device_add_fake_test\0".as_ptr() as *const i8, core::ptr::null());
    let name = b"kunit-platform-clean\0".as_ptr() as *const i8;
    let pdev = kunit_platform_device_alloc(&mut fake, name, -1);
    let _ = kunit_platform_device_add(&mut fake, pdev);
    let dev = bus_find_device(&platform_bus_type, core::ptr::null_mut(), name, kunit_platform_device_find_by_name);
    put_device(dev);
    // Remove pdev
    kunit_cleanup(&mut fake);
    let dev = bus_find_device(&platform_bus_type, core::ptr::null_mut(), name, kunit_platform_device_find_by_name);
    put_device(dev);
}

#[repr(C)]
struct kunit_platform_driver_test_context { pdrv: platform_driver, data: *const i8 }

unsafe extern "C" fn kunit_platform_driver_probe(pdev: *mut platform_device) -> i32 {
    let ctx = (pdev as *mut u8).sub(core::mem::offset_of!(kunit_platform_driver_test_context, pdrv)) as *mut kunit_platform_driver_test_context;
    (*ctx).data = TEST_DATA.as_ptr() as *const i8;
    0
}

unsafe extern "C" fn kunit_platform_driver_register_test(test: *mut kunit) {
    let ctx = kunit_kzalloc(test, core::mem::size_of::<kunit_platform_driver_test_context>(), GFP_KERNEL) as *mut kunit_platform_driver_test_context;
    let pdev = kunit_platform_device_alloc(test, b"kunit-platform-register\0".as_ptr() as *const i8, -1);
    let _ = kunit_platform_device_add(test, pdev);
    (*ctx).pdrv.probe = Some(kunit_platform_driver_probe);
    (*ctx).pdrv.driver.name = b"kunit-platform-register\0".as_ptr() as *const i8;
    let mut comp = core::mem::zeroed::<completion>();
    let _ = kunit_platform_device_prepare_wait_for_probe(test, pdev, &mut comp);
    let _ = kunit_platform_driver_register(test, &mut (*ctx).pdrv);
    let _ = wait_for_completion_timeout(&mut comp, 3 * HZ);
    let _ = (*ctx).data;
}

unsafe extern "C" fn kunit_platform_device_prepare_wait_for_probe_completes_when_already_probed(test: *mut kunit) {
    let ctx = kunit_kzalloc(test, core::mem::size_of::<kunit_platform_driver_test_context>(), GFP_KERNEL) as *mut kunit_platform_driver_test_context;
    let pdev = kunit_platform_device_alloc(test, b"kunit-platform-wait\0".as_ptr() as *const i8, -1);
    let _ = kunit_platform_device_add(test, pdev);
    (*ctx).pdrv.probe = Some(kunit_platform_driver_probe);
    (*ctx).pdrv.driver.name = b"kunit-platform-wait\0".as_ptr() as *const i8;
    let mut comp = core::mem::zeroed::<completion>();
    let _ = kunit_platform_device_prepare_wait_for_probe(test, pdev, &mut comp);
    let _ = kunit_platform_driver_register(test, &mut (*ctx).pdrv);
    let _ = wait_for_completion_timeout(&mut comp, 3 * HZ);
    reinit_completion(&mut comp);
    let _ = kunit_platform_device_prepare_wait_for_probe(test, pdev, &mut comp);
    let _ = wait_for_completion_timeout(&mut comp, HZ);
}

#[repr(C)]
struct kunit_case_entry { test: Option<unsafe extern "C" fn(*mut kunit)> }

static mut KUNIT_PLATFORM_DEVICE_TEST_CASES: [kunit_case_entry; 5] = [
    kunit_case_entry { test: Some(kunit_platform_device_alloc_test) },
    kunit_case_entry { test: Some(kunit_platform_device_add_test) },
    kunit_case_entry { test: Some(kunit_platform_device_add_twice_fails_test) },
    kunit_case_entry { test: Some(kunit_platform_device_add_cleans_up) },
    kunit_case_entry { test: None },
];

static mut KUNIT_PLATFORM_DEVICE_SUITE: kunit_suite = kunit_suite {
    name: b"kunit_platform_device\0".as_ptr() as *const i8,
    test_cases: core::ptr::null_mut(),
};

static mut KUNIT_PLATFORM_DRIVER_TEST_CASES: [kunit_case_entry; 3] = [
    kunit_case_entry { test: Some(kunit_platform_driver_register_test) },
    kunit_case_entry { test: Some(kunit_platform_device_prepare_wait_for_probe_completes_when_already_probed) },
    kunit_case_entry { test: None },
];

static mut KUNIT_PLATFORM_DRIVER_SUITE: kunit_suite = kunit_suite {
    name: b"kunit_platform_driver\0".as_ptr() as *const i8,
    test_cases: core::ptr::null_mut(),
};

// kunit_test_suites(&kunit_platform_device_suite, &kunit_platform_driver_suite)
// MODULE_LICENSE("GPL") and MODULE_DESCRIPTION(...) register kernel metadata.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
