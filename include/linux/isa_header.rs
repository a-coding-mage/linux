/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ISA bus.
 */

// C dependencies supplied by the surrounding kernel translation:
// linux/device.h, linux/errno.h, and linux/kernel.h

#[repr(C)]
pub struct isa_driver {
    pub match_: Option<unsafe extern "C" fn(*mut device, ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub probe: Option<unsafe extern "C" fn(*mut device, ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut device, ::core::ffi::c_uint)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut device, ::core::ffi::c_uint)>,
    pub suspend: Option<unsafe extern "C" fn(
        *mut device,
        ::core::ffi::c_uint,
        pm_message_t,
    ) -> ::core::ffi::c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device, ::core::ffi::c_uint) -> ::core::ffi::c_int>,

    pub driver: device_driver,
    pub devices: *mut device,
}

#[macro_export]
macro_rules! to_isa_driver {
    ($x:expr) => {
        container_of!($x, isa_driver, driver)
    };
}

#[cfg(CONFIG_ISA_BUS_API)]
extern "C" {
    pub fn isa_register_driver(driver: *mut isa_driver, num: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn isa_unregister_driver(driver: *mut isa_driver);
}

#[cfg(not(CONFIG_ISA_BUS_API))]
#[inline]
pub unsafe fn isa_register_driver(
    _d: *mut isa_driver,
    _i: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    -(ENODEV as ::core::ffi::c_int)
}

#[cfg(not(CONFIG_ISA_BUS_API))]
#[inline]
pub unsafe fn isa_unregister_driver(_d: *mut isa_driver) {}

// The C macros below generate module init/exit functions. Rust identifier
// concatenation is retained through the surrounding kernel's module macros.
#[macro_export]
macro_rules! module_isa_driver_init {
    ($isa_driver:ident, $num_isa_dev:expr) => {
        module_init!($isa_driver, isa_register_driver(&mut $isa_driver, $num_isa_dev));
    };
}

#[macro_export]
macro_rules! module_isa_driver_with_irq_init {
    ($isa_driver:ident, $num_isa_dev:expr, $num_irq:expr) => {
        module_init!($isa_driver, {
            if $num_irq != $num_isa_dev {
                pr_err!(
                    "{}: Number of irq ({}) does not match number of base ({})\\n",
                    $isa_driver.driver.name,
                    $num_irq,
                    $num_isa_dev
                );
                return -(EINVAL as ::core::ffi::c_int);
            }
            isa_register_driver(&mut $isa_driver, $num_isa_dev)
        });
    };
}

#[macro_export]
macro_rules! module_isa_driver_exit {
    ($isa_driver:ident) => {
        module_exit!($isa_driver, isa_unregister_driver(&mut $isa_driver));
    };
}

/**
 * module_isa_driver() - Helper macro for registering a ISA driver
 * @__isa_driver: isa_driver struct
 * @__num_isa_dev: number of devices to register
 *
 * Helper macro for ISA drivers which do not do anything special in module
 * init/exit. This eliminates a lot of boilerplate code. Each module may only
 * use this macro once, and calling it replaces module_init and module_exit.
 */
#[macro_export]
macro_rules! module_isa_driver {
    ($isa_driver:ident, $num_isa_dev:expr) => {
        module_isa_driver_init!($isa_driver, $num_isa_dev);
        module_isa_driver_exit!($isa_driver);
    };
}

/**
 * module_isa_driver_with_irq() - Helper macro for registering an ISA driver with irq
 * @__isa_driver: isa_driver struct
 * @__num_isa_dev: number of devices to register
 * @__num_irq: number of IRQ to register
 *
 * Helper macro for ISA drivers with irq that do not do anything special in
 * module init/exit. Each module may only use this macro once, and calling it
 * replaces module_init and module_exit.
 */
#[macro_export]
macro_rules! module_isa_driver_with_irq {
    ($isa_driver:ident, $num_isa_dev:expr, $num_irq:expr) => {
        module_isa_driver_with_irq_init!($isa_driver, $num_isa_dev, $num_irq);
        module_isa_driver_exit!($isa_driver);
    };
}

/**
 * max_num_isa_dev() - Maximum possible number registered of an ISA device
 * @__ida_dev_ext: ISA device address extent
 *
 * The highest base address possible for an ISA device is 0x3FF; this results in
 * 1024 possible base addresses. Dividing the number of possible base addresses
 * by the address extent taken by each device results in the maximum number of
 * devices on a system.
 */
#[macro_export]
macro_rules! max_num_isa_dev {
    ($isa_dev_ext:expr) => {
        1024 / $isa_dev_ext
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
