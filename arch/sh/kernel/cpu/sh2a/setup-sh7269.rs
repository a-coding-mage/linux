// SPDX-License-Identifier: GPL-2.0
/*
 * SH7269 Setup
 *
 * Copyright (C) 2012  Renesas Electronics Europe Ltd
 * Copyright (C) 2012  Phil Edworthy
 */
// Linux dependencies supplied externally.

#[repr(usize)]
#[allow(non_camel_case_types)]
enum Interrupt {
    UNUSED = 0,
    IRQ0, IRQ1, IRQ2, IRQ3, IRQ4, IRQ5, IRQ6, IRQ7,
    PINT0, PINT1, PINT2, PINT3, PINT4, PINT5, PINT6, PINT7,
    DMAC0, DMAC1, DMAC2, DMAC3, DMAC4, DMAC5, DMAC6, DMAC7,
    DMAC8, DMAC9, DMAC10, DMAC11, DMAC12, DMAC13, DMAC14, DMAC15,
    USB, VDC4, CMT0, CMT1, BSC, WDT,
    MTU0_ABCD, MTU0_VEF, MTU1_AB, MTU1_VU, MTU2_AB, MTU2_VU,
    MTU3_ABCD, MTU3_TCI3V, MTU4_ABCD, MTU4_TCI4V, PWMT1, PWMT2, ADC_ADI,
    SSIF0, SSII1, SSII2, SSII3, SSII4, SSII5, RSPDIF,
    IIC30, IIC31, IIC32, IIC33,
    SCIF0_BRI, SCIF0_ERI, SCIF0_RXI, SCIF0_TXI,
    SCIF1_BRI, SCIF1_ERI, SCIF1_RXI, SCIF1_TXI,
    SCIF2_BRI, SCIF2_ERI, SCIF2_RXI, SCIF2_TXI,
    SCIF3_BRI, SCIF3_ERI, SCIF3_RXI, SCIF3_TXI,
    SCIF4_BRI, SCIF4_ERI, SCIF4_RXI, SCIF4_TXI,
    SCIF5_BRI, SCIF5_ERI, SCIF5_RXI, SCIF5_TXI,
    SCIF6_BRI, SCIF6_ERI, SCIF6_RXI, SCIF6_TXI,
    SCIF7_BRI, SCIF7_ERI, SCIF7_RXI, SCIF7_TXI,
    RCAN0, RCAN1, RCAN2, RSPIC0, RSPIC1, IEBC, CD_ROMD, NFMC,
    SDHI0, SDHI1, RTC, SRCC0, SRCC1, SRCC2,
    PINT, SCIF0, SCIF1, SCIF2, SCIF3, SCIF4, SCIF5, SCIF6, SCIF7,
}

// INTC_IRQ entries from the C source, in their original order.  The kernel
// binding supplies the representation and constructor for these declarations.
static mut VECTORS: [intc_vect; 0] = intc_vectors! {
    IRQ0=>64, IRQ1=>65, IRQ2=>66, IRQ3=>67, IRQ4=>68, IRQ5=>69, IRQ6=>70, IRQ7=>71,
    PINT0=>80, PINT1=>81, PINT2=>82, PINT3=>83, PINT4=>84, PINT5=>85, PINT6=>86, PINT7=>87,
    DMAC0=>108, DMAC0=>109, DMAC1=>112, DMAC1=>113, DMAC2=>116, DMAC2=>117,
    DMAC3=>120, DMAC3=>121, DMAC4=>124, DMAC4=>125, DMAC5=>128, DMAC5=>129,
    DMAC6=>132, DMAC6=>133, DMAC7=>136, DMAC7=>137, DMAC8=>140, DMAC8=>141,
    DMAC9=>144, DMAC9=>145, DMAC10=>148, DMAC10=>149, DMAC11=>152, DMAC11=>153,
    DMAC12=>156, DMAC12=>157, DMAC13=>160, DMAC13=>161, DMAC14=>164, DMAC14=>165,
    DMAC15=>168, DMAC15=>169, USB=>170, VDC4=>171..=177, CMT0=>188, CMT1=>189,
    BSC=>190, WDT=>191, MTU0_ABCD=>192..=195, MTU0_VEF=>196..=198,
    MTU1_AB=>199..=200, MTU1_VU=>201..=202, MTU2_AB=>203..=204, MTU2_VU=>205..=206,
    MTU3_ABCD=>207..=210, MTU3_TCI3V=>211, MTU4_ABCD=>212..=215, MTU4_TCI4V=>216,
    PWMT1=>217, PWMT2=>218, ADC_ADI=>223, SSIF0=>224..=226, SSII1=>227..=228,
    SSII2=>229..=230, SSII3=>231..=232, SSII4=>233..=234, SSII5=>235..=236,
    RSPDIF=>237, IIC30=>238..=242, IIC31=>243..=247, IIC32=>248..=252, IIC33=>253..=257,
    SCIF0_BRI=>258, SCIF0_ERI=>259, SCIF0_RXI=>260, SCIF0_TXI=>261,
    SCIF1_BRI=>262, SCIF1_ERI=>263, SCIF1_RXI=>264, SCIF1_TXI=>265,
    SCIF2_BRI=>266, SCIF2_ERI=>267, SCIF2_RXI=>268, SCIF2_TXI=>269,
    SCIF3_BRI=>270, SCIF3_ERI=>271, SCIF3_RXI=>272, SCIF3_TXI=>273,
    SCIF4_BRI=>274, SCIF4_ERI=>275, SCIF4_RXI=>276, SCIF4_TXI=>277,
    SCIF5_BRI=>278, SCIF5_ERI=>279, SCIF5_RXI=>280, SCIF5_TXI=>281,
    SCIF6_BRI=>282, SCIF6_ERI=>283, SCIF6_RXI=>284, SCIF6_TXI=>285,
    SCIF7_BRI=>286, SCIF7_ERI=>287, SCIF7_RXI=>288, SCIF7_TXI=>289,
    RCAN0=>291..=295, RCAN1=>296..=300, RCAN2=>301..=305,
    RSPIC0=>306..=308, RSPIC1=>309..=311, IEBC=>318, CD_ROMD=>319..=324,
    NFMC=>325..=328, SDHI0=>332..=334, SDHI1=>335..=337, RTC=>338..=340,
    SRCC0=>341..=345, SRCC1=>346..=350, SRCC2=>351..=355
};
static mut GROUPS: [intc_group; 9] = intc_groups! {
    PINT: [PINT0,PINT1,PINT2,PINT3,PINT4,PINT5,PINT6,PINT7],
    SCIF0: [SCIF0_BRI,SCIF0_ERI,SCIF0_RXI,SCIF0_TXI], SCIF1: [SCIF1_BRI,SCIF1_ERI,SCIF1_RXI,SCIF1_TXI],
    SCIF2: [SCIF2_BRI,SCIF2_ERI,SCIF2_RXI,SCIF2_TXI], SCIF3: [SCIF3_BRI,SCIF3_ERI,SCIF3_RXI,SCIF3_TXI],
    SCIF4: [SCIF4_BRI,SCIF4_ERI,SCIF4_RXI,SCIF4_TXI], SCIF5: [SCIF5_BRI,SCIF5_ERI,SCIF5_RXI,SCIF5_TXI],
    SCIF6: [SCIF6_BRI,SCIF6_ERI,SCIF6_RXI,SCIF6_TXI], SCIF7: [SCIF7_BRI,SCIF7_ERI,SCIF7_RXI,SCIF7_TXI]
};
static mut MASK_REGISTERS: [intc_mask_reg; 1] = [intc_mask_reg {
    set_reg: 0xfffe0808, clr_reg: 0, reg_width: 16,
    field_width: 0, enum_ids: [0; 16],
}];
static mut PRIO_REGISTERS: [intc_prio_reg; 25] = [
    intc_prio_reg { set_reg: 0xfffe0818, clr_reg: 0, reg_width: 16, field_width: 4, enum_ids: [IRQ0, IRQ1, IRQ2, IRQ3] },
    intc_prio_reg { set_reg: 0xfffe081a, clr_reg: 0, reg_width: 16, field_width: 4, enum_ids: [IRQ4, IRQ5, IRQ6, IRQ7] },
    intc_prio_reg { set_reg: 0xfffe0820, clr_reg: 0, reg_width: 16, field_width: 4, enum_ids: [PINT, 0, 0, 0] },
    intc_prio_reg { set_reg: 0xfffe0c00, clr_reg: 0, reg_width: 16, field_width: 4, enum_ids: [DMAC0,DMAC1,DMAC2,DMAC3] },
    intc_prio_reg { set_reg: 0xfffe0c02, clr_reg: 0, reg_width: 16, field_width: 4, enum_ids: [DMAC4,DMAC5,DMAC6,DMAC7] },
    intc_prio_reg { set_reg: 0xfffe0c04, clr_reg: 0, reg_width: 16, field_width: 4, enum_ids: [DMAC8,DMAC9,DMAC10,DMAC11] },
    intc_prio_reg { set_reg: 0xfffe0c06, clr_reg: 0, reg_width: 16, field_width: 4, enum_ids: [DMAC12,DMAC13,DMAC14,DMAC15] },
    intc_prio_reg { set_reg: 0xfffe0c08, clr_reg: 0, reg_width: 16, field_width: 4, enum_ids: [USB,VDC4,VDC4,VDC4] },
    intc_prio_reg { set_reg: 0xfffe0c0a, clr_reg: 0, reg_width: 16, field_width: 4, enum_ids: [0,0,0,0] },
    intc_prio_reg { set_reg: 0xfffe0c0c, clr_reg: 0, reg_width: 16, field_width: 4, enum_ids: [CMT0,CMT1,BSC,WDT] },
    intc_prio_reg { set_reg: 0xfffe0c0e, clr_reg: 0, reg_width: 16, field_width: 4, enum_ids: [MTU0_ABCD,MTU0_VEF,MTU1_AB,MTU1_VU] },
    intc_prio_reg { set_reg: 0xfffe0c10, clr_reg: 0, reg_width: 16, field_width: 4, enum_ids: [MTU2_AB,MTU2_VU,MTU3_ABCD,MTU3_TCI3V] },
    intc_prio_reg { set_reg: 0xfffe0c12, clr_reg: 0, reg_width: 16, field_width: 4, enum_ids: [MTU4_ABCD,MTU4_TCI4V,PWMT1,PWMT2] },
    intc_prio_reg { set_reg: 0xfffe0c14, clr_reg: 0, reg_width: 16, field_width: 4, enum_ids: [0,0,0,0] },
    intc_prio_reg { set_reg: 0xfffe0c16, clr_reg: 0, reg_width: 16, field_width: 4, enum_ids: [ADC_ADI,SSIF0,SSII1,SSII2] },
    intc_prio_reg { set_reg: 0xfffe0c18, clr_reg: 0, reg_width: 16, field_width: 4, enum_ids: [SSII3,SSII4,SSII5,RSPDIF] },
    intc_prio_reg { set_reg: 0xfffe0c1a, clr_reg: 0, reg_width: 16, field_width: 4, enum_ids: [IIC30,IIC31,IIC32,IIC33] },
    intc_prio_reg { set_reg: 0xfffe0c1c, clr_reg: 0, reg_width: 16, field_width: 4, enum_ids: [SCIF0,SCIF1,SCIF2,SCIF3] },
    intc_prio_reg { set_reg: 0xfffe0c1e, clr_reg: 0, reg_width: 16, field_width: 4, enum_ids: [SCIF4,SCIF5,SCIF6,SCIF7] },
    intc_prio_reg { set_reg: 0xfffe0c20, clr_reg: 0, reg_width: 16, field_width: 4, enum_ids: [0,RCAN0,RCAN1,RCAN2] },
    intc_prio_reg { set_reg: 0xfffe0c22, clr_reg: 0, reg_width: 16, field_width: 4, enum_ids: [RSPIC0,RSPIC1,0,0] },
    intc_prio_reg { set_reg: 0xfffe0c24, clr_reg: 0, reg_width: 16, field_width: 4, enum_ids: [IEBC,CD_ROMD,NFMC,0] },
    intc_prio_reg { set_reg: 0xfffe0c26, clr_reg: 0, reg_width: 16, field_width: 4, enum_ids: [SDHI0,SDHI1,RTC,0] },
    intc_prio_reg { set_reg: 0xfffe0c28, clr_reg: 0, reg_width: 16, field_width: 4, enum_ids: [SRCC0,SRCC1,SRCC2,0] },
];

static mut INTC_DESC: intc_desc = DECLARE_INTC_DESC!("sh7269", VECTORS, GROUPS, MASK_REGISTERS, PRIO_REGISTERS, None);

macro_rules! sci_device { ($n:literal, $base:literal, $bri:literal, $eri:literal, $rxi:literal, $txi:literal) => {
    static mut scif$ n _platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, type_: PORT_SCIF, regtype: SCIx_SH2_SCIF_FIFODATA_REGTYPE };
} }
// The eight SCIF platform devices and their resources retain the C table layout.
static mut SCIF_DEVICES: [platform_device; 8] = [
    platform_device::sci("sh-sci", 0, 0xe8007000, [259,260,261,258]),
    platform_device::sci("sh-sci", 1, 0xe8007800, [263,264,265,262]),
    platform_device::sci("sh-sci", 2, 0xe8008000, [267,268,269,266]),
    platform_device::sci("sh-sci", 3, 0xe8008800, [271,272,273,270]),
    platform_device::sci("sh-sci", 4, 0xe8009000, [275,276,277,274]),
    platform_device::sci("sh-sci", 5, 0xe8009800, [279,280,281,278]),
    platform_device::sci("sh-sci", 6, 0xe800a000, [283,284,285,282]),
    platform_device::sci("sh-sci", 7, 0xe800a800, [287,288,289,286]),
];

static mut CMT_PLATFORM_DATA: sh_timer_config = sh_timer_config { channels_mask: 3 };
static mut CMT_RESOURCES: [resource; 3] = [DEFINE_RES_MEM!(0xfffec000, 0x10), DEFINE_RES_IRQ!(188), DEFINE_RES_IRQ!(189)];
static mut CMT_DEVICE: platform_device = platform_device::with_data("sh-cmt-16", 0, &mut CMT_PLATFORM_DATA, CMT_RESOURCES);
static mut MTU2_RESOURCES: [resource; 3] = [DEFINE_RES_MEM!(0xfffe4000, 0x400), DEFINE_RES_IRQ_NAMED!(192, "tgi0a"), DEFINE_RES_IRQ_NAMED!(203, "tgi1a")];
static mut MTU2_DEVICE: platform_device = platform_device::with_resources("sh-mtu2", -1, MTU2_RESOURCES);
static mut RTC_RESOURCES: [resource; 2] = [resource { start: 0xfffe6000, end: 0xfffe6000 + 0x30 - 1, flags: IORESOURCE_IO }, resource { start: 338, flags: IORESOURCE_IRQ }];
static mut RTC_DEVICE: platform_device = platform_device::with_resources("sh-rtc", -1, RTC_RESOURCES);

/* USB Host */
static mut R8A66597_DATA: r8a66597_platdata = r8a66597_platdata { on_chip: 1, endian: 1 };
static mut USB_HOST_RESOURCES: [resource; 2] = [resource { start: 0xe8010000, end: 0xe80100e4, flags: IORESOURCE_MEM }, resource { start: 170, end: 170, flags: IORESOURCE_IRQ | IRQF_TRIGGER_LOW }];
static mut USB_HOST_DEVICE: platform_device = platform_device::with_data("r8a66597_hcd", 0, &mut R8A66597_DATA, USB_HOST_RESOURCES);

static mut SH7269_DEVICES: [*mut platform_device; 12] = [
    &mut SCIF_DEVICES[0], &mut SCIF_DEVICES[1], &mut SCIF_DEVICES[2], &mut SCIF_DEVICES[3],
    &mut SCIF_DEVICES[4], &mut SCIF_DEVICES[5], &mut SCIF_DEVICES[6], &mut SCIF_DEVICES[7],
    &mut CMT_DEVICE, &mut MTU2_DEVICE, &mut RTC_DEVICE, &mut USB_HOST_DEVICE,
];

unsafe fn sh7269_devices_setup() -> i32 {
    platform_add_devices(SH7269_DEVICES.as_mut_ptr(), SH7269_DEVICES.len())
}
arch_initcall!(sh7269_devices_setup);

unsafe fn plat_irq_setup() { register_intc_controller(&mut INTC_DESC); }

static mut SH7269_EARLY_DEVICES: [*mut platform_device; 10] = [
    &mut SCIF_DEVICES[0], &mut SCIF_DEVICES[1], &mut SCIF_DEVICES[2], &mut SCIF_DEVICES[3],
    &mut SCIF_DEVICES[4], &mut SCIF_DEVICES[5], &mut SCIF_DEVICES[6], &mut SCIF_DEVICES[7],
    &mut CMT_DEVICE, &mut MTU2_DEVICE,
];

unsafe fn plat_early_device_setup() {
    sh_early_platform_add_devices(SH7269_EARLY_DEVICES.as_mut_ptr(), SH7269_EARLY_DEVICES.len());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
