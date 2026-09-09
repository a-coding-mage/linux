// SPDX-License-Identifier: GPL-2.0-only
/*
 * sl28cpld GPIO driver
 *
 * Copyright 2020 Michael Walle <michael@walle.cc>
 */

// Linux kernel dependencies are supplied by the surrounding tree.

const GPIO_REG_DIR: u32 = 0x00;
const GPIO_REG_OUT: u32 = 0x01;
const GPIO_REG_IN: u32 = 0x02;
const GPIO_REG_IE: u32 = 0x03;
const GPIO_REG_IP: u32 = 0x04;

const GPI_REG_IN: u32 = 0x00;
const GPO_REG_OUT: u32 = 0x00;

#[repr(C)]
#[derive(Copy, Clone)]
enum Sl28cpldGpioType {
    SL28CPLD_GPIO = 1,
    SL28CPLD_GPI,
    SL28CPLD_GPO,
}

static SL28CPLD_GPIO_IRQS: [RegmapIrq; 8] = [
    REGMAP_IRQ_REG_LINE!(0, 8),
    REGMAP_IRQ_REG_LINE!(1, 8),
    REGMAP_IRQ_REG_LINE!(2, 8),
    REGMAP_IRQ_REG_LINE!(3, 8),
    REGMAP_IRQ_REG_LINE!(4, 8),
    REGMAP_IRQ_REG_LINE!(5, 8),
    REGMAP_IRQ_REG_LINE!(6, 8),
    REGMAP_IRQ_REG_LINE!(7, 8),
];

unsafe fn sl28cpld_gpio_irq_init(
    pdev: *mut PlatformDevice,
    base: u32,
    config: *mut GpioRegmapConfig,
) -> i32 {
    let mut irq_data: *mut RegmapIrqChipData = core::ptr::null_mut();
    let irq_chip: *mut RegmapIrqChip;
    let dev: *mut Device = &mut (*pdev).dev;
    let irq: i32;
    let ret: i32;

    if !device_property_read_bool(dev, c"interrupt-controller") {
        return 0;
    }

    irq = platform_get_irq(pdev, 0);
    if irq < 0 {
        return irq;
    }

    irq_chip = devm_kzalloc(dev, core::mem::size_of::<RegmapIrqChip>(), GFP_KERNEL);
    if irq_chip.is_null() {
        return -ENOMEM;
    }

    (*irq_chip).name = c"sl28cpld-gpio-irq";
    (*irq_chip).irqs = SL28CPLD_GPIO_IRQS.as_ptr();
    (*irq_chip).num_irqs = SL28CPLD_GPIO_IRQS.len();
    (*irq_chip).num_regs = 1;
    (*irq_chip).status_base = base + GPIO_REG_IP;
    (*irq_chip).unmask_base = base + GPIO_REG_IE;
    (*irq_chip).ack_base = base + GPIO_REG_IP;

    ret = devm_regmap_add_irq_chip_fwnode(
        dev, dev_fwnode(dev), (*config).regmap, irq,
        IRQF_SHARED | IRQF_ONESHOT, 0, irq_chip, &mut irq_data,
    );
    if ret != 0 {
        return ret;
    }

    (*config).irq_domain = regmap_irq_get_domain(irq_data);
    0
}

unsafe fn sl28cpld_gpio_probe(pdev: *mut PlatformDevice) -> i32 {
    let mut config: GpioRegmapConfig = core::mem::zeroed();
    let mut type_: Sl28cpldGpioType;
    let regmap: *mut Regmap;
    let mut base: u32 = 0;
    let ret: i32;

    if (*pdev).dev.parent.is_null() {
        return -ENODEV;
    }

    type_ = device_get_match_data(&mut (*pdev).dev) as usize as Sl28cpldGpioType;
    if !type_ as bool {
        return -ENODEV;
    }

    ret = device_property_read_u32(&mut (*pdev).dev, c"reg", &mut base);
    if ret != 0 {
        return -EINVAL;
    }

    regmap = dev_get_regmap((*pdev).dev.parent, core::ptr::null());
    if regmap.is_null() {
        return -ENODEV;
    }

    config.regmap = regmap;
    config.parent = &mut (*pdev).dev;
    config.ngpio = 8;

    match type_ {
        Sl28cpldGpioType::SL28CPLD_GPIO => {
            config.reg_dat_base = base + GPIO_REG_IN;
            config.reg_set_base = base + GPIO_REG_OUT;
            config.reg_dir_out_base = GPIO_REGMAP_ADDR!(base + GPIO_REG_DIR);
            ret = sl28cpld_gpio_irq_init(pdev, base, &mut config);
            if ret != 0 { return ret; }
        }
        Sl28cpldGpioType::SL28CPLD_GPO => config.reg_set_base = base + GPO_REG_OUT,
        Sl28cpldGpioType::SL28CPLD_GPI => config.reg_dat_base = base + GPI_REG_IN,
        _ => {
            dev_err!(&mut (*pdev).dev, "unknown type {}\n", type_ as i32);
            return -ENODEV;
        }
    }

    PTR_ERR_OR_ZERO!(devm_gpio_regmap_register(&mut (*pdev).dev, &mut config))
}

static SL28CPLD_GPIO_OF_MATCH: [OfDeviceId; 4] = [
    OfDeviceId { compatible: c"kontron,sl28cpld-gpio", data: Sl28cpldGpioType::SL28CPLD_GPIO as *const _ },
    OfDeviceId { compatible: c"kontron,sl28cpld-gpi", data: Sl28cpldGpioType::SL28CPLD_GPI as *const _ },
    OfDeviceId { compatible: c"kontron,sl28cpld-gpo", data: Sl28cpldGpioType::SL28CPLD_GPO as *const _ },
    OfDeviceId::default(),
];

static mut SL28CPLD_GPIO_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(sl28cpld_gpio_probe),
    driver: Driver { name: c"sl28cpld-gpio", of_match_table: SL28CPLD_GPIO_OF_MATCH.as_ptr() },
};

module_platform_driver!(SL28CPLD_GPIO_DRIVER);
module_device_table!(of, SL28CPLD_GPIO_OF_MATCH);
module_description!("sl28cpld GPIO Driver");
module_author!("Michael Walle <michael@walle.cc>");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
