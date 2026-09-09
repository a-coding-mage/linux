// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Promontory GPIO driver
 *
 * Copyright (C) 2015 ASMedia Technology Inc.
 * Author: YD Tseng <yd_tseng@asmedia.com.tw>
 */

// Linux kernel dependencies supplied by the surrounding kernel/Rust bindings.

const PT_TOTAL_GPIO: u32 = 8;
const PT_TOTAL_GPIO_EX: u32 = 24;

/* PCI-E MMIO register offsets */
const PT_DIRECTION_REG: usize = 0x00;
const PT_INPUTDATA_REG: usize = 0x04;
const PT_OUTPUTDATA_REG: usize = 0x08;
const PT_CLOCKRATE_REG: usize = 0x0C;
const PT_SYNC_REG: usize = 0x28;

#[repr(C)]
struct PtGpioChip {
    chip: gpio_generic_chip,
    reg_base: *mut core::ffi::c_void,
}

unsafe fn pt_gpio_request(gc: *mut gpio_chip, offset: u32) -> i32 {
    let gen_gc = to_gpio_generic_chip(gc);
    let pt_gpio = gpiochip_get_data(gc) as *mut PtGpioChip;
    let using_pins: u32;

    dev_dbg((*gc).parent, "pt_gpio_request offset=%x\n", offset);

    // guard(gpio_generic_lock_irqsave)(gen_gc);
    let _lock = gpio_generic_lock_irqsave(gen_gc);

    using_pins = readl((*pt_gpio).reg_base.add(PT_SYNC_REG));
    if using_pins & (1u32.wrapping_shl(offset)) != 0 {
        dev_warn((*gc).parent, "PT GPIO pin %x reconfigured\n", offset);
        return -EINVAL;
    }

    writel(
        using_pins | (1u32.wrapping_shl(offset)),
        (*pt_gpio).reg_base.add(PT_SYNC_REG),
    );

    0
}

unsafe fn pt_gpio_free(gc: *mut gpio_chip, offset: u32) {
    let gen_gc = to_gpio_generic_chip(gc);
    let pt_gpio = gpiochip_get_data(gc) as *mut PtGpioChip;
    let using_pins: u32;

    // guard(gpio_generic_lock_irqsave)(gen_gc);
    let _lock = gpio_generic_lock_irqsave(gen_gc);

    using_pins = readl((*pt_gpio).reg_base.add(PT_SYNC_REG));
    let using_pins = using_pins & !(1u32.wrapping_shl(offset));
    writel(using_pins, (*pt_gpio).reg_base.add(PT_SYNC_REG));

    dev_dbg((*gc).parent, "pt_gpio_free offset=%x\n", offset);
}

unsafe fn pt_gpio_probe(pdev: *mut platform_device) -> i32 {
    let mut config: gpio_generic_chip_config;
    let dev = &mut (*pdev).dev as *mut device;
    let pt_gpio: *mut PtGpioChip;
    let mut ret: i32 = 0;

    if !ACPI_COMPANION(dev) {
        dev_err(dev, "PT GPIO device node not found\n");
        return -ENODEV;
    }

    pt_gpio = devm_kzalloc(dev, core::mem::size_of::<PtGpioChip>(), GFP_KERNEL)
        as *mut PtGpioChip;
    if pt_gpio.is_null() {
        return -ENOMEM;
    }

    (*pt_gpio).reg_base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*pt_gpio).reg_base) {
        dev_err(dev, "Failed to map MMIO resource for PT GPIO.\n");
        return PTR_ERR((*pt_gpio).reg_base);
    }

    config = gpio_generic_chip_config {
        dev,
        sz: 4,
        dat: (*pt_gpio).reg_base.add(PT_INPUTDATA_REG),
        set: (*pt_gpio).reg_base.add(PT_OUTPUTDATA_REG),
        dirout: (*pt_gpio).reg_base.add(PT_DIRECTION_REG),
        flags: GPIO_GENERIC_READ_OUTPUT_REG_SET,
    };

    ret = gpio_generic_chip_init(&mut (*pt_gpio).chip, &mut config);
    if ret != 0 {
        dev_err(dev, "failed to initialize the generic GPIO chip\n");
        return ret;
    }

    (*pt_gpio).chip.gc.owner = THIS_MODULE;
    (*pt_gpio).chip.gc.request = Some(pt_gpio_request);
    (*pt_gpio).chip.gc.free = Some(pt_gpio_free);
    (*pt_gpio).chip.gc.ngpio = device_get_match_data(dev) as usize;

    ret = devm_gpiochip_add_data(dev, &mut (*pt_gpio).chip.gc, pt_gpio as *mut _);
    if ret != 0 {
        dev_err(dev, "Failed to register GPIO lib\n");
        return ret;
    }

    platform_set_drvdata(pdev, pt_gpio as *mut _);

    /* initialize register setting */
    writel(0, (*pt_gpio).reg_base.add(PT_SYNC_REG));
    writel(0, (*pt_gpio).reg_base.add(PT_CLOCKRATE_REG));

    dev_dbg(dev, "PT GPIO driver loaded\n");
    ret
}

static PT_GPIO_ACPI_MATCH: [acpi_device_id; 4] = [
    acpi_device_id { id: "AMDF030", driver_data: PT_TOTAL_GPIO as usize },
    acpi_device_id { id: "AMDIF030", driver_data: PT_TOTAL_GPIO as usize },
    acpi_device_id { id: "AMDIF031", driver_data: PT_TOTAL_GPIO_EX as usize },
    acpi_device_id::default(),
];

static mut PT_GPIO_DRIVER: platform_driver = platform_driver {
    driver: driver {
        name: "pt-gpio",
        acpi_match_table: ACPI_PTR(&PT_GPIO_ACPI_MATCH),
    },
    probe: Some(pt_gpio_probe),
};

module_platform_driver!(PT_GPIO_DRIVER);

MODULE_LICENSE!("GPL");
MODULE_AUTHOR!("YD Tseng <yd_tseng@asmedia.com.tw>");
MODULE_DESCRIPTION!("AMD Promontory GPIO Driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
