// SPDX-License-Identifier: GPL-2.0-only
/*
 * MAXIM MAX77620 GPIO driver
 *
 * Copyright (c) 2016, NVIDIA CORPORATION.  All rights reserved.
 */

// Dependencies supplied by the kernel translation environment:
// linux/gpio/driver.h, linux/interrupt.h, linux/mfd/max77620.h,
// linux/module.h, linux/platform_device.h, linux/regmap.h

#[inline]
fn gpio_reg_addr(offset: unsigned_int) -> unsigned_int {
    MAX77620_REG_GPIO0 + offset
}

#[repr(C)]
struct max77620_gpio {
    gpio_chip: gpio_chip,
    rmap: *mut regmap,
    dev: *mut device,
    buslock: mutex, // irq_bus_lock
    irq_type: [unsigned_int; MAX77620_GPIO_NR],
    irq_enabled: [bool; MAX77620_GPIO_NR],
}

unsafe fn max77620_gpio_irqhandler(irq: int, data: *mut c_void) -> irqreturn_t {
    let gpio = data as *mut max77620_gpio;
    let mut value: unsigned_int = 0;
    let mut offset: unsigned_int;
    let mut pending: unsigned_long;
    let err: int;

    err = regmap_read((*gpio).rmap, MAX77620_REG_IRQ_LVL2_GPIO, &mut value);
    if err < 0 {
        dev_err((*gpio).dev, "REG_IRQ_LVL2_GPIO read failed: %d\n", err);
        return IRQ_NONE;
    }

    pending = value as unsigned_long;

    for_each_set_bit!(offset, &mut pending, MAX77620_GPIO_NR) {
        let virq: unsigned_int;

        virq = irq_find_mapping((*gpio).gpio_chip.irq.domain, offset);
        handle_nested_irq(virq);
    }

    IRQ_HANDLED
}

unsafe fn max77620_gpio_irq_mask(data: *mut irq_data) {
    let chip = irq_data_get_irq_chip_data(data);
    let gpio = gpiochip_get_data(chip) as *mut max77620_gpio;

    (*gpio).irq_enabled[(*data).hwirq as usize] = false;
    gpiochip_disable_irq(chip, (*data).hwirq);
}

unsafe fn max77620_gpio_irq_unmask(data: *mut irq_data) {
    let chip = irq_data_get_irq_chip_data(data);
    let gpio = gpiochip_get_data(chip) as *mut max77620_gpio;

    gpiochip_enable_irq(chip, (*data).hwirq);
    (*gpio).irq_enabled[(*data).hwirq as usize] = true;
}

unsafe fn max77620_gpio_set_irq_type(data: *mut irq_data, type_: unsigned_int) -> int {
    let chip = irq_data_get_irq_chip_data(data);
    let gpio = gpiochip_get_data(chip) as *mut max77620_gpio;
    let irq_type: unsigned_int;

    irq_type = match type_ {
        IRQ_TYPE_EDGE_RISING => MAX77620_CNFG_GPIO_INT_RISING,
        IRQ_TYPE_EDGE_FALLING => MAX77620_CNFG_GPIO_INT_FALLING,
        IRQ_TYPE_EDGE_BOTH => {
            MAX77620_CNFG_GPIO_INT_RISING | MAX77620_CNFG_GPIO_INT_FALLING
        }
        _ => return -EINVAL,
    };

    (*gpio).irq_type[(*data).hwirq as usize] = irq_type;
    0
}

unsafe fn max77620_gpio_bus_lock(data: *mut irq_data) {
    let chip = irq_data_get_irq_chip_data(data);
    let gpio = gpiochip_get_data(chip) as *mut max77620_gpio;

    mutex_lock(&mut (*gpio).buslock);
}

unsafe fn max77620_gpio_bus_sync_unlock(data: *mut irq_data) {
    let chip = irq_data_get_irq_chip_data(data);
    let gpio = gpiochip_get_data(chip) as *mut max77620_gpio;
    let offset = (*data).hwirq;
    let value: unsigned_int;
    let err: int;

    value = if (*gpio).irq_enabled[offset as usize] {
        (*gpio).irq_type[offset as usize]
    } else {
        0
    };

    err = regmap_update_bits(
        (*gpio).rmap,
        gpio_reg_addr(offset),
        MAX77620_CNFG_GPIO_INT_MASK,
        value,
    );
    if err < 0 {
        dev_err((*chip).parent, "failed to update interrupt mask: %d\n", err);
    }

    mutex_unlock(&mut (*gpio).buslock);
}

static MAX77620_GPIO_IRQCHIP: irq_chip = irq_chip {
    name: "max77620-gpio",
    irq_mask: Some(max77620_gpio_irq_mask),
    irq_unmask: Some(max77620_gpio_irq_unmask),
    irq_set_type: Some(max77620_gpio_set_irq_type),
    irq_bus_lock: Some(max77620_gpio_bus_lock),
    irq_bus_sync_unlock: Some(max77620_gpio_bus_sync_unlock),
    flags: IRQCHIP_IMMUTABLE | IRQCHIP_MASK_ON_SUSPEND,
    // GPIOCHIP_IRQ_RESOURCE_HELPERS
};

unsafe fn max77620_gpio_get_dir(gc: *mut gpio_chip, offset: unsigned_int) -> int {
    let mgpio = gpiochip_get_data(gc) as *mut max77620_gpio;
    let mut val: unsigned_int = 0;
    let ret = regmap_read((*mgpio).rmap, gpio_reg_addr(offset), &mut val);

    if ret < 0 {
        dev_err((*mgpio).dev, "CNFG_GPIOx read failed: %d\n", ret);
        return ret;
    }

    if val & MAX77620_CNFG_GPIO_DIR_MASK != 0 {
        GPIO_LINE_DIRECTION_IN
    } else {
        GPIO_LINE_DIRECTION_OUT
    }
}

unsafe fn max77620_gpio_dir_input(gc: *mut gpio_chip, offset: unsigned_int) -> int {
    let mgpio = gpiochip_get_data(gc) as *mut max77620_gpio;
    let ret = regmap_update_bits(
        (*mgpio).rmap,
        gpio_reg_addr(offset),
        MAX77620_CNFG_GPIO_DIR_MASK,
        MAX77620_CNFG_GPIO_DIR_INPUT,
    );
    if ret < 0 {
        dev_err((*mgpio).dev, "CNFG_GPIOx dir update failed: %d\n", ret);
    }
    ret
}

unsafe fn max77620_gpio_get(gc: *mut gpio_chip, offset: unsigned_int) -> int {
    let mgpio = gpiochip_get_data(gc) as *mut max77620_gpio;
    let mut val: unsigned_int = 0;
    let ret = regmap_read((*mgpio).rmap, gpio_reg_addr(offset), &mut val);

    if ret < 0 {
        dev_err((*mgpio).dev, "CNFG_GPIOx read failed: %d\n", ret);
        return ret;
    }

    if val & MAX77620_CNFG_GPIO_DIR_MASK != 0 {
        ((val & MAX77620_CNFG_GPIO_INPUT_VAL_MASK) != 0) as int
    } else {
        ((val & MAX77620_CNFG_GPIO_OUTPUT_VAL_MASK) != 0) as int
    }
}

unsafe fn max77620_gpio_dir_output(
    gc: *mut gpio_chip,
    offset: unsigned_int,
    value: int,
) -> int {
    let mgpio = gpiochip_get_data(gc) as *mut max77620_gpio;
    let val: u8 = if value != 0 {
        MAX77620_CNFG_GPIO_OUTPUT_VAL_HIGH
    } else {
        MAX77620_CNFG_GPIO_OUTPUT_VAL_LOW
    };
    let mut ret = regmap_update_bits(
        (*mgpio).rmap,
        gpio_reg_addr(offset),
        MAX77620_CNFG_GPIO_OUTPUT_VAL_MASK,
        val as unsigned_int,
    );
    if ret < 0 {
        dev_err((*mgpio).dev, "CNFG_GPIOx val update failed: %d\n", ret);
        return ret;
    }

    ret = regmap_update_bits(
        (*mgpio).rmap,
        gpio_reg_addr(offset),
        MAX77620_CNFG_GPIO_DIR_MASK,
        MAX77620_CNFG_GPIO_DIR_OUTPUT,
    );
    if ret < 0 {
        dev_err((*mgpio).dev, "CNFG_GPIOx dir update failed: %d\n", ret);
    }
    ret
}

unsafe fn max77620_gpio_set_debounce(
    mgpio: *mut max77620_gpio,
    offset: unsigned_int,
    debounce: unsigned_int,
) -> int {
    let val: u8;

    val = match debounce {
        0 => MAX77620_CNFG_GPIO_DBNC_None,
        1..=8000 => MAX77620_CNFG_GPIO_DBNC_8ms,
        8001..=16000 => MAX77620_CNFG_GPIO_DBNC_16ms,
        16001..=32000 => MAX77620_CNFG_GPIO_DBNC_32ms,
        _ => {
            dev_err((*mgpio).dev, "Illegal value %u\n", debounce);
            return -EINVAL;
        }
    };

    let ret = regmap_update_bits(
        (*mgpio).rmap,
        gpio_reg_addr(offset),
        MAX77620_CNFG_GPIO_DBNC_MASK,
        val as unsigned_int,
    );
    if ret < 0 {
        dev_err((*mgpio).dev, "CNFG_GPIOx_DBNC update failed: %d\n", ret);
    }
    ret
}

unsafe fn max77620_gpio_set(gc: *mut gpio_chip, offset: unsigned_int, value: int) -> int {
    let mgpio = gpiochip_get_data(gc) as *mut max77620_gpio;
    let val = if value != 0 {
        MAX77620_CNFG_GPIO_OUTPUT_VAL_HIGH
    } else {
        MAX77620_CNFG_GPIO_OUTPUT_VAL_LOW
    };

    regmap_update_bits(
        (*mgpio).rmap,
        gpio_reg_addr(offset),
        MAX77620_CNFG_GPIO_OUTPUT_VAL_MASK,
        val as unsigned_int,
    )
}

unsafe fn max77620_gpio_set_config(
    gc: *mut gpio_chip,
    offset: unsigned_int,
    config: unsigned_long,
) -> int {
    let mgpio = gpiochip_get_data(gc) as *mut max77620_gpio;

    match pinconf_to_config_param(config) {
        PIN_CONFIG_DRIVE_OPEN_DRAIN => regmap_update_bits(
            (*mgpio).rmap, gpio_reg_addr(offset), MAX77620_CNFG_GPIO_DRV_MASK,
            MAX77620_CNFG_GPIO_DRV_OPENDRAIN,
        ),
        PIN_CONFIG_DRIVE_PUSH_PULL => regmap_update_bits(
            (*mgpio).rmap, gpio_reg_addr(offset), MAX77620_CNFG_GPIO_DRV_MASK,
            MAX77620_CNFG_GPIO_DRV_PUSHPULL,
        ),
        PIN_CONFIG_INPUT_DEBOUNCE => max77620_gpio_set_debounce(
            mgpio, offset, pinconf_to_config_argument(config),
        ),
        _ => -ENOTSUPP,
    }
}

unsafe fn max77620_gpio_irq_init_hw(gc: *mut gpio_chip) -> int {
    let gpio = gpiochip_get_data(gc) as *mut max77620_gpio;
    let mut i: unsigned_int;

    /*
     * GPIO interrupts may be left ON after bootloader, hence let's
     * pre-initialize hardware to the expected state by disabling all
     * the interrupts.
     */
    i = 0;
    while i < MAX77620_GPIO_NR {
        let err = regmap_update_bits(
            (*gpio).rmap, gpio_reg_addr(i), MAX77620_CNFG_GPIO_INT_MASK, 0,
        );
        if err < 0 {
            dev_err((*gpio).dev, "failed to disable interrupt: %d\n", err);
            return err;
        }
        i += 1;
    }
    0
}

unsafe fn max77620_gpio_probe(pdev: *mut platform_device) -> int {
    let chip = dev_get_drvdata((*pdev).dev.parent) as *mut max77620_chip;
    let mut mgpio: *mut max77620_gpio;
    let girq: *mut gpio_irq_chip;
    let gpio_irq: unsigned_int;
    let mut ret: int;

    ret = platform_get_irq(pdev, 0);
    if ret < 0 {
        return ret;
    }
    gpio_irq = ret as unsigned_int;

    mgpio = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<max77620_gpio>(), GFP_KERNEL)
        as *mut max77620_gpio;
    if mgpio.is_null() {
        return -ENOMEM;
    }

    mutex_init(&mut (*mgpio).buslock);
    (*mgpio).rmap = (*chip).rmap;
    (*mgpio).dev = &mut (*pdev).dev;

    (*mgpio).gpio_chip.label = (*pdev).name;
    (*mgpio).gpio_chip.parent = (*pdev).dev.parent;
    (*mgpio).gpio_chip.get_direction = Some(max77620_gpio_get_dir);
    (*mgpio).gpio_chip.direction_input = Some(max77620_gpio_dir_input);
    (*mgpio).gpio_chip.get = Some(max77620_gpio_get);
    (*mgpio).gpio_chip.direction_output = Some(max77620_gpio_dir_output);
    (*mgpio).gpio_chip.set = Some(max77620_gpio_set);
    (*mgpio).gpio_chip.set_config = Some(max77620_gpio_set_config);
    (*mgpio).gpio_chip.ngpio = MAX77620_GPIO_NR;
    (*mgpio).gpio_chip.can_sleep = true;
    (*mgpio).gpio_chip.base = -1;

    girq = &mut (*mgpio).gpio_chip.irq;
    gpio_irq_chip_set_chip(girq, &MAX77620_GPIO_IRQCHIP);
    /* This will let us handle the parent IRQ in the driver */
    (*girq).parent_handler = None;
    (*girq).num_parents = 0;
    (*girq).parents = core::ptr::null_mut();
    (*girq).default_type = IRQ_TYPE_NONE;
    (*girq).handler = Some(handle_edge_irq);
    (*girq).init_hw = Some(max77620_gpio_irq_init_hw);
    (*girq).threaded = true;

    ret = devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*mgpio).gpio_chip, mgpio as *mut c_void);
    if ret < 0 {
        dev_err(&mut (*pdev).dev, "gpio_init: Failed to add max77620_gpio\n");
        return ret;
    }

    ret = devm_request_threaded_irq(
        &mut (*pdev).dev, gpio_irq, None, Some(max77620_gpio_irqhandler), IRQF_ONESHOT,
        "max77620-gpio", mgpio as *mut c_void,
    );
    if ret < 0 {
        dev_err(&mut (*pdev).dev, "failed to request IRQ: %d\n", ret);
        return ret;
    }
    0
}

static MAX77620_GPIO_DEVTYPE: [platform_device_id; 3] = [
    platform_device_id { name: "max77620-gpio" },
    platform_device_id { name: "max20024-gpio" },
    platform_device_id { name: "" },
];

// MODULE_DEVICE_TABLE(platform, max77620_gpio_devtype);

static mut MAX77620_GPIO_DRIVER: platform_driver = platform_driver {
    driver: driver { name: "max77620-gpio" },
    probe: Some(max77620_gpio_probe),
    id_table: MAX77620_GPIO_DEVTYPE.as_ptr(),
};

// module_platform_driver(max77620_gpio_driver);
// MODULE_DESCRIPTION("GPIO interface for MAX77620 and MAX20024 PMIC");
// MODULE_AUTHOR("Laxman Dewangan <ldewangan@nvidia.com>");
// MODULE_AUTHOR("Chaitanya Bandi <bandik@nvidia.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
