// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2025 Bootlin
 *
 * Author: Kamel BOUHARA <kamel.bouhara@bootlin.com>
 * Author: Mathieu Dubois-Briand <mathieu.dubois-briand@bootlin.com>
 */

// Linux kernel dependencies supplied by other translation units.

const MAX7360_GPIO_PORT: u32 = 1;
const MAX7360_GPIO_COL: u32 = 2;

#[repr(C)]
struct max7360_gpio_plat_data {
    function: u32,
}

static mut max7360_gpio_port_plat: max7360_gpio_plat_data =
    max7360_gpio_plat_data { function: MAX7360_GPIO_PORT };
static mut max7360_gpio_col_plat: max7360_gpio_plat_data =
    max7360_gpio_plat_data { function: MAX7360_GPIO_COL };

unsafe fn max7360_get_available_gpos(dev: *mut device, available_gpios: *mut u32) -> i32 {
    let mut columns: u32 = 0;
    let ret = device_property_read_u32((*dev).parent, c"keypad,num-columns".as_ptr(), &mut columns);
    if ret != 0 {
        dev_err(dev, c"Failed to read columns count\n".as_ptr());
        return ret;
    }

    *available_gpios = core::cmp::min(MAX7360_MAX_GPO, MAX7360_MAX_KEY_COLS - columns);
    0
}

unsafe fn max7360_gpo_init_valid_mask(
    gc: *mut gpio_chip,
    valid_mask: *mut core::ffi::c_ulong,
    _ngpios: u32,
) -> i32 {
    let mut available_gpios = 0;
    let ret = max7360_get_available_gpos((*gc).parent, &mut available_gpios);
    if ret != 0 { return ret; }
    bitmap_clear(valid_mask, 0, MAX7360_MAX_KEY_COLS - available_gpios);
    0
}

unsafe fn max7360_set_gpos_count(dev: *mut device, regmap: *mut regmap) -> i32 {
    /* MAX7360 COL0 to COL7 pins can be used as keypad columns, GPIOs, or both. */
    let mut available_gpios = 0;
    let ret = max7360_get_available_gpos(dev, &mut available_gpios);
    if ret != 0 { return ret; }
    let val = field_prep(MAX7360_PORTS, available_gpios);
    let ret = regmap_write_bits(regmap, MAX7360_REG_DEBOUNCE, MAX7360_PORTS, val);
    if ret != 0 { dev_err(dev, c"Failed to write max7360 columns/gpos configuration".as_ptr()); }
    ret
}

unsafe fn max7360_gpio_reg_mask_xlate(
    _gpio: *mut gpio_regmap,
    _op: gpio_regmap_operation,
    base: u32,
    offset: u32,
    reg: *mut u32,
    mask: *mut u32,
) -> i32 {
    if base == MAX7360_REG_PWMBASE {
        *reg = base + offset;
        *mask = genmask(7, 0);
    } else {
        *reg = base;
        *mask = 1u32 << offset;
    }
    0
}

static max7360_regmap_irqs: [regmap_irq; MAX7360_MAX_GPIO as usize] = [
    REGMAP_IRQ_REG!(0, 0, 1 << 0), REGMAP_IRQ_REG!(1, 0, 1 << 1),
    REGMAP_IRQ_REG!(2, 0, 1 << 2), REGMAP_IRQ_REG!(3, 0, 1 << 3),
    REGMAP_IRQ_REG!(4, 0, 1 << 4), REGMAP_IRQ_REG!(5, 0, 1 << 5),
    REGMAP_IRQ_REG!(6, 0, 1 << 6), REGMAP_IRQ_REG!(7, 0, 1 << 7),
];

unsafe fn max7360_handle_mask_sync(
    _index: i32, _mask_buf_def: u32, mask_buf: u32, irq_drv_data: *mut core::ffi::c_void,
) -> i32 {
    let regmap = gpio_regmap_get_drvdata(irq_drv_data);
    for i in 0..MAX7360_MAX_GPIO {
        let ret = regmap_assign_bits(regmap, max7360_reg_pwmcfg(i), MAX7360_PORT_CFG_INTERRUPT_MASK, mask_buf & (1 << i));
        if ret != 0 { return ret; }
    }
    0
}

unsafe fn max7360_gpio_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let regmap = dev_get_regmap((*dev).parent, core::ptr::null());
    if regmap.is_null() { return dev_err_probe(dev, -ENODEV, c"could not get parent regmap\n".as_ptr()); }
    let plat_data = device_get_match_data(dev);
    let mut gpio_config: gpio_regmap_config = core::mem::zeroed();
    if (*plat_data).function == MAX7360_GPIO_PORT {
        if device_property_read_bool(dev, c"interrupt-controller".as_ptr()) {
            gpio_config.regmap_irq_flags = IRQF_ONESHOT | IRQF_SHARED;
            gpio_config.regmap_irq_line = fwnode_irq_get_byname(dev_fwnode((*dev).parent), c"inti".as_ptr());
            if gpio_config.regmap_irq_line < 0 { return dev_err_probe(dev, gpio_config.regmap_irq_line, c"Failed to get IRQ\n".as_ptr()); }
            let irq_chip = devm_kzalloc(dev, core::mem::size_of::<regmap_irq_chip>(), GFP_KERNEL) as *mut regmap_irq_chip;
            if irq_chip.is_null() { return -ENOMEM; }
            (*irq_chip).name = dev_name(dev); (*irq_chip).status_base = MAX7360_REG_GPIOIN;
            (*irq_chip).status_is_level = true; (*irq_chip).num_regs = 1; (*irq_chip).num_irqs = MAX7360_MAX_GPIO;
            (*irq_chip).irqs = max7360_regmap_irqs.as_ptr(); (*irq_chip).handle_mask_sync = Some(max7360_handle_mask_sync);
            gpio_config.regmap_irq_chip = irq_chip; gpio_config.drvdata = regmap;
            for i in 0..MAX7360_MAX_GPIO {
                let ret = regmap_write_bits(regmap, max7360_reg_pwmcfg(i), MAX7360_PORT_CFG_INTERRUPT_EDGES, MAX7360_PORT_CFG_INTERRUPT_EDGES);
                if ret != 0 { return dev_err_probe(dev, ret, c"Failed to enable interrupts\n".as_ptr()); }
            }
        }
        let mut outconf = 0;
        let ret = device_property_read_u32(dev, c"maxim,constant-current-disable".as_ptr(), &mut outconf);
        if ret == 0 { let ret = regmap_write(regmap, MAX7360_REG_GPIOOUTM, outconf); if ret != 0 { return dev_err_probe(dev, ret, c"Failed to set constant-current configuration\n".as_ptr()); } }
    }
    gpio_config.parent = dev; gpio_config.regmap = regmap;
    if (*plat_data).function == MAX7360_GPIO_PORT {
        gpio_config.ngpio = MAX7360_MAX_GPIO; gpio_config.reg_dat_base = GPIO_REGMAP_ADDR!(MAX7360_REG_GPIOIN);
        gpio_config.reg_set_base = GPIO_REGMAP_ADDR!(MAX7360_REG_PWMBASE); gpio_config.reg_dir_out_base = GPIO_REGMAP_ADDR!(MAX7360_REG_GPIOCTRL);
        gpio_config.ngpio_per_reg = MAX7360_MAX_GPIO; gpio_config.reg_mask_xlate = Some(max7360_gpio_reg_mask_xlate);
    } else {
        let ret = max7360_set_gpos_count(dev, regmap); if ret != 0 { return dev_err_probe(dev, ret, c"Failed to set GPOS pin count\n".as_ptr()); }
        gpio_config.reg_set_base = GPIO_REGMAP_ADDR!(MAX7360_REG_PORTS); gpio_config.ngpio = MAX7360_MAX_KEY_COLS;
        gpio_config.init_valid_mask = Some(max7360_gpo_init_valid_mask);
    }
    ptr_err_or_zero(devm_gpio_regmap_register(dev, &mut gpio_config))
}

// Device matching, platform-driver registration, and module metadata are supplied by the kernel bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
