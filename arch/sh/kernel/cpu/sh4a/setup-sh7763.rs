// SPDX-License-Identifier: GPL-2.0
/*
 * SH7763 Setup
 *
 *  Copyright (C) 2006  Paul Mundt
 *  Copyright (C) 2007  Yoshihiro Shimoda
 *  Copyright (C) 2008, 2009  Nobuhiro Iwamatsu
 */
// Linux dependencies supplied externally.

static mut scif0_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, type_: PORT_SCIF, regtype: SCIx_SH4_SCIF_FIFODATA_REGTYPE };
static mut scif0_resources: [resource; 2] = [DEFINE_RES_MEM!(0xffe00000, 0x100), DEFINE_RES_IRQ!(evt2irq(0x700))];
static mut scif0_device: platform_device = platform_device { name: "sh-sci", id: 0, resource: scif0_resources.as_mut_ptr(), num_resources: ARRAY_SIZE!(scif0_resources), dev: device { platform_data: &raw mut scif0_platform_data, ..device::default() }, ..platform_device::default() };

static mut scif1_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, type_: PORT_SCIF, regtype: SCIx_SH4_SCIF_FIFODATA_REGTYPE };
static mut scif1_resources: [resource; 2] = [DEFINE_RES_MEM!(0xffe08000, 0x100), DEFINE_RES_IRQ!(evt2irq(0xb80))];
static mut scif1_device: platform_device = platform_device { name: "sh-sci", id: 1, resource: scif1_resources.as_mut_ptr(), num_resources: ARRAY_SIZE!(scif1_resources), dev: device { platform_data: &raw mut scif1_platform_data, ..device::default() }, ..platform_device::default() };

static mut scif2_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, type_: PORT_SCIF, regtype: SCIx_SH4_SCIF_FIFODATA_REGTYPE };
static mut scif2_resources: [resource; 2] = [DEFINE_RES_MEM!(0xffe10000, 0x100), DEFINE_RES_IRQ!(evt2irq(0xf00))];
static mut scif2_device: platform_device = platform_device { name: "sh-sci", id: 2, resource: scif2_resources.as_mut_ptr(), num_resources: ARRAY_SIZE!(scif2_resources), dev: device { platform_data: &raw mut scif2_platform_data, ..device::default() }, ..platform_device::default() };

static mut rtc_resources: [resource; 2] = [resource { start: 0xffe80000, end: 0xffe80000 + 0x58 - 1, flags: IORESOURCE_IO }, resource { start: evt2irq(0x480), end: 0, flags: IORESOURCE_IRQ }];
static mut rtc_device: platform_device = platform_device { name: "sh-rtc", id: -1, num_resources: ARRAY_SIZE!(rtc_resources), resource: rtc_resources.as_mut_ptr(), ..platform_device::default() };

static mut usb_ohci_resources: [resource; 2] = [resource { start: 0xffec8000, end: 0xffec80ff, flags: IORESOURCE_MEM }, resource { start: evt2irq(0xc60), end: evt2irq(0xc60), flags: IORESOURCE_IRQ }];
static mut usb_ohci_dma_mask: u64 = 0xffffffffu64;
static mut usb_ohci_pdata: usb_ohci_pdata = usb_ohci_pdata::default();
static mut usb_ohci_device: platform_device = platform_device { name: "ohci-platform", id: -1, dev: device { dma_mask: &raw mut usb_ohci_dma_mask, coherent_dma_mask: 0xffffffff, platform_data: &raw mut usb_ohci_pdata, ..device::default() }, num_resources: ARRAY_SIZE!(usb_ohci_resources), resource: usb_ohci_resources.as_mut_ptr(), ..platform_device::default() };

static mut usbf_resources: [resource; 2] = [resource { start: 0xffec0000, end: 0xffec00ff, flags: IORESOURCE_MEM }, resource { start: evt2irq(0xc80), end: evt2irq(0xc80), flags: IORESOURCE_IRQ }];
static mut usbf_device: platform_device = platform_device { name: "sh_udc", id: -1, dev: device { dma_mask: core::ptr::null_mut(), coherent_dma_mask: 0xffffffff, ..device::default() }, num_resources: ARRAY_SIZE!(usbf_resources), resource: usbf_resources.as_mut_ptr(), ..platform_device::default() };

static mut tmu0_platform_data: sh_timer_config = sh_timer_config { channels_mask: 7 };
static mut tmu0_resources: [resource; 4] = [DEFINE_RES_MEM!(0xffd80000, 0x30), DEFINE_RES_IRQ!(evt2irq(0x580)), DEFINE_RES_IRQ!(evt2irq(0x5a0)), DEFINE_RES_IRQ!(evt2irq(0x5c0))];
static mut tmu0_device: platform_device = platform_device { name: "sh-tmu", id: 0, dev: device { platform_data: &raw mut tmu0_platform_data, ..device::default() }, resource: tmu0_resources.as_mut_ptr(), num_resources: ARRAY_SIZE!(tmu0_resources), ..platform_device::default() };
static mut tmu1_platform_data: sh_timer_config = sh_timer_config { channels_mask: 7 };
static mut tmu1_resources: [resource; 4] = [DEFINE_RES_MEM!(0xffd88000, 0x2c), DEFINE_RES_IRQ!(evt2irq(0xe00)), DEFINE_RES_IRQ!(evt2irq(0xe20)), DEFINE_RES_IRQ!(evt2irq(0xe40))];
static mut tmu1_device: platform_device = platform_device { name: "sh-tmu", id: 1, dev: device { platform_data: &raw mut tmu1_platform_data, ..device::default() }, resource: tmu1_resources.as_mut_ptr(), num_resources: ARRAY_SIZE!(tmu1_resources), ..platform_device::default() };

static mut sh7763_devices: [*mut platform_device; 8] = [&raw mut scif0_device, &raw mut scif1_device, &raw mut scif2_device, &raw mut tmu0_device, &raw mut tmu1_device, &raw mut rtc_device, &raw mut usb_ohci_device, &raw mut usbf_device];
unsafe fn sh7763_devices_setup() -> c_int { platform_add_devices(sh7763_devices.as_mut_ptr(), ARRAY_SIZE!(sh7763_devices)) }
arch_initcall!(sh7763_devices_setup);
static mut sh7763_early_devices: [*mut platform_device; 5] = [&raw mut scif0_device, &raw mut scif1_device, &raw mut scif2_device, &raw mut tmu0_device, &raw mut tmu1_device];
unsafe fn plat_early_device_setup() { sh_early_platform_add_devices(sh7763_early_devices.as_mut_ptr(), ARRAY_SIZE!(sh7763_early_devices)); }

enum interrupt_source {
    UNUSED = 0,
    IRL_LLLL, IRL_LLLH, IRL_LLHL, IRL_LLHH, IRL_LHLL, IRL_LHLH, IRL_LHHL, IRL_LHHH,
    IRL_HLLL, IRL_HLLH, IRL_HLHL, IRL_HLHH, IRL_HHLL, IRL_HHLH, IRL_HHHL,
    IRQ0, IRQ1, IRQ2, IRQ3, IRQ4, IRQ5, IRQ6, IRQ7, RTC, WDT, TMU0, TMU1, TMU2, TMU2_TICPI,
    HUDI, LCDC, DMAC, SCIF0, IIC0, IIC1, CMT, GETHER, HAC, PCISERR, PCIINTA, PCIINTB, PCIINTC, PCIINTD, PCIC5,
    STIF0, STIF1, SCIF1, SIOF0, SIOF1, SIOF2, USBH, USBF, TPU, PCC, MMCIF, SIM, TMU3, TMU4, TMU5, ADC, SSI0, SSI1, SSI2, SSI3, SCIF2, GPIO, TMU012, TMU345,
}

static mut vectors: [intc_vect; 67] = [
    INTC_VECT!(RTC,0x480), INTC_VECT!(RTC,0x4a0), INTC_VECT!(RTC,0x4c0), INTC_VECT!(WDT,0x560), INTC_VECT!(TMU0,0x580), INTC_VECT!(TMU1,0x5a0), INTC_VECT!(TMU2,0x5c0), INTC_VECT!(TMU2_TICPI,0x5e0), INTC_VECT!(HUDI,0x600), INTC_VECT!(LCDC,0x620), INTC_VECT!(DMAC,0x640), INTC_VECT!(DMAC,0x660), INTC_VECT!(DMAC,0x680), INTC_VECT!(DMAC,0x6a0), INTC_VECT!(DMAC,0x6c0), INTC_VECT!(SCIF0,0x700), INTC_VECT!(SCIF0,0x720), INTC_VECT!(SCIF0,0x740), INTC_VECT!(SCIF0,0x760), INTC_VECT!(DMAC,0x780), INTC_VECT!(DMAC,0x7a0), INTC_VECT!(IIC0,0x8a0), INTC_VECT!(IIC1,0x8c0), INTC_VECT!(CMT,0x900), INTC_VECT!(GETHER,0x920), INTC_VECT!(GETHER,0x940), INTC_VECT!(GETHER,0x960), INTC_VECT!(HAC,0x980), INTC_VECT!(PCISERR,0xa00), INTC_VECT!(PCIINTA,0xa20), INTC_VECT!(PCIINTB,0xa40), INTC_VECT!(PCIINTC,0xa60), INTC_VECT!(PCIINTD,0xa80), INTC_VECT!(PCIC5,0xaa0), INTC_VECT!(PCIC5,0xac0), INTC_VECT!(PCIC5,0xae0), INTC_VECT!(PCIC5,0xb00), INTC_VECT!(PCIC5,0xb20), INTC_VECT!(STIF0,0xb40), INTC_VECT!(STIF1,0xb60), INTC_VECT!(SCIF1,0xb80), INTC_VECT!(SCIF1,0xba0), INTC_VECT!(SCIF1,0xbc0), INTC_VECT!(SCIF1,0xbe0), INTC_VECT!(SIOF0,0xc00), INTC_VECT!(SIOF1,0xc20), INTC_VECT!(USBH,0xc60), INTC_VECT!(USBF,0xc80), INTC_VECT!(USBF,0xca0), INTC_VECT!(TPU,0xcc0), INTC_VECT!(PCC,0xce0), INTC_VECT!(MMCIF,0xd00), INTC_VECT!(MMCIF,0xd20), INTC_VECT!(MMCIF,0xd40), INTC_VECT!(MMCIF,0xd60), INTC_VECT!(SIM,0xd80), INTC_VECT!(SIM,0xda0), INTC_VECT!(SIM,0xdc0), INTC_VECT!(SIM,0xde0), INTC_VECT!(TMU3,0xe00), INTC_VECT!(TMU4,0xe20), INTC_VECT!(TMU5,0xe40), INTC_VECT!(ADC,0xe60), INTC_VECT!(SSI0,0xe80), INTC_VECT!(SSI1,0xea0), INTC_VECT!(SSI2,0xec0), INTC_VECT!(SSI3,0xee0), INTC_VECT!(SCIF2,0xf00), INTC_VECT!(SCIF2,0xf20), INTC_VECT!(SCIF2,0xf40), INTC_VECT!(SCIF2,0xf60), INTC_VECT!(GPIO,0xf80), INTC_VECT!(GPIO,0xfa0), INTC_VECT!(GPIO,0xfc0), INTC_VECT!(GPIO,0xfe0),
];
static mut groups: [intc_group; 2] = [INTC_GROUP!(TMU012,TMU0,TMU1,TMU2,TMU2_TICPI), INTC_GROUP!(TMU345,TMU3,TMU4,TMU5)];
static mut mask_registers: [intc_mask_reg; 2] = [INTC_MASK_REG!(0xffd40038,0xffd4003c,32,[0,0,0,0,0,0,GPIO,0,SSI0,MMCIF,0,SIOF0,PCIC5,PCIINTD,PCIINTC,PCIINTB,PCIINTA,PCISERR,HAC,CMT,0,0,0,DMAC,HUDI,0,WDT,SCIF1,SCIF0,RTC,TMU345,TMU012]), INTC_MASK_REG!(0xffd400d0,0xffd400d4,32,[0,0,0,0,0,0,SCIF2,USBF,0,0,STIF1,STIF0,0,0,USBH,GETHER,PCC,0,0,ADC,TPU,SIM,SIOF2,SIOF1,LCDC,0,IIC1,IIC0,SSI3,SSI2,SSI1,0])];
static mut prio_registers: [intc_prio_reg; 14] = [INTC_PRIO_REG!(0xffd40000,0,32,8,[TMU0,TMU1,TMU2,TMU2_TICPI]), INTC_PRIO_REG!(0xffd40004,0,32,8,[TMU3,TMU4,TMU5,RTC]), INTC_PRIO_REG!(0xffd40008,0,32,8,[SCIF0,SCIF1,WDT]), INTC_PRIO_REG!(0xffd4000c,0,32,8,[HUDI,DMAC,ADC]), INTC_PRIO_REG!(0xffd40010,0,32,8,[CMT,HAC,PCISERR,PCIINTA]), INTC_PRIO_REG!(0xffd40014,0,32,8,[PCIINTB,PCIINTC,PCIINTD,PCIC5]), INTC_PRIO_REG!(0xffd40018,0,32,8,[SIOF0,USBF,MMCIF,SSI0]), INTC_PRIO_REG!(0xffd4001c,0,32,8,[SCIF2,GPIO]), INTC_PRIO_REG!(0xffd400a0,0,32,8,[SSI3,SSI2,SSI1,0]), INTC_PRIO_REG!(0xffd400a4,0,32,8,[LCDC,0,IIC1,IIC0]), INTC_PRIO_REG!(0xffd400a8,0,32,8,[TPU,SIM,SIOF2,SIOF1]), INTC_PRIO_REG!(0xffd400ac,0,32,8,[PCC]), INTC_PRIO_REG!(0xffd400b0,0,32,8,[0,0,USBH,GETHER]), INTC_PRIO_REG!(0xffd400b4,0,32,8,[0,0,STIF1,STIF0])];
static intc_desc: intc_desc_type = DECLARE_INTC_DESC!("sh7763", vectors, groups, mask_registers, prio_registers, None);

static mut irq_vectors: [intc_vect; 8] = [INTC_VECT!(IRQ0,0x240),INTC_VECT!(IRQ1,0x280),INTC_VECT!(IRQ2,0x2c0),INTC_VECT!(IRQ3,0x300),INTC_VECT!(IRQ4,0x340),INTC_VECT!(IRQ5,0x380),INTC_VECT!(IRQ6,0x3c0),INTC_VECT!(IRQ7,0x200)];
static mut irq_mask_registers: [intc_mask_reg; 1] = [INTC_MASK_REG!(0xffd00044,0xffd00064,32,[IRQ0,IRQ1,IRQ2,IRQ3,IRQ4,IRQ5,IRQ6,IRQ7])];
static mut irq_prio_registers: [intc_prio_reg; 1] = [INTC_PRIO_REG!(0xffd00010,0,32,4,[IRQ0,IRQ1,IRQ2,IRQ3,IRQ4,IRQ5,IRQ6,IRQ7])];
static mut irq_sense_registers: [intc_sense_reg; 1] = [INTC_SENSE_REG!(0xffd0001c,32,2,[IRQ0,IRQ1,IRQ2,IRQ3,IRQ4,IRQ5,IRQ6,IRQ7])];
static mut irq_ack_registers: [intc_mask_reg; 1] = [INTC_MASK_REG!(0xffd00024,0,32,[IRQ0,IRQ1,IRQ2,IRQ3,IRQ4,IRQ5,IRQ6,IRQ7])];
static intc_irq_desc: intc_desc_type = DECLARE_INTC_DESC_ACK!("sh7763-irq",irq_vectors,None,irq_mask_registers,irq_prio_registers,irq_sense_registers,irq_ack_registers);

static mut irl_vectors: [intc_vect; 15] = [INTC_VECT!(IRL_LLLL,0x200),INTC_VECT!(IRL_LLLH,0x220),INTC_VECT!(IRL_LLHL,0x240),INTC_VECT!(IRL_LLHH,0x260),INTC_VECT!(IRL_LHLL,0x280),INTC_VECT!(IRL_LHLH,0x2a0),INTC_VECT!(IRL_LHHL,0x2c0),INTC_VECT!(IRL_LHHH,0x2e0),INTC_VECT!(IRL_HLLL,0x300),INTC_VECT!(IRL_HLLH,0x320),INTC_VECT!(IRL_HLHL,0x340),INTC_VECT!(IRL_HLHH,0x360),INTC_VECT!(IRL_HHLL,0x380),INTC_VECT!(IRL_HHLH,0x3a0),INTC_VECT!(IRL_HHHL,0x3c0)];
static mut irl3210_mask_registers: [intc_mask_reg; 1] = [INTC_MASK_REG!(0xffd40080,0xffd40084,32,[IRL_LLLL,IRL_LLLH,IRL_LLHL,IRL_LLHH,IRL_LHLL,IRL_LHLH,IRL_LHHL,IRL_LHHH,IRL_HLLL,IRL_HLLH,IRL_HLHL,IRL_HLHH,IRL_HHLL,IRL_HHLH,IRL_HHHL])];
static mut irl7654_mask_registers: [intc_mask_reg; 1] = [INTC_MASK_REG!(0xffd40080,0xffd40084,32,[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,IRL_LLLL,IRL_LLLH,IRL_LLHL,IRL_LLHH,IRL_LHLL,IRL_LHLH,IRL_LHHL,IRL_LHHH,IRL_HLLL,IRL_HLLH,IRL_HLHL,IRL_HLHH,IRL_HHLL,IRL_HHLH,IRL_HHHL])];
static intc_irl7654_desc: intc_desc_type = DECLARE_INTC_DESC!("sh7763-irl7654",irl_vectors,None,irl7654_mask_registers,None,None);
static intc_irl3210_desc: intc_desc_type = DECLARE_INTC_DESC!("sh7763-irl3210",irl_vectors,None,irl3210_mask_registers,None,None);

const INTC_ICR0: usize = 0xffd00000; const INTC_INTMSK0: usize = 0xffd00044; const INTC_INTMSK1: usize = 0xffd00048; const INTC_INTMSK2: usize = 0xffd40080; const INTC_INTMSKCLR1: usize = 0xffd00068; const INTC_INTMSKCLR2: usize = 0xffd40084;
unsafe fn plat_irq_setup() { __raw_writel(0xff000000, INTC_INTMSK0); __raw_writel(0xc0000000, INTC_INTMSK1); __raw_writel(0xfffefffe, INTC_INTMSK2); register_intc_controller(&intc_desc); }
unsafe fn plat_irq_setup_pins(mode: c_int) { match mode { IRQ_MODE_IRQ => { __raw_writel(__raw_readl(INTC_ICR0) | 0x00c00000, INTC_ICR0); register_intc_controller(&intc_irq_desc); }, IRQ_MODE_IRL7654 => { __raw_writel(0x40000000, INTC_INTMSKCLR1); __raw_writel(0x0000fffe, INTC_INTMSKCLR2); }, IRQ_MODE_IRL3210 => { __raw_writel(0x80000000, INTC_INTMSKCLR1); __raw_writel(0xfffe0000, INTC_INTMSKCLR2); }, IRQ_MODE_IRL7654_MASK => { __raw_writel(0x40000000, INTC_INTMSKCLR1); register_intc_controller(&intc_irl7654_desc); }, IRQ_MODE_IRL3210_MASK => { __raw_writel(0x80000000, INTC_INTMSKCLR1); register_intc_controller(&intc_irl3210_desc); }, _ => BUG!(), } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
