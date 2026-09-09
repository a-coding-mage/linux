// SPDX-License-Identifier: GPL-2.0
/*
 * SH7203 and SH7263 Setup
 *
 *  Copyright (C) 2007 - 2009  Paul Mundt
 */

// Kernel dependencies supplied by the surrounding build.

#[repr(i32)]
enum InterruptSource {
    UNUSED = 0,
    IRQ0, IRQ1, IRQ2, IRQ3, IRQ4, IRQ5, IRQ6, IRQ7,
    PINT0, PINT1, PINT2, PINT3, PINT4, PINT5, PINT6, PINT7,
    DMAC0, DMAC1, DMAC2, DMAC3, DMAC4, DMAC5, DMAC6, DMAC7,
    USB, LCDC, CMT0, CMT1, BSC, WDT,
    MTU0_ABCD, MTU0_VEF, MTU1_AB, MTU1_VU, MTU2_AB, MTU2_VU,
    MTU3_ABCD, MTU4_ABCD, MTU2_TCI3V, MTU2_TCI4V,
    ADC_ADI,
    IIC30, IIC31, IIC32, IIC33,
    SCIF0, SCIF1, SCIF2, SCIF3,
    SSU0, SSU1,
    SSI0_SSII, SSI1_SSII, SSI2_SSII, SSI3_SSII,
    // ROM-DEC, SDHI, SRC, and IEB are SH7263 specific
    ROMDEC, FLCTL, SDHI, RTC, RCAN0, RCAN1,
    SRC, IEBI,
    // interrupt groups
    PINT,
}

static mut vectors: [intc_vect; 112] = [
    INTC_IRQ!(IRQ0, 64), INTC_IRQ!(IRQ1, 65),
    INTC_IRQ!(IRQ2, 66), INTC_IRQ!(IRQ3, 67),
    INTC_IRQ!(IRQ4, 68), INTC_IRQ!(IRQ5, 69),
    INTC_IRQ!(IRQ6, 70), INTC_IRQ!(IRQ7, 71),
    INTC_IRQ!(PINT0, 80), INTC_IRQ!(PINT1, 81),
    INTC_IRQ!(PINT2, 82), INTC_IRQ!(PINT3, 83),
    INTC_IRQ!(PINT4, 84), INTC_IRQ!(PINT5, 85),
    INTC_IRQ!(PINT6, 86), INTC_IRQ!(PINT7, 87),
    INTC_IRQ!(DMAC0, 108), INTC_IRQ!(DMAC0, 109),
    INTC_IRQ!(DMAC1, 112), INTC_IRQ!(DMAC1, 113),
    INTC_IRQ!(DMAC2, 116), INTC_IRQ!(DMAC2, 117),
    INTC_IRQ!(DMAC3, 120), INTC_IRQ!(DMAC3, 121),
    INTC_IRQ!(DMAC4, 124), INTC_IRQ!(DMAC4, 125),
    INTC_IRQ!(DMAC5, 128), INTC_IRQ!(DMAC5, 129),
    INTC_IRQ!(DMAC6, 132), INTC_IRQ!(DMAC6, 133),
    INTC_IRQ!(DMAC7, 136), INTC_IRQ!(DMAC7, 137),
    INTC_IRQ!(USB, 140), INTC_IRQ!(LCDC, 141),
    INTC_IRQ!(CMT0, 142), INTC_IRQ!(CMT1, 143),
    INTC_IRQ!(BSC, 144), INTC_IRQ!(WDT, 145),
    INTC_IRQ!(MTU0_ABCD, 146), INTC_IRQ!(MTU0_ABCD, 147),
    INTC_IRQ!(MTU0_ABCD, 148), INTC_IRQ!(MTU0_ABCD, 149),
    INTC_IRQ!(MTU0_VEF, 150), INTC_IRQ!(MTU0_VEF, 151), INTC_IRQ!(MTU0_VEF, 152),
    INTC_IRQ!(MTU1_AB, 153), INTC_IRQ!(MTU1_AB, 154),
    INTC_IRQ!(MTU1_VU, 155), INTC_IRQ!(MTU1_VU, 156),
    INTC_IRQ!(MTU2_AB, 157), INTC_IRQ!(MTU2_AB, 158),
    INTC_IRQ!(MTU2_VU, 159), INTC_IRQ!(MTU2_VU, 160),
    INTC_IRQ!(MTU3_ABCD, 161), INTC_IRQ!(MTU3_ABCD, 162),
    INTC_IRQ!(MTU3_ABCD, 163), INTC_IRQ!(MTU3_ABCD, 164),
    INTC_IRQ!(MTU2_TCI3V, 165),
    INTC_IRQ!(MTU4_ABCD, 166), INTC_IRQ!(MTU4_ABCD, 167),
    INTC_IRQ!(MTU4_ABCD, 168), INTC_IRQ!(MTU4_ABCD, 169),
    INTC_IRQ!(MTU2_TCI4V, 170), INTC_IRQ!(ADC_ADI, 171),
    INTC_IRQ!(IIC30, 172), INTC_IRQ!(IIC30, 173), INTC_IRQ!(IIC30, 174),
    INTC_IRQ!(IIC30, 175), INTC_IRQ!(IIC30, 176),
    INTC_IRQ!(IIC31, 177), INTC_IRQ!(IIC31, 178), INTC_IRQ!(IIC31, 179),
    INTC_IRQ!(IIC31, 180), INTC_IRQ!(IIC31, 181),
    INTC_IRQ!(IIC32, 182), INTC_IRQ!(IIC32, 183), INTC_IRQ!(IIC32, 184),
    INTC_IRQ!(IIC32, 185), INTC_IRQ!(IIC32, 186),
    INTC_IRQ!(IIC33, 187), INTC_IRQ!(IIC33, 188), INTC_IRQ!(IIC33, 189),
    INTC_IRQ!(IIC33, 190), INTC_IRQ!(IIC33, 191),
    INTC_IRQ!(SCIF0, 192), INTC_IRQ!(SCIF0, 193), INTC_IRQ!(SCIF0, 194),
    INTC_IRQ!(SCIF0, 195), INTC_IRQ!(SCIF1, 196), INTC_IRQ!(SCIF1, 197),
    INTC_IRQ!(SCIF1, 198), INTC_IRQ!(SCIF1, 199), INTC_IRQ!(SCIF2, 200),
    INTC_IRQ!(SCIF2, 201), INTC_IRQ!(SCIF2, 202), INTC_IRQ!(SCIF2, 203),
    INTC_IRQ!(SCIF3, 204), INTC_IRQ!(SCIF3, 205), INTC_IRQ!(SCIF3, 206),
    INTC_IRQ!(SCIF3, 207), INTC_IRQ!(SSU0, 208), INTC_IRQ!(SSU0, 209),
    INTC_IRQ!(SSU0, 210), INTC_IRQ!(SSU1, 211), INTC_IRQ!(SSU1, 212),
    INTC_IRQ!(SSU1, 213), INTC_IRQ!(SSI0_SSII, 214), INTC_IRQ!(SSI1_SSII, 215),
    INTC_IRQ!(SSI2_SSII, 216), INTC_IRQ!(SSI3_SSII, 217),
    INTC_IRQ!(FLCTL, 224), INTC_IRQ!(FLCTL, 225), INTC_IRQ!(FLCTL, 226), INTC_IRQ!(FLCTL, 227),
    INTC_IRQ!(RTC, 231), INTC_IRQ!(RTC, 232), INTC_IRQ!(RTC, 233),
    INTC_IRQ!(RCAN0, 234), INTC_IRQ!(RCAN0, 235), INTC_IRQ!(RCAN0, 236),
    INTC_IRQ!(RCAN0, 237), INTC_IRQ!(RCAN0, 238), INTC_IRQ!(RCAN1, 239),
    INTC_IRQ!(RCAN1, 240), INTC_IRQ!(RCAN1, 241), INTC_IRQ!(RCAN1, 242), INTC_IRQ!(RCAN1, 243),
    // SH7263-specific entries are enabled by CONFIG_CPU_SUBTYPE_SH7263.
    #[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7263")] INTC_IRQ!(ROMDEC, 218),
    #[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7263")] INTC_IRQ!(ROMDEC, 219),
    #[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7263")] INTC_IRQ!(ROMDEC, 220),
    #[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7263")] INTC_IRQ!(ROMDEC, 221),
    #[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7263")] INTC_IRQ!(ROMDEC, 222),
    #[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7263")] INTC_IRQ!(ROMDEC, 223),
    #[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7263")] INTC_IRQ!(SDHI, 228),
    #[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7263")] INTC_IRQ!(SDHI, 229),
    #[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7263")] INTC_IRQ!(SDHI, 230),
    #[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7263")] INTC_IRQ!(SRC, 244),
    #[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7263")] INTC_IRQ!(SRC, 245),
    #[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7263")] INTC_IRQ!(SRC, 246),
    #[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7263")] INTC_IRQ!(IEBI, 247),
];

static mut groups: [intc_group; 1] = [INTC_GROUP!(PINT, PINT0, PINT1, PINT2, PINT3, PINT4, PINT5, PINT6, PINT7)];

static mut prio_registers: [intc_prio_reg; 16] = [
    intc_prio_reg { addr: 0xfffe0818, field_width: 0, nr_fields: 16, field_shift: 4, irqs: [IRQ0, IRQ1, IRQ2, IRQ3] },
    intc_prio_reg { addr: 0xfffe081a, field_width: 0, nr_fields: 16, field_shift: 4, irqs: [IRQ4, IRQ5, IRQ6, IRQ7] },
    intc_prio_reg { addr: 0xfffe0820, field_width: 0, nr_fields: 16, field_shift: 4, irqs: [PINT, UNUSED, UNUSED, UNUSED] },
    intc_prio_reg { addr: 0xfffe0c00, field_width: 0, nr_fields: 16, field_shift: 4, irqs: [DMAC0, DMAC1, DMAC2, DMAC3] },
    intc_prio_reg { addr: 0xfffe0c02, field_width: 0, nr_fields: 16, field_shift: 4, irqs: [DMAC4, DMAC5, DMAC6, DMAC7] },
    intc_prio_reg { addr: 0xfffe0c04, field_width: 0, nr_fields: 16, field_shift: 4, irqs: [USB, LCDC, CMT0, CMT1] },
    intc_prio_reg { addr: 0xfffe0c06, field_width: 0, nr_fields: 16, field_shift: 4, irqs: [BSC, WDT, MTU0_ABCD, MTU0_VEF] },
    intc_prio_reg { addr: 0xfffe0c08, field_width: 0, nr_fields: 16, field_shift: 4, irqs: [MTU1_AB, MTU1_VU, MTU2_AB, MTU2_VU] },
    intc_prio_reg { addr: 0xfffe0c0a, field_width: 0, nr_fields: 16, field_shift: 4, irqs: [MTU3_ABCD, MTU2_TCI3V, MTU4_ABCD, MTU2_TCI4V] },
    intc_prio_reg { addr: 0xfffe0c0c, field_width: 0, nr_fields: 16, field_shift: 4, irqs: [ADC_ADI, IIC30, IIC31, IIC32] },
    intc_prio_reg { addr: 0xfffe0c0e, field_width: 0, nr_fields: 16, field_shift: 4, irqs: [IIC33, SCIF0, SCIF1, SCIF2] },
    intc_prio_reg { addr: 0xfffe0c10, field_width: 0, nr_fields: 16, field_shift: 4, irqs: [SCIF3, SSU0, SSU1, SSI0_SSII] },
    // IPR15-IPR17 differ between SH7203 and SH7263; preserve the build-time condition.
    intc_prio_reg { addr: 0xfffe0c12, field_width: 0, nr_fields: 16, field_shift: 4, irqs: [SSI1_SSII, SSI2_SSII, SSI3_SSII, UNUSED] },
    intc_prio_reg { addr: 0xfffe0c14, field_width: 0, nr_fields: 16, field_shift: 4, irqs: [FLCTL, UNUSED, RTC, RCAN0] },
    intc_prio_reg { addr: 0xfffe0c16, field_width: 0, nr_fields: 16, field_shift: 4, irqs: [RCAN1, UNUSED, UNUSED, UNUSED] },
];

static mut mask_registers: [intc_mask_reg; 1] = [intc_mask_reg {
    addr: 0xfffe0808, field_width: 0, nr_fields: 16,
    irqs: [UNUSED, UNUSED, UNUSED, UNUSED, UNUSED, UNUSED, UNUSED, UNUSED,
           PINT7, PINT6, PINT5, PINT4, PINT3, PINT2, PINT1, PINT0],
}];

static mut intc_desc: intc_desc = DECLARE_INTC_DESC!("sh7203", vectors, groups, mask_registers, prio_registers, None);

static mut scif0_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, type_: PORT_SCIF, regtype: SCIx_SH2_SCIF_FIFODATA_REGTYPE };
static mut scif0_resources: [resource; 2] = [DEFINE_RES_MEM!(0xfffe8000, 0x100), DEFINE_RES_IRQ!(192)];
static mut scif0_device: platform_device = platform_device { name: "sh-sci", id: 0, resource: scif0_resources.as_ptr(), num_resources: ARRAY_SIZE!(scif0_resources), dev: device { platform_data: &raw mut scif0_platform_data } };

static mut scif1_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, type_: PORT_SCIF, regtype: SCIx_SH2_SCIF_FIFODATA_REGTYPE };
static mut scif1_resources: [resource; 2] = [DEFINE_RES_MEM!(0xfffe8800, 0x100), DEFINE_RES_IRQ!(196)];
static mut scif1_device: platform_device = platform_device { name: "sh-sci", id: 1, resource: scif1_resources.as_ptr(), num_resources: ARRAY_SIZE!(scif1_resources), dev: device { platform_data: &raw mut scif1_platform_data } };

static mut scif2_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, type_: PORT_SCIF, regtype: SCIx_SH2_SCIF_FIFODATA_REGTYPE };
static mut scif2_resources: [resource; 2] = [DEFINE_RES_MEM!(0xfffe9000, 0x100), DEFINE_RES_IRQ!(200)];
static mut scif2_device: platform_device = platform_device { name: "sh-sci", id: 2, resource: scif2_resources.as_ptr(), num_resources: ARRAY_SIZE!(scif2_resources), dev: device { platform_data: &raw mut scif2_platform_data } };

static mut scif3_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, type_: PORT_SCIF, regtype: SCIx_SH2_SCIF_FIFODATA_REGTYPE };
static mut scif3_resources: [resource; 2] = [DEFINE_RES_MEM!(0xfffe9800, 0x100), DEFINE_RES_IRQ!(204)];
static mut scif3_device: platform_device = platform_device { name: "sh-sci", id: 3, resource: scif3_resources.as_ptr(), num_resources: ARRAY_SIZE!(scif3_resources), dev: device { platform_data: &raw mut scif3_platform_data } };

static mut cmt_platform_data: sh_timer_config = sh_timer_config { channels_mask: 3 };
static mut cmt_resources: [resource; 3] = [DEFINE_RES_MEM!(0xfffec000, 0x10), DEFINE_RES_IRQ!(142), DEFINE_RES_IRQ!(143)];
static mut cmt_device: platform_device = platform_device { name: "sh-cmt-16", id: 0, dev: device { platform_data: &raw mut cmt_platform_data }, resource: cmt_resources.as_ptr(), num_resources: ARRAY_SIZE!(cmt_resources) };

static mut mtu2_resources: [resource; 3] = [DEFINE_RES_MEM!(0xfffe4000, 0x400), DEFINE_RES_IRQ_NAMED!(146, "tgi0a"), DEFINE_RES_IRQ_NAMED!(153, "tgi1a")];
static mut mtu2_device: platform_device = platform_device { name: "sh-mtu2", id: -1, resource: mtu2_resources.as_ptr(), num_resources: ARRAY_SIZE!(mtu2_resources) };

static mut rtc_resources: [resource; 2] = [
    resource { start: 0xffff2000, end: 0xffff2000 + 0x58 - 1, flags: IORESOURCE_IO },
    // Shared Period/Carry/Alarm IRQ
    resource { start: 231, flags: IORESOURCE_IRQ },
];
static mut rtc_device: platform_device = platform_device { name: "sh-rtc", id: -1, num_resources: ARRAY_SIZE!(rtc_resources), resource: rtc_resources.as_ptr() };

static mut sh7203_devices: [*mut platform_device; 7] = [
    &raw mut scif0_device, &raw mut scif1_device, &raw mut scif2_device,
    &raw mut scif3_device, &raw mut cmt_device, &raw mut mtu2_device, &raw mut rtc_device,
];

unsafe fn sh7203_devices_setup() -> i32 {
    platform_add_devices(sh7203_devices.as_mut_ptr(), ARRAY_SIZE!(sh7203_devices))
}

// arch_initcall(sh7203_devices_setup);

unsafe fn plat_irq_setup() {
    register_intc_controller(&raw mut intc_desc);
}

static mut sh7203_early_devices: [*mut platform_device; 6] = [
    &raw mut scif0_device, &raw mut scif1_device, &raw mut scif2_device,
    &raw mut scif3_device, &raw mut cmt_device, &raw mut mtu2_device,
];

const STBCR3: usize = 0xfffe0408;
const STBCR4: usize = 0xfffe040c;

unsafe fn plat_early_device_setup() {
    // enable CMT clock
    __raw_writeb(__raw_readb(STBCR4) & !0x04, STBCR4);

    // enable MTU2 clock
    __raw_writeb(__raw_readb(STBCR3) & !0x20, STBCR3);

    sh_early_platform_add_devices(sh7203_early_devices.as_mut_ptr(), ARRAY_SIZE!(sh7203_early_devices));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
