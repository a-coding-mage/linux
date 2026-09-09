// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mach-pxa/gumstix.c
 *
 *  Support for the Gumstix motherboards.
 *
 *  Original Author: Craig Hughes
 *  Created: Feb 14, 2008
 *  Copyright: Craig Hughes
 *
 *  Implemented based on lubbock.c by Nicolas Pitre and code from Craig
 *  Hughes
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

static mut flash_resource: resource = resource {
    start: 0x00000000,
    end: SZ_64M - 1,
    flags: IORESOURCE_MEM,
};

static mut gumstix_partitions: [mtd_partition; 2] = [
    mtd_partition {
        name: c"Bootloader".as_ptr(),
        size: 0x00040000,
        offset: 0,
        mask_flags: MTD_WRITEABLE, // force read-only
    },
    mtd_partition {
        name: c"rootfs".as_ptr(),
        size: MTDPART_SIZ_FULL,
        offset: MTDPART_OFS_APPEND,
        ..unsafe { core::mem::zeroed() }
    },
];

static mut gumstix_flash_data: flash_platform_data = flash_platform_data {
    map_name: c"cfi_probe".as_ptr(),
    parts: unsafe { gumstix_partitions.as_mut_ptr() },
    nr_parts: ARRAY_SIZE(gumstix_partitions),
    width: 2,
    ..unsafe { core::mem::zeroed() }
};

static mut gumstix_flash_device: platform_device = platform_device {
    name: c"pxa2xx-flash".as_ptr(),
    id: 0,
    dev: device {
        platform_data: unsafe { &mut gumstix_flash_data as *mut _ as *mut core::ffi::c_void },
        ..unsafe { core::mem::zeroed() }
    },
    resource: unsafe { &mut flash_resource },
    num_resources: 1,
    ..unsafe { core::mem::zeroed() }
};

static mut devices: [*mut platform_device; 1] = [unsafe { &mut gumstix_flash_device }];

#[cfg(CONFIG_MMC_PXA)]
static mut gumstix_mci_platform_data: pxamci_platform_data = pxamci_platform_data {
    ocr_mask: MMC_VDD_32_33 | MMC_VDD_33_34,
    ..unsafe { core::mem::zeroed() }
};

#[cfg(CONFIG_MMC_PXA)]
unsafe fn gumstix_mmc_init() {
    pxa_set_mci_info(&mut gumstix_mci_platform_data, core::ptr::null_mut());
}

#[cfg(not(CONFIG_MMC_PXA))]
unsafe fn gumstix_mmc_init() {
    pr_debug!("Gumstix mmc disabled\n");
}

// Equivalent to IS_ENABLED(CONFIG_USB_PXA25X); retained as a build condition.
#[cfg(CONFIG_USB_PXA25X)]
static gumstix_vbus_props: [property_entry; 3] = [
    PROPERTY_ENTRY_GPIO!("vbus-gpios", &pxa2xx_gpiochip_node, GPIO_GUMSTIX_USB_GPIOn, GPIO_ACTIVE_HIGH),
    PROPERTY_ENTRY_GPIO!("pullup-gpios", &pxa2xx_gpiochip_node, GPIO_GUMSTIX_USB_GPIOx, GPIO_ACTIVE_HIGH),
    property_entry { ..unsafe { core::mem::zeroed() } },
];

#[cfg(CONFIG_USB_PXA25X)]
static gumstix_gpio_vbus_info: platform_device_info = platform_device_info {
    name: c"gpio-vbus".as_ptr(),
    id: PLATFORM_DEVID_NONE,
    properties: gumstix_vbus_props.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};

#[cfg(CONFIG_USB_PXA25X)]
unsafe fn gumstix_udc_init() {
    platform_device_register_full(&gumstix_gpio_vbus_info);
}

#[cfg(not(CONFIG_USB_PXA25X))]
unsafe fn gumstix_udc_init() {
    pr_debug!("Gumstix udc is disabled\n");
}

#[cfg(CONFIG_BT)]
unsafe fn gumstix_setup_bt_clock() {
    let mut timeout: i32 = 500;

    if readl(OSCC) & OSCC_OOK == 0 {
        pr_warn!("32kHz clock was not on. Bootloader may need to be updated\n");
    } else {
        return;
    }

    writel(readl(OSCC) | OSCC_OON, OSCC);
    loop {
        if readl(OSCC) & OSCC_OOK != 0 {
            break;
        }
        udelay(1);
        timeout -= 1;
        if timeout == 0 {
            break;
        }
    }
    if timeout == 0 {
        pr_err!("Failed to start 32kHz clock\n");
    }
}

#[cfg(CONFIG_BT)]
unsafe fn gumstix_bluetooth_init() {
    gumstix_setup_bt_clock();

    let err = gpio_request(GPIO_GUMSTIX_BTRESET, c"BTRST".as_ptr());
    if err != 0 {
        pr_err!("gumstix: failed request gpio for bluetooth reset\n");
        return;
    }

    let err = gpio_direction_output(GPIO_GUMSTIX_BTRESET, 1);
    if err != 0 {
        pr_err!("gumstix: can't reset bluetooth\n");
        return;
    }
    gpio_set_value(GPIO_GUMSTIX_BTRESET, 0);
    udelay(100);
    gpio_set_value(GPIO_GUMSTIX_BTRESET, 1);
}

#[cfg(not(CONFIG_BT))]
unsafe fn gumstix_bluetooth_init() {
    pr_debug!("Gumstix Bluetooth is disabled\n");
}

static mut gumstix_pin_config: [c_ulong; 8] = [
    GPIO12_32KHz,
    // BTUART
    GPIO42_HWUART_RXD,
    GPIO43_HWUART_TXD,
    GPIO44_HWUART_CTS,
    GPIO45_HWUART_RTS,
    // MMC
    GPIO6_MMC_CLK,
    GPIO53_MMC_CLK,
    GPIO8_MMC_CS0,
];

#[no_mangle]
pub unsafe extern "C" fn am200_init() -> i32 { 0 }

#[no_mangle]
pub unsafe extern "C" fn am300_init() -> i32 { 0 }

unsafe fn carrier_board_init() {
    /* put carrier/expansion board init here if they cannot be detected programatically */
    am200_init();
    am300_init();
}

unsafe fn gumstix_init() {
    pxa2xx_mfp_config(gumstix_pin_config.as_mut_ptr(), gumstix_pin_config.len());

    pxa_set_ffuart_info(core::ptr::null_mut());
    pxa_set_btuart_info(core::ptr::null_mut());
    pxa_set_stuart_info(core::ptr::null_mut());
    pxa_set_hwuart_info(core::ptr::null_mut());

    gumstix_bluetooth_init();
    gumstix_udc_init();
    gumstix_mmc_init();
    let _ = platform_add_devices(devices.as_mut_ptr(), devices.len());
    carrier_board_init();
}

// MACHINE_START(GUMSTIX, "Gumstix")
//     .atag_offset = 0x100, /* match u-boot bi_boot_params */
//     .map_io = pxa25x_map_io,
//     .nr_irqs = PXA_NR_IRQS,
//     .init_irq = pxa25x_init_irq,
//     .init_time = pxa_timer_init,
//     .init_machine = gumstix_init,
//     .restart = pxa_restart,
// MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
