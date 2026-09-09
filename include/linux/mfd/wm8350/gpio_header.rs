/* SPDX-License-Identifier: GPL-2.0-or-later */
/* gpio.h -- GPIO Driver for Wolfson WM8350 PMIC */
// Dependency: linux/platform_device.h (provides `platform_device`).

pub const WM8350_GPIO_DEBOUNCE: u16 = 0x80;
pub const WM8350_GPIO_PIN_PULL_UP_CONTROL: u16 = 0x81;
pub const WM8350_GPIO_PULL_DOWN_CONTROL: u16 = 0x82;
pub const WM8350_GPIO_INT_MODE: u16 = 0x83;
pub const WM8350_GPIO_CONTROL: u16 = 0x85;
pub const WM8350_GPIO_CONFIGURATION_I_O: u16 = 0x86;
pub const WM8350_GPIO_PIN_POLARITY_TYPE: u16 = 0x87;
pub const WM8350_GPIO_FUNCTION_SELECT_1: u16 = 0x8C;
pub const WM8350_GPIO_FUNCTION_SELECT_2: u16 = 0x8D;
pub const WM8350_GPIO_FUNCTION_SELECT_3: u16 = 0x8E;
pub const WM8350_GPIO_FUNCTION_SELECT_4: u16 = 0x8F;
pub const WM8350_GPIO_LEVEL: u16 = 0xE6;

// GPIO functions.
pub const WM8350_GPIO0_GPIO_IN: u8 = 0x0; pub const WM8350_GPIO0_GPIO_OUT: u8 = 0x0;
pub const WM8350_GPIO0_PWR_ON_IN: u8 = 0x1; pub const WM8350_GPIO0_PWR_ON_OUT: u8 = 0x1;
pub const WM8350_GPIO0_LDO_EN_IN: u8 = 0x2; pub const WM8350_GPIO0_VRTC_OUT: u8 = 0x2;
pub const WM8350_GPIO0_LPWR1_IN: u8 = 0x3; pub const WM8350_GPIO0_POR_B_OUT: u8 = 0x3;
pub const WM8350_GPIO1_GPIO_IN: u8 = 0x0; pub const WM8350_GPIO1_GPIO_OUT: u8 = 0x0;
pub const WM8350_GPIO1_PWR_ON_IN: u8 = 0x1; pub const WM8350_GPIO1_DO_CONF_OUT: u8 = 0x1;
pub const WM8350_GPIO1_LDO_EN_IN: u8 = 0x2; pub const WM8350_GPIO1_RESET_OUT: u8 = 0x2;
pub const WM8350_GPIO1_LPWR2_IN: u8 = 0x3; pub const WM8350_GPIO1_MEMRST_OUT: u8 = 0x3;
pub const WM8350_GPIO2_GPIO_IN: u8 = 0x0; pub const WM8350_GPIO2_GPIO_OUT: u8 = 0x0;
pub const WM8350_GPIO2_PWR_ON_IN: u8 = 0x1; pub const WM8350_GPIO2_PWR_ON_OUT: u8 = 0x1;
pub const WM8350_GPIO2_WAKE_UP_IN: u8 = 0x2; pub const WM8350_GPIO2_VRTC_OUT: u8 = 0x2;
pub const WM8350_GPIO2_32KHZ_IN: u8 = 0x3; pub const WM8350_GPIO2_32KHZ_OUT: u8 = 0x3;
pub const WM8350_GPIO3_GPIO_IN: u8 = 0x0; pub const WM8350_GPIO3_GPIO_OUT: u8 = 0x0;
pub const WM8350_GPIO3_PWR_ON_IN: u8 = 0x1; pub const WM8350_GPIO3_P_CLK_OUT: u8 = 0x1;
pub const WM8350_GPIO3_LDO_EN_IN: u8 = 0x2; pub const WM8350_GPIO3_VRTC_OUT: u8 = 0x2;
pub const WM8350_GPIO3_PWR_OFF_IN: u8 = 0x3; pub const WM8350_GPIO3_32KHZ_OUT: u8 = 0x3;
pub const WM8350_GPIO4_GPIO_IN: u8 = 0x0; pub const WM8350_GPIO4_GPIO_OUT: u8 = 0x0;
pub const WM8350_GPIO4_MR_IN: u8 = 0x1; pub const WM8350_GPIO4_MEM_RST_OUT: u8 = 0x1;
pub const WM8350_GPIO4_FLASH_IN: u8 = 0x2; pub const WM8350_GPIO4_ADA_OUT: u8 = 0x2;
pub const WM8350_GPIO4_HIBERNATE_IN: u8 = 0x3; pub const WM8350_GPIO4_FLASH_OUT: u8 = 0x3;
pub const WM8350_GPIO4_MICDET_OUT: u8 = 0x4; pub const WM8350_GPIO4_MICSHT_OUT: u8 = 0x5;
pub const WM8350_GPIO5_GPIO_IN: u8 = 0x0; pub const WM8350_GPIO5_GPIO_OUT: u8 = 0x0;
pub const WM8350_GPIO5_LPWR1_IN: u8 = 0x1; pub const WM8350_GPIO5_P_CLK_OUT: u8 = 0x1;
pub const WM8350_GPIO5_ADCLRCLK_IN: u8 = 0x2; pub const WM8350_GPIO5_ADCLRCLK_OUT: u8 = 0x2;
pub const WM8350_GPIO5_HIBERNATE_IN: u8 = 0x3; pub const WM8350_GPIO5_32KHZ_OUT: u8 = 0x3;
pub const WM8350_GPIO5_MICDET_OUT: u8 = 0x4; pub const WM8350_GPIO5_MICSHT_OUT: u8 = 0x5;
pub const WM8350_GPIO5_ADA_OUT: u8 = 0x6; pub const WM8350_GPIO5_OPCLK_OUT: u8 = 0x7;
pub const WM8350_GPIO6_GPIO_IN: u8 = 0x0; pub const WM8350_GPIO6_GPIO_OUT: u8 = 0x0;
pub const WM8350_GPIO6_LPWR2_IN: u8 = 0x1; pub const WM8350_GPIO6_MEMRST_OUT: u8 = 0x1;
pub const WM8350_GPIO6_FLASH_IN: u8 = 0x2; pub const WM8350_GPIO6_ADA_OUT: u8 = 0x2;
pub const WM8350_GPIO6_HIBERNATE_IN: u8 = 0x3; pub const WM8350_GPIO6_RTC_OUT: u8 = 0x3;
pub const WM8350_GPIO6_MICDET_OUT: u8 = 0x4; pub const WM8350_GPIO6_MICSHT_OUT: u8 = 0x5;
pub const WM8350_GPIO6_ADCLRCLKB_OUT: u8 = 0x6; pub const WM8350_GPIO6_SDOUT_OUT: u8 = 0x7;
pub const WM8350_GPIO7_GPIO_IN: u8 = 0x0; pub const WM8350_GPIO7_GPIO_OUT: u8 = 0x0;
pub const WM8350_GPIO7_LPWR3_IN: u8 = 0x1; pub const WM8350_GPIO7_P_CLK_OUT: u8 = 0x1;
pub const WM8350_GPIO7_MASK_IN: u8 = 0x2; pub const WM8350_GPIO7_VCC_FAULT_OUT: u8 = 0x2;
pub const WM8350_GPIO7_HIBERNATE_IN: u8 = 0x3; pub const WM8350_GPIO7_BATT_FAULT_OUT: u8 = 0x3;
pub const WM8350_GPIO7_MICDET_OUT: u8 = 0x4; pub const WM8350_GPIO7_MICSHT_OUT: u8 = 0x5;
pub const WM8350_GPIO7_ADA_OUT: u8 = 0x6; pub const WM8350_GPIO7_CSB_IN: u8 = 0x7;
pub const WM8350_GPIO8_GPIO_IN: u8 = 0x0; pub const WM8350_GPIO8_GPIO_OUT: u8 = 0x0;
pub const WM8350_GPIO8_MR_IN: u8 = 0x1; pub const WM8350_GPIO8_VCC_FAULT_OUT: u8 = 0x1;
pub const WM8350_GPIO8_ADCBCLK_IN: u8 = 0x2; pub const WM8350_GPIO8_ADCBCLK_OUT: u8 = 0x2;
pub const WM8350_GPIO8_PWR_OFF_IN: u8 = 0x3; pub const WM8350_GPIO8_BATT_FAULT_OUT: u8 = 0x3;
pub const WM8350_GPIO8_ALTSCL_IN: u8 = 0xf;
pub const WM8350_GPIO9_GPIO_IN: u8 = 0x0; pub const WM8350_GPIO9_GPIO_OUT: u8 = 0x0;
pub const WM8350_GPIO9_HEARTBEAT_IN: u8 = 0x1; pub const WM8350_GPIO9_VCC_FAULT_OUT: u8 = 0x1;
pub const WM8350_GPIO9_MASK_IN: u8 = 0x2; pub const WM8350_GPIO9_LINE_GT_BATT_OUT: u8 = 0x2;
pub const WM8350_GPIO9_PWR_OFF_IN: u8 = 0x3; pub const WM8350_GPIO9_BATT_FAULT_OUT: u8 = 0x3;
pub const WM8350_GPIO9_ALTSDA_OUT: u8 = 0xf;
pub const WM8350_GPIO10_GPIO_IN: u8 = 0x0; pub const WM8350_GPIO10_GPIO_OUT: u8 = 0x0;
pub const WM8350_GPIO10_ISINKC_OUT: u8 = 0x1; pub const WM8350_GPIO10_PWR_OFF_IN: u8 = 0x2;
pub const WM8350_GPIO10_LINE_GT_BATT_OUT: u8 = 0x2; pub const WM8350_GPIO10_CHD_IND_IN: u8 = 0x3;
pub const WM8350_GPIO11_GPIO_IN: u8 = 0x0; pub const WM8350_GPIO11_GPIO_OUT: u8 = 0x0;
pub const WM8350_GPIO11_ISINKD_OUT: u8 = 0x1; pub const WM8350_GPIO11_WAKEUP_IN: u8 = 0x2;
pub const WM8350_GPIO11_LINE_GT_BATT_OUT: u8 = 0x2; pub const WM8350_GPIO11_CHD_IND_IN: u8 = 0x3;
pub const WM8350_GPIO12_GPIO_IN: u8 = 0x0; pub const WM8350_GPIO12_GPIO_OUT: u8 = 0x0;
pub const WM8350_GPIO12_ISINKE_OUT: u8 = 0x1; pub const WM8350_GPIO12_LINE_GT_BATT_OUT: u8 = 0x2;
pub const WM8350_GPIO12_LINE_EN_OUT: u8 = 0x3; pub const WM8350_GPIO12_32KHZ_OUT: u8 = 0x4;

pub const WM8350_GPIO_DIR_IN: i32 = 0;
pub const WM8350_GPIO_DIR_OUT: i32 = 1;
pub const WM8350_GPIO_ACTIVE_LOW: i32 = 0;
pub const WM8350_GPIO_ACTIVE_HIGH: i32 = 1;
pub const WM8350_GPIO_PULL_NONE: i32 = 0;
pub const WM8350_GPIO_PULL_UP: i32 = 1;
pub const WM8350_GPIO_PULL_DOWN: i32 = 2;
pub const WM8350_GPIO_INVERT_OFF: i32 = 0;
pub const WM8350_GPIO_INVERT_ON: i32 = 1;
pub const WM8350_GPIO_DEBOUNCE_OFF: i32 = 0;
pub const WM8350_GPIO_DEBOUNCE_ON: i32 = 1;

// Register bit masks. The source uses 16-bit register values.
pub const WM8350_GP12_EINT: u16 = 0x1000; pub const WM8350_GP11_EINT: u16 = 0x0800; pub const WM8350_GP10_EINT: u16 = 0x0400; pub const WM8350_GP9_EINT: u16 = 0x0200; pub const WM8350_GP8_EINT: u16 = 0x0100; pub const WM8350_GP7_EINT: u16 = 0x0080; pub const WM8350_GP6_EINT: u16 = 0x0040; pub const WM8350_GP5_EINT: u16 = 0x0020; pub const WM8350_GP4_EINT: u16 = 0x0010; pub const WM8350_GP3_EINT: u16 = 0x0008; pub const WM8350_GP2_EINT: u16 = 0x0004; pub const WM8350_GP1_EINT: u16 = 0x0002; pub const WM8350_GP0_EINT: u16 = 0x0001;
pub const WM8350_GP12_DB: u16 = 0x1000; pub const WM8350_GP11_DB: u16 = 0x0800; pub const WM8350_GP10_DB: u16 = 0x0400; pub const WM8350_GP9_DB: u16 = 0x0200; pub const WM8350_GP8_DB: u16 = 0x0100; pub const WM8350_GP7_DB: u16 = 0x0080; pub const WM8350_GP6_DB: u16 = 0x0040; pub const WM8350_GP5_DB: u16 = 0x0020; pub const WM8350_GP4_DB: u16 = 0x0010; pub const WM8350_GP3_DB: u16 = 0x0008; pub const WM8350_GP2_DB: u16 = 0x0004; pub const WM8350_GP1_DB: u16 = 0x0002; pub const WM8350_GP0_DB: u16 = 0x0001;
pub const WM8350_GP12_PU: u16 = 0x1000; pub const WM8350_GP11_PU: u16 = 0x0800; pub const WM8350_GP10_PU: u16 = 0x0400; pub const WM8350_GP9_PU: u16 = 0x0200; pub const WM8350_GP8_PU: u16 = 0x0100; pub const WM8350_GP7_PU: u16 = 0x0080; pub const WM8350_GP6_PU: u16 = 0x0040; pub const WM8350_GP5_PU: u16 = 0x0020; pub const WM8350_GP4_PU: u16 = 0x0010; pub const WM8350_GP3_PU: u16 = 0x0008; pub const WM8350_GP2_PU: u16 = 0x0004; pub const WM8350_GP1_PU: u16 = 0x0002; pub const WM8350_GP0_PU: u16 = 0x0001;
pub const WM8350_GP12_PD: u16 = 0x1000; pub const WM8350_GP11_PD: u16 = 0x0800; pub const WM8350_GP10_PD: u16 = 0x0400; pub const WM8350_GP9_PD: u16 = 0x0200; pub const WM8350_GP8_PD: u16 = 0x0100; pub const WM8350_GP7_PD: u16 = 0x0080; pub const WM8350_GP6_PD: u16 = 0x0040; pub const WM8350_GP5_PD: u16 = 0x0020; pub const WM8350_GP4_PD: u16 = 0x0010; pub const WM8350_GP3_PD: u16 = 0x0008; pub const WM8350_GP2_PD: u16 = 0x0004; pub const WM8350_GP1_PD: u16 = 0x0002; pub const WM8350_GP0_PD: u16 = 0x0001;
pub const WM8350_GP12_INTMODE: u16 = 0x1000; pub const WM8350_GP11_INTMODE: u16 = 0x0800; pub const WM8350_GP10_INTMODE: u16 = 0x0400; pub const WM8350_GP9_INTMODE: u16 = 0x0200; pub const WM8350_GP8_INTMODE: u16 = 0x0100; pub const WM8350_GP7_INTMODE: u16 = 0x0080; pub const WM8350_GP6_INTMODE: u16 = 0x0040; pub const WM8350_GP5_INTMODE: u16 = 0x0020; pub const WM8350_GP4_INTMODE: u16 = 0x0010; pub const WM8350_GP3_INTMODE: u16 = 0x0008; pub const WM8350_GP2_INTMODE: u16 = 0x0004; pub const WM8350_GP1_INTMODE: u16 = 0x0002; pub const WM8350_GP0_INTMODE: u16 = 0x0001;
pub const WM8350_GP_DBTIME_MASK: u16 = 0x00C0;
pub const WM8350_GP12_DIR: u16 = 0x1000; pub const WM8350_GP11_DIR: u16 = 0x0800; pub const WM8350_GP10_DIR: u16 = 0x0400; pub const WM8350_GP9_DIR: u16 = 0x0200; pub const WM8350_GP8_DIR: u16 = 0x0100; pub const WM8350_GP7_DIR: u16 = 0x0080; pub const WM8350_GP6_DIR: u16 = 0x0040; pub const WM8350_GP5_DIR: u16 = 0x0020; pub const WM8350_GP4_DIR: u16 = 0x0010; pub const WM8350_GP3_DIR: u16 = 0x0008; pub const WM8350_GP2_DIR: u16 = 0x0004; pub const WM8350_GP1_DIR: u16 = 0x0002; pub const WM8350_GP0_DIR: u16 = 0x0001;
pub const WM8350_GP12_CFG: u16 = 0x1000; pub const WM8350_GP11_CFG: u16 = 0x0800; pub const WM8350_GP10_CFG: u16 = 0x0400; pub const WM8350_GP9_CFG: u16 = 0x0200; pub const WM8350_GP8_CFG: u16 = 0x0100; pub const WM8350_GP7_CFG: u16 = 0x0080; pub const WM8350_GP6_CFG: u16 = 0x0040; pub const WM8350_GP5_CFG: u16 = 0x0020; pub const WM8350_GP4_CFG: u16 = 0x0010; pub const WM8350_GP3_CFG: u16 = 0x0008; pub const WM8350_GP2_CFG: u16 = 0x0004; pub const WM8350_GP1_CFG: u16 = 0x0002; pub const WM8350_GP0_CFG: u16 = 0x0001;
pub const WM8350_GP3_FN_MASK: u16 = 0xF000; pub const WM8350_GP2_FN_MASK: u16 = 0x0F00; pub const WM8350_GP1_FN_MASK: u16 = 0x00F0; pub const WM8350_GP0_FN_MASK: u16 = 0x000F;
pub const WM8350_GP7_FN_MASK: u16 = 0xF000; pub const WM8350_GP6_FN_MASK: u16 = 0x0F00; pub const WM8350_GP5_FN_MASK: u16 = 0x00F0; pub const WM8350_GP4_FN_MASK: u16 = 0x000F;
pub const WM8350_GP11_FN_MASK: u16 = 0xF000; pub const WM8350_GP10_FN_MASK: u16 = 0x0F00; pub const WM8350_GP9_FN_MASK: u16 = 0x00F0; pub const WM8350_GP8_FN_MASK: u16 = 0x000F;
pub const WM8350_GP12_FN_MASK: u16 = 0x000F;
pub const WM8350_GP12_LVL: u16 = 0x1000; pub const WM8350_GP11_LVL: u16 = 0x0800; pub const WM8350_GP10_LVL: u16 = 0x0400; pub const WM8350_GP9_LVL: u16 = 0x0200; pub const WM8350_GP8_LVL: u16 = 0x0100; pub const WM8350_GP7_LVL: u16 = 0x0080; pub const WM8350_GP6_LVL: u16 = 0x0040; pub const WM8350_GP5_LVL: u16 = 0x0020; pub const WM8350_GP4_LVL: u16 = 0x0010; pub const WM8350_GP3_LVL: u16 = 0x0008; pub const WM8350_GP2_LVL: u16 = 0x0004; pub const WM8350_GP1_LVL: u16 = 0x0002; pub const WM8350_GP0_LVL: u16 = 0x0001;

#[repr(C)]
pub struct wm8350;

#[repr(C)]
pub struct wm8350_gpio {
    pub pdev: *mut platform_device,
}

extern "C" {
    pub fn wm8350_gpio_config(wm8350: *mut wm8350, gpio: i32, dir: i32, func: i32,
                               pol: i32, pull: i32, invert: i32, debounce: i32) -> i32;
}

pub const fn WM8350_IRQ_GPIO(x: i32) -> i32 { 50 + x }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
