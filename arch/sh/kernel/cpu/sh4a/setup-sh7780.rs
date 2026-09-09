// SPDX-License-Identifier: GPL-2.0
/*
 * SH7780 Setup
 *
 *  Copyright (C) 2006  Paul Mundt
 */
// C headers and kernel-provided types/macros are external dependencies.

static mut scif0_platform_data: plat_sci_port = plat_sci_port {
    scscr: SCSCR_REIE | SCSCR_CKE1, type_: PORT_SCIF,
    regtype: SCIx_SH4_SCIF_FIFODATA_REGTYPE,
};
static mut scif0_resources: [resource; 2] = [
    DEFINE_RES_MEM!(0xffe00000, 0x100), DEFINE_RES_IRQ!(evt2irq(0x700)),
];
static mut scif0_device: platform_device = platform_device {
    name: "sh-sci", id: 0, resource: scif0_resources.as_mut_ptr(),
    num_resources: scif0_resources.len(), dev: device { platform_data: unsafe { &mut scif0_platform_data } },
};

static mut scif1_platform_data: plat_sci_port = plat_sci_port {
    scscr: SCSCR_REIE | SCSCR_CKE1, type_: PORT_SCIF,
    regtype: SCIx_SH4_SCIF_FIFODATA_REGTYPE,
};
static mut scif1_resources: [resource; 2] = [
    DEFINE_RES_MEM!(0xffe10000, 0x100), DEFINE_RES_IRQ!(evt2irq(0xb80)),
];
static mut scif1_device: platform_device = platform_device {
    name: "sh-sci", id: 1, resource: scif1_resources.as_mut_ptr(),
    num_resources: scif1_resources.len(), dev: device { platform_data: unsafe { &mut scif1_platform_data } },
};

static mut tmu0_platform_data: sh_timer_config = sh_timer_config { channels_mask: 7 };
static mut tmu0_resources: [resource; 4] = [
    DEFINE_RES_MEM!(0xffd80000, 0x30), DEFINE_RES_IRQ!(evt2irq(0x580)),
    DEFINE_RES_IRQ!(evt2irq(0x5a0)), DEFINE_RES_IRQ!(evt2irq(0x5c0)),
];
static mut tmu0_device: platform_device = platform_device {
    name: "sh-tmu", id: 0, dev: device { platform_data: unsafe { &mut tmu0_platform_data } },
    resource: tmu0_resources.as_mut_ptr(), num_resources: tmu0_resources.len(),
};
static mut tmu1_platform_data: sh_timer_config = sh_timer_config { channels_mask: 7 };
static mut tmu1_resources: [resource; 4] = [
    DEFINE_RES_MEM!(0xffdc0000, 0x2c), DEFINE_RES_IRQ!(evt2irq(0xe00)),
    DEFINE_RES_IRQ!(evt2irq(0xe20)), DEFINE_RES_IRQ!(evt2irq(0xe40)),
];
static mut tmu1_device: platform_device = platform_device {
    name: "sh-tmu", id: 1, dev: device { platform_data: unsafe { &mut tmu1_platform_data } },
    resource: tmu1_resources.as_mut_ptr(), num_resources: tmu1_resources.len(),
};

static mut rtc_resources: [resource; 2] = [
    resource { start: 0xffe80000, end: 0xffe80000 + 0x58 - 1, flags: IORESOURCE_IO },
    resource { start: evt2irq(0x480), end: 0, flags: IORESOURCE_IRQ },
];
static mut rtc_device: platform_device = platform_device {
    name: "sh-rtc", id: -1, num_resources: rtc_resources.len(), resource: rtc_resources.as_mut_ptr(),
};

static sh7780_dmae0_channels: [sh_dmae_channel; 6] = [
    sh_dmae_channel { offset: 0, dmars: 0, dmars_bit: 0 },
    sh_dmae_channel { offset: 0x10, dmars: 0, dmars_bit: 8 },
    sh_dmae_channel { offset: 0x20, dmars: 4, dmars_bit: 0 },
    sh_dmae_channel { offset: 0x30, dmars: 4, dmars_bit: 8 },
    sh_dmae_channel { offset: 0x50, dmars: 8, dmars_bit: 0 },
    sh_dmae_channel { offset: 0x60, dmars: 8, dmars_bit: 8 },
];
static sh7780_dmae1_channels: [sh_dmae_channel; 6] = [
    sh_dmae_channel { offset: 0, ..Default::default() }, sh_dmae_channel { offset: 0x10, ..Default::default() },
    sh_dmae_channel { offset: 0x20, ..Default::default() }, sh_dmae_channel { offset: 0x30, ..Default::default() },
    sh_dmae_channel { offset: 0x50, ..Default::default() }, sh_dmae_channel { offset: 0x60, ..Default::default() },
];
static ts_shift: &[unsigned int] = &TS_SHIFT;
static mut dma0_platform_data: sh_dmae_pdata = sh_dmae_pdata {
    channel: sh7780_dmae0_channels.as_ptr(), channel_num: sh7780_dmae0_channels.len(),
    ts_low_shift: CHCR_TS_LOW_SHIFT, ts_low_mask: CHCR_TS_LOW_MASK,
    ts_high_shift: CHCR_TS_HIGH_SHIFT, ts_high_mask: CHCR_TS_HIGH_MASK,
    ts_shift, ts_shift_num: TS_SHIFT.len(), dmaor_init: DMAOR_INIT,
};
static mut dma1_platform_data: sh_dmae_pdata = sh_dmae_pdata {
    channel: sh7780_dmae1_channels.as_ptr(), channel_num: sh7780_dmae1_channels.len(),
    ts_low_shift: CHCR_TS_LOW_SHIFT, ts_low_mask: CHCR_TS_LOW_MASK,
    ts_high_shift: CHCR_TS_HIGH_SHIFT, ts_high_mask: CHCR_TS_HIGH_MASK,
    ts_shift, ts_shift_num: TS_SHIFT.len(), dmaor_init: DMAOR_INIT,
};

// Resource tables retain the C initializers and comments; resource/device types are kernel dependencies.
static mut sh7780_dmae0_resources: [resource; 3] = [
    resource { start: 0xfc808020, end: 0xfc80808f, flags: IORESOURCE_MEM },
    resource { start: 0xfc809000, end: 0xfc80900b, flags: IORESOURCE_MEM },
    resource { name: "error_irq", start: evt2irq(0x640), end: evt2irq(0x640), flags: IORESOURCE_IRQ | IORESOURCE_IRQ_SHAREABLE },
];
static mut sh7780_dmae1_resources: [resource; 2] = [
    resource { start: 0xfc818020, end: 0xfc81808f, flags: IORESOURCE_MEM },
    resource { name: "error_irq", start: evt2irq(0x7c0), end: evt2irq(0x7c0), flags: IORESOURCE_IRQ | IORESOURCE_IRQ_SHAREABLE },
];
static mut dma0_device: platform_device = platform_device { name: "sh-dma-engine", id: 0, resource: sh7780_dmae0_resources.as_mut_ptr(), num_resources: 3, dev: device { platform_data: unsafe { &mut dma0_platform_data } } };
static mut dma1_device: platform_device = platform_device { name: "sh-dma-engine", id: 1, resource: sh7780_dmae1_resources.as_mut_ptr(), num_resources: 2, dev: device { platform_data: unsafe { &mut dma1_platform_data } } };

static mut sh7780_devices: [*mut platform_device; 7] = [
    &raw mut scif0_device, &raw mut scif1_device, &raw mut tmu0_device,
    &raw mut tmu1_device, &raw mut rtc_device, &raw mut dma0_device, &raw mut dma1_device,
];
unsafe fn sh7780_devices_setup() -> i32 { platform_add_devices(sh7780_devices.as_mut_ptr(), sh7780_devices.len()) }
// arch_initcall(sh7780_devices_setup)
static mut sh7780_early_devices: [*mut platform_device; 4] = [
    &raw mut scif0_device, &raw mut scif1_device, &raw mut tmu0_device, &raw mut tmu1_device,
];
unsafe fn plat_early_device_setup() {
    if mach_is_sh2007() { scif0_platform_data.scscr &= !SCSCR_CKE1; scif1_platform_data.scscr &= !SCSCR_CKE1; }
    sh_early_platform_add_devices(sh7780_early_devices.as_mut_ptr(), sh7780_early_devices.len());
}

// The following interrupt descriptors preserve the source tables and macro-defined kernel layouts.
static mut vectors: [intc_vect; 52] = [
    INTC_VECT!(RTC,0x480), INTC_VECT!(RTC,0x4a0), INTC_VECT!(RTC,0x4c0), INTC_VECT!(WDT,0x560),
    INTC_VECT!(TMU0,0x580), INTC_VECT!(TMU1,0x5a0), INTC_VECT!(TMU2,0x5c0), INTC_VECT!(TMU2_TICPI,0x5e0),
    INTC_VECT!(HUDI,0x600), INTC_VECT!(DMAC0,0x640), INTC_VECT!(DMAC0,0x660), INTC_VECT!(DMAC0,0x680),
    INTC_VECT!(DMAC0,0x6a0), INTC_VECT!(DMAC0,0x6c0), INTC_VECT!(SCIF0,0x700), INTC_VECT!(SCIF0,0x720),
    INTC_VECT!(SCIF0,0x740), INTC_VECT!(SCIF0,0x760), INTC_VECT!(DMAC0,0x780), INTC_VECT!(DMAC0,0x7a0),
    INTC_VECT!(DMAC1,0x7c0), INTC_VECT!(DMAC1,0x7e0), INTC_VECT!(CMT,0x900), INTC_VECT!(HAC,0x980),
    INTC_VECT!(PCISERR,0xa00), INTC_VECT!(PCIINTA,0xa20), INTC_VECT!(PCIINTB,0xa40), INTC_VECT!(PCIINTC,0xa60),
    INTC_VECT!(PCIINTD,0xa80), INTC_VECT!(PCIC5,0xaa0), INTC_VECT!(PCIC5,0xac0), INTC_VECT!(PCIC5,0xae0),
    INTC_VECT!(PCIC5,0xb00), INTC_VECT!(PCIC5,0xb20), INTC_VECT!(SCIF1,0xb80), INTC_VECT!(SCIF1,0xba0),
    INTC_VECT!(SCIF1,0xbc0), INTC_VECT!(SCIF1,0xbe0), INTC_VECT!(SIOF,0xc00), INTC_VECT!(HSPI,0xc80),
    INTC_VECT!(MMCIF,0xd00), INTC_VECT!(MMCIF,0xd20), INTC_VECT!(MMCIF,0xd40), INTC_VECT!(MMCIF,0xd60),
    INTC_VECT!(DMAC1,0xd80), INTC_VECT!(DMAC1,0xda0), INTC_VECT!(DMAC1,0xdc0), INTC_VECT!(DMAC1,0xde0),
    INTC_VECT!(TMU3,0xe00), INTC_VECT!(TMU4,0xe20), INTC_VECT!(TMU5,0xe40), INTC_VECT!(SSI,0xe80),
    INTC_VECT!(FLCTL,0xf00), INTC_VECT!(FLCTL,0xf20), INTC_VECT!(FLCTL,0xf40), INTC_VECT!(FLCTL,0xf60),
    INTC_VECT!(GPIO,0xf80), INTC_VECT!(GPIO,0xfa0), INTC_VECT!(GPIO,0xfc0), INTC_VECT!(GPIO,0xfe0),
];
static mut groups: [intc_group; 2] = [INTC_GROUP!(TMU012,TMU0,TMU1,TMU2,TMU2_TICPI), INTC_GROUP!(TMU345,TMU3,TMU4,TMU5)];
// Interrupt-source identifiers (the kernel headers provide the enum layout).
#[repr(i32)] enum interrupt_source {
    UNUSED = 0,
    IRL_LLLL, IRL_LLLH, IRL_LLHL, IRL_LLHH, IRL_LHLL, IRL_LHLH, IRL_LHHL, IRL_LHHH,
    IRL_HLLL, IRL_HLLH, IRL_HLHL, IRL_HLHH, IRL_HHLL, IRL_HHLH, IRL_HHHL,
    IRQ0, IRQ1, IRQ2, IRQ3, IRQ4, IRQ5, IRQ6, IRQ7, RTC, WDT, TMU0, TMU1, TMU2,
    TMU2_TICPI, HUDI, DMAC0, SCIF0, DMAC1, CMT, HAC, PCISERR, PCIINTA, PCIINTB,
    PCIINTC, PCIINTD, PCIC5, SCIF1, SIOF, HSPI, MMCIF, TMU3, TMU4, TMU5, SSI, FLCTL, GPIO,
    TMU012, TMU345,
}
static mut mask_registers: [intc_mask_reg; 1] = [intc_mask_reg { set: 0xffd40038, clear: 0xffd4003c, width: 32, fields: [0,0,0,0,GPIO,FLCTL,SSI,MMCIF,HSPI,SIOF,PCIC5,PCIINTD,PCIINTC,PCIINTB,PCIINTA,PCISERR,HAC,CMT,0,0,DMAC1,DMAC0,HUDI,0,WDT,SCIF1,SCIF0,RTC,TMU345,TMU012] }];
static mut prio_registers: [intc_prio_reg; 8] = [
    intc_prio_reg { addr:0xffd40000, offset:0, width:32, field_width:8, fields:[TMU0,TMU1,TMU2,TMU2_TICPI] }, intc_prio_reg { addr:0xffd40004, offset:0, width:32, field_width:8, fields:[TMU3,TMU4,TMU5,RTC] }, intc_prio_reg { addr:0xffd40008, offset:0, width:32, field_width:8, fields:[SCIF0,SCIF1,WDT] }, intc_prio_reg { addr:0xffd4000c, offset:0, width:32, field_width:8, fields:[HUDI,DMAC0,DMAC1] },
    intc_prio_reg { addr:0xffd40010, offset:0, width:32, field_width:8, fields:[CMT,HAC,PCISERR,PCIINTA] }, intc_prio_reg { addr:0xffd40014, offset:0, width:32, field_width:8, fields:[PCIINTB,PCIINTC,PCIINTD,PCIC5] }, intc_prio_reg { addr:0xffd40018, offset:0, width:32, field_width:8, fields:[SIOF,HSPI,MMCIF,SSI] }, intc_prio_reg { addr:0xffd4001c, offset:0, width:32, field_width:8, fields:[FLCTL,GPIO] },
];
static mut irq_vectors: [intc_vect; 8] = [INTC_VECT!(IRQ0,0x240),INTC_VECT!(IRQ1,0x280),INTC_VECT!(IRQ2,0x2c0),INTC_VECT!(IRQ3,0x300),INTC_VECT!(IRQ4,0x340),INTC_VECT!(IRQ5,0x380),INTC_VECT!(IRQ6,0x3c0),INTC_VECT!(IRQ7,0x200)];
static mut irl_vectors: [intc_vect; 15] = [INTC_VECT!(IRL_LLLL,0x200),INTC_VECT!(IRL_LLLH,0x220),INTC_VECT!(IRL_LLHL,0x240),INTC_VECT!(IRL_LLHH,0x260),INTC_VECT!(IRL_LHLL,0x280),INTC_VECT!(IRL_LHLH,0x2a0),INTC_VECT!(IRL_LHHL,0x2c0),INTC_VECT!(IRL_LHHH,0x2e0),INTC_VECT!(IRL_HLLL,0x300),INTC_VECT!(IRL_HLLH,0x320),INTC_VECT!(IRL_HLHL,0x340),INTC_VECT!(IRL_HLHH,0x360),INTC_VECT!(IRL_HHLL,0x380),INTC_VECT!(IRL_HHLH,0x3a0),INTC_VECT!(IRL_HHHL,0x3c0)];
static mut irq_mask_registers: [intc_mask_reg;1] = [intc_mask_reg { set:0xffd00044, clear:0xffd00064, width:32, fields:[IRQ0,IRQ1,IRQ2,IRQ3,IRQ4,IRQ5,IRQ6,IRQ7] }];
static mut irq_prio_registers: [intc_prio_reg;1] = [intc_prio_reg { addr:0xffd00010, offset:0, width:32, field_width:4, fields:[IRQ0,IRQ1,IRQ2,IRQ3,IRQ4,IRQ5,IRQ6,IRQ7] }];
static mut irq_sense_registers: [intc_sense_reg;1] = [intc_sense_reg { addr:0xffd0001c, width:32, field_width:2, fields:[IRQ0,IRQ1,IRQ2,IRQ3,IRQ4,IRQ5,IRQ6,IRQ7] }];
static mut irq_ack_registers: [intc_mask_reg;1] = [intc_mask_reg { set:0xffd00024, clear:0, width:32, fields:[IRQ0,IRQ1,IRQ2,IRQ3,IRQ4,IRQ5,IRQ6,IRQ7] }];
static mut irl3210_mask_registers: [intc_mask_reg;1] = [intc_mask_reg { set:0xffd40080, clear:0xffd40084, width:32, fields:[IRL_LLLL,IRL_LLLH,IRL_LLHL,IRL_LLHH,IRL_LHLL,IRL_LHLH,IRL_LHHL,IRL_LHHH,IRL_HLLL,IRL_HLLH,IRL_HLHL,IRL_HLHH,IRL_HHLL,IRL_HHLH,IRL_HHHL] }];
static mut irl7654_mask_registers: [intc_mask_reg;1] = [intc_mask_reg { set:0xffd40080, clear:0xffd40084, width:32, fields:[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,IRL_LLLL,IRL_LLLH,IRL_LLHL,IRL_LLHH,IRL_LHLL,IRL_LHLH,IRL_LHHL,IRL_LHHH,IRL_HLLL,IRL_HLLH,IRL_HLHL,IRL_HLHH,IRL_HHLL,IRL_HHLH,IRL_HHHL] }];
static mut intc_desc: intc_desc = DECLARE_INTC_DESC!("sh7780", vectors, groups, mask_registers, prio_registers, None);
static mut intc_irq_desc: intc_desc = DECLARE_INTC_DESC_ACK!("sh7780-irq", irq_vectors, None, irq_mask_registers, irq_prio_registers, irq_sense_registers, irq_ack_registers);
static mut intc_irl7654_desc: intc_desc = DECLARE_INTC_DESC!("sh7780-irl7654", irl_vectors, None, irl7654_mask_registers, None, None);
static mut intc_irl3210_desc: intc_desc = DECLARE_INTC_DESC!("sh7780-irl3210", irl_vectors, None, irl3210_mask_registers, None, None);

const INTC_ICR0: usize = 0xffd00000;
const INTC_INTMSK0: usize = 0xffd00044;
const INTC_INTMSK1: usize = 0xffd00048;
const INTC_INTMSK2: usize = 0xffd40080;
const INTC_INTMSKCLR1: usize = 0xffd00068;
const INTC_INTMSKCLR2: usize = 0xffd40084;

unsafe fn plat_irq_setup() {
    __raw_writel(0xff000000, INTC_INTMSK0); __raw_writel(0xc0000000, INTC_INTMSK1); __raw_writel(0xfffefffe, INTC_INTMSK2);
    __raw_writel(__raw_readl(INTC_ICR0) & !0x00c00000, INTC_ICR0);
    __raw_writel(__raw_readl(INTC_ICR0) | 0x00200000, INTC_ICR0);
    register_intc_controller(&raw mut intc_desc);
}
unsafe fn plat_irq_setup_pins(mode: i32) {
    match mode {
        IRQ_MODE_IRQ => { __raw_writel(__raw_readl(INTC_ICR0) | 0x00c00000, INTC_ICR0); register_intc_controller(&raw mut intc_irq_desc); }
        IRQ_MODE_IRL7654 => { __raw_writel(0x40000000, INTC_INTMSKCLR1); __raw_writel(0x0000fffe, INTC_INTMSKCLR2); }
        IRQ_MODE_IRL3210 => { __raw_writel(0x80000000, INTC_INTMSKCLR1); __raw_writel(0xfffe0000, INTC_INTMSKCLR2); }
        IRQ_MODE_IRL7654_MASK => { __raw_writel(0x40000000, INTC_INTMSKCLR1); register_intc_controller(&raw mut intc_irl7654_desc); }
        IRQ_MODE_IRL3210_MASK => { __raw_writel(0x80000000, INTC_INTMSKCLR1); register_intc_controller(&raw mut intc_irl3210_desc); }
        _ => BUG!(),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
