// SPDX-License-Identifier: GPL-2.0
/***************************************************************************/

/*
 *	firebee.c -- extra startup code support for the FireBee boards
 *
 *	Copyright (C) 2011, Greg Ungerer (gerg@snapgear.com)
 */

/***************************************************************************/

// Dependencies supplied by the kernel and architecture-specific headers:
// linux/kernel.h, linux/init.h, linux/io.h, linux/platform_device.h,
// linux/mtd/mtd.h, linux/mtd/partitions.h, linux/mtd/physmap.h,
// asm/coldfire.h, asm/mcfsim.h

/***************************************************************************/

/*
 *	8MB of NOR flash fitted to the FireBee board.
 */
const FLASH_PHYS_ADDR: usize = 0xe0000000; /* Physical address of flash */
const FLASH_PHYS_SIZE: usize = 0x00800000; /* Size of flash */

const PART_BOOT_START: usize = 0x00000000; /* Start at bottom of flash */
const PART_BOOT_SIZE: usize = 0x00040000; /* 256k in size */
const PART_IMAGE_START: usize = 0x00040000; /* Start after boot loader */
const PART_IMAGE_SIZE: usize = 0x006c0000; /* Most of flash */
const PART_FPGA_START: usize = 0x00700000; /* Start at offset 7MB */
const PART_FPGA_SIZE: usize = 0x00100000; /* 1MB in size */

static mut firebee_flash_parts: [mtd_partition; 3] = [
    mtd_partition {
        name: "dBUG",
        offset: PART_BOOT_START,
        size: PART_BOOT_SIZE,
    },
    mtd_partition {
        name: "FPGA",
        offset: PART_FPGA_START,
        size: PART_FPGA_SIZE,
    },
    mtd_partition {
        name: "image",
        offset: PART_IMAGE_START,
        size: PART_IMAGE_SIZE,
    },
];

static mut firebee_flash_data: physmap_flash_data = physmap_flash_data {
    width: 2,
    nr_parts: firebee_flash_parts.len(),
    parts: unsafe { firebee_flash_parts.as_mut_ptr() },
};

static mut firebee_flash_resource: resource = resource {
    start: FLASH_PHYS_ADDR,
    end: FLASH_PHYS_ADDR + FLASH_PHYS_SIZE,
    flags: IORESOURCE_MEM,
};

static mut firebee_flash: platform_device = platform_device {
    name: "physmap-flash",
    id: 0,
    dev: device {
        platform_data: unsafe {
            &mut firebee_flash_data as *mut physmap_flash_data as *mut core::ffi::c_void
        },
    },
    num_resources: 1,
    resource: unsafe { &mut firebee_flash_resource as *mut resource },
};

/***************************************************************************/

unsafe extern "C" {
    fn platform_device_register(device: *mut platform_device) -> i32;
}

unsafe fn init_firebee() -> i32
{
    platform_device_register(&mut firebee_flash);
    0
}

// arch_initcall(init_firebee);

/***************************************************************************/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
