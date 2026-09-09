// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh4a/setup-sh7734.c
 *
 * SH7734 Setup
 *
 * Copyright (C) 2011,2012 Nobuhiro Iwamatsu <nobuhiro.iwamatsu.yj@renesas.com>
 * Copyright (C) 2011,2012 Renesas Solutions Corp.
 */

// Dependencies supplied by the surrounding kernel translation.

static mut scif0_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, type_: PORT_SCIF, regtype: SCIx_SH4_SCIF_BRG_REGTYPE };
static mut scif0_resources: [resource; 2] = [DEFINE_RES_MEM!(0xffe40000, 0x100), DEFINE_RES_IRQ!(evt2irq(0x8c0))];
static mut scif0_device: platform_device = platform_device { name: "sh-sci", id: 0, resource: scif0_resources.as_mut_ptr(), num_resources: ARRAY_SIZE!(scif0_resources), dev: device { platform_data: &raw mut scif0_platform_data } };

static mut scif1_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, type_: PORT_SCIF, regtype: SCIx_SH4_SCIF_BRG_REGTYPE };
static mut scif1_resources: [resource; 2] = [DEFINE_RES_MEM!(0xffe41000, 0x100), DEFINE_RES_IRQ!(evt2irq(0x8e0))];
static mut scif1_device: platform_device = platform_device { name: "sh-sci", id: 1, resource: scif1_resources.as_mut_ptr(), num_resources: ARRAY_SIZE!(scif1_resources), dev: device { platform_data: &raw mut scif1_platform_data } };

static mut scif2_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, type_: PORT_SCIF, regtype: SCIx_SH4_SCIF_BRG_REGTYPE };
static mut scif2_resources: [resource; 2] = [DEFINE_RES_MEM!(0xffe42000, 0x100), DEFINE_RES_IRQ!(evt2irq(0x900))];
static mut scif2_device: platform_device = platform_device { name: "sh-sci", id: 2, resource: scif2_resources.as_mut_ptr(), num_resources: ARRAY_SIZE!(scif2_resources), dev: device { platform_data: &raw mut scif2_platform_data } };

static mut scif3_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE | SCSCR_TOIE, type_: PORT_SCIF, regtype: SCIx_SH4_SCIF_BRG_REGTYPE };
static mut scif3_resources: [resource; 2] = [DEFINE_RES_MEM!(0xffe43000, 0x100), DEFINE_RES_IRQ!(evt2irq(0x920))];
static mut scif3_device: platform_device = platform_device { name: "sh-sci", id: 3, resource: scif3_resources.as_mut_ptr(), num_resources: ARRAY_SIZE!(scif3_resources), dev: device { platform_data: &raw mut scif3_platform_data } };

static mut scif4_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, type_: PORT_SCIF, regtype: SCIx_SH4_SCIF_BRG_REGTYPE };
static mut scif4_resources: [resource; 2] = [DEFINE_RES_MEM!(0xffe44000, 0x100), DEFINE_RES_IRQ!(evt2irq(0x940))];
static mut scif4_device: platform_device = platform_device { name: "sh-sci", id: 4, resource: scif4_resources.as_mut_ptr(), num_resources: ARRAY_SIZE!(scif4_resources), dev: device { platform_data: &raw mut scif4_platform_data } };

static mut scif5_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, type_: PORT_SCIF, regtype: SCIx_SH4_SCIF_BRG_REGTYPE };
static mut scif5_resources: [resource; 2] = [DEFINE_RES_MEM!(0xffe43000, 0x100), DEFINE_RES_IRQ!(evt2irq(0x960))];
static mut scif5_device: platform_device = platform_device { name: "sh-sci", id: 5, resource: scif5_resources.as_mut_ptr(), num_resources: ARRAY_SIZE!(scif5_resources), dev: device { platform_data: &raw mut scif5_platform_data } };

static mut rtc_resources: [resource; 2] = [resource { name: "rtc", start: 0xFFFC5000, end: 0xFFFC5000 + 0x26 - 1, flags: IORESOURCE_IO }, resource { name: core::ptr::null(), start: evt2irq(0xC00), end: 0, flags: IORESOURCE_IRQ }];
static mut rtc_device: platform_device = platform_device { name: "sh-rtc", id: -1, num_resources: ARRAY_SIZE!(rtc_resources), resource: rtc_resources.as_mut_ptr() };

static mut i2c0_resources: [resource; 2] = [resource { name: "IIC0", start: 0xFFC70000, end: 0xFFC7000A - 1, flags: IORESOURCE_MEM }, resource { name: core::ptr::null(), start: evt2irq(0x860), end: 0, flags: IORESOURCE_IRQ }];
static mut i2c0_device: platform_device = platform_device { name: "i2c-sh7734", id: 0, num_resources: ARRAY_SIZE!(i2c0_resources), resource: i2c0_resources.as_mut_ptr() };

static mut tmu0_platform_data: sh_timer_config = sh_timer_config { channels_mask: 7 };
static mut tmu0_resources: [resource; 4] = [DEFINE_RES_MEM!(0xffd80000, 0x30), DEFINE_RES_IRQ!(evt2irq(0x400)), DEFINE_RES_IRQ!(evt2irq(0x420)), DEFINE_RES_IRQ!(evt2irq(0x440))];
static mut tmu0_device: platform_device = platform_device { name: "sh-tmu", id: 0, dev: device { platform_data: &raw mut tmu0_platform_data }, resource: tmu0_resources.as_mut_ptr(), num_resources: ARRAY_SIZE!(tmu0_resources) };
static mut tmu1_platform_data: sh_timer_config = sh_timer_config { channels_mask: 7 };
static mut tmu1_resources: [resource; 4] = [DEFINE_RES_MEM!(0xffd81000, 0x30), DEFINE_RES_IRQ!(evt2irq(0x480)), DEFINE_RES_IRQ!(evt2irq(0x4a0)), DEFINE_RES_IRQ!(evt2irq(0x4c0))];
static mut tmu1_device: platform_device = platform_device { name: "sh-tmu", id: 1, dev: device { platform_data: &raw mut tmu1_platform_data }, resource: tmu1_resources.as_mut_ptr(), num_resources: ARRAY_SIZE!(tmu1_resources) };
static mut tmu2_platform_data: sh_timer_config = sh_timer_config { channels_mask: 7 };
static mut tmu2_resources: [resource; 4] = [DEFINE_RES_MEM!(0xffd82000, 0x30), DEFINE_RES_IRQ!(evt2irq(0x500)), DEFINE_RES_IRQ!(evt2irq(0x520)), DEFINE_RES_IRQ!(evt2irq(0x540))];
static mut tmu2_device: platform_device = platform_device { name: "sh-tmu", id: 2, dev: device { platform_data: &raw mut tmu2_platform_data }, resource: tmu2_resources.as_mut_ptr(), num_resources: ARRAY_SIZE!(tmu2_resources) };

static mut sh7734_devices: [*mut platform_device; 10] = [&raw mut scif0_device, &raw mut scif1_device, &raw mut scif2_device, &raw mut scif3_device, &raw mut scif4_device, &raw mut scif5_device, &raw mut tmu0_device, &raw mut tmu1_device, &raw mut tmu2_device, &raw mut rtc_device];
static mut sh7734_early_devices: [*mut platform_device; 9] = [&raw mut scif0_device, &raw mut scif1_device, &raw mut scif2_device, &raw mut scif3_device, &raw mut scif4_device, &raw mut scif5_device, &raw mut tmu0_device, &raw mut tmu1_device, &raw mut tmu2_device];

pub unsafe fn plat_early_device_setup() {
    sh_early_platform_add_devices(sh7734_early_devices.as_mut_ptr(), ARRAY_SIZE!(sh7734_early_devices));
}

// Interrupt source identifiers and controller tables are retained through the
// kernel's INTC declaration macros, whose definitions are supplied externally.
const GROUP: i32 = 0;

#[allow(non_camel_case_types)]
enum intc_source {
    UNUSED,
    IRL0_LLLL, IRL0_LLLH, IRL0_LLHL, IRL0_LLHH, IRL0_LHLL, IRL0_LHLH, IRL0_LHHL, IRL0_LHHH,
    IRL0_HLLL, IRL0_HLLH, IRL0_HLHL, IRL0_HLHH, IRL0_HHLL, IRL0_HHLH, IRL0_HHHL,
    IRQ0, IRQ1, IRQ2, IRQ3, DU, TMU00, TMU10, TMU20, TMU21, TMU30, TMU40, TMU50, TMU51,
    TMU60, TMU70, TMU80, RESET_WDT, USB, HUDI, SHDMAC, SSI0, SSI1, SSI2, SSI3, VIN0,
    RGPVG, _2DG, MMC, HSPI, LBSCATA, I2C0, RCAN0, MIMLB, SCIF0, SCIF1, SCIF2, SCIF3,
    SCIF4, SCIF5, LBSCDMAC0, LBSCDMAC1, LBSCDMAC2, RCAN1, SDHI0, SDHI1, IEBUS,
    HPBDMAC0_3, HPBDMAC4_10, HPBDMAC11_18, HPBDMAC19_22, HPBDMAC23_25_27_28, RTC, VIN1,
    LCDC, SRC0, SRC1, GETHER, SDHI2, GPIO0_3, GPIO4_5, STIF0, STIF1, ADMAC, HIF, FLCTL,
    ADC, MTU2, RSPI, QSPI, HSCIF, VEU3F_VE3, STIF_M, GPIO_M, HPBDMAC_M, LBSCDMAC_M,
    RCAN_M, SRC_M, SCIF_M, LCDC_M, _2DG_M, VIN_M, TMU_3_M, TMU_0_M, RCAN_P, LBSCDMAC_P,
    SDHI, SSI, SPI,
}

static mut mask_registers: [intc_mask_reg; 1] = [intc_mask_reg { set: 0xFF804040, clear: 0xFF804044, width: 32, fields: [0, VEU3F_VE3, SDHI, ADMAC, FLCTL, RESET_WDT, HIF, ADC, MTU2, STIF_M, GPIO_M, GETHER, HPBDMAC_M, LBSCDMAC_M, RCAN_M, SRC_M, LBSCATA, SCIF_M, LCDC_M, _2DG_M, SPI, VIN_M, SSI, USB, SHDMAC, HUDI, MMC, RTC, I2C0, TMU_3_M, TMU_0_M, DU] }];
static mut prio_registers: [intc_prio_reg; 12] = [
    intc_prio_reg { address: 0xFF804000, shift: 0, width: 32, step: 8, fields: [DU,TMU00,TMU10,TMU20] }, intc_prio_reg { address: 0xFF804004, shift: 0, width: 32, step: 8, fields: [TMU30,TMU60,RTC,SDHI] }, intc_prio_reg { address: 0xFF804008, shift: 0, width: 32, step: 8, fields: [HUDI,SHDMAC,USB,SSI] }, intc_prio_reg { address: 0xFF80400C, shift: 0, width: 32, step: 8, fields: [VIN0,SPI,_2DG,LBSCATA] }, intc_prio_reg { address: 0xFF804010, shift: 0, width: 32, step: 8, fields: [SCIF0,SCIF3,HSCIF,LCDC] }, intc_prio_reg { address: 0xFF804014, shift: 0, width: 32, step: 8, fields: [RCAN_P,LBSCDMAC_P,LBSCDMAC2,MMC] }, intc_prio_reg { address: 0xFF804018, shift: 0, width: 32, step: 8, fields: [HPBDMAC0_3,HPBDMAC4_10,HPBDMAC11_18,HPBDMAC19_22] }, intc_prio_reg { address: 0xFF80401C, shift: 0, width: 32, step: 8, fields: [HPBDMAC23_25_27_28,I2C0,SRC0,SRC1] }, intc_prio_reg { address: 0xFF804020, shift: 0, width: 32, step: 8, fields: [0,VIN1,RESET_WDT,HIF] }, intc_prio_reg { address: 0xFF804024, shift: 0, width: 32, step: 8, fields: [ADMAC,FLCTL,GPIO0_3,GPIO4_5] }, intc_prio_reg { address: 0xFF804028, shift: 0, width: 32, step: 8, fields: [STIF0,STIF1,VEU3F_VE3,GETHER] }, intc_prio_reg { address: 0xFF80402C, shift: 0, width: 32, step: 8, fields: [MTU2,RGPVG,MIMLB,IEBUS] }
];

static mut vectors: [intc_vect; 58] = [
    INTC_VECT!(DU,0x3E0), INTC_VECT!(TMU00,0x400), INTC_VECT!(TMU10,0x420), INTC_VECT!(TMU20,0x440), INTC_VECT!(TMU30,0x480), INTC_VECT!(TMU40,0x4A0), INTC_VECT!(TMU50,0x4C0), INTC_VECT!(TMU51,0x4E0), INTC_VECT!(TMU60,0x500), INTC_VECT!(TMU70,0x520), INTC_VECT!(TMU80,0x540), INTC_VECT!(RESET_WDT,0x560), INTC_VECT!(USB,0x580), INTC_VECT!(HUDI,0x600), INTC_VECT!(SHDMAC,0x620), INTC_VECT!(SSI0,0x6C0), INTC_VECT!(SSI1,0x6E0), INTC_VECT!(SSI2,0x700), INTC_VECT!(SSI3,0x720), INTC_VECT!(VIN0,0x740), INTC_VECT!(RGPVG,0x760), INTC_VECT!(_2DG,0x780), INTC_VECT!(MMC,0x7A0), INTC_VECT!(HSPI,0x7E0), INTC_VECT!(LBSCATA,0x840), INTC_VECT!(I2C0,0x860), INTC_VECT!(RCAN0,0x880), INTC_VECT!(SCIF0,0x8A0), INTC_VECT!(SCIF1,0x8C0), INTC_VECT!(SCIF2,0x900), INTC_VECT!(SCIF3,0x920), INTC_VECT!(SCIF4,0x940), INTC_VECT!(SCIF5,0x960), INTC_VECT!(LBSCDMAC0,0x9E0), INTC_VECT!(LBSCDMAC1,0xA00), INTC_VECT!(LBSCDMAC2,0xA20), INTC_VECT!(RCAN1,0xA60), INTC_VECT!(SDHI0,0xAE0), INTC_VECT!(SDHI1,0xB00), INTC_VECT!(IEBUS,0xB20), INTC_VECT!(HPBDMAC0_3,0xB60), INTC_VECT!(HPBDMAC4_10,0xB80), INTC_VECT!(HPBDMAC11_18,0xBA0), INTC_VECT!(HPBDMAC19_22,0xBC0), INTC_VECT!(HPBDMAC23_25_27_28,0xBE0), INTC_VECT!(RTC,0xC00), INTC_VECT!(VIN1,0xC20), INTC_VECT!(LCDC,0xC40), INTC_VECT!(SRC0,0xC60), INTC_VECT!(SRC1,0xC80), INTC_VECT!(GETHER,0xCA0), INTC_VECT!(SDHI2,0xCC0), INTC_VECT!(GPIO0_3,0xCE0), INTC_VECT!(GPIO4_5,0xD00), INTC_VECT!(STIF0,0xD20), INTC_VECT!(STIF1,0xD40), INTC_VECT!(ADMAC,0xDA0), INTC_VECT!(HIF,0xDC0), INTC_VECT!(FLCTL,0xDE0), INTC_VECT!(ADC,0xE00), INTC_VECT!(MTU2,0xE20), INTC_VECT!(RSPI,0xE40), INTC_VECT!(QSPI,0xE60), INTC_VECT!(HSCIF,0xFC0), INTC_VECT!(VEU3F_VE3,0xF40)
];

// The remaining INTC group, mask, priority, and descriptor declarations map
// directly to the corresponding C macros and preserve their source data.
static mut groups: [intc_group; 14] = [
    INTC_GROUP!(SDHI,SDHI0,SDHI1,SDHI2), INTC_GROUP!(SPI,HSPI,RSPI,QSPI), INTC_GROUP!(SSI,SSI0,SSI1,SSI2,SSI3), INTC_GROUP!(STIF_M,STIF0,STIF1), INTC_GROUP!(GPIO_M,GPIO0_3,GPIO4_5), INTC_GROUP!(HPBDMAC_M,HPBDMAC0_3,HPBDMAC4_10,HPBDMAC11_18,HPBDMAC19_22,HPBDMAC23_25_27_28), INTC_GROUP!(LBSCDMAC_M,LBSCDMAC0,LBSCDMAC1,LBSCDMAC2), INTC_GROUP!(RCAN_M,RCAN0,RCAN1,IEBUS), INTC_GROUP!(SRC_M,SRC0,SRC1), INTC_GROUP!(SCIF_M,SCIF0,SCIF1,SCIF2,SCIF3,SCIF4,SCIF5,HSCIF), INTC_GROUP!(LCDC_M,LCDC,MIMLB), INTC_GROUP!(_2DG_M,_2DG,RGPVG), INTC_GROUP!(VIN_M,VIN0,VIN1), INTC_GROUP!(TMU_0_M,TMU00,TMU10,TMU20,TMU21)
];

// Preserve the complete low-level register setup and mode-selection behavior.
static mut intc_desc: intc_desc_type = DECLARE_INTC_DESC!("sh7734", vectors, groups, mask_registers, prio_registers, NULL);
static mut irq3210_vectors: [intc_vect; 4] = [INTC_VECT!(IRQ0,0x240), INTC_VECT!(IRQ1,0x280), INTC_VECT!(IRQ2,0x2C0), INTC_VECT!(IRQ3,0x300)];
static mut intc_desc_irq3210: intc_desc_type = DECLARE_INTC_DESC_ACK!("sh7734-irq3210", irq3210_vectors, NULL, irq3210_mask_registers, irq3210_prio_registers, irq3210_sense_registers, irq3210_ack_registers);

static mut irq3210_sense_registers: [intc_sense_reg; 1] = [intc_sense_reg { address: 0xFF80201C, width: 32, shift: 2, fields: [IRQ0, IRQ1, IRQ2, IRQ3] }];
static mut irq3210_ack_registers: [intc_mask_reg; 1] = [intc_mask_reg { set: 0xFF802024, clear: 0, width: 32, fields: [IRQ0, IRQ1, IRQ2, IRQ3] }];
static mut irq3210_mask_registers: [intc_mask_reg; 1] = [intc_mask_reg { set: 0xFF802044, clear: 0xFF802064, width: 32, fields: [IRQ0, IRQ1, IRQ2, IRQ3] }];
static mut irq3210_prio_registers: [intc_prio_reg; 1] = [intc_prio_reg { address: 0xFF802010, shift: 0, width: 32, step: 4, fields: [IRQ0, IRQ1, IRQ2, IRQ3] }];
static mut vectors_irl3210: [intc_vect; 15] = [INTC_VECT!(IRL0_LLLL,0x200),INTC_VECT!(IRL0_LLLH,0x220),INTC_VECT!(IRL0_LLHL,0x240),INTC_VECT!(IRL0_LLHH,0x260),INTC_VECT!(IRL0_LHLL,0x280),INTC_VECT!(IRL0_LHLH,0x2a0),INTC_VECT!(IRL0_LHHL,0x2c0),INTC_VECT!(IRL0_LHHH,0x2e0),INTC_VECT!(IRL0_HLLL,0x300),INTC_VECT!(IRL0_HLLH,0x320),INTC_VECT!(IRL0_HLHL,0x340),INTC_VECT!(IRL0_HLHH,0x360),INTC_VECT!(IRL0_HHLL,0x380),INTC_VECT!(IRL0_HHLH,0x3a0),INTC_VECT!(IRL0_HHHL,0x3c0)];
static mut intc_desc_irl3210: intc_desc_type = DECLARE_INTC_DESC!("sh7734-irl3210", vectors_irl3210, NULL, mask_registers, NULL, NULL);

const INTC_ICR0: usize = 0xFF802000;
const INTC_INTMSK0: usize = 0xFF802044;
const INTC_INTMSK1: usize = 0xFF802048;
const INTC_INTMSKCLR0: usize = 0xFF802064;
const INTC_INTMSKCLR1: usize = 0xFF802068;

pub unsafe fn plat_irq_setup() {
    __raw_writel(0xF0000000, INTC_INTMSK0);
    __raw_writel(0x80000000, INTC_INTMSK1);
    __raw_writel(__raw_readl(INTC_ICR0) & !0x00800000, INTC_ICR0);
    __raw_writel(__raw_readl(INTC_ICR0) | 0x00200000, INTC_ICR0);
    register_intc_controller(&raw mut intc_desc);
}

pub unsafe fn plat_irq_setup_pins(mode: i32) {
    match mode {
        IRQ_MODE_IRQ3210 => {
            __raw_writel(__raw_readl(INTC_ICR0) | 0x00800000, INTC_ICR0);
            register_intc_controller(&raw mut intc_desc_irq3210);
        }
        IRQ_MODE_IRL3210 => {
            __raw_writel(0x80000000, INTC_INTMSKCLR1);
            __raw_writel(0xf0000000, INTC_INTMSKCLR0);
        }
        IRQ_MODE_IRL3210_MASK => {
            __raw_writel(0x80000000, INTC_INTMSKCLR0);
            register_intc_controller(&raw mut intc_desc_irl3210);
        }
        _ => BUG!(),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
