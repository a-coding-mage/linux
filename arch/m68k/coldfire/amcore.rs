/*
 * amcore.c -- Support for Kernelspace AMCORE open board
 *
 * (C) Copyright 2026, Angelo Dureghello <angelo@kernel-space.org>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

// Declarations supplied by the Linux and ColdFire headers included by amcore.c

use core::ffi::c_void;

#[repr(C)]
pub struct resource {
    pub start: usize,
    pub end: usize,
    pub flags: u32,
}

#[repr(C)]
pub struct dm9000_plat_data {
    pub flags: u32,
}

#[repr(C)]
pub struct device {
    pub platform_data: *mut c_void,
}

#[repr(C)]
pub struct platform_device {
    pub name: *const i8,
    pub id: i32,
    pub num_resources: usize,
    pub resource: *mut resource,
    pub dev: device,
}

#[repr(C)]
pub struct mtd_partition {
    pub name: *const i8,
    pub size: usize,
    pub offset: usize,
}

#[repr(C)]
pub struct physmap_flash_data {
    pub parts: *mut mtd_partition,
    pub nr_parts: usize,
    pub width: u32,
}

#[repr(C)]
pub struct i2c_board_info {
    pub type_: [i8; 20],
    pub addr: u16,
}

unsafe extern "C" {
    fn mcf_autovector(irq: u32);
    fn i2c_register_board_info(busnum: i32, info: *mut i2c_board_info, len: usize) -> i32;
    fn platform_add_devices(devices: *mut *mut platform_device, num: usize) -> i32;
}

const DM9000_IRQ: u32 = 25;
const DM9000_ADDR: usize = 0x30000000;
const IORESOURCE_MEM: u32 = 0;
const IORESOURCE_IRQ: u32 = 0;
const DM9000_PLATF_32BITONLY: u32 = 0;
const MTDPART_OFS_APPEND: usize = 0;
const MTDPART_SIZ_FULL: usize = 0;

// DEVICES and related device RESOURCES
#[cfg(feature = "config_dm9000")]
static mut dm9000_resources: [resource; 3] = [
    // physical address of the address register (CMD [A2] to 0)
    resource { start: DM9000_ADDR, end: DM9000_ADDR, flags: IORESOURCE_MEM },
    // physical address of the data register (CMD [A2] to 1),
    // driver wants a range >=4 to assume a 32bit data bus
    resource { start: DM9000_ADDR + 4, end: DM9000_ADDR + 7, flags: IORESOURCE_MEM },
    // IRQ line the device's interrupt pin is connected to
    resource { start: DM9000_IRQ as usize, end: DM9000_IRQ as usize, flags: IORESOURCE_IRQ },
];

#[cfg(feature = "config_dm9000")]
static mut dm9000_platdata: dm9000_plat_data = dm9000_plat_data {
    flags: DM9000_PLATF_32BITONLY,
};

#[cfg(feature = "config_dm9000")]
static mut dm9000_device: platform_device = platform_device {
    name: b"dm9000\0".as_ptr() as *const i8,
    id: 0,
    num_resources: 3,
    resource: unsafe { dm9000_resources.as_mut_ptr() },
    dev: device {
        platform_data: unsafe { &mut dm9000_platdata as *mut dm9000_plat_data as *mut c_void },
    },
};

unsafe fn dm9000_pre_init() {
    // Set the dm9000 interrupt to be auto-vectored
    unsafe { mcf_autovector(DM9000_IRQ) };
}

// Partitioning of parallel NOR flash (39VF3201B)
static mut amcore_partitions: [mtd_partition; 3] = [
    mtd_partition { name: b"U-Boot (128K)\0".as_ptr() as *const i8, size: 0x20000, offset: 0x0 },
    mtd_partition { name: b"Kernel+ROMfs (2994K)\0".as_ptr() as *const i8, size: 0x2E0000, offset: MTDPART_OFS_APPEND },
    mtd_partition { name: b"Flash Free Space (1024K)\0".as_ptr() as *const i8, size: MTDPART_SIZ_FULL, offset: MTDPART_OFS_APPEND },
];

static mut flash_data: physmap_flash_data = physmap_flash_data {
    parts: unsafe { amcore_partitions.as_mut_ptr() },
    nr_parts: 3,
    width: 2,
};

static mut flash_resource: resource = resource {
    start: 0xffc00000,
    end: 0xffffffff,
    flags: IORESOURCE_MEM,
};

static mut flash_device: platform_device = platform_device {
    name: b"physmap-flash\0".as_ptr() as *const i8,
    id: -1,
    resource: unsafe { &mut flash_resource },
    num_resources: 1,
    dev: device {
        platform_data: unsafe { &mut flash_data as *mut physmap_flash_data as *mut c_void },
    },
};

static mut rtc_device: platform_device = platform_device {
    name: b"rtc-ds1307\0".as_ptr() as *const i8,
    id: -1,
    num_resources: 0,
    resource: core::ptr::null_mut(),
    dev: device { platform_data: core::ptr::null_mut() },
};

static mut amcore_i2c_info: [i2c_board_info; 1] = [i2c_board_info {
    type_: [b'd' as i8, b's' as i8, b'1' as i8, b'3' as i8, b'3' as i8, b'8' as i8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    addr: 0x68,
}];

static mut amcore_devices: [*mut platform_device; 3] = [
    #[cfg(feature = "config_dm9000")]
    unsafe { &mut dm9000_device },
    unsafe { &mut flash_device },
    unsafe { &mut rtc_device },
];

unsafe fn init_amcore() -> i32 {
    #[cfg(feature = "config_dm9000")]
    unsafe { dm9000_pre_init(); }

    // Add i2c RTC Dallas chip supprt
    unsafe {
        i2c_register_board_info(0, amcore_i2c_info.as_mut_ptr(), amcore_i2c_info.len());
        platform_add_devices(amcore_devices.as_mut_ptr(), amcore_devices.len());
    }

    0
}

// arch_initcall(init_amcore)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
