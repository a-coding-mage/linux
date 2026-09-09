// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2023 Huawei. All rights reserved. */

// Dependencies supplied by the surrounding kernel/CXL implementation.

#[repr(C)]
pub struct Device {
    pub parent: *mut Device,
    pub bus: *const BusType,
    pub type_: *const DeviceType,
}

#[repr(C)]
pub struct DeviceType {
    pub name: *const core::ffi::c_char,
    pub release: Option<unsafe extern "C" fn(*mut Device)>,
}

#[repr(C)]
pub struct BusType;

#[repr(C)]
pub struct CxlPmu {
    pub dev: Device,
    pub assoc_id: core::ffi::c_int,
    pub index: core::ffi::c_int,
    pub type_: CxlPmuType,
    pub base: u64,
}

#[repr(C)]
pub struct CxlPmuRegs {
    pub pmu: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum CxlPmuType {
    CxlPmuMemdev,
}

unsafe extern "C" {
    fn kfree(ptr: *mut core::ffi::c_void);
    fn device_unregister(dev: *mut Device);
    fn device_initialize(dev: *mut Device);
    fn device_set_pm_not_required(dev: *mut Device);
    fn dev_set_name(dev: *mut Device, fmt: *const core::ffi::c_char, ... ) -> core::ffi::c_int;
    fn device_add(dev: *mut Device) -> core::ffi::c_int;
    fn devm_add_action_or_reset(
        parent: *mut Device,
        action: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
        data: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn put_device(dev: *mut Device);
    fn kzalloc_obj<T>() -> *mut T;
    fn to_cxl_pmu(dev: *mut Device) -> *mut CxlPmu;
}

unsafe extern "C" {
    static cxl_bus_type: BusType;
}

unsafe extern "C" fn cxl_pmu_release(dev: *mut Device) {
    let pmu = to_cxl_pmu(dev);

    kfree(pmu.cast());
}

#[no_mangle]
pub static cxl_pmu_type: DeviceType = DeviceType {
    name: c"cxl_pmu".as_ptr(),
    release: Some(cxl_pmu_release),
};

unsafe extern "C" fn remove_dev(dev: *mut core::ffi::c_void) {
    device_unregister(dev.cast());
}

#[no_mangle]
pub unsafe extern "C" fn devm_cxl_pmu_add(
    parent: *mut Device,
    regs: *mut CxlPmuRegs,
    assoc_id: core::ffi::c_int,
    index: core::ffi::c_int,
    type_: CxlPmuType,
) -> core::ffi::c_int {
    let pmu: *mut CxlPmu;
    let dev: *mut Device;
    let rc: core::ffi::c_int;

    pmu = kzalloc_obj::<CxlPmu>();
    if pmu.is_null() {
        return -12; // -ENOMEM
    }

    (*pmu).assoc_id = assoc_id;
    (*pmu).index = index;
    (*pmu).type_ = type_;
    (*pmu).base = (*regs).pmu;
    dev = &mut (*pmu).dev;
    device_initialize(dev);
    device_set_pm_not_required(dev);
    (*dev).parent = parent;
    (*dev).bus = &cxl_bus_type;
    (*dev).type_ = &cxl_pmu_type;
    rc = match (*pmu).type_ {
        CxlPmuType::CxlPmuMemdev => {
            dev_set_name(dev, c"pmu_mem%d.%d".as_ptr(), assoc_id, index)
        }
    };
    if rc != 0 {
        put_device(&mut (*pmu).dev);
        return rc;
    }

    let rc = device_add(dev);
    if rc != 0 {
        put_device(&mut (*pmu).dev);
        return rc;
    }

    devm_add_action_or_reset(parent, Some(remove_dev), dev.cast())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
