// SPDX-License-Identifier: GPL-2.0
/*
 * SH3 Setup code for SH7710, SH7712
 *
 *  Copyright (C) 2006 - 2009  Paul Mundt
 *  Copyright (C) 2007  Nobuhiro Iwamatsu
 */

// Linux header dependencies and build-time configuration are supplied by the
// surrounding kernel translation.

#[repr(i32)]
enum IntcSource {
    UNUSED = 0,
    IRQ0, IRQ1, IRQ2, IRQ3, IRQ4, IRQ5,
    DMAC1, SCIF0, SCIF1, DMAC2, IPSEC,
    EDMAC0, EDMAC1, EDMAC2,
    SIOF0, SIOF1,
    TMU0, TMU1, TMU2,
    RTC, WDT, REF,
}

static mut VECTORS: [crate::intc_vect; 31] = [
    INTC_VECT!(DMAC1, 0x800), INTC_VECT!(DMAC1, 0x820),
    INTC_VECT!(DMAC1, 0x840), INTC_VECT!(DMAC1, 0x860),
    INTC_VECT!(SCIF0, 0x880), INTC_VECT!(SCIF0, 0x8a0),
    INTC_VECT!(SCIF0, 0x8c0), INTC_VECT!(SCIF0, 0x8e0),
    INTC_VECT!(SCIF1, 0x900), INTC_VECT!(SCIF1, 0x920),
    INTC_VECT!(SCIF1, 0x940), INTC_VECT!(SCIF1, 0x960),
    INTC_VECT!(DMAC2, 0xb80), INTC_VECT!(DMAC2, 0xba0),
    // CONFIG_CPU_SUBTYPE_SH7710: INTC_VECT!(IPSEC, 0xbe0),
    INTC_VECT!(EDMAC0, 0xc00), INTC_VECT!(EDMAC1, 0xc20),
    INTC_VECT!(EDMAC2, 0xc40),
    INTC_VECT!(SIOF0, 0xe00), INTC_VECT!(SIOF0, 0xe20),
    INTC_VECT!(SIOF0, 0xe40), INTC_VECT!(SIOF0, 0xe60),
    INTC_VECT!(SIOF1, 0xe80), INTC_VECT!(SIOF1, 0xea0),
    INTC_VECT!(SIOF1, 0xec0), INTC_VECT!(SIOF1, 0xee0),
    INTC_VECT!(TMU0, 0x400), INTC_VECT!(TMU1, 0x420),
    INTC_VECT!(TMU2, 0x440), INTC_VECT!(RTC, 0x480),
    INTC_VECT!(RTC, 0x4a0), INTC_VECT!(RTC, 0x4c0),
    INTC_VECT!(WDT, 0x560), INTC_VECT!(REF, 0x580),
];

static mut PRIO_REGISTERS: [crate::intc_prio_reg; 9] = [
    crate::intc_prio_reg::new(0xfffffee2, 0, 16, 4, [TMU0, TMU1, TMU2, RTC]),
    crate::intc_prio_reg::new(0xfffffee4, 0, 16, 4, [WDT, REF, UNUSED, UNUSED]),
    crate::intc_prio_reg::new(0xa4000016, 0, 16, 4, [IRQ3, IRQ2, IRQ1, IRQ0]),
    crate::intc_prio_reg::new(0xa4000018, 0, 16, 4, [UNUSED, UNUSED, IRQ5, IRQ4]),
    crate::intc_prio_reg::new(0xa400001a, 0, 16, 4, [DMAC1, SCIF0, SCIF1, UNUSED]),
    crate::intc_prio_reg::new(0xa4080000, 0, 16, 4, [IPSEC, DMAC2, UNUSED, UNUSED]),
    crate::intc_prio_reg::new(0xa4080002, 0, 16, 4, [EDMAC0, EDMAC1, EDMAC2, UNUSED]),
    crate::intc_prio_reg::new(0xa4080004, 0, 16, 4, [UNUSED, UNUSED, UNUSED, SIOF0]),
    crate::intc_prio_reg::new(0xa4080006, 0, 16, 4, [UNUSED, UNUSED, SIOF1, UNUSED]),
];

static mut INTC_DESC: crate::intc_desc = crate::intc_desc::new("sh7710", &mut VECTORS, &mut PRIO_REGISTERS);

static mut RTC_RESOURCES: [crate::resource; 2] = [
    crate::resource::io(0xa413fec0, 0xa413fedE, crate::IORESOURCE_IO),
    crate::resource::irq(crate::evt2irq(0x480)),
];
static mut RTC_INFO: crate::sh_rtc_platform_info = crate::sh_rtc_platform_info { capabilities: crate::RTC_CAP_4_DIGIT_YEAR };
static mut RTC_DEVICE: crate::platform_device = crate::platform_device::new("sh-rtc", -1, &mut RTC_RESOURCES, &mut RTC_INFO);

static mut SCIF0_PLATFORM_DATA: crate::plat_sci_port = crate::plat_sci_port { scscr: crate::SCSCR_REIE | crate::SCSCR_CKE1, type_: crate::PORT_SCIF };
static mut SCIF0_RESOURCES: [crate::resource; 2] = [crate::resource::mem(0xa4400000, 0x100), crate::resource::irq(crate::evt2irq(0x880))];
static mut SCIF0_DEVICE: crate::platform_device = crate::platform_device::new("sh-sci", 0, &mut SCIF0_RESOURCES, &mut SCIF0_PLATFORM_DATA);
static mut SCIF1_PLATFORM_DATA: crate::plat_sci_port = crate::plat_sci_port { scscr: crate::SCSCR_REIE | crate::SCSCR_CKE1, type_: crate::PORT_SCIF };
static mut SCIF1_RESOURCES: [crate::resource; 2] = [crate::resource::mem(0xa4410000, 0x100), crate::resource::irq(crate::evt2irq(0x900))];
static mut SCIF1_DEVICE: crate::platform_device = crate::platform_device::new("sh-sci", 1, &mut SCIF1_RESOURCES, &mut SCIF1_PLATFORM_DATA);
static mut TMU0_PLATFORM_DATA: crate::sh_timer_config = crate::sh_timer_config { channels_mask: 7 };
static mut TMU0_RESOURCES: [crate::resource; 4] = [crate::resource::mem(0xa412fe90, 0x28), crate::resource::irq(crate::evt2irq(0x400)), crate::resource::irq(crate::evt2irq(0x420)), crate::resource::irq(crate::evt2irq(0x440))];
static mut TMU0_DEVICE: crate::platform_device = crate::platform_device::new("sh-tmu-sh3", 0, &mut TMU0_RESOURCES, &mut TMU0_PLATFORM_DATA);

static mut SH7710_DEVICES: [*mut crate::platform_device; 4] = [&mut SCIF0_DEVICE, &mut SCIF1_DEVICE, &mut TMU0_DEVICE, &mut RTC_DEVICE];

unsafe extern "C" { fn platform_add_devices(devices: *mut *mut crate::platform_device, count: usize) -> i32; }
unsafe extern "C" { fn sh_early_platform_add_devices(devices: *mut *mut crate::platform_device, count: usize); }
unsafe extern "C" { fn register_intc_controller(desc: *mut crate::intc_desc); fn plat_irq_setup_sh3(); }

#[allow(non_snake_case)]
unsafe fn sh7710_devices_setup() -> i32 { platform_add_devices(SH7710_DEVICES.as_mut_ptr(), SH7710_DEVICES.len()) }

static mut SH7710_EARLY_DEVICES: [*mut crate::platform_device; 3] = [&mut SCIF0_DEVICE, &mut SCIF1_DEVICE, &mut TMU0_DEVICE];

pub unsafe fn plat_early_device_setup() { sh_early_platform_add_devices(SH7710_EARLY_DEVICES.as_mut_ptr(), SH7710_EARLY_DEVICES.len()); }
pub unsafe fn plat_irq_setup() { register_intc_controller(&mut INTC_DESC); plat_irq_setup_sh3(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
