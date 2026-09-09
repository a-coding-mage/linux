/*
 * Broadcom BCM63xx flash registration
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2008 Maxime Bizon <mbizon@freebox.fr>
 * Copyright (C) 2008 Florian Fainelli <florian@openwrt.org>
 * Copyright (C) 2012 Jonas Gorski <jonas.gorski@gmail.com>
 */

// External declarations are supplied by the corresponding Linux BCM63xx headers.

#[repr(C)]
pub struct MtdPartition {
    pub name: *const ::core::ffi::c_char,
    pub offset: u64,
    pub size: u64,
}

#[repr(C)]
pub struct PhysmapFlashData {
    pub width: u32,
    pub parts: *mut MtdPartition,
    pub part_probe_types: *const *const ::core::ffi::c_char,
}

#[repr(C)]
pub struct Resource {
    pub start: u32,
    pub end: u32,
    pub flags: u64,
}

#[repr(C)]
pub struct Device {
    pub platform_data: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct PlatformDevice {
    pub name: *const ::core::ffi::c_char,
    pub resource: *mut Resource,
    pub num_resources: usize,
    pub dev: Device,
}

extern "C" {
    fn bcm63xx_get_cpu_id() -> u32;
    fn bcm_misc_readl(reg: u32) -> u32;
    fn bcm_gpio_readl(reg: u32) -> u32;
    fn bcm_mpi_readl(reg: u32) -> u32;
    fn platform_device_register(dev: *mut PlatformDevice) -> i32;
    fn pr_warn(fmt: *const ::core::ffi::c_char, ...);
    fn pr_err(fmt: *const ::core::ffi::c_char, ...);
}

extern "C" {
    static BCM6328_CPU_ID: u32;
    static BCM6338_CPU_ID: u32;
    static BCM6345_CPU_ID: u32;
    static BCM6348_CPU_ID: u32;
    static BCM3368_CPU_ID: u32;
    static BCM6358_CPU_ID: u32;
    static BCM6362_CPU_ID: u32;
    static BCM6368_CPU_ID: u32;
}

static mut MTD_PARTITIONS: [MtdPartition; 1] = [MtdPartition {
    name: b"cfe\0".as_ptr() as *const _,
    offset: 0x0,
    size: 0x40000,
}];

static BCM63XX_PART_TYPES: [*const ::core::ffi::c_char; 2] =
    [b"bcm63xxpart\0".as_ptr() as *const _, ::core::ptr::null()];

static mut FLASH_DATA: PhysmapFlashData = PhysmapFlashData {
    width: 2,
    parts: MTD_PARTITIONS.as_ptr() as *mut MtdPartition,
    part_probe_types: BCM63XX_PART_TYPES.as_ptr(),
};

static mut MTD_RESOURCES: [Resource; 1] = [Resource {
    start: 0, // filled at runtime
    end: 0,   // filled at runtime
    flags: IORESOURCE_MEM,
}];

static mut MTD_DEV: PlatformDevice = PlatformDevice {
    name: b"physmap-flash\0".as_ptr() as *const _,
    resource: MTD_RESOURCES.as_ptr() as *mut Resource,
    num_resources: 1,
    dev: Device { platform_data: &mut FLASH_DATA as *mut _ as *mut ::core::ffi::c_void },
};

pub unsafe fn bcm63xx_detect_flash_type() -> i32 {
    let mut val: u32;

    match bcm63xx_get_cpu_id() {
        BCM6328_CPU_ID => {
            val = bcm_misc_readl(MISC_STRAPBUS_6328_REG);
            if val & STRAPBUS_6328_BOOT_SEL_SERIAL != 0 {
                BCM63XX_FLASH_TYPE_SERIAL
            } else {
                BCM63XX_FLASH_TYPE_NAND
            }
        }
        BCM6338_CPU_ID | BCM6345_CPU_ID | BCM6348_CPU_ID => {
            // no way to auto detect so assume parallel
            BCM63XX_FLASH_TYPE_PARALLEL
        }
        BCM3368_CPU_ID | BCM6358_CPU_ID => {
            val = bcm_gpio_readl(GPIO_STRAPBUS_REG);
            if val & STRAPBUS_6358_BOOT_SEL_PARALLEL != 0 {
                BCM63XX_FLASH_TYPE_PARALLEL
            } else {
                BCM63XX_FLASH_TYPE_SERIAL
            }
        }
        BCM6362_CPU_ID => {
            val = bcm_misc_readl(MISC_STRAPBUS_6362_REG);
            if val & STRAPBUS_6362_BOOT_SEL_SERIAL != 0 {
                BCM63XX_FLASH_TYPE_SERIAL
            } else {
                BCM63XX_FLASH_TYPE_NAND
            }
        }
        BCM6368_CPU_ID => {
            val = bcm_gpio_readl(GPIO_STRAPBUS_REG);
            match val & STRAPBUS_6368_BOOT_SEL_MASK {
                STRAPBUS_6368_BOOT_SEL_NAND => BCM63XX_FLASH_TYPE_NAND,
                STRAPBUS_6368_BOOT_SEL_SERIAL => BCM63XX_FLASH_TYPE_SERIAL,
                STRAPBUS_6368_BOOT_SEL_PARALLEL => BCM63XX_FLASH_TYPE_PARALLEL,
                _ => -EINVAL,
            }
        }
        _ => -EINVAL,
    }
}

pub unsafe fn bcm63xx_flash_register() -> i32 {
    let flash_type = bcm63xx_detect_flash_type();
    let mut val: u32;

    match flash_type {
        BCM63XX_FLASH_TYPE_PARALLEL => {
            // read base address of boot chip select (0)
            val = bcm_mpi_readl(MPI_CSBASE_REG(0));
            val &= MPI_CSBASE_BASE_MASK;
            MTD_RESOURCES[0].start = val;
            MTD_RESOURCES[0].end = 0x1FFFFFFF;
            platform_device_register(&mut MTD_DEV)
        }
        BCM63XX_FLASH_TYPE_SERIAL => {
            pr_warn(b"unsupported serial flash detected\n\0".as_ptr() as *const _);
            -ENODEV
        }
        BCM63XX_FLASH_TYPE_NAND => {
            pr_warn(b"unsupported NAND flash detected\n\0".as_ptr() as *const _);
            -ENODEV
        }
        _ => {
            pr_err(b"flash detection failed for BCM%x: %d\n\0".as_ptr() as *const _, bcm63xx_get_cpu_id(), flash_type);
            -ENODEV
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
