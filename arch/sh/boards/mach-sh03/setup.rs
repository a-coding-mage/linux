// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/sh/boards/sh03/setup.c
 *
 * Copyright (C) 2004  Interface Co.,Ltd. Saito.K
 *
 */

// Dependencies supplied by the Linux kernel and the SH03 machine headers.

extern "C" {
    fn plat_irq_setup_pins(mode: i32);
    fn virt_to_phys(addr: *const core::ffi::c_void) -> usize;
    fn ioremap_prot(addr: usize, size: usize, prot: PgprotT) -> *mut core::ffi::c_void;
    fn printk(fmt: *const core::ffi::c_char, ...) -> i32;
    fn platform_add_devices(devices: *mut *mut PlatformDevice, count: usize) -> i32;
}

#[repr(C)]
pub struct Resource {
    pub start: usize,
    pub end: usize,
    pub flags: usize,
}

#[repr(C)]
pub struct PlatformDevice {
    pub name: *const core::ffi::c_char,
    pub id: i32,
    pub num_resources: usize,
    pub resource: *mut Resource,
}

#[repr(C)]
pub struct ShMachineVector {
    pub mv_name: *const core::ffi::c_char,
    pub mv_init_irq: Option<unsafe extern "C" fn()>,
}

pub type PgprotT = usize;

const IRQ_MODE_IRQ: i32 = 0;
const IORESOURCE_IO: usize = 0;
const IORESOURCE_IRQ: usize = 0;
const IORESOURCE_MEM: usize = 0;
const IRL2_IRQ: usize = 0;
const PA_AREA5_IO: usize = 0;
const PAGE_SIZE: usize = 4096;
const _PAGE_PCC_IO16: usize = 0;
const ENOMEM: i32 = 12;

unsafe fn init_sh03_IRQ() {
    plat_irq_setup_pins(IRQ_MODE_IRQ);
}

static mut cf_ide_resources: [Resource; 3] = [
    Resource {
        start: 0x1f0,
        end: 0x1f0 + 8,
        flags: IORESOURCE_IO,
    },
    Resource {
        start: 0x1f0 + 0x206,
        end: 0x1f0 + 8 + 0x206 + 8,
        flags: IORESOURCE_IO,
    },
    Resource {
        start: IRL2_IRQ,
        end: 0,
        flags: IORESOURCE_IRQ,
    },
];

static mut cf_ide_device: PlatformDevice = PlatformDevice {
    name: b"pata_platform\0".as_ptr() as *const core::ffi::c_char,
    id: -1,
    num_resources: 3,
    resource: core::ptr::addr_of_mut!(cf_ide_resources) as *mut Resource,
};

static mut heartbeat_resources: [Resource; 1] = [Resource {
    start: 0xa0800000,
    end: 0xa0800000,
    flags: IORESOURCE_MEM,
}];

static mut heartbeat_device: PlatformDevice = PlatformDevice {
    name: b"heartbeat\0".as_ptr() as *const core::ffi::c_char,
    id: -1,
    num_resources: 1,
    resource: core::ptr::addr_of_mut!(heartbeat_resources) as *mut Resource,
};

static mut sh03_devices: [*mut PlatformDevice; 2] = [
    core::ptr::addr_of_mut!(heartbeat_device),
    core::ptr::addr_of_mut!(cf_ide_device),
];

unsafe extern "C" fn sh03_devices_setup() -> i32 {
    let prot: PgprotT;
    let paddrbase: usize;
    let cf_ide_base: *mut core::ffi::c_void;

    /* open I/O area window */
    paddrbase = virt_to_phys(PA_AREA5_IO as *const core::ffi::c_void);
    prot = PAGE_KERNEL_PCC(1, _PAGE_PCC_IO16);
    cf_ide_base = ioremap_prot(paddrbase, PAGE_SIZE, prot);
    if cf_ide_base.is_null() {
        printk(b"allocate_cf_area : can't open CF I/O window!\n\0".as_ptr() as *const core::ffi::c_char);
        return -ENOMEM;
    }

    /* IDE cmd address : 0x1f0-0x1f7 and 0x3f6 */
    let base = cf_ide_base as usize;
    cf_ide_resources[0].start = cf_ide_resources[0].start.wrapping_add(base);
    cf_ide_resources[0].end = cf_ide_resources[0].end.wrapping_add(base);
    cf_ide_resources[1].start = cf_ide_resources[1].start.wrapping_add(base);
    cf_ide_resources[1].end = cf_ide_resources[1].end.wrapping_add(base);

    platform_add_devices(sh03_devices.as_mut_ptr(), 2)
}

static mut mv_sh03: ShMachineVector = ShMachineVector {
    mv_name: b"Interface (CTP/PCI-SH03)\0".as_ptr() as *const core::ffi::c_char,
    mv_init_irq: Some(init_sh03_IRQ),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
