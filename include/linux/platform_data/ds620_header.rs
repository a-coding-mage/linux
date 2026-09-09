/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
// #include <linux/types.h>
// #include <linux/i2c.h>

/* platform data for the DS620 temperature sensor and thermostat */

#[repr(C)]
pub struct ds620_platform_data {
    /*
     *  Thermostat output pin PO mode:
     *  0 = always low (default)
     *  1 = PO_LOW
     *  2 = PO_HIGH
     *
     * (see Documentation/hwmon/ds620.rst)
     */
    pub pomode: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
