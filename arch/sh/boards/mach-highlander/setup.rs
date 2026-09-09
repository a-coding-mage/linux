// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/boards/renesas/r7780rp/setup.c
 *
 * Renesas Solutions Highlander Support.
 *
 * Copyright (C) 2002 Atom Create Engineering Co., Ltd.
 * Copyright (C) 2005 - 2008 Paul Mundt
 *
 * This contains support for the R7780RP-1, R7780MP, and R7785RP
 * Highlander modules.
 */

// Linux and architecture dependencies supplied by the surrounding kernel.

static mut r8a66597_data: r8a66597_platdata = r8a66597_platdata {
    xtal: R8A66597_PLATDATA_XTAL_12MHZ,
    vif: 1,
};

static mut r8a66597_usb_host_resources: [resource; 2] = [
    resource { start: 0xA4200000, end: 0xA42000FF, flags: IORESOURCE_MEM, ..resource::default() },
    resource { start: IRQ_EXT1, end: IRQ_EXT1, flags: IORESOURCE_IRQ | IRQF_TRIGGER_LOW, ..resource::default() },
];

static mut r8a66597_usb_host_device: platform_device = platform_device {
    name: "r8a66597_hcd",
    id: -1,
    dev: device {
        dma_mask: core::ptr::null_mut(),
        coherent_dma_mask: 0xffffffff,
        platform_data: unsafe { &mut r8a66597_data },
        ..device::default()
    },
    num_resources: 2,
    resource: unsafe { r8a66597_usb_host_resources.as_mut_ptr() },
    ..platform_device::default()
};

static mut usbf_platdata: m66592_platdata = m66592_platdata {
    xtal: M66592_PLATDATA_XTAL_24MHZ,
    vif: 1,
};

static mut m66592_usb_peripheral_resources: [resource; 2] = [
    resource { name: "m66592_udc", start: 0xb0000000, end: 0xb00000FF, flags: IORESOURCE_MEM, ..resource::default() },
    resource { name: "m66592_udc", start: IRQ_EXT4, end: IRQ_EXT4, flags: IORESOURCE_IRQ, ..resource::default() },
];

static mut m66592_usb_peripheral_device: platform_device = platform_device {
    name: "m66592_udc", id: -1,
    dev: device { dma_mask: core::ptr::null_mut(), coherent_dma_mask: 0xffffffff, platform_data: unsafe { &mut usbf_platdata }, ..device::default() },
    num_resources: 2, resource: unsafe { m66592_usb_peripheral_resources.as_mut_ptr() }, ..platform_device::default()
};

static mut cf_ide_resources: [resource; 3] = [
    resource { start: PA_AREA5_IO + 0x1000, end: PA_AREA5_IO + 0x1000 + 0x08 - 1, flags: IORESOURCE_MEM, ..resource::default() },
    resource { start: PA_AREA5_IO + 0x80c, end: PA_AREA5_IO + 0x80c + 0x16 - 1, flags: IORESOURCE_MEM, ..resource::default() },
    resource { start: IRQ_CF, flags: IORESOURCE_IRQ, ..resource::default() },
];

static mut pata_info: pata_platform_info = pata_platform_info { ioport_shift: 1, ..pata_platform_info::default() };
static mut cf_ide_device: platform_device = platform_device {
    name: "pata_platform", id: -1, num_resources: 3,
    resource: unsafe { cf_ide_resources.as_mut_ptr() },
    dev: device { platform_data: unsafe { &mut pata_info }, ..device::default() }, ..platform_device::default()
};

static mut heartbeat_resources: [resource; 1] = [resource { start: PA_OBLED, end: PA_OBLED, flags: IORESOURCE_MEM, ..resource::default() }];

#[cfg(not(CONFIG_SH_R7785RP))]
static mut heartbeat_bit_pos: [u8; 8] = [2, 1, 0, 3, 6, 5, 4, 7];
#[cfg(not(CONFIG_SH_R7785RP))]
static mut heartbeat_data: heartbeat_data = heartbeat_data { bit_pos: unsafe { heartbeat_bit_pos.as_mut_ptr() }, nr_bits: 8 };

static mut heartbeat_device: platform_device = platform_device {
    name: "heartbeat", id: -1,
    #[cfg(not(CONFIG_SH_R7785RP))]
    dev: device { platform_data: unsafe { &mut heartbeat_data }, ..device::default() },
    num_resources: 1, resource: unsafe { heartbeat_resources.as_mut_ptr() }, ..platform_device::default()
};

static mut ax88796_platdata: ax_plat_data = ax_plat_data { flags: AXFLG_HAS_93CX6, wordlength: 2, dcr_val: 0x1, rcr_val: 0x40 };
static mut ax88796_resources: [resource; 2] = [
    resource { start: {
        #[cfg(CONFIG_SH_R7780RP)] { 0xa5800400 }
        #[cfg(not(CONFIG_SH_R7780RP))] { 0xa4100400 }
    }, end: 0, flags: IORESOURCE_MEM, ..resource::default() },
    resource { start: IRQ_AX88796, end: IRQ_AX88796, flags: IORESOURCE_IRQ, ..resource::default() },
];
static mut ax88796_device: platform_device = platform_device {
    name: "ax88796", id: 0, dev: device { platform_data: unsafe { &mut ax88796_platdata }, ..device::default() },
    num_resources: 2, resource: unsafe { ax88796_resources.as_mut_ptr() }, ..platform_device::default()
};

static mut nor_flash_partitions: [mtd_partition; 4] = [
    mtd_partition { name: "loader", offset: 0x00000000, size: 512 * 1024, ..mtd_partition::default() },
    mtd_partition { name: "bootenv", offset: MTDPART_OFS_APPEND, size: 512 * 1024, ..mtd_partition::default() },
    mtd_partition { name: "kernel", offset: MTDPART_OFS_APPEND, size: 4 * 1024 * 1024, ..mtd_partition::default() },
    mtd_partition { name: "data", offset: MTDPART_OFS_APPEND, size: MTDPART_SIZ_FULL, ..mtd_partition::default() },
];
static mut nor_flash_data: physmap_flash_data = physmap_flash_data { width: 4, parts: unsafe { nor_flash_partitions.as_mut_ptr() }, nr_parts: 4 };
static mut nor_flash_resources: [resource; 1] = [resource { start: PA_NORFLASH_ADDR, end: PA_NORFLASH_ADDR + PA_NORFLASH_SIZE - 1, flags: IORESOURCE_MEM, ..resource::default() }];
static mut nor_flash_device: platform_device = platform_device { name: "physmap-flash", dev: device { platform_data: unsafe { &mut nor_flash_data }, ..device::default() }, num_resources: 1, resource: unsafe { nor_flash_resources.as_mut_ptr() }, ..platform_device::default() };

static mut smbus_resources: [resource; 2] = [
    resource { start: PA_SMCR, end: PA_SMCR + 0x100 - 1, flags: IORESOURCE_MEM, ..resource::default() },
    resource { start: IRQ_SMBUS, end: IRQ_SMBUS, flags: IORESOURCE_IRQ, ..resource::default() },
];
static mut smbus_device: platform_device = platform_device { name: "i2c-highlander", id: 0, num_resources: 2, resource: unsafe { smbus_resources.as_mut_ptr() }, ..platform_device::default() };
static mut highlander_i2c_devices: [i2c_board_info; 1] = [I2C_BOARD_INFO!("r2025sd", 0x32)];
static mut r7780rp_devices: [*mut platform_device; 5] = unsafe { [&mut r8a66597_usb_host_device, &mut m66592_usb_peripheral_device, &mut heartbeat_device, &mut smbus_device, &mut nor_flash_device] };

/* The CF is connected using a 16-bit bus; trap 8-bit ATA operations. */
static mut cf_trapped_io: trapped_io = trapped_io { resource: unsafe { cf_ide_resources.as_mut_ptr() }, num_resources: 2, minimum_bus_width: 16 };

unsafe fn r7780rp_devices_setup() -> i32 {
    let mut ret = 0;
    // CONFIG_SH_R7780RP conditionally includes trapped CF I/O registration.
    if register_trapped_io(&mut cf_trapped_io) == 0 { ret |= platform_device_register(&mut cf_ide_device); }
    ret |= platform_add_devices(r7780rp_devices.as_mut_ptr(), 5);
    ret |= i2c_register_board_info(0, highlander_i2c_devices.as_mut_ptr(), 1);
    ret
}
device_initcall!(r7780rp_devices_setup);

static unsafe fn ivdr_clk_enable(_clk: *mut clk) -> i32 { __raw_writew(__raw_readw(PA_IVDRCTL) | (1 << IVDR_CK_ON), PA_IVDRCTL); 0 }
static unsafe fn ivdr_clk_disable(_clk: *mut clk) { __raw_writew(__raw_readw(PA_IVDRCTL) & !(1 << IVDR_CK_ON), PA_IVDRCTL); }
static mut ivdr_clk_ops: sh_clk_ops = sh_clk_ops { enable: Some(ivdr_clk_enable), disable: Some(ivdr_clk_disable), ..sh_clk_ops::default() };
static mut ivdr_clk: clk = clk { ops: unsafe { &mut ivdr_clk_ops }, ..clk::default() };
static mut r7780rp_clocks: [*mut clk; 1] = [unsafe { &mut ivdr_clk }];
static mut lookups: [clk_lookup; 1] = [CLKDEV_CON_ID!("ivdr_clk", unsafe { &mut ivdr_clk })];

unsafe fn r7780rp_power_off() { if mach_is_r7780mp() || mach_is_r7785rp() { __raw_writew(0x0001, PA_POFF); } }

unsafe fn highlander_setup(_cmdline_p: *mut *mut u8) {
    let ver: u16 = __raw_readw(PA_VERREG);
    printk!(KERN_INFO, "Renesas Solutions Highlander %s support.\\n", if mach_is_r7780rp() { "R7780RP-1" } else if mach_is_r7780mp() { "R7780MP" } else { "R7785RP" });
    printk!(KERN_INFO, "Board version: %d (revision %d), FPGA version: %d (revision %d)\\n", (ver >> 12) & 0xf, (ver >> 8) & 0xf, (ver >> 4) & 0xf, ver & 0xf);
    highlander_plat_pinmux_setup();
    for i in 0..1 { let clk = r7780rp_clocks[i]; clk_register(clk); clk_enable(clk); }
    clkdev_add_table(lookups.as_mut_ptr(), 1);
    __raw_writew(0x0000, PA_OBLED);
    if mach_is_r7780rp() { __raw_writew(0x0001, PA_SDPOW); }
    __raw_writew(__raw_readw(PA_IVDRCTL) | 0x01, PA_IVDRCTL);
    pm_power_off = Some(r7780rp_power_off);
}

static mut irl2irq: [u8; HL_NR_IRL] = [0; HL_NR_IRL];
unsafe fn highlander_irq_demux(irq: i32) -> i32 { if irq >= HL_NR_IRL + 16 || irq < 16 || irl2irq[(irq - 16) as usize] == 0 { irq } else { irl2irq[(irq - 16) as usize] as i32 } }
unsafe fn highlander_init_irq() { let ucp = highlander_plat_irq_setup(); if !ucp.is_null() { plat_irq_setup_pins(IRQ_MODE_IRL3210); core::ptr::copy_nonoverlapping(ucp, irl2irq.as_mut_ptr(), HL_NR_IRL); } }

/* The Machine Vector */
static mut mv_highlander: sh_machine_vector = sh_machine_vector {
    mv_name: "Highlander", mv_setup: Some(highlander_setup), mv_init_irq: Some(highlander_init_irq), mv_irq_demux: Some(highlander_irq_demux), ..sh_machine_vector::default()
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
