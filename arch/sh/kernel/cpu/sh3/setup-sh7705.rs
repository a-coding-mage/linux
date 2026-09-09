// SPDX-License-Identifier: GPL-2.0
/*
 * SH7705 Setup
 *
 *  Copyright (C) 2006 - 2009  Paul Mundt
 *  Copyright (C) 2007  Nobuhiro Iwamatsu
 */

// Translated dependencies supplied by the surrounding kernel bindings.

#[repr(C)]
pub struct intc_vect {
    pub irq: i32,
    pub vec: u16,
}

#[repr(C)]
pub struct intc_prio_reg {
    pub addr: u32,
    pub set_fn: i32,
    pub width: u32,
    pub fields: u32,
    pub irqs: [i32; 4],
}

#[repr(C)]
pub struct plat_sci_port {
    pub scscr: u32,
    pub type_: u32,
    pub ops: *const core::ffi::c_void,
    pub regtype: u32,
}

#[repr(C)]
pub struct resource {
    pub start: usize,
    pub end: usize,
    pub flags: u64,
}

#[repr(C)]
pub struct platform_device {
    pub name: *const u8,
    pub id: i32,
    pub resource: *mut resource,
    pub num_resources: usize,
    pub dev: platform_device_dev,
}

#[repr(C)]
pub struct platform_device_dev {
    pub platform_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct sh_rtc_platform_info {
    pub capabilities: u32,
}

#[repr(C)]
pub struct sh_timer_config {
    pub channels_mask: u32,
}

extern "C" {
    static sh770x_sci_port_ops: core::ffi::c_void;
    fn platform_add_devices(devices: *mut *mut platform_device, count: usize) -> i32;
    fn sh_early_platform_add_devices(devices: *mut *mut platform_device, count: usize);
    fn register_intc_controller(desc: *const core::ffi::c_void);
    fn plat_irq_setup_sh3();
}

const UNUSED: i32 = 0;
const IRQ0: i32 = 1;
const IRQ1: i32 = 2;
const IRQ2: i32 = 3;
const IRQ3: i32 = 4;
const IRQ4: i32 = 5;
const IRQ5: i32 = 6;
const PINT07: i32 = 7;
const PINT815: i32 = 8;
const DMAC: i32 = 9;
const SCIF0: i32 = 10;
const SCIF2: i32 = 11;
const ADC_ADI: i32 = 12;
const USB: i32 = 13;
const TPU0: i32 = 14;
const TPU1: i32 = 15;
const TPU2: i32 = 16;
const TPU3: i32 = 17;
const TMU0: i32 = 18;
const TMU1: i32 = 19;
const TMU2: i32 = 20;
const RTC: i32 = 21;
const WDT: i32 = 22;
const REF_RCMI: i32 = 23;

static mut vectors: [intc_vect; 28] = [
    intc_vect { irq: PINT07, vec: 0x700 }, intc_vect { irq: PINT815, vec: 0x720 },
    intc_vect { irq: DMAC, vec: 0x800 }, intc_vect { irq: DMAC, vec: 0x820 },
    intc_vect { irq: DMAC, vec: 0x840 }, intc_vect { irq: DMAC, vec: 0x860 },
    intc_vect { irq: SCIF0, vec: 0x880 }, intc_vect { irq: SCIF0, vec: 0x8a0 },
    intc_vect { irq: SCIF0, vec: 0x8e0 }, intc_vect { irq: SCIF2, vec: 0x900 },
    intc_vect { irq: SCIF2, vec: 0x920 }, intc_vect { irq: SCIF2, vec: 0x960 },
    intc_vect { irq: ADC_ADI, vec: 0x980 }, intc_vect { irq: USB, vec: 0xa20 },
    intc_vect { irq: USB, vec: 0xa40 }, intc_vect { irq: TPU0, vec: 0xc00 },
    intc_vect { irq: TPU1, vec: 0xc20 }, intc_vect { irq: TPU2, vec: 0xc80 },
    intc_vect { irq: TPU3, vec: 0xca0 }, intc_vect { irq: TMU0, vec: 0x400 },
    intc_vect { irq: TMU1, vec: 0x420 }, intc_vect { irq: TMU2, vec: 0x440 },
    intc_vect { irq: TMU2, vec: 0x460 }, intc_vect { irq: RTC, vec: 0x480 },
    intc_vect { irq: RTC, vec: 0x4a0 }, intc_vect { irq: RTC, vec: 0x4c0 },
    intc_vect { irq: WDT, vec: 0x560 }, intc_vect { irq: REF_RCMI, vec: 0x580 },
];

static mut prio_registers: [intc_prio_reg; 8] = [
    intc_prio_reg { addr: 0xfffffee2, set_fn: 0, width: 16, fields: 4, irqs: [TMU0, TMU1, TMU2, RTC] },
    intc_prio_reg { addr: 0xfffffee4, set_fn: 0, width: 16, fields: 4, irqs: [WDT, REF_RCMI, UNUSED, UNUSED] },
    intc_prio_reg { addr: 0xa4000016, set_fn: 0, width: 16, fields: 4, irqs: [IRQ3, IRQ2, IRQ1, IRQ0] },
    intc_prio_reg { addr: 0xa4000018, set_fn: 0, width: 16, fields: 4, irqs: [PINT07, PINT815, IRQ5, IRQ4] },
    intc_prio_reg { addr: 0xa400001a, set_fn: 0, width: 16, fields: 4, irqs: [DMAC, SCIF0, SCIF2, ADC_ADI] },
    intc_prio_reg { addr: 0xa4080000, set_fn: 0, width: 16, fields: 4, irqs: [UNUSED, UNUSED, USB, UNUSED] },
    intc_prio_reg { addr: 0xa4080002, set_fn: 0, width: 16, fields: 4, irqs: [TPU0, TPU1, UNUSED, UNUSED] },
    intc_prio_reg { addr: 0xa4080004, set_fn: 0, width: 16, fields: 4, irqs: [TPU2, TPU3, UNUSED, UNUSED] },
];

// The remaining platform objects retain the C aggregate layout and external macro-derived values.
extern "C" {
    static mut scif0_device: platform_device;
    static mut scif1_device: platform_device;
    static mut tmu0_device: platform_device;
    static mut rtc_device: platform_device;
}

static mut sh7705_devices: [*mut platform_device; 4] = unsafe {
    [&mut scif0_device, &mut scif1_device, &mut tmu0_device, &mut rtc_device]
};

pub unsafe fn sh7705_devices_setup() -> i32 {
    platform_add_devices(sh7705_devices.as_mut_ptr(), sh7705_devices.len())
}

static mut sh7705_early_devices: [*mut platform_device; 3] = unsafe {
    [&mut scif0_device, &mut scif1_device, &mut tmu0_device]
};

pub unsafe fn plat_early_device_setup() {
    sh_early_platform_add_devices(sh7705_early_devices.as_mut_ptr(), sh7705_early_devices.len());
}

pub unsafe fn plat_irq_setup() {
    register_intc_controller(core::ptr::null());
    plat_irq_setup_sh3();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
