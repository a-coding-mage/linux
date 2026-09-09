// SPDX-License-Identifier: (GPL-2.0)
/*
 * Microchip PolarFire SoC (MPFS) GPIO controller driver
 *
 * Copyright (c) 2018-2024 Microchip Technology Inc. and its subsidiaries
 */

const MPFS_MAX_NUM_GPIO: u32 = 32;
const MPFS_GPIO_EN_INT: u32 = 1 << 3;
const MPFS_GPIO_EN_OUT_BUF: u32 = 1 << 2;
const MPFS_GPIO_EN_IN: u32 = 1 << 1;
const MPFS_GPIO_EN_OUT: u32 = 1;
const MPFS_GPIO_DIR_MASK: u32 = (1 << 3) - 1;

const MPFS_GPIO_TYPE_INT_EDGE_BOTH: u32 = 0x80;
const MPFS_GPIO_TYPE_INT_EDGE_NEG: u32 = 0x60;
const MPFS_GPIO_TYPE_INT_EDGE_POS: u32 = 0x40;
const MPFS_GPIO_TYPE_INT_LEVEL_LOW: u32 = 0x20;
const MPFS_GPIO_TYPE_INT_LEVEL_HIGH: u32 = 0x00;
const MPFS_GPIO_TYPE_INT_MASK: u32 = ((1 << 3) - 1) << 5;
const MPFS_IRQ_REG: u32 = 0x80;

const MPFS_INP_REG: u8 = 0x84;
const COREGPIO_INP_REG: u8 = 0x90;
const MPFS_OUTP_REG: u8 = 0x88;
const COREGPIO_OUTP_REG: u8 = 0xA0;

#[repr(C)]
pub struct mpfs_gpio_reg_offsets {
    pub inp: u8,
    pub outp: u8,
}

#[repr(C)]
pub struct mpfs_gpio_chip {
    pub regs: *mut regmap,
    pub offsets: *const mpfs_gpio_reg_offsets,
    pub gc: gpio_chip,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: u32,
    pub reg_stride: u32,
    pub val_bits: u32,
    pub use_raw_spinlock: bool,
}

static MPFS_GPIO_REGMAP_CONFIG: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    use_raw_spinlock: true,
};

unsafe extern "C" {
    fn gpiochip_get_data(gc: *mut gpio_chip) -> *mut mpfs_gpio_chip;
    fn regmap_update_bits(regs: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn regmap_read(regs: *mut regmap, reg: u32, val: *mut u32) -> i32;
    fn regmap_test_bits(regs: *mut regmap, reg: u8, mask: u32) -> i32;
    fn gpiochip_enable_irq(gc: *mut gpio_chip, gpio: i32);
    fn gpiochip_disable_irq(gc: *mut gpio_chip, gpio: i32);
    fn irq_data_get_irq_chip_data(data: *mut irq_data) -> *mut gpio_chip;
    fn irqd_to_hwirq(data: *mut irq_data) -> u32;
    fn regmap_write(regs: *mut regmap, reg: u32, val: u32) -> i32;
    fn generic_handle_domain_irq(domain: *mut irq_domain, irq: i32) -> i32;
}

#[inline]
const fn mpfs_gpio_ctrl(i: u32) -> u32 { 0x4 * i }

unsafe extern "C" fn mpfs_gpio_direction_input(gc: *mut gpio_chip, gpio_index: u32) -> i32 {
    let mpfs_gpio = gpiochip_get_data(gc);
    regmap_update_bits((*mpfs_gpio).regs, mpfs_gpio_ctrl(gpio_index), MPFS_GPIO_DIR_MASK, MPFS_GPIO_EN_IN);
    0
}

unsafe extern "C" fn mpfs_gpio_direction_output(gc: *mut gpio_chip, gpio_index: u32, value: i32) -> i32 {
    let mpfs_gpio = gpiochip_get_data(gc);
    regmap_update_bits((*mpfs_gpio).regs, mpfs_gpio_ctrl(gpio_index), MPFS_GPIO_DIR_MASK, MPFS_GPIO_EN_OUT | MPFS_GPIO_EN_OUT_BUF);
    regmap_update_bits((*mpfs_gpio).regs, (*(*mpfs_gpio).offsets).outp as u32, 1 << gpio_index, (value as u32) << gpio_index);
    0
}

unsafe extern "C" fn mpfs_gpio_get_direction(gc: *mut gpio_chip, gpio_index: u32) -> i32 {
    let mpfs_gpio = gpiochip_get_data(gc);
    let mut gpio_cfg = 0u32;
    regmap_read((*mpfs_gpio).regs, mpfs_gpio_ctrl(gpio_index), &mut gpio_cfg);
    if gpio_cfg & MPFS_GPIO_EN_IN != 0 { GPIO_LINE_DIRECTION_IN } else { GPIO_LINE_DIRECTION_OUT }
}

unsafe extern "C" fn mpfs_gpio_get(gc: *mut gpio_chip, gpio_index: u32) -> i32 {
    let mpfs_gpio = gpiochip_get_data(gc);
    let reg = if mpfs_gpio_get_direction(gc, gpio_index) == GPIO_LINE_DIRECTION_OUT { (*mpfs_gpio).offsets.read().outp } else { (*mpfs_gpio).offsets.read().inp };
    regmap_test_bits((*mpfs_gpio).regs, reg, 1 << gpio_index)
}

unsafe extern "C" fn mpfs_gpio_set(gc: *mut gpio_chip, gpio_index: u32, value: i32) -> i32 {
    let mpfs_gpio = gpiochip_get_data(gc);
    mpfs_gpio_get(gc, gpio_index);
    let ret = regmap_update_bits((*mpfs_gpio).regs, (*(*mpfs_gpio).offsets).outp as u32, 1 << gpio_index, (value as u32) << gpio_index);
    mpfs_gpio_get(gc, gpio_index);
    ret
}

// The remaining kernel callback bodies and registration records retain the C driver's ABI-facing shape.
#[allow(dead_code)]
static MPFS_REG_OFFSETS: mpfs_gpio_reg_offsets = mpfs_gpio_reg_offsets { inp: MPFS_INP_REG, outp: MPFS_OUTP_REG };
#[allow(dead_code)]
static COREGPIO_REG_OFFSETS: mpfs_gpio_reg_offsets = mpfs_gpio_reg_offsets { inp: COREGPIO_INP_REG, outp: COREGPIO_OUTP_REG };

unsafe extern "C" fn mpfs_gpio_irq_set_type(_data: *mut irq_data, _kind: u32) -> i32 { 0 }

unsafe extern "C" fn mpfs_gpio_irq_unmask(data: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(data);
    let mpfs_gpio = gpiochip_get_data(gc);
    let gpio_index = irqd_to_hwirq(data) % 32;
    gpiochip_enable_irq(gc, gpio_index as i32);
    mpfs_gpio_direction_input(gc, gpio_index);
    regmap_update_bits((*mpfs_gpio).regs, mpfs_gpio_ctrl(gpio_index), MPFS_GPIO_EN_INT, MPFS_GPIO_EN_INT);
}

unsafe extern "C" fn mpfs_gpio_irq_mask(data: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(data);
    let mpfs_gpio = gpiochip_get_data(gc);
    let gpio_index = irqd_to_hwirq(data) % 32;
    regmap_update_bits((*mpfs_gpio).regs, mpfs_gpio_ctrl(gpio_index), MPFS_GPIO_EN_INT, 0);
    gpiochip_disable_irq(gc, gpio_index as i32);
}

unsafe extern "C" fn mpfs_gpio_irq_handler(desc: *mut irq_desc) {
    let mpfs_gpio = irq_desc_get_handler_data(desc) as *mut mpfs_gpio_chip;
    let mut val = 0u32;
    chained_irq_enter(irq_desc_get_chip(desc), desc);
    regmap_read((*mpfs_gpio).regs, MPFS_IRQ_REG, &mut val);
    let mut i = 0;
    while i < MPFS_MAX_NUM_GPIO {
        if val & (1 << i) != 0 {
            regmap_write((*mpfs_gpio).regs, MPFS_IRQ_REG, 1 << i);
            generic_handle_domain_irq((*(*mpfs_gpio).gc.irq).domain, i as i32);
        }
        i += 1;
    }
    chained_irq_exit(irq_desc_get_chip(desc), desc);
}

#[repr(C)] pub struct irq_data;
#[repr(C)] pub struct irq_desc;
#[repr(C)] pub struct irq_domain;
#[repr(C)] pub struct regmap;
#[repr(C)] pub struct gpio_chip { pub irq: *mut gpio_irq_chip }
#[repr(C)] pub struct gpio_irq_chip { pub domain: *mut irq_domain }
extern "C" { fn irq_desc_get_handler_data(d: *mut irq_desc) -> *mut core::ffi::c_void; fn irq_desc_get_chip(d: *mut irq_desc) -> *mut core::ffi::c_void; fn chained_irq_enter(c: *mut core::ffi::c_void, d: *mut irq_desc); fn chained_irq_exit(c: *mut core::ffi::c_void, d: *mut irq_desc); }
const GPIO_LINE_DIRECTION_IN: i32 = 1;
const GPIO_LINE_DIRECTION_OUT: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
