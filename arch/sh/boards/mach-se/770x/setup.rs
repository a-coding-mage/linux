// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/sh/boards/se/770x/setup.c
 *
 * Copyright (C) 2000  Kazumoto Kojima
 *
 * Hitachi SolutionEngine Support.
 *
 */

/* Configure the Super I/O chip */
unsafe fn smsc_config(index: i32, data: i32) {
    outb_p(index, INDEX_PORT);
    outb_p(data, DATA_PORT);
}

/* XXX: Another candidate for a more generic cchip machine vector */
unsafe fn smsc_setup(_cmdline_p: *mut *mut i8) {
    outb_p(CONFIG_ENTER, CONFIG_PORT);
    outb_p(CONFIG_ENTER, CONFIG_PORT);

    /* FDC */
    smsc_config(CURRENT_LDN_INDEX, LDN_FDC);
    smsc_config(ACTIVATE_INDEX, 0x01);
    smsc_config(IRQ_SELECT_INDEX, 6); /* IRQ6 */

    /* AUXIO (GPIO): to use IDE1 */
    smsc_config(CURRENT_LDN_INDEX, LDN_AUXIO);
    smsc_config(GPIO46_INDEX, 0x00); /* nIOROP */
    smsc_config(GPIO47_INDEX, 0x00); /* nIOWOP */

    /* COM1 */
    smsc_config(CURRENT_LDN_INDEX, LDN_COM1);
    smsc_config(ACTIVATE_INDEX, 0x01);
    smsc_config(IO_BASE_HI_INDEX, 0x03);
    smsc_config(IO_BASE_LO_INDEX, 0xf8);
    smsc_config(IRQ_SELECT_INDEX, 4); /* IRQ4 */

    /* COM2 */
    smsc_config(CURRENT_LDN_INDEX, LDN_COM2);
    smsc_config(ACTIVATE_INDEX, 0x01);
    smsc_config(IO_BASE_HI_INDEX, 0x02);
    smsc_config(IO_BASE_LO_INDEX, 0xf8);
    smsc_config(IRQ_SELECT_INDEX, 3); /* IRQ3 */

    /* RTC */
    smsc_config(CURRENT_LDN_INDEX, LDN_RTC);
    smsc_config(ACTIVATE_INDEX, 0x01);
    smsc_config(IRQ_SELECT_INDEX, 8); /* IRQ8 */

    /* XXX: PARPORT, KBD, and MOUSE will come here... */
    outb_p(CONFIG_EXIT, CONFIG_PORT);
}

static mut cf_ide_resources: [struct_resource; 3] = [
    struct_resource { start: PA_MRSHPC_IO + 0x1f0, end: PA_MRSHPC_IO + 0x1f0 + 8, flags: IORESOURCE_MEM },
    struct_resource { start: PA_MRSHPC_IO + 0x1f0 + 0x206, end: PA_MRSHPC_IO + 0x1f0 + 8 + 0x206 + 8, flags: IORESOURCE_MEM },
    struct_resource { start: IRQ_CFCARD, end: 0, flags: IORESOURCE_IRQ },
];

static mut cf_ide_device: platform_device = platform_device {
    name: "pata_platform",
    id: -1,
    num_resources: cf_ide_resources.len(),
    resource: cf_ide_resources.as_mut_ptr(),
};

static mut heartbeat_bit_pos: [u8; 8] = [8, 9, 10, 11, 12, 13, 14, 15];

static mut heartbeat_data: heartbeat_data = heartbeat_data {
    bit_pos: heartbeat_bit_pos.as_mut_ptr(),
    nr_bits: heartbeat_bit_pos.len(),
};

static mut heartbeat_resource: struct_resource = struct_resource {
    start: PA_LED,
    end: PA_LED,
    flags: IORESOURCE_MEM | IORESOURCE_MEM_16BIT,
};

static mut heartbeat_device: platform_device = platform_device {
    name: "heartbeat",
    id: -1,
    dev: device { platform_data: &mut heartbeat_data as *mut _ as *mut core::ffi::c_void },
    num_resources: 1,
    resource: &mut heartbeat_resource,
};

/* Build-time condition preserved from CONFIG_CPU_SUBTYPE_SH7710/SH7712. */
#[cfg(any(CONFIG_CPU_SUBTYPE_SH7710, CONFIG_CPU_SUBTYPE_SH7712))]
static mut sh_eth_plat: sh_eth_plat_data = sh_eth_plat_data {
    phy: PHY_ID,
    phy_interface: PHY_INTERFACE_MODE_MII,
};

#[cfg(any(CONFIG_CPU_SUBTYPE_SH7710, CONFIG_CPU_SUBTYPE_SH7712))]
static mut sh_eth0_resources: [struct_resource; 3] = [
    struct_resource { start: SH_ETH0_BASE, end: SH_ETH0_BASE + 0x1B8 - 1, flags: IORESOURCE_MEM },
    struct_resource { start: SH_TSU_BASE, end: SH_TSU_BASE + 0x200 - 1, flags: IORESOURCE_MEM },
    struct_resource { start: SH_ETH0_IRQ, end: SH_ETH0_IRQ, flags: IORESOURCE_IRQ },
];

#[cfg(any(CONFIG_CPU_SUBTYPE_SH7710, CONFIG_CPU_SUBTYPE_SH7712))]
static mut sh_eth0_device: platform_device = platform_device {
    name: "sh771x-ether", id: 0,
    dev: device { platform_data: &mut sh_eth_plat as *mut _ as *mut core::ffi::c_void },
    num_resources: sh_eth0_resources.len(), resource: sh_eth0_resources.as_mut_ptr(),
};

#[cfg(any(CONFIG_CPU_SUBTYPE_SH7710, CONFIG_CPU_SUBTYPE_SH7712))]
static mut sh_eth1_resources: [struct_resource; 3] = [
    struct_resource { start: SH_ETH1_BASE, end: SH_ETH1_BASE + 0x1B8 - 1, flags: IORESOURCE_MEM },
    struct_resource { start: SH_TSU_BASE, end: SH_TSU_BASE + 0x200 - 1, flags: IORESOURCE_MEM },
    struct_resource { start: SH_ETH1_IRQ, end: SH_ETH1_IRQ, flags: IORESOURCE_IRQ },
];

#[cfg(any(CONFIG_CPU_SUBTYPE_SH7710, CONFIG_CPU_SUBTYPE_SH7712))]
static mut sh_eth1_device: platform_device = platform_device {
    name: "sh771x-ether", id: 1,
    dev: device { platform_data: &mut sh_eth_plat as *mut _ as *mut core::ffi::c_void },
    num_resources: sh_eth1_resources.len(), resource: sh_eth1_resources.as_mut_ptr(),
};

static mut se_devices: [*mut platform_device; 4] = [
    &mut heartbeat_device,
    &mut cf_ide_device,
    #[cfg(any(CONFIG_CPU_SUBTYPE_SH7710, CONFIG_CPU_SUBTYPE_SH7712))]
    &mut sh_eth0_device,
    #[cfg(any(CONFIG_CPU_SUBTYPE_SH7710, CONFIG_CPU_SUBTYPE_SH7712))]
    &mut sh_eth1_device,
];

unsafe fn se_devices_setup() -> i32 {
    mrshpc_setup_windows();
    platform_add_devices(se_devices.as_mut_ptr(), se_devices.len())
}

/* device_initcall(se_devices_setup); */

/* The Machine Vector */
static mut mv_se: sh_machine_vector = sh_machine_vector {
    mv_name: "SolutionEngine",
    mv_setup: Some(smsc_setup),
    mv_init_irq: Some(init_se_IRQ),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
