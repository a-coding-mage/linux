/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2014 Imagination Technologies Ltd.
 */

// Dependencies supplied by the surrounding kernel translation.

/**
 * struct mips_cdmm_device - Represents a single device on a CDMM bus.
 * @dev:     Driver model device object.
 * @cpu:     CPU which can access this device.
 * @res:     MMIO resource.
 * @type:    Device type identifier.
 * @rev:     Device revision number.
 */
#[repr(C)]
pub struct mips_cdmm_device {
    pub dev: device,
    pub cpu: ::core::ffi::c_uint,
    pub res: resource,
    pub type_: ::core::ffi::c_uint,
    pub rev: ::core::ffi::c_uint,
}

/**
 * struct mips_cdmm_driver - Represents a driver for a CDMM device.
 * @drv:      Driver model driver object.
 * @probe:    Callback for probing newly discovered devices.
 * @remove:   Callback to remove the device.
 * @shutdown: Callback on system shutdown.
 * @cpu_down: Callback when the parent CPU is going down.
 *            Any CPU pinned threads/timers should be disabled.
 * @cpu_up:   Callback when the parent CPU is coming back up again.
 *            CPU pinned threads/timers can be restarted.
 * @id_table: Table for CDMM IDs to match against.
 */
#[repr(C)]
pub struct mips_cdmm_driver {
    pub drv: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut mips_cdmm_device) -> ::core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut mips_cdmm_device) -> ::core::ffi::c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut mips_cdmm_device)>,
    pub cpu_down: Option<unsafe extern "C" fn(*mut mips_cdmm_device) -> ::core::ffi::c_int>,
    pub cpu_up: Option<unsafe extern "C" fn(*mut mips_cdmm_device) -> ::core::ffi::c_int>,
    pub id_table: *const mips_cdmm_device_id,
}

/**
 * mips_cdmm_phys_base() - Choose a physical base address for CDMM region.
 *
 * Picking a suitable physical address at which to map the CDMM region is
 * platform specific, so this function can be defined by platform code to
 * pick a suitable value if none is configured by the bootloader.
 *
 * This address must be 32kB aligned, and the region occupies a maximum of 32kB
 * of physical address space which must not be used for anything else.
 *
 * Returns:     Physical base address for CDMM region, or 0 on failure.
 */
unsafe extern "C" {
    pub fn mips_cdmm_phys_base() -> phys_addr_t;
    pub static mips_cdmm_bustype: bus_type;
    pub fn mips_cdmm_early_probe(dev_type: ::core::ffi::c_uint) -> *mut ::core::ffi::c_void;
    pub fn mips_cdmm_driver_register(driver: *mut mips_cdmm_driver) -> ::core::ffi::c_int;
    pub fn mips_cdmm_driver_unregister(driver: *mut mips_cdmm_driver);
}

// Equivalent of container_of(d, struct mips_cdmm_device, dev).
#[inline]
pub unsafe fn to_mips_cdmm_device(d: *mut device) -> *mut mips_cdmm_device {
    (d as *mut u8).sub(::core::mem::offset_of!(mips_cdmm_device, dev)) as *mut mips_cdmm_device
}

#[inline]
pub unsafe fn mips_cdmm_get_drvdata(d: *mut mips_cdmm_device) -> *mut ::core::ffi::c_void {
    dev_get_drvdata(&mut (*d).dev)
}

#[inline]
pub unsafe fn mips_cdmm_set_drvdata(d: *mut mips_cdmm_device, p: *mut ::core::ffi::c_void) {
    dev_set_drvdata(&mut (*d).dev, p);
}

/*
 * module_mips_cdmm_driver() - Helper macro for drivers that don't do
 * anything special in module init/exit.  This eliminates a lot of
 * boilerplate.  Each module may only use this macro once, and
 * calling it replaces module_init() and module_exit()
 */
// The C module_driver expansion is supplied by the surrounding kernel.

/*
 * builtin_mips_cdmm_driver() - Helper macro for drivers that don't do anything
 * special in init and have no exit. This eliminates some boilerplate. Each
 * driver may only use this macro once, and calling it replaces device_initcall
 * (or in some cases, the legacy __initcall). This is meant to be a direct
 * parallel of module_mips_cdmm_driver() above but without the __exit stuff that
 * is not used for builtin cases.
 */
// The C builtin_driver expansion is supplied by the surrounding kernel.

/* drivers/tty/mips_ejtag_fdc.c */

// CONFIG_MIPS_EJTAG_FDC_EARLYCON selects the external implementation.
#[cfg(feature = "CONFIG_MIPS_EJTAG_FDC_EARLYCON")]
unsafe extern "C" {
    pub fn setup_early_fdc_console() -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_MIPS_EJTAG_FDC_EARLYCON"))]
#[inline]
pub fn setup_early_fdc_console() -> ::core::ffi::c_int {
    -19 /* -ENODEV */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
