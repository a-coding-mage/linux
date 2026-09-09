/*
 * Broadcom specific AMBA
 * ChipCommon serial flash interface
 *
 * Licensed under the GNU/GPL. See COPYING for details.
 */

// Dependencies supplied by the surrounding kernel/bcma translation unit.

#[repr(C)]
pub struct bcma_sflash_tbl_e {
    pub name: *mut ::core::ffi::c_char,
    pub id: u32,
    pub blocksize: u32,
    pub numblocks: u16,
}

extern "C" {
    fn bcma_cc_write32(cc: *mut bcma_drv_cc, offset: u32, value: u32);
    fn bcma_cc_read32(cc: *mut bcma_drv_cc, offset: u32) -> u32;
    fn bcma_err(bus: *mut bcma_bus, fmt: *const ::core::ffi::c_char, ...);
    fn bcma_info(bus: *mut bcma_bus, fmt: *const ::core::ffi::c_char, ...);
    fn cpu_relax();
}

#[repr(C)]
pub struct bcma_drv_cc {
    pub core: *mut bcma_device,
    pub capabilities: u32,
    pub sflash: bcma_sflash,
}

#[repr(C)]
pub struct bcma_sflash {
    pub blocksize: u32,
    pub numblocks: u16,
    pub size: u32,
    pub present: bool,
}

#[repr(C)]
pub struct bcma_device {
    pub bus: *mut bcma_bus,
}

#[repr(C)]
pub struct bcma_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub name: *const ::core::ffi::c_char,
    pub start: u64,
    pub end: u64,
    pub flags: u64,
}

#[repr(C)]
pub struct device {
    pub platform_data: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct platform_device {
    pub name: *const ::core::ffi::c_char,
    pub resource: *mut resource,
    pub num_resources: u32,
    pub dev: device,
}

const BCMA_SOC_FLASH2: u64 = 0;
const IORESOURCE_MEM: u64 = 0;
const IORESOURCE_READONLY: u64 = 0;
const BCMA_CC_FLASHCTL: u32 = 0;
const BCMA_CC_FLASHADDR: u32 = 0;
const BCMA_CC_FLASHDATA: u32 = 0;
const BCMA_CC_FLASHCTL_START: u32 = 0;
const BCMA_CC_FLASHCTL_BUSY: u32 = 0;
const BCMA_CC_FLASHCTL_ST_DP: u32 = 0;
const BCMA_CC_FLASHCTL_ST_RES: u32 = 0;
const BCMA_CC_FLASHCTL_AT_STATUS: u32 = 0;
const BCMA_CC_CAP_FLASHT: u32 = 0;
const BCMA_CC_FLASHT_STSER: u32 = 0;
const BCMA_CC_FLASHT_ATSER: u32 = 0;
const ENOTSUPP: i32 = 95;

static mut bcma_sflash_resource: resource = resource {
    name: b"bcma_sflash\0".as_ptr() as *const _,
    start: BCMA_SOC_FLASH2,
    end: 0,
    flags: IORESOURCE_MEM | IORESOURCE_READONLY,
};

#[no_mangle]
pub static mut bcma_sflash_dev: platform_device = platform_device {
    name: b"bcma_sflash\0".as_ptr() as *const _,
    resource: core::ptr::addr_of_mut!(bcma_sflash_resource),
    num_resources: 1,
    dev: device { platform_data: core::ptr::null_mut() },
};

static mut bcma_sflash_st_tbl: [bcma_sflash_tbl_e; 8] = [
    bcma_sflash_tbl_e { name: b"M25P20\0".as_ptr() as *mut _, id: 0x11, blocksize: 0x10000, numblocks: 4 },
    bcma_sflash_tbl_e { name: b"M25P40\0".as_ptr() as *mut _, id: 0x12, blocksize: 0x10000, numblocks: 8 },
    bcma_sflash_tbl_e { name: b"M25P16\0".as_ptr() as *mut _, id: 0x14, blocksize: 0x10000, numblocks: 32 },
    bcma_sflash_tbl_e { name: b"M25P32\0".as_ptr() as *mut _, id: 0x15, blocksize: 0x10000, numblocks: 64 },
    bcma_sflash_tbl_e { name: b"M25P64\0".as_ptr() as *mut _, id: 0x16, blocksize: 0x10000, numblocks: 128 },
    bcma_sflash_tbl_e { name: b"M25FL128\0".as_ptr() as *mut _, id: 0x17, blocksize: 0x10000, numblocks: 256 },
    bcma_sflash_tbl_e { name: b"MX25L25635F\0".as_ptr() as *mut _, id: 0x18, blocksize: 0x10000, numblocks: 512 },
    bcma_sflash_tbl_e { name: core::ptr::null_mut(), id: 0, blocksize: 0, numblocks: 0 },
];

static mut bcma_sflash_sst_tbl: [bcma_sflash_tbl_e; 14] = [
    bcma_sflash_tbl_e { name: b"SST25WF512\0".as_ptr() as *mut _, id: 1, blocksize: 0x1000, numblocks: 16 },
    bcma_sflash_tbl_e { name: b"SST25VF512\0".as_ptr() as *mut _, id: 0x48, blocksize: 0x1000, numblocks: 16 },
    bcma_sflash_tbl_e { name: b"SST25WF010\0".as_ptr() as *mut _, id: 2, blocksize: 0x1000, numblocks: 32 },
    bcma_sflash_tbl_e { name: b"SST25VF010\0".as_ptr() as *mut _, id: 0x49, blocksize: 0x1000, numblocks: 32 },
    bcma_sflash_tbl_e { name: b"SST25WF020\0".as_ptr() as *mut _, id: 3, blocksize: 0x1000, numblocks: 64 },
    bcma_sflash_tbl_e { name: b"SST25VF020\0".as_ptr() as *mut _, id: 0x43, blocksize: 0x1000, numblocks: 64 },
    bcma_sflash_tbl_e { name: b"SST25WF040\0".as_ptr() as *mut _, id: 4, blocksize: 0x1000, numblocks: 128 },
    bcma_sflash_tbl_e { name: b"SST25VF040\0".as_ptr() as *mut _, id: 0x44, blocksize: 0x1000, numblocks: 128 },
    bcma_sflash_tbl_e { name: b"SST25VF040B\0".as_ptr() as *mut _, id: 0x8d, blocksize: 0x1000, numblocks: 128 },
    bcma_sflash_tbl_e { name: b"SST25WF080\0".as_ptr() as *mut _, id: 5, blocksize: 0x1000, numblocks: 256 },
    bcma_sflash_tbl_e { name: b"SST25VF080B\0".as_ptr() as *mut _, id: 0x8e, blocksize: 0x1000, numblocks: 256 },
    bcma_sflash_tbl_e { name: b"SST25VF016\0".as_ptr() as *mut _, id: 0x41, blocksize: 0x1000, numblocks: 512 },
    bcma_sflash_tbl_e { name: b"SST25VF032\0".as_ptr() as *mut _, id: 0x4a, blocksize: 0x1000, numblocks: 1024 },
    bcma_sflash_tbl_e { name: b"SST25VF064\0".as_ptr() as *mut _, id: 0x4b, blocksize: 0x1000, numblocks: 2048 },
];

static mut bcma_sflash_at_tbl: [bcma_sflash_tbl_e; 7] = [
    bcma_sflash_tbl_e { name: b"AT45DB011\0".as_ptr() as *mut _, id: 0xc, blocksize: 256, numblocks: 512 },
    bcma_sflash_tbl_e { name: b"AT45DB021\0".as_ptr() as *mut _, id: 0x14, blocksize: 256, numblocks: 1024 },
    bcma_sflash_tbl_e { name: b"AT45DB041\0".as_ptr() as *mut _, id: 0x1c, blocksize: 256, numblocks: 2048 },
    bcma_sflash_tbl_e { name: b"AT45DB081\0".as_ptr() as *mut _, id: 0x24, blocksize: 256, numblocks: 4096 },
    bcma_sflash_tbl_e { name: b"AT45DB161\0".as_ptr() as *mut _, id: 0x2c, blocksize: 512, numblocks: 4096 },
    bcma_sflash_tbl_e { name: b"AT45DB321\0".as_ptr() as *mut _, id: 0x34, blocksize: 512, numblocks: 8192 },
    bcma_sflash_tbl_e { name: b"AT45DB642\0".as_ptr() as *mut _, id: 0x3c, blocksize: 1024, numblocks: 8192 },
];

unsafe fn bcma_sflash_cmd(cc: *mut bcma_drv_cc, opcode: u32) {
    bcma_cc_write32(cc, BCMA_CC_FLASHCTL, BCMA_CC_FLASHCTL_START | opcode);
    for _ in 0..1000 {
        if bcma_cc_read32(cc, BCMA_CC_FLASHCTL) & BCMA_CC_FLASHCTL_BUSY == 0 { return; }
        cpu_relax();
    }
    bcma_err((*(*cc).core).bus, b"SFLASH control command failed (timeout)!\n\0".as_ptr() as *const _);
}

pub unsafe fn bcma_sflash_init(cc: *mut bcma_drv_cc) -> i32 {
    let bus = (*(*cc).core).bus;
    let sflash = &mut (*cc).sflash;
    let mut e: *const bcma_sflash_tbl_e = core::ptr::null();
    let id: u32;
    let id2: u32;
    match (*cc).capabilities & BCMA_CC_CAP_FLASHT {
        BCMA_CC_FLASHT_STSER => {
            bcma_sflash_cmd(cc, BCMA_CC_FLASHCTL_ST_DP);
            bcma_cc_write32(cc, BCMA_CC_FLASHADDR, 0);
            bcma_sflash_cmd(cc, BCMA_CC_FLASHCTL_ST_RES);
            id = bcma_cc_read32(cc, BCMA_CC_FLASHDATA);
            bcma_cc_write32(cc, BCMA_CC_FLASHADDR, 1);
            bcma_sflash_cmd(cc, BCMA_CC_FLASHCTL_ST_RES);
            id2 = bcma_cc_read32(cc, BCMA_CC_FLASHDATA);
            if id == 0x13 { return -ENOTSUPP; }
            let table = if id == 0xbf { &bcma_sflash_sst_tbl } else { &bcma_sflash_st_tbl };
            for entry in table.iter() { if entry.id == if id == 0xbf { id2 } else { id } { e = entry; break; } }
        }
        BCMA_CC_FLASHT_ATSER => { bcma_sflash_cmd(cc, BCMA_CC_FLASHCTL_AT_STATUS); id = bcma_cc_read32(cc, BCMA_CC_FLASHDATA) & 0x3c; id2 = id; for entry in bcma_sflash_at_tbl.iter() { if entry.id == id { e = entry; break; } } }
        _ => { bcma_err(bus, b"Unsupported flash type\n\0".as_ptr() as *const _); return -ENOTSUPP; }
    }
    if e.is_null() { bcma_err(bus, b"Unsupported serial flash\n\0".as_ptr() as *const _); return -ENOTSUPP; }
    sflash.blocksize = (*e).blocksize;
    sflash.numblocks = (*e).numblocks;
    sflash.size = sflash.blocksize * sflash.numblocks as u32;
    sflash.present = true;
    bcma_sflash_resource.end = bcma_sflash_resource.start + sflash.size as u64;
    (*cc).sflash = *sflash;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
