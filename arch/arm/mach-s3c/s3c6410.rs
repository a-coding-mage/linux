// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2008 Simtec Electronics
// Copyright 2008 Simtec Electronics
//	Ben Dooks <ben@simtec.co.uk>
//	http://armlinux.simtec.co.uk/

/*
 * NOTE: Code in this file is not used when booting with Device Tree support.
 */

// Linux kernel and architecture dependencies supplied by other translation units.

extern "C" {
    fn s3c6410_default_sdhci0();
    fn s3c6410_default_sdhci1();
    fn s3c6410_default_sdhci2();
    fn s3c_i2c0_setname(name: *const core::ffi::c_char);
    fn s3c_i2c1_setname(name: *const core::ffi::c_char);
    fn s3c64xx_init_irq(vic0_irqs: u32, vic1_irqs: u32);
    fn of_have_populated_dt() -> bool;
    fn soc_is_s3c64xx() -> bool;
    fn subsys_system_register(subsys: *const bus_type, groups: *const core::ffi::c_void) -> i32;
    fn printk(format: *const core::ffi::c_char, ...);
    fn device_register(dev: *mut device) -> i32;
}

#[repr(C)]
pub struct bus_type {
    pub name: *const core::ffi::c_char,
    pub dev_name: *const core::ffi::c_char,
}

#[repr(C)]
pub struct device {
    pub bus: *const bus_type,
}

pub unsafe fn s3c6410_map_io() {
    /* initialise device information early */
    s3c6410_default_sdhci0();
    s3c6410_default_sdhci1();
    s3c6410_default_sdhci2();

    /* the i2c devices are directly compatible with s3c2440 */
    s3c_i2c0_setname(b"s3c2440-i2c\0".as_ptr() as *const core::ffi::c_char);
    s3c_i2c1_setname(b"s3c2440-i2c\0".as_ptr() as *const core::ffi::c_char);
}

pub unsafe fn s3c6410_init_irq() {
    /* VIC0 is missing IRQ7, VIC1 is fully populated. */
    s3c64xx_init_irq(!0u32 & !(1u32 << 7), !0u32);
}

#[no_mangle]
pub static s3c6410_subsys: bus_type = bus_type {
    name: b"s3c6410-core\0".as_ptr() as *const core::ffi::c_char,
    dev_name: b"s3c6410-core\0".as_ptr() as *const core::ffi::c_char,
};

static mut s3c6410_dev: device = device {
    bus: &s3c6410_subsys as *const bus_type,
};

unsafe fn s3c6410_core_init() -> i32 {
    /* Not applicable when using DT. */
    if of_have_populated_dt() || !soc_is_s3c64xx() {
        return 0;
    }

    subsys_system_register(&s3c6410_subsys, core::ptr::null())
}

// core_initcall(s3c6410_core_init);

pub unsafe fn s3c6410_init() -> i32 {
    printk(b"S3C6410: Initialising architecture\n\0".as_ptr() as *const core::ffi::c_char);

    device_register(&raw mut s3c6410_dev)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
