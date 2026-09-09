/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Core interface for WM831x; translated from the C header. */

// External Linux types and functions are supplied by other translated units.

/* Register values. */
pub const WM831X_RESET_ID: u16 = 0x00;
pub const WM831X_REVISION: u16 = 0x01;
pub const WM831X_PARENT_ID: u16 = 0x4000;
pub const WM831X_SYSVDD_CONTROL: u16 = 0x4001;
pub const WM831X_THERMAL_MONITORING: u16 = 0x4002;
pub const WM831X_POWER_STATE: u16 = 0x4003;
pub const WM831X_WATCHDOG: u16 = 0x4004;
pub const WM831X_ON_PIN_CONTROL: u16 = 0x4005;
pub const WM831X_RESET_CONTROL: u16 = 0x4006;
pub const WM831X_CONTROL_INTERFACE: u16 = 0x4007;
pub const WM831X_SECURITY_KEY: u16 = 0x4008;
pub const WM831X_SOFTWARE_SCRATCH: u16 = 0x4009;
pub const WM831X_OTP_CONTROL: u16 = 0x400A;
pub const WM831X_GPIO_LEVEL: u16 = 0x400C;
pub const WM831X_SYSTEM_STATUS: u16 = 0x400D;
pub const WM831X_ON_SOURCE: u16 = 0x400E;
pub const WM831X_OFF_SOURCE: u16 = 0x400F;
pub const WM831X_SYSTEM_INTERRUPTS: u16 = 0x4010;
pub const WM831X_INTERRUPT_STATUS_1: u16 = 0x4011;
pub const WM831X_INTERRUPT_STATUS_2: u16 = 0x4012;
pub const WM831X_INTERRUPT_STATUS_3: u16 = 0x4013;
pub const WM831X_INTERRUPT_STATUS_4: u16 = 0x4014;
pub const WM831X_INTERRUPT_STATUS_5: u16 = 0x4015;
pub const WM831X_IRQ_CONFIG: u16 = 0x4017;
pub const WM831X_SYSTEM_INTERRUPTS_MASK: u16 = 0x4018;
pub const WM831X_INTERRUPT_STATUS_1_MASK: u16 = 0x4019;
pub const WM831X_INTERRUPT_STATUS_2_MASK: u16 = 0x401A;
pub const WM831X_INTERRUPT_STATUS_3_MASK: u16 = 0x401B;
pub const WM831X_INTERRUPT_STATUS_4_MASK: u16 = 0x401C;
pub const WM831X_INTERRUPT_STATUS_5_MASK: u16 = 0x401D;
pub const WM831X_RTC_WRITE_COUNTER: u16 = 0x4020;
pub const WM831X_RTC_TIME_1: u16 = 0x4021;
pub const WM831X_RTC_TIME_2: u16 = 0x4022;
pub const WM831X_RTC_ALARM_1: u16 = 0x4023;
pub const WM831X_RTC_ALARM_2: u16 = 0x4024;
pub const WM831X_RTC_CONTROL: u16 = 0x4025;
pub const WM831X_RTC_TRIM: u16 = 0x4026;
pub const WM831X_TOUCH_CONTROL_1: u16 = 0x4028;
pub const WM831X_TOUCH_CONTROL_2: u16 = 0x4029;
pub const WM831X_TOUCH_DATA_X: u16 = 0x402A;
pub const WM831X_TOUCH_DATA_Y: u16 = 0x402B;
pub const WM831X_TOUCH_DATA_Z: u16 = 0x402C;
pub const WM831X_AUXADC_DATA: u16 = 0x402D;
pub const WM831X_AUXADC_CONTROL: u16 = 0x402E;
pub const WM831X_AUXADC_SOURCE: u16 = 0x402F;
pub const WM831X_COMPARATOR_CONTROL: u16 = 0x4030;
pub const WM831X_COMPARATOR_1: u16 = 0x4031;
pub const WM831X_COMPARATOR_2: u16 = 0x4032;
pub const WM831X_COMPARATOR_3: u16 = 0x4033;
pub const WM831X_COMPARATOR_4: u16 = 0x4034;
pub const WM831X_GPIO1_CONTROL: u16 = 0x4038;
pub const WM831X_GPIO2_CONTROL: u16 = 0x4039;
pub const WM831X_GPIO3_CONTROL: u16 = 0x403A;
pub const WM831X_GPIO4_CONTROL: u16 = 0x403B;
pub const WM831X_GPIO5_CONTROL: u16 = 0x403C;
pub const WM831X_GPIO6_CONTROL: u16 = 0x403D;
pub const WM831X_GPIO7_CONTROL: u16 = 0x403E;
pub const WM831X_GPIO8_CONTROL: u16 = 0x403F;
pub const WM831X_GPIO9_CONTROL: u16 = 0x4040;
pub const WM831X_GPIO10_CONTROL: u16 = 0x4041;
pub const WM831X_GPIO11_CONTROL: u16 = 0x4042;
pub const WM831X_GPIO12_CONTROL: u16 = 0x4043;
pub const WM831X_GPIO13_CONTROL: u16 = 0x4044;
pub const WM831X_GPIO14_CONTROL: u16 = 0x4045;
pub const WM831X_GPIO15_CONTROL: u16 = 0x4046;
pub const WM831X_GPIO16_CONTROL: u16 = 0x4047;
pub const WM831X_CHARGER_CONTROL_1: u16 = 0x4048;
pub const WM831X_CHARGER_CONTROL_2: u16 = 0x4049;
pub const WM831X_CHARGER_STATUS: u16 = 0x404A;
pub const WM831X_BACKUP_CHARGER_CONTROL: u16 = 0x404B;
pub const WM831X_STATUS_LED_1: u16 = 0x404C;
pub const WM831X_STATUS_LED_2: u16 = 0x404D;
pub const WM831X_CURRENT_SINK_1: u16 = 0x404E;
pub const WM831X_CURRENT_SINK_2: u16 = 0x404F;
pub const WM831X_DCDC_ENABLE: u16 = 0x4050;
pub const WM831X_LDO_ENABLE: u16 = 0x4051;
pub const WM831X_DCDC_STATUS: u16 = 0x4052;
pub const WM831X_LDO_STATUS: u16 = 0x4053;
pub const WM831X_DCDC_UV_STATUS: u16 = 0x4054;
pub const WM831X_LDO_UV_STATUS: u16 = 0x4055;
pub const WM831X_DC1_CONTROL_1: u16 = 0x4056;
pub const WM831X_DC1_CONTROL_2: u16 = 0x4057;
pub const WM831X_DC1_ON_CONFIG: u16 = 0x4058;
pub const WM831X_DC1_SLEEP_CONTROL: u16 = 0x4059;
pub const WM831X_DC1_DVS_CONTROL: u16 = 0x405A;
pub const WM831X_DC2_CONTROL_1: u16 = 0x405B;
pub const WM831X_DC2_CONTROL_2: u16 = 0x405C;
pub const WM831X_DC2_ON_CONFIG: u16 = 0x405D;
pub const WM831X_DC2_SLEEP_CONTROL: u16 = 0x405E;
pub const WM831X_DC2_DVS_CONTROL: u16 = 0x405F;
pub const WM831X_DC3_CONTROL_1: u16 = 0x4060;
pub const WM831X_DC3_CONTROL_2: u16 = 0x4061;
pub const WM831X_DC3_ON_CONFIG: u16 = 0x4062;
pub const WM831X_DC3_SLEEP_CONTROL: u16 = 0x4063;
pub const WM831X_DC4_CONTROL: u16 = 0x4064;
pub const WM831X_DC4_SLEEP_CONTROL: u16 = 0x4065;
pub const WM832X_DC4_SLEEP_CONTROL: u16 = 0x4067;
pub const WM831X_EPE1_CONTROL: u16 = 0x4066;
pub const WM831X_EPE2_CONTROL: u16 = 0x4067;
pub const WM831X_LDO1_CONTROL: u16 = 0x4068;
pub const WM831X_LDO1_ON_CONTROL: u16 = 0x4069;
pub const WM831X_LDO1_SLEEP_CONTROL: u16 = 0x406A;
pub const WM831X_LDO2_CONTROL: u16 = 0x406B;
pub const WM831X_LDO2_ON_CONTROL: u16 = 0x406C;
pub const WM831X_LDO2_SLEEP_CONTROL: u16 = 0x406D;
pub const WM831X_LDO3_CONTROL: u16 = 0x406E;
pub const WM831X_LDO3_ON_CONTROL: u16 = 0x406F;
pub const WM831X_LDO3_SLEEP_CONTROL: u16 = 0x4070;
pub const WM831X_LDO4_CONTROL: u16 = 0x4071;
pub const WM831X_LDO4_ON_CONTROL: u16 = 0x4072;
pub const WM831X_LDO4_SLEEP_CONTROL: u16 = 0x4073;
pub const WM831X_LDO5_CONTROL: u16 = 0x4074;
pub const WM831X_LDO5_ON_CONTROL: u16 = 0x4075;
pub const WM831X_LDO5_SLEEP_CONTROL: u16 = 0x4076;
pub const WM831X_LDO6_CONTROL: u16 = 0x4077;
pub const WM831X_LDO6_ON_CONTROL: u16 = 0x4078;
pub const WM831X_LDO6_SLEEP_CONTROL: u16 = 0x4079;
pub const WM831X_LDO7_CONTROL: u16 = 0x407A;
pub const WM831X_LDO7_ON_CONTROL: u16 = 0x407B;
pub const WM831X_LDO7_SLEEP_CONTROL: u16 = 0x407C;
pub const WM831X_LDO8_CONTROL: u16 = 0x407D;
pub const WM831X_LDO8_ON_CONTROL: u16 = 0x407E;
pub const WM831X_LDO8_SLEEP_CONTROL: u16 = 0x407F;
pub const WM831X_LDO9_CONTROL: u16 = 0x4080;
pub const WM831X_LDO9_ON_CONTROL: u16 = 0x4081;
pub const WM831X_LDO9_SLEEP_CONTROL: u16 = 0x4082;
pub const WM831X_LDO10_CONTROL: u16 = 0x4083;
pub const WM831X_LDO10_ON_CONTROL: u16 = 0x4084;
pub const WM831X_LDO10_SLEEP_CONTROL: u16 = 0x4085;
pub const WM831X_LDO11_ON_CONTROL: u16 = 0x4087;
pub const WM831X_LDO11_SLEEP_CONTROL: u16 = 0x4088;
pub const WM831X_POWER_GOOD_SOURCE_1: u16 = 0x408E;
pub const WM831X_POWER_GOOD_SOURCE_2: u16 = 0x408F;
pub const WM831X_CLOCK_CONTROL_1: u16 = 0x4090;
pub const WM831X_CLOCK_CONTROL_2: u16 = 0x4091;
pub const WM831X_FLL_CONTROL_1: u16 = 0x4092;
pub const WM831X_FLL_CONTROL_2: u16 = 0x4093;
pub const WM831X_FLL_CONTROL_3: u16 = 0x4094;
pub const WM831X_FLL_CONTROL_4: u16 = 0x4095;
pub const WM831X_FLL_CONTROL_5: u16 = 0x4096;
pub const WM831X_UNIQUE_ID_1: u16 = 0x7800;
pub const WM831X_UNIQUE_ID_2: u16 = 0x7801;
pub const WM831X_UNIQUE_ID_3: u16 = 0x7802;
pub const WM831X_UNIQUE_ID_4: u16 = 0x7803;
pub const WM831X_UNIQUE_ID_5: u16 = 0x7804;
pub const WM831X_UNIQUE_ID_6: u16 = 0x7805;
pub const WM831X_UNIQUE_ID_7: u16 = 0x7806;
pub const WM831X_UNIQUE_ID_8: u16 = 0x7807;
pub const WM831X_FACTORY_OTP_ID: u16 = 0x7808;
pub const WM831X_FACTORY_OTP_1: u16 = 0x7809;
pub const WM831X_FACTORY_OTP_2: u16 = 0x780A;
pub const WM831X_FACTORY_OTP_3: u16 = 0x780B;
pub const WM831X_FACTORY_OTP_4: u16 = 0x780C;
pub const WM831X_FACTORY_OTP_5: u16 = 0x780D;
pub const WM831X_CUSTOMER_OTP_ID: u16 = 0x7810;
pub const WM831X_DC1_OTP_CONTROL: u16 = 0x7811;
pub const WM831X_DC2_OTP_CONTROL: u16 = 0x7812;
pub const WM831X_DC3_OTP_CONTROL: u16 = 0x7813;
pub const WM831X_LDO1_2_OTP_CONTROL: u16 = 0x7814;
pub const WM831X_LDO3_4_OTP_CONTROL: u16 = 0x7815;
pub const WM831X_LDO5_6_OTP_CONTROL: u16 = 0x7816;
pub const WM831X_LDO7_8_OTP_CONTROL: u16 = 0x7817;
pub const WM831X_LDO9_10_OTP_CONTROL: u16 = 0x7818;
pub const WM831X_LDO11_EPE_CONTROL: u16 = 0x7819;
pub const WM831X_GPIO1_OTP_CONTROL: u16 = 0x781A;
pub const WM831X_GPIO2_OTP_CONTROL: u16 = 0x781B;
pub const WM831X_GPIO3_OTP_CONTROL: u16 = 0x781C;
pub const WM831X_GPIO4_OTP_CONTROL: u16 = 0x781D;
pub const WM831X_GPIO5_OTP_CONTROL: u16 = 0x781E;
pub const WM831X_GPIO6_OTP_CONTROL: u16 = 0x781F;
pub const WM831X_DBE_CHECK_DATA: u16 = 0x7827;

/* Field masks, shifts, and widths. */
pub const WM831X_CHIP_ID_MASK: u16 = 0xFFFF;
pub const WM831X_CHIP_ID_SHIFT: u16 = 0;
pub const WM831X_CHIP_ID_WIDTH: u16 = 16;
pub const WM831X_PARENT_REV_MASK: u16 = 0xFF00;
pub const WM831X_PARENT_REV_SHIFT: u16 = 8;
pub const WM831X_PARENT_REV_WIDTH: u16 = 8;
pub const WM831X_CHILD_REV_MASK: u16 = 0x00FF;
pub const WM831X_CHILD_REV_SHIFT: u16 = 0;
pub const WM831X_CHILD_REV_WIDTH: u16 = 8;
pub const WM831X_PARENT_ID_MASK: u16 = 0xFFFF;
pub const WM831X_PARENT_ID_SHIFT: u16 = 0;
pub const WM831X_PARENT_ID_WIDTH: u16 = 16;
pub const WM831X_ON_PIN_SECACT_MASK: u16 = 0x0300;
pub const WM831X_ON_PIN_SECACT_SHIFT: u16 = 8;
pub const WM831X_ON_PIN_SECACT_WIDTH: u16 = 2;
pub const WM831X_ON_PIN_PRIMACT_MASK: u16 = 0x0030;
pub const WM831X_ON_PIN_PRIMACT_SHIFT: u16 = 4;
pub const WM831X_ON_PIN_PRIMACT_WIDTH: u16 = 2;
pub const WM831X_ON_PIN_STS: u16 = 0x0008;
pub const WM831X_ON_PIN_STS_MASK: u16 = 0x0008;
pub const WM831X_ON_PIN_STS_SHIFT: u16 = 3;
pub const WM831X_ON_PIN_STS_WIDTH: u16 = 1;
pub const WM831X_ON_PIN_TO_MASK: u16 = 0x0003;
pub const WM831X_ON_PIN_TO_SHIFT: u16 = 0;
pub const WM831X_ON_PIN_TO_WIDTH: u16 = 2;

pub const WM831X_CLKOUT_ENA: u16 = 0x8000;
pub const WM831X_CLKOUT_ENA_MASK: u16 = 0x8000;
pub const WM831X_CLKOUT_ENA_SHIFT: u16 = 15;
pub const WM831X_CLKOUT_ENA_WIDTH: u16 = 1;
pub const WM831X_CLKOUT_OD: u16 = 0x2000;
pub const WM831X_CLKOUT_OD_MASK: u16 = 0x2000;
pub const WM831X_CLKOUT_OD_SHIFT: u16 = 13;
pub const WM831X_CLKOUT_OD_WIDTH: u16 = 1;
pub const WM831X_CLKOUT_SLOT_MASK: u16 = 0x0700;
pub const WM831X_CLKOUT_SLOT_SHIFT: u16 = 8;
pub const WM831X_CLKOUT_SLOT_WIDTH: u16 = 3;
pub const WM831X_CLKOUT_SLPSLOT_MASK: u16 = 0x0070;
pub const WM831X_CLKOUT_SLPSLOT_SHIFT: u16 = 4;
pub const WM831X_CLKOUT_SLPSLOT_WIDTH: u16 = 3;
pub const WM831X_CLKOUT_SRC: u16 = 0x0001;
pub const WM831X_CLKOUT_SRC_MASK: u16 = 0x0001;
pub const WM831X_CLKOUT_SRC_SHIFT: u16 = 0;
pub const WM831X_CLKOUT_SRC_WIDTH: u16 = 1;
pub const WM831X_XTAL_INH: u16 = 0x8000;
pub const WM831X_XTAL_INH_MASK: u16 = 0x8000;
pub const WM831X_XTAL_INH_SHIFT: u16 = 15;
pub const WM831X_XTAL_INH_WIDTH: u16 = 1;
pub const WM831X_XTAL_ENA: u16 = 0x2000;
pub const WM831X_XTAL_ENA_MASK: u16 = 0x2000;
pub const WM831X_XTAL_ENA_SHIFT: u16 = 13;
pub const WM831X_XTAL_ENA_WIDTH: u16 = 1;
pub const WM831X_XTAL_BKUPENA: u16 = 0x1000;
pub const WM831X_XTAL_BKUPENA_MASK: u16 = 0x1000;
pub const WM831X_XTAL_BKUPENA_SHIFT: u16 = 12;
pub const WM831X_XTAL_BKUPENA_WIDTH: u16 = 1;
pub const WM831X_FLL_AUTO: u16 = 0x0080;
pub const WM831X_FLL_AUTO_MASK: u16 = 0x0080;
pub const WM831X_FLL_AUTO_SHIFT: u16 = 7;
pub const WM831X_FLL_AUTO_WIDTH: u16 = 1;
pub const WM831X_FLL_AUTO_FREQ_MASK: u16 = 0x0007;
pub const WM831X_FLL_AUTO_FREQ_SHIFT: u16 = 0;
pub const WM831X_FLL_AUTO_FREQ_WIDTH: u16 = 3;
pub const WM831X_FLL_FRAC: u16 = 0x0004;
pub const WM831X_FLL_FRAC_MASK: u16 = 0x0004;
pub const WM831X_FLL_FRAC_SHIFT: u16 = 2;
pub const WM831X_FLL_FRAC_WIDTH: u16 = 1;
pub const WM831X_FLL_OSC_ENA: u16 = 0x0002;
pub const WM831X_FLL_OSC_ENA_MASK: u16 = 0x0002;
pub const WM831X_FLL_OSC_ENA_SHIFT: u16 = 1;
pub const WM831X_FLL_OSC_ENA_WIDTH: u16 = 1;
pub const WM831X_FLL_ENA: u16 = 0x0001;
pub const WM831X_FLL_ENA_MASK: u16 = 0x0001;
pub const WM831X_FLL_ENA_SHIFT: u16 = 0;
pub const WM831X_FLL_ENA_WIDTH: u16 = 1;
pub const WM831X_FLL_OUTDIV_MASK: u16 = 0x3F00;
pub const WM831X_FLL_OUTDIV_SHIFT: u16 = 8;
pub const WM831X_FLL_OUTDIV_WIDTH: u16 = 6;
pub const WM831X_FLL_CTRL_RATE_MASK: u16 = 0x0070;
pub const WM831X_FLL_CTRL_RATE_SHIFT: u16 = 4;
pub const WM831X_FLL_CTRL_RATE_WIDTH: u16 = 3;
pub const WM831X_FLL_FRATIO_MASK: u16 = 0x0007;
pub const WM831X_FLL_FRATIO_SHIFT: u16 = 0;
pub const WM831X_FLL_FRATIO_WIDTH: u16 = 3;
pub const WM831X_FLL_K_MASK: u16 = 0xFFFF;
pub const WM831X_FLL_K_SHIFT: u16 = 0;
pub const WM831X_FLL_K_WIDTH: u16 = 16;
pub const WM831X_FLL_N_MASK: u16 = 0x7FE0;
pub const WM831X_FLL_N_SHIFT: u16 = 5;
pub const WM831X_FLL_N_WIDTH: u16 = 10;
pub const WM831X_FLL_GAIN_MASK: u16 = 0x000F;
pub const WM831X_FLL_GAIN_SHIFT: u16 = 0;
pub const WM831X_FLL_GAIN_WIDTH: u16 = 4;
pub const WM831X_FLL_CLK_REF_DIV_MASK: u16 = 0x0018;
pub const WM831X_FLL_CLK_REF_DIV_SHIFT: u16 = 3;
pub const WM831X_FLL_CLK_REF_DIV_WIDTH: u16 = 2;
pub const WM831X_FLL_CLK_SRC_MASK: u16 = 0x0003;
pub const WM831X_FLL_CLK_SRC_SHIFT: u16 = 0;
pub const WM831X_FLL_CLK_SRC_WIDTH: u16 = 2;

pub const WM831X_NUM_IRQ_REGS: usize = 5;
pub const WM831X_NUM_GPIO_REGS: usize = 16;

#[repr(u16)]
pub enum Wm831xParent { WM8310 = 0x8310, WM8311 = 0x8311, WM8312 = 0x8312,
    WM8320 = 0x8320, WM8321 = 0x8321, WM8325 = 0x8325, WM8326 = 0x8326 }

pub type Wm831xAuxadcReadFn = unsafe extern "C" fn(*mut Wm831x, Wm831xAuxadc) -> i32;
pub type Wm831xAuxadc = u32;

#[repr(C)]
pub struct Wm831x {
    pub io_lock: Mutex, pub dev: *mut Device, pub regmap: *mut Regmap,
    pub pdata: Wm831xPdata, pub type_: Wm831xParent, pub irq: i32,
    pub irq_lock: Mutex, pub irq_domain: *mut IrqDomain,
    pub irq_masks_cur: [i32; WM831X_NUM_IRQ_REGS],
    pub irq_masks_cache: [i32; WM831X_NUM_IRQ_REGS], pub soft_shutdown: bool,
    pub has_gpio_ena: u32, pub has_cs_sts: u32, pub charger_irq_wake: u32,
    pub num_gpio: i32, pub gpio_update: [i32; WM831X_NUM_GPIO_REGS],
    pub gpio_level_high: [bool; WM831X_NUM_GPIO_REGS],
    pub gpio_level_low: [bool; WM831X_NUM_GPIO_REGS], pub auxadc_lock: Mutex,
    pub auxadc_pending: ListHead, pub auxadc_active: u16,
    pub auxadc_read: Option<Wm831xAuxadcReadFn>, pub key_lock: Mutex,
    pub locked: u32,
}

extern "C" {
    pub fn wm831x_reg_read(wm831x: *mut Wm831x, reg: u16) -> i32;
    pub fn wm831x_reg_write(wm831x: *mut Wm831x, reg: u16, val: u16) -> i32;
    pub fn wm831x_reg_lock(wm831x: *mut Wm831x);
    pub fn wm831x_reg_unlock(wm831x: *mut Wm831x) -> i32;
    pub fn wm831x_set_bits(wm831x: *mut Wm831x, reg: u16, mask: u16, val: u16) -> i32;
    pub fn wm831x_bulk_read(wm831x: *mut Wm831x, reg: u16, count: i32, buf: *mut u16) -> i32;
    pub fn wm831x_device_init(wm831x: *mut Wm831x, irq: i32) -> i32;
    pub fn wm831x_device_suspend(wm831x: *mut Wm831x) -> i32;
    pub fn wm831x_device_shutdown(wm831x: *mut Wm831x);
    pub fn wm831x_irq_init(wm831x: *mut Wm831x, irq: i32) -> i32;
    pub fn wm831x_irq_exit(wm831x: *mut Wm831x);
    pub fn wm831x_auxadc_init(wm831x: *mut Wm831x);
}

extern "C" { pub static mut wm831x_regmap_config: RegmapConfig; pub static wm831x_of_match: OfDeviceId; }

// Mutex, Device, Regmap, Wm831xPdata, IrqDomain, ListHead, RegmapConfig,
// and OfDeviceId are supplied by the corresponding translated Linux headers.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
