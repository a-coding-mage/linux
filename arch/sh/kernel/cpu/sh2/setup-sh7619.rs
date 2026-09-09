// SPDX-License-Identifier: GPL-2.0
/*
 * SH7619 Setup
 *
 *  Copyright (C) 2006  Yoshinori Sato
 *  Copyright (C) 2009  Paul Mundt
 */

// Linux headers and build-time macros are supplied by the surrounding kernel
// translation.  Their names are retained here as external dependencies.

#[repr(C)]
pub struct intc_vect { pub irq: i32, pub vect: i32 }
#[repr(C)]
pub struct intc_prio_reg { pub addr: u32, pub set_reg: i32, pub width: i32, pub field_width: i32, pub fields: [i32; 4] }
#[repr(C)]
pub struct plat_sci_port { pub scscr: u32, pub type_: u32 }
#[repr(C)]
pub struct resource { pub start: u32, pub end: u32, pub flags: u32 }
#[repr(C)]
pub struct platform_device { pub name: *const u8, pub id: i32, pub resource: *mut resource, pub num_resources: usize, pub platform_data: *mut core::ffi::c_void }
#[repr(C)]
pub struct sh_eth_plat_data { pub phy: i32, pub phy_interface: u32 }
#[repr(C)]
pub struct sh_timer_config { pub channels_mask: u32 }
#[repr(C)]
pub struct intc_desc { pub name: *const u8, pub vectors: *mut intc_vect, pub prio_registers: *mut intc_prio_reg }

const UNUSED: i32 = 0;
const IRQ0: i32 = 1; const IRQ1: i32 = 2; const IRQ2: i32 = 3; const IRQ3: i32 = 4;
const IRQ4: i32 = 5; const IRQ5: i32 = 6; const IRQ6: i32 = 7; const IRQ7: i32 = 8;
const WDT: i32 = 9; const EDMAC: i32 = 10; const CMT0: i32 = 11; const CMT1: i32 = 12;
const SCIF0: i32 = 13; const SCIF1: i32 = 14; const SCIF2: i32 = 15;
const HIF_HIFI: i32 = 16; const HIF_HIFBI: i32 = 17;
const DMAC0: i32 = 18; const DMAC1: i32 = 19; const DMAC2: i32 = 20; const DMAC3: i32 = 21;
const SIOF: i32 = 22;

static mut vectors: [intc_vect; 31] = [
    intc_vect { irq: IRQ0, vect: 64 }, intc_vect { irq: IRQ1, vect: 65 },
    intc_vect { irq: IRQ2, vect: 66 }, intc_vect { irq: IRQ3, vect: 67 },
    intc_vect { irq: IRQ4, vect: 80 }, intc_vect { irq: IRQ5, vect: 81 },
    intc_vect { irq: IRQ6, vect: 82 }, intc_vect { irq: IRQ7, vect: 83 },
    intc_vect { irq: WDT, vect: 84 }, intc_vect { irq: EDMAC, vect: 85 },
    intc_vect { irq: CMT0, vect: 86 }, intc_vect { irq: CMT1, vect: 87 },
    intc_vect { irq: SCIF0, vect: 88 }, intc_vect { irq: SCIF0, vect: 89 },
    intc_vect { irq: SCIF0, vect: 90 }, intc_vect { irq: SCIF0, vect: 91 },
    intc_vect { irq: SCIF1, vect: 92 }, intc_vect { irq: SCIF1, vect: 93 },
    intc_vect { irq: SCIF1, vect: 94 }, intc_vect { irq: SCIF1, vect: 95 },
    intc_vect { irq: SCIF2, vect: 96 }, intc_vect { irq: SCIF2, vect: 97 },
    intc_vect { irq: SCIF2, vect: 98 }, intc_vect { irq: SCIF2, vect: 99 },
    intc_vect { irq: HIF_HIFI, vect: 100 }, intc_vect { irq: HIF_HIFBI, vect: 101 },
    intc_vect { irq: DMAC0, vect: 104 }, intc_vect { irq: DMAC1, vect: 105 },
    intc_vect { irq: DMAC2, vect: 106 }, intc_vect { irq: DMAC3, vect: 107 },
    intc_vect { irq: SIOF, vect: 108 },
];

static mut prio_registers: [intc_prio_reg; 7] = [
    intc_prio_reg { addr: 0xf8140006, set_reg: 0, width: 16, field_width: 4, fields: [IRQ0, IRQ1, IRQ2, IRQ3] },
    intc_prio_reg { addr: 0xf8140008, set_reg: 0, width: 16, field_width: 4, fields: [IRQ4, IRQ5, IRQ6, IRQ7] },
    intc_prio_reg { addr: 0xf8080000, set_reg: 0, width: 16, field_width: 4, fields: [WDT, EDMAC, CMT0, CMT1] },
    intc_prio_reg { addr: 0xf8080002, set_reg: 0, width: 16, field_width: 4, fields: [SCIF0, SCIF1, SCIF2, UNUSED] },
    intc_prio_reg { addr: 0xf8080004, set_reg: 0, width: 16, field_width: 4, fields: [HIF_HIFI, HIF_HIFBI, UNUSED, UNUSED] },
    intc_prio_reg { addr: 0xf8080006, set_reg: 0, width: 16, field_width: 4, fields: [DMAC0, DMAC1, DMAC2, DMAC3] },
    intc_prio_reg { addr: 0xf8080008, set_reg: 0, width: 16, field_width: 4, fields: [SIOF, UNUSED, UNUSED, UNUSED] },
];

extern "C" {
    fn platform_add_devices(devices: *mut *mut platform_device, count: usize) -> i32;
    fn register_intc_controller(desc: *mut intc_desc);
    fn sh_early_platform_add_devices(devices: *mut *mut platform_device, count: usize);
    fn __raw_readb(addr: u32) -> u8;
    fn __raw_writeb(value: u8, addr: u32);
}

static mut intc_desc_instance: intc_desc = intc_desc { name: b"sh7619\0".as_ptr(), vectors: unsafe { vectors.as_mut_ptr() }, prio_registers: unsafe { prio_registers.as_mut_ptr() } };

const SCSCR_REIE: u32 = 0; // supplied by linux/serial_sci.h
const PORT_SCIF: u32 = 0; // supplied by linux/serial.h
const IORESOURCE_MEM: u32 = 0x00000200;
const IORESOURCE_IRQ: u32 = 0x00000400;
const PHY_INTERFACE_MODE_MII: u32 = 0;

static mut scif0_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, type_: PORT_SCIF };
static mut scif1_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, type_: PORT_SCIF };
static mut scif2_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, type_: PORT_SCIF };
static mut scif0_resources: [resource; 2] = [resource { start: 0xf8400000, end: 0xf84000ff, flags: IORESOURCE_MEM }, resource { start: 88, end: 88, flags: IORESOURCE_IRQ }];
static mut scif1_resources: [resource; 2] = [resource { start: 0xf8410000, end: 0xf84100ff, flags: IORESOURCE_MEM }, resource { start: 92, end: 92, flags: IORESOURCE_IRQ }];
static mut scif2_resources: [resource; 2] = [resource { start: 0xf8420000, end: 0xf84200ff, flags: IORESOURCE_MEM }, resource { start: 96, end: 96, flags: IORESOURCE_IRQ }];
static mut eth_platform_data: sh_eth_plat_data = sh_eth_plat_data { phy: 1, phy_interface: PHY_INTERFACE_MODE_MII };
static mut eth_resources: [resource; 2] = [resource { start: 0xfb000000, end: 0xfb0001c7, flags: IORESOURCE_MEM }, resource { start: 85, end: 85, flags: IORESOURCE_IRQ }];
static mut cmt_platform_data: sh_timer_config = sh_timer_config { channels_mask: 3 };
static mut cmt_resources: [resource; 3] = [resource { start: 0xf84a0070, end: 0xf84a007f, flags: IORESOURCE_MEM }, resource { start: 86, end: 86, flags: IORESOURCE_IRQ }, resource { start: 87, end: 87, flags: IORESOURCE_IRQ }];

// The platform_device initializers retain the C names, IDs, resources, and
// platform-data relationships; kernel-specific device fields are external.
static mut scif0_device: platform_device = platform_device { name: b"sh-sci\0".as_ptr(), id: 0, resource: unsafe { scif0_resources.as_mut_ptr() }, num_resources: 2, platform_data: unsafe { &mut scif0_platform_data as *mut _ as *mut _ } };
static mut scif1_device: platform_device = platform_device { name: b"sh-sci\0".as_ptr(), id: 1, resource: unsafe { scif1_resources.as_mut_ptr() }, num_resources: 2, platform_data: unsafe { &mut scif1_platform_data as *mut _ as *mut _ } };
static mut scif2_device: platform_device = platform_device { name: b"sh-sci\0".as_ptr(), id: 2, resource: unsafe { scif2_resources.as_mut_ptr() }, num_resources: 2, platform_data: unsafe { &mut scif2_platform_data as *mut _ as *mut _ } };
static mut eth_device: platform_device = platform_device { name: b"sh7619-ether\0".as_ptr(), id: -1, resource: unsafe { eth_resources.as_mut_ptr() }, num_resources: 2, platform_data: unsafe { &mut eth_platform_data as *mut _ as *mut _ } };
static mut cmt_device: platform_device = platform_device { name: b"sh-cmt-16\0".as_ptr(), id: 0, resource: unsafe { cmt_resources.as_mut_ptr() }, num_resources: 3, platform_data: unsafe { &mut cmt_platform_data as *mut _ as *mut _ } };
static mut sh7619_devices: [*mut platform_device; 5] = [&mut scif0_device, &mut scif1_device, &mut scif2_device, &mut eth_device, &mut cmt_device];
static mut sh7619_early_devices: [*mut platform_device; 4] = [&mut scif0_device, &mut scif1_device, &mut scif2_device, &mut cmt_device];

pub unsafe fn sh7619_devices_setup() -> i32 { platform_add_devices(sh7619_devices.as_mut_ptr(), 5) }
const STBCR3: u32 = 0xf80a0000;

pub unsafe fn plat_irq_setup() { register_intc_controller(&mut intc_desc_instance); }

pub unsafe fn plat_early_device_setup() {
    // enable CMT clock
    __raw_writeb(__raw_readb(STBCR3) & !0x10, STBCR3);
    sh_early_platform_add_devices(sh7619_early_devices.as_mut_ptr(), 4);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
