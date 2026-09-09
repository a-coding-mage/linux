// SPDX-License-Identifier: GPL-2.0
/*
 * Renesas Technology Europe RSK+ Support.
 *
 * Copyright (C) 2008 Paul Mundt
 * Copyright (C) 2008 Peter Griffin <pgriffin@mpc-data.co.uk>
 */

/* Dependencies supplied by the surrounding kernel bindings. */

/* Dummy supplies, where voltage doesn't matter */
static mut DUMMY_SUPPLIES: [regulator_consumer_supply; 2] = [
    regulator_consumer_supply {
        supply: b"vddvario\\0".as_ptr() as *const i8,
        dev_name: b"smsc911x\\0".as_ptr() as *const i8,
    },
    regulator_consumer_supply {
        supply: b"vdd33a\\0".as_ptr() as *const i8,
        dev_name: b"smsc911x\\0".as_ptr() as *const i8,
    },
];

static mut RSK_PARTITIONS: [mtd_partition; 3] = [
    mtd_partition {
        name: b"Bootloader\\0".as_ptr() as *const i8,
        offset: 0x00000000,
        size: 0x00040000,
        mask_flags: MTD_WRITEABLE,
    },
    mtd_partition {
        name: b"Kernel\\0".as_ptr() as *const i8,
        offset: MTDPART_OFS_NXTBLK,
        size: 0x001c0000,
        ..unsafe { core::mem::zeroed() }
    },
    mtd_partition {
        name: b"Flash_FS\\0".as_ptr() as *const i8,
        offset: MTDPART_OFS_NXTBLK,
        size: MTDPART_SIZ_FULL,
        ..unsafe { core::mem::zeroed() }
    },
];

static mut FLASH_DATA: physmap_flash_data = physmap_flash_data {
    parts: unsafe { RSK_PARTITIONS.as_ptr() as *mut mtd_partition },
    nr_parts: 3,
    width: 2,
};

static mut FLASH_RESOURCE: resource = resource {
    start: 0x20000000,
    end: 0x20400000,
    flags: IORESOURCE_MEM,
    ..unsafe { core::mem::zeroed() }
};

static mut FLASH_DEVICE: platform_device = platform_device {
    name: b"physmap-flash\\0".as_ptr() as *const i8,
    id: -1,
    resource: unsafe { &mut FLASH_RESOURCE },
    num_resources: 1,
    dev: device {
        platform_data: unsafe { &mut FLASH_DATA as *mut _ as *mut core::ffi::c_void },
        ..unsafe { core::mem::zeroed() }
    },
    ..unsafe { core::mem::zeroed() }
};

static mut RSK_DEVICES: [*mut platform_device; 1] = [unsafe { &mut FLASH_DEVICE }];

unsafe extern "C" fn rsk_devices_setup() -> i32 {
    regulator_register_fixed(0, DUMMY_SUPPLIES.as_mut_ptr(), DUMMY_SUPPLIES.len());

    platform_add_devices(RSK_DEVICES.as_mut_ptr(), RSK_DEVICES.len())
}

device_initcall!(rsk_devices_setup);

/*
 * The Machine Vector
 */
static mut MV_RSK: sh_machine_vector = sh_machine_vector {
    mv_name: b"RSK+\\0".as_ptr() as *const i8,
    ..unsafe { core::mem::zeroed() }
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
