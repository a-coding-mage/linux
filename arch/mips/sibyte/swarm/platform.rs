// SPDX-License-Identifier: GPL-2.0
// C dependencies supplied by the surrounding kernel translation unit.

// #if defined(CONFIG_SIBYTE_SWARM) || defined(CONFIG_SIBYTE_LITTLESUR)

pub const DRV_NAME: &[u8] = b"pata-swarm\0";

pub const SWARM_IDE_SHIFT: usize = 5;
pub const SWARM_IDE_BASE: usize = 0x1f0;
pub const SWARM_IDE_CTRL: usize = 0x3f6;

#[repr(C)]
pub struct Resource {
    pub name: *const u8,
    pub flags: usize,
    pub start: usize,
    pub end: usize,
}

#[repr(C)]
pub struct PataPlatformInfo {
    pub ioport_shift: usize,
}

#[repr(C)]
pub struct Device {
    pub platform_data: *mut core::ffi::c_void,
    pub coherent_dma_mask: usize,
}

#[repr(C)]
pub struct PlatformDevice {
    pub name: *const u8,
    pub id: i32,
    pub resource: *mut Resource,
    pub num_resources: usize,
    pub dev: Device,
}

extern "C" {
    static SIBYTE_HAVE_IDE: bool;
    static A_IO_EXT_BASE: usize;
    static A_PHYS_GENBUS: usize;
    static A_PHYS_GENBUS_END: usize;
    static IDE_CS: usize;

    fn ioremap(addr: usize, size: usize) -> *mut u8;
    fn __raw_readq(addr: *mut u8) -> usize;
    fn iounmap(addr: *mut u8);
    fn platform_device_register(device: *mut PlatformDevice) -> i32;
    fn pr_info(format: *const u8, ...);
    fn G_IO_START_ADDR(value: usize) -> usize;
    fn G_IO_MULT_SIZE(value: usize) -> usize;
    fn R_IO_EXT_REG(reg: usize, cs: usize) -> usize;
    static R_IO_EXT_START_ADDR: usize;
    static R_IO_EXT_MULT_SIZE: usize;
    static S_IO_ADDRBASE: usize;
    static S_IO_REGSIZE: usize;
}

pub const IORESOURCE_MEM: usize = 0x0000_0200;
pub const IORESOURCE_IRQ: usize = 0x0000_0400;
pub const ENODEV: i32 = 19;
pub const EBUSY: i32 = 16;
pub const K_INT_GB_IDE: usize = 0;

pub static mut swarm_pata_resource: [Resource; 3] = [
    Resource { name: b"Swarm GenBus IDE\0".as_ptr(), flags: IORESOURCE_MEM, start: 0, end: 0 },
    Resource { name: b"Swarm GenBus IDE\0".as_ptr(), flags: IORESOURCE_MEM, start: 0, end: 0 },
    Resource { name: b"Swarm GenBus IDE\0".as_ptr(), flags: IORESOURCE_IRQ, start: 0, end: 0 },
];

pub static mut pata_platform_data: PataPlatformInfo = PataPlatformInfo {
    ioport_shift: SWARM_IDE_SHIFT,
};

pub static mut swarm_pata_device: PlatformDevice = PlatformDevice {
    name: b"pata_platform\0".as_ptr(),
    id: -1,
    resource: unsafe { swarm_pata_resource.as_mut_ptr() },
    num_resources: 3,
    dev: Device {
        platform_data: unsafe { &mut pata_platform_data as *mut _ as *mut core::ffi::c_void },
        coherent_dma_mask: !0,
    },
};

pub unsafe extern "C" fn swarm_pata_init() -> i32 {
    let mut base: *mut u8;
    let mut offset: usize;
    let mut size: usize;
    let r: *mut Resource;

    if !SIBYTE_HAVE_IDE {
        return -ENODEV;
    }

    base = ioremap(A_IO_EXT_BASE, 0x800);
    offset = __raw_readq(base.add(R_IO_EXT_REG(R_IO_EXT_START_ADDR, IDE_CS)));
    size = __raw_readq(base.add(R_IO_EXT_REG(R_IO_EXT_MULT_SIZE, IDE_CS)));
    iounmap(base);

    offset = G_IO_START_ADDR(offset) << S_IO_ADDRBASE;
    size = (G_IO_MULT_SIZE(size) + 1) << S_IO_REGSIZE;
    if offset < A_PHYS_GENBUS || offset >= A_PHYS_GENBUS_END {
        pr_info(b"%s: PATA interface at GenBus disabled\n\0".as_ptr(), DRV_NAME.as_ptr());
        return -EBUSY;
    }

    pr_info(b"%s: PATA interface at GenBus slot %i\n\0".as_ptr(), DRV_NAME.as_ptr(), IDE_CS);

    r = swarm_pata_resource.as_mut_ptr();
    (*r.add(0)).start = offset + (SWARM_IDE_BASE << SWARM_IDE_SHIFT);
    (*r.add(0)).end = offset + ((SWARM_IDE_BASE + 8) << SWARM_IDE_SHIFT) - 1;
    (*r.add(1)).start = offset + (SWARM_IDE_CTRL << SWARM_IDE_SHIFT);
    (*r.add(1)).end = offset + ((SWARM_IDE_CTRL + 1) << SWARM_IDE_SHIFT) - 1;

    platform_device_register(&mut swarm_pata_device)
}

// device_initcall(swarm_pata_init);

// #endif /* defined(CONFIG_SIBYTE_SWARM) || defined(CONFIG_SIBYTE_LITTLESUR) */

#[repr(C)]
pub struct Sb1250Resource {
    pub name: *const u8,
    pub flags: usize,
    pub start: usize,
    pub end: usize,
}

pub static mut sb1250_res0: Sb1250Resource = Sb1250Resource { name: b"SB1250 MAC 0\0".as_ptr(), flags: IORESOURCE_MEM, start: 0, end: 0 };
pub static mut sb1250_res1: Sb1250Resource = Sb1250Resource { name: b"SB1250 MAC 1\0".as_ptr(), flags: IORESOURCE_MEM, start: 0, end: 0 };
pub static mut sb1250_res2: Sb1250Resource = Sb1250Resource { name: b"SB1250 MAC 2\0".as_ptr(), flags: IORESOURCE_MEM, start: 0, end: 0 };
pub static mut sb1250_res3: Sb1250Resource = Sb1250Resource { name: b"SB1250 MAC 3\0".as_ptr(), flags: IORESOURCE_MEM, start: 0, end: 0 };

pub static mut sb1250_dev0: PlatformDevice = PlatformDevice { name: b"sb1250-mac\0".as_ptr(), id: 0, resource: unsafe { &mut sb1250_res0 as *mut _ as *mut Resource }, num_resources: 1, dev: Device { platform_data: core::ptr::null_mut(), coherent_dma_mask: 0 } };
pub static mut sb1250_dev1: PlatformDevice = PlatformDevice { name: b"sb1250-mac\0".as_ptr(), id: 1, resource: unsafe { &mut sb1250_res1 as *mut _ as *mut Resource }, num_resources: 1, dev: Device { platform_data: core::ptr::null_mut(), coherent_dma_mask: 0 } };
pub static mut sb1250_dev2: PlatformDevice = PlatformDevice { name: b"sb1250-mac\0".as_ptr(), id: 2, resource: unsafe { &mut sb1250_res2 as *mut _ as *mut Resource }, num_resources: 1, dev: Device { platform_data: core::ptr::null_mut(), coherent_dma_mask: 0 } };
pub static mut sb1250_dev3: PlatformDevice = PlatformDevice { name: b"sb1250-mac\0".as_ptr(), id: 3, resource: unsafe { &mut sb1250_res3 as *mut _ as *mut Resource }, num_resources: 1, dev: Device { platform_data: core::ptr::null_mut(), coherent_dma_mask: 0 } };

pub static mut sb1250_devs: [*mut PlatformDevice; 4] = [
    unsafe { &mut sb1250_dev0 },
    unsafe { &mut sb1250_dev1 },
    unsafe { &mut sb1250_dev2 },
    unsafe { &mut sb1250_dev3 },
];

extern "C" {
    static soc_type: usize;
    static K_SYS_SOC_TYPE_BCM1250: usize;
    static K_SYS_SOC_TYPE_BCM1250_ALT: usize;
    static K_SYS_SOC_TYPE_BCM1120: usize;
    static K_SYS_SOC_TYPE_BCM1125: usize;
    static K_SYS_SOC_TYPE_BCM1125H: usize;
    static K_SYS_SOC_TYPE_BCM1250_ALT2: usize;
    static K_SYS_SOC_TYPE_BCM1x55: usize;
    static K_SYS_SOC_TYPE_BCM1x80: usize;
    fn platform_add_devices(devices: *mut *mut PlatformDevice, count: usize) -> i32;
}

pub unsafe extern "C" fn sb1250_device_init() -> i32 {
    let ret: i32;
    match soc_type {
        x if x == K_SYS_SOC_TYPE_BCM1250 || x == K_SYS_SOC_TYPE_BCM1250_ALT => {
            ret = platform_add_devices(sb1250_devs.as_mut_ptr(), 3);
        }
        x if x == K_SYS_SOC_TYPE_BCM1120 || x == K_SYS_SOC_TYPE_BCM1125 || x == K_SYS_SOC_TYPE_BCM1125H || x == K_SYS_SOC_TYPE_BCM1250_ALT2 => {
            ret = platform_add_devices(sb1250_devs.as_mut_ptr(), 2);
        }
        x if x == K_SYS_SOC_TYPE_BCM1x55 || x == K_SYS_SOC_TYPE_BCM1x80 => {
            ret = platform_add_devices(sb1250_devs.as_mut_ptr(), 4);
        }
        _ => {
            ret = -ENODEV;
        }
    }
    ret
}

// device_initcall(sb1250_device_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
