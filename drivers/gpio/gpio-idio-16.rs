// SPDX-License-Identifier: GPL-2.0
/*
 * GPIO library for the ACCES IDIO-16 family
 * Copyright (C) 2022 William Breathitt Gray
 */

// C headers and build-time symbol namespace omitted; required kernel symbols
// are declared externally below.

const IDIO_16_DAT_BASE: u32 = 0x0;
const IDIO_16_OUT_BASE: u32 = IDIO_16_DAT_BASE;
const IDIO_16_IN_BASE: u32 = IDIO_16_DAT_BASE + 1;
const IDIO_16_CLEAR_INTERRUPT: u32 = 0x1;
const IDIO_16_ENABLE_IRQ: u32 = 0x2;
const IDIO_16_DEACTIVATE_INPUT_FILTERS: u32 = 0x3;
const IDIO_16_DISABLE_IRQ: u32 = IDIO_16_ENABLE_IRQ;
const IDIO_16_INTERRUPT_STATUS: u32 = 0x6;

const IDIO_16_NGPIO: usize = 32;
const IDIO_16_NGPIO_PER_REG: u32 = 8;
const IDIO_16_REG_STRIDE: u32 = 4;

#[repr(C)]
pub struct idio_16_data {
    pub map: *mut regmap,
    pub irq_mask: u32,
}

#[repr(C)]
pub struct device;
#[repr(C)]
pub struct regmap;
#[repr(C)]
pub struct gpio_regmap;
#[repr(C)]
pub struct regmap_irq_chip;
#[repr(C)]
pub struct regmap_irq_chip_data;
#[repr(C)]
pub struct idio_16_regmap_config {
    pub parent: *mut device,
    pub map: *mut regmap,
    pub regmap_irqs: *mut core::ffi::c_void,
    pub no_status: bool,
    pub irq: u32,
    pub num_regmap_irqs: u32,
    pub filters: bool,
}

#[repr(C)]
pub enum gpio_regmap_operation { _Opaque }

extern "C" {
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> i32;
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> i32;
    fn devm_kzalloc(dev: *const device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn dev_name(dev: *const device) -> *const core::ffi::c_char;
    fn devm_regmap_add_irq_chip(dev: *const device, map: *mut regmap, irq: u32, flags: u32,
        type_: u32, chip: *mut regmap_irq_chip, data: *mut *mut regmap_irq_chip_data) -> i32;
    fn dev_err_probe(dev: *const device, err: i32, fmt: *const core::ffi::c_char) -> i32;
    fn regmap_irq_get_domain(data: *mut regmap_irq_chip_data) -> *mut core::ffi::c_void;
    fn devm_gpio_regmap_register(dev: *const device, config: *mut gpio_regmap_config) -> i32;
    fn gpio_regmap_addr(addr: u32) -> usize;
    fn ptr_err_or_zero(ptr: *mut core::ffi::c_void) -> i32;
}

#[repr(C)]
struct gpio_regmap_config {
    parent: *mut device,
    regmap: *mut regmap,
    ngpio: usize,
    names: *const *const core::ffi::c_char,
    reg_dat_base: usize,
    reg_set_base: usize,
    ngpio_per_reg: u32,
    reg_stride: u32,
    irq_domain: *mut core::ffi::c_void,
    reg_mask_xlate: Option<unsafe extern "C" fn(*mut gpio_regmap, gpio_regmap_operation, u32, u32, *mut u32, *mut u32) -> i32>,
    fixed_direction_output: *const u64,
}

// Opaque kernel structure fields represented here to preserve assignments made by C.
#[repr(C)]
struct regmap_irq_chip_fields {
    name: *const core::ffi::c_char,
    status_base: u32,
    mask_base: u32,
    ack_base: u32,
    no_status: bool,
    num_regs: u32,
    irqs: *mut core::ffi::c_void,
    num_irqs: u32,
    handle_mask_sync: Option<unsafe extern "C" fn(i32, u32, u32, *mut core::ffi::c_void) -> i32>,
    irq_drv_data: *mut core::ffi::c_void,
}

unsafe extern "C" fn idio_16_handle_mask_sync(_index: i32, mask_buf_def: u32,
    mask_buf: u32, irq_drv_data: *mut core::ffi::c_void) -> i32 {
    let data = &mut *(irq_drv_data as *mut idio_16_data);
    let prev_mask = data.irq_mask;
    if mask_buf == prev_mask { return 0; }
    data.irq_mask = mask_buf;
    if prev_mask == mask_buf_def {
        let err = regmap_write(data.map, IDIO_16_CLEAR_INTERRUPT, 0x00);
        if err != 0 { return err; }
        let mut val = 0;
        return regmap_read(data.map, IDIO_16_ENABLE_IRQ, &mut val);
    }
    if mask_buf == mask_buf_def { return regmap_write(data.map, IDIO_16_DISABLE_IRQ, 0x00); }
    0
}

unsafe extern "C" fn idio_16_reg_mask_xlate(_gpio: *mut gpio_regmap, _op: gpio_regmap_operation,
    _base: u32, offset: u32, reg: *mut u32, mask: *mut u32) -> i32 {
    let stride;
    if offset < 16 {
        stride = offset / IDIO_16_NGPIO_PER_REG;
        *reg = IDIO_16_OUT_BASE + stride * IDIO_16_REG_STRIDE;
    } else {
        stride = (offset - 16) / IDIO_16_NGPIO_PER_REG;
        *reg = IDIO_16_IN_BASE + stride * IDIO_16_REG_STRIDE;
    }
    *mask = 1u32 << (offset % IDIO_16_NGPIO_PER_REG);
    0
}

static IDIO_16_NAMES: [&[u8]; IDIO_16_NGPIO] = [
    b"OUT0\0", b"OUT1\0", b"OUT2\0", b"OUT3\0", b"OUT4\0", b"OUT5\0", b"OUT6\0", b"OUT7\0",
    b"OUT8\0", b"OUT9\0", b"OUT10\0", b"OUT11\0", b"OUT12\0", b"OUT13\0", b"OUT14\0", b"OUT15\0",
    b"IIN0\0", b"IIN1\0", b"IIN2\0", b"IIN3\0", b"IIN4\0", b"IIN5\0", b"IIN6\0", b"IIN7\0",
    b"IIN8\0", b"IIN9\0", b"IIN10\0", b"IIN11\0", b"IIN12\0", b"IIN13\0", b"IIN14\0", b"IIN15\0",
];

// devm_idio_16_regmap_register - Register an IDIO-16 GPIO device
pub unsafe extern "C" fn devm_idio_16_regmap_register(dev: *mut device,
    config: *const idio_16_regmap_config) -> i32 {
    if (*config).parent.is_null() || (*config).map.is_null() || (*config).regmap_irqs.is_null() { return -22; }
    let data = devm_kzalloc(dev, core::mem::size_of::<idio_16_data>(), 0) as *mut idio_16_data;
    if data.is_null() { return -12; }
    (*data).map = (*config).map;
    let chip = devm_kzalloc(dev, core::mem::size_of::<regmap_irq_chip>(), 0) as *mut regmap_irq_chip;
    if chip.is_null() { return -12; }
    let chip = chip as *mut regmap_irq_chip_fields;
    (*chip).name = dev_name(dev);
    (*chip).status_base = IDIO_16_INTERRUPT_STATUS;
    (*chip).mask_base = IDIO_16_ENABLE_IRQ;
    (*chip).ack_base = IDIO_16_CLEAR_INTERRUPT;
    (*chip).no_status = (*config).no_status;
    (*chip).num_regs = 1;
    (*chip).irqs = (*config).regmap_irqs;
    (*chip).num_irqs = (*config).num_regmap_irqs;
    (*chip).handle_mask_sync = Some(idio_16_handle_mask_sync);
    (*chip).irq_drv_data = data as *mut core::ffi::c_void;
    let err = regmap_write((*data).map, IDIO_16_DISABLE_IRQ, 0x00);
    if err != 0 { return err; }
    let mut chip_data = core::ptr::null_mut();
    let err = devm_regmap_add_irq_chip(dev, (*data).map, (*config).irq, 0, 0,
        chip as *mut regmap_irq_chip, &mut chip_data);
    if err != 0 { return dev_err_probe(dev, err, b"IRQ registration failed\n\0".as_ptr() as _); }
    if (*config).filters {
        let err = regmap_write((*data).map, IDIO_16_DEACTIVATE_INPUT_FILTERS, 0x00);
        if err != 0 { return err; }
    }
    let mut gpio_config: gpio_regmap_config = core::mem::zeroed();
    gpio_config.parent = (*config).parent; gpio_config.regmap = (*data).map; gpio_config.ngpio = IDIO_16_NGPIO;
    gpio_config.names = IDIO_16_NAMES.as_ptr() as *const *const core::ffi::c_char;
    gpio_config.reg_dat_base = gpio_regmap_addr(IDIO_16_DAT_BASE); gpio_config.reg_set_base = gpio_regmap_addr(IDIO_16_DAT_BASE);
    gpio_config.ngpio_per_reg = IDIO_16_NGPIO_PER_REG; gpio_config.reg_stride = IDIO_16_REG_STRIDE;
    gpio_config.irq_domain = regmap_irq_get_domain(chip_data); gpio_config.reg_mask_xlate = Some(idio_16_reg_mask_xlate);
    let fixed_direction_output = 0xffffu64;
    gpio_config.fixed_direction_output = &fixed_direction_output;
    ptr_err_or_zero(devm_gpio_regmap_register(dev, &mut gpio_config))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
