/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency intent from <linux/bits.h>: BIT and GENMASK are expected to be
// supplied by the surrounding translation environment.

pub const MAX7360_MAX_KEY_ROWS: u32 = 8;
pub const MAX7360_MAX_KEY_COLS: u32 = 8;
pub const MAX7360_MAX_KEY_NUM: u32 = MAX7360_MAX_KEY_ROWS * MAX7360_MAX_KEY_COLS;
pub const MAX7360_ROW_SHIFT: u32 = 3;

pub const MAX7360_MAX_GPIO: u32 = 8;
pub const MAX7360_MAX_GPO: u32 = 6;
pub const MAX7360_PORT_PWM_COUNT: u32 = 8;
pub const MAX7360_PORT_RTR_PIN: u32 = MAX7360_PORT_PWM_COUNT - 1;

/*
 * MAX7360 registers
 */
pub const MAX7360_REG_KEYFIFO: u32 = 0x00;
pub const MAX7360_REG_CONFIG: u32 = 0x01;
pub const MAX7360_REG_DEBOUNCE: u32 = 0x02;
pub const MAX7360_REG_INTERRUPT: u32 = 0x03;
pub const MAX7360_REG_PORTS: u32 = 0x04;
pub const MAX7360_REG_KEYREP: u32 = 0x05;
pub const MAX7360_REG_SLEEP: u32 = 0x06;

/*
 * MAX7360 GPIO registers
 *
 * All these registers are reset together when writing bit 3 of
 * MAX7360_REG_GPIOCFG.
 */
pub const MAX7360_REG_GPIOCFG: u32 = 0x40;
pub const MAX7360_REG_GPIOCTRL: u32 = 0x41;
pub const MAX7360_REG_GPIODEB: u32 = 0x42;
pub const MAX7360_REG_GPIOCURR: u32 = 0x43;
pub const MAX7360_REG_GPIOOUTM: u32 = 0x44;
pub const MAX7360_REG_PWMCOM: u32 = 0x45;
pub const MAX7360_REG_RTRCFG: u32 = 0x46;
pub const MAX7360_REG_I2C_TIMEOUT: u32 = 0x48;
pub const MAX7360_REG_GPIOIN: u32 = 0x49;
pub const MAX7360_REG_RTR_CNT: u32 = 0x4A;
pub const MAX7360_REG_PWMBASE: u32 = 0x50;
pub const MAX7360_REG_PWMCFGBASE: u32 = 0x58;

pub const MAX7360_REG_GPIO_LAST: u32 = 0x5F;

#[inline]
pub const fn MAX7360_REG_PWM(x: u32) -> u32 { MAX7360_REG_PWMBASE + x }
#[inline]
pub const fn MAX7360_REG_PWMCFG(x: u32) -> u32 { MAX7360_REG_PWMCFGBASE + x }

/*
 * Configuration register bits
 */
pub const MAX7360_FIFO_EMPTY: u32 = 0x3F;
pub const MAX7360_FIFO_OVERFLOW: u32 = 0x7F;
pub const MAX7360_FIFO_RELEASE: u32 = 1 << 6;
pub const MAX7360_FIFO_COL: u32 = 0x3F << 3;
pub const MAX7360_FIFO_ROW: u32 = 0x07;

pub const MAX7360_CFG_SLEEP: u32 = 1 << 7;
pub const MAX7360_CFG_INTERRUPT: u32 = 1 << 5;
pub const MAX7360_CFG_KEY_RELEASE: u32 = 1 << 3;
pub const MAX7360_CFG_WAKEUP: u32 = 1 << 1;
pub const MAX7360_CFG_TIMEOUT: u32 = 1 << 0;

pub const MAX7360_DEBOUNCE: u32 = 0x1F;
pub const MAX7360_DEBOUNCE_MIN: u32 = 9;
pub const MAX7360_DEBOUNCE_MAX: u32 = 40;
pub const MAX7360_PORTS: u32 = 0x0F << 5;

pub const MAX7360_INTERRUPT_TIME_MASK: u32 = 0x1F;
pub const MAX7360_INTERRUPT_FIFO_MASK: u32 = 0x07 << 5;

pub const MAX7360_PORT_CFG_INTERRUPT_MASK: u32 = 1 << 7;
pub const MAX7360_PORT_CFG_INTERRUPT_EDGES: u32 = 1 << 6;
pub const MAX7360_PORT_CFG_COMMON_PWM: u32 = 1 << 5;

/*
 * Autosleep register values
 */
pub const MAX7360_AUTOSLEEP_8192MS: u32 = 0x01;
pub const MAX7360_AUTOSLEEP_4096MS: u32 = 0x02;
pub const MAX7360_AUTOSLEEP_2048MS: u32 = 0x03;
pub const MAX7360_AUTOSLEEP_1024MS: u32 = 0x04;
pub const MAX7360_AUTOSLEEP_512MS: u32 = 0x05;
pub const MAX7360_AUTOSLEEP_256MS: u32 = 0x06;

pub const MAX7360_GPIO_CFG_RTR_EN: u32 = 1 << 7;
pub const MAX7360_GPIO_CFG_GPIO_EN: u32 = 1 << 4;
pub const MAX7360_GPIO_CFG_GPIO_RST: u32 = 1 << 3;

pub const MAX7360_ROT_DEBOUNCE: u32 = 0x0F;
pub const MAX7360_ROT_DEBOUNCE_MIN: u32 = 0;
pub const MAX7360_ROT_DEBOUNCE_MAX: u32 = 15;
pub const MAX7360_ROT_INTCNT: u32 = 0x07 << 4;
pub const MAX7360_ROT_INTCNT_DLY: u32 = 1 << 7;

pub const MAX7360_INT_INTI: u32 = 0;
pub const MAX7360_INT_INTK: u32 = 1;

pub const MAX7360_INT_GPIO: u32 = 0;
pub const MAX7360_INT_KEYPAD: u32 = 1;
pub const MAX7360_INT_ROTARY: u32 = 2;

pub const MAX7360_NR_INTERNAL_IRQS: u32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
