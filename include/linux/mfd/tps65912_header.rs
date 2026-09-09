/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2015 Texas Instruments Incorporated - https://www.ti.com/
 *	Andrew F. Davis <afd@ti.com>
 *
 * Based on the TPS65218 driver and the previous TPS65912 driver by
 * Margarita Olaya Cabrera <magi@slimlogic.co.uk>
 */

// Linux header dependencies are supplied by the surrounding translation.

/* List of registers for TPS65912 */
pub const TPS65912_DCDC1_CTRL: u32 = 0x00;
pub const TPS65912_DCDC2_CTRL: u32 = 0x01;
pub const TPS65912_DCDC3_CTRL: u32 = 0x02;
pub const TPS65912_DCDC4_CTRL: u32 = 0x03;
pub const TPS65912_DCDC1_OP: u32 = 0x04;
pub const TPS65912_DCDC1_AVS: u32 = 0x05;
pub const TPS65912_DCDC1_LIMIT: u32 = 0x06;
pub const TPS65912_DCDC2_OP: u32 = 0x07;
pub const TPS65912_DCDC2_AVS: u32 = 0x08;
pub const TPS65912_DCDC2_LIMIT: u32 = 0x09;
pub const TPS65912_DCDC3_OP: u32 = 0x0A;
pub const TPS65912_DCDC3_AVS: u32 = 0x0B;
pub const TPS65912_DCDC3_LIMIT: u32 = 0x0C;
pub const TPS65912_DCDC4_OP: u32 = 0x0D;
pub const TPS65912_DCDC4_AVS: u32 = 0x0E;
pub const TPS65912_DCDC4_LIMIT: u32 = 0x0F;
pub const TPS65912_LDO1_OP: u32 = 0x10;
pub const TPS65912_LDO1_AVS: u32 = 0x11;
pub const TPS65912_LDO1_LIMIT: u32 = 0x12;
pub const TPS65912_LDO2_OP: u32 = 0x13;
pub const TPS65912_LDO2_AVS: u32 = 0x14;
pub const TPS65912_LDO2_LIMIT: u32 = 0x15;
pub const TPS65912_LDO3_OP: u32 = 0x16;
pub const TPS65912_LDO3_AVS: u32 = 0x17;
pub const TPS65912_LDO3_LIMIT: u32 = 0x18;
pub const TPS65912_LDO4_OP: u32 = 0x19;
pub const TPS65912_LDO4_AVS: u32 = 0x1A;
pub const TPS65912_LDO4_LIMIT: u32 = 0x1B;
pub const TPS65912_LDO5: u32 = 0x1C;
pub const TPS65912_LDO6: u32 = 0x1D;
pub const TPS65912_LDO7: u32 = 0x1E;
pub const TPS65912_LDO8: u32 = 0x1F;
pub const TPS65912_LDO9: u32 = 0x20;
pub const TPS65912_LDO10: u32 = 0x21;
pub const TPS65912_THRM: u32 = 0x22;
pub const TPS65912_CLK32OUT: u32 = 0x23;
pub const TPS65912_DEVCTRL: u32 = 0x24;
pub const TPS65912_DEVCTRL2: u32 = 0x25;
pub const TPS65912_I2C_SPI_CFG: u32 = 0x26;
pub const TPS65912_KEEP_ON: u32 = 0x27;
pub const TPS65912_KEEP_ON2: u32 = 0x28;
pub const TPS65912_SET_OFF1: u32 = 0x29;
pub const TPS65912_SET_OFF2: u32 = 0x2A;
pub const TPS65912_DEF_VOLT: u32 = 0x2B;
pub const TPS65912_DEF_VOLT_MAPPING: u32 = 0x2C;
pub const TPS65912_DISCHARGE: u32 = 0x2D;
pub const TPS65912_DISCHARGE2: u32 = 0x2E;
pub const TPS65912_EN1_SET1: u32 = 0x2F;
pub const TPS65912_EN1_SET2: u32 = 0x30;
pub const TPS65912_EN2_SET1: u32 = 0x31;
pub const TPS65912_EN2_SET2: u32 = 0x32;
pub const TPS65912_EN3_SET1: u32 = 0x33;
pub const TPS65912_EN3_SET2: u32 = 0x34;
pub const TPS65912_EN4_SET1: u32 = 0x35;
pub const TPS65912_EN4_SET2: u32 = 0x36;
pub const TPS65912_PGOOD: u32 = 0x37;
pub const TPS65912_PGOOD2: u32 = 0x38;
pub const TPS65912_INT_STS: u32 = 0x39;
pub const TPS65912_INT_MSK: u32 = 0x3A;
pub const TPS65912_INT_STS2: u32 = 0x3B;
pub const TPS65912_INT_MSK2: u32 = 0x3C;
pub const TPS65912_INT_STS3: u32 = 0x3D;
pub const TPS65912_INT_MSK3: u32 = 0x3E;
pub const TPS65912_INT_STS4: u32 = 0x3F;
pub const TPS65912_INT_MSK4: u32 = 0x40;
pub const TPS65912_GPIO1: u32 = 0x41;
pub const TPS65912_GPIO2: u32 = 0x42;
pub const TPS65912_GPIO3: u32 = 0x43;
pub const TPS65912_GPIO4: u32 = 0x44;
pub const TPS65912_GPIO5: u32 = 0x45;
pub const TPS65912_VMON: u32 = 0x46;
pub const TPS65912_LEDA_CTRL1: u32 = 0x47;
pub const TPS65912_LEDA_CTRL2: u32 = 0x48;
pub const TPS65912_LEDA_CTRL3: u32 = 0x49;
pub const TPS65912_LEDA_CTRL4: u32 = 0x4A;
pub const TPS65912_LEDA_CTRL5: u32 = 0x4B;
pub const TPS65912_LEDA_CTRL6: u32 = 0x4C;
pub const TPS65912_LEDA_CTRL7: u32 = 0x4D;
pub const TPS65912_LEDA_CTRL8: u32 = 0x4E;
pub const TPS65912_LEDB_CTRL1: u32 = 0x4F;
pub const TPS65912_LEDB_CTRL2: u32 = 0x50;
pub const TPS65912_LEDB_CTRL3: u32 = 0x51;
pub const TPS65912_LEDB_CTRL4: u32 = 0x52;
pub const TPS65912_LEDB_CTRL5: u32 = 0x53;
pub const TPS65912_LEDB_CTRL6: u32 = 0x54;
pub const TPS65912_LEDB_CTRL7: u32 = 0x55;
pub const TPS65912_LEDB_CTRL8: u32 = 0x56;
pub const TPS65912_LEDC_CTRL1: u32 = 0x57;
pub const TPS65912_LEDC_CTRL2: u32 = 0x58;
pub const TPS65912_LEDC_CTRL3: u32 = 0x59;
pub const TPS65912_LEDC_CTRL4: u32 = 0x5A;
pub const TPS65912_LEDC_CTRL5: u32 = 0x5B;
pub const TPS65912_LEDC_CTRL6: u32 = 0x5C;
pub const TPS65912_LEDC_CTRL7: u32 = 0x5D;
pub const TPS65912_LEDC_CTRL8: u32 = 0x5E;
pub const TPS65912_LED_RAMP_UP_TIME: u32 = 0x5F;
pub const TPS65912_LED_RAMP_DOWN_TIME: u32 = 0x60;
pub const TPS65912_LED_SEQ_EN: u32 = 0x61;
pub const TPS65912_LOADSWITCH: u32 = 0x62;
pub const TPS65912_SPARE: u32 = 0x63;
pub const TPS65912_VERNUM: u32 = 0x64;
pub const TPS6591X_MAX_REGISTER: u32 = 0x64;

macro_rules! bit { ($n:expr) => { 1u32 << $n }; }

/* INT_STS register field definitions */
pub const TPS65912_INT_STS_PWRHOLD_F: u32 = bit!(0);
pub const TPS65912_INT_STS_VMON: u32 = bit!(1);
pub const TPS65912_INT_STS_PWRON: u32 = bit!(2);
pub const TPS65912_INT_STS_PWRON_LP: u32 = bit!(3);
pub const TPS65912_INT_STS_PWRHOLD_R: u32 = bit!(4);
pub const TPS65912_INT_STS_HOTDIE: u32 = bit!(5);
pub const TPS65912_INT_STS_GPIO1_R: u32 = bit!(6);
pub const TPS65912_INT_STS_GPIO1_F: u32 = bit!(7);
pub const TPS65912_INT_STS2_GPIO2_R: u32 = bit!(0);
pub const TPS65912_INT_STS2_GPIO2_F: u32 = bit!(1);
pub const TPS65912_INT_STS2_GPIO3_R: u32 = bit!(2);
pub const TPS65912_INT_STS2_GPIO3_F: u32 = bit!(3);
pub const TPS65912_INT_STS2_GPIO4_R: u32 = bit!(4);
pub const TPS65912_INT_STS2_GPIO4_F: u32 = bit!(5);
pub const TPS65912_INT_STS2_GPIO5_R: u32 = bit!(6);
pub const TPS65912_INT_STS2_GPIO5_F: u32 = bit!(7);
pub const TPS65912_INT_STS3_PGOOD_DCDC1: u32 = bit!(0);
pub const TPS65912_INT_STS3_PGOOD_DCDC2: u32 = bit!(1);
pub const TPS65912_INT_STS3_PGOOD_DCDC3: u32 = bit!(2);
pub const TPS65912_INT_STS3_PGOOD_DCDC4: u32 = bit!(3);
pub const TPS65912_INT_STS3_PGOOD_LDO1: u32 = bit!(4);
pub const TPS65912_INT_STS3_PGOOD_LDO2: u32 = bit!(5);
pub const TPS65912_INT_STS3_PGOOD_LDO3: u32 = bit!(6);
pub const TPS65912_INT_STS3_PGOOD_LDO4: u32 = bit!(7);
pub const TPS65912_INT_STS4_PGOOD_LDO5: u32 = bit!(0);
pub const TPS65912_INT_STS4_PGOOD_LDO6: u32 = bit!(1);
pub const TPS65912_INT_STS4_PGOOD_LDO7: u32 = bit!(2);
pub const TPS65912_INT_STS4_PGOOD_LDO8: u32 = bit!(3);
pub const TPS65912_INT_STS4_PGOOD_LDO9: u32 = bit!(4);
pub const TPS65912_INT_STS4_PGOOD_LDO10: u32 = bit!(5);

/* GPIO and regulator field definitions */
pub const GPIO_SLEEP_MASK: u32 = 0x80; pub const GPIO_SLEEP_SHIFT: u32 = 7;
pub const GPIO_DEB_MASK: u32 = 0x10; pub const GPIO_DEB_SHIFT: u32 = 4;
pub const GPIO_CFG_MASK: u32 = 0x04; pub const GPIO_CFG_SHIFT: u32 = 2;
pub const GPIO_STS_MASK: u32 = 0x02; pub const GPIO_STS_SHIFT: u32 = 1;
pub const GPIO_SET_MASK: u32 = 0x01; pub const GPIO_SET_SHIFT: u32 = 0;

pub const GPIO3_SLEEP_MASK: u32 = 0x80; pub const GPIO3_SLEEP_SHIFT: u32 = 7;
pub const GPIO3_SEL_MASK: u32 = 0x40; pub const GPIO3_SEL_SHIFT: u32 = 6;
pub const GPIO3_ODEN_MASK: u32 = 0x20; pub const GPIO3_ODEN_SHIFT: u32 = 5;
pub const GPIO3_DEB_MASK: u32 = 0x10; pub const GPIO3_DEB_SHIFT: u32 = 4;
pub const GPIO3_PDEN_MASK: u32 = 0x08; pub const GPIO3_PDEN_SHIFT: u32 = 3;
pub const GPIO3_CFG_MASK: u32 = 0x04; pub const GPIO3_CFG_SHIFT: u32 = 2;
pub const GPIO3_STS_MASK: u32 = 0x02; pub const GPIO3_STS_SHIFT: u32 = 1;
pub const GPIO3_SET_MASK: u32 = 0x01; pub const GPIO3_SET_SHIFT: u32 = 0;

pub const GPIO4_SLEEP_MASK: u32 = 0x80; pub const GPIO4_SLEEP_SHIFT: u32 = 7;
pub const GPIO4_SEL_MASK: u32 = 0x40; pub const GPIO4_SEL_SHIFT: u32 = 6;
pub const GPIO4_ODEN_MASK: u32 = 0x20; pub const GPIO4_ODEN_SHIFT: u32 = 5;
pub const GPIO4_DEB_MASK: u32 = 0x10; pub const GPIO4_DEB_SHIFT: u32 = 4;
pub const GPIO4_PDEN_MASK: u32 = 0x08; pub const GPIO4_PDEN_SHIFT: u32 = 3;
pub const GPIO4_CFG_MASK: u32 = 0x04; pub const GPIO4_CFG_SHIFT: u32 = 2;
pub const GPIO4_STS_MASK: u32 = 0x02; pub const GPIO4_STS_SHIFT: u32 = 1;
pub const GPIO4_SET_MASK: u32 = 0x01; pub const GPIO4_SET_SHIFT: u32 = 0;

pub const THERM_THERM_HD_MASK: u32 = 0x20; pub const THERM_THERM_HD_SHIFT: u32 = 5;
pub const THERM_THERM_TS_MASK: u32 = 0x10; pub const THERM_THERM_TS_SHIFT: u32 = 4;
pub const THERM_THERM_HDSEL_MASK: u32 = 0x0C; pub const THERM_THERM_HDSEL_SHIFT: u32 = 2;
pub const THERM_RSVD1_MASK: u32 = 0x02; pub const THERM_RSVD1_SHIFT: u32 = 1;
pub const THERM_THERM_STATE_MASK: u32 = 0x01; pub const THERM_THERM_STATE_SHIFT: u32 = 0;

pub const DCDCCTRL_VCON_ENABLE_MASK: u32 = 0x80; pub const DCDCCTRL_VCON_ENABLE_SHIFT: u32 = 7;
pub const DCDCCTRL_VCON_RANGE1_MASK: u32 = 0x40; pub const DCDCCTRL_VCON_RANGE1_SHIFT: u32 = 6;
pub const DCDCCTRL_VCON_RANGE0_MASK: u32 = 0x20; pub const DCDCCTRL_VCON_RANGE0_SHIFT: u32 = 5;
pub const DCDCCTRL_TSTEP2_MASK: u32 = 0x10; pub const DCDCCTRL_TSTEP2_SHIFT: u32 = 4;
pub const DCDCCTRL_TSTEP1_MASK: u32 = 0x08; pub const DCDCCTRL_TSTEP1_SHIFT: u32 = 3;
pub const DCDCCTRL_TSTEP0_MASK: u32 = 0x04; pub const DCDCCTRL_TSTEP0_SHIFT: u32 = 2;
pub const DCDCCTRL_DCDC1_MODE_MASK: u32 = 0x02; pub const DCDCCTRL_DCDC1_MODE_SHIFT: u32 = 1;
/* The C header repeats these identifiers for DCDCCTRL2/3 with identical values. */
pub const DCDCCTRL_DCDC_MODE_MASK: u32 = 0x02; pub const DCDCCTRL_DCDC_MODE_SHIFT: u32 = 1;
pub const DCDCCTRL_RSVD0_MASK: u32 = 0x01; pub const DCDCCTRL_RSVD0_SHIFT: u32 = 0;
pub const DCDCCTRL_RAMP_TIME_MASK: u32 = 0x01; pub const DCDCCTRL_RAMP_TIME_SHIFT: u32 = 0;
pub const DCDC_AVS_ENABLE_MASK: u32 = 0x80; pub const DCDC_AVS_ENABLE_SHIFT: u32 = 7;
pub const DCDC_AVS_ECO_MASK: u32 = 0x40; pub const DCDC_AVS_ECO_SHIFT: u32 = 6;
pub const DCDC_LIMIT_RANGE_MASK: u32 = 0xC0; pub const DCDC_LIMIT_RANGE_SHIFT: u32 = 6;
pub const DCDC_LIMIT_MAX_SEL_MASK: u32 = 0x3F; pub const DCDC_LIMIT_MAX_SEL_SHIFT: u32 = 0;

/* Define the TPS65912 IRQ numbers */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum tps65912_irqs {
    TPS65912_IRQ_PWRHOLD_F,
    TPS65912_IRQ_VMON,
    TPS65912_IRQ_PWRON,
    TPS65912_IRQ_PWRON_LP,
    TPS65912_IRQ_PWRHOLD_R,
    TPS65912_IRQ_HOTDIE,
    TPS65912_IRQ_GPIO1_R,
    TPS65912_IRQ_GPIO1_F,
    TPS65912_IRQ_GPIO2_R,
    TPS65912_IRQ_GPIO2_F,
    TPS65912_IRQ_GPIO3_R,
    TPS65912_IRQ_GPIO3_F,
    TPS65912_IRQ_GPIO4_R,
    TPS65912_IRQ_GPIO4_F,
    TPS65912_IRQ_GPIO5_R,
    TPS65912_IRQ_GPIO5_F,
    TPS65912_IRQ_PGOOD_DCDC1,
    TPS65912_IRQ_PGOOD_DCDC2,
    TPS65912_IRQ_PGOOD_DCDC3,
    TPS65912_IRQ_PGOOD_DCDC4,
    TPS65912_IRQ_PGOOD_LDO1,
    TPS65912_IRQ_PGOOD_LDO2,
    TPS65912_IRQ_PGOOD_LDO3,
    TPS65912_IRQ_PGOOD_LDO4,
    TPS65912_IRQ_PGOOD_LDO5,
    TPS65912_IRQ_PGOOD_LDO6,
    TPS65912_IRQ_PGOOD_LDO7,
    TPS65912_IRQ_PGOOD_LDO8,
    TPS65912_IRQ_PGOOD_LDO9,
    TPS65912_IRQ_PGOOD_LDO10,
}

/*
 * struct tps65912 - state holder for the tps65912 driver
 *
 * Device data may be used to access the TPS65912 chip
 */
#[repr(C)]
pub struct tps65912 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    /* IRQ Data */
    pub irq: i32,
    pub irq_data: *mut regmap_irq_chip_data,
}

extern "C" {
    pub static tps65912_regmap_config: regmap_config;
    pub fn tps65912_device_init(tps: *mut tps65912) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
