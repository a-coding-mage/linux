// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/sh/boards/se/7722/setup.c
 *
 * Copyright (C) 2007 Nobuhiro Iwamatsu
 * Copyright (C) 2012 Paul Mundt
 *
 * Hitachi UL SolutionEngine 7722 Support.
 */

// C header dependencies are supplied by the surrounding kernel translation.

extern "C" {
    static mut se7722_irq_domain: *mut irq_domain;
    fn mrshpc_setup_windows();
    fn irq_find_mapping(domain: *mut irq_domain, hwirq: u32) -> u32;
    fn platform_add_devices(devices: *mut *mut platform_device, num: usize) -> i32;
    fn init_se7722_IRQ();
    fn __raw_writew(value: u16, address: usize);
    fn __raw_readw(address: usize) -> u16;
}

#[repr(C)]
pub struct irq_domain { _private: [u8; 0] }

#[repr(C)]
pub struct resource {
    pub name: *const u8,
    pub start: usize,
    pub end: usize,
    pub flags: u64,
}

#[repr(C)]
pub struct device {
    pub dma_mask: *mut u64,
    pub coherent_dma_mask: u64,
    pub platform_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct platform_device {
    pub name: *const u8,
    pub id: i32,
    pub dev: device,
    pub num_resources: usize,
    pub resource: *mut resource,
}

#[repr(C)]
pub struct smc91x_platdata { pub flags: u32 }

#[repr(C)]
pub struct sh_keysc_info {
    pub mode: u32,
    pub scan_timing: u32,
    pub delay: u32,
    pub keycodes: [u32; 30],
}

const IORESOURCE_MEM: u64 = 0x0000_0200;
const IORESOURCE_MEM_16BIT: u64 = 0x0000_0010;
const IORESOURCE_IRQ: u64 = 0x0000_0400;
const IORESOURCE_IO: u64 = 0x0000_0100;
const SMC91X_USE_16BIT: u32 = 1 << 0;
const SH_KEYSC_MODE_1: u32 = 1;

extern "C" {
    static PA_LED: usize;
    static PA_LAN: usize;
    static PA_MRSHPC_IO: usize;
    static FPGA_OUT: usize;
    static PORT_PECR: usize;
    static PORT_PJCR: usize;
    static PORT_PSELD: usize;
    static PORT_PSELB: usize;
    static PORT_PSELC: usize;
    static PORT_PKCR: usize;
    static PORT_PHCR: usize;
    static PORT_PLCR: usize;
    static PORT_PMCR: usize;
    static PORT_PRCR: usize;
    static PORT_PXCR: usize;
    static PORT_PSELA: usize;
    static PORT_PYCR: usize;
    static PORT_PZCR: usize;
    static PORT_HIZCRA: usize;
    static PORT_HIZCRC: usize;
    static SE7722_FPGA_IRQ_MRSHPC0: u32;
    static SE7722_FPGA_IRQ_SMC: u32;
}

extern "C" { fn evt2irq(event: u32) -> u32; }

static mut heartbeat_resource: resource = resource {
    name: core::ptr::null(), start: 0, end: 0,
    flags: IORESOURCE_MEM | IORESOURCE_MEM_16BIT,
};

static mut heartbeat_device: platform_device = platform_device {
    name: b"heartbeat\0".as_ptr(), id: -1,
    dev: device { dma_mask: core::ptr::null_mut(), coherent_dma_mask: 0, platform_data: core::ptr::null_mut() },
    num_resources: 1, resource: core::ptr::null_mut(),
};

static mut smc91x_info: smc91x_platdata = smc91x_platdata { flags: SMC91X_USE_16BIT };
static mut smc91x_eth_resources: [resource; 2] = [
    resource { name: b"smc91x-regs\0".as_ptr(), start: 0, end: 0, flags: IORESOURCE_MEM },
    resource { name: core::ptr::null(), start: 0, end: 0, flags: IORESOURCE_IRQ },
];
static mut smc91x_eth_device: platform_device = platform_device {
    name: b"smc91x\0".as_ptr(), id: 0,
    dev: device { dma_mask: core::ptr::null_mut(), coherent_dma_mask: 0xffff_ffff, platform_data: core::ptr::null_mut() },
    num_resources: 2, resource: core::ptr::null_mut(),
};

static mut cf_ide_resources: [resource; 3] = [
    resource { name: core::ptr::null(), start: 0, end: 0, flags: IORESOURCE_IO },
    resource { name: core::ptr::null(), start: 0, end: 0, flags: IORESOURCE_IO },
    resource { name: core::ptr::null(), start: 0, end: 0, flags: IORESOURCE_IRQ },
];
static mut cf_ide_device: platform_device = platform_device {
    name: b"pata_platform\0".as_ptr(), id: -1,
    dev: device { dma_mask: core::ptr::null_mut(), coherent_dma_mask: 0, platform_data: core::ptr::null_mut() },
    num_resources: 3, resource: core::ptr::null_mut(),
};

static mut sh_keysc_info: sh_keysc_info = sh_keysc_info {
    mode: SH_KEYSC_MODE_1, scan_timing: 3, delay: 5,
    keycodes: [KEY_A, KEY_B, KEY_C, KEY_D, KEY_E, KEY_F, KEY_G, KEY_H, KEY_I, KEY_J,
        KEY_K, KEY_L, KEY_M, KEY_N, KEY_O, KEY_P, KEY_Q, KEY_R, KEY_S, KEY_T,
        KEY_U, KEY_V, KEY_W, KEY_X, KEY_Y, KEY_Z, KEY_HOME, KEY_SLEEP, KEY_WAKEUP, KEY_COFFEE],
};
static mut sh_keysc_resources: [resource; 2] = [
    resource { name: core::ptr::null(), start: 0x044b0000, end: 0x044b000f, flags: IORESOURCE_MEM },
    resource { name: core::ptr::null(), start: 0, end: 0, flags: IORESOURCE_IRQ },
];
static mut sh_keysc_device: platform_device = platform_device {
    name: b"sh_keysc\0".as_ptr(), id: 0,
    dev: device { dma_mask: core::ptr::null_mut(), coherent_dma_mask: 0, platform_data: core::ptr::null_mut() },
    num_resources: 2, resource: core::ptr::null_mut(),
};

static mut se7722_devices: [*mut platform_device; 4] = [
    &raw mut heartbeat_device, &raw mut smc91x_eth_device,
    &raw mut cf_ide_device, &raw mut sh_keysc_device,
];

unsafe extern "C" fn se7722_devices_setup() -> i32 {
    mrshpc_setup_windows();
    let irq = irq_find_mapping(se7722_irq_domain, SE7722_FPGA_IRQ_MRSHPC0);
    cf_ide_resources[2].start = irq as usize; cf_ide_resources[2].end = irq as usize;
    let irq = irq_find_mapping(se7722_irq_domain, SE7722_FPGA_IRQ_SMC);
    smc91x_eth_resources[1].start = irq as usize; smc91x_eth_resources[1].end = irq as usize;
    platform_add_devices(se7722_devices.as_mut_ptr(), se7722_devices.len())
}

// device_initcall(se7722_devices_setup);

unsafe extern "C" fn se7722_setup(_cmdline_p: *mut *mut u8) {
    __raw_writew(0x010D, FPGA_OUT); __raw_writew(0x0000, PORT_PECR); __raw_writew(0x1000, PORT_PJCR);
    __raw_writew(0x0020, PORT_PSELD); __raw_writew(0x0003, PORT_PSELB); __raw_writew(0xe000, PORT_PSELC); __raw_writew(0x0000, PORT_PKCR);
    __raw_writew(0x4020, PORT_PHCR); __raw_writew(0x0000, PORT_PLCR); __raw_writew(0x0000, PORT_PMCR); __raw_writew(0x0002, PORT_PRCR); __raw_writew(0x0000, PORT_PXCR);
    __raw_writew(0x0A10, PORT_PSELA); __raw_writew(0x0000, PORT_PYCR); __raw_writew(0x0000, PORT_PZCR);
    __raw_writew(__raw_readw(PORT_HIZCRA) & !0x4000, PORT_HIZCRA);
    __raw_writew(__raw_readw(PORT_HIZCRC) & !0xc000, PORT_HIZCRC);
}

// The Machine Vector
#[repr(C)]
struct sh_machine_vector { mv_name: *const u8, mv_setup: unsafe extern "C" fn(*mut *mut u8), mv_init_irq: unsafe extern "C" fn() }
static mut mv_se7722: sh_machine_vector = sh_machine_vector { mv_name: b"Solution Engine 7722\0".as_ptr(), mv_setup: se7722_setup, mv_init_irq: init_se7722_IRQ };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
