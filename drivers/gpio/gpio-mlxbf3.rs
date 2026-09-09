// SPDX-License-Identifier: GPL-2.0-only OR BSD-3-Clause
/* Copyright (C) 2022 NVIDIA CORPORATION & AFFILIATES */

// Linux kernel dependencies supplied externally.

/*
 * There are 2 YU GPIO blocks:
 * gpio[0]: HOST_GPIO0->HOST_GPIO31
 * gpio[1]: HOST_GPIO32->HOST_GPIO55
 */
const MLXBF3_GPIO_MAX_PINS_PER_BLOCK: u32 = 32;
const MLXBF3_GPIO_MAX_PINS_BLOCK0: u32 = 32;
const MLXBF3_GPIO_MAX_PINS_BLOCK1: u32 = 24;

/* fw_gpio[x] block registers and their offset */
const MLXBF_GPIO_FW_OUTPUT_ENABLE_SET: usize = 0x00;
const MLXBF_GPIO_FW_DATA_OUT_SET: usize = 0x04;
const MLXBF_GPIO_FW_OUTPUT_ENABLE_CLEAR: usize = 0x00;
const MLXBF_GPIO_FW_DATA_OUT_CLEAR: usize = 0x04;
const MLXBF_GPIO_CAUSE_RISE_EN: usize = 0x00;
const MLXBF_GPIO_CAUSE_FALL_EN: usize = 0x04;
const MLXBF_GPIO_READ_DATA_IN: usize = 0x08;
const MLXBF_GPIO_CAUSE_OR_CAUSE_EVTEN0: usize = 0x00;
const MLXBF_GPIO_CAUSE_OR_EVTEN0: usize = 0x14;
const MLXBF_GPIO_CAUSE_OR_CLRCAUSE: usize = 0x18;
const MLXBF_GPIO_CLR_ALL_INTS: u32 = 0xffff_ffff;

#[repr(C)]
struct Mlxbf3GpioContext {
    chip: gpio_generic_chip,
    /* YU GPIO block address */
    gpio_set_io: *mut core::ffi::c_void,
    gpio_clr_io: *mut core::ffi::c_void,
    gpio_io: *mut core::ffi::c_void,
    /* YU GPIO cause block address */
    gpio_cause_io: *mut core::ffi::c_void,
}

unsafe fn mlxbf3_gpio_irq_enable(irqd: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(irqd);
    let gs = gpiochip_get_data(gc);
    let offset = irqd_to_hwirq(irqd);
    let mut val: u32;

    gpiochip_enable_irq(gc, offset);
    gpio_generic_lock_irqsave(&mut (*gs).chip);

    writel(1u32.wrapping_shl(offset), (*gs).gpio_cause_io.add(MLXBF_GPIO_CAUSE_OR_CLRCAUSE));
    val = readl((*gs).gpio_cause_io.add(MLXBF_GPIO_CAUSE_OR_EVTEN0));
    val |= 1u32.wrapping_shl(offset);
    writel(val, (*gs).gpio_cause_io.add(MLXBF_GPIO_CAUSE_OR_EVTEN0));
    gpio_generic_unlock_irqrestore(&mut (*gs).chip);
}

unsafe fn mlxbf3_gpio_irq_disable(irqd: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(irqd);
    let gs = gpiochip_get_data(gc);
    let offset = irqd_to_hwirq(irqd);
    let mut val = readl((*gs).gpio_cause_io.add(MLXBF_GPIO_CAUSE_OR_EVTEN0));
    gpio_generic_lock_irqsave(&mut (*gs).chip);
    val &= !(1u32.wrapping_shl(offset));
    writel(val, (*gs).gpio_cause_io.add(MLXBF_GPIO_CAUSE_OR_EVTEN0));
    writel(1u32.wrapping_shl(offset), (*gs).gpio_cause_io.add(MLXBF_GPIO_CAUSE_OR_CLRCAUSE));
    gpio_generic_unlock_irqrestore(&mut (*gs).chip);
    gpiochip_disable_irq(gc, offset);
}

unsafe fn mlxbf3_gpio_irq_handler(_irq: i32, ptr: *mut core::ffi::c_void) -> irqreturn_t {
    let gs = ptr as *mut Mlxbf3GpioContext;
    let gc = &mut (*gs).chip.gc;
    let pending = readl((*gs).gpio_cause_io.add(MLXBF_GPIO_CAUSE_OR_CAUSE_EVTEN0));
    writel(pending, (*gs).gpio_cause_io.add(MLXBF_GPIO_CAUSE_OR_CLRCAUSE));
    for level in 0..gc.ngpio {
        if (pending & (1u32 << level)) != 0 {
            generic_handle_domain_irq(gc.irq.domain, level);
        }
    }
    irq_retval(pending)
}

unsafe fn mlxbf3_gpio_irq_set_type(irqd: *mut irq_data, type_: u32) -> i32 {
    let gc = irq_data_get_irq_chip_data(irqd);
    let gs = gpiochip_get_data(gc);
    let offset = irqd_to_hwirq(irqd);
    let mut val: u32;
    gpio_generic_lock_irqsave(&mut (*gs).chip);
    match type_ & IRQ_TYPE_SENSE_MASK {
        IRQ_TYPE_EDGE_BOTH => {
            val = readl((*gs).gpio_io.add(MLXBF_GPIO_CAUSE_FALL_EN));
            writel(val | (1u32 << offset), (*gs).gpio_io.add(MLXBF_GPIO_CAUSE_FALL_EN));
            val = readl((*gs).gpio_io.add(MLXBF_GPIO_CAUSE_RISE_EN));
            writel(val | (1u32 << offset), (*gs).gpio_io.add(MLXBF_GPIO_CAUSE_RISE_EN));
        }
        IRQ_TYPE_EDGE_RISING => {
            val = readl((*gs).gpio_io.add(MLXBF_GPIO_CAUSE_RISE_EN));
            writel(val | (1u32 << offset), (*gs).gpio_io.add(MLXBF_GPIO_CAUSE_RISE_EN));
        }
        IRQ_TYPE_EDGE_FALLING => {
            val = readl((*gs).gpio_io.add(MLXBF_GPIO_CAUSE_FALL_EN));
            writel(val | (1u32 << offset), (*gs).gpio_io.add(MLXBF_GPIO_CAUSE_FALL_EN));
        }
        _ => { gpio_generic_unlock_irqrestore(&mut (*gs).chip); return -EINVAL; }
    }
    gpio_generic_unlock_irqrestore(&mut (*gs).chip);
    irq_set_handler_locked(irqd, handle_edge_irq);
    0
}

/* This function needs to be defined for handle_edge_irq() */
unsafe fn mlxbf3_gpio_irq_ack(_data: *mut irq_data) {}

unsafe fn mlxbf3_gpio_add_pin_ranges(chip: *mut gpio_chip) -> i32 {
    let id = match (*chip).ngpio {
        MLXBF3_GPIO_MAX_PINS_BLOCK0 => 0,
        MLXBF3_GPIO_MAX_PINS_BLOCK1 => 1,
        _ => return -EINVAL,
    };
    gpiochip_add_pin_range(chip, c"MLNXBF34:00".as_ptr(), (*chip).base,
        id * MLXBF3_GPIO_MAX_PINS_PER_BLOCK, (*chip).ngpio)
}

unsafe fn mlxbf3_gpio_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let gs = devm_kzalloc(dev, core::mem::size_of::<Mlxbf3GpioContext>(), GFP_KERNEL)
        as *mut Mlxbf3GpioContext;
    if gs.is_null() { return -ENOMEM; }
    (*gs).gpio_io = devm_platform_ioremap_resource(pdev, 0);
    if is_err((*gs).gpio_io) { return ptr_err((*gs).gpio_io); }
    (*gs).gpio_cause_io = devm_platform_ioremap_resource(pdev, 1);
    if is_err((*gs).gpio_cause_io) { return ptr_err((*gs).gpio_cause_io); }
    (*gs).gpio_set_io = devm_platform_ioremap_resource(pdev, 2);
    if is_err((*gs).gpio_set_io) { return ptr_err((*gs).gpio_set_io); }
    (*gs).gpio_clr_io = devm_platform_ioremap_resource(pdev, 3);
    if is_err((*gs).gpio_clr_io) { return ptr_err((*gs).gpio_clr_io); }
    let gc = &mut (*gs).chip.gc;
    let config = gpio_generic_chip_config {
        dev, sz: 4, dat: (*gs).gpio_io.add(MLXBF_GPIO_READ_DATA_IN),
        set: (*gs).gpio_set_io.add(MLXBF_GPIO_FW_DATA_OUT_SET),
        clr: (*gs).gpio_clr_io.add(MLXBF_GPIO_FW_DATA_OUT_CLEAR),
        dirout: (*gs).gpio_set_io.add(MLXBF_GPIO_FW_OUTPUT_ENABLE_SET),
        dirin: (*gs).gpio_clr_io.add(MLXBF_GPIO_FW_OUTPUT_ENABLE_CLEAR),
    };
    let ret = gpio_generic_chip_init(&mut (*gs).chip, &config);
    if ret != 0 { return dev_err_probe(dev, ret, c"%s: failed to initialize the generic GPIO chip".as_ptr(), c"mlxbf3_gpio_probe".as_ptr()); }
    (*gc).request = gpiochip_generic_request;
    (*gc).free = gpiochip_generic_free;
    (*gc).owner = THIS_MODULE;
    (*gc).add_pin_ranges = mlxbf3_gpio_add_pin_ranges;
    let irq = platform_get_irq_optional(pdev, 0);
    if irq >= 0 { let ret = devm_request_irq(dev, irq, mlxbf3_gpio_irq_handler, IRQF_SHARED, dev_name(dev), gs as *mut _); if ret != 0 { return dev_err_probe(dev, ret, c"failed to request IRQ".as_ptr()); } }
    platform_set_drvdata(pdev, gs as *mut _);
    let ret = devm_gpiochip_add_data(dev, gc, gs as *mut _);
    if ret != 0 { return dev_err_probe(dev, ret, c"Failed adding memory mapped gpiochip\n".as_ptr()); }
    0
}

unsafe fn mlxbf3_gpio_shutdown(pdev: *mut platform_device) {
    let gs = platform_get_drvdata(pdev) as *mut Mlxbf3GpioContext;
    writel(0, (*gs).gpio_cause_io.add(MLXBF_GPIO_CAUSE_OR_EVTEN0));
    writel(MLXBF_GPIO_CLR_ALL_INTS, (*gs).gpio_cause_io.add(MLXBF_GPIO_CAUSE_OR_CLRCAUSE));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
