// SPDX-License-Identifier: GPL-2.0
/*
 *  SH7201 setup
 *
 *  Copyright (C) 2008  Peter Griffin pgriffin@mpc-data.co.uk
 *  Copyright (C) 2009  Paul Mundt
 */

// Linux header dependencies are supplied by the surrounding translation unit.

#[repr(i32)]
enum InterruptSource {
    UNUSED = 0,
    IRQ0, IRQ1, IRQ2, IRQ3, IRQ4, IRQ5, IRQ6, IRQ7,
    PINT0, PINT1, PINT2, PINT3, PINT4, PINT5, PINT6, PINT7,
    ADC_ADI,
    MTU20_ABCD, MTU20_VEF, MTU21_AB, MTU21_VU, MTU22_AB, MTU22_VU,
    MTU23_ABCD, MTU24_ABCD, MTU25_UVW, MTU2_TCI3V, MTU2_TCI4V,
    RTC, WDT,
    IIC30, IIC31, IIC32,
    DMAC0_DMINT0, DMAC1_DMINT1, DMAC2_DMINT2, DMAC3_DMINT3,
    SCIF0, SCIF1, SCIF2, SCIF3, SCIF4, SCIF5, SCIF6, SCIF7,
    DMAC0_DMINTA, DMAC4_DMINT4, DMAC5_DMINT5, DMAC6_DMINT6, DMAC7_DMINT7,
    RCAN0, RCAN1,
    SSI0_SSII, SSI1_SSII,
    TMR0, TMR1,
    PINT,
}

static mut VECTORS: &[IntcVect] = &[
    INTC_IRQ!(IRQ0,64), INTC_IRQ!(IRQ1,65), INTC_IRQ!(IRQ2,66), INTC_IRQ!(IRQ3,67),
    INTC_IRQ!(IRQ4,68), INTC_IRQ!(IRQ5,69), INTC_IRQ!(IRQ6,70), INTC_IRQ!(IRQ7,71),
    INTC_IRQ!(PINT0,80), INTC_IRQ!(PINT1,81), INTC_IRQ!(PINT2,82), INTC_IRQ!(PINT3,83),
    INTC_IRQ!(PINT4,84), INTC_IRQ!(PINT5,85), INTC_IRQ!(PINT6,86), INTC_IRQ!(PINT7,87),
    INTC_IRQ!(ADC_ADI,92),
    INTC_IRQ!(MTU20_ABCD,108), INTC_IRQ!(MTU20_ABCD,109), INTC_IRQ!(MTU20_ABCD,110), INTC_IRQ!(MTU20_ABCD,111),
    INTC_IRQ!(MTU20_VEF,112), INTC_IRQ!(MTU20_VEF,113), INTC_IRQ!(MTU20_VEF,114),
    INTC_IRQ!(MTU21_AB,116), INTC_IRQ!(MTU21_AB,117), INTC_IRQ!(MTU21_VU,120), INTC_IRQ!(MTU21_VU,121),
    INTC_IRQ!(MTU22_AB,124), INTC_IRQ!(MTU22_AB,125), INTC_IRQ!(MTU22_VU,128), INTC_IRQ!(MTU22_VU,129),
    INTC_IRQ!(MTU23_ABCD,132), INTC_IRQ!(MTU23_ABCD,133), INTC_IRQ!(MTU23_ABCD,134), INTC_IRQ!(MTU23_ABCD,135),
    INTC_IRQ!(MTU2_TCI3V,136),
    INTC_IRQ!(MTU24_ABCD,140), INTC_IRQ!(MTU24_ABCD,141), INTC_IRQ!(MTU24_ABCD,142), INTC_IRQ!(MTU24_ABCD,143),
    INTC_IRQ!(MTU2_TCI4V,144), INTC_IRQ!(MTU25_UVW,148), INTC_IRQ!(MTU25_UVW,149), INTC_IRQ!(MTU25_UVW,150),
    INTC_IRQ!(RTC,152), INTC_IRQ!(RTC,153), INTC_IRQ!(RTC,154), INTC_IRQ!(WDT,156),
    INTC_IRQ!(IIC30,157), INTC_IRQ!(IIC30,158), INTC_IRQ!(IIC30,159), INTC_IRQ!(IIC30,160), INTC_IRQ!(IIC30,161),
    INTC_IRQ!(IIC31,164), INTC_IRQ!(IIC31,165), INTC_IRQ!(IIC31,166), INTC_IRQ!(IIC31,167), INTC_IRQ!(IIC31,168),
    INTC_IRQ!(IIC32,170), INTC_IRQ!(IIC32,171), INTC_IRQ!(IIC32,172), INTC_IRQ!(IIC32,173), INTC_IRQ!(IIC32,174),
    INTC_IRQ!(DMAC0_DMINT0,176), INTC_IRQ!(DMAC1_DMINT1,177), INTC_IRQ!(DMAC2_DMINT2,178), INTC_IRQ!(DMAC3_DMINT3,179),
    INTC_IRQ!(SCIF0,180), INTC_IRQ!(SCIF0,181), INTC_IRQ!(SCIF0,182), INTC_IRQ!(SCIF0,183),
    INTC_IRQ!(SCIF1,184), INTC_IRQ!(SCIF1,185), INTC_IRQ!(SCIF1,186), INTC_IRQ!(SCIF1,187),
    INTC_IRQ!(SCIF2,188), INTC_IRQ!(SCIF2,189), INTC_IRQ!(SCIF2,190), INTC_IRQ!(SCIF2,191),
    INTC_IRQ!(SCIF3,192), INTC_IRQ!(SCIF3,193), INTC_IRQ!(SCIF3,194), INTC_IRQ!(SCIF3,195),
    INTC_IRQ!(SCIF4,196), INTC_IRQ!(SCIF4,197), INTC_IRQ!(SCIF4,198), INTC_IRQ!(SCIF4,199),
    INTC_IRQ!(SCIF5,200), INTC_IRQ!(SCIF5,201), INTC_IRQ!(SCIF5,202), INTC_IRQ!(SCIF5,203),
    INTC_IRQ!(SCIF6,204), INTC_IRQ!(SCIF6,205), INTC_IRQ!(SCIF6,206), INTC_IRQ!(SCIF6,207),
    INTC_IRQ!(SCIF7,208), INTC_IRQ!(SCIF7,209), INTC_IRQ!(SCIF7,210), INTC_IRQ!(SCIF7,211),
    INTC_IRQ!(DMAC0_DMINTA,212), INTC_IRQ!(DMAC4_DMINT4,216), INTC_IRQ!(DMAC5_DMINT5,217), INTC_IRQ!(DMAC6_DMINT6,218), INTC_IRQ!(DMAC7_DMINT7,219),
    INTC_IRQ!(RCAN0,228), INTC_IRQ!(RCAN0,229), INTC_IRQ!(RCAN0,230), INTC_IRQ!(RCAN0,231), INTC_IRQ!(RCAN0,232),
    INTC_IRQ!(RCAN1,234), INTC_IRQ!(RCAN1,235), INTC_IRQ!(RCAN1,236), INTC_IRQ!(RCAN1,237), INTC_IRQ!(RCAN1,238),
    INTC_IRQ!(SSI0_SSII,244), INTC_IRQ!(SSI1_SSII,245), INTC_IRQ!(TMR0,246), INTC_IRQ!(TMR0,247), INTC_IRQ!(TMR0,248),
    INTC_IRQ!(TMR1,252), INTC_IRQ!(TMR1,253), INTC_IRQ!(TMR1,254),
];

static mut GROUPS: &[IntcGroup] = &[INTC_GROUP!(PINT, PINT0, PINT1, PINT2, PINT3, PINT4, PINT5, PINT6, PINT7)];

static mut PRIO_REGISTERS: &[IntcPrioReg] = &[
    intc_prio!(0xfffe9418, 0, 16, 4, IRQ0, IRQ1, IRQ2, IRQ3), intc_prio!(0xfffe941a, 0, 16, 4, IRQ4, IRQ5, IRQ6, IRQ7),
    intc_prio!(0xfffe9420, 0, 16, 4, PINT, 0, ADC_ADI, 0), intc_prio!(0xfffe9800, 0, 16, 4, 0, MTU20_ABCD, MTU20_VEF, MTU21_AB),
    intc_prio!(0xfffe9802, 0, 16, 4, MTU21_VU, MTU22_AB, MTU22_VU, MTU23_ABCD), intc_prio!(0xfffe9804, 0, 16, 4, MTU2_TCI3V, MTU24_ABCD, MTU2_TCI4V, MTU25_UVW),
    intc_prio!(0xfffe9806, 0, 16, 4, RTC, WDT, IIC30, 0), intc_prio!(0xfffe9808, 0, 16, 4, IIC31, IIC32, DMAC0_DMINT0, DMAC1_DMINT1),
    intc_prio!(0xfffe980a, 0, 16, 4, DMAC2_DMINT2, DMAC3_DMINT3, SCIF0, SCIF1), intc_prio!(0xfffe980c, 0, 16, 4, SCIF2, SCIF3, SCIF4, SCIF5),
    intc_prio!(0xfffe980e, 0, 16, 4, SCIF6, SCIF7, DMAC0_DMINTA, DMAC4_DMINT4), intc_prio!(0xfffe9810, 0, 16, 4, DMAC5_DMINT5, DMAC6_DMINT6, DMAC7_DMINT7, 0),
    intc_prio!(0xfffe9812, 0, 16, 4, 0, RCAN0, RCAN1, 0), intc_prio!(0xfffe9814, 0, 16, 4, SSI0_SSII, SSI1_SSII, TMR0, TMR1),
];

static mut MASK_REGISTERS: &[IntcMaskReg] = &[intc_mask!(0xfffe9408, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, PINT7, PINT6, PINT5, PINT4, PINT3, PINT2, PINT1, PINT0)];
static INTC_DESC: IntcDesc = DECLARE_INTC_DESC!("sh7201", VECTORS, GROUPS, MASK_REGISTERS, PRIO_REGISTERS, None);

static mut SCIF_PLATFORM_DATA: [PlatSciPort; 8] = [PlatSciPort { scscr: SCSCR_REIE, port_type: PORT_SCIF }; 8];
static mut SCIF_RESOURCES: [Resource; 16] = [
    DEFINE_RES_MEM!(0xfffe8000,0x100), DEFINE_RES_IRQ!(180), DEFINE_RES_MEM!(0xfffe8800,0x100), DEFINE_RES_IRQ!(184),
    DEFINE_RES_MEM!(0xfffe9000,0x100), DEFINE_RES_IRQ!(188), DEFINE_RES_MEM!(0xfffe9800,0x100), DEFINE_RES_IRQ!(192),
    DEFINE_RES_MEM!(0xfffea000,0x100), DEFINE_RES_IRQ!(196), DEFINE_RES_MEM!(0xfffea800,0x100), DEFINE_RES_IRQ!(200),
    DEFINE_RES_MEM!(0xfffeb000,0x100), DEFINE_RES_IRQ!(204), DEFINE_RES_MEM!(0xfffeb800,0x100), DEFINE_RES_IRQ!(208),
];

static mut RTC_RESOURCES: [Resource; 2] = [Resource { start: 0xffff0800, end: 0xffff2000 + 0x58 - 1, flags: IORESOURCE_IO }, Resource { start: 152, flags: IORESOURCE_IRQ }];
static mut MTU2_RESOURCES: [Resource; 4] = [DEFINE_RES_MEM!(0xfffe4000,0x400), DEFINE_RES_IRQ_NAMED!(108, "tgi0a"), DEFINE_RES_IRQ_NAMED!(116, "tgi1a"), DEFINE_RES_IRQ_NAMED!(124, "tgi1b")];

static mut scif0_device: *mut PlatformDevice = platform_device!("sh-sci", 0, &SCIF_RESOURCES[0], 2, &SCIF_PLATFORM_DATA[0]);
static mut scif1_device: *mut PlatformDevice = platform_device!("sh-sci", 1, &SCIF_RESOURCES[2], 2, &SCIF_PLATFORM_DATA[1]);
static mut scif2_device: *mut PlatformDevice = platform_device!("sh-sci", 2, &SCIF_RESOURCES[4], 2, &SCIF_PLATFORM_DATA[2]);
static mut scif3_device: *mut PlatformDevice = platform_device!("sh-sci", 3, &SCIF_RESOURCES[6], 2, &SCIF_PLATFORM_DATA[3]);
static mut scif4_device: *mut PlatformDevice = platform_device!("sh-sci", 4, &SCIF_RESOURCES[8], 2, &SCIF_PLATFORM_DATA[4]);
static mut scif5_device: *mut PlatformDevice = platform_device!("sh-sci", 5, &SCIF_RESOURCES[10], 2, &SCIF_PLATFORM_DATA[5]);
static mut scif6_device: *mut PlatformDevice = platform_device!("sh-sci", 6, &SCIF_RESOURCES[12], 2, &SCIF_PLATFORM_DATA[6]);
static mut scif7_device: *mut PlatformDevice = platform_device!("sh-sci", 7, &SCIF_RESOURCES[14], 2, &SCIF_PLATFORM_DATA[7]);
static mut rtc_device: *mut PlatformDevice = platform_device!("sh-rtc", -1, &RTC_RESOURCES, 2, None);
static mut mtu2_device: *mut PlatformDevice = platform_device!("sh-mtu2", -1, &MTU2_RESOURCES, 4, None);

static mut SH7201_DEVICES: [*mut PlatformDevice; 10] = [scif0_device, scif1_device, scif2_device, scif3_device, scif4_device, scif5_device, scif6_device, scif7_device, rtc_device, mtu2_device];
unsafe fn sh7201_devices_setup() -> i32 { platform_add_devices(SH7201_DEVICES.as_ptr(), SH7201_DEVICES.len()) }
// arch_initcall(sh7201_devices_setup)
unsafe fn plat_irq_setup() { register_intc_controller(&INTC_DESC); }
static mut SH7201_EARLY_DEVICES: [*mut PlatformDevice; 9] = [scif0_device, scif1_device, scif2_device, scif3_device, scif4_device, scif5_device, scif6_device, scif7_device, mtu2_device];

const STBCR3: usize = 0xfffe0408;
unsafe fn plat_early_device_setup() {
    // enable MTU2 clock
    __raw_writeb(__raw_readb(STBCR3) & !0x20, STBCR3);
    sh_early_platform_add_devices(SH7201_EARLY_DEVICES.as_ptr(), SH7201_EARLY_DEVICES.len());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
