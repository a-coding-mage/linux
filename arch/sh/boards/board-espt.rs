// SPDX-License-Identifier: GPL-2.0
/*
 * Data Technology Inc. ESPT-GIGA board support
 *
 * Copyright (C) 2008, 2009 Renesas Solutions Corp.
 * Copyright (C) 2008, 2009 Nobuhiro Iwamatsu <iwamatsu.nobuhiro@renesas.com>
 */

// Linux headers and build-time definitions are supplied by the surrounding
// platform.  Their Rust equivalents are intentionally referenced externally.

/* NOR Flash */
static mut espt_nor_flash_partitions: [mtd_partition; 3] = [
    mtd_partition {
        name: "U-Boot",
        offset: 0,
        size: 2 * SZ_128K,
        mask_flags: MTD_WRITEABLE, // Read-only
    },
    mtd_partition {
        name: "Linux-Kernel",
        offset: MTDPART_OFS_APPEND,
        size: 20 * SZ_128K,
        mask_flags: 0,
    },
    mtd_partition {
        name: "Root Filesystem",
        offset: MTDPART_OFS_APPEND,
        size: MTDPART_SIZ_FULL,
        mask_flags: 0,
    },
];

static mut espt_nor_flash_data: physmap_flash_data = physmap_flash_data {
    width: 2,
    parts: espt_nor_flash_partitions.as_ptr() as *mut mtd_partition,
    nr_parts: espt_nor_flash_partitions.len(),
};

static mut espt_nor_flash_resources: [resource; 1] = [resource {
    name: "NOR Flash",
    start: 0,
    end: SZ_8M - 1,
    flags: IORESOURCE_MEM,
}];

static mut espt_nor_flash_device: platform_device = platform_device {
    name: "physmap-flash",
    resource: espt_nor_flash_resources.as_mut_ptr(),
    num_resources: espt_nor_flash_resources.len(),
    dev: device {
        platform_data: &mut espt_nor_flash_data as *mut physmap_flash_data as *mut _,
    },
};

/* SH-Ether */
static mut sh_eth_resources: [resource; 3] = [
    resource {
        start: 0xFEE00800, // use eth1
        end: 0xFEE00F7C - 1,
        flags: IORESOURCE_MEM,
    },
    resource {
        start: 0xFEE01800, // TSU
        end: 0xFEE01FFF,
        flags: IORESOURCE_MEM,
    },
    resource {
        start: unsafe { evt2irq(0x920) }, // irq number
        end: 0,
        flags: IORESOURCE_IRQ,
    },
];

static mut sh7763_eth_pdata: sh_eth_plat_data = sh_eth_plat_data {
    phy: 0,
    phy_interface: PHY_INTERFACE_MODE_MII,
};

static mut espt_eth_device: platform_device = platform_device {
    name: "sh7763-gether",
    resource: sh_eth_resources.as_mut_ptr(),
    num_resources: sh_eth_resources.len(),
    dev: device {
        platform_data: &mut sh7763_eth_pdata as *mut sh_eth_plat_data as *mut _,
    },
};

static mut espt_devices: [*mut platform_device; 2] = [
    &mut espt_nor_flash_device,
    &mut espt_eth_device,
];

unsafe extern "C" {
    fn platform_add_devices(devices: *mut *mut platform_device, count: usize) -> i32;
    fn evt2irq(event: u32) -> u32;
}

unsafe fn espt_devices_setup() -> i32 {
    platform_add_devices(espt_devices.as_mut_ptr(), espt_devices.len())
}

// device_initcall(espt_devices_setup);

static mut mv_espt: sh_machine_vector = sh_machine_vector {
    mv_name: "ESPT-GIGA",
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
