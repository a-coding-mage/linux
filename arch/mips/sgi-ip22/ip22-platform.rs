// SPDX-License-Identifier: GPL-2.0
// C dependencies are supplied by the surrounding kernel translation.

use core::ffi::c_void;

extern "C" {
    static mut hpc3c0: *mut hpc3_regs;
    static mut hpc3c1: *mut hpc3_regs;
    static mut sgimc: *mut sgimc_regs;

    fn platform_device_register(dev: *mut platform_device) -> i32;
    fn platform_device_register_simple(
        name: *const u8,
        id: i32,
        resource: *mut resource,
        num: usize,
    ) -> *mut platform_device;
    fn ip22_is_fullhouse() -> bool;
    fn ip22_nvram_read(offset: i32) -> u16;
    fn ip22_eeprom_read(eeprom: *mut c_void, offset: i32) -> u16;
    fn get_dbe(value: u32, address: *mut u32) -> i32;
}

#[repr(C)]
struct resource {
    name: *const u8,
    start: usize,
    end: usize,
    flags: u64,
}

#[repr(C)]
struct device {
    platform_data: *mut c_void,
    dma_mask: *mut u64,
    coherent_dma_mask: u64,
}

#[repr(C)]
struct platform_device {
    name: *const u8,
    id: i32,
    num_resources: usize,
    resource: *mut resource,
    dev: device,
}

#[repr(C)]
struct sgiwd93_platform_data {
    unit: i32,
    irq: usize,
    hregs: *mut c_void,
    wdregs: *mut u8,
}

#[repr(C)]
struct sgiseeq_platform_data {
    hpc: *mut hpc3_regs,
    irq: usize,
    mac: [u8; 6],
}

#[repr(C)]
struct hpc3_regs {
    scsi_chan0: c_void,
    scsi_chan1: c_void,
    scsi0_ext: [u8; 1],
    scsi1_ext: [u8; 1],
    pbdma: [[u32; 2]; 2],
    pbus_piocfg: [[u32; 1]; 1],
    pbus_extregs: [[u32; 1]; 1],
    eeprom: c_void,
    rtcregs: [u8; 1],
}

#[repr(C)]
struct sgimc_regs {
    giopar: u32,
}

const SGI_WD93_0_IRQ: usize = 0;
const SGI_WD93_1_IRQ: usize = 0;
const SGI_ENET_IRQ: usize = 0;
const SGI_GIO_0_IRQ: usize = 0;
const SGI_SERIAL_IRQ: usize = 0;
const IORESOURCE_IRQ: u64 = 0;
const IORESOURCE_MEM: u64 = 0;
const DMA_BIT_MASK_32: u64 = 0xffff_ffff;
const HPC3_CHIP0_BASE: usize = 0;
const SGIMC_GIOPAR_MASTEREXP1: u32 = 0;
const SGIMC_GIOPAR_EXP164: u32 = 0;
const SGIMC_GIOPAR_HPC264: u32 = 0;

static mut sgiwd93_0_resources: [resource; 1] = [resource {
    name: b"eth0 irq\0".as_ptr(), start: SGI_WD93_0_IRQ, end: SGI_WD93_0_IRQ, flags: IORESOURCE_IRQ,
}];
static mut sgiwd93_0_pd: sgiwd93_platform_data = sgiwd93_platform_data { unit: 0, irq: SGI_WD93_0_IRQ, hregs: core::ptr::null_mut(), wdregs: core::ptr::null_mut() };
static mut sgiwd93_0_dma_mask: u64 = DMA_BIT_MASK_32;
static mut sgiwd93_0_device: platform_device = platform_device {
    name: b"sgiwd93\0".as_ptr(), id: 0, num_resources: 1, resource: core::ptr::addr_of_mut!(sgiwd93_0_resources) as *mut resource,
    dev: device { platform_data: core::ptr::addr_of_mut!(sgiwd93_0_pd) as *mut c_void, dma_mask: core::ptr::addr_of_mut!(sgiwd93_0_dma_mask), coherent_dma_mask: DMA_BIT_MASK_32 },
};

static mut sgiwd93_1_resources: [resource; 1] = [resource { name: b"eth0 irq\0".as_ptr(), start: SGI_WD93_1_IRQ, end: SGI_WD93_1_IRQ, flags: IORESOURCE_IRQ }];
static mut sgiwd93_1_pd: sgiwd93_platform_data = sgiwd93_platform_data { unit: 1, irq: SGI_WD93_1_IRQ, hregs: core::ptr::null_mut(), wdregs: core::ptr::null_mut() };
static mut sgiwd93_1_dma_mask: u64 = DMA_BIT_MASK_32;
static mut sgiwd93_1_device: platform_device = platform_device { name: b"sgiwd93\0".as_ptr(), id: 1, num_resources: 1, resource: core::ptr::addr_of_mut!(sgiwd93_1_resources) as *mut resource, dev: device { platform_data: core::ptr::addr_of_mut!(sgiwd93_1_pd) as *mut c_void, dma_mask: core::ptr::addr_of_mut!(sgiwd93_1_dma_mask), coherent_dma_mask: DMA_BIT_MASK_32 } };

// Create a platform device for the GPI port that receives image data from the embedded camera.
unsafe fn sgiwd93_devinit() -> i32 {
    sgiwd93_0_pd.hregs = core::ptr::addr_of_mut!((*hpc3c0).scsi_chan0) as *mut c_void;
    sgiwd93_0_pd.wdregs = (*hpc3c0).scsi0_ext.as_mut_ptr();
    let res = platform_device_register(core::ptr::addr_of_mut!(sgiwd93_0_device));
    if res != 0 { return res; }
    if !ip22_is_fullhouse() { return 0; }
    sgiwd93_1_pd.hregs = core::ptr::addr_of_mut!((*hpc3c0).scsi_chan1) as *mut c_void;
    sgiwd93_1_pd.wdregs = (*hpc3c0).scsi1_ext.as_mut_ptr();
    platform_device_register(core::ptr::addr_of_mut!(sgiwd93_1_device))
}

static mut sgiseeq_0_resources: [resource; 1] = [resource { name: b"eth0 irq\0".as_ptr(), start: SGI_ENET_IRQ, end: SGI_ENET_IRQ, flags: IORESOURCE_IRQ }];
static mut eth0_pd: sgiseeq_platform_data = sgiseeq_platform_data { hpc: core::ptr::null_mut(), irq: 0, mac: [0; 6] };
static mut sgiseeq_dma_mask: u64 = DMA_BIT_MASK_32;
static mut eth0_device: platform_device = platform_device { name: b"sgiseeq\0".as_ptr(), id: 0, num_resources: 1, resource: core::ptr::addr_of_mut!(sgiseeq_0_resources) as *mut resource, dev: device { platform_data: core::ptr::addr_of_mut!(eth0_pd) as *mut c_void, dma_mask: core::ptr::addr_of_mut!(sgiseeq_dma_mask), coherent_dma_mask: DMA_BIT_MASK_32 } };
static mut sgiseeq_1_resources: [resource; 1] = [resource { name: b"eth1 irq\0".as_ptr(), start: SGI_GIO_0_IRQ, end: SGI_GIO_0_IRQ, flags: IORESOURCE_IRQ }];
static mut eth1_pd: sgiseeq_platform_data = sgiseeq_platform_data { hpc: core::ptr::null_mut(), irq: 0, mac: [0; 6] };
static mut eth1_device: platform_device = platform_device { name: b"sgiseeq\0".as_ptr(), id: 1, num_resources: 1, resource: core::ptr::addr_of_mut!(sgiseeq_1_resources) as *mut resource, dev: device { platform_data: core::ptr::addr_of_mut!(eth1_pd) as *mut c_void, dma_mask: core::ptr::null_mut(), coherent_dma_mask: 0 } };

unsafe fn sgiseeq_devinit() -> i32 {
    let mut pbdma: u32 = 0;
    eth0_pd.hpc = hpc3c0; eth0_pd.irq = SGI_ENET_IRQ;
    for i in 0..3 { let tmp = ip22_nvram_read(250 / 2 + i); eth0_pd.mac[2 * i as usize] = (tmp >> 8) as u8; eth0_pd.mac[2 * i as usize + 1] = (tmp & 0xff) as u8; }
    let res = platform_device_register(core::ptr::addr_of_mut!(eth0_device)); if res != 0 { return res; }
    if ip22_is_fullhouse() || get_dbe(pbdma, core::ptr::addr_of_mut!((*hpc3c1).pbdma[1][0])) != 0 { return 0; }
    (*sgimc).giopar |= SGIMC_GIOPAR_MASTEREXP1 | SGIMC_GIOPAR_EXP164 | SGIMC_GIOPAR_HPC264;
    (*hpc3c1).pbus_piocfg[0][0] = 0x3ffff; (*hpc3c1).pbus_extregs[0][0] = 0x30;
    eth1_pd.hpc = hpc3c1; eth1_pd.irq = SGI_GIO_0_IRQ;
    for i in 0..3 { let tmp = ip22_eeprom_read(core::ptr::addr_of_mut!((*hpc3c1).eeprom), 250 / 2 + i); eth1_pd.mac[2 * i as usize] = (tmp >> 8) as u8; eth1_pd.mac[2 * i as usize + 1] = (tmp & 0xff) as u8; }
    platform_device_register(core::ptr::addr_of_mut!(eth1_device))
}

unsafe fn sgi_hal2_devinit() -> i32 { if platform_device_register_simple(b"sgihal2\0".as_ptr(), 0, core::ptr::null_mut(), 0).is_null() { -1 } else { 0 } }
unsafe fn sgi_button_devinit() -> i32 { if ip22_is_fullhouse() { return 0; } if platform_device_register_simple(b"sgibtns\0".as_ptr(), -1, core::ptr::null_mut(), 0).is_null() { -1 } else { 0 } }

unsafe fn sgi_ds1286_devinit() -> i32 {
    let mut res = resource { name: core::ptr::null(), start: 0, end: 0, flags: 0 };
    res.start = HPC3_CHIP0_BASE; res.end = res.start; res.flags = IORESOURCE_MEM;
    if platform_device_register_simple(b"rtc-ds1286\0".as_ptr(), -1, core::ptr::addr_of_mut!(res), 1).is_null() { -1 } else { 0 }
}

static mut sgi_zilog_resources: [resource; 2] = [resource { name: core::ptr::null(), start: HPC3_CHIP0_BASE, end: HPC3_CHIP0_BASE + 15, flags: IORESOURCE_MEM }, resource { name: core::ptr::null(), start: SGI_SERIAL_IRQ, end: SGI_SERIAL_IRQ, flags: IORESOURCE_IRQ }];
static mut zilog_device: platform_device = platform_device { name: b"ip22zilog\0".as_ptr(), id: 0, num_resources: 2, resource: core::ptr::addr_of_mut!(sgi_zilog_resources) as *mut resource, dev: device { platform_data: core::ptr::null_mut(), dma_mask: core::ptr::null_mut(), coherent_dma_mask: 0 } };
unsafe fn sgi_zilog_devinit() -> i32 { platform_device_register(core::ptr::addr_of_mut!(zilog_device)) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
