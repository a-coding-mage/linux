// SPDX-License-Identifier: GPL-2.0+
/*
 * Freescale vf610 GPIO support through PORT and GPIO
 *
 * Copyright (c) 2014 Toradex AG.
 *
 * Author: Stefan Agner <stefan@agner.ch>.
 */

const VF610_GPIO_PER_PORT: usize = 32;

#[repr(C)]
pub struct fsl_gpio_soc_data {
    /* SoCs has a Port Data Direction Register (PDDR) */
    pub have_paddr: bool,
    pub have_dual_base: bool,
}

#[repr(C)]
pub struct vf610_gpio_port {
    pub chip: gpio_generic_chip,
    pub base: *mut core::ffi::c_void,
    pub gpio_base: *mut core::ffi::c_void,
    pub sdata: *const fsl_gpio_soc_data,
    pub irqc: [u8; VF610_GPIO_PER_PORT],
    pub clk_port: *mut clk,
    pub clk_gpio: *mut clk,
    pub irq: i32,
}

const GPIO_PDOR: usize = 0x00;
const GPIO_PSOR: usize = 0x04;
const GPIO_PCOR: usize = 0x08;
const GPIO_PTOR: usize = 0x0c;
const GPIO_PDIR: usize = 0x10;
const GPIO_PDDR: usize = 0x14;

#[inline]
const fn PORT_PCR(n: usize) -> usize { n * 0x4 }
const PORT_PCR_IRQC_OFFSET: u32 = 16;

const PORT_ISFR: usize = 0xa0;
const PORT_DFER: usize = 0xc0;
const PORT_DFCR: usize = 0xc4;
const PORT_DFWR: usize = 0xc8;

const PORT_INT_OFF: u8 = 0x0;
const PORT_INT_LOGIC_ZERO: u8 = 0x8;
const PORT_INT_RISING_EDGE: u8 = 0x9;
const PORT_INT_FALLING_EDGE: u8 = 0xa;
const PORT_INT_EITHER_EDGE: u8 = 0xb;
const PORT_INT_LOGIC_ONE: u8 = 0xc;

const IMX8ULP_GPIO_BASE_OFF: usize = 0x40;
const IMX8ULP_BASE_OFF: usize = 0x80;

static vf610_data: fsl_gpio_soc_data = fsl_gpio_soc_data {
    have_paddr: false,
    have_dual_base: true,
};

static imx_data: fsl_gpio_soc_data = fsl_gpio_soc_data {
    have_paddr: true,
    have_dual_base: true,
};

static imx8ulp_data: fsl_gpio_soc_data = fsl_gpio_soc_data {
    have_paddr: true,
    have_dual_base: false,
};

static vf610_gpio_dt_ids: [of_device_id; 4] = [
    of_device_id { compatible: "fsl,vf610-gpio", data: &vf610_data as *const _ as *const core::ffi::c_void },
    of_device_id { compatible: "fsl,imx7ulp-gpio", data: &imx_data as *const _ as *const core::ffi::c_void },
    of_device_id { compatible: "fsl,imx8ulp-gpio", data: &imx8ulp_data as *const _ as *const core::ffi::c_void },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

#[inline]
unsafe fn vf610_gpio_writel(val: u32, reg: *mut core::ffi::c_void) {
    writel_relaxed(val, reg);
}

#[inline]
unsafe fn vf610_gpio_readl(reg: *mut core::ffi::c_void) -> u32 {
    readl_relaxed(reg)
}

unsafe fn vf610_gpio_irq_handler(desc: *mut irq_desc) {
    let port = gpiochip_get_data(irq_desc_get_handler_data(desc)) as *mut vf610_gpio_port;
    let chip = irq_desc_get_chip(desc);
    let mut pin: usize;
    let irq_isfr: usize;

    chained_irq_enter(chip, desc);
    irq_isfr = vf610_gpio_readl((*port).base.add(PORT_ISFR)) as usize;

    for pin in 0..VF610_GPIO_PER_PORT {
        if (irq_isfr & (1usize << pin)) != 0 {
            vf610_gpio_writel(1u32 << pin, (*port).base.add(PORT_ISFR));
            generic_handle_domain_irq((*port).chip.gc.irq.domain, pin as irq_hw_number_t);
        }
    }
    chained_irq_exit(chip, desc);
}

unsafe fn vf610_gpio_irq_ack(d: *mut irq_data) {
    let port = gpiochip_get_data(irq_data_get_irq_chip_data(d)) as *mut vf610_gpio_port;
    let gpio = (*d).hwirq;
    vf610_gpio_writel(1u32 << gpio, (*port).base.add(PORT_ISFR));
}

unsafe fn vf610_gpio_irq_set_type(d: *mut irq_data, type_: u32) -> i32 {
    let port = gpiochip_get_data(irq_data_get_irq_chip_data(d)) as *mut vf610_gpio_port;
    let irqc: u8 = match type_ {
        IRQ_TYPE_EDGE_RISING => PORT_INT_RISING_EDGE,
        IRQ_TYPE_EDGE_FALLING => PORT_INT_FALLING_EDGE,
        IRQ_TYPE_EDGE_BOTH => PORT_INT_EITHER_EDGE,
        IRQ_TYPE_LEVEL_LOW => PORT_INT_LOGIC_ZERO,
        IRQ_TYPE_LEVEL_HIGH => PORT_INT_LOGIC_ONE,
        _ => return -EINVAL,
    };
    (*port).irqc[(*d).hwirq] = irqc;
    if (type_ & IRQ_TYPE_LEVEL_MASK) != 0 {
        irq_set_handler_locked(d, handle_level_irq);
    } else {
        irq_set_handler_locked(d, handle_edge_irq);
    }
    0
}

unsafe fn vf610_gpio_irq_mask(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d) as *mut gpio_chip;
    let port = gpiochip_get_data(gc) as *mut vf610_gpio_port;
    let gpio_num = irqd_to_hwirq(d);
    let pcr_base = (*port).base.add(PORT_PCR(gpio_num as usize));
    vf610_gpio_writel(0, pcr_base);
    gpiochip_disable_irq(gc, gpio_num);
}

unsafe fn vf610_gpio_irq_unmask(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d) as *mut gpio_chip;
    let port = gpiochip_get_data(gc) as *mut vf610_gpio_port;
    let gpio_num = irqd_to_hwirq(d);
    let pcr_base = (*port).base.add(PORT_PCR(gpio_num as usize));
    gpiochip_enable_irq(gc, gpio_num);
    vf610_gpio_writel(((*port).irqc[gpio_num] as u32) << PORT_PCR_IRQC_OFFSET, pcr_base);
}

unsafe fn vf610_gpio_irq_set_wake(d: *mut irq_data, enable: u32) -> i32 {
    let port = gpiochip_get_data(irq_data_get_irq_chip_data(d)) as *mut vf610_gpio_port;
    if enable != 0 { enable_irq_wake((*port).irq); } else { disable_irq_wake((*port).irq); }
    0
}

static vf610_irqchip: irq_chip = irq_chip {
    name: "gpio-vf610",
    irq_ack: Some(vf610_gpio_irq_ack),
    irq_mask: Some(vf610_gpio_irq_mask),
    irq_unmask: Some(vf610_gpio_irq_unmask),
    irq_set_type: Some(vf610_gpio_irq_set_type),
    irq_set_wake: Some(vf610_gpio_irq_set_wake),
    flags: IRQCHIP_IMMUTABLE | IRQCHIP_MASK_ON_SUSPEND | IRQCHIP_ENABLE_WAKEUP_ON_SUSPEND,
    ..IRQ_CHIP_IRQ_RESOURCE_HELPERS
};

unsafe fn vf610_gpio_disable_clk(data: *mut core::ffi::c_void) {
    clk_disable_unprepare(data as *mut clk);
}

unsafe fn vf610_gpio_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let port = devm_kzalloc(dev, core::mem::size_of::<vf610_gpio_port>(), GFP_KERNEL) as *mut vf610_gpio_port;
    if port.is_null() { return -ENOMEM; }
    (*port).sdata = device_get_match_data(dev) as *const fsl_gpio_soc_data;
    let mut dual_base = (*(*port).sdata).have_dual_base;

    /*
     * Handle legacy compatible combinations which used two reg values
     * for the i.MX8ULP and i.MX93.
     */
    if device_is_compatible(dev, "fsl,imx7ulp-gpio") &&
       (device_is_compatible(dev, "fsl,imx93-gpio") || device_is_compatible(dev, "fsl,imx8ulp-gpio")) {
        dual_base = true;
    }
    if dual_base {
        (*port).base = devm_platform_ioremap_resource(pdev, 0);
        if IS_ERR((*port).base) { return PTR_ERR((*port).base); }
        (*port).gpio_base = devm_platform_ioremap_resource(pdev, 1);
        if IS_ERR((*port).gpio_base) { return PTR_ERR((*port).gpio_base); }
    } else {
        (*port).base = devm_platform_ioremap_resource(pdev, 0);
        if IS_ERR((*port).base) { return PTR_ERR((*port).base); }
        (*port).gpio_base = (*port).base.add(IMX8ULP_GPIO_BASE_OFF);
        (*port).base = (*port).base.add(IMX8ULP_BASE_OFF);
    }

    (*port).irq = platform_get_irq(pdev, 0);
    if (*port).irq < 0 { return (*port).irq; }
    (*port).clk_port = devm_clk_get(dev, "port");
    let mut ret = PTR_ERR_OR_ZERO((*port).clk_port);
    if ret == 0 {
        ret = clk_prepare_enable((*port).clk_port); if ret != 0 { return ret; }
        ret = devm_add_action_or_reset(dev, Some(vf610_gpio_disable_clk), (*port).clk_port as *mut _); if ret != 0 { return ret; }
    } else if ret == -EPROBE_DEFER { return ret; }
    (*port).clk_gpio = devm_clk_get(dev, "gpio");
    ret = PTR_ERR_OR_ZERO((*port).clk_gpio);
    if ret == 0 {
        ret = clk_prepare_enable((*port).clk_gpio); if ret != 0 { return ret; }
        ret = devm_add_action_or_reset(dev, Some(vf610_gpio_disable_clk), (*port).clk_gpio as *mut _); if ret != 0 { return ret; }
    } else if ret == -EPROBE_DEFER { return ret; }

    let gc = &mut (*port).chip.gc;
    let mut flags = GPIO_GENERIC_PINCTRL_BACKEND;
    if (*(*port).sdata).have_paddr { flags |= GPIO_GENERIC_READ_OUTPUT_REG_SET; }
    let config = gpio_generic_chip_config {
        dev,
        sz: 4,
        dat: (*port).gpio_base.add(GPIO_PDIR),
        set: (*port).gpio_base.add(GPIO_PDOR),
        dirout: if (*(*port).sdata).have_paddr { (*port).gpio_base.add(GPIO_PDDR) } else { core::ptr::null_mut() },
        flags,
    };
    ret = gpio_generic_chip_init(&mut (*port).chip, &config);
    if ret != 0 { return dev_err_probe(dev, ret, "unable to init generic GPIO\n"); }
    (*gc).label = dev_name(dev); (*gc).base = -1;
    for i in 0..(*gc).ngpio { vf610_gpio_writel(0, (*port).base.add(PORT_PCR(i as usize)); }
    vf610_gpio_writel(!0, (*port).base.add(PORT_ISFR));
    let girq = &mut (*gc).irq;
    gpio_irq_chip_set_chip(girq, &vf610_irqchip);
    girq.parent_handler = Some(vf610_gpio_irq_handler); girq.num_parents = 1;
    girq.parents = devm_kcalloc(&mut (*pdev).dev, 1, core::mem::size_of::<i32>(), GFP_KERNEL) as *mut i32;
    if girq.parents.is_null() { return -ENOMEM; }
    *girq.parents = (*port).irq; girq.default_type = IRQ_TYPE_NONE; girq.handler = Some(handle_edge_irq);
    devm_gpiochip_add_data(dev, gc, port as *mut core::ffi::c_void)
}

static mut vf610_gpio_driver: platform_driver = platform_driver {
    driver: device_driver { name: "gpio-vf610", of_match_table: vf610_gpio_dt_ids.as_ptr(), ..device_driver::default() },
    probe: Some(vf610_gpio_probe),
    ..platform_driver::default()
};

module_platform_driver!(vf610_gpio_driver);
module_description!("VF610 GPIO driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
