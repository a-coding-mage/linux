// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/boards/landisk/setup.c
 *
 * I-O DATA Device, Inc. LANDISK Support.
 *
 * Copyright (C) 2000 Kazumoto Kojima
 * Copyright (C) 2002 Paul Mundt
 * Copylight (C) 2002 Atom Create Engineering Co., Ltd.
 * Copyright (C) 2005-2007 kogiidena
 */

// Linux kernel and architecture dependencies supplied by other translation units.
extern "C" {
    fn __raw_writeb(value: u8, address: usize);
    fn __raw_readb(address: usize) -> u8;
    fn virt_to_phys(address: *mut core::ffi::c_void) -> usize;
    fn PAGE_KERNEL_PCC(arg: i32, flags: i32) -> pgprot_t;
    fn ioremap_prot(address: usize, size: usize, prot: pgprot_t) -> *mut core::ffi::c_void;
    fn printk(format: *const core::ffi::c_char, ...);
    fn platform_add_devices(devices: *mut *mut platform_device, count: usize) -> i32;
    fn __set_io_port_base(base: usize);
    fn init_landisk_IRQ();
}

type pgprot_t = usize;

#[repr(C)]
pub struct resource {
    pub start: usize,
    pub end: usize,
    pub flags: usize,
}

#[repr(C)]
pub struct pata_platform_info {
    pub ioport_shift: i32,
}

#[repr(C)]
pub struct device {
    pub platform_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct platform_device {
    pub name: *const core::ffi::c_char,
    pub id: i32,
    pub num_resources: usize,
    pub resource: *mut resource,
    pub dev: device,
}

#[repr(C)]
pub struct sh_machine_vector {
    pub mv_name: *const core::ffi::c_char,
    pub mv_setup: Option<unsafe extern "C" fn(*mut *mut core::ffi::c_char)>,
    pub mv_init_irq: Option<unsafe extern "C" fn()>,
}

extern "C" {
    static mut pm_power_off: Option<unsafe extern "C" fn()>;
}

// These constants are supplied by the architecture headers.
const PA_SHUTDOWN: usize = 0;
const PA_LED: usize = 0;
const PA_AREA5_IO: usize = 0;
const _PAGE_PCC_IO16: i32 = 0;
const PAGE_SIZE: usize = 4096;
const IORESOURCE_IO: usize = 0;
const IORESOURCE_IRQ: usize = 0;
const IRQ_FATA: usize = 0;
const ENOMEM: i32 = 12;

unsafe extern "C" fn landisk_power_off() {
    __raw_writeb(0x01, PA_SHUTDOWN);
}

static mut cf_ide_resources: [resource; 3] = [
    resource { start: 0, end: 0, flags: 0 },
    resource { start: 0, end: 0, flags: 0 },
    resource { start: 0, end: 0, flags: 0 },
];

static mut pata_info: pata_platform_info = pata_platform_info {
    ioport_shift: 1,
};

static mut cf_ide_device: platform_device = platform_device {
    name: b"pata_platform\0".as_ptr() as *const core::ffi::c_char,
    id: -1,
    num_resources: 3,
    resource: core::ptr::addr_of_mut!(cf_ide_resources) as *mut resource,
    dev: device {
        platform_data: core::ptr::addr_of_mut!(pata_info) as *mut core::ffi::c_void,
    },
};

static mut rtc_device: platform_device = platform_device {
    name: b"rs5c313\0".as_ptr() as *const core::ffi::c_char,
    id: -1,
    num_resources: 0,
    resource: core::ptr::null_mut(),
    dev: device { platform_data: core::ptr::null_mut() },
};

static mut landisk_devices: [*mut platform_device; 2] = [
    core::ptr::addr_of_mut!(cf_ide_device),
    core::ptr::addr_of_mut!(rtc_device),
];

unsafe extern "C" fn landisk_devices_setup() -> i32 {
    let prot: pgprot_t;
    let paddrbase: usize;
    let cf_ide_base: *mut core::ffi::c_void;

    /* open I/O area window */
    paddrbase = virt_to_phys(PA_AREA5_IO as *mut core::ffi::c_void);
    prot = PAGE_KERNEL_PCC(1, _PAGE_PCC_IO16);
    cf_ide_base = ioremap_prot(paddrbase, PAGE_SIZE, prot);
    if cf_ide_base.is_null() {
        printk(b"allocate_cf_area : can't open CF I/O window!\n\0".as_ptr() as *const core::ffi::c_char);
        return -ENOMEM;
    }

    /* IDE cmd address : 0x1f0-0x1f7 and 0x3f6 */
    (*core::ptr::addr_of_mut!(cf_ide_resources).add(0)).start = cf_ide_base as usize + 0x40;
    (*core::ptr::addr_of_mut!(cf_ide_resources).add(0)).end = cf_ide_base as usize + 0x40 + 0x0f;
    (*core::ptr::addr_of_mut!(cf_ide_resources).add(0)).flags = IORESOURCE_IO;
    (*core::ptr::addr_of_mut!(cf_ide_resources).add(1)).start = cf_ide_base as usize + 0x2c;
    (*core::ptr::addr_of_mut!(cf_ide_resources).add(1)).end = cf_ide_base as usize + 0x2c + 0x03;
    (*core::ptr::addr_of_mut!(cf_ide_resources).add(1)).flags = IORESOURCE_IO;
    (*core::ptr::addr_of_mut!(cf_ide_resources).add(2)).start = IRQ_FATA;
    (*core::ptr::addr_of_mut!(cf_ide_resources).add(2)).flags = IORESOURCE_IRQ;

    platform_add_devices(core::ptr::addr_of_mut!(landisk_devices).cast(), 2)
}

// device_initcall(landisk_devices_setup);

unsafe extern "C" fn landisk_setup(_cmdline_p: *mut *mut core::ffi::c_char) {
    /* I/O port identity mapping */
    __set_io_port_base(0);

    /* LED ON */
    __raw_writeb(__raw_readb(PA_LED) | 0x03, PA_LED);

    printk(b"I-O DATA DEVICE, INC. \"LANDISK Series\" support.\n\0".as_ptr() as *const core::ffi::c_char);
    pm_power_off = Some(landisk_power_off);
}

/*
 * The Machine Vector
 */
static mut mv_landisk: sh_machine_vector = sh_machine_vector {
    mv_name: b"LANDISK\0".as_ptr() as *const core::ffi::c_char,
    mv_setup: Some(landisk_setup),
    mv_init_irq: Some(init_landisk_IRQ),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
