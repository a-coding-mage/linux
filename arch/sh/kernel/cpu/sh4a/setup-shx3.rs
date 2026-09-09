// SPDX-License-Identifier: GPL-2.0
/* SH-X3 Prototype Setup. Copyright (C) 2007 - 2010 Paul Mundt. */
// Linux kernel headers and build-time attributes are supplied by external dependencies.

static mut SCIF0_PLATFORM_DATA: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, type_: PORT_SCIF };
static mut SCIF0_RESOURCES: [resource; 5] = [
    DEFINE_RES_MEM(0xffc30000, 0x100), DEFINE_RES_IRQ(evt2irq(0x700)),
    DEFINE_RES_IRQ(evt2irq(0x720)), DEFINE_RES_IRQ(evt2irq(0x760)), DEFINE_RES_IRQ(evt2irq(0x740)),
];
static mut SCIF0_DEVICE: platform_device = platform_device { name: "sh-sci", id: 0, resource: SCIF0_RESOURCES.as_mut_ptr(), num_resources: 5, dev: device { platform_data: &raw mut SCIF0_PLATFORM_DATA } };

static mut SCIF1_PLATFORM_DATA: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, type_: PORT_SCIF };
static mut SCIF1_RESOURCES: [resource; 5] = [
    DEFINE_RES_MEM(0xffc40000, 0x100), DEFINE_RES_IRQ(evt2irq(0x780)),
    DEFINE_RES_IRQ(evt2irq(0x7a0)), DEFINE_RES_IRQ(evt2irq(0x7e0)), DEFINE_RES_IRQ(evt2irq(0x7c0)),
];
static mut SCIF1_DEVICE: platform_device = platform_device { name: "sh-sci", id: 1, resource: SCIF1_RESOURCES.as_mut_ptr(), num_resources: 5, dev: device { platform_data: &raw mut SCIF1_PLATFORM_DATA } };

static mut SCIF2_PLATFORM_DATA: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, type_: PORT_SCIF };
static mut SCIF2_RESOURCES: [resource; 5] = [
    DEFINE_RES_MEM(0xffc60000, 0x100), DEFINE_RES_IRQ(evt2irq(0x880)),
    DEFINE_RES_IRQ(evt2irq(0x8a0)), DEFINE_RES_IRQ(evt2irq(0x8e0)), DEFINE_RES_IRQ(evt2irq(0x8c0)),
];
static mut SCIF2_DEVICE: platform_device = platform_device { name: "sh-sci", id: 2, resource: SCIF2_RESOURCES.as_mut_ptr(), num_resources: 5, dev: device { platform_data: &raw mut SCIF2_PLATFORM_DATA } };

static mut TMU0_PLATFORM_DATA: sh_timer_config = sh_timer_config { channels_mask: 7 };
static mut TMU0_RESOURCES: [resource; 4] = [DEFINE_RES_MEM(0xffc10000, 0x30), DEFINE_RES_IRQ(evt2irq(0x400)), DEFINE_RES_IRQ(evt2irq(0x420)), DEFINE_RES_IRQ(evt2irq(0x440))];
static mut TMU0_DEVICE: platform_device = platform_device { name: "sh-tmu", id: 0, dev: device { platform_data: &raw mut TMU0_PLATFORM_DATA }, resource: TMU0_RESOURCES.as_mut_ptr(), num_resources: 4 };
static mut TMU1_PLATFORM_DATA: sh_timer_config = sh_timer_config { channels_mask: 7 };
static mut TMU1_RESOURCES: [resource; 4] = [DEFINE_RES_MEM(0xffc20000, 0x2c), DEFINE_RES_IRQ(evt2irq(0x460)), DEFINE_RES_IRQ(evt2irq(0x480)), DEFINE_RES_IRQ(evt2irq(0x4a0))];
static mut TMU1_DEVICE: platform_device = platform_device { name: "sh-tmu", id: 1, dev: device { platform_data: &raw mut TMU1_PLATFORM_DATA }, resource: TMU1_RESOURCES.as_mut_ptr(), num_resources: 4 };

static mut SHX3_EARLY_DEVICES: [*mut platform_device; 5] = [&raw mut SCIF0_DEVICE, &raw mut SCIF1_DEVICE, &raw mut SCIF2_DEVICE, &raw mut TMU0_DEVICE, &raw mut TMU1_DEVICE];

unsafe fn shx3_devices_setup() -> i32 { platform_add_devices(SHX3_EARLY_DEVICES.as_mut_ptr(), SHX3_EARLY_DEVICES.len()) }
// arch_initcall(shx3_devices_setup)
unsafe fn plat_early_device_setup() { sh_early_platform_add_devices(SHX3_EARLY_DEVICES.as_mut_ptr(), SHX3_EARLY_DEVICES.len()); }

#[repr(u32)]
enum InterruptSource {
    UNUSED = 0,
    IRL_LLLL, IRL_LLLH, IRL_LLHL, IRL_LLHH, IRL_LHLL, IRL_LHLH, IRL_LHHL, IRL_LHHH,
    IRL_HLLL, IRL_HLLH, IRL_HLHL, IRL_HLHH, IRL_HHLL, IRL_HHLH, IRL_HHHL,
    IRQ0, IRQ1, IRQ2, IRQ3, HUDII,
    TMU0, TMU1, TMU2, TMU3, TMU4, TMU5,
    PCII0, PCII1, PCII2, PCII3, PCII4, PCII5, PCII6, PCII7, PCII8, PCII9,
    SCIF0_ERI, SCIF0_RXI, SCIF0_BRI, SCIF0_TXI, SCIF1_ERI, SCIF1_RXI, SCIF1_BRI, SCIF1_TXI,
    SCIF2_ERI, SCIF2_RXI, SCIF2_BRI, SCIF2_TXI, SCIF3_ERI, SCIF3_RXI, SCIF3_BRI, SCIF3_TXI,
    DMAC0_DMINT0, DMAC0_DMINT1, DMAC0_DMINT2, DMAC0_DMINT3, DMAC0_DMINT4, DMAC0_DMINT5, DMAC0_DMAE, DU,
    DMAC1_DMINT6, DMAC1_DMINT7, DMAC1_DMINT8, DMAC1_DMINT9, DMAC1_DMINT10, DMAC1_DMINT11, DMAC1_DMAE,
    IIC, VIN0, VIN1, VCORE0, ATAPI, DTU0, DTU1, DTU2, DTU3, FE0, FE1, GPIO0, GPIO1, GPIO2, GPIO3, PAM, IRM,
    INTICI0, INTICI1, INTICI2, INTICI3, INTICI4, INTICI5, INTICI6, INTICI7,
    IRL, PCII56789, SCIF0, SCIF1, SCIF2, SCIF3, DMAC0, DMAC1,
}

static mut VECTORS: [intc_vect; 91] = [
    INTC_VECT(HUDII,0x3e0), INTC_VECT(TMU0,0x400), INTC_VECT(TMU1,0x420), INTC_VECT(TMU2,0x440), INTC_VECT(TMU3,0x460), INTC_VECT(TMU4,0x480), INTC_VECT(TMU5,0x4a0),
    INTC_VECT(PCII0,0x500), INTC_VECT(PCII1,0x520), INTC_VECT(PCII2,0x540), INTC_VECT(PCII3,0x560), INTC_VECT(PCII4,0x580), INTC_VECT(PCII5,0x5a0), INTC_VECT(PCII6,0x5c0), INTC_VECT(PCII7,0x5e0), INTC_VECT(PCII8,0x600), INTC_VECT(PCII9,0x620),
    INTC_VECT(SCIF0_ERI,0x700), INTC_VECT(SCIF0_RXI,0x720), INTC_VECT(SCIF0_BRI,0x740), INTC_VECT(SCIF0_TXI,0x760), INTC_VECT(SCIF1_ERI,0x780), INTC_VECT(SCIF1_RXI,0x7a0), INTC_VECT(SCIF1_BRI,0x7c0), INTC_VECT(SCIF1_TXI,0x7e0),
    INTC_VECT(SCIF3_ERI,0x880), INTC_VECT(SCIF3_RXI,0x8a0), INTC_VECT(SCIF3_BRI,0x8c0), INTC_VECT(SCIF3_TXI,0x8e0),
    INTC_VECT(DMAC0_DMINT0,0x900), INTC_VECT(DMAC0_DMINT1,0x920), INTC_VECT(DMAC0_DMINT2,0x940), INTC_VECT(DMAC0_DMINT3,0x960), INTC_VECT(DMAC0_DMINT4,0x980), INTC_VECT(DMAC0_DMINT5,0x9a0), INTC_VECT(DMAC0_DMAE,0x9c0), INTC_VECT(DU,0x9e0),
    INTC_VECT(DMAC1_DMINT6,0xa00), INTC_VECT(DMAC1_DMINT7,0xa20), INTC_VECT(DMAC1_DMINT8,0xa40), INTC_VECT(DMAC1_DMINT9,0xa60), INTC_VECT(DMAC1_DMINT10,0xa80), INTC_VECT(DMAC1_DMINT11,0xaa0), INTC_VECT(DMAC1_DMAE,0xac0), INTC_VECT(IIC,0xae0), INTC_VECT(VIN0,0xb00), INTC_VECT(VIN1,0xb20), INTC_VECT(VCORE0,0xb00), INTC_VECT(ATAPI,0xb60),
    INTC_VECT(DTU0,0xc00), INTC_VECT(DTU0,0xc20), INTC_VECT(DTU0,0xc40), INTC_VECT(DTU1,0xc60), INTC_VECT(DTU1,0xc80), INTC_VECT(DTU1,0xca0), INTC_VECT(DTU2,0xcc0), INTC_VECT(DTU2,0xce0), INTC_VECT(DTU2,0xd00), INTC_VECT(DTU3,0xd20), INTC_VECT(DTU3,0xd40), INTC_VECT(DTU3,0xd60),
    INTC_VECT(FE0,0xe00), INTC_VECT(FE1,0xe20), INTC_VECT(GPIO0,0xe40), INTC_VECT(GPIO1,0xe60), INTC_VECT(GPIO2,0xe80), INTC_VECT(GPIO3,0xea0), INTC_VECT(PAM,0xec0), INTC_VECT(IRM,0xee0), INTC_VECT(INTICI0,0xf00), INTC_VECT(INTICI1,0xf20), INTC_VECT(INTICI2,0xf40), INTC_VECT(INTICI3,0xf60), INTC_VECT(INTICI4,0xf80), INTC_VECT(INTICI5,0xfa0), INTC_VECT(INTICI6,0xfc0), INTC_VECT(INTICI7,0xfe0),
];

// The remaining INTC tables retain the original kernel macro/data form.
static mut GROUPS: [intc_group; 7] = [
    INTC_GROUP(IRL, IRL_LLLL, IRL_LLLH, IRL_LLHL, IRL_LLHH, IRL_LHLL, IRL_LHLH, IRL_LHHL, IRL_LHHH, IRL_HLLL, IRL_HLLH, IRL_HLHL, IRL_HLHH, IRL_HHLL, IRL_HHLH, IRL_HHHL),
    INTC_GROUP(PCII56789, PCII5, PCII6, PCII7, PCII8, PCII9), INTC_GROUP(SCIF0, SCIF0_ERI, SCIF0_RXI, SCIF0_BRI, SCIF0_TXI), INTC_GROUP(SCIF1, SCIF1_ERI, SCIF1_RXI, SCIF1_BRI, SCIF1_TXI), INTC_GROUP(SCIF3, SCIF3_ERI, SCIF3_RXI, SCIF3_BRI, SCIF3_TXI),
    INTC_GROUP(DMAC0, DMAC0_DMINT0, DMAC0_DMINT1, DMAC0_DMINT2, DMAC0_DMINT3, DMAC0_DMINT4, DMAC0_DMINT5, DMAC0_DMAE), INTC_GROUP(DMAC1, DMAC1_DMINT6, DMAC1_DMINT7, DMAC1_DMINT8, DMAC1_DMINT9, DMAC1_DMINT10, DMAC1_DMINT11),
];

const INT2DISTCR0: usize = 0xfe4108a0;
const INT2DISTCR1: usize = 0xfe4108a4;
const INT2DISTCR2: usize = 0xfe4108a8;
static mut MASK_REGISTERS: [intc_mask_reg; 5] = [
    intc_mask_reg { set: 0xfe410030, clr: 0xfe410050, width: 32, enum_: [IRQ0, IRQ1, IRQ2, IRQ3], balance: None },
    intc_mask_reg { set: 0xfe410040, clr: 0xfe410060, width: 32, enum_: [IRL], balance: None },
    intc_mask_reg { set: 0xfe410820, clr: 0xfe410850, width: 32, enum_: [FE1, FE0, 0, ATAPI, VCORE0, VIN1, VIN0, IIC, DU, GPIO3, GPIO2, GPIO1, GPIO0, PAM, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, TMU5, TMU4, TMU3, TMU2, TMU1, TMU0, 0], balance: INTC_SMP_BALANCING(INT2DISTCR0) },
    intc_mask_reg { set: 0xfe410830, clr: 0xfe410860, width: 32, enum_: [0, 0, 0, 0, DTU3, DTU2, DTU1, DTU0, 0, PCII9, PCII8, PCII7, PCII6, PCII5, PCII4, PCII3, PCII2, PCII1, PCII0, DMAC1_DMAE, DMAC1_DMINT11, DMAC1_DMINT10, DMAC1_DMINT9, DMAC1_DMINT8, DMAC1_DMINT7, DMAC1_DMINT6, DMAC0_DMAE, DMAC0_DMINT5, DMAC0_DMINT4, DMAC0_DMINT3, DMAC0_DMINT2, DMAC0_DMINT1, DMAC0_DMINT0], balance: INTC_SMP_BALANCING(INT2DISTCR1) },
    intc_mask_reg { set: 0xfe410840, clr: 0xfe410870, width: 32, enum_: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, SCIF3_TXI, SCIF3_BRI, SCIF3_RXI, SCIF3_ERI, SCIF2_TXI, SCIF2_BRI, SCIF2_RXI, SCIF2_ERI, SCIF1_TXI, SCIF1_BRI, SCIF1_RXI, SCIF1_ERI, SCIF0_TXI, SCIF0_BRI, SCIF0_RXI, SCIF0_ERI], balance: INTC_SMP_BALANCING(INT2DISTCR2) },
];
static mut PRIO_REGISTERS: [intc_prio_reg; 6] = [
    intc_prio_reg { set: 0xfe410010, clr: 0, width: 32, shift: 4, enum_: [IRQ0, IRQ1, IRQ2, IRQ3], balance: None },
    intc_prio_reg { set: 0xfe410800, clr: 0, width: 32, shift: 4, enum_: [0, HUDII, TMU5, TMU4, TMU3, TMU2, TMU1, TMU0], balance: None },
    intc_prio_reg { set: 0xfe410804, clr: 0, width: 32, shift: 4, enum_: [DTU3, DTU2, DTU1, DTU0, SCIF3, SCIF2, SCIF1, SCIF0], balance: None },
    intc_prio_reg { set: 0xfe410808, clr: 0, width: 32, shift: 4, enum_: [DMAC1, DMAC0, PCII56789, PCII4, PCII3, PCII2, PCII1, PCII0], balance: None },
    intc_prio_reg { set: 0xfe41080c, clr: 0, width: 32, shift: 4, enum_: [FE1, FE0, ATAPI, VCORE0, VIN1, VIN0, IIC, DU], balance: None },
    intc_prio_reg { set: 0xfe410810, clr: 0, width: 32, shift: 4, enum_: [0, 0, PAM, GPIO3, GPIO2, GPIO1, GPIO0, IRM], balance: INTC_SMP(4, 4) },
];
static mut SENSE_REGISTERS: [intc_sense_reg; 1] = [intc_sense_reg { addr: 0xfe41001c, width: 32, shift: 2, enum_: [IRQ0, IRQ1, IRQ2, IRQ3] }];
static mut VECTORS_IRQ: [intc_vect; 4] = [INTC_VECT(IRQ0,0x240), INTC_VECT(IRQ1,0x280), INTC_VECT(IRQ2,0x2c0), INTC_VECT(IRQ3,0x300)];
static mut VECTORS_IRL: [intc_vect; 15] = [INTC_VECT(IRL_LLLL,0x200), INTC_VECT(IRL_LLLH,0x220), INTC_VECT(IRL_LLHL,0x240), INTC_VECT(IRL_LLHH,0x260), INTC_VECT(IRL_LHLL,0x280), INTC_VECT(IRL_LHLH,0x2a0), INTC_VECT(IRL_LHHL,0x2c0), INTC_VECT(IRL_LHHH,0x2e0), INTC_VECT(IRL_HLLL,0x300), INTC_VECT(IRL_HLLH,0x320), INTC_VECT(IRL_HLHL,0x340), INTC_VECT(IRL_HLHH,0x360), INTC_VECT(IRL_HHLL,0x380), INTC_VECT(IRL_HHLH,0x3a0), INTC_VECT(IRL_HHHL,0x3c0)];
static mut INTC_DESC: intc_desc = DECLARE_INTC_DESC("shx3", VECTORS, GROUPS, MASK_REGISTERS, PRIO_REGISTERS, None);
static mut INTC_DESC_IRQ: intc_desc = DECLARE_INTC_DESC("shx3-irq", VECTORS_IRQ, GROUPS, MASK_REGISTERS, PRIO_REGISTERS, SENSE_REGISTERS);
static mut INTC_DESC_IRL: intc_desc = DECLARE_INTC_DESC("shx3-irl", VECTORS_IRL, GROUPS, MASK_REGISTERS, PRIO_REGISTERS, None);

unsafe fn plat_irq_setup_pins(mode: i32) {
    let mut ret = 0;
    match mode {
        IRQ_MODE_IRQ => { ret |= gpio_request(GPIO_FN_IRQ3, INTC_DESC_IRQ.name); ret |= gpio_request(GPIO_FN_IRQ2, INTC_DESC_IRQ.name); ret |= gpio_request(GPIO_FN_IRQ1, INTC_DESC_IRQ.name); ret |= gpio_request(GPIO_FN_IRQ0, INTC_DESC_IRQ.name); if ret != 0 { pr_err!("Failed to set IRQ mode\n"); return; } register_intc_controller(&raw mut INTC_DESC_IRQ); }
        IRQ_MODE_IRL3210 => { ret |= gpio_request(GPIO_FN_IRL3, INTC_DESC_IRL.name); ret |= gpio_request(GPIO_FN_IRL2, INTC_DESC_IRL.name); ret |= gpio_request(GPIO_FN_IRL1, INTC_DESC_IRL.name); ret |= gpio_request(GPIO_FN_IRL0, INTC_DESC_IRL.name); if ret != 0 { pr_err!("Failed to set IRL mode\n"); return; } register_intc_controller(&raw mut INTC_DESC_IRL); }
        _ => BUG!(),
    }
}
unsafe fn plat_irq_setup() { register_intc_controller(&raw mut INTC_DESC); }
unsafe fn plat_mem_setup() { let mut nid: u32 = 1; setup_bootmem_node(nid, 0x145f0000, 0x14610000); nid += 1; /* CPU1-CPU3 nodes intentionally disabled in C (#if 0). */ setup_bootmem_node(nid, 0x16000000, 0x16020000); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
