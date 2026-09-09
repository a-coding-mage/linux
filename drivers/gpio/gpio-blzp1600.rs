// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2019 VeriSilicon Limited.
 * Copyright (C) 2025 Blaize, Inc.
 */

// Translated from the Linux kernel C implementation. Kernel headers and
// externally supplied symbols remain dependencies of this implementation.

const GPIO_DIR_REG: usize = 0x00;
const GPIO_CTRL_REG: usize = 0x04;
const GPIO_SET_REG: usize = 0x08;
const GPIO_CLR_REG: usize = 0x0C;
const GPIO_ODATA_REG: usize = 0x10;
const GPIO_IDATA_REG: usize = 0x14;
const GPIO_IEN_REG: usize = 0x18;
const GPIO_IS_REG: usize = 0x1C;
const GPIO_IBE_REG: usize = 0x20;
const GPIO_IEV_REG: usize = 0x24;
const GPIO_RIS_REG: usize = 0x28;
const GPIO_IM_REG: usize = 0x2C;
const GPIO_MIS_REG: usize = 0x30;
const GPIO_IC_REG: usize = 0x34;
const GPIO_DB_REG: usize = 0x38;
const GPIO_DFG_REG: usize = 0x3C;

const DRIVER_NAME: &str = "blzp1600-gpio";

#[repr(C)]
struct Blzp1600Gpio {
    base: *mut core::ffi::c_void,
    gen_gc: gpio_generic_chip,
    irq: i32,
}

unsafe extern "C" {
    fn gpiochip_get_data(gc: *mut gpio_chip) -> *mut core::ffi::c_void;
    fn irq_data_get_irq_chip_data(d: *mut irq_data) -> *mut gpio_chip;
    fn irq_desc_get_handler_data(d: *mut irq_desc) -> *mut gpio_chip;
    fn readl_relaxed(addr: *const core::ffi::c_void) -> u32;
    fn writel_relaxed(val: u32, addr: *mut core::ffi::c_void);
}

unsafe fn get_blzp1600_gpio_from_irq_data(d: *mut irq_data) -> *mut Blzp1600Gpio {
    gpiochip_get_data(irq_data_get_irq_chip_data(d)) as *mut Blzp1600Gpio
}

unsafe fn get_blzp1600_gpio_from_irq_desc(d: *mut irq_desc) -> *mut Blzp1600Gpio {
    gpiochip_get_data(irq_desc_get_handler_data(d)) as *mut Blzp1600Gpio
}

unsafe fn blzp1600_gpio_read(chip: *mut Blzp1600Gpio, offset: usize) -> u32 {
    readl_relaxed((*chip).base.byte_add(offset))
}

unsafe fn blzp1600_gpio_write(chip: *mut Blzp1600Gpio, offset: usize, val: u32) {
    writel_relaxed(val, (*chip).base.byte_add(offset))
}

unsafe fn blzp1600_gpio_rmw(reg: *mut core::ffi::c_void, mask: u32, set: bool) {
    let mut val = readl_relaxed(reg);
    if set { val |= mask; } else { val &= !mask; }
    writel_relaxed(val, reg);
}

unsafe fn blzp1600_gpio_irq_mask(d: *mut irq_data) {
    let chip = get_blzp1600_gpio_from_irq_data(d);
    let _guard = guard_gpio_generic_lock_irqsave(&mut (*chip).gen_gc);
    blzp1600_gpio_rmw((*chip).base.byte_add(GPIO_IM_REG), 1u32 << (*d).hwirq, true);
}

unsafe fn blzp1600_gpio_irq_unmask(d: *mut irq_data) {
    let chip = get_blzp1600_gpio_from_irq_data(d);
    let _guard = guard_gpio_generic_lock_irqsave(&mut (*chip).gen_gc);
    blzp1600_gpio_rmw((*chip).base.byte_add(GPIO_IM_REG), 1u32 << (*d).hwirq, false);
}

unsafe fn blzp1600_gpio_irq_ack(d: *mut irq_data) {
    let chip = get_blzp1600_gpio_from_irq_data(d);
    blzp1600_gpio_write(chip, GPIO_IC_REG, 1u32 << (*d).hwirq);
}

unsafe fn blzp1600_gpio_irq_enable(d: *mut irq_data) {
    let chip = get_blzp1600_gpio_from_irq_data(d);
    gpiochip_enable_irq(&mut (*chip).gen_gc.gc, irqd_to_hwirq(d));
    let _guard = guard_gpio_generic_lock_irqsave(&mut (*chip).gen_gc);
    blzp1600_gpio_rmw((*chip).base.byte_add(GPIO_DIR_REG), 1u32 << (*d).hwirq, false);
    blzp1600_gpio_rmw((*chip).base.byte_add(GPIO_IEN_REG), 1u32 << (*d).hwirq, true);
}

unsafe fn blzp1600_gpio_irq_disable(d: *mut irq_data) {
    let chip = get_blzp1600_gpio_from_irq_data(d);
    let _guard = guard_gpio_generic_lock_irqsave(&mut (*chip).gen_gc);
    blzp1600_gpio_rmw((*chip).base.byte_add(GPIO_IEN_REG), 1u32 << (*d).hwirq, false);
    gpiochip_disable_irq(&mut (*chip).gen_gc.gc, irqd_to_hwirq(d));
}

unsafe fn blzp1600_gpio_irq_set_type(d: *mut irq_data, type_: u32) -> i32 {
    let chip = get_blzp1600_gpio_from_irq_data(d);
    let mask = 1u32 << (*d).hwirq;
    let _guard = guard_gpio_generic_lock_irqsave(&mut (*chip).gen_gc);
    let mut edge_level = blzp1600_gpio_read(chip, GPIO_IS_REG);
    let mut single_both = blzp1600_gpio_read(chip, GPIO_IBE_REG);
    let mut fall_rise = blzp1600_gpio_read(chip, GPIO_IEV_REG);
    match type_ {
        IRQ_TYPE_EDGE_BOTH => { edge_level &= !mask; single_both |= mask; }
        IRQ_TYPE_EDGE_RISING => { edge_level &= !mask; single_both &= !mask; fall_rise |= mask; }
        IRQ_TYPE_EDGE_FALLING => { edge_level &= !mask; single_both &= !mask; fall_rise &= !mask; }
        IRQ_TYPE_LEVEL_HIGH => { edge_level |= mask; fall_rise |= mask; }
        IRQ_TYPE_LEVEL_LOW => { edge_level |= mask; fall_rise &= !mask; }
        _ => return -EINVAL,
    }
    blzp1600_gpio_write(chip, GPIO_IS_REG, edge_level);
    blzp1600_gpio_write(chip, GPIO_IBE_REG, single_both);
    blzp1600_gpio_write(chip, GPIO_IEV_REG, fall_rise);
    if type_ & IRQ_TYPE_LEVEL_MASK != 0 { irq_set_handler_locked(d, handle_level_irq); }
    else { irq_set_handler_locked(d, handle_edge_irq); }
    0
}

unsafe fn blzp1600_gpio_irqhandler(desc: *mut irq_desc) {
    let gpio = get_blzp1600_gpio_from_irq_desc(desc);
    let irqchip = irq_desc_get_chip(desc);
    chained_irq_enter(irqchip, desc);
    let irq_status = blzp1600_gpio_read(gpio, GPIO_RIS_REG) as usize;
    let mut hwirq = 0usize;
    while hwirq < (*gpio).gen_gc.gc.ngpio as usize {
        if irq_status & (1usize << hwirq) != 0 {
            generic_handle_domain_irq((*gpio).gen_gc.gc.irq.domain, hwirq as u32);
        }
        hwirq += 1;
    }
    chained_irq_exit(irqchip, desc);
}

unsafe fn blzp1600_gpio_set_debounce(gc: *mut gpio_chip, offset: u32, debounce: u32) -> i32 {
    let chip = gpiochip_get_data(gc) as *mut Blzp1600Gpio;
    let _guard = guard_gpio_generic_lock_irqsave(&mut (*chip).gen_gc);
    blzp1600_gpio_rmw((*chip).base.byte_add(GPIO_DB_REG), 1u32 << offset, debounce != 0);
    0
}

unsafe fn blzp1600_gpio_set_config(gc: *mut gpio_chip, offset: u32, config: u64) -> i32 {
    if pinconf_to_config_param(config) != PIN_CONFIG_INPUT_DEBOUNCE { return -ENOTSUPP; }
    blzp1600_gpio_set_debounce(gc, offset, pinconf_to_config_argument(config))
}

unsafe fn blzp1600_gpio_probe(pdev: *mut platform_device) -> i32 {
    let mut chip = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<Blzp1600Gpio>(), GFP_KERNEL) as *mut Blzp1600Gpio;
    if chip.is_null() { return -ENOMEM; }
    (*chip).base = devm_platform_ioremap_resource(pdev, 0);
    if is_err((*chip).base) { return ptr_err((*chip).base) as i32; }
    let config = gpio_generic_chip_config {
        dev: &mut (*pdev).dev, sz: 4,
        dat: (*chip).base.byte_add(GPIO_IDATA_REG), set: (*chip).base.byte_add(GPIO_SET_REG),
        clr: (*chip).base.byte_add(GPIO_CLR_REG), dirout: (*chip).base.byte_add(GPIO_DIR_REG),
    };
    let ret = gpio_generic_chip_init(&mut (*chip).gen_gc, &config);
    if ret != 0 { return dev_err_probe(&mut (*pdev).dev, ret, "Failed to register generic gpio\n"); }
    (*chip).gen_gc.gc.set_config = Some(blzp1600_gpio_set_config);
    if device_property_present(&mut (*pdev).dev, "interrupt-controller") {
        (*chip).irq = platform_get_irq(pdev, 0);
        if (*chip).irq < 0 { return (*chip).irq; }
        let girq = &mut (*chip).gen_gc.gc.irq;
        gpio_irq_chip_set_chip(girq, &blzp1600_gpio_irqchip);
        girq.parent_handler = Some(blzp1600_gpio_irqhandler);
        girq.num_parents = 1;
        girq.parents = devm_kcalloc(&mut (*pdev).dev, 1, core::mem::size_of::<i32>(), GFP_KERNEL);
        if girq.parents.is_null() { return -ENOMEM; }
        *girq.parents = (*chip).irq;
        girq.default_type = IRQ_TYPE_NONE;
    }
    devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*chip).gen_gc.gc, chip)
}

#[no_mangle]
pub static mut blzp1600_gpio_driver: platform_driver = platform_driver {
    driver: driver { name: DRIVER_NAME, of_match_table: blzp1600_gpio_of_match },
    probe: Some(blzp1600_gpio_probe),
};

#[no_mangle]
pub static blzp1600_gpio_of_match: [of_device_id; 2] = [
    of_device_id { compatible: "blaize,blzp1600-gpio" },
    of_device_id { compatible: core::ptr::null() },
];

// Equivalent of module_platform_driver(blzp1600_gpio_driver).
// MODULE_DEVICE_TABLE(of, blzp1600_gpio_of_match);
// MODULE_AUTHOR("Nikolaos Pasaloukos <nikolaos.pasaloukos@blaize.com>");
// MODULE_DESCRIPTION("Blaize BLZP1600 GPIO driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
