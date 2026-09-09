/* SPDX-License-Identifier: GPL-2.0-only */
/* Core driver interface to access RICOH_RC5T583 power management chip. */

// C dependencies supplied by other translation units:
pub enum device {}
pub enum regmap {}
pub enum mutex {}
pub enum regulator_init_data {}

pub const MAX_MAIN_INTERRUPT: usize = 5;
pub const RC5T583_MAX_GPEDGE_REG: usize = 2;
pub const RC5T583_MAX_INTERRUPT_EN_REGS: usize = 8;
pub const RC5T583_MAX_INTERRUPT_MASK_REGS: usize = 9;

pub const RC5T583_INT_EN_SYS1: u8 = 0x19;
pub const RC5T583_INT_EN_SYS2: u8 = 0x1D;
pub const RC5T583_INT_EN_DCDC: u8 = 0x41;
pub const RC5T583_INT_EN_RTC: u8 = 0xED;
pub const RC5T583_INT_EN_ADC1: u8 = 0x90;
pub const RC5T583_INT_EN_ADC2: u8 = 0x91;
pub const RC5T583_INT_EN_ADC3: u8 = 0x92;

pub const RC5T583_INTC_INTPOL: u8 = 0xAD;
pub const RC5T583_INTC_INTEN: u8 = 0xAE;
pub const RC5T583_INTC_INTMON: u8 = 0xAF;
pub const RC5T583_INT_MON_GRP: u8 = 0xAF;
pub const RC5T583_INT_MON_SYS1: u8 = 0x1B;
pub const RC5T583_INT_MON_SYS2: u8 = 0x1F;
pub const RC5T583_INT_MON_DCDC: u8 = 0x43;
pub const RC5T583_INT_MON_RTC: u8 = 0xEE;

pub const RC5T583_INT_IR_SYS1: u8 = 0x1A;
pub const RC5T583_INT_IR_SYS2: u8 = 0x1E;
pub const RC5T583_INT_IR_DCDC: u8 = 0x42;
pub const RC5T583_INT_IR_RTC: u8 = 0xEE;
pub const RC5T583_INT_IR_ADCL: u8 = 0x94;
pub const RC5T583_INT_IR_ADCH: u8 = 0x95;
pub const RC5T583_INT_IR_ADCEND: u8 = 0x96;
pub const RC5T583_INT_IR_GPIOR: u8 = 0xA9;
pub const RC5T583_INT_IR_GPIOF: u8 = 0xAA;

pub const RC5T583_SLPSEQ1: u8 = 0x21;
pub const RC5T583_SLPSEQ2: u8 = 0x22;
pub const RC5T583_SLPSEQ3: u8 = 0x23;
pub const RC5T583_SLPSEQ4: u8 = 0x24;
pub const RC5T583_SLPSEQ5: u8 = 0x25;
pub const RC5T583_SLPSEQ6: u8 = 0x26;
pub const RC5T583_SLPSEQ7: u8 = 0x27;
pub const RC5T583_SLPSEQ8: u8 = 0x28;
pub const RC5T583_SLPSEQ9: u8 = 0x29;
pub const RC5T583_SLPSEQ10: u8 = 0x2A;
pub const RC5T583_SLPSEQ11: u8 = 0x2B;

pub const RC5T583_REG_DC0CTL: u8 = 0x30; pub const RC5T583_REG_DC0DAC: u8 = 0x31; pub const RC5T583_REG_DC0LATCTL: u8 = 0x32; pub const RC5T583_REG_SR0CTL: u8 = 0x33;
pub const RC5T583_REG_DC1CTL: u8 = 0x34; pub const RC5T583_REG_DC1DAC: u8 = 0x35; pub const RC5T583_REG_DC1LATCTL: u8 = 0x36; pub const RC5T583_REG_SR1CTL: u8 = 0x37;
pub const RC5T583_REG_DC2CTL: u8 = 0x38; pub const RC5T583_REG_DC2DAC: u8 = 0x39; pub const RC5T583_REG_DC2LATCTL: u8 = 0x3A; pub const RC5T583_REG_SR2CTL: u8 = 0x3B;
pub const RC5T583_REG_DC3CTL: u8 = 0x3C; pub const RC5T583_REG_DC3DAC: u8 = 0x3D; pub const RC5T583_REG_DC3LATCTL: u8 = 0x3E; pub const RC5T583_REG_SR3CTL: u8 = 0x3F;
pub const RC5T583_REG_LDOEN1: u8 = 0x50; pub const RC5T583_REG_LDOEN2: u8 = 0x51; pub const RC5T583_REG_LDODIS1: u8 = 0x52; pub const RC5T583_REG_LDODIS2: u8 = 0x53;
pub const RC5T583_REG_LDO0DAC: u8 = 0x54; pub const RC5T583_REG_LDO1DAC: u8 = 0x55; pub const RC5T583_REG_LDO2DAC: u8 = 0x56; pub const RC5T583_REG_LDO3DAC: u8 = 0x57; pub const RC5T583_REG_LDO4DAC: u8 = 0x58; pub const RC5T583_REG_LDO5DAC: u8 = 0x59; pub const RC5T583_REG_LDO6DAC: u8 = 0x5A; pub const RC5T583_REG_LDO7DAC: u8 = 0x5B; pub const RC5T583_REG_LDO8DAC: u8 = 0x5C; pub const RC5T583_REG_LDO9DAC: u8 = 0x5D;
pub const RC5T583_REG_DC0DAC_DS: u8 = 0x60; pub const RC5T583_REG_DC1DAC_DS: u8 = 0x61; pub const RC5T583_REG_DC2DAC_DS: u8 = 0x62; pub const RC5T583_REG_DC3DAC_DS: u8 = 0x63;
pub const RC5T583_REG_LDO0DAC_DS: u8 = 0x64; pub const RC5T583_REG_LDO1DAC_DS: u8 = 0x65; pub const RC5T583_REG_LDO2DAC_DS: u8 = 0x66; pub const RC5T583_REG_LDO3DAC_DS: u8 = 0x67; pub const RC5T583_REG_LDO4DAC_DS: u8 = 0x68; pub const RC5T583_REG_LDO5DAC_DS: u8 = 0x69; pub const RC5T583_REG_LDO6DAC_DS: u8 = 0x6A; pub const RC5T583_REG_LDO7DAC_DS: u8 = 0x6B; pub const RC5T583_REG_LDO8DAC_DS: u8 = 0x6C; pub const RC5T583_REG_LDO9DAC_DS: u8 = 0x6D;

pub const RC5T583_GPIO_IOSEL: u8 = 0xA0; pub const RC5T583_GPIO_PDEN: u8 = 0xA1; pub const RC5T583_GPIO_IOOUT: u8 = 0xA2; pub const RC5T583_GPIO_PGSEL: u8 = 0xA3; pub const RC5T583_GPIO_GPINV: u8 = 0xA4; pub const RC5T583_GPIO_GPDEB: u8 = 0xA5; pub const RC5T583_GPIO_GPEDGE1: u8 = 0xA6; pub const RC5T583_GPIO_GPEDGE2: u8 = 0xA7; pub const RC5T583_GPIO_EN_INT: u8 = 0xA8; pub const RC5T583_GPIO_MON_IOIN: u8 = 0xAB; pub const RC5T583_GPIO_GPOFUNC: u8 = 0xAC;

pub const RC5T583_RTC_SEC: u8 = 0xE0; pub const RC5T583_RTC_MIN: u8 = 0xE1; pub const RC5T583_RTC_HOUR: u8 = 0xE2; pub const RC5T583_RTC_WDAY: u8 = 0xE3; pub const RC5T583_RTC_DAY: u8 = 0xE4; pub const RC5T583_RTC_MONTH: u8 = 0xE5; pub const RC5T583_RTC_YEAR: u8 = 0xE6; pub const RC5T583_RTC_ADJ: u8 = 0xE7; pub const RC5T583_RTC_AW_MIN: u8 = 0xE8; pub const RC5T583_RTC_AW_HOUR: u8 = 0xE9; pub const RC5T583_RTC_AW_WEEK: u8 = 0xEA; pub const RC5T583_RTC_AD_MIN: u8 = 0xEB; pub const RC5T583_RTC_AD_HOUR: u8 = 0xEC; pub const RC5T583_RTC_CTL1: u8 = 0xED; pub const RC5T583_RTC_CTL2: u8 = 0xEE; pub const RC5T583_RTC_AY_MIN: u8 = 0xF0; pub const RC5T583_RTC_AY_HOUR: u8 = 0xF1; pub const RC5T583_RTC_AY_DAY: u8 = 0xF2; pub const RC5T583_RTC_AY_MONTH: u8 = 0xF3; pub const RC5T583_RTC_AY_YEAR: u8 = 0xF4;
pub const RC5T583_MAX_REG: u8 = 0xF7;
pub const RC5T583_NUM_REGS: usize = RC5T583_MAX_REG as usize + 1;

#[repr(usize)] pub enum Rc5t583Irq { RC5T583_IRQ_ONKEY, RC5T583_IRQ_ACOK, RC5T583_IRQ_LIDOPEN, RC5T583_IRQ_PREOT, RC5T583_IRQ_CLKSTP, RC5T583_IRQ_ONKEY_OFF, RC5T583_IRQ_WD, RC5T583_IRQ_EN_PWRREQ1, RC5T583_IRQ_EN_PWRREQ2, RC5T583_IRQ_PRE_VINDET, RC5T583_IRQ_DC0LIM, RC5T583_IRQ_DC1LIM, RC5T583_IRQ_DC2LIM, RC5T583_IRQ_DC3LIM, RC5T583_IRQ_CTC, RC5T583_IRQ_YALE, RC5T583_IRQ_DALE, RC5T583_IRQ_WALE, RC5T583_IRQ_AIN1L, RC5T583_IRQ_AIN2L, RC5T583_IRQ_AIN3L, RC5T583_IRQ_VBATL, RC5T583_IRQ_VIN3L, RC5T583_IRQ_VIN8L, RC5T583_IRQ_AIN1H, RC5T583_IRQ_AIN2H, RC5T583_IRQ_AIN3H, RC5T583_IRQ_VBATH, RC5T583_IRQ_VIN3H, RC5T583_IRQ_VIN8H, RC5T583_IRQ_ADCEND, RC5T583_IRQ_GPIO0, RC5T583_IRQ_GPIO1, RC5T583_IRQ_GPIO2, RC5T583_IRQ_GPIO3, RC5T583_IRQ_GPIO4, RC5T583_IRQ_GPIO5, RC5T583_IRQ_GPIO6, RC5T583_IRQ_GPIO7, RC5T583_MAX_IRQS }
#[repr(usize)] pub enum Rc5t583Gpio { RC5T583_GPIO0, RC5T583_GPIO1, RC5T583_GPIO2, RC5T583_GPIO3, RC5T583_GPIO4, RC5T583_GPIO5, RC5T583_GPIO6, RC5T583_GPIO7, RC5T583_MAX_GPIO }
#[repr(usize)] pub enum Rc5t583Ds { RC5T583_DS_NONE, RC5T583_DS_DC0, RC5T583_DS_DC1, RC5T583_DS_DC2, RC5T583_DS_DC3, RC5T583_DS_LDO0, RC5T583_DS_LDO1, RC5T583_DS_LDO2, RC5T583_DS_LDO3, RC5T583_DS_LDO4, RC5T583_DS_LDO5, RC5T583_DS_LDO6, RC5T583_DS_LDO7, RC5T583_DS_LDO8, RC5T583_DS_LDO9, RC5T583_DS_PSO0, RC5T583_DS_PSO1, RC5T583_DS_PSO2, RC5T583_DS_PSO3, RC5T583_DS_PSO4, RC5T583_DS_PSO5, RC5T583_DS_PSO6, RC5T583_DS_PSO7, RC5T583_DS_MAX }

pub const RC5T583_EXT_PWRREQ1_CONTROL: i32 = 0x1;
pub const RC5T583_EXT_PWRREQ2_CONTROL: i32 = 0x2;
#[repr(usize)] pub enum Rc5t583Regulator { RC5T583_REGULATOR_DC0, RC5T583_REGULATOR_DC1, RC5T583_REGULATOR_DC2, RC5T583_REGULATOR_DC3, RC5T583_REGULATOR_LDO0, RC5T583_REGULATOR_LDO1, RC5T583_REGULATOR_LDO2, RC5T583_REGULATOR_LDO3, RC5T583_REGULATOR_LDO4, RC5T583_REGULATOR_LDO5, RC5T583_REGULATOR_LDO6, RC5T583_REGULATOR_LDO7, RC5T583_REGULATOR_LDO8, RC5T583_REGULATOR_LDO9, RC5T583_REGULATOR_MAX }

#[repr(C)] pub struct rc5t583 { pub dev: *mut device, pub regmap: *mut regmap, pub chip_irq: i32, pub irq_base: i32, pub irq_lock: mutex, pub group_irq_en: [usize; MAX_MAIN_INTERRUPT], pub intc_inten_reg: u8, pub irq_en_reg: [u8; RC5T583_MAX_INTERRUPT_EN_REGS], pub gpedge_reg: [u8; RC5T583_MAX_GPEDGE_REG] }
#[repr(C)] pub struct rc5t583_platform_data { pub irq_base: i32, pub gpio_base: i32, pub enable_shutdown: bool, pub regulator_deepsleep_slot: [i32; 15], pub regulator_ext_pwr_control: [usize; 15], pub reg_init_data: [*mut regulator_init_data; 15] }

extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut rc5t583;
    fn regmap_write(map: *mut regmap, reg: u8, val: u8) -> i32;
    fn regmap_read(map: *mut regmap, reg: u8, val: *mut u32) -> i32;
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    pub fn rc5t583_ext_power_req_config(dev: *mut device, deepsleep_id: i32, ext_pwr_req: i32, deepsleep_slot_nr: i32) -> i32;
    pub fn rc5t583_irq_init(chip: *mut rc5t583, irq: i32, irq_base: i32) -> i32;
    pub fn rc5t583_irq_exit(chip: *mut rc5t583) -> i32;
}

#[inline] pub unsafe fn rc5t583_write(dev: *mut device, reg: u8, val: u8) -> i32 { regmap_write((*dev_get_drvdata(dev)).regmap, reg, val) }
#[inline] pub unsafe fn rc5t583_read(dev: *mut device, reg: u8, val: *mut u8) -> i32 { let mut ival = 0u32; let ret = regmap_read((*dev_get_drvdata(dev)).regmap, reg, &mut ival); if ret == 0 { *val = ival as u8; } ret }
#[inline] pub unsafe fn rc5t583_set_bits(dev: *mut device, reg: u32, bit_mask: u32) -> i32 { regmap_update_bits((*dev_get_drvdata(dev)).regmap, reg, bit_mask, bit_mask) }
#[inline] pub unsafe fn rc5t583_clear_bits(dev: *mut device, reg: u32, bit_mask: u32) -> i32 { regmap_update_bits((*dev_get_drvdata(dev)).regmap, reg, bit_mask, 0) }
#[inline] pub unsafe fn rc5t583_update(dev: *mut device, reg: u32, val: u32, mask: u32) -> i32 { regmap_update_bits((*dev_get_drvdata(dev)).regmap, reg, mask, val) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
