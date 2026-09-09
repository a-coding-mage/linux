// SPDX-License-Identifier: GPL-2.0
/*
 * SH7091/SH7750/SH7750S/SH7750R/SH7751/SH7751R Setup
 *
 *  Copyright (C) 2006  Paul Mundt
 *  Copyright (C) 2006  Jamie Lenehan
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ptr;

#[repr(C)]
struct Resource { start: usize, end: usize, flags: usize }
#[repr(C)]
struct PlatformDevice { name: *const u8, id: i32, num_resources: usize, resource: *mut Resource, platform_data: *mut u8 }
#[repr(C)]
struct PlatSciPort { scscr: u32, r#type: u32 }
#[repr(C)]
struct ShTimerConfig { channels_mask: u32 }
#[repr(C)]
struct IntcVect { irq: i32, vector: u32 }
#[repr(C)]
struct IntcPrioReg { addr: u32, set: u32, width: u32, shift: u32, enum_ids: [i32; 8] }
#[repr(C)]
struct IntcMaskReg { set: u32, clr: u32, width: u32, enum_ids: [i32; 32] }
#[repr(C)]
struct IntcGroup { group: i32, members: [i32; 8] }
#[repr(C)]
struct IntcDesc;

extern "C" {
    fn evt2irq(vector: u32) -> usize;
    fn platform_device_register(dev: *mut PlatformDevice) -> i32;
    fn platform_add_devices(devs: *mut *mut PlatformDevice, count: usize) -> i32;
    fn mach_is_rts7751r2d() -> bool;
    fn sh_early_platform_add_devices(devs: *mut *mut PlatformDevice, count: usize);
    fn register_intc_controller(desc: *const IntcDesc);
    fn __raw_readw(addr: usize) -> u16;
    fn __raw_writew(value: u16, addr: usize);
    fn bug() -> !;
}

static mut vectors: [IntcVect; 20] = [
    vect!(HUDI, 0x600), vect!(GPIOI, 0x620), vect!(TMU0, 0x400), vect!(TMU1, 0x420),
    vect!(TMU2, 0x440), vect!(TMU2, 0x460), vect!(RTC, 0x480), vect!(RTC, 0x4a0),
    vect!(RTC, 0x4c0), vect!(SCI1, 0x4e0), vect!(SCI1, 0x500), vect!(SCI1, 0x520),
    vect!(SCI1, 0x540), vect!(SCIF, 0x700), vect!(SCIF, 0x720), vect!(SCIF, 0x740),
    vect!(SCIF, 0x760), vect!(WDT, 0x560), vect!(REF, 0x580), vect!(REF, 0x5a0),
];
static mut prio_registers: [IntcPrioReg; 5] = [
    IntcPrioReg { addr: 0xffd00004, set: 0, width: 16, shift: 4, enum_ids: [TMU0, TMU1, TMU2, RTC, 0, 0, 0, 0] },
    IntcPrioReg { addr: 0xffd00008, set: 0, width: 16, shift: 4, enum_ids: [WDT, REF, SCI1, 0, 0, 0, 0, 0] },
    IntcPrioReg { addr: 0xffd0000c, set: 0, width: 16, shift: 4, enum_ids: [GPIOI, DMAC, SCIF, HUDI, 0, 0, 0, 0] },
    IntcPrioReg { addr: 0xffd00010, set: 0, width: 16, shift: 4, enum_ids: [IRL0, IRL1, IRL2, IRL3, 0, 0, 0, 0] },
    IntcPrioReg { addr: 0xfe080000, set: 0, width: 32, shift: 4, enum_ids: [0, 0, 0, 0, TMU4, TMU3, PCIC1, PCIC0_PCISERR] },
];
static mut vectors_dma4: [IntcVect; 5] = [vect!(DMAC, 0x640), vect!(DMAC, 0x660), vect!(DMAC, 0x680), vect!(DMAC, 0x6a0), vect!(DMAC, 0x6c0)];
static mut vectors_dma8: [IntcVect; 8] = [vect!(DMAC, 0x640), vect!(DMAC, 0x660), vect!(DMAC, 0x680), vect!(DMAC, 0x6a0), vect!(DMAC, 0x780), vect!(DMAC, 0x7a0), vect!(DMAC, 0x7c0), vect!(DMAC, 0x7e0)];
static mut vectors_tmu34: [IntcVect; 2] = [vect!(TMU3, 0xb00), vect!(TMU4, 0xb80)];
static mut vectors_irlm: [IntcVect; 4] = [vect!(IRL0, 0x240), vect!(IRL1, 0x2a0), vect!(IRL2, 0x300), vect!(IRL3, 0x360)];
static mut vectors_pci: [IntcVect; 8] = [vect!(PCIC0_PCISERR, 0xa00), vect!(PCIC1_PCIERR, 0xae0), vect!(PCIC1_PCIPWDWN, 0xac0), vect!(PCIC1_PCIPWON, 0xaa0), vect!(PCIC1_PCIDMA0, 0xa80), vect!(PCIC1_PCIDMA1, 0xa60), vect!(PCIC1_PCIDMA2, 0xa40), vect!(PCIC1_PCIDMA3, 0xa20)];

#[no_mangle]
pub unsafe extern "C" fn plat_irq_setup() {
    register_intc_controller(&intc_desc);
    #[cfg(any(feature = "cpu_subtype_sh7750", feature = "cpu_subtype_sh7750s", feature = "cpu_subtype_sh7091", feature = "cpu_subtype_sh7751"))]
    register_intc_controller(&intc_desc_dma4);
    #[cfg(any(feature = "cpu_subtype_sh7750r", feature = "cpu_subtype_sh7751r"))]
    register_intc_controller(&intc_desc_dma8);
    #[cfg(any(feature = "cpu_subtype_sh7750r", feature = "cpu_subtype_sh7751", feature = "cpu_subtype_sh7751r"))]
    register_intc_controller(&intc_desc_tmu34);
    #[cfg(any(feature = "cpu_subtype_sh7751", feature = "cpu_subtype_sh7751r"))]
    register_intc_controller(&intc_desc_pci);
}

const IORESOURCE_IO: usize = 0x00000100;
const IORESOURCE_IRQ: usize = 0x00000200;
const PORT_SCI: u32 = 0;
const PORT_SCIF: u32 = 1;
const SCSCR_REIE: u32 = 1 << 2;
const SCSCR_CKE1: u32 = 1 << 1;

macro_rules! res_mem { ($start:expr, $size:expr) => { Resource { start: $start, end: $start + $size - 1, flags: 0 } }; }
macro_rules! res_irq { ($v:expr) => { Resource { start: unsafe { evt2irq($v) }, end: 0, flags: IORESOURCE_IRQ } }; }
macro_rules! vect { ($irq:expr, $v:expr) => { IntcVect { irq: $irq, vector: $v } }; }

static mut rtc_resources: [Resource; 2] = [
    Resource { start: 0xffc80000, end: 0xffc80000 + 0x58 - 1, flags: IORESOURCE_IO },
    Resource { start: 0, end: 0, flags: IORESOURCE_IRQ },
];
static mut rtc_device: PlatformDevice = PlatformDevice { name: b"sh-rtc\0".as_ptr(), id: -1, num_resources: 2, resource: ptr::null_mut(), platform_data: ptr::null_mut() };
static mut sci_platform_data: PlatSciPort = PlatSciPort { scscr: 0, r#type: PORT_SCI };
static mut sci_resources: [Resource; 2] = [res_mem!(0xffe00000, 0x20), res_irq!(0x4e0)];
static mut sci_device: PlatformDevice = PlatformDevice { name: b"sh-sci\0".as_ptr(), id: 0, num_resources: 2, resource: ptr::null_mut(), platform_data: ptr::null_mut() };
static mut scif_platform_data: PlatSciPort = PlatSciPort { scscr: SCSCR_REIE, r#type: PORT_SCIF };
static mut scif_resources: [Resource; 2] = [res_mem!(0xffe80000, 0x100), res_irq!(0x700)];
static mut scif_device: PlatformDevice = PlatformDevice { name: b"sh-sci\0".as_ptr(), id: 1, num_resources: 2, resource: ptr::null_mut(), platform_data: ptr::null_mut() };
static mut tmu0_platform_data: ShTimerConfig = ShTimerConfig { channels_mask: 7 };
static mut tmu0_resources: [Resource; 4] = [res_mem!(0xffd80000, 0x30), res_irq!(0x400), res_irq!(0x420), res_irq!(0x440)];
static mut tmu0_device: PlatformDevice = PlatformDevice { name: b"sh-tmu\0".as_ptr(), id: 0, num_resources: 4, resource: ptr::null_mut(), platform_data: ptr::null_mut() };

// The following declarations preserve the source's subtype-conditional objects.
#[cfg(any(feature = "cpu_subtype_sh7750r", feature = "cpu_subtype_sh7751", feature = "cpu_subtype_sh7751r"))]
static mut tmu1_platform_data: ShTimerConfig = ShTimerConfig { channels_mask: 3 };
#[cfg(any(feature = "cpu_subtype_sh7750r", feature = "cpu_subtype_sh7751", feature = "cpu_subtype_sh7751r"))]
static mut tmu1_resources: [Resource; 3] = [res_mem!(0xfe100000, 0x20), res_irq!(0xb00), res_irq!(0xb80)];
#[cfg(any(feature = "cpu_subtype_sh7750r", feature = "cpu_subtype_sh7751", feature = "cpu_subtype_sh7751r"))]
static mut tmu1_device: PlatformDevice = PlatformDevice { name: b"sh-tmu\0".as_ptr(), id: 1, num_resources: 3, resource: ptr::null_mut(), platform_data: ptr::null_mut() };

// Interrupt source numbers and descriptor contents are retained below; descriptor
// construction is provided by the architecture's INTC support.
const UNUSED: i32 = 0;
const IRL0: i32 = 1; const IRL1: i32 = 2; const IRL2: i32 = 3; const IRL3: i32 = 4;
const HUDI: i32 = 5; const GPIOI: i32 = 6; const DMAC: i32 = 7;
const PCIC0_PCISERR: i32 = 8; const PCIC1_PCIERR: i32 = 9; const PCIC1_PCIPWDWN: i32 = 10; const PCIC1_PCIPWON: i32 = 11;
const PCIC1_PCIDMA0: i32 = 12; const PCIC1_PCIDMA1: i32 = 13; const PCIC1_PCIDMA2: i32 = 14; const PCIC1_PCIDMA3: i32 = 15;
const TMU3: i32 = 16; const TMU4: i32 = 17; const TMU0: i32 = 18; const TMU1: i32 = 19; const TMU2: i32 = 20;
const RTC: i32 = 21; const SCI1: i32 = 22; const SCIF: i32 = 23; const WDT: i32 = 24; const REF: i32 = 25; const PCIC1: i32 = 26;

extern "C" {
    static mut intc_desc: IntcDesc;
    static mut intc_desc_dma4: IntcDesc;
    static mut intc_desc_dma8: IntcDesc;
    static mut intc_desc_tmu34: IntcDesc;
    static mut intc_desc_irlm: IntcDesc;
    static mut intc_desc_pci: IntcDesc;
}

#[no_mangle]
pub unsafe extern "C" fn sh7750_devices_setup() -> i32 {
    if mach_is_rts7751r2d() { platform_device_register(&mut scif_device); }
    else { platform_device_register(&mut sci_device); platform_device_register(&mut scif_device); }
    let mut devices: [*mut PlatformDevice; 2] = [&mut rtc_device, &mut tmu0_device];
    #[cfg(any(feature = "cpu_subtype_sh7750r", feature = "cpu_subtype_sh7751", feature = "cpu_subtype_sh7751r"))]
    { devices[1] = &mut tmu1_device; }
    platform_add_devices(devices.as_mut_ptr(), devices.len())
}

#[no_mangle]
pub unsafe extern "C" fn plat_early_device_setup() {
    let mut dev: [*mut PlatformDevice; 1] = [ptr::null_mut()];
    if mach_is_rts7751r2d() { scif_platform_data.scscr |= SCSCR_CKE1; dev[0] = &mut scif_device; sh_early_platform_add_devices(dev.as_mut_ptr(), 1); }
    else { dev[0] = &mut sci_device; sh_early_platform_add_devices(dev.as_mut_ptr(), 1); dev[0] = &mut scif_device; sh_early_platform_add_devices(dev.as_mut_ptr(), 1); }
    sh_early_platform_add_devices(&mut tmu0_device, 1);
}

const INTC_ICR: usize = 0xffd00000;
const INTC_ICR_IRLM: u16 = 1 << 7;

#[no_mangle]
pub unsafe extern "C" fn plat_irq_setup_pins(mode: i32) {
    #[cfg(any(feature = "cpu_subtype_sh7750", feature = "cpu_subtype_sh7091"))]
    { bug(); }
    if mode == 0 { __raw_writew(__raw_readw(INTC_ICR) | INTC_ICR_IRLM, INTC_ICR); register_intc_controller(&intc_desc_irlm); }
    else { bug(); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
