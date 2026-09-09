/* SPDX-License-Identifier: GPL-2.0 */

/* Declarations supplied by the Linux kernel and related headers. */
pub struct device;
pub struct mutex;
pub struct list_head;
pub struct module;
pub struct fpga_manager;
pub struct fpga_bridge;
pub struct fpga_image_info;
pub struct fpga_compat_id;

pub struct fpga_region;

/**
 * struct fpga_region_info - collection of parameters an FPGA Region
 * @mgr: fpga region manager
 * @compat_id: FPGA region id for compatibility check.
 * @priv: fpga region private data
 * @get_bridges: optional function to get bridges to a list
 *
 * fpga_region_info contains parameters for the register_full function.
 * These are separated into an info structure because they some are optional
 * others could be added to in the future. The info structure facilitates
 * maintaining a stable API.
 */
#[repr(C)]
pub struct fpga_region_info {
    pub mgr: *mut fpga_manager,
    pub compat_id: *mut fpga_compat_id,
    pub priv_: *mut core::ffi::c_void,
    pub get_bridges: Option<unsafe extern "C" fn(region: *mut fpga_region) -> core::ffi::c_int>,
}

/**
 * struct fpga_region - FPGA Region structure
 * @dev: FPGA Region device
 * @mutex: enforces exclusive reference to region
 * @bridge_list: list of FPGA bridges specified in region
 * @mgr: FPGA manager
 * @info: FPGA image info
 * @compat_id: FPGA region id for compatibility check.
 * @ops_owner: module containing the get_bridges function
 * @priv: private data
 * @get_bridges: optional function to get bridges to a list
 */
#[repr(C)]
pub struct fpga_region {
    pub dev: device,
    pub mutex: mutex, /* for exclusive reference to region */
    pub bridge_list: list_head,
    pub mgr: *mut fpga_manager,
    pub info: *mut fpga_image_info,
    pub compat_id: *mut fpga_compat_id,
    pub ops_owner: *mut module,
    pub priv_: *mut core::ffi::c_void,
    pub get_bridges: Option<unsafe extern "C" fn(region: *mut fpga_region) -> core::ffi::c_int>,
}

#[inline]
pub unsafe fn to_fpga_region(d: *mut device) -> *mut fpga_region {
    d as *mut fpga_region
}

unsafe extern "C" {
    pub fn fpga_region_class_find(
        start: *mut device,
        data: *const core::ffi::c_void,
        r#match: Option<unsafe extern "C" fn(*mut device, *const core::ffi::c_void) -> core::ffi::c_int>,
    ) -> *mut fpga_region;

    pub fn fpga_region_program_fpga(region: *mut fpga_region) -> core::ffi::c_int;

    pub fn __fpga_region_register_full(
        parent: *mut device,
        info: *const fpga_region_info,
        owner: *mut module,
    ) -> *mut fpga_region;

    pub fn __fpga_region_register(
        parent: *mut device,
        mgr: *mut fpga_manager,
        get_bridges: Option<unsafe extern "C" fn(*mut fpga_region) -> core::ffi::c_int>,
        owner: *mut module,
    ) -> *mut fpga_region;

    pub fn fpga_region_unregister(region: *mut fpga_region);
}

/* The C macros pass THIS_MODULE as the owner; the corresponding symbol is
 * supplied by the kernel build environment. */
extern "C" {
    static mut THIS_MODULE: *mut module;
}

#[inline]
pub unsafe fn fpga_region_register_full(
    parent: *mut device,
    info: *const fpga_region_info,
) -> *mut fpga_region {
    __fpga_region_register_full(parent, info, THIS_MODULE)
}

#[inline]
pub unsafe fn fpga_region_register(
    parent: *mut device,
    mgr: *mut fpga_manager,
    get_bridges: Option<unsafe extern "C" fn(*mut fpga_region) -> core::ffi::c_int>,
) -> *mut fpga_region {
    __fpga_region_register(parent, mgr, get_bridges, THIS_MODULE)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
