// SPDX-License-Identifier: GPL-2.0
/*
 * SH7760 Setup
 *
 *  Copyright (C) 2006  Paul Mundt
 */

// Linux dependencies supplied by the surrounding kernel translation.

#[repr(i32)]
enum InterruptSource {
    UNUSED = 0,
    IRL0, IRL1, IRL2, IRL3, HUDI, GPIOI, DMAC,
    IRQ4, IRQ5, IRQ6, IRQ7, HCAN20, HCAN21, SSI0, SSI1,
    HAC0, HAC1, I2C0, I2C1, USB, LCDC, DMABRG0, DMABRG1, DMABRG2,
    SCIF0_ERI, SCIF0_RXI, SCIF0_BRI, SCIF0_TXI,
    SCIF1_ERI, SCIF1_RXI, SCIF1_BRI, SCIF1_TXI,
    SCIF2_ERI, SCIF2_RXI, SCIF2_BRI, SCIF2_TXI,
    SIM_ERI, SIM_RXI, SIM_TXI, SIM_TEI, HSPI,
    MMCIF0, MMCIF1, MMCIF2, MMCIF3, MFI, ADC, CMT,
    TMU0, TMU1, TMU2, WDT, REF,
    DMABRG, SCIF0, SCIF1, SCIF2, SIM, MMCIF,
}

static mut VECTORS: &[intc_vect] = &[
    INTC_VECT!(HUDI, 0x600), INTC_VECT!(GPIOI, 0x620),
    INTC_VECT!(DMAC, 0x640), INTC_VECT!(DMAC, 0x660), INTC_VECT!(DMAC, 0x680),
    INTC_VECT!(DMAC, 0x6a0), INTC_VECT!(DMAC, 0x780), INTC_VECT!(DMAC, 0x7a0),
    INTC_VECT!(DMAC, 0x7c0), INTC_VECT!(DMAC, 0x7e0), INTC_VECT!(DMAC, 0x6c0),
    INTC_VECT!(IRQ4, 0x800), INTC_VECT!(IRQ5, 0x820), INTC_VECT!(IRQ6, 0x840),
    INTC_VECT!(IRQ6, 0x860), INTC_VECT!(HCAN20, 0x900), INTC_VECT!(HCAN21, 0x920),
    INTC_VECT!(SSI0, 0x940), INTC_VECT!(SSI1, 0x960), INTC_VECT!(HAC0, 0x980),
    INTC_VECT!(HAC1, 0x9a0), INTC_VECT!(I2C0, 0x9c0), INTC_VECT!(I2C1, 0x9e0),
    INTC_VECT!(USB, 0xa00), INTC_VECT!(LCDC, 0xa20), INTC_VECT!(DMABRG0, 0xa80),
    INTC_VECT!(DMABRG1, 0xaa0), INTC_VECT!(DMABRG2, 0xac0),
    INTC_VECT!(SCIF0_ERI, 0x880), INTC_VECT!(SCIF0_RXI, 0x8a0), INTC_VECT!(SCIF0_BRI, 0x8c0), INTC_VECT!(SCIF0_TXI, 0x8e0),
    INTC_VECT!(SCIF1_ERI, 0xb00), INTC_VECT!(SCIF1_RXI, 0xb20), INTC_VECT!(SCIF1_BRI, 0xb40), INTC_VECT!(SCIF1_TXI, 0xb60),
    INTC_VECT!(SCIF2_ERI, 0xb80), INTC_VECT!(SCIF2_RXI, 0xba0), INTC_VECT!(SCIF2_BRI, 0xbc0), INTC_VECT!(SCIF2_TXI, 0xbe0),
    INTC_VECT!(SIM_ERI, 0xc00), INTC_VECT!(SIM_RXI, 0xc20), INTC_VECT!(SIM_TXI, 0xc40), INTC_VECT!(SIM_TEI, 0xc60),
    INTC_VECT!(HSPI, 0xc80), INTC_VECT!(MMCIF0, 0xd00), INTC_VECT!(MMCIF1, 0xd20), INTC_VECT!(MMCIF2, 0xd40), INTC_VECT!(MMCIF3, 0xd60),
    INTC_VECT!(MFI, 0xe80), // 0xf80 according to data sheet
    INTC_VECT!(ADC, 0xf80), INTC_VECT!(CMT, 0xfa0), INTC_VECT!(TMU0, 0x400), INTC_VECT!(TMU1, 0x420),
    INTC_VECT!(TMU2, 0x440), INTC_VECT!(TMU2, 0x460), INTC_VECT!(WDT, 0x560), INTC_VECT!(REF, 0x580), INTC_VECT!(REF, 0x5a0),
];

static mut GROUPS: &[intc_group] = &[
    INTC_GROUP!(DMABRG, DMABRG0, DMABRG1, DMABRG2),
    INTC_GROUP!(SCIF0, SCIF0_ERI, SCIF0_RXI, SCIF0_BRI, SCIF0_TXI),
    INTC_GROUP!(SCIF1, SCIF1_ERI, SCIF1_RXI, SCIF1_BRI, SCIF1_TXI),
    INTC_GROUP!(SCIF2, SCIF2_ERI, SCIF2_RXI, SCIF2_BRI, SCIF2_TXI),
    INTC_GROUP!(SIM, SIM_ERI, SIM_RXI, SIM_TXI, SIM_TEI),
    INTC_GROUP!(MMCIF, MMCIF0, MMCIF1, MMCIF2, MMCIF3),
];

static mut MASK_REGISTERS: &[intc_mask_reg] = &[
    intc_mask_reg { set_reg: 0xfe080040, clr_reg: 0xfe080060, width: 32, enum_ids: [IRQ4, IRQ5, IRQ6, IRQ7, 0, 0, HCAN20, HCAN21, SSI0, SSI1, HAC0, HAC1, I2C0, I2C1, USB, LCDC, 0, DMABRG0, DMABRG1, DMABRG2, SCIF0_ERI, SCIF0_RXI, SCIF0_BRI, SCIF0_TXI, SCIF1_ERI, SCIF1_RXI, SCIF1_BRI, SCIF1_TXI, SCIF2_ERI, SCIF2_RXI, SCIF2_BRI, SCIF2_TXI] },
    intc_mask_reg { set_reg: 0xfe080044, clr_reg: 0xfe080064, width: 32, enum_ids: [0,0,0,0,0,0,0,0,SIM_ERI,SIM_RXI,SIM_TXI,SIM_TEI,HSPI,MMCIF0,MMCIF1,MMCIF2,MMCIF3,0,0,0,0,0,0,0,0,0,MFI,0,0,0,0,ADC,CMT] },
];

static mut PRIO_REGISTERS: &[intc_prio_reg] = &[
    intc_prio_reg { reg: 0xffd00004, set_reg: 0, width: 16, field_width: 4, enum_ids: [TMU0,TMU1,TMU2] },
    intc_prio_reg { reg: 0xffd00008, set_reg: 0, width: 16, field_width: 4, enum_ids: [WDT,REF,0,0] },
    intc_prio_reg { reg: 0xffd0000c, set_reg: 0, width: 16, field_width: 4, enum_ids: [GPIOI,DMAC,0,HUDI] },
    intc_prio_reg { reg: 0xffd00010, set_reg: 0, width: 16, field_width: 4, enum_ids: [IRL0,IRL1,IRL2,IRL3] },
    intc_prio_reg { reg: 0xfe080000, set_reg: 0, width: 32, field_width: 4, enum_ids: [IRQ4,IRQ5,IRQ6,IRQ7] },
    intc_prio_reg { reg: 0xfe080004, set_reg: 0, width: 32, field_width: 4, enum_ids: [HCAN20,HCAN21,SSI0,SSI1,HAC0,HAC1,I2C0,I2C1] },
    intc_prio_reg { reg: 0xfe080008, set_reg: 0, width: 32, field_width: 4, enum_ids: [USB,LCDC,DMABRG,SCIF0,SCIF1,SCIF2,SIM,HSPI] },
    intc_prio_reg { reg: 0xfe08000c, set_reg: 0, width: 32, field_width: 4, enum_ids: [0,0,MMCIF,0,MFI,0,ADC,CMT] },
];

static mut INTC_DESC: intc_desc = DECLARE_INTC_DESC!("sh7760", VECTORS, GROUPS, MASK_REGISTERS, PRIO_REGISTERS, None);
static mut VECTORS_IRQ: &[intc_vect] = &[INTC_VECT!(IRL0, 0x240), INTC_VECT!(IRL1, 0x2a0), INTC_VECT!(IRL2, 0x300), INTC_VECT!(IRL3, 0x360)];
static mut INTC_DESC_IRQ: intc_desc = DECLARE_INTC_DESC!("sh7760-irq", VECTORS_IRQ, GROUPS, MASK_REGISTERS, PRIO_REGISTERS, None);

static mut SCIF0_PLATFORM_DATA: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, type_: PORT_SCIF, regtype: SCIx_SH4_SCIF_FIFODATA_REGTYPE };
static mut SCIF1_PLATFORM_DATA: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, type_: PORT_SCIF, regtype: SCIx_SH4_SCIF_FIFODATA_REGTYPE };
static mut SCIF2_PLATFORM_DATA: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, type_: PORT_SCIF, regtype: SCIx_SH4_SCIF_FIFODATA_REGTYPE };
static mut SCIF3_PLATFORM_DATA: plat_sci_port = plat_sci_port { type_: PORT_SCI };
// SIM card serial port: additional registers are deliberately excluded from the resource.
static mut TMU0_PLATFORM_DATA: sh_timer_config = sh_timer_config { channels_mask: 7 };

// Resource and platform-device initializers retain the exact C resources and ordering.
static mut SCIF0_RESOURCES: &[resource] = &[DEFINE_RES_MEM!(0xfe600000, 0x100), DEFINE_RES_IRQ!(evt2irq!(0x880)), DEFINE_RES_IRQ!(evt2irq!(0x8a0)), DEFINE_RES_IRQ!(evt2irq!(0x8e0)), DEFINE_RES_IRQ!(evt2irq!(0x8c0))];
static mut SCIF1_RESOURCES: &[resource] = &[DEFINE_RES_MEM!(0xfe610000, 0x100), DEFINE_RES_IRQ!(evt2irq!(0xb00)), DEFINE_RES_IRQ!(evt2irq!(0xb20)), DEFINE_RES_IRQ!(evt2irq!(0xb60)), DEFINE_RES_IRQ!(evt2irq!(0xb40))];
static mut SCIF2_RESOURCES: &[resource] = &[DEFINE_RES_MEM!(0xfe620000, 0x100), DEFINE_RES_IRQ!(evt2irq!(0xb80)), DEFINE_RES_IRQ!(evt2irq!(0xba0)), DEFINE_RES_IRQ!(evt2irq!(0xbe0)), DEFINE_RES_IRQ!(evt2irq!(0xbc0))];
static mut SCIF3_RESOURCES: &[resource] = &[DEFINE_RES_MEM!(0xfe480000, 0x10), DEFINE_RES_IRQ!(evt2irq!(0xc00)), DEFINE_RES_IRQ!(evt2irq!(0xc20)), DEFINE_RES_IRQ!(evt2irq!(0xc40))];
static mut TMU0_RESOURCES: &[resource] = &[DEFINE_RES_MEM!(0xffd80000, 0x30), DEFINE_RES_IRQ!(evt2irq!(0x400)), DEFINE_RES_IRQ!(evt2irq!(0x420)), DEFINE_RES_IRQ!(evt2irq!(0x440))];

static mut SCIF0_DEVICE: platform_device = PLATFORM_DEVICE!("sh-sci", 0, SCIF0_RESOURCES, &mut SCIF0_PLATFORM_DATA);
static mut SCIF1_DEVICE: platform_device = PLATFORM_DEVICE!("sh-sci", 1, SCIF1_RESOURCES, &mut SCIF1_PLATFORM_DATA);
static mut SCIF2_DEVICE: platform_device = PLATFORM_DEVICE!("sh-sci", 2, SCIF2_RESOURCES, &mut SCIF2_PLATFORM_DATA);
static mut SCIF3_DEVICE: platform_device = PLATFORM_DEVICE!("sh-sci", 3, SCIF3_RESOURCES, &mut SCIF3_PLATFORM_DATA);
static mut TMU0_DEVICE: platform_device = PLATFORM_DEVICE!("sh-tmu", 0, TMU0_RESOURCES, &mut TMU0_PLATFORM_DATA);

extern "C" {
    fn platform_add_devices(devices: *mut *mut platform_device, count: usize) -> i32;
    fn sh_early_platform_add_devices(devices: *mut *mut platform_device, count: usize);
    fn register_intc_controller(desc: *mut intc_desc);
    fn __raw_readw(addr: usize) -> u16;
    fn __raw_writew(value: u16, addr: usize);
    fn BUG() -> !;
}

#[no_mangle]
unsafe extern "C" fn sh7760_devices_setup() -> i32 {
    let mut devices = [
        &mut SCIF0_DEVICE, &mut SCIF1_DEVICE, &mut SCIF2_DEVICE,
        &mut SCIF3_DEVICE, &mut TMU0_DEVICE,
    ];
    platform_add_devices(devices.as_mut_ptr(), devices.len())
}

#[no_mangle]
unsafe extern "C" fn plat_early_device_setup() {
    let mut devices = [&mut SCIF0_DEVICE, &mut SCIF1_DEVICE, &mut SCIF2_DEVICE, &mut SCIF3_DEVICE, &mut TMU0_DEVICE];
    sh_early_platform_add_devices(devices.as_mut_ptr(), devices.len());
}

const INTC_ICR: usize = 0xffd00000;
const INTC_ICR_IRLM: u16 = 1 << 7;

#[no_mangle]
unsafe extern "C" fn plat_irq_setup_pins(mode: i32) {
    match mode {
        IRQ_MODE_IRQ => {
            __raw_writew(__raw_readw(INTC_ICR) | INTC_ICR_IRLM, INTC_ICR);
            register_intc_controller(&mut INTC_DESC_IRQ);
        }
        _ => BUG(),
    }
}

#[no_mangle]
unsafe extern "C" fn plat_irq_setup() {
    register_intc_controller(&mut INTC_DESC);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
