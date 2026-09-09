/* SPDX-License-Identifier: GPL-2.0 */

// LED_H header guard omitted; Rust module boundaries provide equivalent protection.

pub const LED7: i32 = 0x80; // top (or furthest right) LED
pub const LED6: i32 = 0x40;
pub const LED5: i32 = 0x20;
pub const LED4: i32 = 0x10;
pub const LED3: i32 = 0x08;
pub const LED2: i32 = 0x04;
pub const LED1: i32 = 0x02;
pub const LED0: i32 = 0x01; // bottom (or furthest left) LED

pub const LED_LAN_RCV: i32 = LED0; // for LAN receive activity
pub const LED_LAN_TX: i32 = LED1; // for LAN transmit activity
pub const LED_DISK_IO: i32 = LED2; // for disk activity
pub const LED_HEARTBEAT: i32 = LED3; // heartbeat

// values for pdc_chassis_lcd_info_ret_block.model:
pub const DISPLAY_MODEL_LCD: i32 = 0; // KittyHawk LED or LCD
pub const DISPLAY_MODEL_NONE: i32 = 1; // no LED or LCD
pub const DISPLAY_MODEL_LASI: i32 = 2; // LASI style 8 bit LED
pub const DISPLAY_MODEL_OLD_ASP: i32 = 0x7F; // faked: ASP style 8 x 1 bit LED (only very old ASP versions)

pub const LED_CMD_REG_NONE: i32 = 0; // NULL == no addr for the cmd register

unsafe extern "C" {
    // register_led_driver()
    pub fn register_led_driver(
        model: core::ffi::c_int,
        cmd_reg: core::ffi::c_ulong,
        data_reg: core::ffi::c_ulong,
    ) -> core::ffi::c_int;
}

// The following declaration is present only when CONFIG_CHASSIS_LCD_LED is enabled.
#[cfg(CONFIG_CHASSIS_LCD_LED)]
unsafe extern "C" {
    // writes a string to the LCD display (if possible on this h/w)
    pub fn lcd_print(str_: *const core::ffi::c_char);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
