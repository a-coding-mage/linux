// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2011 Samsung Electronics Co., Ltd.
//		http://www.samsung.com
//
// Base Samsung platform device definitions

// C includes are supplied by the surrounding kernel translation unit.

const SAMSUNG_DEVICE_DMA_MASK: u64 = DMA_BIT_MASK(32);

/* FB */
#[cfg(CONFIG_S3C_DEV_FB)]
static mut S3C_FB_RESOURCE: [struct_resource; 4] = [
    DEFINE_RES_MEM(S3C_PA_FB, SZ_16K),
    DEFINE_RES_IRQ(IRQ_LCD_VSYNC),
    DEFINE_RES_IRQ(IRQ_LCD_FIFO),
    DEFINE_RES_IRQ(IRQ_LCD_SYSTEM),
];

#[cfg(CONFIG_S3C_DEV_FB)]
pub static mut s3c_device_fb: platform_device = platform_device {
    name: "s3c-fb", id: -1, num_resources: ARRAY_SIZE(S3C_FB_RESOURCE),
    resource: S3C_FB_RESOURCE.as_mut_ptr(),
    dev: device { dma_mask: &SAMSUNG_DEVICE_DMA_MASK, coherent_dma_mask: DMA_BIT_MASK(32), ..device::default() },
};

#[cfg(CONFIG_S3C_DEV_FB)]
pub unsafe fn s3c_fb_set_platdata(pd: *mut s3c_fb_platdata) {
    s3c_set_platdata(pd, core::mem::size_of::<s3c_fb_platdata>(), &mut s3c_device_fb);
}

/* HSMMC */
macro_rules! hsmmc_device {
    ($n:literal, $pa:ident, $irq:ident, $res:ident, $pd:ident, $dev:ident, $set:ident) => {
        #[cfg(concat_id(CONFIG_S3C_DEV_HSMMC, $n))]
        static mut $res: [struct_resource; 2] = [DEFINE_RES_MEM($pa, SZ_4K), DEFINE_RES_IRQ($irq)];
        #[cfg(concat_id(CONFIG_S3C_DEV_HSMMC, $n))]
        pub static mut $pd: s3c_sdhci_platdata = s3c_sdhci_platdata {
            max_width: 4, host_caps: MMC_CAP_4_BIT_DATA | MMC_CAP_MMC_HIGHSPEED | MMC_CAP_SD_HIGHSPEED,
            ..s3c_sdhci_platdata::default()
        };
        #[cfg(concat_id(CONFIG_S3C_DEV_HSMMC, $n))]
        pub static mut $dev: platform_device = platform_device {
            name: "s3c-sdhci", id: $n, num_resources: ARRAY_SIZE($res), resource: $res.as_mut_ptr(),
            dev: device { dma_mask: &SAMSUNG_DEVICE_DMA_MASK, coherent_dma_mask: DMA_BIT_MASK(32), platform_data: &$pd, ..device::default() },
        };
        #[cfg(concat_id(CONFIG_S3C_DEV_HSMMC, $n))]
        pub unsafe fn $set(pd: *mut s3c_sdhci_platdata) { s3c_sdhci_set_platdata(pd, &mut $pd); }
    };
}

// The C source provides four independently conditional HSMMC instances.
hsmmc_device!(0, S3C_PA_HSMMC0, IRQ_HSMMC0, S3C_HSMMC0_RESOURCE, s3c_hsmmc0_def_platdata, s3c_device_hsmmc0, s3c_sdhci0_set_platdata);
hsmmc_device!(1, S3C_PA_HSMMC1, IRQ_HSMMC1, S3C_HSMMC1_RESOURCE, s3c_hsmmc1_def_platdata, s3c_device_hsmmc1, s3c_sdhci1_set_platdata);
hsmmc_device!(2, S3C_PA_HSMMC2, IRQ_HSMMC2, S3C_HSMMC2_RESOURCE, s3c_hsmmc2_def_platdata, s3c_device_hsmmc2, s3c_sdhci2_set_platdata);
hsmmc_device!(3, S3C_PA_HSMMC3, IRQ_HSMMC3, S3C_HSMMC3_RESOURCE, s3c_hsmmc3_def_platdata, s3c_device_hsmmc3, s3c_sdhci3_set_platdata);

/* I2C */
static mut S3C_I2C0_RESOURCE: [struct_resource; 2] = [DEFINE_RES_MEM(S3C_PA_IIC, SZ_4K), DEFINE_RES_IRQ(IRQ_IIC)];
pub static mut s3c_device_i2c0: platform_device = platform_device { name: "s3c2410-i2c", id: 0, num_resources: 2, resource: S3C_I2C0_RESOURCE.as_mut_ptr(), ..platform_device::default() };
pub static mut default_i2c_data: s3c2410_platform_i2c = s3c2410_platform_i2c { flags: 0, slave_addr: 0x10, frequency: 100 * 1000, sda_delay: 100, ..s3c2410_platform_i2c::default() };

pub unsafe fn s3c_i2c0_set_platdata(mut pd: *mut s3c2410_platform_i2c) {
    if pd.is_null() { pd = &mut default_i2c_data; (*pd).bus_num = 0; }
    let npd = s3c_set_platdata(pd, core::mem::size_of::<s3c2410_platform_i2c>(), &mut s3c_device_i2c0);
    if (*npd).cfg_gpio.is_none() { (*npd).cfg_gpio = Some(s3c_i2c0_cfg_gpio); }
}

#[cfg(CONFIG_S3C_DEV_I2C1)]
static mut S3C_I2C1_RESOURCE: [struct_resource; 2] = [DEFINE_RES_MEM(S3C_PA_IIC1, SZ_4K), DEFINE_RES_IRQ(IRQ_IIC1)];
#[cfg(CONFIG_S3C_DEV_I2C1)]
pub static mut s3c_device_i2c1: platform_device = platform_device { name: "s3c2410-i2c", id: 1, num_resources: 2, resource: S3C_I2C1_RESOURCE.as_mut_ptr(), ..platform_device::default() };
#[cfg(CONFIG_S3C_DEV_I2C1)]
pub unsafe fn s3c_i2c1_set_platdata(mut pd: *mut s3c2410_platform_i2c) { if pd.is_null() { pd = &mut default_i2c_data; (*pd).bus_num = 1; } let npd = s3c_set_platdata(pd, core::mem::size_of::<s3c2410_platform_i2c>(), &mut s3c_device_i2c1); if (*npd).cfg_gpio.is_none() { (*npd).cfg_gpio = Some(s3c_i2c1_cfg_gpio); } }

/* KEYPAD */
#[cfg(CONFIG_SAMSUNG_DEV_KEYPAD)]
static mut SAMSUNG_KEYPAD_RESOURCES: [struct_resource; 2] = [DEFINE_RES_MEM(SAMSUNG_PA_KEYPAD, SZ_32), DEFINE_RES_IRQ(IRQ_KEYPAD)];
#[cfg(CONFIG_SAMSUNG_DEV_KEYPAD)]
pub static mut samsung_device_keypad: platform_device = platform_device { name: "samsung-keypad", id: -1, num_resources: 2, resource: SAMSUNG_KEYPAD_RESOURCES.as_mut_ptr(), ..platform_device::default() };
#[cfg(CONFIG_SAMSUNG_DEV_KEYPAD)]
pub unsafe fn samsung_keypad_set_platdata(pd: *mut samsung_keypad_platdata) { let npd = s3c_set_platdata(pd, core::mem::size_of::<samsung_keypad_platdata>(), &mut samsung_device_keypad); if (*npd).cfg_gpio.is_none() { (*npd).cfg_gpio = Some(samsung_keypad_cfg_gpio); } }

/* PWM Timer */
#[cfg(CONFIG_SAMSUNG_DEV_PWM)]
static mut SAMSUNG_PWM_RESOURCE: [struct_resource; 1] = [DEFINE_RES_MEM(SAMSUNG_PA_TIMER, SZ_4K)];
#[cfg(CONFIG_SAMSUNG_DEV_PWM)]
pub static mut samsung_device_pwm: platform_device = platform_device { name: "samsung-pwm", id: -1, num_resources: 1, resource: SAMSUNG_PWM_RESOURCE.as_mut_ptr(), ..platform_device::default() };
#[cfg(CONFIG_SAMSUNG_DEV_PWM)]
pub unsafe fn samsung_pwm_set_platdata(pd: *mut samsung_pwm_variant) { samsung_device_pwm.dev.platform_data = pd as *mut _; }

/* USB */
#[cfg(CONFIG_S3C_DEV_USB_HOST)]
static mut S3C_USB_RESOURCE: [struct_resource; 2] = [DEFINE_RES_MEM(S3C_PA_USBHOST, SZ_256), DEFINE_RES_IRQ(IRQ_USBH)];
#[cfg(CONFIG_S3C_DEV_USB_HOST)]
pub static mut s3c_device_ohci: platform_device = platform_device { name: "s3c2410-ohci", id: -1, num_resources: 2, resource: S3C_USB_RESOURCE.as_mut_ptr(), dev: device { dma_mask: &SAMSUNG_DEVICE_DMA_MASK, coherent_dma_mask: DMA_BIT_MASK(32), ..device::default() }, };

/* USB HSOTG */
#[cfg(CONFIG_S3C_DEV_USB_HSOTG)]
static mut S3C_USB_HSOTG_RESOURCES: [struct_resource; 2] = [DEFINE_RES_MEM(S3C_PA_USB_HSOTG, SZ_128K), DEFINE_RES_IRQ(IRQ_OTG)];
#[cfg(CONFIG_S3C_DEV_USB_HSOTG)]
pub static mut s3c_device_usb_hsotg: platform_device = platform_device { name: "s3c-hsotg", id: -1, num_resources: 2, resource: S3C_USB_HSOTG_RESOURCES.as_mut_ptr(), dev: device { dma_mask: &SAMSUNG_DEVICE_DMA_MASK, coherent_dma_mask: DMA_BIT_MASK(32), ..device::default() }, };
#[cfg(CONFIG_S3C_DEV_USB_HSOTG)]
pub unsafe fn dwc2_hsotg_set_platdata(pd: *mut dwc2_hsotg_plat) { let npd = s3c_set_platdata(pd, core::mem::size_of::<dwc2_hsotg_plat>(), &mut s3c_device_usb_hsotg); if (*npd).phy_init.is_none() { (*npd).phy_init = Some(s3c_usb_phy_init); } if (*npd).phy_exit.is_none() { (*npd).phy_exit = Some(s3c_usb_phy_exit); } }

#[cfg(CONFIG_S3C64XX_DEV_SPI0)]
static mut S3C64XX_SPI0_RESOURCE: [struct_resource; 2] = [DEFINE_RES_MEM(S3C_PA_SPI0, SZ_256), DEFINE_RES_IRQ(IRQ_SPI0)];
#[cfg(CONFIG_S3C64XX_DEV_SPI0)]
pub static mut s3c64xx_device_spi0: platform_device = platform_device { name: "s3c6410-spi", id: 0, num_resources: 2, resource: S3C64XX_SPI0_RESOURCE.as_mut_ptr(), dev: device { dma_mask: &SAMSUNG_DEVICE_DMA_MASK, coherent_dma_mask: DMA_BIT_MASK(32), ..device::default() }, };
#[cfg(CONFIG_S3C64XX_DEV_SPI0)]
pub unsafe fn s3c64xx_spi0_set_platdata(src_clk_nr: i32, num_cs: i32) { if num_cs == 0 || src_clk_nr < 0 { pr_err!("{}: Invalid SPI configuration\n", "s3c64xx_spi0_set_platdata"); return; } let pd = s3c64xx_spi_info { num_cs, src_clk_nr, cfg_gpio: Some(s3c64xx_spi0_cfg_gpio), ..s3c64xx_spi_info::default() }; s3c_set_platdata(&pd, core::mem::size_of::<s3c64xx_spi_info>(), &mut s3c64xx_device_spi0); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
