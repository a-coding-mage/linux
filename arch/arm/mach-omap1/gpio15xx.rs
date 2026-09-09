// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP15xx specific gpio init
 *
 * Copyright (C) 2010 Texas Instruments Incorporated - https://www.ti.com/
 *
 * Author:
 *	Charulatha V <charu@ti.com>
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/platform_data/gpio-omap.h, linux/soc/ti/omap1-soc.h, asm/irq.h,
// and "irqs.h".

const OMAP1_MPUIO_VBASE: usize = OMAP1_MPUIO_BASE;
const OMAP1510_GPIO_BASE: usize = 0xFFFCE000;

/* gpio1 */
static mut omap15xx_mpu_gpio_resources: [resource; 2] = [
    resource {
        start: OMAP1_MPUIO_VBASE,
        end: OMAP1_MPUIO_VBASE + SZ_2K - 1,
        flags: IORESOURCE_MEM,
    },
    resource {
        start: INT_MPUIO,
        end: 0,
        flags: IORESOURCE_IRQ,
    },
];

static mut omap15xx_mpuio_regs: omap_gpio_reg_offs = omap_gpio_reg_offs {
    revision: USHRT_MAX,
    direction: OMAP_MPUIO_IO_CNTL,
    datain: OMAP_MPUIO_INPUT_LATCH,
    dataout: OMAP_MPUIO_OUTPUT,
    irqstatus: OMAP_MPUIO_GPIO_INT,
    irqenable: OMAP_MPUIO_GPIO_MASKIT,
    irqenable_inv: true,
    irqctrl: OMAP_MPUIO_GPIO_INT_EDGE,
    ..omap_gpio_reg_offs::default()
};

static mut omap15xx_mpu_gpio_config: omap_gpio_platform_data = omap_gpio_platform_data {
    is_mpuio: true,
    bank_width: 16,
    bank_stride: 1,
    regs: unsafe { &mut omap15xx_mpuio_regs },
    ..omap_gpio_platform_data::default()
};

static mut omap15xx_mpu_gpio: platform_device = platform_device {
    name: "omap_gpio",
    id: 0,
    dev: device {
        platform_data: unsafe { &mut omap15xx_mpu_gpio_config },
        ..device::default()
    },
    num_resources: omap15xx_mpu_gpio_resources.len(),
    resource: unsafe { &mut omap15xx_mpu_gpio_resources[0] },
    ..platform_device::default()
};

/* gpio2 */
static mut omap15xx_gpio_resources: [resource; 2] = [
    resource {
        start: OMAP1510_GPIO_BASE,
        end: OMAP1510_GPIO_BASE + SZ_2K - 1,
        flags: IORESOURCE_MEM,
    },
    resource {
        start: INT_GPIO_BANK1,
        end: 0,
        flags: IORESOURCE_IRQ,
    },
];

static mut omap15xx_gpio_regs: omap_gpio_reg_offs = omap_gpio_reg_offs {
    revision: USHRT_MAX,
    direction: OMAP1510_GPIO_DIR_CONTROL,
    datain: OMAP1510_GPIO_DATA_INPUT,
    dataout: OMAP1510_GPIO_DATA_OUTPUT,
    irqstatus: OMAP1510_GPIO_INT_STATUS,
    irqenable: OMAP1510_GPIO_INT_MASK,
    irqenable_inv: true,
    irqctrl: OMAP1510_GPIO_INT_CONTROL,
    pinctrl: OMAP1510_GPIO_PIN_CONTROL,
    ..omap_gpio_reg_offs::default()
};

static mut omap15xx_gpio_config: omap_gpio_platform_data = omap_gpio_platform_data {
    bank_width: 16,
    regs: unsafe { &mut omap15xx_gpio_regs },
    ..omap_gpio_platform_data::default()
};

static mut omap15xx_gpio: platform_device = platform_device {
    name: "omap_gpio",
    id: 1,
    dev: device {
        platform_data: unsafe { &mut omap15xx_gpio_config },
        ..device::default()
    },
    num_resources: omap15xx_gpio_resources.len(),
    resource: unsafe { &mut omap15xx_gpio_resources[0] },
    ..platform_device::default()
};

/*
 * omap15xx_gpio_init needs to be done before
 * machine_init functions access gpio APIs.
 * Hence omap15xx_gpio_init is a postcore_initcall.
 */
unsafe fn omap15xx_gpio_init() -> i32 {
    if !cpu_is_omap15xx() {
        return -EINVAL;
    }

    platform_device_register(&mut omap15xx_mpu_gpio);
    platform_device_register(&mut omap15xx_gpio);

    0
}

postcore_initcall!(omap15xx_gpio_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
