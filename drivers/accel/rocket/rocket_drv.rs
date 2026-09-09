// SPDX-License-Identifier: GPL-2.0-only
/* Copyright 2024-2025 Tomeu Vizoso <tomeu@tomeuvizoso.net> */

// External Linux/DRM and Rocket declarations are supplied by the surrounding
// kernel bindings.

use core::ffi::c_void;

// Facade device, used to expose a single DRM device to userspace, that
// schedules jobs to any RKNN cores in the system.
static mut drm_dev: *mut platform_device = core::ptr::null_mut();
static mut rdev: *mut rocket_device = core::ptr::null_mut();

unsafe extern "C" {
    type platform_device;
    type rocket_device;
    type rocket_iommu_domain;
    type rocket_file_priv;
    type drm_device;
    type drm_file;
    type kref;
    type device;
    type drm_ioctl_desc;
    type drm_driver;
    type of_device_id;
    type clk;
    type rocket_core;
    type drm_accel_fops;
    type rocket_pm_ops_type;

    fn iommu_domain_free(domain: *mut c_void);
    fn iommu_paging_domain_alloc(dev: *mut device) -> *mut c_void;
    fn kref_init(kref: *mut kref);
    fn kref_get(kref: *mut kref);
    fn kref_put(kref: *mut kref, release: unsafe extern "C" fn(*mut kref));
    fn kfree(ptr: *mut c_void);
    fn kmalloc(size: usize) -> *mut c_void;
    fn kzalloc(size: usize) -> *mut c_void;
    fn try_module_get(module: *mut c_void) -> bool;
    fn module_put(module: *mut c_void);
    fn rocket_device_init(dev: *mut platform_device, driver: *const drm_driver) -> *mut rocket_device;
    fn rocket_device_fini(dev: *mut rocket_device);
    fn rocket_core_init(core: *mut rocket_core) -> i32;
    fn rocket_core_fini(core: *mut rocket_core);
    fn rocket_job_open(priv_: *mut rocket_file_priv) -> i32;
    fn rocket_job_close(priv_: *mut rocket_file_priv);
    fn rocket_job_is_idle(core: *mut rocket_core) -> bool;
    fn rocket_gem_create_object(dev: *mut drm_device, size: usize) -> *mut c_void;
    fn rocket_ioctl_create_bo(dev: *mut drm_device, file: *mut drm_file, data: *mut c_void) -> i32;
    fn rocket_ioctl_submit(dev: *mut drm_device, file: *mut drm_file, data: *mut c_void) -> i32;
    fn rocket_ioctl_prep_bo(dev: *mut drm_device, file: *mut drm_file, data: *mut c_void) -> i32;
    fn rocket_ioctl_fini_bo(dev: *mut drm_device, file: *mut drm_file, data: *mut c_void) -> i32;
    fn drm_mm_init(mm: *mut c_void, start: u64, size: u64);
    fn drm_mm_takedown(mm: *mut c_void);
    fn mutex_init(lock: *mut c_void);
    fn mutex_destroy(lock: *mut c_void);
    fn platform_device_register_simple(name: *const u8, id: i32, data: *mut c_void, size: u32) -> *mut platform_device;
    fn platform_device_unregister(dev: *mut platform_device);
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn platform_driver_unregister(driver: *mut platform_driver);
    fn dev_set_drvdata(dev: *mut device, data: *mut rocket_device);
    fn dev_get_drvdata(dev: *mut device) -> *mut rocket_device;
    fn clk_bulk_prepare_enable(count: usize, clks: *mut clk) -> i32;
    fn clk_bulk_disable_unprepare(count: usize, clks: *mut clk);
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
}

#[repr(C)]
struct drm_mm { _opaque: [u8; 0] }

#[repr(C)]
struct rocket_iommu_domain_layout {
    kref: kref,
    domain: *mut iommu_domain_layout,
}
#[repr(C)] struct iommu_domain_layout { geometry: iommu_geometry }
#[repr(C)] struct iommu_geometry { aperture_start: u64, aperture_end: u64 }

#[repr(C)] struct rocket_file_priv_layout {
    rdev: *mut rocket_device,
    domain: *mut rocket_iommu_domain_layout,
    mm: drm_mm,
    mm_lock: [u8; 0],
}
#[repr(C)] struct rocket_core_layout { rdev: *mut rocket_device, dev: *mut device, index: u32, clks: *mut clk }
#[repr(C)] struct rocket_device_layout { cores: *mut rocket_core_layout, num_cores: u32 }
#[repr(C)] struct drm_file_layout { driver_priv: *mut c_void }

unsafe extern "C" fn rocket_iommu_domain_destroy(kref_: *mut kref) {
    let domain = kref_ as *mut rocket_iommu_domain_layout;
    iommu_domain_free((*domain).domain as *mut c_void);
    (*domain).domain = core::ptr::null_mut();
    kfree(domain as *mut c_void);
}

unsafe fn rocket_iommu_domain_create(dev: *mut device) -> *mut rocket_iommu_domain_layout {
    let domain = kzalloc(core::mem::size_of::<rocket_iommu_domain_layout>()) as *mut rocket_iommu_domain_layout;
    if domain.is_null() { return (-12i32) as isize as *mut rocket_iommu_domain_layout; }
    (*domain).domain = iommu_paging_domain_alloc(dev) as *mut iommu_domain_layout;
    if (*domain).domain.is_null() { kfree(domain as *mut c_void); return (-12i32) as isize as *mut rocket_iommu_domain_layout; }
    kref_init(&mut (*domain).kref);
    domain
}

#[no_mangle]
pub unsafe extern "C" fn rocket_iommu_domain_get(priv_: *mut rocket_file_priv_layout) -> *mut rocket_iommu_domain_layout {
    kref_get(&mut (*(*priv_).domain).kref); (*priv_).domain
}
#[no_mangle]
pub unsafe extern "C" fn rocket_iommu_domain_put(domain: *mut rocket_iommu_domain_layout) { kref_put(&mut (*domain).kref, rocket_iommu_domain_destroy); }

unsafe extern "C" fn rocket_open(dev: *mut drm_device, file: *mut drm_file_layout) -> i32 {
    let device = dev as *mut rocket_device;
    if !try_module_get(core::ptr::null_mut()) { return -22; }
    let priv_ = kzalloc(core::mem::size_of::<rocket_file_priv_layout>()) as *mut rocket_file_priv_layout;
    if priv_.is_null() { module_put(core::ptr::null_mut()); return -12; }
    (*priv_).rdev = device;
    (*priv_).domain = rocket_iommu_domain_create((*(*device).cores).dev);
    if ((*priv_).domain as isize) < 0 { kfree(priv_ as *mut c_void); module_put(core::ptr::null_mut()); return (*priv_).domain as isize as i32; }
    (*file).driver_priv = priv_ as *mut c_void;
    let start = (*(*priv_).domain).domain.as_ref().unwrap().geometry.aperture_start;
    let end = (*(*priv_).domain).domain.as_ref().unwrap().geometry.aperture_end;
    drm_mm_init(&mut (*priv_).mm as *mut _ as *mut c_void, start, end - start + 1);
    mutex_init(&mut (*priv_).mm_lock as *mut _ as *mut c_void);
    let ret = rocket_job_open(priv_ as *mut rocket_file_priv);
    if ret != 0 { mutex_destroy(&mut (*priv_).mm_lock as *mut _ as *mut c_void); drm_mm_takedown(&mut (*priv_).mm as *mut _ as *mut c_void); rocket_iommu_domain_put((*priv_).domain); kfree(priv_ as *mut c_void); module_put(core::ptr::null_mut()); return ret; }
    0
}

unsafe extern "C" fn rocket_postclose(_dev: *mut drm_device, file: *mut drm_file_layout) {
    let priv_ = (*file).driver_priv as *mut rocket_file_priv_layout;
    rocket_job_close(priv_ as *mut rocket_file_priv); mutex_destroy(&mut (*priv_).mm_lock as *mut _ as *mut c_void); drm_mm_takedown(&mut (*priv_).mm as *mut _ as *mut c_void); rocket_iommu_domain_put((*priv_).domain); kfree(priv_ as *mut c_void); module_put(core::ptr::null_mut());
}

#[repr(C)] struct drm_driver { driver_features: u32, open: Option<unsafe extern "C" fn(*mut drm_device,*mut drm_file_layout)->i32>, postclose: Option<unsafe extern "C" fn(*mut drm_device,*mut drm_file_layout)>, gem_create_object: Option<unsafe extern "C" fn(*mut drm_device,usize)->*mut c_void>, ioctls: *const drm_ioctl_desc, num_ioctls: usize, fops: *const c_void, name: *const u8, desc: *const u8 }
#[repr(C)] struct drm_ioctl_desc { cmd: u32, handler: *const c_void, flags: u32 }
#[repr(C)] struct platform_driver { probe: Option<unsafe extern "C" fn(*mut platform_device)->i32>, remove: Option<unsafe extern "C" fn(*mut platform_device)>, driver: driver_inner }
#[repr(C)] struct driver_inner { name: *const u8, pm: *const c_void, of_match_table: *const of_device_id }

static rocket_drm_driver_ioctls: [drm_ioctl_desc; 4] = [drm_ioctl_desc{cmd:0,handler:rocket_ioctl_create_bo as *const c_void,flags:0},drm_ioctl_desc{cmd:1,handler:rocket_ioctl_submit as *const c_void,flags:0},drm_ioctl_desc{cmd:2,handler:rocket_ioctl_prep_bo as *const c_void,flags:0},drm_ioctl_desc{cmd:3,handler:rocket_ioctl_fini_bo as *const c_void,flags:0}];
static rocket_drm_driver: drm_driver = drm_driver { driver_features: 0, open: Some(rocket_open), postclose: Some(rocket_postclose), gem_create_object: Some(rocket_gem_create_object), ioctls: rocket_drm_driver_ioctls.as_ptr(), num_ioctls: 4, fops: core::ptr::null(), name: b"rocket\0".as_ptr(), desc: b"rocket DRM\0".as_ptr() };

unsafe fn find_core_for_dev(dev: *mut device) -> i32 { let d=dev_get_drvdata(dev); for core in 0..(*d).num_cores { if (*(*d).cores.add(core as usize)).dev==dev { return core as i32; } } -1 }
unsafe extern "C" fn rocket_probe(pdev:*mut platform_device)->i32 { if rdev.is_null(){rdev=rocket_device_init(drm_dev,&rocket_drm_driver);if (rdev as isize)<0{return rdev as isize as i32;}} let core=(*rdev).num_cores; let dev=pdev as *mut device;dev_set_drvdata(dev,rdev);(*(*rdev).cores.add(core as usize)).rdev=rdev;(*(*rdev).cores.add(core as usize)).dev=dev;(*(*rdev).cores.add(core as usize)).index=core;(*rdev).num_cores+=1;let ret=rocket_core_init((*rdev).cores.add(core as usize) as *mut rocket_core);if ret!=0{(*rdev).num_cores-=1;if (*rdev).num_cores==0{rocket_device_fini(rdev);rdev=core::ptr::null_mut();}}ret }
unsafe extern "C" fn rocket_remove(pdev:*mut platform_device){let core=find_core_for_dev(pdev as *mut device);if core<0{return;}rocket_core_fini((*rdev).cores.add(core as usize) as *mut rocket_core);(*rdev).num_cores-=1;if (*rdev).num_cores==0{rocket_device_fini(rdev);rdev=core::ptr::null_mut();}}

unsafe extern "C" fn rocket_device_runtime_resume(dev: *mut device) -> i32 {
    let device = dev_get_drvdata(dev); let core = find_core_for_dev(dev); if core < 0 { return -19; }
    let ret = clk_bulk_prepare_enable(0, (*device).cores.add(core as usize).as_mut().unwrap().clks);
    if ret != 0 { dev_err(dev, b"failed to enable (%d) clocks for core %d\n\0".as_ptr(), ret, core); return ret; } 0
}
unsafe extern "C" fn rocket_device_runtime_suspend(dev: *mut device) -> i32 {
    let device = dev_get_drvdata(dev); let core = find_core_for_dev(dev); if core < 0 { return -19; }
    let c = device.as_ref().unwrap().cores.add(core as usize); if !rocket_job_is_idle(c as *mut rocket_core) { return -16; }
    clk_bulk_disable_unprepare(0, (*device).cores.add(core as usize).as_mut().unwrap().clks); 0
}

#[repr(C)] struct of_device_id_layout { compatible: *const u8 }
static dt_match: [of_device_id_layout; 2] = [of_device_id_layout { compatible: b"rockchip,rk3588-rknn-core\0".as_ptr() }, of_device_id_layout { compatible: core::ptr::null() }];

#[no_mangle] pub unsafe extern "C" fn rocket_register()->i32{drm_dev=platform_device_register_simple(b"rknn\0".as_ptr(),-1,core::ptr::null_mut(),0);if (drm_dev as isize)<0{return drm_dev as isize as i32;}platform_driver_register(&mut ROCKET_DRIVER)}
#[no_mangle] pub unsafe extern "C" fn rocket_unregister(){platform_driver_unregister(&mut ROCKET_DRIVER);platform_device_unregister(drm_dev);}
static mut ROCKET_DRIVER: platform_driver=platform_driver{probe:Some(rocket_probe),remove:Some(rocket_remove),driver:driver_inner{name:b"rocket\0".as_ptr(),pm:core::ptr::null(),of_match_table:core::ptr::null()}};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
