// SPDX-License-Identifier: GPL-2.0
// External Linux kernel headers and ../bus.h provide the declarations used here.

use core::ffi::c_void;

#[repr(C)]
pub struct device;
#[repr(C)]
pub struct work_struct;
#[repr(C)]
pub struct resource {
    pub name: *const core::ffi::c_char,
    pub start: usize,
    pub end: usize,
    pub flags: u64,
    pub desc: u32,
    pub child: *mut resource,
    pub sibling: *mut resource,
}

#[repr(C)]
pub struct platform_device {
    pub name: *const core::ffi::c_char,
    pub id: i32,
}

#[repr(C)]
pub struct hmem_platform_device {
    pub pdev: platform_device,
    pub work: work_struct,
}

pub type walk_hmem_fn = unsafe extern "C" fn(*mut device, i32, *mut resource) -> i32;

extern "C" {
    static mut hmem_resource_lock: c_void;
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn __request_region(parent: *mut resource, start: usize, n: usize,
                        name: *const core::ffi::c_char, flags: u64) -> *mut resource;
    fn resource_size(res: *mut resource) -> usize;
    fn platform_device_register(pdev: *mut platform_device) -> i32;
    fn phys_to_target_node(start: usize) -> i32;
    fn walk_soft_reserve_res(start: usize, end: usize, data: *mut c_void,
                             fn_: unsafe extern "C" fn(*mut resource, *mut c_void) -> i32);
    fn pr_debug(fmt: *const core::ffi::c_char, ...);
    fn pr_err_once(fmt: *const core::ffi::c_char, ...);
}

static mut nohmem: bool = false;
// module_param_named(disable, nohmem, bool, 0444);
static mut platform_initialized: bool = false;

const IORESOURCE_MEM: u64 = 0x0000_0200;

static mut hmem_active: resource = resource {
    name: b"HMEM devices\0".as_ptr() as *const core::ffi::c_char,
    start: 0,
    end: usize::MAX,
    flags: IORESOURCE_MEM,
    desc: 0,
    child: core::ptr::null_mut(),
    sibling: core::ptr::null_mut(),
};

static mut hmem_platform: hmem_platform_device = hmem_platform_device {
    pdev: platform_device {
        name: b"hmem_platform\0".as_ptr() as *const core::ffi::c_char,
        id: 0,
    },
    // __WORK_INITIALIZER(hmem_platform.work, hmem_work);
    work: work_struct {},
};

#[no_mangle]
pub unsafe extern "C" fn walk_hmem_resources(host: *mut device, fn_: walk_hmem_fn) -> i32 {
    let mut res: *mut resource;
    let mut rc: i32 = 0;

    mutex_lock(&mut hmem_resource_lock as *mut c_void);
    res = hmem_active.child;
    while !res.is_null() {
        rc = fn_(host, (*res).desc as i32, res);
        if rc != 0 {
            break;
        }
        res = (*res).sibling;
    }
    mutex_unlock(&mut hmem_resource_lock as *mut c_void);
    rc
}

// EXPORT_SYMBOL_GPL(walk_hmem_resources);

unsafe extern "C" fn hmem_work(_work: *mut work_struct) {
    /* place holder until dax_hmem driver attaches */
}

// __WORK_INITIALIZER(hmem_platform.work, hmem_work);

unsafe fn __hmem_register_resource(target_nid: i32, res: *mut resource) {
    let new = __request_region(
        &mut hmem_active,
        (*res).start,
        resource_size(res),
        b"\0".as_ptr() as *const core::ffi::c_char,
        0,
    );
    if new.is_null() {
        pr_debug(b"hmem range %pr already active\n\0".as_ptr() as *const core::ffi::c_char);
        return;
    }

    (*new).desc = target_nid as u32;

    if platform_initialized {
        return;
    }

    let rc = platform_device_register(&mut hmem_platform.pdev);
    if rc != 0 {
        pr_err_once(b"failed to register device-dax hmem_platform device\n\0".as_ptr()
            as *const core::ffi::c_char);
        return;
    }

    platform_initialized = true;
}

#[no_mangle]
pub unsafe extern "C" fn hmem_register_resource(target_nid: i32, res: *mut resource) {
    if nohmem {
        return;
    }

    mutex_lock(&mut hmem_resource_lock as *mut c_void);
    __hmem_register_resource(target_nid, res);
    mutex_unlock(&mut hmem_resource_lock as *mut c_void);
}

unsafe extern "C" fn hmem_register_one(res: *mut resource, _data: *mut c_void) -> i32 {
    hmem_register_resource(phys_to_target_node((*res).start), res);
    0
}

unsafe extern "C" fn hmem_init() -> i32 {
    walk_soft_reserve_res(0, usize::MAX, core::ptr::null_mut(), hmem_register_one);
    0
}

/*
 * As this is a fallback for address ranges unclaimed by the ACPI HMAT
 * parsing it must be at an initcall level greater than hmat_init().
 */
// device_initcall(hmem_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
