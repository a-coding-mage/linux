/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2013-2015 Intel Corporation. All rights reserved.
 */

// C dependencies: linux/fs.h, linux/ndctl.h, linux/device.h,
// linux/badblocks.h, and linux/perf_event.h.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvdimm_event {
    NVDIMM_REVALIDATE_POISON,
    NVDIMM_REVALIDATE_REGION,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvdimm_claim_class {
    NVDIMM_CCLASS_NONE,
    NVDIMM_CCLASS_BTT,
    NVDIMM_CCLASS_BTT2,
    NVDIMM_CCLASS_PFN,
    NVDIMM_CCLASS_DAX,
    NVDIMM_CCLASS_UNKNOWN,
}

// NVDIMM_EVENT_VAR(_id) expands to event_attr_##_id.
// NVDIMM_EVENT_PTR(_id) expands to (&event_attr_##_id.attr.attr).
// NVDIMM_EVENT_ATTR(_name, _id) expands to PMU_EVENT_ATTR(_name,
// NVDIMM_EVENT_VAR(_id), _id, nvdimm_events_sysfs_show).

pub const NVDIMM_PMU_FORMAT_ATTR: usize = 0;
pub const NVDIMM_PMU_EVENT_ATTR: usize = 1;
pub const NVDIMM_PMU_CPUMASK_ATTR: usize = 2;
pub const NVDIMM_PMU_NULL_ATTR: usize = 3;

#[repr(C)]
pub struct nvdimm_pmu {
    pub pmu: pmu,
    pub dev: *mut device,
    pub cpu: core::ffi::c_int,
    pub node: hlist_node,
    pub cpuhp_state: cpuhp_state,
    /* cpumask provided by arch/platform specific code */
    pub arch_cpumask: cpumask,
}

#[repr(C)]
pub struct platform_device;

// The CONFIG_PERF_EVENTS branch is selected by the build configuration.
#[cfg(feature = "CONFIG_PERF_EVENTS")]
extern "C" {
    pub fn nvdimm_events_sysfs_show(
        dev: *mut device,
        attr: *mut device_attribute,
        page: *mut core::ffi::c_char,
    ) -> ssize_t;

    pub fn register_nvdimm_pmu(nvdimm: *mut nvdimm_pmu, pdev: *mut platform_device) -> core::ffi::c_int;
    pub fn unregister_nvdimm_pmu(nd_pmu: *mut nvdimm_pmu);
}

#[cfg(not(feature = "CONFIG_PERF_EVENTS"))]
pub unsafe fn register_nvdimm_pmu(_nvdimm: *mut nvdimm_pmu, _pdev: *mut platform_device) -> core::ffi::c_int {
    -ENXIO
}

#[cfg(not(feature = "CONFIG_PERF_EVENTS"))]
pub unsafe fn unregister_nvdimm_pmu(_nd_pmu: *mut nvdimm_pmu) {}

#[repr(C)]
pub struct nd_device_driver {
    pub drv: device_driver,
    pub type_: c_ulong,
    pub probe: Option<unsafe extern "C" fn(dev: *mut device) -> core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn(dev: *mut device)>,
    pub shutdown: Option<unsafe extern "C" fn(dev: *mut device)>,
    pub notify: Option<unsafe extern "C" fn(dev: *mut device, event: nvdimm_event)>,
}

// to_nd_device_driver(__drv) expands to container_of_const(__drv,
// struct nd_device_driver, drv).

#[repr(C)]
pub struct nd_namespace_common {
    pub force_raw: core::ffi::c_int,
    pub dev: device,
    pub claim: *mut device,
    pub claim_class: nvdimm_claim_class,
    pub rw_bytes: Option<unsafe extern "C" fn(
        *mut nd_namespace_common,
        resource_size_t,
        *mut core::ffi::c_void,
        usize,
        core::ffi::c_int,
        c_ulong,
    ) -> core::ffi::c_int>,
}

pub unsafe fn to_ndns(dev: *mut device) -> *mut nd_namespace_common {
    container_of(dev, core::mem::offset_of!(nd_namespace_common, dev))
}

#[repr(C)]
pub struct nd_namespace_io {
    pub common: nd_namespace_common,
    pub res: resource,
    pub size: resource_size_t,
    pub addr: *mut core::ffi::c_void,
    pub bb: badblocks,
}

#[repr(C)]
pub struct nd_namespace_pmem {
    pub nsio: nd_namespace_io,
    pub lbasize: c_ulong,
    pub alt_name: *mut core::ffi::c_char,
    pub uuid: *mut uuid_t,
    pub id: core::ffi::c_int,
}

pub unsafe fn to_nd_namespace_io(dev: *const device) -> *mut nd_namespace_io {
    container_of(dev as *mut device, core::mem::offset_of!(nd_namespace_io, common) + core::mem::offset_of!(nd_namespace_common, dev))
}

pub unsafe fn to_nd_namespace_pmem(dev: *const device) -> *mut nd_namespace_pmem {
    let nsio = to_nd_namespace_io(dev);
    container_of(nsio, core::mem::offset_of!(nd_namespace_pmem, nsio))
}

pub unsafe fn nvdimm_read_bytes(
    ndns: *mut nd_namespace_common,
    offset: resource_size_t,
    buf: *mut core::ffi::c_void,
    size: usize,
    flags: c_ulong,
) -> core::ffi::c_int {
    ((*ndns).rw_bytes.unwrap())(ndns, offset, buf, size, READ, flags)
}

pub unsafe fn nvdimm_write_bytes(
    ndns: *mut nd_namespace_common,
    offset: resource_size_t,
    buf: *mut core::ffi::c_void,
    size: usize,
    flags: c_ulong,
) -> core::ffi::c_int {
    ((*ndns).rw_bytes.unwrap())(ndns, offset, buf, size, WRITE, flags)
}

// MODULE_ALIAS_ND_DEVICE(type) expands to MODULE_ALIAS("nd:t" __stringify(type) "*").
pub const ND_DEVICE_MODALIAS_FMT: &str = "nd:t%d";

#[repr(C)]
pub struct nd_region;

extern "C" {
    pub fn nvdimm_region_notify(nd_region: *mut nd_region, event: nvdimm_event);
    pub fn __nd_driver_register(
        nd_drv: *mut nd_device_driver,
        module: *mut module,
        mod_name: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    pub fn driver_unregister(drv: *mut device_driver);
}

pub unsafe fn nd_driver_unregister(drv: *mut nd_device_driver) {
    driver_unregister(&mut (*drv).drv);
}

// nd_driver_register(driver) expands to __nd_driver_register(driver, THIS_MODULE, KBUILD_MODNAME).
// module_nd_driver(driver) expands to module_driver(driver, nd_driver_register, nd_driver_unregister).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
