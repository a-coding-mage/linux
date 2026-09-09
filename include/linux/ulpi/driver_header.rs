/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the Linux device-id/ulpi and device interfaces.

pub struct ulpi {
    pub dev: device,
    pub id: ulpi_device_id,
    pub ops: *const ulpi_ops,
}

pub struct ulpi_ops;

// Equivalent to: container_of(d, struct ulpi, dev).
#[macro_export]
macro_rules! to_ulpi_dev {
    ($d:expr) => {
        unsafe { container_of!($d, ulpi, dev) }
    };
}

#[inline]
pub unsafe fn ulpi_set_drvdata(ulpi: *mut ulpi, data: *mut core::ffi::c_void) {
    dev_set_drvdata(core::ptr::addr_of_mut!((*ulpi).dev), data);
}

#[inline]
pub unsafe fn ulpi_get_drvdata(ulpi: *mut ulpi) -> *mut core::ffi::c_void {
    dev_get_drvdata(core::ptr::addr_of_mut!((*ulpi).dev))
}

pub struct ulpi_driver {
    pub id_table: *const ulpi_device_id,
    pub probe: Option<unsafe extern "C" fn(ulpi: *mut ulpi) -> core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn(ulpi: *mut ulpi)>,
    pub driver: device_driver,
}

// Equivalent to: container_of(d, struct ulpi_driver, driver).
#[macro_export]
macro_rules! to_ulpi_driver {
    ($d:expr) => {
        unsafe { container_of!($d, ulpi_driver, driver) }
    };
}

// Use a macro to avoid include chaining to get THIS_MODULE.
#[macro_export]
macro_rules! ulpi_register_driver {
    ($drv:expr) => {
        __ulpi_register_driver($drv, THIS_MODULE)
    };
}

unsafe extern "C" {
    pub fn __ulpi_register_driver(
        drv: *mut ulpi_driver,
        module: *mut module,
    ) -> core::ffi::c_int;
    pub fn ulpi_unregister_driver(drv: *mut ulpi_driver);
    pub fn ulpi_read(ulpi: *mut ulpi, addr: u8) -> core::ffi::c_int;
    pub fn ulpi_write(ulpi: *mut ulpi, addr: u8, val: u8) -> core::ffi::c_int;
}

// Equivalent to module_driver(__ulpi_driver, ulpi_register_driver,
// ulpi_unregister_driver).
#[macro_export]
macro_rules! module_ulpi_driver {
    ($ulpi_driver:expr) => {
        module_driver!($ulpi_driver, ulpi_register_driver, ulpi_unregister_driver)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
