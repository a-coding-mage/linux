// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2020 HiSilicon Limited. */

// Translated from the Linux kernel GPIO driver implementation.

const HISI_GPIO_SWPORT_DR_SET_WX: u32 = 0x000;
const HISI_GPIO_SWPORT_DR_CLR_WX: u32 = 0x004;
const HISI_GPIO_SWPORT_DDR_SET_WX: u32 = 0x010;
const HISI_GPIO_SWPORT_DDR_CLR_WX: u32 = 0x014;
const HISI_GPIO_SWPORT_DDR_ST_WX: u32 = 0x018;
const HISI_GPIO_INTEN_SET_WX: u32 = 0x020;
const HISI_GPIO_INTEN_CLR_WX: u32 = 0x024;
const HISI_GPIO_INTMASK_SET_WX: u32 = 0x030;
const HISI_GPIO_INTMASK_CLR_WX: u32 = 0x034;
const HISI_GPIO_INTTYPE_EDGE_SET_WX: u32 = 0x040;
const HISI_GPIO_INTTYPE_EDGE_CLR_WX: u32 = 0x044;
const HISI_GPIO_INT_POLARITY_SET_WX: u32 = 0x050;
const HISI_GPIO_INT_POLARITY_CLR_WX: u32 = 0x054;
const HISI_GPIO_DEBOUNCE_SET_WX: u32 = 0x060;
const HISI_GPIO_DEBOUNCE_CLR_WX: u32 = 0x064;
const HISI_GPIO_INTSTATUS_WX: u32 = 0x070;
const HISI_GPIO_PORTA_EOI_WX: u32 = 0x078;
const HISI_GPIO_EXT_PORT_WX: u32 = 0x080;
const HISI_GPIO_INTCOMB_MASK_WX: u32 = 0x0a0;
const HISI_GPIO_INT_DEDGE_SET: u32 = 0x0b0;
const HISI_GPIO_INT_DEDGE_CLR: u32 = 0x0b4;
const HISI_GPIO_INT_DEDGE_ST: u32 = 0x0b8;

const HISI_GPIO_LINE_NUM_MAX: u32 = 32;
const HISI_GPIO_DRIVER_NAME: &str = "gpio-hisi";

#[repr(C)]
struct HisiGpio {
    chip: gpio_generic_chip,
    dev: *mut device,
    reg_base: *mut core::ffi::c_void,
    line_num: u32,
    irq: i32,
}

unsafe fn hisi_gpio_read_reg(chip: *mut gpio_chip, off: u32) -> u32 {
    let hisi_gpio = container_of(to_gpio_generic_chip(chip), HisiGpio, chip);
    let reg = (hisi_gpio.reg_base as *mut u8).add(off as usize) as *mut u32;
    readl(reg)
}

unsafe fn hisi_gpio_write_reg(chip: *mut gpio_chip, off: u32, val: u32) {
    let hisi_gpio = container_of(to_gpio_generic_chip(chip), HisiGpio, chip);
    let reg = (hisi_gpio.reg_base as *mut u8).add(off as usize) as *mut u32;
    writel(val, reg);
}

unsafe fn hisi_gpio_set_debounce(chip: *mut gpio_chip, off: u32, debounce: u32) {
    if debounce != 0 {
        hisi_gpio_write_reg(chip, HISI_GPIO_DEBOUNCE_SET_WX, 1u32 << off);
    } else {
        hisi_gpio_write_reg(chip, HISI_GPIO_DEBOUNCE_CLR_WX, 1u32 << off);
    }
}

unsafe fn hisi_gpio_set_config(
    chip: *mut gpio_chip,
    offset: u32,
    config: libc::c_ulong,
) -> i32 {
    let config_para = pinconf_to_config_param(config);
    let config_arg: u32;

    match config_para {
        PIN_CONFIG_INPUT_DEBOUNCE => {
            config_arg = pinconf_to_config_argument(config);
            hisi_gpio_set_debounce(chip, offset, config_arg);
        }
        _ => return -ENOTSUPP,
    }

    0
}

unsafe fn hisi_gpio_set_ack(d: *mut irq_data) {
    let chip = irq_data_get_irq_chip_data(d);
    hisi_gpio_write_reg(chip, HISI_GPIO_PORTA_EOI_WX, 1u32 << irqd_to_hwirq(d));
}

unsafe fn hisi_gpio_irq_set_mask(d: *mut irq_data) {
    let chip = irq_data_get_irq_chip_data(d);
    hisi_gpio_write_reg(chip, HISI_GPIO_INTMASK_SET_WX, 1u32 << irqd_to_hwirq(d));
    gpiochip_disable_irq(chip, irqd_to_hwirq(d));
}

unsafe fn hisi_gpio_irq_clr_mask(d: *mut irq_data) {
    let chip = irq_data_get_irq_chip_data(d);
    gpiochip_enable_irq(chip, irqd_to_hwirq(d));
    hisi_gpio_write_reg(chip, HISI_GPIO_INTMASK_CLR_WX, 1u32 << irqd_to_hwirq(d));
}

unsafe fn hisi_gpio_irq_set_type(d: *mut irq_data, irq_type: u32) -> i32 {
    let chip = irq_data_get_irq_chip_data(d);
    let mask = 1u32 << irqd_to_hwirq(d);

    match irq_type {
        IRQ_TYPE_EDGE_BOTH => hisi_gpio_write_reg(chip, HISI_GPIO_INT_DEDGE_SET, mask),
        IRQ_TYPE_EDGE_RISING => {
            hisi_gpio_write_reg(chip, HISI_GPIO_INTTYPE_EDGE_SET_WX, mask);
            hisi_gpio_write_reg(chip, HISI_GPIO_INT_POLARITY_SET_WX, mask);
        }
        IRQ_TYPE_EDGE_FALLING => {
            hisi_gpio_write_reg(chip, HISI_GPIO_INTTYPE_EDGE_SET_WX, mask);
            hisi_gpio_write_reg(chip, HISI_GPIO_INT_POLARITY_CLR_WX, mask);
        }
        IRQ_TYPE_LEVEL_HIGH => {
            hisi_gpio_write_reg(chip, HISI_GPIO_INTTYPE_EDGE_CLR_WX, mask);
            hisi_gpio_write_reg(chip, HISI_GPIO_INT_POLARITY_SET_WX, mask);
        }
        IRQ_TYPE_LEVEL_LOW => {
            hisi_gpio_write_reg(chip, HISI_GPIO_INTTYPE_EDGE_CLR_WX, mask);
            hisi_gpio_write_reg(chip, HISI_GPIO_INT_POLARITY_CLR_WX, mask);
        }
        _ => return -EINVAL,
    }

    /* The dual-edge interrupt registers have higher priority and must be
     * disabled before configuring other interrupt types. */
    if irq_type != IRQ_TYPE_EDGE_BOTH {
        let both = hisi_gpio_read_reg(chip, HISI_GPIO_INT_DEDGE_ST);
        if both & mask != 0 {
            hisi_gpio_write_reg(chip, HISI_GPIO_INT_DEDGE_CLR, mask);
        }
    }

    if irq_type & IRQ_TYPE_LEVEL_MASK != 0 {
        irq_set_handler_locked(d, handle_level_irq);
    } else if irq_type & IRQ_TYPE_EDGE_BOTH != 0 {
        irq_set_handler_locked(d, handle_edge_irq);
    }
    0
}

unsafe fn hisi_gpio_irq_enable(d: *mut irq_data) {
    let chip = irq_data_get_irq_chip_data(d);
    hisi_gpio_irq_clr_mask(d);
    hisi_gpio_write_reg(chip, HISI_GPIO_INTEN_SET_WX, 1u32 << irqd_to_hwirq(d));
}

unsafe fn hisi_gpio_irq_disable(d: *mut irq_data) {
    let chip = irq_data_get_irq_chip_data(d);
    hisi_gpio_irq_set_mask(d);
    hisi_gpio_write_reg(chip, HISI_GPIO_INTEN_CLR_WX, 1u32 << irqd_to_hwirq(d));
}

unsafe fn hisi_gpio_irq_handler(desc: *mut irq_desc) {
    let hisi_gpio = irq_desc_get_handler_data(desc) as *mut HisiGpio;
    let irq_msk = hisi_gpio_read_reg(&mut (*hisi_gpio).chip.gc, HISI_GPIO_INTSTATUS_WX);
    let irq_c = irq_desc_get_chip(desc);
    chained_irq_enter(irq_c, desc);
    for hwirq in 0..HISI_GPIO_LINE_NUM_MAX {
        if irq_msk & (1u32 << hwirq) != 0 {
            generic_handle_domain_irq((*hisi_gpio).chip.gc.irq.domain, hwirq);
        }
    }
    chained_irq_exit(irq_c, desc);
}

static HISI_GPIO_IRQ_CHIP: irq_chip = irq_chip {
    name: "HISI-GPIO",
    irq_ack: Some(hisi_gpio_set_ack),
    irq_mask: Some(hisi_gpio_irq_set_mask),
    irq_unmask: Some(hisi_gpio_irq_clr_mask),
    irq_set_type: Some(hisi_gpio_irq_set_type),
    irq_enable: Some(hisi_gpio_irq_enable),
    irq_disable: Some(hisi_gpio_irq_disable),
    flags: IRQCHIP_IMMUTABLE,
};

unsafe fn hisi_gpio_init_irq(hisi_gpio: *mut HisiGpio) {
    let chip = &mut (*hisi_gpio).chip.gc;
    let girq_chip = &mut chip.irq;
    gpio_irq_chip_set_chip(girq_chip, &HISI_GPIO_IRQ_CHIP);
    girq_chip.default_type = IRQ_TYPE_NONE;
    girq_chip.num_parents = 1;
    girq_chip.parents = &mut (*hisi_gpio).irq;
    girq_chip.parent_handler = Some(hisi_gpio_irq_handler);
    girq_chip.parent_handler_data = hisi_gpio as *mut _;
    hisi_gpio_write_reg(chip, HISI_GPIO_INTCOMB_MASK_WX, 1);
}

static HISI_GPIO_ACPI_MATCH: [acpi_device_id; 2] = [
    acpi_device_id { id: "HISI0184", driver_data: 0 },
    acpi_device_id { id: "", driver_data: 0 },
];

static HISI_GPIO_DTS_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: "hisilicon,ascend910-gpio" },
    of_device_id { compatible: "" },
];

unsafe fn hisi_gpio_get_pdata(dev: *mut device, hisi_gpio: *mut HisiGpio) {
    let pdev = to_platform_device(dev);
    let mut idx: i32 = 0;
    let mut fwnode: *mut fwnode_handle = core::ptr::null_mut();

    while device_for_each_child_node(dev, &mut fwnode) {
        if fwnode_property_read_u32(fwnode, "ngpios", &mut (*hisi_gpio).line_num) != 0 {
            dev_err(dev, "failed to get number of lines for port%d and use default value instead\n", idx);
            (*hisi_gpio).line_num = HISI_GPIO_LINE_NUM_MAX;
        }
        if WARN_ON((*hisi_gpio).line_num > HISI_GPIO_LINE_NUM_MAX) {
            (*hisi_gpio).line_num = HISI_GPIO_LINE_NUM_MAX;
        }
        (*hisi_gpio).irq = platform_get_irq(pdev, idx);
        dev_info(dev, "get hisi_gpio[%d] with %u lines\n", idx, (*hisi_gpio).line_num);
        idx += 1;
    }
}

unsafe fn hisi_gpio_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let mut config: gpio_generic_chip_config;
    let hisi_gpio = devm_kzalloc(dev, core::mem::size_of::<HisiGpio>(), GFP_KERNEL)
        as *mut HisiGpio;
    if hisi_gpio.is_null() {
        return -ENOMEM;
    }

    let port_num = device_get_child_node_count(dev);
    if WARN_ON(port_num != 1) {
        return -ENODEV;
    }

    (*hisi_gpio).reg_base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*hisi_gpio).reg_base) {
        return PTR_ERR((*hisi_gpio).reg_base);
    }
    hisi_gpio_get_pdata(dev, hisi_gpio);
    (*hisi_gpio).dev = dev;

    config = gpio_generic_chip_config {
        dev: (*hisi_gpio).dev,
        sz: 4,
        dat: ((*hisi_gpio).reg_base as *mut u8).add(HISI_GPIO_EXT_PORT_WX as usize),
        set: ((*hisi_gpio).reg_base as *mut u8).add(HISI_GPIO_SWPORT_DR_SET_WX as usize),
        clr: ((*hisi_gpio).reg_base as *mut u8).add(HISI_GPIO_SWPORT_DR_CLR_WX as usize),
        dirout: ((*hisi_gpio).reg_base as *mut u8).add(HISI_GPIO_SWPORT_DDR_SET_WX as usize),
        dirin: ((*hisi_gpio).reg_base as *mut u8).add(HISI_GPIO_SWPORT_DDR_CLR_WX as usize),
        flags: GPIO_GENERIC_NO_SET_ON_INPUT | GPIO_GENERIC_UNREADABLE_REG_DIR,
    };

    let mut ret = gpio_generic_chip_init(&mut (*hisi_gpio).chip, &mut config);
    if ret != 0 {
        dev_err(dev, "failed to init, ret = %d\n", ret);
        return ret;
    }
    (*hisi_gpio).chip.gc.set_config = Some(hisi_gpio_set_config);
    (*hisi_gpio).chip.gc.ngpio = (*hisi_gpio).line_num;
    (*hisi_gpio).chip.gc.base = -1;
    if (*hisi_gpio).irq > 0 {
        hisi_gpio_init_irq(hisi_gpio);
    }
    ret = devm_gpiochip_add_data(dev, &mut (*hisi_gpio).chip.gc, hisi_gpio as *mut _);
    if ret != 0 {
        dev_err(dev, "failed to register gpiochip, ret = %d\n", ret);
        return ret;
    }
    0
}

static HISI_GPIO_DRIVER: platform_driver = platform_driver {
    driver: driver {
        name: HISI_GPIO_DRIVER_NAME,
        acpi_match_table: &HISI_GPIO_ACPI_MATCH,
        of_match_table: &HISI_GPIO_DTS_MATCH,
    },
    probe: Some(hisi_gpio_probe),
};

module_platform_driver!(HISI_GPIO_DRIVER);
module_license!("GPL");
module_author!("Luo Jiaxing <luojiaxing@huawei.com>");
module_description!("HiSilicon GPIO controller driver");
module_alias!(concat!("platform:", HISI_GPIO_DRIVER_NAME));

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
