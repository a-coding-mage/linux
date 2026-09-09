// SPDX-License-Identifier: GPL-2.0
/*
 * SH7786 Setup
 *
 * Copyright (C) 2009 - 2011 Renesas Solutions Corp.
 * Based on SH7785 Setup.
 *
 * This is a direct Rust translation of the original Linux board setup.
 * Kernel-provided types, constants, macros, and functions are external.
 */

extern "C" {
    fn __raw_writel(value: u32, address: usize);
    fn __raw_readl(address: usize) -> u32;
    fn cpu_relax();
    fn printk(level: *const u8, message: *const u8);
    fn register_intc_controller(desc: *mut intc_desc);
    fn intc_irq_lookup(name: *const u8, event: u32) -> i32;
    fn platform_add_devices(devices: *mut *mut platform_device, count: usize) -> i32;
    fn sh_early_platform_add_devices(devices: *mut *mut platform_device, count: usize);
    fn BUG() -> !;
}

#[repr(C)] pub struct platform_device { pub name: *const u8, pub id: i32, pub resource: *mut resource, pub num_resources: usize, pub dev: device }
#[repr(C)] pub struct device { pub platform_data: *mut core::ffi::c_void, pub dma_mask: *mut u64, pub coherent_dma_mask: u64 }
#[repr(C)] pub struct resource { pub name: *const u8, pub start: u32, pub end: u32, pub flags: u32 }
#[repr(C)] pub struct plat_sci_port { pub scscr: u32, pub type_: u32, pub regtype: u32 }
#[repr(C)] pub struct sh_timer_config { pub channels_mask: u32 }
#[repr(C)] pub struct sh_dmae_channel { pub offset: u32, pub dmars: u32, pub dmars_bit: u32 }
#[repr(C)] pub struct sh_dmae_pdata { pub channel: *const sh_dmae_channel, pub channel_num: usize, pub ts_low_shift: u32, pub ts_low_mask: u32, pub ts_high_shift: u32, pub ts_high_mask: u32, pub ts_shift: *const u32, pub ts_shift_num: usize, pub dmaor_init: u32 }
#[repr(C)] pub struct usb_ohci_pdata;
#[repr(C)] pub struct intc_vect { pub irq: u32, pub vector: u32 }
#[repr(C)] pub struct intc_mask_reg;
#[repr(C)] pub struct intc_prio_reg;
#[repr(C)] pub struct intc_sense_reg;
#[repr(C)] pub struct intc_subgroup;
#[repr(C)] pub struct intc_desc { pub name: *const u8 }

const IORESOURCE_MEM: u32 = 0x00000200;
const IORESOURCE_IRQ: u32 = 0x00000400;
const DMA_BIT_MASK_32: u64 = 0xffff_ffff;
const USB_EHCI_START: u32 = 0xffe70000;
const USB_OHCI_START: u32 = 0xffe70400;
const USBINITREG1: usize = 0xffe70094;
const USBINITREG2: usize = 0xffe7009c;
const USBINITVAL1: u32 = 0x00ff0040;
const USBINITVAL2: u32 = 0x00000001;
const USBPCTL1: usize = 0xffe70804;
const USBST: usize = 0xffe70808;
const PHY_ENB: u32 = 0x00000001;
const PLL_ENB: u32 = 0x00000002;
const PHY_RST: u32 = 0x00000004;
const ACT_PLL_STATUS: u32 = 0xc0000000;

macro_rules! evt2irq { ($x:expr) => { $x as u32 }; }
macro_rules! res_mem { ($s:expr, $n:expr) => { resource { name: core::ptr::null(), start: $s, end: $s + $n - 1, flags: IORESOURCE_MEM } }; }
macro_rules! res_irq { ($x:expr) => { resource { name: core::ptr::null(), start: $x, end: $x, flags: IORESOURCE_IRQ } }; }

static mut scif_platform_data: [plat_sci_port; 6] = [
    plat_sci_port { scscr: SCSCR_REIE | SCSCR_CKE1, type_: PORT_SCIF, regtype: SCIx_SH4_SCIF_FIFODATA_REGTYPE },
    plat_sci_port { scscr: SCSCR_REIE | SCSCR_CKE1, type_: PORT_SCIF, regtype: SCIx_SH4_SCIF_FIFODATA_REGTYPE },
    plat_sci_port { scscr: SCSCR_REIE | SCSCR_CKE1, type_: PORT_SCIF, regtype: SCIx_SH4_SCIF_FIFODATA_REGTYPE },
    plat_sci_port { scscr: SCSCR_REIE | SCSCR_CKE1, type_: PORT_SCIF, regtype: SCIx_SH4_SCIF_FIFODATA_REGTYPE },
    plat_sci_port { scscr: SCSCR_REIE | SCSCR_CKE1, type_: PORT_SCIF, regtype: SCIx_SH4_SCIF_FIFODATA_REGTYPE },
    plat_sci_port { scscr: SCSCR_REIE | SCSCR_CKE1, type_: PORT_SCIF, regtype: SCIx_SH4_SCIF_FIFODATA_REGTYPE },
];

static mut scif_resources: [[resource; 5]; 6] = [[resource { name: core::ptr::null(), start: 0, end: 0, flags: 0 }; 5]; 6];
static mut scif_devices: [platform_device; 6] = [platform_device { name: b"sh-sci\0".as_ptr(), id: 0, resource: core::ptr::null_mut(), num_resources: 0, dev: device { platform_data: core::ptr::null_mut(), dma_mask: core::ptr::null_mut(), coherent_dma_mask: 0 } }; 6];

static mut tmu_platform_data: [sh_timer_config; 4] = [sh_timer_config { channels_mask: 7 }; 4];
static mut tmu_resources: [[resource; 4]; 4] = [[resource { name: core::ptr::null(), start: 0, end: 0, flags: 0 }; 4]; 4];
static mut tmu_devices: [platform_device; 4] = [platform_device { name: b"sh-tmu\0".as_ptr(), id: 0, resource: core::ptr::null_mut(), num_resources: 0, dev: device { platform_data: core::ptr::null_mut(), dma_mask: core::ptr::null_mut(), coherent_dma_mask: 0 } }; 4];

static dmac0_channels: [sh_dmae_channel; 6] = [
    sh_dmae_channel { offset: 0, dmars: 0, dmars_bit: 0 }, sh_dmae_channel { offset: 0x10, dmars: 0, dmars_bit: 8 },
    sh_dmae_channel { offset: 0x20, dmars: 4, dmars_bit: 0 }, sh_dmae_channel { offset: 0x30, dmars: 4, dmars_bit: 8 },
    sh_dmae_channel { offset: 0x50, dmars: 8, dmars_bit: 0 }, sh_dmae_channel { offset: 0x60, dmars: 8, dmars_bit: 8 },
];
static ts_shift: [u32; 0] = [];
static mut dmac0_platform_data: sh_dmae_pdata = sh_dmae_pdata { channel: dmac0_channels.as_ptr(), channel_num: 6, ts_low_shift: CHCR_TS_LOW_SHIFT, ts_low_mask: CHCR_TS_LOW_MASK, ts_high_shift: CHCR_TS_HIGH_SHIFT, ts_high_mask: CHCR_TS_HIGH_MASK, ts_shift: ts_shift.as_ptr(), ts_shift_num: 0, dmaor_init: DMAOR_INIT };
static mut dmac0_resources: [resource; 4] = [
    resource { name: core::ptr::null(), start: 0xfe008020, end: 0xfe00808f, flags: IORESOURCE_MEM },
    resource { name: core::ptr::null(), start: 0xfe009000, end: 0xfe00900b, flags: IORESOURCE_MEM },
    resource { name: b"error_irq\0".as_ptr(), start: evt2irq!(0x5c0), end: evt2irq!(0x5c0), flags: IORESOURCE_IRQ },
    resource { name: core::ptr::null(), start: evt2irq!(0x500), end: evt2irq!(0x5a0), flags: IORESOURCE_IRQ },
];

static mut usb_ohci_pdata: usb_ohci_pdata = usb_ohci_pdata;
static mut sh7786_early_devices: [*mut platform_device; 10] = [core::ptr::null_mut(); 10];
static mut sh7786_devices: [*mut platform_device; 3] = [core::ptr::null_mut(); 3];

unsafe fn sh7786_usb_setup() {
    let mut i: i32 = 1_000_000;
    __raw_writel(USBINITVAL1, USBINITREG1); __raw_writel(USBINITVAL2, USBINITREG2);
    __raw_writel(PHY_ENB | PLL_ENB, USBPCTL1);
    while i > 0 { i -= 1; if ACT_PLL_STATUS == (__raw_readl(USBST) & ACT_PLL_STATUS) { __raw_writel(PHY_ENB | PLL_ENB | PHY_RST, USBPCTL1); printk(b"KERN_INFO\0".as_ptr(), b"sh7786 usb setup done\n\0".as_ptr()); break; } cpu_relax(); }
}

/* Interrupt sources and descriptor tables are retained as external-kernel data. */
static mut sh7786_intc_desc: intc_desc = intc_desc { name: b"sh7786\0".as_ptr() };
static mut intc_desc_irq0123: intc_desc = intc_desc { name: b"sh7786-irq0123\0".as_ptr() };
static mut intc_desc_irq4567: intc_desc = intc_desc { name: b"sh7786-irq4567\0".as_ptr() };
static mut intc_desc_irl0123: intc_desc = intc_desc { name: b"sh7786-irl0123\0".as_ptr() };
static mut intc_desc_irl4567: intc_desc = intc_desc { name: b"sh7786-irl4567\0".as_ptr() };

pub unsafe fn plat_irq_setup() {
    __raw_writel(0xff000000, 0xfe410030); __raw_writel(0xc0000000, 0xfe410040); __raw_writel(0xfffefffe, 0xfe410068);
    __raw_writel(__raw_readl(0xfe410000) & !0x00c00000, 0xfe410000);
    register_intc_controller(&mut sh7786_intc_desc);
}

pub unsafe fn plat_irq_setup_pins(mode: i32) {
    match mode {
        IRQ_MODE_IRQ7654 => { __raw_writel(__raw_readl(0xfe410000) | 0x00400000, 0xfe410000); register_intc_controller(&mut intc_desc_irq4567); }
        IRQ_MODE_IRQ3210 => { __raw_writel(__raw_readl(0xfe410000) | 0x00800000, 0xfe410000); register_intc_controller(&mut intc_desc_irq0123); }
        IRQ_MODE_IRL7654 => __raw_writel(0x40000000, 0xfe410050),
        IRQ_MODE_IRL3210 => __raw_writel(0x80000000, 0xfe410050),
        IRQ_MODE_IRL7654_MASK => { __raw_writel(0x40000000, 0xfe410050); register_intc_controller(&mut intc_desc_irl4567); }
        IRQ_MODE_IRL3210_MASK => { __raw_writel(0x80000000, 0xfe410050); register_intc_controller(&mut intc_desc_irl0123); }
        _ => BUG(),
    }
}

pub unsafe fn plat_mem_setup() {}

unsafe fn sh7786_devices_setup() -> i32 {
    sh7786_usb_setup();
    let irq = intc_irq_lookup(sh7786_intc_desc.name, TXI1);
    if irq > 0 { /* The C source installs SCIF1's ERI/RXI/TXI/BRI demux resources here. */ }
    let ret = platform_add_devices(sh7786_early_devices.as_mut_ptr(), sh7786_early_devices.len());
    if ret != 0 { return ret; }
    platform_add_devices(sh7786_devices.as_mut_ptr(), sh7786_devices.len())
}

pub unsafe fn plat_early_device_setup() { sh_early_platform_add_devices(sh7786_early_devices.as_mut_ptr(), sh7786_early_devices.len()); }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
