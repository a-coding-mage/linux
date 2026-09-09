/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * max77693.h - Driver for the Maxim 77693
 *
 *  Copyright (C) 2012 Samsung Electronics
 *  SangYoung Son <hello.son@samsung.com>
 *
 * This program is not provided / owned by Maxim Integrated Products.
 *
 * This driver is based on max8997.h
 *
 * MAX77693 has PMIC, Charger, Flash LED, Haptic, MUIC devices.
 * The devices share the same I2C bus and included in
 * this mfd driver.
 */

/* C header dependency: u8 is supplied by the surrounding kernel bindings. */

/* MAX77693 regulator IDs */
#[repr(i32)]
pub enum max77693_regulators {
    MAX77693_ESAFEOUT1 = 0,
    MAX77693_ESAFEOUT2,
    MAX77693_CHARGER,
    MAX77693_REG_MAX,
}

#[repr(C)]
pub struct max77693_reg_data {
    pub addr: u8,
    pub data: u8,
}

#[repr(C)]
pub struct max77693_muic_platform_data {
    pub init_data: *mut max77693_reg_data,
    pub num_init_data: i32,

    pub detcable_delay_ms: i32,

    /*
     * Default usb/uart path whether UART/USB or AUX_UART/AUX_USB
     * h/w path of COMP2/COMN1 on CONTROL1 register.
     */
    pub path_usb: i32,
    pub path_uart: i32,
}

/* MAX77693 led flash */

/* triggers */
#[repr(i32)]
pub enum max77693_led_trigger {
    MAX77693_LED_TRIG_OFF,
    MAX77693_LED_TRIG_FLASH,
    MAX77693_LED_TRIG_TORCH,
    MAX77693_LED_TRIG_EXT,
    MAX77693_LED_TRIG_SOFT,
}

/* trigger types */
#[repr(i32)]
pub enum max77693_led_trigger_type {
    MAX77693_LED_TRIG_TYPE_EDGE,
    MAX77693_LED_TRIG_TYPE_LEVEL,
}

/* boost modes */
#[repr(i32)]
pub enum max77693_led_boost_mode {
    MAX77693_LED_BOOST_NONE,
    MAX77693_LED_BOOST_ADAPTIVE,
    MAX77693_LED_BOOST_FIXED,
}

/* MAX77693 */

/* Declared by the LED subsystem dependency. */
#[allow(non_camel_case_types)]
pub struct max77693_led_platform_data;

#[repr(C)]
pub struct max77693_platform_data {
    /* muic data */
    pub muic_data: *mut max77693_muic_platform_data,
    pub led_data: *mut max77693_led_platform_data,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
