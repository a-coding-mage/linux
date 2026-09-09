/* SPDX-License-Identifier: GPL-2.0 */

/* Unified sub device IDs for DA9030/DA9034/DA9035 */
pub const DA9030_ID_LED_1: i32 = 0;
pub const DA9030_ID_LED_2: i32 = 1;
pub const DA9030_ID_LED_3: i32 = 2;
pub const DA9030_ID_LED_4: i32 = 3;
pub const DA9030_ID_LED_PC: i32 = 4;
pub const DA9030_ID_VIBRA: i32 = 5;
pub const DA9030_ID_WLED: i32 = 6;
pub const DA9030_ID_BUCK1: i32 = 7;
pub const DA9030_ID_BUCK2: i32 = 8;
pub const DA9030_ID_LDO1: i32 = 9;
pub const DA9030_ID_LDO2: i32 = 10;
pub const DA9030_ID_LDO3: i32 = 11;
pub const DA9030_ID_LDO4: i32 = 12;
pub const DA9030_ID_LDO5: i32 = 13;
pub const DA9030_ID_LDO6: i32 = 14;
pub const DA9030_ID_LDO7: i32 = 15;
pub const DA9030_ID_LDO8: i32 = 16;
pub const DA9030_ID_LDO9: i32 = 17;
pub const DA9030_ID_LDO10: i32 = 18;
pub const DA9030_ID_LDO11: i32 = 19;
pub const DA9030_ID_LDO12: i32 = 20;
pub const DA9030_ID_LDO13: i32 = 21;
pub const DA9030_ID_LDO14: i32 = 22;
pub const DA9030_ID_LDO15: i32 = 23;
pub const DA9030_ID_LDO16: i32 = 24;
pub const DA9030_ID_LDO17: i32 = 25;
pub const DA9030_ID_LDO18: i32 = 26;
pub const DA9030_ID_LDO19: i32 = 27;
pub const DA9030_ID_LDO_INT: i32 = 28;
pub const DA9030_ID_BAT: i32 = 29;
pub const DA9034_ID_LED_1: i32 = 30;
pub const DA9034_ID_LED_2: i32 = 31;
pub const DA9034_ID_VIBRA: i32 = 32;
pub const DA9034_ID_WLED: i32 = 33;
pub const DA9034_ID_TOUCH: i32 = 34;
pub const DA9034_ID_BUCK1: i32 = 35;
pub const DA9034_ID_BUCK2: i32 = 36;
pub const DA9034_ID_LDO1: i32 = 37;
pub const DA9034_ID_LDO2: i32 = 38;
pub const DA9034_ID_LDO3: i32 = 39;
pub const DA9034_ID_LDO4: i32 = 40;
pub const DA9034_ID_LDO5: i32 = 41;
pub const DA9034_ID_LDO6: i32 = 42;
pub const DA9034_ID_LDO7: i32 = 43;
pub const DA9034_ID_LDO8: i32 = 44;
pub const DA9034_ID_LDO9: i32 = 45;
pub const DA9034_ID_LDO10: i32 = 46;
pub const DA9034_ID_LDO11: i32 = 47;
pub const DA9034_ID_LDO12: i32 = 48;
pub const DA9034_ID_LDO13: i32 = 49;
pub const DA9034_ID_LDO14: i32 = 50;
pub const DA9034_ID_LDO15: i32 = 51;
pub const DA9035_ID_BUCK3: i32 = 52;

pub const DA9030_LED_RATE_ON: u32 = 0 << 5;
pub const DA9030_LED_RATE_052S: u32 = 1 << 5;
pub const DA9030_LED_DUTY_1_16: u32 = 0 << 3;
pub const DA9030_LED_DUTY_1_8: u32 = 1 << 3;
pub const DA9030_LED_DUTY_1_4: u32 = 2 << 3;
pub const DA9030_LED_DUTY_1_2: u32 = 3 << 3;
pub const DA9030_VIBRA_MODE_1P3V: u32 = 0 << 1;
pub const DA9030_VIBRA_MODE_2P7V: u32 = 1 << 1;
pub const DA9030_VIBRA_FREQ_1HZ: u32 = 0 << 2;
pub const DA9030_VIBRA_FREQ_2HZ: u32 = 1 << 2;
pub const DA9030_VIBRA_FREQ_4HZ: u32 = 2 << 2;
pub const DA9030_VIBRA_FREQ_8HZ: u32 = 3 << 2;
pub const DA9030_VIBRA_DUTY_ON: u32 = 0 << 4;
pub const DA9030_VIBRA_DUTY_75P: u32 = 1 << 4;
pub const DA9030_VIBRA_DUTY_50P: u32 = 2 << 4;
pub const DA9030_VIBRA_DUTY_25P: u32 = 3 << 4;
pub const DA9034_LED_RAMP: u32 = 1 << 7;

#[repr(C)]
pub struct da9034_touch_pdata { pub interval_ms: i32, pub x_inverted: i32, pub y_inverted: i32 }
#[repr(C)]
pub struct da9034_backlight_pdata { pub output_current: i32 }
pub struct power_supply_info;
#[repr(C)]
pub struct da9030_battery_info {
    pub battery_info: *mut power_supply_info,
    pub charge_milliamp: u32, pub charge_millivolt: u32,
    pub vbat_low: i32, pub vbat_crit: i32, pub vbat_charge_start: i32,
    pub vbat_charge_stop: i32, pub vbat_charge_restart: i32,
    pub vcharge_min: i32, pub vcharge_max: i32,
    pub tbat_low: i32, pub tbat_high: i32, pub tbat_restart: i32,
    pub batmon_interval: u32,
    pub battery_low: Option<unsafe extern "C" fn()>,
    pub battery_critical: Option<unsafe extern "C" fn()>,
}
#[repr(C)]
pub struct da903x_subdev_info { pub id: i32, pub name: *const i8, pub platform_data: *mut core::ffi::c_void }
#[repr(C)]
pub struct da903x_platform_data { pub num_subdevs: i32, pub subdevs: *mut da903x_subdev_info }

pub const DA9030_EVENT_ONKEY: u32 = 1 << 0; pub const DA9030_EVENT_PWREN: u32 = 1 << 1;
pub const DA9030_EVENT_EXTON: u32 = 1 << 2; pub const DA9030_EVENT_CHDET: u32 = 1 << 3;
pub const DA9030_EVENT_TBAT: u32 = 1 << 4; pub const DA9030_EVENT_VBATMON: u32 = 1 << 5;
pub const DA9030_EVENT_VBATMON_TXON: u32 = 1 << 6; pub const DA9030_EVENT_CHIOVER: u32 = 1 << 7;
pub const DA9030_EVENT_TCTO: u32 = 1 << 8; pub const DA9030_EVENT_CCTO: u32 = 1 << 9;
pub const DA9030_EVENT_ADC_READY: u32 = 1 << 10; pub const DA9030_EVENT_VBUS_4P4: u32 = 1 << 11;
pub const DA9030_EVENT_VBUS_4P0: u32 = 1 << 12; pub const DA9030_EVENT_SESS_VALID: u32 = 1 << 13;
pub const DA9030_EVENT_SRP_DETECT: u32 = 1 << 14; pub const DA9030_EVENT_WATCHDOG: u32 = 1 << 15;
pub const DA9030_EVENT_LDO15: u32 = 1 << 16; pub const DA9030_EVENT_LDO16: u32 = 1 << 17;
pub const DA9030_EVENT_LDO17: u32 = 1 << 18; pub const DA9030_EVENT_LDO18: u32 = 1 << 19;
pub const DA9030_EVENT_LDO19: u32 = 1 << 20; pub const DA9030_EVENT_BUCK2: u32 = 1 << 21;
pub const DA9034_EVENT_ONKEY: u32 = 1 << 0; pub const DA9034_EVENT_EXTON: u32 = 1 << 2;
pub const DA9034_EVENT_CHDET: u32 = 1 << 3; pub const DA9034_EVENT_TBAT: u32 = 1 << 4;
pub const DA9034_EVENT_VBATMON: u32 = 1 << 5; pub const DA9034_EVENT_REV_IOVER: u32 = 1 << 6;
pub const DA9034_EVENT_CH_IOVER: u32 = 1 << 7; pub const DA9034_EVENT_CH_TCTO: u32 = 1 << 8;
pub const DA9034_EVENT_CH_CCTO: u32 = 1 << 9; pub const DA9034_EVENT_USB_DEV: u32 = 1 << 10;
pub const DA9034_EVENT_OTGCP_IOVER: u32 = 1 << 11; pub const DA9034_EVENT_VBUS_4P55: u32 = 1 << 12;
pub const DA9034_EVENT_VBUS_3P8: u32 = 1 << 13; pub const DA9034_EVENT_SESS_1P8: u32 = 1 << 14;
pub const DA9034_EVENT_SRP_READY: u32 = 1 << 15; pub const DA9034_EVENT_ADC_MAN: u32 = 1 << 16;
pub const DA9034_EVENT_ADC_AUTO4: u32 = 1 << 17; pub const DA9034_EVENT_ADC_AUTO5: u32 = 1 << 18;
pub const DA9034_EVENT_ADC_AUTO6: u32 = 1 << 19; pub const DA9034_EVENT_PEN_DOWN: u32 = 1 << 20;
pub const DA9034_EVENT_TSI_READY: u32 = 1 << 21; pub const DA9034_EVENT_UART_TX: u32 = 1 << 22;
pub const DA9034_EVENT_UART_RX: u32 = 1 << 23; pub const DA9034_EVENT_HEADSET: u32 = 1 << 25;
pub const DA9034_EVENT_HOOKSWITCH: u32 = 1 << 26; pub const DA9034_EVENT_WATCHDOG: u32 = 1 << 27;

pub const DA9030_STATUS_ONKEY: u32 = 1 << 0; pub const DA9030_STATUS_PWREN1: u32 = 1 << 1;
pub const DA9030_STATUS_EXTON: u32 = 1 << 2; pub const DA9030_STATUS_CHDET: u32 = 1 << 3;
pub const DA9030_STATUS_TBAT: u32 = 1 << 4; pub const DA9030_STATUS_VBATMON: u32 = 1 << 5;
pub const DA9030_STATUS_VBATMON_TXON: u32 = 1 << 6; pub const DA9030_STATUS_MCLKDET: u32 = 1 << 7;
pub const DA9034_STATUS_ONKEY: u32 = 1 << 0; pub const DA9034_STATUS_EXTON: u32 = 1 << 2;
pub const DA9034_STATUS_CHDET: u32 = 1 << 3; pub const DA9034_STATUS_TBAT: u32 = 1 << 4;
pub const DA9034_STATUS_VBATMON: u32 = 1 << 5; pub const DA9034_STATUS_PEN_DOWN: u32 = 1 << 6;
pub const DA9034_STATUS_MCLKDET: u32 = 1 << 7; pub const DA9034_STATUS_USB_DEV: u32 = 1 << 8;
pub const DA9034_STATUS_HEADSET: u32 = 1 << 9; pub const DA9034_STATUS_HOOKSWITCH: u32 = 1 << 10;
pub const DA9034_STATUS_REMCON: u32 = 1 << 11; pub const DA9034_STATUS_VBUS_VALID_4P55: u32 = 1 << 12;
pub const DA9034_STATUS_VBUS_VALID_3P8: u32 = 1 << 13; pub const DA9034_STATUS_SESS_VALID_1P8: u32 = 1 << 14;
pub const DA9034_STATUS_SRP_READY: u32 = 1 << 15;

extern "C" {
    pub fn da903x_register_notifier(dev: *mut device, nb: *mut notifier_block, events: u32) -> i32;
    pub fn da903x_unregister_notifier(dev: *mut device, nb: *mut notifier_block, events: u32) -> i32;
    pub fn da903x_query_status(dev: *mut device, status: u32) -> i32;
    pub fn da903x_write(dev: *mut device, reg: i32, val: u8) -> i32;
    pub fn da903x_writes(dev: *mut device, reg: i32, len: i32, val: *mut u8) -> i32;
    pub fn da903x_read(dev: *mut device, reg: i32, val: *mut u8) -> i32;
    pub fn da903x_reads(dev: *mut device, reg: i32, len: i32, val: *mut u8) -> i32;
    pub fn da903x_update(dev: *mut device, reg: i32, val: u8, mask: u8) -> i32;
    pub fn da903x_set_bits(dev: *mut device, reg: i32, bit_mask: u8) -> i32;
    pub fn da903x_clr_bits(dev: *mut device, reg: i32, bit_mask: u8) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
