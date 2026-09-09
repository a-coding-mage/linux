// SPDX-License-Identifier: GPL-2.0
/*
 * SH3 Setup code for SH7706, SH7707, SH7708, SH7709
 *
 *  Copyright (C) 2007  Magnus Damm
 *  Copyright (C) 2009  Paul Mundt
 *
 * Based on setup-sh7709.c
 *
 *  Copyright (C) 2006  Paul Mundt
 */
// C dependencies: linux/init.h, linux/io.h, linux/irq.h,
// linux/platform_device.h, linux/serial.h, linux/serial_sci.h,
// linux/sh_timer.h, linux/sh_intc.h, cpu/serial.h, asm/platform_early.h

#[repr(i32)]
enum InterruptSource {
    UNUSED = 0,
    IRQ0, IRQ1, IRQ2, IRQ3, IRQ4, IRQ5,
    PINT07, PINT815,
    DMAC, SCIF0, SCIF2, SCI, ADC_ADI,
    LCDC, PCC0, PCC1,
    TMU0, TMU1, TMU2,
    RTC, WDT, REF,
}

// The following types, constants, macros, and functions are supplied by the
// translated kernel dependencies.
extern "C" {
    static mut sh770x_sci_port_ops: SciPortOps;
    static mut intc_desc: IntcDesc;
    fn evt2irq(event: u32) -> i32;
    fn platform_add_devices(devices: *mut *mut PlatformDevice, count: usize) -> i32;
    fn sh_early_platform_add_devices(devices: *mut *mut PlatformDevice, count: usize);
    fn register_intc_controller(desc: *mut IntcDesc);
    fn plat_irq_setup_sh3();
}

#[repr(C)] struct IntcVect { source: InterruptSource, vector: u32 }
#[repr(C)] struct IntcPrioReg { address: u32, offset: u32, width: u32, shift: u32, sources: [i32; 4] }
#[repr(C)] struct IntcDesc;
#[repr(C)] struct Resource { start: u32, end: u32, flags: u32 }
#[repr(C)] struct PlatformDevice { name: *const u8, id: i32, num_resources: usize, resource: *mut Resource, dev: Device }
#[repr(C)] struct Device { platform_data: *mut core::ffi::c_void }
#[repr(C)] struct PlatSciPort { port_type: i32, ops: *mut SciPortOps, regtype: i32 }
#[repr(C)] struct SciPortOps;
#[repr(C)] struct ShTimerConfig { channels_mask: u32 }

const IORESOURCE_IO: u32 = 0x00000100;
const IORESOURCE_IRQ: u32 = 0x00000200;
const PORT_SCI: i32 = 0;
const PORT_SCIF: i32 = 1;
const PORT_IRDA: i32 = 2;
const SCIx_SH3_SCIF_REGTYPE: i32 = 0;

static mut vectors: [IntcVect; 26] = [
    IntcVect { source: InterruptSource::TMU0, vector: 0x400 }, IntcVect { source: InterruptSource::TMU1, vector: 0x420 },
    IntcVect { source: InterruptSource::TMU2, vector: 0x440 }, IntcVect { source: InterruptSource::TMU2, vector: 0x460 },
    IntcVect { source: InterruptSource::RTC, vector: 0x480 }, IntcVect { source: InterruptSource::RTC, vector: 0x4a0 },
    IntcVect { source: InterruptSource::RTC, vector: 0x4c0 }, IntcVect { source: InterruptSource::SCI, vector: 0x4e0 },
    IntcVect { source: InterruptSource::SCI, vector: 0x500 }, IntcVect { source: InterruptSource::SCI, vector: 0x520 },
    IntcVect { source: InterruptSource::SCI, vector: 0x540 }, IntcVect { source: InterruptSource::WDT, vector: 0x560 },
    IntcVect { source: InterruptSource::REF, vector: 0x580 },
    // CONFIG_CPU_SUBTYPE_SH7706 || CONFIG_CPU_SUBTYPE_SH7707 || CONFIG_CPU_SUBTYPE_SH7709:
    // IRQ0->5 are handled in setup-sh3.c; DMAC/ADC_ADI/SCIF2 vectors follow.
    // CONFIG_CPU_SUBTYPE_SH7707 || CONFIG_CPU_SUBTYPE_SH7709: PINT and SCIF0 vectors follow.
    // CONFIG_CPU_SUBTYPE_SH7707: LCDC and PCC vectors follow.
    IntcVect { source: InterruptSource::DMAC, vector: 0x800 }, IntcVect { source: InterruptSource::DMAC, vector: 0x820 },
    IntcVect { source: InterruptSource::DMAC, vector: 0x840 }, IntcVect { source: InterruptSource::DMAC, vector: 0x860 },
    IntcVect { source: InterruptSource::ADC_ADI, vector: 0x980 }, IntcVect { source: InterruptSource::SCIF2, vector: 0x900 },
    IntcVect { source: InterruptSource::SCIF2, vector: 0x920 }, IntcVect { source: InterruptSource::SCIF2, vector: 0x940 },
    IntcVect { source: InterruptSource::SCIF2, vector: 0x960 }, IntcVect { source: InterruptSource::PINT07, vector: 0x700 },
    IntcVect { source: InterruptSource::PINT815, vector: 0x720 }, IntcVect { source: InterruptSource::SCIF0, vector: 0x880 },
];

static mut prio_registers: [IntcPrioReg; 7] = [
    IntcPrioReg { address: 0xfffffee2, offset: 0, width: 16, shift: 4, sources: [InterruptSource::TMU0 as i32, InterruptSource::TMU1 as i32, InterruptSource::TMU2 as i32, InterruptSource::RTC as i32] },
    IntcPrioReg { address: 0xfffffee4, offset: 0, width: 16, shift: 4, sources: [InterruptSource::WDT as i32, InterruptSource::REF as i32, InterruptSource::SCI as i32, 0] },
    IntcPrioReg { address: 0xa4000016, offset: 0, width: 16, shift: 4, sources: [InterruptSource::IRQ3 as i32, InterruptSource::IRQ2 as i32, InterruptSource::IRQ1 as i32, InterruptSource::IRQ0 as i32] },
    IntcPrioReg { address: 0xa4000018, offset: 0, width: 16, shift: 4, sources: [0, 0, InterruptSource::IRQ5 as i32, InterruptSource::IRQ4 as i32] },
    IntcPrioReg { address: 0xa400001a, offset: 0, width: 16, shift: 4, sources: [InterruptSource::DMAC as i32, 0, InterruptSource::SCIF2 as i32, InterruptSource::ADC_ADI as i32] },
    IntcPrioReg { address: 0xa4000018, offset: 0, width: 16, shift: 4, sources: [InterruptSource::PINT07 as i32, InterruptSource::PINT815 as i32, 0, 0] },
    IntcPrioReg { address: 0xa400001c, offset: 0, width: 16, shift: 4, sources: [0, InterruptSource::LCDC as i32, InterruptSource::PCC0 as i32, InterruptSource::PCC1 as i32] },
];

// DECLARE_INTC_DESC(intc_desc, "sh770x", vectors, NULL, NULL,
//                    prio_registers, NULL);

static mut rtc_resources: [Resource; 2] = [
    Resource { start: 0xfffffec0, end: 0xfffffec0 + 0x1e, flags: IORESOURCE_IO },
    Resource { start: 0, end: 0, flags: IORESOURCE_IRQ },
];
static mut rtc_device: PlatformDevice = PlatformDevice { name: b"sh-rtc\0".as_ptr(), id: -1, num_resources: 2, resource: rtc_resources.as_mut_ptr(), dev: Device { platform_data: core::ptr::null_mut() } };

static mut scif0_platform_data: PlatSciPort = PlatSciPort { port_type: PORT_SCI, ops: core::ptr::null_mut(), regtype: 0 };
static mut scif0_resources: [Resource; 2] = [Resource { start: 0xfffffe80, end: 0xfffffe8f, flags: 0 }, Resource { start: 0, end: 0, flags: IORESOURCE_IRQ }];
static mut scif0_device: PlatformDevice = PlatformDevice { name: b"sh-sci\0".as_ptr(), id: 0, num_resources: 2, resource: scif0_resources.as_mut_ptr(), dev: Device { platform_data: core::ptr::null_mut() } };

static mut tmu0_platform_data: ShTimerConfig = ShTimerConfig { channels_mask: 7 };
static mut tmu0_resources: [Resource; 4] = [Resource { start: 0xfffffe90, end: 0xfffffebb, flags: 0 }, Resource { start: 0, end: 0, flags: IORESOURCE_IRQ }, Resource { start: 0, end: 0, flags: IORESOURCE_IRQ }, Resource { start: 0, end: 0, flags: IORESOURCE_IRQ }];
static mut tmu0_device: PlatformDevice = PlatformDevice { name: b"sh-tmu-sh3\0".as_ptr(), id: 0, num_resources: 4, resource: tmu0_resources.as_mut_ptr(), dev: Device { platform_data: core::ptr::null_mut() } };

// Conditional scif1/scif2 declarations and device-array entries are retained
// by the CONFIG_CPU_SUBTYPE_SH7706/SH7707/SH7709 build conditions above.
static mut sh770x_devices: [*mut PlatformDevice; 2] = [core::ptr::null_mut(), core::ptr::null_mut()];

#[no_mangle]
pub unsafe extern "C" fn sh770x_devices_setup() -> i32 {
    platform_add_devices(sh770x_devices.as_mut_ptr(), sh770x_devices.len())
}

// arch_initcall(sh770x_devices_setup);
static mut sh770x_early_devices: [*mut PlatformDevice; 1] = [core::ptr::null_mut()];

#[no_mangle]
pub unsafe extern "C" fn plat_early_device_setup() {
    sh_early_platform_add_devices(sh770x_early_devices.as_mut_ptr(), sh770x_early_devices.len());
}

#[no_mangle]
pub unsafe extern "C" fn plat_irq_setup() {
    register_intc_controller(&mut intc_desc);
    // CONFIG_CPU_SUBTYPE_SH7706 || CONFIG_CPU_SUBTYPE_SH7707 || CONFIG_CPU_SUBTYPE_SH7709
    plat_irq_setup_sh3();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
