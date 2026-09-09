/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2019 STMicroelectronics
 * Author(s): Amelie Delaunay <amelie.delaunay@st.com>.
 */

/* Dependency: linux/regmap.h */

/* General */
pub const STMFX_REG_CHIP_ID: u32 = 0x00; /* R */
pub const STMFX_REG_FW_VERSION_MSB: u32 = 0x01; /* R */
pub const STMFX_REG_FW_VERSION_LSB: u32 = 0x02; /* R */
pub const STMFX_REG_SYS_CTRL: u32 = 0x40; /* RW */
/* IRQ output management */
pub const STMFX_REG_IRQ_OUT_PIN: u32 = 0x41; /* RW */
pub const STMFX_REG_IRQ_SRC_EN: u32 = 0x42; /* RW */
pub const STMFX_REG_IRQ_PENDING: u32 = 0x08; /* R */
pub const STMFX_REG_IRQ_ACK: u32 = 0x44; /* RW */
/* GPIO management */
pub const STMFX_REG_IRQ_GPI_PENDING1: u32 = 0x0C; /* R */
pub const STMFX_REG_IRQ_GPI_PENDING2: u32 = 0x0D; /* R */
pub const STMFX_REG_IRQ_GPI_PENDING3: u32 = 0x0E; /* R */
pub const STMFX_REG_GPIO_STATE1: u32 = 0x10; /* R */
pub const STMFX_REG_GPIO_STATE2: u32 = 0x11; /* R */
pub const STMFX_REG_GPIO_STATE3: u32 = 0x12; /* R */
pub const STMFX_REG_IRQ_GPI_SRC1: u32 = 0x48; /* RW */
pub const STMFX_REG_IRQ_GPI_SRC2: u32 = 0x49; /* RW */
pub const STMFX_REG_IRQ_GPI_SRC3: u32 = 0x4A; /* RW */
pub const STMFX_REG_IRQ_GPI_EVT1: u32 = 0x4C; /* RW */
pub const STMFX_REG_IRQ_GPI_EVT2: u32 = 0x4D; /* RW */
pub const STMFX_REG_IRQ_GPI_EVT3: u32 = 0x4E; /* RW */
pub const STMFX_REG_IRQ_GPI_TYPE1: u32 = 0x50; /* RW */
pub const STMFX_REG_IRQ_GPI_TYPE2: u32 = 0x51; /* RW */
pub const STMFX_REG_IRQ_GPI_TYPE3: u32 = 0x52; /* RW */
pub const STMFX_REG_IRQ_GPI_ACK1: u32 = 0x54; /* RW */
pub const STMFX_REG_IRQ_GPI_ACK2: u32 = 0x55; /* RW */
pub const STMFX_REG_IRQ_GPI_ACK3: u32 = 0x56; /* RW */
pub const STMFX_REG_GPIO_DIR1: u32 = 0x60; /* RW */
pub const STMFX_REG_GPIO_DIR2: u32 = 0x61; /* RW */
pub const STMFX_REG_GPIO_DIR3: u32 = 0x62; /* RW */
pub const STMFX_REG_GPIO_TYPE1: u32 = 0x64; /* RW */
pub const STMFX_REG_GPIO_TYPE2: u32 = 0x65; /* RW */
pub const STMFX_REG_GPIO_TYPE3: u32 = 0x66; /* RW */
pub const STMFX_REG_GPIO_PUPD1: u32 = 0x68; /* RW */
pub const STMFX_REG_GPIO_PUPD2: u32 = 0x69; /* RW */
pub const STMFX_REG_GPIO_PUPD3: u32 = 0x6A; /* RW */
pub const STMFX_REG_GPO_SET1: u32 = 0x6C; /* RW */
pub const STMFX_REG_GPO_SET2: u32 = 0x6D; /* RW */
pub const STMFX_REG_GPO_SET3: u32 = 0x6E; /* RW */
pub const STMFX_REG_GPO_CLR1: u32 = 0x70; /* RW */
pub const STMFX_REG_GPO_CLR2: u32 = 0x71; /* RW */
pub const STMFX_REG_GPO_CLR3: u32 = 0x72; /* RW */

pub const STMFX_REG_MAX: u32 = 0xB0;
pub const STMFX_BOOT_TIME_MS: u32 = 10;

pub const STMFX_REG_CHIP_ID_MASK: u32 = 0xFF;
pub const STMFX_REG_SYS_CTRL_GPIO_EN: u32 = 1 << 0;
pub const STMFX_REG_SYS_CTRL_TS_EN: u32 = 1 << 1;
pub const STMFX_REG_SYS_CTRL_IDD_EN: u32 = 1 << 2;
pub const STMFX_REG_SYS_CTRL_ALTGPIO_EN: u32 = 1 << 3;
pub const STMFX_REG_SYS_CTRL_SWRST: u32 = 1 << 7;
pub const STMFX_REG_IRQ_OUT_PIN_TYPE: u32 = 1 << 0; /* 0-OD 1-PP */
pub const STMFX_REG_IRQ_OUT_PIN_POL: u32 = 1 << 1; /* 0-active LOW 1-active HIGH */

#[repr(u32)]
pub enum stmfx_irqs {
    STMFX_REG_IRQ_SRC_EN_GPIO = 0,
    STMFX_REG_IRQ_SRC_EN_IDD,
    STMFX_REG_IRQ_SRC_EN_ERROR,
    STMFX_REG_IRQ_SRC_EN_TS_DET,
    STMFX_REG_IRQ_SRC_EN_TS_NE,
    STMFX_REG_IRQ_SRC_EN_TS_TH,
    STMFX_REG_IRQ_SRC_EN_TS_FULL,
    STMFX_REG_IRQ_SRC_EN_TS_OVF,
    STMFX_REG_IRQ_SRC_MAX,
}

#[repr(u32)]
pub enum stmfx_functions {
    STMFX_FUNC_GPIO = 1 << 0, /* GPIO[15:0] */
    STMFX_FUNC_ALTGPIO_LOW = 1 << 1, /* aGPIO[3:0] */
    STMFX_FUNC_ALTGPIO_HIGH = 1 << 2, /* aGPIO[7:4] */
    STMFX_FUNC_TS = 1 << 3,
    STMFX_FUNC_IDD = 1 << 4,
}

/* External kernel types supplied by other files. */
pub enum device {}
pub enum regmap {}
pub enum regulator {}
pub enum irq_domain {}
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stmfx {
    pub dev: *mut device,
    pub map: *mut regmap,
    pub vdd: *mut regulator,
    pub irq: i32,
    pub irq_domain: *mut irq_domain,
    pub lock: mutex, /* IRQ bus lock */
    pub irq_src: u8,
    pub bkp_sysctrl: u8,
    pub bkp_irqoutpin: u8,
}

extern "C" {
    pub fn stmfx_function_enable(stmfx: *mut stmfx, func: u32) -> i32;
    pub fn stmfx_function_disable(stmfx: *mut stmfx, func: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
