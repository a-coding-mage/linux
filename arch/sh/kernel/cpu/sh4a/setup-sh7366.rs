// SPDX-License-Identifier: GPL-2.0
/* SH7366 Setup; direct Rust translation of setup-sh7366.c. */

use core::ptr;

extern "C" {
    fn evt2irq(event: u32) -> u32;
    fn platform_resource_setup_memory(dev: *mut platform_device, name: *const u8, size: usize);
    fn platform_add_devices(devices: *mut *mut platform_device, count: usize) -> i32;
    fn sh_early_platform_add_devices(devices: *mut *mut platform_device, count: usize);
    fn register_intc_controller(desc: *mut intc_desc);
}

#[repr(C)] pub struct plat_sci_port { pub scscr: u32, pub kind: u32 }
#[repr(C)] pub struct resource { pub name: *const u8, pub start: u32, pub end: u32, pub flags: u32 }
#[repr(C)] pub struct device { pub platform_data: *mut core::ffi::c_void, pub dma_mask: *mut u64, pub coherent_dma_mask: u64 }
#[repr(C)] pub struct platform_device { pub name: *const u8, pub id: i32, pub resource: *mut resource, pub num_resources: usize, pub dev: device }
#[repr(C)] pub struct uio_info { pub name: *const u8, pub version: *const u8, pub irq: u32 }
#[repr(C)] pub struct sh_timer_config { pub channels_mask: u32 }
#[repr(C)] pub struct r8a66597_platdata { pub on_chip: i32 }
#[repr(C)] pub struct intc_vect { pub irq: i32, pub vector: u32 }
#[repr(C)] pub struct intc_group { pub group: i32, pub members: [i32; 4] }
#[repr(C)] pub struct intc_mask_reg { pub set: u32, pub clear: u32, pub width: u32, pub enum_ids: [i32; 8] }
#[repr(C)] pub struct intc_prio_reg { pub addr: u32, pub reserved: u32, pub width: u32, pub field_width: u32, pub enum_ids: [i32; 8] }
#[repr(C)] pub struct intc_sense_reg { pub addr: u32, pub width: u32, pub field_width: u32, pub enum_ids: [i32; 8] }
#[repr(C)] pub struct intc_desc { pub name: *const u8, pub force_enable: i32, pub force_disable: i32, pub hw: *const core::ffi::c_void }

const SCSCR_REIE: u32 = 1;
const PORT_SCIF: u32 = 0;
const IORESOURCE_MEM: u32 = 0x0000_0200;
const IORESOURCE_IRQ: u32 = 0x0000_0400;
const IRQF_TRIGGER_LOW: u32 = 0x0000_0008;

static mut scif0_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, kind: PORT_SCIF };
static mut scif0_resources: [resource; 2] = [
    resource { name: ptr::null(), start: 0xffe00000, end: 0xffe000ff, flags: IORESOURCE_MEM },
    resource { name: ptr::null(), start: 0, end: 0, flags: IORESOURCE_IRQ },
];
static mut scif0_device: platform_device = platform_device { name: b"sh-sci\0".as_ptr(), id: 0, resource: unsafe { scif0_resources.as_mut_ptr() }, num_resources: 2, dev: device { platform_data: unsafe { &mut scif0_platform_data as *mut _ as *mut _ }, dma_mask: ptr::null_mut(), coherent_dma_mask: 0 } };

static mut iic_resources: [resource; 2] = [
    resource { name: b"IIC\0".as_ptr(), start: 0x04470000, end: 0x04470017, flags: IORESOURCE_MEM },
    resource { name: ptr::null(), start: 0, end: 0, flags: IORESOURCE_IRQ },
];
static mut iic_device: platform_device = platform_device { name: b"i2c-sh_mobile\0".as_ptr(), id: 0, resource: unsafe { iic_resources.as_mut_ptr() }, num_resources: 2, dev: device { platform_data: ptr::null_mut(), dma_mask: ptr::null_mut(), coherent_dma_mask: 0 } };
static mut r8a66597_data: r8a66597_platdata = r8a66597_platdata { on_chip: 1 };
static mut usb_host_resources: [resource; 2] = [resource { name: ptr::null(), start: 0xa4d80000, end: 0xa4d800ff, flags: IORESOURCE_MEM }, resource { name: ptr::null(), start: 0, end: 0, flags: IORESOURCE_IRQ | IRQF_TRIGGER_LOW }];
static mut usb_host_device: platform_device = platform_device { name: b"r8a66597_hcd\0".as_ptr(), id: -1, resource: unsafe { usb_host_resources.as_mut_ptr() }, num_resources: 2, dev: device { platform_data: unsafe { &mut r8a66597_data as *mut _ as *mut _ }, dma_mask: ptr::null_mut(), coherent_dma_mask: 0xffff_ffff } };

static mut vpu_platform_data: uio_info = uio_info { name: b"VPU5\0".as_ptr(), version: b"0\0".as_ptr(), irq: 0 };
static mut vpu_resources: [resource; 2] = [resource { name: b"VPU\0".as_ptr(), start: 0xfe900000, end: 0xfe902807, flags: IORESOURCE_MEM }, resource { name: ptr::null(), start: 0, end: 0, flags: 0 }];
static mut vpu_device: platform_device = platform_device { name: b"uio_pdrv_genirq\0".as_ptr(), id: 0, resource: unsafe { vpu_resources.as_mut_ptr() }, num_resources: 2, dev: device { platform_data: unsafe { &mut vpu_platform_data as *mut _ as *mut _ }, dma_mask: ptr::null_mut(), coherent_dma_mask: 0 } };
static mut veu0_platform_data: uio_info = uio_info { name: b"VEU\0".as_ptr(), version: b"0\0".as_ptr(), irq: 0 };
static mut veu1_platform_data: uio_info = uio_info { name: b"VEU\0".as_ptr(), version: b"0\0".as_ptr(), irq: 0 };
static mut veu0_resources: [resource; 2] = [resource { name: b"VEU(1)\0".as_ptr(), start: 0xfe920000, end: 0xfe9200b7, flags: IORESOURCE_MEM }, resource { name: ptr::null(), start: 0, end: 0, flags: 0 }];
static mut veu1_resources: [resource; 2] = [resource { name: b"VEU(2)\0".as_ptr(), start: 0xfe924000, end: 0xfe9240b7, flags: IORESOURCE_MEM }, resource { name: ptr::null(), start: 0, end: 0, flags: 0 }];
static mut veu0_device: platform_device = platform_device { name: b"uio_pdrv_genirq\0".as_ptr(), id: 1, resource: unsafe { veu0_resources.as_mut_ptr() }, num_resources: 2, dev: device { platform_data: unsafe { &mut veu0_platform_data as *mut _ as *mut _ }, dma_mask: ptr::null_mut(), coherent_dma_mask: 0 } };
static mut veu1_device: platform_device = platform_device { name: b"uio_pdrv_genirq\0".as_ptr(), id: 2, resource: unsafe { veu1_resources.as_mut_ptr() }, num_resources: 2, dev: device { platform_data: unsafe { &mut veu1_platform_data as *mut _ as *mut _ }, dma_mask: ptr::null_mut(), coherent_dma_mask: 0 } };

static mut cmt_platform_data: sh_timer_config = sh_timer_config { channels_mask: 0x20 };
static mut cmt_resources: [resource; 2] = [resource { name: ptr::null(), start: 0x044a0000, end: 0x044a006f, flags: IORESOURCE_MEM }, resource { name: ptr::null(), start: 0, end: 0, flags: IORESOURCE_IRQ }];
static mut cmt_device: platform_device = platform_device { name: b"sh-cmt-32\0".as_ptr(), id: 0, resource: unsafe { cmt_resources.as_mut_ptr() }, num_resources: 2, dev: device { platform_data: unsafe { &mut cmt_platform_data as *mut _ as *mut _ }, dma_mask: ptr::null_mut(), coherent_dma_mask: 0 } };
static mut tmu0_platform_data: sh_timer_config = sh_timer_config { channels_mask: 7 };
static mut tmu0_resources: [resource; 4] = [resource { name: ptr::null(), start: 0xffd80000, end: 0xffd8002b, flags: IORESOURCE_MEM }; 4];
static mut tmu0_device: platform_device = platform_device { name: b"sh-tmu\0".as_ptr(), id: 0, resource: unsafe { tmu0_resources.as_mut_ptr() }, num_resources: 4, dev: device { platform_data: unsafe { &mut tmu0_platform_data as *mut _ as *mut _ }, dma_mask: ptr::null_mut(), coherent_dma_mask: 0 } };

static mut sh7366_devices: [*mut platform_device; 8] = unsafe { [&mut scif0_device, &mut cmt_device, &mut tmu0_device, &mut iic_device, &mut usb_host_device, &mut vpu_device, &mut veu0_device, &mut veu1_device] };
unsafe fn sh7366_devices_setup() -> i32 { platform_resource_setup_memory(&mut vpu_device, b"vpu\0".as_ptr(), 2 << 20); platform_resource_setup_memory(&mut veu0_device, b"veu0\0".as_ptr(), 2 << 20); platform_resource_setup_memory(&mut veu1_device, b"veu1\0".as_ptr(), 2 << 20); platform_add_devices(sh7366_devices.as_mut_ptr(), 8) }
static mut sh7366_early_devices: [*mut platform_device; 3] = unsafe { [&mut scif0_device, &mut cmt_device, &mut tmu0_device] };
pub unsafe fn plat_early_device_setup() { sh_early_platform_add_devices(sh7366_early_devices.as_mut_ptr(), 3); }

#[repr(i32)] enum IntcId { UNUSED=0, ENABLED, DISABLED, IRQ0, IRQ1, IRQ2, IRQ3, IRQ4, IRQ5, IRQ6, IRQ7, ICB, DMAC0, DMAC1, DMAC2, DMAC3, VIO_CEUI, VIO_BEUI, VIO_VEUI, VOU, MFI, VPU, USB, MMC_MMC1I, MMC_MMC2I, MMC_MMC3I, DMAC4, DMAC5, DMAC_DADERR, SCIF, SCIFA1, SCIFA2, DENC, MSIOF, FLCTL_FLSTEI, FLCTL_FLENDI, FLCTL_FLTREQ0I, FLCTL_FLTREQ1I, I2C_ALI, I2C_TACKI, I2C_WAITI, I2C_DTEI, SDHI, CMT, TSIF, SIU, TMU0, TMU1, TMU2, VEU2, LCDC, DMAC0123, VIOVOU, MMC, DMAC45, FLCTL, I2C }

static mut vectors: [intc_vect; 53] = [
    intc_vect { irq: 3, vector: 0x600 }, intc_vect { irq: 4, vector: 0x620 }, intc_vect { irq: 5, vector: 0x640 }, intc_vect { irq: 6, vector: 0x660 }, intc_vect { irq: 7, vector: 0x680 }, intc_vect { irq: 8, vector: 0x6a0 }, intc_vect { irq: 9, vector: 0x6c0 }, intc_vect { irq: 10, vector: 0x6e0 }, intc_vect { irq: 11, vector: 0x700 },
    intc_vect { irq: 12, vector: 0x800 }, intc_vect { irq: 13, vector: 0x820 }, intc_vect { irq: 14, vector: 0x840 }, intc_vect { irq: 15, vector: 0x860 }, intc_vect { irq: 16, vector: 0x880 }, intc_vect { irq: 17, vector: 0x8a0 }, intc_vect { irq: 18, vector: 0x8c0 }, intc_vect { irq: 19, vector: 0x8e0 }, intc_vect { irq: 20, vector: 0x900 }, intc_vect { irq: 21, vector: 0x980 }, intc_vect { irq: 22, vector: 0xa20 },
    intc_vect { irq: 23, vector: 0xb00 }, intc_vect { irq: 24, vector: 0xb20 }, intc_vect { irq: 25, vector: 0xb40 }, intc_vect { irq: 26, vector: 0xb80 }, intc_vect { irq: 27, vector: 0xba0 }, intc_vect { irq: 28, vector: 0xbc0 }, intc_vect { irq: 29, vector: 0xc00 }, intc_vect { irq: 30, vector: 0xc20 }, intc_vect { irq: 31, vector: 0xc40 }, intc_vect { irq: 32, vector: 0xc60 }, intc_vect { irq: 33, vector: 0xc80 },
    intc_vect { irq: 34, vector: 0xd80 }, intc_vect { irq: 35, vector: 0xda0 }, intc_vect { irq: 36, vector: 0xdc0 }, intc_vect { irq: 37, vector: 0xde0 }, intc_vect { irq: 38, vector: 0xe00 }, intc_vect { irq: 39, vector: 0xe20 }, intc_vect { irq: 40, vector: 0xe40 }, intc_vect { irq: 41, vector: 0xe60 }, intc_vect { irq: 42, vector: 0xe80 }, intc_vect { irq: 42, vector: 0xea0 }, intc_vect { irq: 42, vector: 0xec0 }, intc_vect { irq: 42, vector: 0xee0 }, intc_vect { irq: 43, vector: 0xf00 }, intc_vect { irq: 44, vector: 0xf20 }, intc_vect { irq: 45, vector: 0xf80 }, intc_vect { irq: 46, vector: 0x400 }, intc_vect { irq: 47, vector: 0x420 }, intc_vect { irq: 48, vector: 0x440 }, intc_vect { irq: 49, vector: 0x560 }, intc_vect { irq: 50, vector: 0x580 },
];
static mut mask_registers: [intc_mask_reg; 12] = [
    intc_mask_reg { set: 0xa4080080, clear: 0xa40800c0, width: 8, enum_ids: [0; 8] }, intc_mask_reg { set: 0xa4080084, clear: 0xa40800c4, width: 8, enum_ids: [19,18,17,16,15,14,13,12] }, intc_mask_reg { set: 0xa4080088, clear: 0xa40800c8, width: 8, enum_ids: [0,0,0,21,0,0,0,20] }, intc_mask_reg { set: 0xa408008c, clear: 0xa40800cc, width: 8, enum_ids: [0,0,0,11,0,0,0,0] }, intc_mask_reg { set: 0xa4080090, clear: 0xa40800d0, width: 8, enum_ids: [0,48,47,46,49,0,0,50] }, intc_mask_reg { set: 0xa4080094, clear: 0xa40800d4, width: 8, enum_ids: [0,28,27,26,32,31,30,29] }, intc_mask_reg { set: 0xa4080098, clear: 0xa40800d8, width: 8, enum_ids: [0,0,0,0,0,0,0,33] }, intc_mask_reg { set: 0xa408009c, clear: 0xa40800dc, width: 8, enum_ids: [41,40,39,38,37,36,35,34] }, intc_mask_reg { set: 0xa40800a0, clear: 0xa40800e0, width: 8, enum_ids: [2,1,1,1,0,0,0,45] }, intc_mask_reg { set: 0xa40800a4, clear: 0xa40800e4, width: 8, enum_ids: [0,0,0,43,0,22,0,0] }, intc_mask_reg { set: 0xa40800a8, clear: 0xa40800e8, width: 8, enum_ids: [0,25,24,23,0,0,0,0] }, intc_mask_reg { set: 0xa4140044, clear: 0xa4140064, width: 8, enum_ids: [3,4,5,6,7,8,9,10] },
];
static mut intc_desc_sh7366: intc_desc = intc_desc { name: b"sh7366\0".as_ptr(), force_enable: 1, force_disable: 2, hw: ptr::null() };
pub unsafe fn plat_irq_setup() { register_intc_controller(&mut intc_desc_sh7366); }
pub unsafe fn plat_mem_setup() { /* TODO: Register Node 1 */ }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
