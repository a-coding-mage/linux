/*
 * Broadcom specific AMBA
 * ChipCommon parallel flash
 *
 * Licensed under the GNU/GPL. See COPYING for details.
 */

// C dependencies supplied by the surrounding kernel/BCMA code are intentionally
// left as external declarations here.

use core::ffi::c_void;

pub const IORESOURCE_MEM: u64 = 0x0000_0200;

#[repr(C)]
pub struct physmap_flash_data {
    pub part_probe_types: *const *const u8,
    pub width: u32,
}

#[repr(C)]
pub struct resource {
    pub name: *const u8,
    pub start: usize,
    pub end: usize,
    pub flags: u64,
}

#[repr(C)]
pub struct device {
    pub platform_data: *mut c_void,
}

#[repr(C)]
pub struct platform_device {
    pub name: *const u8,
    pub dev: device,
    pub resource: *mut resource,
    pub num_resources: u32,
}

#[repr(C)]
pub struct bcma_pflash {
    pub present: bool,
}

#[repr(C)]
pub struct bcma_drv_cc {
    pub core: *mut c_void,
    pub pflash: bcma_pflash,
}

unsafe extern "C" {
    pub fn bcma_read32(core: *mut c_void, offset: u32) -> u32;
}

// These values are supplied by the BCMA headers in the original C build.
unsafe extern "C" {
    pub static BCMA_CC_FLASH_CFG: u32;
    pub static BCMA_CC_FLASH_CFG_DS: u32;
    pub static BCMA_SOC_FLASH2: usize;
    pub static BCMA_SOC_FLASH2_SZ: usize;
}

static PART_PROBES_BCM47XXPART: &[u8] = b"bcm47xxpart\0";
static PART_PROBES_NULL: *const u8 = core::ptr::null();
static PART_PROBES: [*const u8; 2] = [
    PART_PROBES_BCM47XXPART.as_ptr(),
    PART_PROBES_NULL,
];

static mut BCMA_PFLASH_DATA: physmap_flash_data = physmap_flash_data {
    part_probe_types: PART_PROBES.as_ptr(),
    width: 0,
};

static mut BCMA_PFLASH_RESOURCE: resource = resource {
    name: b"bcma_pflash\0".as_ptr(),
    start: 0,
    end: 0,
    flags: IORESOURCE_MEM,
};

pub static mut BCMA_PFLASH_DEV: platform_device = platform_device {
    name: b"physmap-flash\0".as_ptr(),
    dev: device {
        platform_data: core::ptr::addr_of_mut!(BCMA_PFLASH_DATA).cast(),
    },
    resource: core::ptr::addr_of_mut!(BCMA_PFLASH_RESOURCE),
    num_resources: 1,
};

pub unsafe fn bcma_pflash_init(cc: *mut bcma_drv_cc) -> i32 {
    let pflash: *mut bcma_pflash = core::ptr::addr_of_mut!((*cc).pflash);

    (*pflash).present = true;

    if (bcma_read32((*cc).core, BCMA_CC_FLASH_CFG)
        & BCMA_CC_FLASH_CFG_DS)
        == 0
    {
        BCMA_PFLASH_DATA.width = 1;
    } else {
        BCMA_PFLASH_DATA.width = 2;
    }

    BCMA_PFLASH_RESOURCE.start = BCMA_SOC_FLASH2;
    BCMA_PFLASH_RESOURCE.end = BCMA_SOC_FLASH2 + BCMA_SOC_FLASH2_SZ;

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
