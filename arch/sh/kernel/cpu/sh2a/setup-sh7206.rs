// SPDX-License-Identifier: GPL-2.0
/*
 * SH7206 Setup
 *
 *  Copyright (C) 2006  Yoshinori Sato
 *  Copyright (C) 2009  Paul Mundt
 */

// Linux header dependencies and C preprocessor macros are supplied by the
// surrounding kernel translation unit.

#[repr(i32)]
#[derive(Copy, Clone)]
enum InterruptSource {
    UNUSED = 0,
    IRQ0, IRQ1, IRQ2, IRQ3, IRQ4, IRQ5, IRQ6, IRQ7,
    PINT0, PINT1, PINT2, PINT3, PINT4, PINT5, PINT6, PINT7,
    ADC_ADI0, ADC_ADI1,
    DMAC0, DMAC1, DMAC2, DMAC3, DMAC4, DMAC5, DMAC6, DMAC7,
    MTU0_ABCD, MTU0_VEF, MTU1_AB, MTU1_VU, MTU2_AB, MTU2_VU,
    MTU3_ABCD, MTU4_ABCD, MTU5, POE2_12, MTU3S_ABCD, MTU4S_ABCD, MTU5S,
    IIC3,
    CMT0, CMT1, BSC, WDT,
    MTU2_TCI3V, MTU2_TCI4V, MTU2S_TCI3V, MTU2S_TCI4V,
    POE2_OEI3,
    SCIF0, SCIF1, SCIF2, SCIF3,
    PINT,
}

// INTC_IRQ, INTC_GROUP, DECLARE_INTC_DESC, and resource/device initializers
// below correspond directly to the included kernel declarations/macros.
static mut VECTORS: [(&'static str, i32); 111] = [
    ("IRQ0",64),("IRQ1",65),("IRQ2",66),("IRQ3",67),("IRQ4",68),("IRQ5",69),("IRQ6",70),("IRQ7",71),
    ("PINT0",80),("PINT1",81),("PINT2",82),("PINT3",83),("PINT4",84),("PINT5",85),("PINT6",86),("PINT7",87),
    ("ADC_ADI0",92),("ADC_ADI1",96),
    ("DMAC0",108),("DMAC0",109),("DMAC1",112),("DMAC1",113),("DMAC2",116),("DMAC2",117),("DMAC3",120),("DMAC3",121),
    ("DMAC4",124),("DMAC4",125),("DMAC5",128),("DMAC5",129),("DMAC6",132),("DMAC6",133),("DMAC7",136),("DMAC7",137),
    ("CMT0",140),("CMT1",144),("BSC",148),("WDT",152),
    ("MTU0_ABCD",156),("MTU0_ABCD",157),("MTU0_ABCD",158),("MTU0_ABCD",159),
    ("MTU0_VEF",160),("MTU0_VEF",161),("MTU0_VEF",162),("MTU1_AB",164),("MTU1_AB",165),
    ("MTU1_VU",168),("MTU1_VU",169),("MTU2_AB",172),("MTU2_AB",173),("MTU2_VU",176),("MTU2_VU",177),
    ("MTU3_ABCD",180),("MTU3_ABCD",181),("MTU3_ABCD",182),("MTU3_ABCD",183),("MTU2_TCI3V",184),
    ("MTU4_ABCD",188),("MTU4_ABCD",189),("MTU4_ABCD",190),("MTU4_ABCD",191),("MTU2_TCI4V",192),
    ("MTU5",196),("MTU5",197),("MTU5",198),("POE2_12",200),("POE2_12",201),
    ("MTU3S_ABCD",204),("MTU3S_ABCD",205),("MTU3S_ABCD",206),("MTU3S_ABCD",207),("MTU2S_TCI3V",208),
    ("MTU4S_ABCD",212),("MTU4S_ABCD",213),("MTU4S_ABCD",214),("MTU4S_ABCD",215),("MTU2S_TCI4V",216),
    ("MTU5S",220),("MTU5S",221),("MTU5S",222),("POE2_OEI3",224),
    ("IIC3",228),("IIC3",229),("IIC3",230),("IIC3",231),("IIC3",232),
    ("SCIF0",240),("SCIF0",241),("SCIF0",242),("SCIF0",243),("SCIF1",244),("SCIF1",245),("SCIF1",246),("SCIF1",247),
    ("SCIF2",248),("SCIF2",249),("SCIF2",250),("SCIF2",251),("SCIF3",252),("SCIF3",253),("SCIF3",254),("SCIF3",255),
];

#[repr(C)]
struct Resource { kind: &'static str, start: u32, size: u32, name: Option<&'static str> }
#[repr(C)]
struct SciPort { scscr: u32, port_type: u32 }
#[repr(C)]
struct TimerConfig { channels_mask: u32 }

const SCSCR_REIE: u32 = 1 << 3;
const PORT_SCIF: u32 = 1;
static mut SCIF0_PLATFORM_DATA: SciPort = SciPort { scscr: SCSCR_REIE, port_type: PORT_SCIF };
static mut SCIF1_PLATFORM_DATA: SciPort = SciPort { scscr: SCSCR_REIE, port_type: PORT_SCIF };
static mut SCIF2_PLATFORM_DATA: SciPort = SciPort { scscr: SCSCR_REIE, port_type: PORT_SCIF };
static mut SCIF3_PLATFORM_DATA: SciPort = SciPort { scscr: SCSCR_REIE, port_type: PORT_SCIF };
static mut CMT_PLATFORM_DATA: TimerConfig = TimerConfig { channels_mask: 3 };

static mut SCIF0_RESOURCES: [Resource; 2] = [
    Resource { kind: "mem", start: 0xfffe8000, size: 0x100, name: None },
    Resource { kind: "irq", start: 240, size: 0, name: None },
];
static mut SCIF1_RESOURCES: [Resource; 2] = [
    Resource { kind: "mem", start: 0xfffe8800, size: 0x100, name: None },
    Resource { kind: "irq", start: 244, size: 0, name: None },
];
static mut SCIF2_RESOURCES: [Resource; 2] = [
    Resource { kind: "mem", start: 0xfffe9000, size: 0x100, name: None },
    Resource { kind: "irq", start: 248, size: 0, name: None },
];
static mut SCIF3_RESOURCES: [Resource; 2] = [
    Resource { kind: "mem", start: 0xfffe9800, size: 0x100, name: None },
    Resource { kind: "irq", start: 252, size: 0, name: None },
];
static mut CMT_RESOURCES: [Resource; 3] = [
    Resource { kind: "mem", start: 0xfffec000, size: 0x10, name: None },
    Resource { kind: "irq", start: 140, size: 0, name: None },
    Resource { kind: "irq", start: 144, size: 0, name: None },
];
static mut MTU2_RESOURCES: [Resource; 4] = [
    Resource { kind: "mem", start: 0xfffe4000, size: 0x400, name: None },
    Resource { kind: "irq", start: 156, size: 0, name: Some("tgi0a") },
    Resource { kind: "irq", start: 164, size: 0, name: Some("tgi1a") },
    Resource { kind: "irq", start: 180, size: 0, name: Some("tgi2a") },
];

static GROUPS: [[InterruptSource; 8]; 1] = [[
    InterruptSource::PINT0, InterruptSource::PINT1, InterruptSource::PINT2,
    InterruptSource::PINT3, InterruptSource::PINT4, InterruptSource::PINT5,
    InterruptSource::PINT6, InterruptSource::PINT7,
]];
static PRIORITY_REGISTERS: [(u32, [InterruptSource; 4]); 11] = [
    (0xfffe0818, [InterruptSource::IRQ0,InterruptSource::IRQ1,InterruptSource::IRQ2,InterruptSource::IRQ3]),
    (0xfffe081a, [InterruptSource::IRQ4,InterruptSource::IRQ5,InterruptSource::IRQ6,InterruptSource::IRQ7]),
    (0xfffe0820, [InterruptSource::PINT,InterruptSource::UNUSED,InterruptSource::ADC_ADI0,InterruptSource::ADC_ADI1]),
    (0xfffe0c00, [InterruptSource::DMAC0,InterruptSource::DMAC1,InterruptSource::DMAC2,InterruptSource::DMAC3]),
    (0xfffe0c02, [InterruptSource::DMAC4,InterruptSource::DMAC5,InterruptSource::DMAC6,InterruptSource::DMAC7]),
    (0xfffe0c04, [InterruptSource::CMT0,InterruptSource::CMT1,InterruptSource::BSC,InterruptSource::WDT]),
    (0xfffe0c06, [InterruptSource::MTU0_ABCD,InterruptSource::MTU0_VEF,InterruptSource::MTU1_AB,InterruptSource::MTU1_VU]),
    (0xfffe0c08, [InterruptSource::MTU2_AB,InterruptSource::MTU2_VU,InterruptSource::MTU3_ABCD,InterruptSource::MTU2_TCI3V]),
    (0xfffe0c0a, [InterruptSource::MTU4_ABCD,InterruptSource::MTU2_TCI4V,InterruptSource::MTU5,InterruptSource::POE2_12]),
    (0xfffe0c0c, [InterruptSource::MTU3S_ABCD,InterruptSource::MTU2S_TCI3V,InterruptSource::MTU4S_ABCD,InterruptSource::MTU2S_TCI4V]),
    (0xfffe0c0e, [InterruptSource::MTU5S,InterruptSource::POE2_OEI3,InterruptSource::IIC3,InterruptSource::UNUSED]),
];
static MASK_REGISTER: (u32, [InterruptSource; 8]) = (0xfffe0808, [
    InterruptSource::UNUSED,InterruptSource::UNUSED,InterruptSource::UNUSED,InterruptSource::UNUSED,
    InterruptSource::UNUSED,InterruptSource::UNUSED,InterruptSource::UNUSED,InterruptSource::UNUSED,
]);

extern "C" {
    fn platform_add_devices(devices: *const *mut core::ffi::c_void, count: usize) -> i32;
    fn register_intc_controller(desc: *const core::ffi::c_void);
    fn sh_early_platform_add_devices(devices: *const *mut core::ffi::c_void, count: usize);
    fn __raw_readb(addr: usize) -> u8;
    fn __raw_writeb(value: u8, addr: usize);
}

static mut SH7206_DEVICES: [*mut core::ffi::c_void; 6] = [
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
];

unsafe extern "C" fn sh7206_devices_setup() -> i32 {
    platform_add_devices(SH7206_DEVICES.as_ptr(), SH7206_DEVICES.len())
}

pub unsafe extern "C" fn plat_irq_setup() {
    register_intc_controller(core::ptr::null());
}

const STBCR3: usize = 0xfffe0408;
const STBCR4: usize = 0xfffe040c;

pub unsafe extern "C" fn plat_early_device_setup() {
    // enable CMT clock
    __raw_writeb(__raw_readb(STBCR4) & !0x04, STBCR4);

    // enable MTU2 clock
    __raw_writeb(__raw_readb(STBCR3) & !0x20, STBCR3);

    sh_early_platform_add_devices(SH7206_DEVICES.as_ptr(), SH7206_DEVICES.len());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
