// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * File: arch/arm/plat-omap/fb.c
 *
 * Framebuffer device registration for TI OMAP platforms
 *
 * Copyright (C) 2006 Nokia Corporation
 * Author: Imre Deak <imre.deak@nokia.com>
 */

// External kernel dependencies supplied by the surrounding repository.

#[cfg(feature = "CONFIG_FB_OMAP")]
static mut OMAPFB_LCD_CONFIGURED: bool = false;

#[cfg(feature = "CONFIG_FB_OMAP")]
static mut OMAPFB_CONFIG: omapfb_platform_data = unsafe { core::mem::zeroed() };

#[cfg(feature = "CONFIG_FB_OMAP")]
static mut OMAP_FB_DMA_MASK: u64 = !(0_u32 as u64);

#[cfg(feature = "CONFIG_FB_OMAP")]
static mut OMAP_FB_RESOURCES: [resource; 2] = [
    resource {
        name: b"irq\0".as_ptr() as *const i8,
        start: INT_LCD_CTRL,
        flags: IORESOURCE_IRQ,
    },
    resource {
        name: b"irq\0".as_ptr() as *const i8,
        start: INT_SOSSI_MATCH,
        flags: IORESOURCE_IRQ,
    },
];

#[cfg(feature = "CONFIG_FB_OMAP")]
static mut OMAP_FB_DEVICE: platform_device = platform_device {
    name: b"omapfb\0".as_ptr() as *const i8,
    id: -1,
    dev: device {
        dma_mask: unsafe { &raw mut OMAP_FB_DMA_MASK },
        coherent_dma_mask: DMA_BIT_MASK(32),
        platform_data: unsafe { &raw mut OMAPFB_CONFIG as *mut _ },
    },
    num_resources: 2,
    resource: unsafe { &raw mut OMAP_FB_RESOURCES as *mut resource },
};

#[cfg(feature = "CONFIG_FB_OMAP")]
pub unsafe fn omapfb_set_lcd_config(config: *const omap_lcd_config) {
    OMAPFB_CONFIG.lcd = *config;
    OMAPFB_LCD_CONFIGURED = true;
}

#[cfg(feature = "CONFIG_FB_OMAP")]
unsafe fn omap_init_fb() -> i32 {
    /*
     * If the board file has not set the lcd config with
     * omapfb_set_lcd_config(), don't bother registering the omapfb device
     */
    if !OMAPFB_LCD_CONFIGURED {
        return 0;
    }

    platform_device_register(&raw mut OMAP_FB_DEVICE)
}

// arch_initcall(omap_init_fb);

#[cfg(not(feature = "CONFIG_FB_OMAP"))]
pub unsafe fn omapfb_set_lcd_config(_config: *const omap_lcd_config) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
