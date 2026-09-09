/* SPDX-License-Identifier: GPL-2.0-only */
/* Marvell 88PM860x Interface */

pub const MFD_NAME_SIZE: usize = 40;

#[repr(i32)]
pub enum Chip { CHIP_INVALID = 0, CHIP_PM8606, CHIP_PM8607, CHIP_MAX }
#[repr(i32)]
pub enum Pm8606Id { PM8606_ID_INVALID, PM8606_ID_BACKLIGHT, PM8606_ID_LED, PM8606_ID_VIBRATOR, PM8606_ID_TOUCH, PM8606_ID_SOUND, PM8606_ID_CHARGER, PM8606_ID_MAX }

macro_rules! reg { ($($n:ident = $v:expr),* $(,)?) => { $(pub const $n: u8 = $v;) * }; }
reg! {
 PM8606_DCM_BOOST=0x00, PM8606_PWM=0x01, PM8607_MISC2=0x42, PM8607_POWER_UP_LOG=0x3F,
 PM8607_CCNT=0x47, PM8607_CHG_CTRL1=0x48, PM8607_CHG_CTRL2=0x49, PM8607_CHG_CTRL3=0x4A, PM8607_CHG_CTRL4=0x4B, PM8607_CHG_CTRL5=0x4C, PM8607_CHG_CTRL6=0x4D, PM8607_CHG_CTRL7=0x4E,
 PM8606_WLED1A=2, PM8606_WLED1B=3, PM8606_WLED2A=4, PM8606_WLED2B=5, PM8606_WLED3A=6, PM8606_WLED3B=7,
 PM8606_RGB2A=8, PM8606_RGB2B=9, PM8606_RGB2C=0x0A, PM8606_RGB2D=0x0B, PM8606_RGB1A=0x0C, PM8606_RGB1B=0x0D, PM8606_RGB1C=0x0E, PM8606_RGB1D=0x0F,
 PM8606_PREREGULATORA=0x10, PM8606_PREREGULATORB=0x11, PM8606_VIBRATORA=0x12, PM8606_VIBRATORB=0x13, PM8606_VCHG=0x14, PM8606_VSYS=0x15, PM8606_MISC=0x16, PM8606_CHIP_ID=0x17, PM8606_STATUS=0x18, PM8606_FLAGS=0x19, PM8606_PROTECTA=0x1A, PM8606_PROTECTB=0x1B, PM8606_PROTECTC=0x1C,
 PM8607_STATUS_1=1, PM8607_STATUS_2=2, PM8607_INT_STATUS1=3, PM8607_INT_STATUS2=4, PM8607_INT_STATUS3=5, PM8607_INT_MASK_1=6, PM8607_INT_MASK_2=7, PM8607_INT_MASK_3=8,
 PM8607_LDO1=0x10, PM8607_LDO2=0x11, PM8607_LDO3=0x12, PM8607_LDO4=0x13, PM8607_LDO5=0x14, PM8607_LDO6=0x15, PM8607_LDO7=0x16, PM8607_LDO8=0x17, PM8607_LDO9=0x18, PM8607_LDO10=0x19, PM8607_LDO12=0x1A, PM8607_LDO14=0x1B, PM8607_SLEEP_MODE1=0x1C, PM8607_SLEEP_MODE2=0x1D, PM8607_SLEEP_MODE3=0x1E, PM8607_SLEEP_MODE4=0x1F, PM8607_GO=0x20, PM8607_SLEEP_BUCK1=0x21, PM8607_SLEEP_BUCK2=0x22, PM8607_SLEEP_BUCK3=0x23, PM8607_BUCK1=0x24, PM8607_BUCK2=0x25, PM8607_BUCK3=0x26, PM8607_BUCK_CONTROLS=0x27, PM8607_SUPPLIES_EN11=0x2B, PM8607_SUPPLIES_EN12=0x2C, PM8607_GROUP1=0x2D, PM8607_GROUP2=0x2E, PM8607_GROUP3=0x2F, PM8607_GROUP4=0x30, PM8607_GROUP5=0x31, PM8607_GROUP6=0x32, PM8607_SUPPLIES_EN21=0x33, PM8607_SUPPLIES_EN22=0x34,
 PM8607_VIBRATOR_SET=0x28, PM8607_VIBRATOR_PWM=0x29, PM8607_GP_BIAS1=0x4F, PM8607_MEAS_EN1=0x50, PM8607_MEAS_EN2=0x51, PM8607_MEAS_EN3=0x52, PM8607_MEAS_OFF_TIME1=0x53, PM8607_MEAS_OFF_TIME2=0x54, PM8607_TSI_PREBIAS=0x55, PM8607_PD_PREBIAS=0x56, PM8607_GPADC_MISC1=0x57,
 PM8607_GP_BIAS2=0x5A, PM8607_VBAT_LOWTH=0x5B, PM8607_VCHG_LOWTH=0x5C, PM8607_VSYS_LOWTH=0x5D, PM8607_TINT_LOWTH=0x5E, PM8607_GPADC0_LOWTH=0x5F, PM8607_GPADC1_LOWTH=0x60, PM8607_GPADC2_LOWTH=0x61, PM8607_GPADC3_LOWTH=0x62, PM8607_VBAT_HIGHTH=0x63, PM8607_VCHG_HIGHTH=0x64, PM8607_VSYS_HIGHTH=0x65, PM8607_TINT_HIGHTH=0x66, PM8607_GPADC0_HIGHTH=0x67, PM8607_GPADC1_HIGHTH=0x68, PM8607_GPADC2_HIGHTH=0x69, PM8607_GPADC3_HIGHTH=0x6A, PM8607_IBAT_MEAS1=0x6B, PM8607_IBAT_MEAS2=0x6C, PM8607_VBAT_MEAS1=0x6D, PM8607_VBAT_MEAS2=0x6E, PM8607_VCHG_MEAS1=0x6F, PM8607_VCHG_MEAS2=0x70, PM8607_VSYS_MEAS1=0x71, PM8607_VSYS_MEAS2=0x72, PM8607_TINT_MEAS1=0x73, PM8607_TINT_MEAS2=0x74, PM8607_GPADC0_MEAS1=0x75, PM8607_GPADC0_MEAS2=0x76, PM8607_GPADC1_MEAS1=0x77, PM8607_GPADC1_MEAS2=0x78, PM8607_GPADC2_MEAS1=0x79, PM8607_GPADC2_MEAS2=0x7A, PM8607_GPADC3_MEAS1=0x7B, PM8607_GPADC3_MEAS2=0x7C, PM8607_CCNT_MEAS1=0x95, PM8607_CCNT_MEAS2=0x96, PM8607_VBAT_AVG=0x97, PM8607_VCHG_AVG=0x98, PM8607_VSYS_AVG=0x99, PM8607_VBAT_MIN=0x9A, PM8607_VCHG_MIN=0x9B, PM8607_VSYS_MIN=0x9C, PM8607_VBAT_MAX=0x9D, PM8607_VCHG_MAX=0x9E, PM8607_VSYS_MAX=0x9F,
 PM8607_GPADC_MISC2=0x59, PM8607_RTC1=0xA0, PM8607_RTC_COUNTER1=0xA1, PM8607_RTC_COUNTER2=0xA2, PM8607_RTC_COUNTER3=0xA3, PM8607_RTC_COUNTER4=0xA4, PM8607_RTC_EXPIRE1=0xA5, PM8607_RTC_EXPIRE2=0xA6, PM8607_RTC_EXPIRE3=0xA7, PM8607_RTC_EXPIRE4=0xA8, PM8607_RTC_TRIM1=0xA9, PM8607_RTC_TRIM2=0xAA, PM8607_RTC_TRIM3=0xAB, PM8607_RTC_TRIM4=0xAC, PM8607_RTC_MISC1=0xAD, PM8607_RTC_MISC2=0xAE, PM8607_RTC_MISC3=0xAF,
 PM8607_CHIP_ID=0, PM8607_B0_MISC1=0x0C, PM8607_DVC3=0x26, PM8607_A1_MISC1=0x40
}

pub const PM8606_DCM_500MA:u8=0; pub const PM8606_DCM_750MA:u8=1; pub const PM8606_DCM_1000MA:u8=2; pub const PM8606_DCM_1250MA:u8=3;
pub const PM8606_DCM_250MV:u8=0<<2; pub const PM8606_DCM_300MV:u8=1<<2; pub const PM8606_DCM_350MV:u8=2<<2; pub const PM8606_DCM_400MV:u8=3<<2;
pub const PM8606_PWM_FREQ_MASK:u8=7;
pub const PM8606_WLED_ON:u8=1<<0; pub const PM8606_VSYS_EN:u8=1<<1; pub const PM8606_MISC_OSC_EN:u8=1<<4;
#[inline] pub const fn PM8606_WLED_CURRENT(x:u8)->u8 {(x&0x1f)<<1}
#[inline] pub const fn PM8606_LED_CURRENT(x:u8)->u8 {((x>>2)&7)<<5}

#[repr(i32)] pub enum Pm8607Id { PM8607_ID_BUCK1=0, PM8607_ID_BUCK2, PM8607_ID_BUCK3, PM8607_ID_LDO1, PM8607_ID_LDO2, PM8607_ID_LDO3, PM8607_ID_LDO4, PM8607_ID_LDO5, PM8607_ID_LDO6, PM8607_ID_LDO7, PM8607_ID_LDO8, PM8607_ID_LDO9, PM8607_ID_LDO10, PM8607_ID_LDO11, PM8607_ID_LDO12, PM8607_ID_LDO13, PM8607_ID_LDO14, PM8607_ID_LDO15, PM8606_ID_PREG, PM8607_ID_RG_MAX }
pub const PM8607_VERSION_MASK:u8=0xF0;
pub const PM8607_MEAS_EN1_VBAT:u8=1<<0; pub const PM8607_MEAS_EN1_VCHG:u8=1<<1; pub const PM8607_MEAS_EN1_VSYS:u8=1<<2; pub const PM8607_MEAS_EN1_TINT:u8=1<<3; pub const PM8607_MEAS_EN1_RFTMP:u8=1<<4; pub const PM8607_MEAS_EN1_TBAT:u8=1<<5; pub const PM8607_MEAS_EN1_GPADC2:u8=1<<6; pub const PM8607_MEAS_EN1_GPADC3:u8=1<<7;
pub const PM8607_GPADC0_GP_BIAS_A0:u8=1; pub const PM8607_GPADC1_GP_BIAS_A1:u8=2; pub const PM8607_GPADC2_GP_BIAS_A2:u8=4; pub const PM8607_GPADC3_GP_BIAS_A3:u8=8; pub const PM8607_GPADC2_GP_BIAS_OUT2:u8=1<<6;
pub const PM8607_STATUS_CC:u16=1<<3; pub const PM8607_STATUS_PEN:u16=1<<4; pub const PM8607_STATUS_HEADSET:u16=1<<5; pub const PM8607_STATUS_HOOK:u16=1<<6; pub const PM8607_STATUS_MICIN:u16=1<<7; pub const PM8607_STATUS_ONKEY:u16=1<<8; pub const PM8607_STATUS_EXTON:u16=1<<9; pub const PM8607_STATUS_CHG:u16=1<<10; pub const PM8607_STATUS_BAT:u16=1<<11; pub const PM8607_STATUS_VBUS:u16=1<<12; pub const PM8607_STATUS_OV:u16=1<<13;
pub const PM8607_BUCK3_DOUBLE:u8=1<<6; pub const PM8607_A1_MISC1_PI2C:u8=1; pub const PM8607_B0_MISC1_INV_INT:u8=1; pub const PM8607_B0_MISC1_INT_CLEAR:u8=2; pub const PM8607_B0_MISC1_INT_MASK:u8=4; pub const PM8607_B0_MISC1_PI2C:u8=8; pub const PM8607_B0_MISC1_RESET:u8=1<<6;
pub const PM8607_GPADC_EN:u8=1; pub const PM8607_GPADC_PREBIAS_MASK:u8=3<<1; pub const PM8607_GPADC_SLOT_CYCLE_MASK:u8=3<<3; pub const PM8607_GPADC_OFF_SCALE_MASK:u8=3<<5; pub const PM8607_GPADC_SW_CAL_MASK:u8=1<<7; pub const PM8607_PD_PREBIAS_MASK:u8=0x1f; pub const PM8607_PD_PRECHG_MASK:u8=7<<5;
pub const PM8606_REF_GP_OSC_OFF:u8=0; pub const PM8606_REF_GP_OSC_ON:u8=1; pub const PM8606_REF_GP_OSC_UNKNOWN:u8=2;

#[repr(u16)] pub enum Pm8606RefGpAndOscClients { REF_GP_NO_CLIENTS=0, WLED1_DUTY=1<<0, WLED2_DUTY=1<<1, WLED3_DUTY=1<<2, RGB1_ENABLE=1<<3, RGB2_ENABLE=1<<4, LDO_VBR_EN=1<<5, REF_GP_MAX_CLIENT=0xFFFF }
#[repr(i32)] pub enum Pm8607Irq { PM8607_IRQ_ONKEY, PM8607_IRQ_EXTON, PM8607_IRQ_CHG, PM8607_IRQ_BAT, PM8607_IRQ_RTC, PM8607_IRQ_CC, PM8607_IRQ_VBAT, PM8607_IRQ_VCHG, PM8607_IRQ_VSYS, PM8607_IRQ_TINT, PM8607_IRQ_GPADC0, PM8607_IRQ_GPADC1, PM8607_IRQ_GPADC2, PM8607_IRQ_GPADC3, PM8607_IRQ_AUDIO_SHORT, PM8607_IRQ_PEN, PM8607_IRQ_HEADSET, PM8607_IRQ_HOOK, PM8607_IRQ_MICIN, PM8607_IRQ_CHG_FAIL, PM8607_IRQ_CHG_DONE, PM8607_IRQ_CHG_FAULT }
#[repr(i32)] pub enum Pm8607Chip { PM8607_CHIP_A0=0x40, PM8607_CHIP_A1=0x41, PM8607_CHIP_B0=0x48 }

#[repr(C)] pub struct device { _private: [u8;0] } #[repr(C)] pub struct mutex { _private: [u8;0] } #[repr(C)] pub struct i2c_client { _private: [u8;0] } #[repr(C)] pub struct regmap { _private: [u8;0] } #[repr(C)] pub struct regulator_init_data { _private: [u8;0] } #[repr(C)] pub struct charger_desc { _private: [u8;0] }
#[repr(C)] pub struct pm860x_chip { pub dev:*mut device, pub irq_lock:mutex, pub osc_lock:mutex, pub client:*mut i2c_client, pub companion:*mut i2c_client, pub regmap:*mut regmap, pub regmap_companion:*mut regmap, pub buck3_double:i32, pub companion_addr:i32, pub osc_vote:u16, pub id:i32, pub irq_mode:i32, pub irq_base:i32, pub core_irq:i32, pub chip_version:u8, pub osc_status:u8, pub wakeup_flag:u32 }
#[repr(i32)] pub enum I2cPort { GI2C_PORT=0, PI2C_PORT }
#[repr(C)] pub struct pm860x_backlight_pdata { pub pwm:i32, pub iset:i32 }
#[repr(C)] pub struct pm860x_led_pdata { pub iset:i32 }
#[repr(C)] pub struct pm860x_rtc_pdata { pub sync:Option<unsafe extern "C" fn(u32)->i32>, pub vrtc:i32 }
#[repr(C)] pub struct pm860x_touch_pdata { pub gpadc_prebias:i32, pub slot_cycle:i32, pub off_scale:i32, pub sw_cal:i32, pub tsi_prebias:i32, pub pen_prebias:i32, pub pen_prechg:i32, pub res_x:i32, pub flags:usize }
#[repr(C)] pub struct pm860x_power_pdata { pub max_capacity:i32, pub resistor:i32 }
#[repr(C)] pub struct pm860x_platform_data { pub backlight:*mut pm860x_backlight_pdata, pub led:*mut pm860x_led_pdata, pub rtc:*mut pm860x_rtc_pdata, pub touch:*mut pm860x_touch_pdata, pub power:*mut pm860x_power_pdata, pub buck1:*mut regulator_init_data, pub buck2:*mut regulator_init_data, pub buck3:*mut regulator_init_data, pub ldo1:*mut regulator_init_data, pub ldo2:*mut regulator_init_data, pub ldo3:*mut regulator_init_data, pub ldo4:*mut regulator_init_data, pub ldo5:*mut regulator_init_data, pub ldo6:*mut regulator_init_data, pub ldo7:*mut regulator_init_data, pub ldo8:*mut regulator_init_data, pub ldo9:*mut regulator_init_data, pub ldo10:*mut regulator_init_data, pub ldo12:*mut regulator_init_data, pub ldo_vibrator:*mut regulator_init_data, pub ldo14:*mut regulator_init_data, pub chg_desc:*mut charger_desc, pub companion_addr:i32, pub i2c_port:i32, pub irq_mode:i32, pub irq_base:i32, pub num_leds:i32, pub num_backlights:i32 }

extern "C" { pub fn pm8606_osc_enable(chip:*mut pm860x_chip, client:u16)->i32; pub fn pm8606_osc_disable(chip:*mut pm860x_chip, client:u16)->i32; pub fn pm860x_reg_read(c:*mut i2c_client, r:i32)->i32; pub fn pm860x_reg_write(c:*mut i2c_client,r:i32,v:u8)->i32; pub fn pm860x_bulk_read(c:*mut i2c_client,r:i32,n:i32,v:*mut u8)->i32; pub fn pm860x_bulk_write(c:*mut i2c_client,r:i32,n:i32,v:*mut u8)->i32; pub fn pm860x_set_bits(c:*mut i2c_client,r:i32,mask:u8,val:u8)->i32; pub fn pm860x_page_reg_write(c:*mut i2c_client,r:i32,v:u8)->i32; pub fn pm860x_page_bulk_read(c:*mut i2c_client,r:i32,n:i32,v:*mut u8)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
