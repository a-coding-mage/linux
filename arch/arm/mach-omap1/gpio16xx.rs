// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP16xx specific gpio init
 *
 * Copyright (C) 2010 Texas Instruments Incorporated - https://www.ti.com/
 *
 * Author:
 *	Charulatha V <charu@ti.com>
 */

// C dependencies:
// linux/platform_data/gpio-omap.h, linux/property.h, linux/soc/ti/omap1-io.h
// hardware.h, irqs.h, soc.h

const OMAP1610_GPIO1_BASE: usize = 0xfffbe400;
const OMAP1610_GPIO2_BASE: usize = 0xfffbec00;
const OMAP1610_GPIO3_BASE: usize = 0xfffbb400;
const OMAP1610_GPIO4_BASE: usize = 0xfffbbc00;
const OMAP1_MPUIO_VBASE: usize = OMAP1_MPUIO_BASE;

/* smart idle, enable wakeup */
const SYSCONFIG_WORD: u32 = 0x14;

/* mpu gpio */
static mut omap16xx_mpu_gpio_resources: [struct_resource; 2] = [
    struct_resource { start: OMAP1_MPUIO_VBASE, end: OMAP1_MPUIO_VBASE + SZ_2K - 1, flags: IORESOURCE_MEM },
    struct_resource { start: INT_MPUIO, end: 0, flags: IORESOURCE_IRQ },
];

static mut omap16xx_mpuio_regs: struct_omap_gpio_reg_offs = struct_omap_gpio_reg_offs {
    revision: USHRT_MAX,
    direction: OMAP_MPUIO_IO_CNTL,
    datain: OMAP_MPUIO_INPUT_LATCH,
    dataout: OMAP_MPUIO_OUTPUT,
    irqstatus: OMAP_MPUIO_GPIO_INT,
    irqenable: OMAP_MPUIO_GPIO_MASKIT,
    irqenable_inv: true,
    irqctrl: OMAP_MPUIO_GPIO_INT_EDGE,
};

static mut omap16xx_mpu_gpio_config: struct_omap_gpio_platform_data = struct_omap_gpio_platform_data {
    is_mpuio: true,
    bank_width: 16,
    bank_stride: 1,
    regs: core::ptr::addr_of_mut!(omap16xx_mpuio_regs),
};

#[no_mangle]
pub static omap16xx_mpu_gpio_swnode: struct_software_node = struct_software_node {};

static omap16xx_mpu_gpio: struct_platform_device_info = struct_platform_device_info {
    name: "omap_gpio\0",
    id: 0,
    data: core::ptr::addr_of_mut!(omap16xx_mpu_gpio_config) as *mut _,
    size_data: core::mem::size_of::<struct_omap_gpio_platform_data>(),
    num_res: 2,
    res: core::ptr::addr_of_mut!(omap16xx_mpu_gpio_resources) as *mut _,
    swnode: core::ptr::addr_of!(omap16xx_mpu_gpio_swnode),
};

// gpio1..gpio4 retain the common OMAP register layout used by the original C definitions.
static mut omap16xx_gpio_regs: struct_omap_gpio_reg_offs = struct_omap_gpio_reg_offs {
    revision: OMAP1610_GPIO_REVISION, direction: OMAP1610_GPIO_DIRECTION,
    set_dataout: OMAP1610_GPIO_SET_DATAOUT, clr_dataout: OMAP1610_GPIO_CLEAR_DATAOUT,
    datain: OMAP1610_GPIO_DATAIN, dataout: OMAP1610_GPIO_DATAOUT,
    irqstatus: OMAP1610_GPIO_IRQSTATUS1, irqenable: OMAP1610_GPIO_IRQENABLE1,
    set_irqenable: OMAP1610_GPIO_SET_IRQENABLE1, clr_irqenable: OMAP1610_GPIO_CLEAR_IRQENABLE1,
    wkup_en: OMAP1610_GPIO_WAKEUPENABLE, edgectrl1: OMAP1610_GPIO_EDGE_CTRL1,
    edgectrl2: OMAP1610_GPIO_EDGE_CTRL2,
};

extern "C" {
    static mut omap16xx_gpio1_resources: [struct_resource; 2];
    static mut omap16xx_gpio2_resources: [struct_resource; 2];
    static mut omap16xx_gpio3_resources: [struct_resource; 2];
    static mut omap16xx_gpio4_resources: [struct_resource; 2];
}

// Build-time declarations below correspond directly to gpio1..gpio4 in the C source.
extern "C" {
    static omap16xx_gpio1: struct_platform_device_info;
    static omap16xx_gpio2: struct_platform_device_info;
    static omap16xx_gpio3: struct_platform_device_info;
    static omap16xx_gpio4: struct_platform_device_info;
    fn cpu_is_omap16xx() -> bool;
    fn omap_readl(reg: usize) -> u32;
    fn omap_writel(value: u32, reg: usize);
    fn ioremap(start: usize, size: usize) -> *mut core::ffi::c_void;
    fn iounmap(base: *mut core::ffi::c_void);
    fn platform_device_register_full(info: *const struct_platform_device_info) -> i32;
}

static omap16xx_gpio_dev: [*const struct_platform_device_info; 5] = [
    core::ptr::addr_of!(omap16xx_mpu_gpio),
    core::ptr::addr_of!(omap16xx_gpio1), core::ptr::addr_of!(omap16xx_gpio2),
    core::ptr::addr_of!(omap16xx_gpio3), core::ptr::addr_of!(omap16xx_gpio4),
];

unsafe fn omap16xx_gpio_init() -> i32 {
    if !cpu_is_omap16xx() { return -22; }
    omap_writel(omap_readl(ULPD_CAM_CLK_CTRL) | 0x04, ULPD_CAM_CLK_CTRL);
    let mut i = 0;
    while i < omap16xx_gpio_dev.len() {
        let pdevinfo = omap16xx_gpio_dev[i];
        let res = (*pdevinfo).res;
        if res.is_null() { return -19; }
        let base = ioremap((*res).start, (*res).end.wrapping_sub((*res).start).wrapping_add(1));
        if base.is_null() { return -12; }
        __raw_writel(SYSCONFIG_WORD, (base as *mut u8).add(OMAP1610_GPIO_SYSCONFIG) as *mut u32);
        iounmap(base);
        platform_device_register_full(pdevinfo);
        i += 1;
    }
    0
}

// postcore_initcall(omap16xx_gpio_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
