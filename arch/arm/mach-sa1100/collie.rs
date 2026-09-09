/*
 * linux/arch/arm/mach-sa1100/collie.c
 *
 * May be copied or modified under the terms of the GNU General Public
 * License.  See linux/COPYING for more information.
 *
 * This file contains all Collie-specific tweaks.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 *
 * ChangeLog:
 *  2006 Pavel Machek <pavel@ucw.cz>
 *  03-06-2004 John Lenz <lenz@cs.wisc.edu>
 *  06-04-2002 Chris Larson <kergoth@digitalnemesis.net>
 *  04-16-2001 Lineo Japan,Inc. ...
 */

// Kernel headers and symbols referenced below are supplied by external dependencies.

static mut collie_scoop_resources: [resource; 1] = [resource {
    start: 0x40800000,
    end: 0x40800fff,
    flags: IORESOURCE_MEM,
}];

static mut collie_scoop_setup: scoop_config = scoop_config {
    io_dir: COLLIE_SCOOP_IO_DIR,
    io_out: COLLIE_SCOOP_IO_OUT,
    gpio_base: COLLIE_SCOOP_GPIO_BASE,
};

pub static mut colliescoop_device: platform_device = platform_device {
    name: "sharp-scoop",
    id: -1,
    dev: device { platform_data: unsafe { &mut collie_scoop_setup as *mut _ as *mut c_void } },
    num_resources: 1,
    resource: unsafe { collie_scoop_resources.as_mut_ptr() },
};

static mut collie_pcmcia_scoop: [scoop_pcmcia_dev; 1] = [scoop_pcmcia_dev {
    dev: unsafe { &mut colliescoop_device.dev },
    irq: COLLIE_IRQ_GPIO_CF_IRQ,
    cd_irq: COLLIE_IRQ_GPIO_CF_CD,
    cd_irq_str: "PCMCIA0 CD",
}];

static mut collie_pcmcia_config: scoop_pcmcia_config = scoop_pcmcia_config {
    devs: unsafe { &mut collie_pcmcia_scoop[0] },
    num_devs: 1,
};

static mut collie_ucb1x00_data: ucb1x00_plat_data = ucb1x00_plat_data {
    gpio_base: COLLIE_TC35143_GPIO_BASE,
};

static mut collie_mcp_data: mcp_plat_data = mcp_plat_data {
    mccr0: MCCR0_ADM | MCCR0_ExtClk,
    sclk_rate: 9216000,
    codec_pdata: unsafe { &mut collie_ucb1x00_data },
};

/* Battery management GPIOs */
static mut collie_battery_gpiod_table: gpiod_lookup_table = gpiod_lookup_table {
    dev_id: "ucb1x00",
    table: [
        GPIO_LOOKUP("gpio", COLLIE_GPIO_CO, "main battery full", GPIO_ACTIVE_HIGH),
        GPIO_LOOKUP("gpio", COLLIE_GPIO_MAIN_BAT_LOW, "main battery low", GPIO_ACTIVE_HIGH),
        GPIO_LOOKUP("sharp-scoop", 0, "main charge on", GPIO_ACTIVE_HIGH),
        GPIO_LOOKUP_END,
    ],
};

/* Collie AC IN */
static mut collie_power_gpiod_table: gpiod_lookup_table = gpiod_lookup_table {
    dev_id: "gpio-charger",
    table: [
        GPIO_LOOKUP("gpio", COLLIE_GPIO_AC_IN, core::ptr::null(), GPIO_ACTIVE_HIGH),
        GPIO_LOOKUP_END,
    ],
};

static mut collie_ac_supplied_to: [&'static str; 2] = ["main-battery", "backup-battery"];

static mut collie_power_data: gpio_charger_platform_data = gpio_charger_platform_data {
    name: "charger",
    type_: POWER_SUPPLY_TYPE_MAINS,
    supplied_to: unsafe { collie_ac_supplied_to.as_mut_ptr() },
    num_supplicants: 2,
};

static mut collie_power_device: platform_device = platform_device {
    name: "gpio-charger",
    id: -1,
    dev: device { platform_data: unsafe { &mut collie_power_data as *mut _ as *mut c_void } },
    ..platform_device::default()
};

#[cfg(CONFIG_SHARP_LOCOMO)]
pub static mut collie_locomo_device: platform_device = platform_device::default();

#[cfg(CONFIG_SHARP_LOCOMO)]
unsafe fn collie_uart_set_mctrl(port: *mut uart_port, mctrl: u32) {
    if mctrl & TIOCM_RTS != 0 { locomo_gpio_write(&mut collie_locomo_device.dev, LOCOMO_GPIO_RTS, 0); }
    else { locomo_gpio_write(&mut collie_locomo_device.dev, LOCOMO_GPIO_RTS, 1); }
    if mctrl & TIOCM_DTR != 0 { locomo_gpio_write(&mut collie_locomo_device.dev, LOCOMO_GPIO_DTR, 0); }
    else { locomo_gpio_write(&mut collie_locomo_device.dev, LOCOMO_GPIO_DTR, 1); }
}

#[cfg(CONFIG_SHARP_LOCOMO)]
unsafe fn collie_uart_get_mctrl(_port: *mut uart_port) -> u32 {
    let mut ret = TIOCM_CD;
    let r = locomo_gpio_read_output(&mut collie_locomo_device.dev, LOCOMO_GPIO_CTS & LOCOMO_GPIO_DSR);
    if r == -ENODEV { return ret; }
    if r & LOCOMO_GPIO_CTS != 0 { ret |= TIOCM_CTS; }
    if r & LOCOMO_GPIO_DSR != 0 { ret |= TIOCM_DSR; }
    ret
}

#[cfg(CONFIG_SHARP_LOCOMO)]
static mut collie_port_fns: sa1100_port_fns = sa1100_port_fns { set_mctrl: Some(collie_uart_set_mctrl), get_mctrl: Some(collie_uart_get_mctrl) };

#[cfg(CONFIG_SHARP_LOCOMO)]
unsafe fn collie_uart_probe(_dev: *mut locomo_dev) -> i32 { 0 }

#[cfg(CONFIG_SHARP_LOCOMO)]
static mut collie_uart_driver: locomo_driver = locomo_driver { drv: driver { name: "collie_uart" }, devid: LOCOMO_DEVID_UART, probe: Some(collie_uart_probe) };

#[cfg(CONFIG_SHARP_LOCOMO)]
unsafe fn collie_uart_init() -> i32 { locomo_driver_register(&mut collie_uart_driver) }

static mut locomo_resources: [resource; 2] = [
    resource { start: 0x40000000, end: 0x40001fff, flags: IORESOURCE_MEM },
    resource { start: IRQ_GPIO25, end: IRQ_GPIO25, flags: IORESOURCE_IRQ },
];
static mut locomo_info: locomo_platform_data = locomo_platform_data { irq_base: IRQ_BOARD_START };
pub static mut collie_locomo_device: platform_device = platform_device { name: "locomo", id: 0, dev: device { platform_data: unsafe { &mut locomo_info as *mut _ as *mut c_void } }, num_resources: 2, resource: unsafe { locomo_resources.as_mut_ptr() } };

static collie_gpio_keys_node: software_node = software_node { name: "collie-gpio-keys" };
static collie_on_key_props: [property_entry; 6] = [
    PROPERTY_ENTRY_U32("linux,code", KEY_RESERVED),
    PROPERTY_ENTRY_GPIO("gpios", &sa1100_gpiochip_node, COLLIE_GPIO_ON_KEY, GPIO_ACTIVE_LOW),
    PROPERTY_ENTRY_STRING("label", "On key"), PROPERTY_ENTRY_U32("linux,input-type", EV_PWR),
    PROPERTY_ENTRY_BOOL("wakeup-source"), PROPERTY_ENTRY_END,
];
static collie_on_key_node: software_node = software_node { parent: &collie_gpio_keys_node, properties: collie_on_key_props.as_ptr() };
static collie_wakeup_key_props: [property_entry; 6] = [
    PROPERTY_ENTRY_U32("linux,code", KEY_WAKEUP),
    PROPERTY_ENTRY_GPIO("gpios", &sa1100_gpiochip_node, COLLIE_GPIO_WAKEUP, GPIO_ACTIVE_LOW),
    PROPERTY_ENTRY_STRING("label", "Sync"), PROPERTY_ENTRY_U32("linux,input-type", EV_PWR),
    PROPERTY_ENTRY_BOOL("wakeup-source"), PROPERTY_ENTRY_END,
];
static collie_wakeup_key_node: software_node = software_node { parent: &collie_gpio_keys_node, properties: collie_wakeup_key_props.as_ptr() };
static collie_gpio_keys_swnodes: [*const software_node; 4] = [&collie_gpio_keys_node, &collie_on_key_node, &collie_wakeup_key_node, core::ptr::null()];
static collie_gpio_keys_dev_info: platform_device_info = platform_device_info { name: "gpio-keys", id: PLATFORM_DEVID_NONE, swnode: &collie_gpio_keys_node };
static mut devices: [*mut platform_device; 3] = [unsafe { &mut collie_locomo_device }, unsafe { &mut colliescoop_device }, unsafe { &mut collie_power_device }];

static mut collie_partitions: [mtd_partition; 4] = [
    mtd_partition { name: "bootloader", offset: 0, size: 0x000c0000, mask_flags: MTD_WRITEABLE },
    mtd_partition { name: "kernel", offset: MTDPART_OFS_APPEND, size: 0x00100000, ..mtd_partition::default() },
    mtd_partition { name: "rootfs", offset: MTDPART_OFS_APPEND, size: 0x00e20000, ..mtd_partition::default() },
    mtd_partition { name: "bootblock", offset: MTDPART_OFS_APPEND, size: 0x00020000, mask_flags: MTD_WRITEABLE },
];

unsafe fn collie_flash_init() -> i32 { let mut rc = gpio_request(COLLIE_GPIO_VPEN, "flash Vpp enable"); if rc != 0 { return rc; } rc = gpio_direction_output(COLLIE_GPIO_VPEN, 1); if rc != 0 { gpio_free(COLLIE_GPIO_VPEN); } rc }
unsafe fn collie_set_vpp(vpp: i32) { gpio_set_value(COLLIE_GPIO_VPEN, vpp); }
unsafe fn collie_flash_exit() { gpio_free(COLLIE_GPIO_VPEN); }
static mut collie_flash_data: flash_platform_data = flash_platform_data { map_name: "cfi_probe", init: Some(collie_flash_init), set_vpp: Some(collie_set_vpp), exit: Some(collie_flash_exit), parts: collie_partitions.as_mut_ptr(), nr_parts: 4 };
static mut collie_flash_resources: [resource; 1] = [resource { start: SA1100_CS0_PHYS, end: SA1100_CS0_PHYS + SZ_32M - 1, flags: IORESOURCE_MEM }];

static mut collie_lcd_info: sa1100fb_mach_info = sa1100fb_mach_info { pixclock: 171521, bpp: 16, xres: 320, yres: 240, hsync_len: 5, vsync_len: 1, left_margin: 11, upper_margin: 2, right_margin: 30, lower_margin: 0, sync: FB_SYNC_HOR_HIGH_ACT | FB_SYNC_VERT_HIGH_ACT, lccr0: LCCR0_Color | LCCR0_Sngl | LCCR0_Act, lccr3: LCCR3_OutEnH | LCCR3_PixRsEdg | LCCR3_ACBsDiv(2), ..sa1100fb_mach_info::default() };

unsafe fn collie_init() {
    GAFR = GPIO_SSP_TXD | GPIO_SSP_SCLK | GPIO_SSP_SFRM | GPIO_SSP_CLK | GPIO_MCP_CLK | GPIO_32_768kHz;
    GPDR = GPIO_LDD8 | GPIO_LDD9 | GPIO_LDD10 | GPIO_LDD11 | GPIO_LDD12 | GPIO_LDD13 | GPIO_LDD14 | GPIO_LDD15 | GPIO_SSP_TXD | GPIO_SSP_SCLK | GPIO_SSP_SFRM | GPIO_SDLC_SCLK | _COLLIE_GPIO_UCB1x00_RESET | _COLLIE_GPIO_nMIC_ON | _COLLIE_GPIO_nREMOCON_ON | GPIO_32_768kHz;
    PPDR = PPC_LDD0 | PPC_LDD1 | PPC_LDD2 | PPC_LDD3 | PPC_LDD4 | PPC_LDD5 | PPC_LDD6 | PPC_LDD7 | PPC_L_PCLK | PPC_L_LCLK | PPC_L_FCLK | PPC_L_BIAS | PPC_TXD1 | PPC_TXD2 | PPC_TXD3 | PPC_TXD4 | PPC_SCLK | PPC_SFRM;
    PWER = 0; PGSR = _COLLIE_GPIO_nREMOCON_ON; PSDR = PPC_RXD1 | PPC_RXD2 | PPC_RXD3 | PPC_RXD4; PCFR = PCFR_OPDE; GPSR |= _COLLIE_GPIO_UCB1x00_RESET;
    sa11x0_ppc_configure_mcp(); platform_scoop_config = &mut collie_pcmcia_config;
    gpiod_add_lookup_table(&mut collie_power_gpiod_table); gpiod_add_lookup_table(&mut collie_battery_gpiod_table);
    let ret = platform_add_devices(devices.as_mut_ptr(), 3); if ret != 0 { printk(KERN_WARNING, "collie: Unable to register LoCoMo device\n"); }
    software_node_register_node_group(collie_gpio_keys_swnodes.as_ptr()); platform_device_register_full(&collie_gpio_keys_dev_info);
    sa11x0_register_lcd(&mut collie_lcd_info); sa11x0_register_mtd(&mut collie_flash_data, collie_flash_resources.as_mut_ptr(), 1); sa11x0_register_mcp(&mut collie_mcp_data); sharpsl_save_param();
}

static mut collie_io_desc: [map_desc; 2] = [
    map_desc { virtual_: 0xe8000000, pfn: __phys_to_pfn(0), length: 0x02000000, type_: MT_DEVICE },
    map_desc { virtual_: 0xea000000, pfn: __phys_to_pfn(0x08000000), length: 0x02000000, type_: MT_DEVICE },
];
unsafe fn collie_map_io() { sa1100_map_io(); iotable_init(collie_io_desc.as_mut_ptr(), 2); #[cfg(CONFIG_SHARP_LOCOMO)] sa1100_register_uart_fns(&mut collie_port_fns); sa1100_register_uart(0, 3); sa1100_register_uart(1, 1); }

// MACHINE_START(COLLIE, "Sharp-Collie")
// .map_io = collie_map_io, .nr_irqs = SA1100_NR_IRQS, .init_irq = sa1100_init_irq,
// .init_time = sa1100_timer_init, .init_machine = collie_init,
// .init_late = sa11x0_init_late, .restart = sa11x0_restart, MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
