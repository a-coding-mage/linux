/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * KB3310B Embedded Controller
 *
 *  Copyright (C) 2008 Lemote Inc.
 *  Author: liujl <liujl@lemote.com>, 2008-03-14
 */

pub unsafe extern "C" {
    pub fn ec_read(addr: u16) -> u8;
    pub fn ec_write(addr: u16, val: u8);
    pub fn ec_query_seq(cmd: u8) -> i32;
    pub fn ec_query_event_num() -> i32;
    pub fn ec_get_event_num() -> i32;
}

pub type sci_handler = unsafe extern "C" fn(status: i32) -> i32;
pub unsafe extern "C" {
    pub static mut yeeloong_report_lid_status: Option<sci_handler>;
}

pub const SCI_IRQ_NUM: u32 = 0x0A;

/*
 * The following registers are determined by the EC index configuration.
 * 1, fill the PORT_HIGH as EC register high part.
 * 2, fill the PORT_LOW as EC register low part.
 * 3, fill the PORT_DATA as EC register write data or get the data from it.
 */
pub const EC_IO_PORT_HIGH: u32 = 0x0381;
pub const EC_IO_PORT_LOW: u32 = 0x0382;
pub const EC_IO_PORT_DATA: u32 = 0x0383;

/* EC delay time is 500us for register and status access */
pub const EC_REG_DELAY: u32 = 500; /* unit : us */
pub const EC_CMD_TIMEOUT: u32 = 0x1000;

/* EC access port for SCI communication */
pub const EC_CMD_PORT: u32 = 0x66;
pub const EC_STS_PORT: u32 = 0x66;
pub const EC_DAT_PORT: u32 = 0x62;
pub const CMD_INIT_IDLE_MODE: u32 = 0xdd;
pub const CMD_EXIT_IDLE_MODE: u32 = 0xdf;
pub const CMD_INIT_RESET_MODE: u32 = 0xd8;
pub const CMD_REBOOT_SYSTEM: u32 = 0x8c;
pub const CMD_GET_EVENT_NUM: u32 = 0x84;
pub const CMD_PROGRAM_PIECE: u32 = 0xda;

/* temperature & fan registers */
pub const REG_TEMPERATURE_VALUE: u32 = 0xF458;
pub const REG_FAN_AUTO_MAN_SWITCH: u32 = 0xF459;
pub const BIT_FAN_AUTO: u32 = 0;
pub const BIT_FAN_MANUAL: u32 = 1;
pub const REG_FAN_CONTROL: u32 = 0xF4D2;
pub const BIT_FAN_CONTROL_ON: u32 = 1 << 0;
pub const BIT_FAN_CONTROL_OFF: u32 = 0 << 0;
pub const REG_FAN_STATUS: u32 = 0xF4DA;
pub const BIT_FAN_STATUS_ON: u32 = 1 << 0;
pub const BIT_FAN_STATUS_OFF: u32 = 0 << 0;
pub const REG_FAN_SPEED_HIGH: u32 = 0xFE22;
pub const REG_FAN_SPEED_LOW: u32 = 0xFE23;
pub const REG_FAN_SPEED_LEVEL: u32 = 0xF4CC;
/* fan speed divider */
pub const FAN_SPEED_DIVIDER: u32 = 480000; /* (60*1000*1000/62.5/2)*/

/* battery registers */
pub const REG_BAT_DESIGN_CAP_HIGH: u32 = 0xF77D;
pub const REG_BAT_DESIGN_CAP_LOW: u32 = 0xF77E;
pub const REG_BAT_FULLCHG_CAP_HIGH: u32 = 0xF780;
pub const REG_BAT_FULLCHG_CAP_LOW: u32 = 0xF781;
pub const REG_BAT_DESIGN_VOL_HIGH: u32 = 0xF782;
pub const REG_BAT_DESIGN_VOL_LOW: u32 = 0xF783;
pub const REG_BAT_CURRENT_HIGH: u32 = 0xF784;
pub const REG_BAT_CURRENT_LOW: u32 = 0xF785;
pub const REG_BAT_VOLTAGE_HIGH: u32 = 0xF786;
pub const REG_BAT_VOLTAGE_LOW: u32 = 0xF787;
pub const REG_BAT_TEMPERATURE_HIGH: u32 = 0xF788;
pub const REG_BAT_TEMPERATURE_LOW: u32 = 0xF789;
pub const REG_BAT_RELATIVE_CAP_HIGH: u32 = 0xF492;
pub const REG_BAT_RELATIVE_CAP_LOW: u32 = 0xF493;
pub const REG_BAT_VENDOR: u32 = 0xF4C4;
pub const FLAG_BAT_VENDOR_SANYO: u32 = 0x01;
pub const FLAG_BAT_VENDOR_SIMPLO: u32 = 0x02;
pub const REG_BAT_CELL_COUNT: u32 = 0xF4C6;
pub const FLAG_BAT_CELL_3S1P: u32 = 0x03;
pub const FLAG_BAT_CELL_3S2P: u32 = 0x06;
pub const REG_BAT_CHARGE: u32 = 0xF4A2;
pub const FLAG_BAT_CHARGE_DISCHARGE: u32 = 0x01;
pub const FLAG_BAT_CHARGE_CHARGE: u32 = 0x02;
pub const FLAG_BAT_CHARGE_ACPOWER: u32 = 0x00;
pub const REG_BAT_STATUS: u32 = 0xF4B0;
pub const BIT_BAT_STATUS_LOW: u32 = 1 << 5;
pub const BIT_BAT_STATUS_DESTROY: u32 = 1 << 2;
pub const BIT_BAT_STATUS_FULL: u32 = 1 << 1;
pub const BIT_BAT_STATUS_IN: u32 = 1 << 0;
pub const REG_BAT_CHARGE_STATUS: u32 = 0xF4B1;
pub const BIT_BAT_CHARGE_STATUS_OVERTEMP: u32 = 1 << 2;
pub const BIT_BAT_CHARGE_STATUS_PRECHG: u32 = 1 << 1;
pub const REG_BAT_STATE: u32 = 0xF482;
pub const BIT_BAT_STATE_CHARGING: u32 = 1 << 1;
pub const BIT_BAT_STATE_DISCHARGING: u32 = 1 << 0;
pub const REG_BAT_POWER: u32 = 0xF440;
pub const BIT_BAT_POWER_S3: u32 = 1 << 2;
pub const BIT_BAT_POWER_ON: u32 = 1 << 1;
pub const BIT_BAT_POWER_ACIN: u32 = 1 << 0;

/* other registers */
/* Audio: rd/wr */
pub const REG_AUDIO_VOLUME: u32 = 0xF46C;
pub const REG_AUDIO_MUTE: u32 = 0xF4E7;
pub const REG_AUDIO_BEEP: u32 = 0xF4D0;
/* USB port power or not: rd/wr */
pub const REG_USB0_FLAG: u32 = 0xF461;
pub const REG_USB1_FLAG: u32 = 0xF462;
pub const REG_USB2_FLAG: u32 = 0xF463;
pub const BIT_USB_FLAG_ON: u32 = 1;
pub const BIT_USB_FLAG_OFF: u32 = 0;
/* LID */
pub const REG_LID_DETECT: u32 = 0xF4BD;
pub const BIT_LID_DETECT_ON: u32 = 1;
pub const BIT_LID_DETECT_OFF: u32 = 0;
/* CRT */
pub const REG_CRT_DETECT: u32 = 0xF4AD;
pub const BIT_CRT_DETECT_PLUG: u32 = 1;
pub const BIT_CRT_DETECT_UNPLUG: u32 = 0;
/* LCD backlight brightness adjust: 9 levels */
pub const REG_DISPLAY_BRIGHTNESS: u32 = 0xF4F5;
/* Black screen Status */
pub const BIT_DISPLAY_LCD_ON: u32 = 1;
pub const BIT_DISPLAY_LCD_OFF: u32 = 0;
/* LCD backlight control: off/restore */
pub const REG_BACKLIGHT_CTRL: u32 = 0xF7BD;
pub const BIT_BACKLIGHT_ON: u32 = 1;
pub const BIT_BACKLIGHT_OFF: u32 = 0;
/* Reset the machine auto-clear: rd/wr */
pub const REG_RESET: u32 = 0xF4EC;
pub const BIT_RESET_ON: u32 = 1;
/* Light the led: rd/wr */
pub const REG_LED: u32 = 0xF4C8;
pub const BIT_LED_RED_POWER: u32 = 1 << 0;
pub const BIT_LED_ORANGE_POWER: u32 = 1 << 1;
pub const BIT_LED_GREEN_CHARGE: u32 = 1 << 2;
pub const BIT_LED_RED_CHARGE: u32 = 1 << 3;
pub const BIT_LED_NUMLOCK: u32 = 1 << 4;
/* Test led mode, all led on/off */
pub const REG_LED_TEST: u32 = 0xF4C2;
pub const BIT_LED_TEST_IN: u32 = 1;
pub const BIT_LED_TEST_OUT: u32 = 0;
/* Camera on/off */
pub const REG_CAMERA_STATUS: u32 = 0xF46A;
pub const BIT_CAMERA_STATUS_ON: u32 = 1;
pub const BIT_CAMERA_STATUS_OFF: u32 = 0;
pub const REG_CAMERA_CONTROL: u32 = 0xF7B7;
pub const BIT_CAMERA_CONTROL_OFF: u32 = 0;
pub const BIT_CAMERA_CONTROL_ON: u32 = 1;
/* Wlan Status */
pub const REG_WLAN: u32 = 0xF4FA;
pub const BIT_WLAN_ON: u32 = 1;
pub const BIT_WLAN_OFF: u32 = 0;
pub const REG_DISPLAY_LCD: u32 = 0xF79F;

/* SCI Event Number from EC */
#[repr(i32)]
pub enum Event {
    EVENT_LID = 0x23, /*  LID open/close */
    EVENT_DISPLAY_TOGGLE, /*  Fn+F3 for display switch */
    EVENT_SLEEP, /*  Fn+F1 for entering sleep mode */
    EVENT_OVERTEMP, /*  Over-temperature happened */
    EVENT_CRT_DETECT, /*  CRT is connected */
    EVENT_CAMERA, /*  Camera on/off */
    EVENT_USB_OC2, /*  USB2 Over Current occurred */
    EVENT_USB_OC0, /*  USB0 Over Current occurred */
    EVENT_BLACK_SCREEN, /*  Turn on/off backlight */
    EVENT_AUDIO_MUTE, /*  Mute on/off */
    EVENT_DISPLAY_BRIGHTNESS, /* LCD backlight brightness adjust */
    EVENT_AC_BAT, /*  AC & Battery relative issue */
    EVENT_AUDIO_VOLUME, /*  Volume adjust */
    EVENT_WLAN, /*  Wlan on/off */
    EVENT_END,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
