/* SPDX-License-Identifier: (GPL-2.0 OR BSD-2-Clause) */
/*
 * This header provides macros for the common LEDs device tree bindings.
 *
 * Copyright (C) 2015, Samsung Electronics Co., Ltd.
 * Author: Jacek Anaszewski <j.anaszewski@samsung.com>
 *
 * Copyright (C) 2019 Jacek Anaszewski <jacek.anaszewski@gmail.com>
 * Copyright (C) 2020 Pavel Machek <pavel@ucw.cz>
 */

/* External trigger type */
pub const LEDS_TRIG_TYPE_EDGE: i32 = 0;
pub const LEDS_TRIG_TYPE_LEVEL: i32 = 1;

/* Boost modes */
pub const LEDS_BOOST_OFF: i32 = 0;
pub const LEDS_BOOST_ADAPTIVE: i32 = 1;
pub const LEDS_BOOST_FIXED: i32 = 2;

/* Standard LED colors */
pub const LED_COLOR_ID_WHITE: i32 = 0;
pub const LED_COLOR_ID_RED: i32 = 1;
pub const LED_COLOR_ID_GREEN: i32 = 2;
pub const LED_COLOR_ID_BLUE: i32 = 3;
pub const LED_COLOR_ID_AMBER: i32 = 4;
pub const LED_COLOR_ID_VIOLET: i32 = 5;
pub const LED_COLOR_ID_YELLOW: i32 = 6;
pub const LED_COLOR_ID_IR: i32 = 7;
pub const LED_COLOR_ID_MULTI: i32 = 8; /* For multicolor LEDs */
pub const LED_COLOR_ID_RGB: i32 = 9; /* For multicolor LEDs that can do arbitrary color,
                                        so this would include RGBW and similar */
pub const LED_COLOR_ID_PURPLE: i32 = 10;
pub const LED_COLOR_ID_ORANGE: i32 = 11;
pub const LED_COLOR_ID_PINK: i32 = 12;
pub const LED_COLOR_ID_CYAN: i32 = 13;
pub const LED_COLOR_ID_LIME: i32 = 14;
pub const LED_COLOR_ID_MAX: i32 = 15;

/* Standard LED functions */
/* Keyboard LEDs, usually it would be input4::capslock etc. */
/*   Obsolete equivalent: "shift-key-light" */
pub const LED_FUNCTION_CAPSLOCK: &str = "capslock";
pub const LED_FUNCTION_SCROLLLOCK: &str = "scrolllock";
pub const LED_FUNCTION_NUMLOCK: &str = "numlock";
pub const LED_FUNCTION_FNLOCK: &str = "fnlock";
/*   Obsolete equivalents: "tpacpi::thinklight" (IBM/Lenovo Thinkpads),
     "lp5523:kb{1,2,3,4,5,6}" (Nokia N900) */
pub const LED_FUNCTION_KBD_BACKLIGHT: &str = "kbd_backlight";

/* System LEDs, usually found on system body.
   platform::mute (etc) is sometimes seen, :mute would be better */
pub const LED_FUNCTION_POWER: &str = "power";
pub const LED_FUNCTION_DISK: &str = "disk";

/*   Obsolete: "platform:*:charging" (allwinner sun50i) */
pub const LED_FUNCTION_CHARGING: &str = "charging";
/*   Used RGB notification LEDs common on phones.
     Obsolete equivalents: "status-led:{red,green,blue}" (Motorola Droid 4),
     "lp5523:{r,g,b}" (Nokia N900) */
pub const LED_FUNCTION_STATUS: &str = "status";

pub const LED_FUNCTION_MICMUTE: &str = "micmute";
pub const LED_FUNCTION_MUTE: &str = "mute";

/* Used for player LEDs as found on game controllers from e.g. Nintendo, Sony. */
pub const LED_FUNCTION_PLAYER1: &str = "player-1";
pub const LED_FUNCTION_PLAYER2: &str = "player-2";
pub const LED_FUNCTION_PLAYER3: &str = "player-3";
pub const LED_FUNCTION_PLAYER4: &str = "player-4";
pub const LED_FUNCTION_PLAYER5: &str = "player-5";

/* Miscelleaus functions. Use functions above if you can. */
pub const LED_FUNCTION_ACTIVITY: &str = "activity";
pub const LED_FUNCTION_ALARM: &str = "alarm";
pub const LED_FUNCTION_BACKLIGHT: &str = "backlight";
pub const LED_FUNCTION_BLUETOOTH: &str = "bluetooth";
pub const LED_FUNCTION_BOOT: &str = "boot";
pub const LED_FUNCTION_CPU: &str = "cpu";
pub const LED_FUNCTION_DEBUG: &str = "debug";
pub const LED_FUNCTION_DISK_ACTIVITY: &str = "disk-activity";
pub const LED_FUNCTION_DISK_ERR: &str = "disk-err";
pub const LED_FUNCTION_DISK_READ: &str = "disk-read";
pub const LED_FUNCTION_DISK_WRITE: &str = "disk-write";
pub const LED_FUNCTION_FAULT: &str = "fault";
pub const LED_FUNCTION_FLASH: &str = "flash";
pub const LED_FUNCTION_HEARTBEAT: &str = "heartbeat";
pub const LED_FUNCTION_INDICATOR: &str = "indicator";
pub const LED_FUNCTION_LAN: &str = "lan";
pub const LED_FUNCTION_MAIL: &str = "mail";
pub const LED_FUNCTION_MOBILE: &str = "mobile";
pub const LED_FUNCTION_MTD: &str = "mtd";
pub const LED_FUNCTION_PANIC: &str = "panic";
pub const LED_FUNCTION_PROGRAMMING: &str = "programming";
pub const LED_FUNCTION_RX: &str = "rx";
pub const LED_FUNCTION_SD: &str = "sd";
pub const LED_FUNCTION_SPEED_LAN: &str = "speed-lan";
pub const LED_FUNCTION_SPEED_WAN: &str = "speed-wan";
pub const LED_FUNCTION_STANDBY: &str = "standby";
pub const LED_FUNCTION_TORCH: &str = "torch";
pub const LED_FUNCTION_TX: &str = "tx";
pub const LED_FUNCTION_USB: &str = "usb";
pub const LED_FUNCTION_WAN: &str = "wan";
pub const LED_FUNCTION_WAN_ONLINE: &str = "wan-online";
pub const LED_FUNCTION_WLAN: &str = "wlan";
pub const LED_FUNCTION_WLAN_2GHZ: &str = "wlan-2ghz";
pub const LED_FUNCTION_WLAN_5GHZ: &str = "wlan-5ghz";
pub const LED_FUNCTION_WLAN_6GHZ: &str = "wlan-6ghz";
pub const LED_FUNCTION_WPS: &str = "wps";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
