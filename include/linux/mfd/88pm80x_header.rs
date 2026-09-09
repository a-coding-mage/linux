/* SPDX-License-Identifier: GPL-2.0-only */
/* Marvell 88PM80x Interface */

/* C header dependencies are supplied by the surrounding kernel bindings. */

pub const CHIP_INVALID: i32 = 0;
pub const CHIP_PM800: i32 = 1;
pub const CHIP_PM805: i32 = 2;
pub const CHIP_PM860: i32 = 3;
pub const CHIP_MAX: i32 = 4;

pub const PM800_ID_BUCK1: i32 = 0;
pub const PM800_ID_BUCK2: i32 = 1;
pub const PM800_ID_BUCK3: i32 = 2;
pub const PM800_ID_BUCK4: i32 = 3;
pub const PM800_ID_BUCK5: i32 = 4;
pub const PM800_ID_LDO1: i32 = 5;
pub const PM800_ID_LDO2: i32 = 6;
pub const PM800_ID_LDO3: i32 = 7;
pub const PM800_ID_LDO4: i32 = 8;
pub const PM800_ID_LDO5: i32 = 9;
pub const PM800_ID_LDO6: i32 = 10;
pub const PM800_ID_LDO7: i32 = 11;
pub const PM800_ID_LDO8: i32 = 12;
pub const PM800_ID_LDO9: i32 = 13;
pub const PM800_ID_LDO10: i32 = 14;
pub const PM800_ID_LDO11: i32 = 15;
pub const PM800_ID_LDO12: i32 = 16;
pub const PM800_ID_LDO13: i32 = 17;
pub const PM800_ID_LDO14: i32 = 18;
pub const PM800_ID_LDO15: i32 = 19;
pub const PM800_ID_LDO16: i32 = 20;
pub const PM800_ID_LDO17: i32 = 21;
pub const PM800_ID_LDO18: i32 = 22;
pub const PM800_ID_LDO19: i32 = 23;
pub const PM800_ID_RG_MAX: usize = 24;
pub const PM800_MAX_REGULATOR: usize = PM800_ID_RG_MAX;
pub const PM800_NUM_BUCK: usize = 5;
pub const PM800_NUM_LDO: usize = 19;

macro_rules! bit { ($n:expr) => { 1u32 << ($n) }; }

pub const PM800_STATUS_1: u8 = 0x01;
pub const PM800_ONKEY_STS1: u32 = bit!(0); pub const PM800_EXTON_STS1: u32 = bit!(1);
pub const PM800_CHG_STS1: u32 = bit!(2); pub const PM800_BAT_STS1: u32 = bit!(3);
pub const PM800_VBUS_STS1: u32 = bit!(4); pub const PM800_LDO_PGOOD_STS1: u32 = bit!(5);
pub const PM800_BUCK_PGOOD_STS1: u32 = bit!(6);
pub const PM800_STATUS_2: u8 = 0x02; pub const PM800_RTC_ALARM_STS2: u32 = bit!(0);
pub const PM800_WAKEUP1: u8 = 0x0D; pub const PM800_WAKEUP2: u8 = 0x0E;
pub const PM800_WAKEUP2_INV_INT: u32 = bit!(0); pub const PM800_WAKEUP2_INT_CLEAR: u32 = bit!(1); pub const PM800_WAKEUP2_INT_MASK: u32 = bit!(2);
pub const PM800_POWER_UP_LOG: u8 = 0x10;
pub const PM800_LOW_POWER1: u8 = 0x20; pub const PM800_LOW_POWER2: u8 = 0x21; pub const PM800_LOW_POWER_CONFIG3: u8 = 0x22; pub const PM800_LOW_POWER_CONFIG4: u8 = 0x23;
pub const PM800_GPIO_0_1_CNTRL: u8 = 0x30; pub const PM800_GPIO0_VAL: u32 = bit!(0);
pub const PM800_GPIO1_VAL: u32 = bit!(4); pub const PM800_GPIO_2_3_CNTRL: u8 = 0x31; pub const PM800_GPIO2_VAL: u32 = bit!(0); pub const PM800_GPIO3_VAL: u32 = bit!(4); pub const PM800_GPIO3_MODE_MASK: u32 = 0x1f;
pub const PM800_GPIO_4_CNTRL: u8 = 0x32; pub const PM800_GPIO4_VAL: u32 = bit!(0);
pub const PM800_HEADSET_CNTRL: u8 = 0x38; pub const PM800_HEADSET_DET_EN: u32 = bit!(7); pub const PM800_HSDET_SLP: u32 = bit!(1);
pub const PM800_PWM1: u8 = 0x40; pub const PM800_PWM2: u8 = 0x41; pub const PM800_PWM3: u8 = 0x42; pub const PM800_PWM4: u8 = 0x43;
pub const PM800_RTC_CONTROL: u8 = 0xd0; pub const PM800_RTC_MISC1: u8 = 0xe1; pub const PM800_RTC_MISC2: u8 = 0xe2; pub const PM800_RTC_MISC3: u8 = 0xe3; pub const PM800_RTC_MISC4: u8 = 0xe4; pub const PM800_RTC_MISC5: u8 = 0xe7;
pub const PM800_ALARM1_EN: u32 = bit!(0); pub const PM800_ALARM_WAKEUP: u32 = bit!(4); pub const PM800_ALARM: u32 = bit!(5); pub const PM800_RTC1_USE_XO: u32 = bit!(7);
pub const PM800_SLEEP_BUCK1: u8 = 0x30; pub const PM800_BUCK_SLP1: u8 = 0x5a; pub const PM800_BUCK1_SLP1_SHIFT: u32 = 0; pub const PM800_BUCK1_SLP1_MASK: u32 = 0x3 << PM800_BUCK1_SLP1_SHIFT;

pub const PM800_GPADC_MEAS_EN1: u8 = 1; pub const PM800_MEAS_EN1_VBAT: u32 = bit!(2); pub const PM800_GPADC_MEAS_EN2: u8 = 2; pub const PM800_MEAS_EN2_RFTMP: u32 = bit!(0);
pub const PM800_MEAS_GP0_EN: u32 = bit!(2); pub const PM800_MEAS_GP1_EN: u32 = bit!(3); pub const PM800_MEAS_GP2_EN: u32 = bit!(4); pub const PM800_MEAS_GP3_EN: u32 = bit!(5); pub const PM800_MEAS_GP4_EN: u32 = bit!(6);
pub const PM800_GPADC_MISC_CONFIG1: u8 = 5; pub const PM800_GPADC_MISC_CONFIG2: u8 = 6; pub const PM800_GPADC_MISC_GPFSM_EN: u32 = bit!(0); pub const PM800_GPADC_MISC_CONFIG3: u8 = 9; pub const PM800_GPADC_MISC_CONFIG4: u8 = 0x0a;
pub const PM800_GPADC_PREBIAS1: u8 = 0x0f; pub const PM800_GPADC_PREBIAS2: u8 = 0x10; pub const PM800_GP_BIAS_ENA1: u8 = 0x14; pub const PM800_GP_BIAS_OUT1: u8 = 0x15;
pub const PM800_GPADC_GP_BIAS_EN0: u32 = bit!(0); pub const PM800_GPADC_GP_BIAS_EN1: u32 = bit!(1); pub const PM800_GPADC_GP_BIAS_EN2: u32 = bit!(2); pub const PM800_GPADC_GP_BIAS_EN3: u32 = bit!(3);
pub const PM800_BIAS_OUT_GP0: u32 = bit!(0); pub const PM800_BIAS_OUT_GP1: u32 = bit!(1); pub const PM800_BIAS_OUT_GP2: u32 = bit!(2); pub const PM800_BIAS_OUT_GP3: u32 = bit!(3);

pub const PM800_GPADC0_LOW_TH: u8=0x20; pub const PM800_GPADC1_LOW_TH: u8=0x21; pub const PM800_GPADC2_LOW_TH: u8=0x22; pub const PM800_GPADC3_LOW_TH: u8=0x23; pub const PM800_GPADC4_LOW_TH: u8=0x24;
pub const PM800_GPADC0_UPP_TH: u8=0x30; pub const PM800_GPADC1_UPP_TH: u8=0x31; pub const PM800_GPADC2_UPP_TH: u8=0x32; pub const PM800_GPADC3_UPP_TH: u8=0x33; pub const PM800_GPADC4_UPP_TH: u8=0x34;
pub const PM800_VBBAT_MEAS1:u8=0x40; pub const PM800_VBBAT_MEAS2:u8=0x41; pub const PM800_VBAT_MEAS1:u8=0x42; pub const PM800_VBAT_MEAS2:u8=0x43; pub const PM800_VSYS_MEAS1:u8=0x44; pub const PM800_VSYS_MEAS2:u8=0x45; pub const PM800_VCHG_MEAS1:u8=0x46; pub const PM800_VCHG_MEAS2:u8=0x47; pub const PM800_TINT_MEAS1:u8=0x50; pub const PM800_TINT_MEAS2:u8=0x51; pub const PM800_PMOD_MEAS1:u8=0x52; pub const PM800_PMOD_MEAS2:u8=0x53;
pub const PM800_GPADC0_MEAS1:u8=0x54; pub const PM800_GPADC0_MEAS2:u8=0x55; pub const PM800_GPADC1_MEAS1:u8=0x56; pub const PM800_GPADC1_MEAS2:u8=0x57; pub const PM800_GPADC2_MEAS1:u8=0x58; pub const PM800_GPADC2_MEAS2:u8=0x59; pub const PM800_GPADC3_MEAS1:u8=0x5a; pub const PM800_GPADC3_MEAS2:u8=0x5b; pub const PM800_GPADC4_MEAS1:u8=0x5c; pub const PM800_GPADC4_MEAS2:u8=0x5d; pub const PM800_GPADC4_AVG1:u8=0xa8; pub const PM800_GPADC4_AVG2:u8=0xa9;

pub const PM805_MAIN_POWERUP:u8=1; pub const PM805_INT_STATUS0:u8=2; pub const PM805_STATUS0_INT_CLEAR:u32=1; pub const PM805_STATUS0_INV_INT:u32=2; pub const PM800_STATUS0_INT_MASK:u32=4; pub const PM805_INT_STATUS1:u8=3;
pub const PM805_INT1_HP1_SHRT:u32=bit!(0); pub const PM805_INT1_HP2_SHRT:u32=bit!(1); pub const PM805_INT1_MIC_CONFLICT:u32=bit!(2); pub const PM805_INT1_CLIP_FAULT:u32=bit!(3); pub const PM805_INT1_LDO_OFF:u32=bit!(4); pub const PM805_INT1_SRC_DPLL_LOCK:u32=bit!(5);
pub const PM805_INT_STATUS2:u8=4; pub const PM805_INT2_MIC_DET:u32=bit!(0); pub const PM805_INT2_SHRT_BTN_DET:u32=bit!(1); pub const PM805_INT2_VOLM_BTN_DET:u32=bit!(2); pub const PM805_INT2_VOLP_BTN_DET:u32=bit!(3); pub const PM805_INT2_RAW_PLL_FAULT:u32=bit!(4); pub const PM805_INT2_FINE_PLL_FAULT:u32=bit!(5);
pub const PM805_INT_MASK1:u8=5; pub const PM805_INT_MASK2:u8=6; pub const PM805_SHRT_BTN_DET:u32=bit!(1); pub const PM805_INT_REG_NUM:usize=2; pub const PM805_MIC_DET1:u8=7; pub const PM805_MIC_DET_EN_MIC_DET:u32=bit!(0); pub const PM805_MIC_DET2:u8=8; pub const PM805_MIC_DET_STATUS1:u8=9; pub const PM805_MIC_DET_STATUS3:u8=0xa; pub const PM805_AUTO_SEQ_STATUS1:u8=0xb; pub const PM805_AUTO_SEQ_STATUS2:u8=0xc;
pub const PM805_ADC_SETTING1:u8=0x10; pub const PM805_ADC_SETTING2:u8=0x11; pub const PM805_ADC_SETTING3:u8=0x11; pub const PM805_ADC_GAIN1:u8=0x12; pub const PM805_ADC_GAIN2:u8=0x13; pub const PM805_DMIC_SETTING:u8=0x15; pub const PM805_DWS_SETTING:u8=0x16; pub const PM805_MIC_CONFLICT_STS:u8=0x17;
pub const PM805_PDM_SETTING1:u8=0x20; pub const PM805_PDM_SETTING2:u8=0x21; pub const PM805_PDM_SETTING3:u8=0x22; pub const PM805_PDM_CONTROL1:u8=0x23; pub const PM805_PDM_CONTROL2:u8=0x24; pub const PM805_PDM_CONTROL3:u8=0x25; pub const PM805_HEADPHONE_SETTING:u8=0x26; pub const PM805_HEADPHONE_GAIN_A2A:u8=0x27; pub const PM805_HEADPHONE_SHORT_STATE:u8=0x28; pub const PM805_EARPHONE_SETTING:u8=0x29; pub const PM805_AUTO_SEQ_SETTING:u8=0x2a;

#[repr(C)] pub struct pm80x_rtc_pdata { pub vrtc: i32, pub rtc_wakeup: i32 }
#[repr(C)] pub struct pm80x_subchip { pub power_page:*mut i2c_client, pub gpadc_page:*mut i2c_client, pub regmap_power:*mut regmap, pub regmap_gpadc:*mut regmap, pub power_page_addr:u16, pub gpadc_page_addr:u16 }
#[repr(C)] pub struct pm80x_chip { pub subchip:*mut pm80x_subchip, pub dev:*mut device, pub client:*mut i2c_client, pub companion:*mut i2c_client, pub regmap:*mut regmap, pub regmap_irq_chip:*const regmap_irq_chip, pub irq_data:*mut regmap_irq_chip_data, pub type_:i32, pub irq:i32, pub irq_mode:i32, pub wu_flag:usize, pub lock:spinlock_t }
#[repr(C)] pub struct pm80x_platform_data { pub rtc:*mut pm80x_rtc_pdata, pub regulators:[*mut regulator_init_data; PM800_ID_RG_MAX], pub num_regulators:u32, pub irq_mode:i32, pub batt_det:i32, pub plat_config:Option<unsafe extern "C" fn(*mut pm80x_chip,*mut pm80x_platform_data)->i32> }

extern "C" { pub static pm80x_pm_ops: dev_pm_ops; pub static pm80x_regmap_config: regmap_config; pub fn request_threaded_irq(irq:i32, primary:Option<unsafe extern "C" fn()> , handler:irq_handler_t, flags:usize, name:*const i8, data:*mut core::ffi::c_void)->i32; pub fn regmap_irq_get_virq(data:*mut regmap_irq_chip_data, irq:i32)->i32; pub fn free_irq(irq:i32,data:*mut core::ffi::c_void); pub fn pm80x_init(client:*mut i2c_client)->i32; pub fn pm80x_deinit()->i32; }
pub type irq_handler_t = Option<unsafe extern "C" fn(i32,*mut core::ffi::c_void)->i32>;
extern "C" { pub fn to_platform_device(dev:*mut device)->*mut platform_device; pub fn dev_get_drvdata(dev:*mut device)->*mut core::ffi::c_void; pub fn platform_get_irq(pdev:*mut platform_device,index:i32)->i32; pub fn device_may_wakeup(dev:*mut device)->bool; pub fn set_bit(n:i32, addr:*mut usize); pub fn clear_bit(n:i32, addr:*mut usize); }
#[inline] pub unsafe fn pm80x_request_irq(p:*mut pm80x_chip, irq:i32, handler:irq_handler_t, flags:usize, name:*const i8, data:*mut core::ffi::c_void)->i32 { if (*p).irq_data.is_null() { return -22; } request_threaded_irq(regmap_irq_get_virq((*p).irq_data,irq),None,handler,flags,name,data) }
#[inline] pub unsafe fn pm80x_free_irq(p:*mut pm80x_chip, irq:i32, data:*mut core::ffi::c_void) { if !(*p).irq_data.is_null() { free_irq(regmap_irq_get_virq((*p).irq_data,irq),data); } }
/* CONFIG_PM conditional helpers are retained as declarations when enabled by the surrounding build. */
#[cfg(feature="CONFIG_PM")] #[inline] pub unsafe fn pm80x_dev_suspend(dev:*mut device)->i32 { let p=to_platform_device(dev); let c=dev_get_drvdata((*p).dev.parent) as *mut pm80x_chip; let irq=platform_get_irq(p,0); if device_may_wakeup(dev) { set_bit(irq,&mut (*c).wu_flag); } 0 }
#[cfg(feature="CONFIG_PM")] #[inline] pub unsafe fn pm80x_dev_resume(dev:*mut device)->i32 { let p=to_platform_device(dev); let c=dev_get_drvdata((*p).dev.parent) as *mut pm80x_chip; let irq=platform_get_irq(p,0); if device_may_wakeup(dev) { clear_bit(irq,&mut (*c).wu_flag); } 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
