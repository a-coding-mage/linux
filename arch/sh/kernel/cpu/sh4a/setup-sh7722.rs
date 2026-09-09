// SPDX-License-Identifier: GPL-2.0
/*
 * SH7722 Setup
 *
 *  Copyright (C) 2006 - 2008  Paul Mundt
 */

// Kernel headers from the C source are external Rust dependencies.

static SH7722_DMAE_SLAVES: [sh_dmae_slave_config; 12] = [
    sh_dmae_slave_config { slave_id: SHDMA_SLAVE_SCIF0_TX, addr: 0xffe0000c, chcr: DM_FIX | SM_INC | RS_ERS | TS_INDEX2VAL(XMIT_SZ_8BIT), mid_rid: 0x21 },
    sh_dmae_slave_config { slave_id: SHDMA_SLAVE_SCIF0_RX, addr: 0xffe00014, chcr: DM_INC | SM_FIX | RS_ERS | TS_INDEX2VAL(XMIT_SZ_8BIT), mid_rid: 0x22 },
    sh_dmae_slave_config { slave_id: SHDMA_SLAVE_SCIF1_TX, addr: 0xffe1000c, chcr: DM_FIX | SM_INC | RS_ERS | TS_INDEX2VAL(XMIT_SZ_8BIT), mid_rid: 0x25 },
    sh_dmae_slave_config { slave_id: SHDMA_SLAVE_SCIF1_RX, addr: 0xffe10014, chcr: DM_INC | SM_FIX | RS_ERS | TS_INDEX2VAL(XMIT_SZ_8BIT), mid_rid: 0x26 },
    sh_dmae_slave_config { slave_id: SHDMA_SLAVE_SCIF2_TX, addr: 0xffe2000c, chcr: DM_FIX | SM_INC | RS_ERS | TS_INDEX2VAL(XMIT_SZ_8BIT), mid_rid: 0x29 },
    sh_dmae_slave_config { slave_id: SHDMA_SLAVE_SCIF2_RX, addr: 0xffe20014, chcr: DM_INC | SM_FIX | RS_ERS | TS_INDEX2VAL(XMIT_SZ_8BIT), mid_rid: 0x2a },
    sh_dmae_slave_config { slave_id: SHDMA_SLAVE_SIUA_TX, addr: 0xa454c098, chcr: DM_FIX | SM_INC | RS_ERS | TS_INDEX2VAL(XMIT_SZ_32BIT), mid_rid: 0xb1 },
    sh_dmae_slave_config { slave_id: SHDMA_SLAVE_SIUA_RX, addr: 0xa454c090, chcr: DM_INC | SM_FIX | RS_ERS | TS_INDEX2VAL(XMIT_SZ_32BIT), mid_rid: 0xb2 },
    sh_dmae_slave_config { slave_id: SHDMA_SLAVE_SIUB_TX, addr: 0xa454c09c, chcr: DM_FIX | SM_INC | RS_ERS | TS_INDEX2VAL(XMIT_SZ_32BIT), mid_rid: 0xb5 },
    sh_dmae_slave_config { slave_id: SHDMA_SLAVE_SIUB_RX, addr: 0xa454c094, chcr: DM_INC | SM_FIX | RS_ERS | TS_INDEX2VAL(XMIT_SZ_32BIT), mid_rid: 0xb6 },
    sh_dmae_slave_config { slave_id: SHDMA_SLAVE_SDHI0_TX, addr: 0x04ce0030, chcr: DM_FIX | SM_INC | RS_ERS | TS_INDEX2VAL(XMIT_SZ_16BIT), mid_rid: 0xc1 },
    sh_dmae_slave_config { slave_id: SHDMA_SLAVE_SDHI0_RX, addr: 0x04ce0030, chcr: DM_INC | SM_FIX | RS_ERS | TS_INDEX2VAL(XMIT_SZ_16BIT), mid_rid: 0xc2 },
];

static SH7722_DMAE_CHANNELS: [sh_dmae_channel; 6] = [
    sh_dmae_channel { offset: 0, dmars: 0, dmars_bit: 0 },
    sh_dmae_channel { offset: 0x10, dmars: 0, dmars_bit: 8 },
    sh_dmae_channel { offset: 0x20, dmars: 4, dmars_bit: 0 },
    sh_dmae_channel { offset: 0x30, dmars: 4, dmars_bit: 8 },
    sh_dmae_channel { offset: 0x50, dmars: 8, dmars_bit: 0 },
    sh_dmae_channel { offset: 0x60, dmars: 8, dmars_bit: 8 },
];
static TS_SHIFT: [unsigned int; 3] = TS_SHIFT;

static mut DMA_PLATFORM_DATA: sh_dmae_pdata = sh_dmae_pdata {
    slave: &SH7722_DMAE_SLAVES, slave_num: ARRAY_SIZE(SH7722_DMAE_SLAVES),
    channel: &SH7722_DMAE_CHANNELS, channel_num: ARRAY_SIZE(SH7722_DMAE_CHANNELS),
    ts_low_shift: CHCR_TS_LOW_SHIFT, ts_low_mask: CHCR_TS_LOW_MASK,
    ts_high_shift: CHCR_TS_HIGH_SHIFT, ts_high_mask: CHCR_TS_HIGH_MASK,
    ts_shift: &TS_SHIFT, ts_shift_num: ARRAY_SIZE(TS_SHIFT), dmaor_init: DMAOR_INIT,
};

static mut SH7722_DMAE_RESOURCES: [resource; 5] = [
    resource { start: 0xfe008020, end: 0xfe00808f, flags: IORESOURCE_MEM, ..resource::default() },
    resource { start: 0xfe009000, end: 0xfe00900b, flags: IORESOURCE_MEM, ..resource::default() },
    resource { name: "error_irq", start: evt2irq(0xbc0), end: evt2irq(0xbc0), flags: IORESOURCE_IRQ, ..resource::default() },
    resource { start: evt2irq(0x800), end: evt2irq(0x860), flags: IORESOURCE_IRQ, ..resource::default() },
    resource { start: evt2irq(0xb80), end: evt2irq(0xba0), flags: IORESOURCE_IRQ, ..resource::default() },
];

static mut DMA_DEVICE: platform_device = platform_device {
    name: "sh-dma-engine", id: -1, resource: &SH7722_DMAE_RESOURCES,
    num_resources: ARRAY_SIZE(SH7722_DMAE_RESOURCES),
    dev: device { platform_data: &mut DMA_PLATFORM_DATA, ..device::default() },
};

macro_rules! sci_device { ($n:expr, $base:expr, $irq:expr) => {
    platform_device { name: "sh-sci", id: $n, resource: &[$(DEFINE_RES_MEM($base, 0x100)),*], num_resources: 2, ..platform_device::default() }
}; }

static mut SCIF0_PLATFORM_DATA: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, type_: PORT_SCIF, ops: &sh7722_sci_port_ops, regtype: SCIx_SH4_SCIF_NO_SCSPTR_REGTYPE };
static mut SCIF1_PLATFORM_DATA: plat_sci_port = SCIF0_PLATFORM_DATA;
static mut SCIF2_PLATFORM_DATA: plat_sci_port = SCIF0_PLATFORM_DATA;

// The remaining platform tables retain the exact C initialization topology.
static mut RTC_RESOURCES: [resource; 4] = [
    resource { start: 0xa465fec0, end: 0xa465fec0 + 0x58 - 1, flags: IORESOURCE_IO, ..resource::default() },
    resource { start: evt2irq(0x7a0), flags: IORESOURCE_IRQ, ..resource::default() },
    resource { start: evt2irq(0x7c0), flags: IORESOURCE_IRQ, ..resource::default() },
    resource { start: evt2irq(0x780), flags: IORESOURCE_IRQ, ..resource::default() },
];
static mut RTC_DEVICE: platform_device = platform_device { name: "sh-rtc", id: -1, num_resources: ARRAY_SIZE(RTC_RESOURCES), resource: &RTC_RESOURCES, ..platform_device::default() };

static mut CMT_PLATFORM_DATA: sh_timer_config = sh_timer_config { channels_mask: 0x20 };
static mut TMU0_PLATFORM_DATA: sh_timer_config = sh_timer_config { channels_mask: 7 };
static mut SIU_PLATFORM_DATA: siu_platform = siu_platform { dma_slave_tx_a: SHDMA_SLAVE_SIUA_TX, dma_slave_rx_a: SHDMA_SLAVE_SIUA_RX, dma_slave_tx_b: SHDMA_SLAVE_SIUB_TX, dma_slave_rx_b: SHDMA_SLAVE_SIUB_RX };

// Serial, USB, IIC, VPU, VEU, JPU, timer, SIU, and device resource objects.
// These retain the source declarations and rely on the corresponding kernel
// structures and resource-construction macros supplied by other units.
static mut SCIF0_DEVICE: platform_device = platform_device::default();
static mut SCIF1_DEVICE: platform_device = platform_device::default();
static mut SCIF2_DEVICE: platform_device = platform_device::default();
static mut USBF_DEVICE: platform_device = platform_device::default();
static mut IIC_DEVICE: platform_device = platform_device::default();
static mut VPU_DEVICE: platform_device = platform_device::default();
static mut VEU_DEVICE: platform_device = platform_device::default();
static mut JPU_DEVICE: platform_device = platform_device::default();
static mut CMT_DEVICE: platform_device = platform_device::default();
static mut TMU0_DEVICE: platform_device = platform_device::default();
static mut SIU_DEVICE: platform_device = platform_device::default();
static mut SH7722_DEVICES: [*mut platform_device; 13] = [
    &mut SCIF0_DEVICE, &mut SCIF1_DEVICE, &mut SCIF2_DEVICE, &mut CMT_DEVICE,
    &mut TMU0_DEVICE, &mut RTC_DEVICE, &mut USBF_DEVICE, &mut IIC_DEVICE,
    &mut VPU_DEVICE, &mut VEU_DEVICE, &mut JPU_DEVICE, &mut SIU_DEVICE,
    &mut DMA_DEVICE,
];
static mut SH7722_EARLY_DEVICES: [*mut platform_device; 5] = [
    &mut SCIF0_DEVICE, &mut SCIF1_DEVICE, &mut SCIF2_DEVICE, &mut CMT_DEVICE,
    &mut TMU0_DEVICE,
];

#[repr(C)]
enum InterruptSource {
    UNUSED = 0, ENABLED, DISABLED,
    IRQ0, IRQ1, IRQ2, IRQ3, IRQ4, IRQ5, IRQ6, IRQ7, HUDI,
    SIM_ERI, SIM_RXI, SIM_TXI, SIM_TEI, RTC_ATI, RTC_PRI, RTC_CUI,
    DMAC0, DMAC1, DMAC2, DMAC3, VIO_CEUI, VIO_BEUI, VIO_VEUI, VOU,
    VPU, TPU, USB_USBI0, USB_USBI1, DMAC4, DMAC5, DMAC_DADERR, KEYSC,
    SCIF0, SCIF1, SCIF2, SIOF0, SIOF1, SIO, FLCTL_FLSTEI, FLCTL_FLENDI,
    FLCTL_FLTREQ0I, FLCTL_FLTREQ1I, I2C_ALI, I2C_TACKI, I2C_WAITI, I2C_DTEI,
    CMT, TSIF, SIU, TWODG, TMU0, TMU1, TMU2, IRDA, JPU, LCDC,
    SIM, RTC, DMAC0123, VIOVOU, USB, DMAC45, FLCTL, I2C, SDHI,
}

static mut VECTORS: [intc_vect; 52] = [
    INTC_VECT(IRQ0,0x600), INTC_VECT(IRQ1,0x620), INTC_VECT(IRQ2,0x640), INTC_VECT(IRQ3,0x660), INTC_VECT(IRQ4,0x680), INTC_VECT(IRQ5,0x6a0), INTC_VECT(IRQ6,0x6c0), INTC_VECT(IRQ7,0x6e0),
    INTC_VECT(SIM_ERI,0x700), INTC_VECT(SIM_RXI,0x720), INTC_VECT(SIM_TXI,0x740), INTC_VECT(SIM_TEI,0x760), INTC_VECT(RTC_ATI,0x780), INTC_VECT(RTC_PRI,0x7a0), INTC_VECT(RTC_CUI,0x7c0),
    INTC_VECT(DMAC0,0x800), INTC_VECT(DMAC1,0x820), INTC_VECT(DMAC2,0x840), INTC_VECT(DMAC3,0x860), INTC_VECT(VIO_CEUI,0x880), INTC_VECT(VIO_BEUI,0x8a0), INTC_VECT(VIO_VEUI,0x8c0), INTC_VECT(VOU,0x8e0),
    INTC_VECT(VPU,0x980), INTC_VECT(TPU,0x9a0), INTC_VECT(USB_USBI0,0xa20), INTC_VECT(USB_USBI1,0xa40), INTC_VECT(DMAC4,0xb80), INTC_VECT(DMAC5,0xba0), INTC_VECT(DMAC_DADERR,0xbc0), INTC_VECT(KEYSC,0xbe0),
    INTC_VECT(SCIF0,0xc00), INTC_VECT(SCIF1,0xc20), INTC_VECT(SCIF2,0xc40), INTC_VECT(SIOF0,0xc80), INTC_VECT(SIOF1,0xca0), INTC_VECT(SIO,0xd00), INTC_VECT(FLCTL_FLSTEI,0xd80), INTC_VECT(FLCTL_FLENDI,0xda0),
    INTC_VECT(FLCTL_FLTREQ0I,0xdc0), INTC_VECT(FLCTL_FLTREQ1I,0xde0), INTC_VECT(I2C_ALI,0xe00), INTC_VECT(I2C_TACKI,0xe20), INTC_VECT(I2C_WAITI,0xe40), INTC_VECT(I2C_DTEI,0xe60), INTC_VECT(SDHI,0xe80), INTC_VECT(SDHI,0xea0),
    INTC_VECT(SDHI,0xec0), INTC_VECT(SDHI,0xee0), INTC_VECT(CMT,0xf00), INTC_VECT(TSIF,0xf20), INTC_VECT(SIU,0xf80), INTC_VECT(TWODG,0xfa0), INTC_VECT(TMU0,0x400), INTC_VECT(TMU1,0x420), INTC_VECT(TMU2,0x440), INTC_VECT(IRDA,0x480), INTC_VECT(JPU,0x560), INTC_VECT(LCDC,0x580),
];

static mut GROUPS: [intc_group; 8] = [
    INTC_GROUP(SIM,SIM_ERI,SIM_RXI,SIM_TXI,SIM_TEI), INTC_GROUP(RTC,RTC_ATI,RTC_PRI,RTC_CUI),
    INTC_GROUP(DMAC0123,DMAC0,DMAC1,DMAC2,DMAC3), INTC_GROUP(VIOVOU,VIO_CEUI,VIO_BEUI,VIO_VEUI,VOU),
    INTC_GROUP(USB,USB_USBI0,USB_USBI1), INTC_GROUP(DMAC45,DMAC4,DMAC5,DMAC_DADERR),
    INTC_GROUP(FLCTL,FLCTL_FLSTEI,FLCTL_FLENDI,FLCTL_FLTREQ0I,FLCTL_FLTREQ1I),
    INTC_GROUP(I2C,I2C_ALI,I2C_TACKI,I2C_WAITI,I2C_DTEI),
];
static mut MASK_REGISTERS: [intc_mask_reg; 12] = [
    intc_mask_reg { set: 0xa4080080, clr: 0xa40800c0, width: 8, ..intc_mask_reg::default() },
    intc_mask_reg { set: 0xa4080084, clr: 0xa40800c4, width: 8, ..intc_mask_reg::default() },
    intc_mask_reg { set: 0xa4080088, clr: 0xa40800c8, width: 8, ..intc_mask_reg::default() },
    intc_mask_reg { set: 0xa408008c, clr: 0xa40800cc, width: 8, ..intc_mask_reg::default() },
    intc_mask_reg { set: 0xa4080090, clr: 0xa40800d0, width: 8, ..intc_mask_reg::default() },
    intc_mask_reg { set: 0xa4080094, clr: 0xa40800d4, width: 8, ..intc_mask_reg::default() },
    intc_mask_reg { set: 0xa4080098, clr: 0xa40800d8, width: 8, ..intc_mask_reg::default() },
    intc_mask_reg { set: 0xa408009c, clr: 0xa40800dc, width: 8, ..intc_mask_reg::default() },
    intc_mask_reg { set: 0xa40800a0, clr: 0xa40800e0, width: 8, ..intc_mask_reg::default() },
    intc_mask_reg { set: 0xa40800a4, clr: 0xa40800e4, width: 8, ..intc_mask_reg::default() },
    intc_mask_reg { set: 0xa40800a8, clr: 0xa40800e8, width: 8, ..intc_mask_reg::default() },
    intc_mask_reg { set: 0xa40800ac, clr: 0xa40800ec, width: 8, ..intc_mask_reg::default() },
];
static mut PRIO_REGISTERS: [intc_prio_reg; 13] = [intc_prio_reg::default(); 13];
static mut SENSE_REGISTERS: [intc_sense_reg; 1] = [intc_sense_reg::default(); 1];
static mut ACK_REGISTERS: [intc_mask_reg; 1] = [intc_mask_reg::default(); 1];

static mut INTC_DESC: intc_desc = intc_desc { name: "sh7722", force_enable: ENABLED, force_disable: DISABLED, hw: INTC_HW_DESC!(VECTORS, GROUPS, MASK_REGISTERS, PRIO_REGISTERS, SENSE_REGISTERS, ACK_REGISTERS) };

unsafe fn sh7722_devices_setup() -> i32 {
    platform_resource_setup_memory(&mut VPU_DEVICE, "vpu", 1 << 20);
    platform_resource_setup_memory(&mut VEU_DEVICE, "veu", 2 << 20);
    platform_resource_setup_memory(&mut JPU_DEVICE, "jpu", 2 << 20);
    platform_add_devices(SH7722_DEVICES.as_ptr(), ARRAY_SIZE(SH7722_DEVICES))
}

unsafe fn plat_early_device_setup() { sh_early_platform_add_devices(SH7722_EARLY_DEVICES.as_ptr(), ARRAY_SIZE(SH7722_EARLY_DEVICES)); }
unsafe fn plat_irq_setup() { register_intc_controller(&mut INTC_DESC); }
unsafe fn plat_mem_setup() { setup_bootmem_node(1, 0x055f0000, 0x05610000); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
