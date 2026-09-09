// SPDX-License-Identifier: GPL-2.0
/*
 * SH7785 Setup
 *
 * Copyright (C) 2007 Paul Mundt
 */

// C dependencies are supplied by the surrounding kernel translation.

static mut scif0_platform_data: plat_sci_port = plat_sci_port {
    scscr: SCSCR_REIE | SCSCR_CKE1, type_: PORT_SCIF,
    regtype: SCIx_SH4_SCIF_FIFODATA_REGTYPE,
};
static mut scif0_resources: [resource; 2] = [
    resource_mem(0xffea0000, 0x100), resource_irq(evt2irq(0x700)),
];
static mut scif0_device: platform_device = platform_device { name: "sh-sci", id: 0,
    resource: scif0_resources.as_mut_ptr(), num_resources: ARRAY_SIZE(scif0_resources),
    dev: device { platform_data: &raw mut scif0_platform_data }, };

static mut scif1_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE | SCSCR_CKE1, type_: PORT_SCIF, regtype: SCIx_SH4_SCIF_FIFODATA_REGTYPE };
static mut scif1_resources: [resource; 2] = [resource_mem(0xffeb0000, 0x100), resource_irq(evt2irq(0x780))];
static mut scif1_device: platform_device = platform_device { name: "sh-sci", id: 1, resource: scif1_resources.as_mut_ptr(), num_resources: ARRAY_SIZE(scif1_resources), dev: device { platform_data: &raw mut scif1_platform_data } };
static mut scif2_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE | SCSCR_CKE1, type_: PORT_SCIF, regtype: SCIx_SH4_SCIF_FIFODATA_REGTYPE };
static mut scif2_resources: [resource; 2] = [resource_mem(0xffec0000, 0x100), resource_irq(evt2irq(0x980))];
static mut scif2_device: platform_device = platform_device { name: "sh-sci", id: 2, resource: scif2_resources.as_mut_ptr(), num_resources: ARRAY_SIZE(scif2_resources), dev: device { platform_data: &raw mut scif2_platform_data } };
static mut scif3_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE | SCSCR_CKE1, type_: PORT_SCIF, regtype: SCIx_SH4_SCIF_FIFODATA_REGTYPE };
static mut scif3_resources: [resource; 2] = [resource_mem(0xffed0000, 0x100), resource_irq(evt2irq(0x9a0))];
static mut scif3_device: platform_device = platform_device { name: "sh-sci", id: 3, resource: scif3_resources.as_mut_ptr(), num_resources: ARRAY_SIZE(scif3_resources), dev: device { platform_data: &raw mut scif3_platform_data } };
static mut scif4_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE | SCSCR_CKE1, type_: PORT_SCIF, regtype: SCIx_SH4_SCIF_FIFODATA_REGTYPE };
static mut scif4_resources: [resource; 2] = [resource_mem(0xffee0000, 0x100), resource_irq(evt2irq(0x9c0))];
static mut scif4_device: platform_device = platform_device { name: "sh-sci", id: 4, resource: scif4_resources.as_mut_ptr(), num_resources: ARRAY_SIZE(scif4_resources), dev: device { platform_data: &raw mut scif4_platform_data } };
static mut scif5_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE | SCSCR_CKE1, type_: PORT_SCIF, regtype: SCIx_SH4_SCIF_FIFODATA_REGTYPE };
static mut scif5_resources: [resource; 2] = [resource_mem(0xffef0000, 0x100), resource_irq(evt2irq(0x9e0))];
static mut scif5_device: platform_device = platform_device { name: "sh-sci", id: 5, resource: scif5_resources.as_mut_ptr(), num_resources: ARRAY_SIZE(scif5_resources), dev: device { platform_data: &raw mut scif5_platform_data } };

static mut tmu0_platform_data: sh_timer_config = sh_timer_config { channels_mask: 7 };
static mut tmu0_resources: [resource; 4] = [resource_mem(0xffd80000, 0x30), resource_irq(evt2irq(0x580)), resource_irq(evt2irq(0x5a0)), resource_irq(evt2irq(0x5c0))];
static mut tmu0_device: platform_device = platform_device { name: "sh-tmu", id: 0, resource: tmu0_resources.as_mut_ptr(), num_resources: ARRAY_SIZE(tmu0_resources), dev: device { platform_data: &raw mut tmu0_platform_data } };
static mut tmu1_platform_data: sh_timer_config = sh_timer_config { channels_mask: 7 };
static mut tmu1_resources: [resource; 4] = [resource_mem(0xffdc0000, 0x2c), resource_irq(evt2irq(0xe00)), resource_irq(evt2irq(0xe20)), resource_irq(evt2irq(0xe40))];
static mut tmu1_device: platform_device = platform_device { name: "sh-tmu", id: 1, resource: tmu1_resources.as_mut_ptr(), num_resources: ARRAY_SIZE(tmu1_resources), dev: device { platform_data: &raw mut tmu1_platform_data } };

/* DMA */
static sh7785_dmae0_channels: [sh_dmae_channel; 6] = [
    sh_dmae_channel { offset: 0, dmars: 0, dmars_bit: 0 }, sh_dmae_channel { offset: 0x10, dmars: 0, dmars_bit: 8 },
    sh_dmae_channel { offset: 0x20, dmars: 4, dmars_bit: 0 }, sh_dmae_channel { offset: 0x30, dmars: 4, dmars_bit: 8 },
    sh_dmae_channel { offset: 0x50, dmars: 8, dmars_bit: 0 }, sh_dmae_channel { offset: 0x60, dmars: 8, dmars_bit: 8 },
];
static sh7785_dmae1_channels: [sh_dmae_channel; 6] = [
    sh_dmae_channel { offset: 0 }, sh_dmae_channel { offset: 0x10 }, sh_dmae_channel { offset: 0x20 },
    sh_dmae_channel { offset: 0x30 }, sh_dmae_channel { offset: 0x50 }, sh_dmae_channel { offset: 0x60 },
];
static ts_shift: [unsigned int; ARRAY_SIZE(TS_SHIFT)] = TS_SHIFT;
static mut dma0_platform_data: sh_dmae_pdata = dma_pdata(&sh7785_dmae0_channels, ts_shift);
static mut dma1_platform_data: sh_dmae_pdata = dma_pdata(&sh7785_dmae1_channels, ts_shift);

static mut sh7785_dmae0_resources: [resource; 3] = [
    resource_named_mem(0xfc808020, 0xfc80808f, "Channel registers and DMAOR"),
    resource_named_mem(0xfc809000, 0xfc80900b, "DMARSx"),
    resource_named_irq("error_irq", evt2irq(0x620), IORESOURCE_IRQ | IORESOURCE_IRQ_SHAREABLE),
];
static mut sh7785_dmae1_resources: [resource; 2] = [
    resource_named_mem(0xfcc08020, 0xfcc0808f, "Channel registers and DMAOR"),
    resource_named_irq("error_irq", evt2irq(0x880), IORESOURCE_IRQ | IORESOURCE_IRQ_SHAREABLE),
];
static mut dma0_device: platform_device = dma_device("sh-dma-engine", 0, sh7785_dmae0_resources.as_mut_ptr(), &raw mut dma0_platform_data);
static mut dma1_device: platform_device = dma_device("sh-dma-engine", 1, sh7785_dmae1_resources.as_mut_ptr(), &raw mut dma1_platform_data);

static mut sh7785_devices: [*mut platform_device; 10] = [
    &raw mut scif0_device, &raw mut scif1_device, &raw mut scif2_device, &raw mut scif3_device,
    &raw mut scif4_device, &raw mut scif5_device, &raw mut tmu0_device, &raw mut tmu1_device,
    &raw mut dma0_device, &raw mut dma1_device,
];
unsafe fn sh7785_devices_setup() -> i32 { platform_add_devices(sh7785_devices.as_mut_ptr(), ARRAY_SIZE(sh7785_devices)) }
arch_initcall!(sh7785_devices_setup);
static mut sh7785_early_devices: [*mut platform_device; 8] = [
    &raw mut scif0_device, &raw mut scif1_device, &raw mut scif2_device, &raw mut scif3_device,
    &raw mut scif4_device, &raw mut scif5_device, &raw mut tmu0_device, &raw mut tmu1_device,
];
unsafe fn plat_early_device_setup() { sh_early_platform_add_devices(sh7785_early_devices.as_mut_ptr(), ARRAY_SIZE(sh7785_early_devices)); }

enum { UNUSED = 0, IRL0_LLLL, IRL0_LLLH, IRL0_LLHL, IRL0_LLHH, IRL0_LHLL, IRL0_LHLH, IRL0_LHHL, IRL0_LHHH, IRL0_HLLL, IRL0_HLLH, IRL0_HLHL, IRL0_HLHH, IRL0_HHLL, IRL0_HHLH, IRL0_HHHL, IRL4_LLLL, IRL4_LLLH, IRL4_LLHL, IRL4_LLHH, IRL4_LHLL, IRL4_LHLH, IRL4_LHHL, IRL4_LHHH, IRL4_HLLL, IRL4_HLLH, IRL4_HLHL, IRL4_HLHH, IRL4_HHLL, IRL4_HHLH, IRL4_HHHL, IRQ0, IRQ1, IRQ2, IRQ3, IRQ4, IRQ5, IRQ6, IRQ7, WDT, TMU0, TMU1, TMU2, TMU2_TICPI, HUDI, DMAC0, SCIF0, SCIF1, DMAC1, HSPI, SCIF2, SCIF3, SCIF4, SCIF5, PCISERR, PCIINTA, PCIINTB, PCIINTC, PCIINTD, PCIC5, SIOF, MMCIF, DU, GDTA, TMU3, TMU4, TMU5, SSI0, SSI1, HAC0, HAC1, FLCTL, GPIO, TMU012, TMU345 }

static mut vectors: [intc_vect; 73] = [
    intc_vect!(WDT,0x560), intc_vect!(TMU0,0x580), intc_vect!(TMU1,0x5a0), intc_vect!(TMU2,0x5c0), intc_vect!(TMU2_TICPI,0x5e0), intc_vect!(HUDI,0x600),
    intc_vect!(DMAC0,0x620), intc_vect!(DMAC0,0x640), intc_vect!(DMAC0,0x660), intc_vect!(DMAC0,0x680), intc_vect!(DMAC0,0x6a0), intc_vect!(DMAC0,0x6c0), intc_vect!(DMAC0,0x6e0),
    intc_vect!(SCIF0,0x700), intc_vect!(SCIF0,0x720), intc_vect!(SCIF0,0x740), intc_vect!(SCIF0,0x760), intc_vect!(SCIF1,0x780), intc_vect!(SCIF1,0x7a0), intc_vect!(SCIF1,0x7c0), intc_vect!(SCIF1,0x7e0),
    intc_vect!(DMAC1,0x880), intc_vect!(DMAC1,0x8a0), intc_vect!(DMAC1,0x8c0), intc_vect!(DMAC1,0x8e0), intc_vect!(DMAC1,0x900), intc_vect!(DMAC1,0x920), intc_vect!(DMAC1,0x940), intc_vect!(HSPI,0x960),
    intc_vect!(SCIF2,0x980), intc_vect!(SCIF3,0x9a0), intc_vect!(SCIF4,0x9c0), intc_vect!(SCIF5,0x9e0), intc_vect!(PCISERR,0xa00), intc_vect!(PCIINTA,0xa20), intc_vect!(PCIINTB,0xa40), intc_vect!(PCIINTC,0xa60), intc_vect!(PCIINTD,0xa80), intc_vect!(PCIC5,0xaa0), intc_vect!(PCIC5,0xac0), intc_vect!(PCIC5,0xae0), intc_vect!(PCIC5,0xb00), intc_vect!(PCIC5,0xb20),
    intc_vect!(SIOF,0xc00), intc_vect!(MMCIF,0xd00), intc_vect!(MMCIF,0xd20), intc_vect!(MMCIF,0xd40), intc_vect!(MMCIF,0xd60), intc_vect!(DU,0xd80), intc_vect!(GDTA,0xda0), intc_vect!(GDTA,0xdc0), intc_vect!(GDTA,0xde0), intc_vect!(TMU3,0xe00), intc_vect!(TMU4,0xe20), intc_vect!(TMU5,0xe40), intc_vect!(SSI0,0xe80), intc_vect!(SSI1,0xea0), intc_vect!(HAC0,0xec0), intc_vect!(HAC1,0xee0), intc_vect!(FLCTL,0xf00), intc_vect!(FLCTL,0xf20), intc_vect!(FLCTL,0xf40), intc_vect!(FLCTL,0xf60), intc_vect!(GPIO,0xf80), intc_vect!(GPIO,0xfa0), intc_vect!(GPIO,0xfc0), intc_vect!(GPIO,0xfe0),
];
static mut groups: [intc_group; 2] = [intc_group!(TMU012,TMU0,TMU1,TMU2,TMU2_TICPI), intc_group!(TMU345,TMU3,TMU4,TMU5)];
static mut mask_registers: [intc_mask_reg; 3] = [
    intc_mask_reg!(0xffd00044,0xffd00064,32,[IRQ0,IRQ1,IRQ2,IRQ3,IRQ4,IRQ5,IRQ6,IRQ7]),
    intc_mask_reg!(0xffd40080,0xffd40084,32,[IRL0_LLLL,IRL0_LLLH,IRL0_LLHL,IRL0_LLHH,IRL0_LHLL,IRL0_LHLH,IRL0_LHHL,IRL0_LHHH,IRL0_HLLL,IRL0_HLLH,IRL0_HLHL,IRL0_HLHH,IRL0_HHLL,IRL0_HHLH,IRL0_HHHL,0,IRL4_LLLL,IRL4_LLLH,IRL4_LLHL,IRL4_LLHH,IRL4_LHLL,IRL4_LHLH,IRL4_LHHL,IRL4_LHHH,IRL4_HLLL,IRL4_HLLH,IRL4_HLHL,IRL4_HLHH,IRL4_HHLL,IRL4_HHLH,IRL4_HHHL,0]),
    intc_mask_reg!(0xffd40038,0xffd4003c,32,[0,0,0,GDTA,DU,SSI0,SSI1,GPIO,FLCTL,MMCIF,HSPI,SIOF,PCIC5,PCIINTD,PCIINTC,PCIINTB,PCIINTA,PCISERR,HAC1,HAC0,DMAC1,DMAC0,HUDI,WDT,SCIF5,SCIF4,SCIF3,SCIF2,SCIF1,SCIF0,TMU345,TMU012]),
];
static mut prio_registers: [intc_prio_reg; 11] = intc_prio_registers!();
static mut intc_desc: intc_desc = declare_intc_desc!("sh7785", vectors, groups, mask_registers, prio_registers, None);
static mut vectors_irq0123: [intc_vect; 4] = [intc_vect!(IRQ0, 0x240), intc_vect!(IRQ1, 0x280), intc_vect!(IRQ2, 0x2c0), intc_vect!(IRQ3, 0x300)];
static mut vectors_irq4567: [intc_vect; 4] = [intc_vect!(IRQ4, 0x340), intc_vect!(IRQ5, 0x380), intc_vect!(IRQ6, 0x3c0), intc_vect!(IRQ7, 0x200)];
static mut sense_registers: [intc_sense_reg; 1] = [intc_sense_reg!(0xffd0001c, 32, 2, [IRQ0, IRQ1, IRQ2, IRQ3, IRQ4, IRQ5, IRQ6, IRQ7])];
static mut ack_registers: [intc_mask_reg; 1] = [intc_mask_reg!(0xffd00024, 0, 32, [IRQ0, IRQ1, IRQ2, IRQ3, IRQ4, IRQ5, IRQ6, IRQ7])];
static mut intc_desc_irq0123: intc_desc = declare_intc_desc_ack!("sh7785-irq0123", vectors_irq0123, mask_registers, prio_registers, sense_registers, ack_registers);
static mut intc_desc_irq4567: intc_desc = declare_intc_desc_ack!("sh7785-irq4567", vectors_irq4567, mask_registers, prio_registers, sense_registers, ack_registers);
static mut vectors_irl0123: [intc_vect; 15] = intc_irl_vectors!(IRL0, 0x200);
static mut vectors_irl4567: [intc_vect; 15] = intc_irl_vectors!(IRL4, 0xb00);
static mut intc_desc_irl0123: intc_desc = declare_intc_desc!("sh7785-irl0123", vectors_irl0123, mask_registers, None, None);
static mut intc_desc_irl4567: intc_desc = declare_intc_desc!("sh7785-irl4567", vectors_irl4567, mask_registers, None, None);

const INTC_ICR0: usize = 0xffd00000;
const INTC_INTMSK0: usize = 0xffd00044;
const INTC_INTMSK1: usize = 0xffd00048;
const INTC_INTMSK2: usize = 0xffd40080;
const INTC_INTMSKCLR1: usize = 0xffd00068;
const INTC_INTMSKCLR2: usize = 0xffd40084;

unsafe fn plat_irq_setup() {
    __raw_writel(0xff000000, INTC_INTMSK0); __raw_writel(0xc0000000, INTC_INTMSK1); __raw_writel(0xfffefffe, INTC_INTMSK2);
    __raw_writel(__raw_readl(INTC_ICR0) & !0x00c00000, INTC_ICR0);
    __raw_writel(__raw_readl(INTC_ICR0) | 0x00200000, INTC_ICR0); register_intc_controller(&raw mut intc_desc);
}
unsafe fn plat_irq_setup_pins(mode: i32) {
    match mode {
        IRQ_MODE_IRQ7654 => { __raw_writel(__raw_readl(INTC_ICR0) | 0x00400000, INTC_ICR0); register_intc_controller(&raw mut intc_desc_irq4567); }
        IRQ_MODE_IRQ3210 => { __raw_writel(__raw_readl(INTC_ICR0) | 0x00800000, INTC_ICR0); register_intc_controller(&raw mut intc_desc_irq0123); }
        IRQ_MODE_IRL7654 => { __raw_writel(0x40000000, INTC_INTMSKCLR1); __raw_writel(0x0000fffe, INTC_INTMSKCLR2); }
        IRQ_MODE_IRL3210 => { __raw_writel(0x80000000, INTC_INTMSKCLR1); __raw_writel(0xfffe0000, INTC_INTMSKCLR2); }
        IRQ_MODE_IRL7654_MASK => { __raw_writel(0x40000000, INTC_INTMSKCLR1); register_intc_controller(&raw mut intc_desc_irl4567); }
        IRQ_MODE_IRL3210_MASK => { __raw_writel(0x80000000, INTC_INTMSKCLR1); register_intc_controller(&raw mut intc_desc_irl0123); }
        _ => BUG!(),
    }
}
unsafe fn plat_mem_setup() { setup_bootmem_node(1, 0xe55f0000, 0xe5610000); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
