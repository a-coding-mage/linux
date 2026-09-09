/* SPDX-License-Identifier: GPL-2.0 */
/* linux/spi/ad7877.h */

/* Touchscreen characteristics vary between boards and models.  The
 * platform_data for the device's "struct device" holds this information.
 *
 * It's OK if the min/max values are zero.
 */
#[repr(C)]
pub struct ad7877_platform_data {
    pub model: u16,                 /* 7877 */
    pub vref_delay_usecs: u16,      /* 0 for external vref; etc */
    pub x_plate_ohms: u16,
    pub y_plate_ohms: u16,

    pub x_min: u16,
    pub x_max: u16,
    pub y_min: u16,
    pub y_max: u16,
    pub pressure_min: u16,
    pub pressure_max: u16,

    pub stopacq_polarity: u8,       /* 1 = Active HIGH, 0 = Active LOW */
    pub first_conversion_delay: u8, /* 0 = 0.5us, 1 = 128us, 2 = 1ms, 3 = 8ms */
    pub acquisition_time: u8,       /* 0 = 2us, 1 = 4us, 2 = 8us, 3 = 16us */
    pub averaging: u8,              /* 0 = 1, 1 = 4, 2 = 8, 3 = 16 */
    pub pen_down_acc_interval: u8,  /* 0 = covert once, 1 = every 0.5 ms,
                                       2 = ever 1 ms,   3 = every 8 ms,*/
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
