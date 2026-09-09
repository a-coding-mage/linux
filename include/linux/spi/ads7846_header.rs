/* SPDX-License-Identifier: GPL-2.0 */
/* linux/spi/ads7846.h */

#[repr(C)]
pub struct ads7846_platform_data {
    pub model: u16,                         /* 7843, 7845, 7846, 7873. */
    pub vref_delay_usecs: u16,              /* 0 for external vref; etc */
    pub vref_mv: u16,                       /* external vref value, milliVolts
                                             * ads7846: if 0, use internal vref */
    pub keep_vref_on: bool,                 /* set to keep vref on for differential
                                             * measurements as well */
    pub swap_xy: bool,                      /* swap x and y axes */

    /* Settling time of the analog signals; a function of Vcc and the
     * capacitance on the X/Y drivers.  If set to non-zero, two samples are
     * taken with settle_delay us apart, and the second one is used.
     * ~150 uSec with 0.01uF caps.
     */
    pub settle_delay_usecs: u16,

    /* If set to non-zero, after samples are taken this delay is applied
     * and penirq is rechecked, to help avoid false events.  This value
     * is affected by the material used to build the touch layer.
     */
    pub penirq_recheck_delay_usecs: u16,

    pub x_plate_ohms: u16,
    pub y_plate_ohms: u16,

    pub x_min: u16,
    pub x_max: u16,
    pub y_min: u16,
    pub y_max: u16,
    pub pressure_min: u16,
    pub pressure_max: u16,

    pub debounce_max: u16,                   /* max number of additional readings
                                             * per sample */
    pub debounce_tol: u16,                   /* tolerance used for filtering */
    pub debounce_rep: u16,                   /* additional consecutive good readings
                                             * required after the first two */
    pub gpio_pendown_debounce: i32,          /* platform specific debounce time for
                                             * the gpio_pendown */
    pub get_pendown_state: Option<unsafe extern "C" fn() -> i32>,
    pub wait_for_sync: Option<unsafe extern "C" fn()>,
    pub wakeup: bool,
    pub irq_flags: ::core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
