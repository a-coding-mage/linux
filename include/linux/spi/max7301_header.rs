/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the Linux GPIO driver headers:
// #include <linux/gpio/driver.h>

/*
 * Some registers must be read back to modify.
 * To save time we cache them here in memory
 */
#[repr(C)]
pub struct max7301 {
    pub lock: mutex,
    pub port_config: [u8; 8], /* field 0 is unused */
    pub out_level: u32, /* cached output levels */
    pub input_pullup_active: u32,
    pub chip: gpio_chip,
    pub dev: *mut device,
    pub write: Option<unsafe extern "C" fn(dev: *mut device, reg: u32, val: u32) -> i32>,
    pub read: Option<unsafe extern "C" fn(dev: *mut device, reg: u32) -> i32>,
}

#[repr(C)]
pub struct max7301_platform_data {
    /* number assigned to the first GPIO */
    pub base: u32,
    /*
     * bitmask controlling the pullup configuration,
     *
     * _note_ the 4 lowest bits are unused, because the first 4
     * ports of the controller are not used, too.
     */
    pub input_pullup_active: u32,
}

extern "C" {
    pub fn __max730x_remove(dev: *mut device);
    pub fn __max730x_probe(ts: *mut max7301) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
