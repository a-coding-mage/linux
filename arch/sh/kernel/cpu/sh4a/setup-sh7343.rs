// SPDX-License-Identifier: GPL-2.0
/* SH7343 Setup; translated from the Linux C implementation. */

// External Linux/kernel definitions used by this translation are supplied by
// the surrounding tree (platform_device, serial, UIO, timer, and INTC APIs).

static mut scif0_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_CKE1, type_: PORT_SCIF };
static mut scif0_resources: [resource; 2] = [DEFINE_RES_MEM(0xffe00000, 0x100), DEFINE_RES_IRQ(evt2irq(0xc00))];
static mut scif0_device: platform_device = platform_device { name: "sh-sci", id: 0, resource: scif0_resources.as_mut_ptr(), num_resources: ARRAY_SIZE(&scif0_resources), dev: device { platform_data: &mut scif0_platform_data as *mut _ as *mut core::ffi::c_void } };

static mut scif1_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_CKE1, type_: PORT_SCIF };
static mut scif1_resources: [resource; 2] = [DEFINE_RES_MEM(0xffe10000, 0x100), DEFINE_RES_IRQ(evt2irq(0xc20))];
static mut scif1_device: platform_device = platform_device { name: "sh-sci", id: 1, resource: scif1_resources.as_mut_ptr(), num_resources: ARRAY_SIZE(&scif1_resources), dev: device { platform_data: &mut scif1_platform_data as *mut _ as *mut core::ffi::c_void } };

static mut scif2_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_CKE1, type_: PORT_SCIF };
static mut scif2_resources: [resource; 2] = [DEFINE_RES_MEM(0xffe20000, 0x100), DEFINE_RES_IRQ(evt2irq(0xc40))];
static mut scif2_device: platform_device = platform_device { name: "sh-sci", id: 2, resource: scif2_resources.as_mut_ptr(), num_resources: ARRAY_SIZE(&scif2_resources), dev: device { platform_data: &mut scif2_platform_data as *mut _ as *mut core::ffi::c_void } };

static mut scif3_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_CKE1, type_: PORT_SCIF };
static mut scif3_resources: [resource; 2] = [DEFINE_RES_MEM(0xffe30000, 0x100), DEFINE_RES_IRQ(evt2irq(0xc60))];
static mut scif3_device: platform_device = platform_device { name: "sh-sci", id: 3, resource: scif3_resources.as_mut_ptr(), num_resources: ARRAY_SIZE(&scif3_resources), dev: device { platform_data: &mut scif3_platform_data as *mut _ as *mut core::ffi::c_void } };

static mut iic0_resources: [resource; 2] = [resource { name: "IIC0", start: 0x04470000, end: 0x04470017, flags: IORESOURCE_MEM }, resource { name: core::ptr::null(), start: evt2irq(0xe00), end: evt2irq(0xe60), flags: IORESOURCE_IRQ }];
static mut iic0_device: platform_device = platform_device { name: "i2c-sh_mobile", id: 0, num_resources: ARRAY_SIZE(&iic0_resources), resource: iic0_resources.as_mut_ptr(), ..platform_device::zeroed() };
static mut iic1_resources: [resource; 2] = [resource { name: "IIC1", start: 0x04750000, end: 0x04750017, flags: IORESOURCE_MEM }, resource { name: core::ptr::null(), start: evt2irq(0x780), end: evt2irq(0x7e0), flags: IORESOURCE_IRQ }];
static mut iic1_device: platform_device = platform_device { name: "i2c-sh_mobile", id: 1, num_resources: ARRAY_SIZE(&iic1_resources), resource: iic1_resources.as_mut_ptr(), ..platform_device::zeroed() };

static mut vpu_platform_data: uio_info = uio_info { name: "VPU4", version: "0", irq: evt2irq(0x980) };
static mut vpu_resources: [resource; 2] = [resource { name: "VPU", start: 0xfe900000, end: 0xfe9022eb, flags: IORESOURCE_MEM }, resource::zeroed()];
static mut vpu_device: platform_device = platform_device { name: "uio_pdrv_genirq", id: 0, resource: vpu_resources.as_mut_ptr(), num_resources: ARRAY_SIZE(&vpu_resources), dev: device { platform_data: &mut vpu_platform_data as *mut _ as *mut core::ffi::c_void } };
static mut veu_platform_data: uio_info = uio_info { name: "VEU", version: "0", irq: evt2irq(0x8c0) };
static mut veu_resources: [resource; 2] = [resource { name: "VEU", start: 0xfe920000, end: 0xfe9200b7, flags: IORESOURCE_MEM }, resource::zeroed()];
static mut veu_device: platform_device = platform_device { name: "uio_pdrv_genirq", id: 1, resource: veu_resources.as_mut_ptr(), num_resources: ARRAY_SIZE(&veu_resources), dev: device { platform_data: &mut veu_platform_data as *mut _ as *mut core::ffi::c_void } };
static mut jpu_platform_data: uio_info = uio_info { name: "JPU", version: "0", irq: evt2irq(0x560) };
static mut jpu_resources: [resource; 2] = [resource { name: "JPU", start: 0xfea00000, end: 0xfea102d3, flags: IORESOURCE_MEM }, resource::zeroed()];
static mut jpu_device: platform_device = platform_device { name: "uio_pdrv_genirq", id: 2, resource: jpu_resources.as_mut_ptr(), num_resources: ARRAY_SIZE(&jpu_resources), dev: device { platform_data: &mut jpu_platform_data as *mut _ as *mut core::ffi::c_void } };

static mut cmt_platform_data: sh_timer_config = sh_timer_config { channels_mask: 0x20 };
static mut cmt_resources: [resource; 2] = [DEFINE_RES_MEM(0x044a0000, 0x70), DEFINE_RES_IRQ(evt2irq(0xf00))];
static mut cmt_device: platform_device = platform_device { name: "sh-cmt-32", id: 0, dev: device { platform_data: &mut cmt_platform_data as *mut _ as *mut core::ffi::c_void }, resource: cmt_resources.as_mut_ptr(), num_resources: ARRAY_SIZE(&cmt_resources) };
static mut tmu0_platform_data: sh_timer_config = sh_timer_config { channels_mask: 7 };
static mut tmu0_resources: [resource; 4] = [DEFINE_RES_MEM(0xffd80000, 0x2c), DEFINE_RES_IRQ(evt2irq(0x400)), DEFINE_RES_IRQ(evt2irq(0x420)), DEFINE_RES_IRQ(evt2irq(0x440))];
static mut tmu0_device: platform_device = platform_device { name: "sh-tmu", id: 0, dev: device { platform_data: &mut tmu0_platform_data as *mut _ as *mut core::ffi::c_void }, resource: tmu0_resources.as_mut_ptr(), num_resources: ARRAY_SIZE(&tmu0_resources) };

static mut sh7343_devices: [*mut platform_device; 11] = [&mut scif0_device, &mut scif1_device, &mut scif2_device, &mut scif3_device, &mut cmt_device, &mut tmu0_device, &mut iic0_device, &mut iic1_device, &mut vpu_device, &mut veu_device, &mut jpu_device];

unsafe fn sh7343_devices_setup() -> i32 {
    platform_resource_setup_memory(&mut vpu_device, "vpu", 1 << 20);
    platform_resource_setup_memory(&mut veu_device, "veu", 2 << 20);
    platform_resource_setup_memory(&mut jpu_device, "jpu", 2 << 20);
    platform_add_devices(sh7343_devices.as_mut_ptr(), ARRAY_SIZE(&sh7343_devices))
}
arch_initcall!(sh7343_devices_setup);

static mut sh7343_early_devices: [*mut platform_device; 6] = [&mut scif0_device, &mut scif1_device, &mut scif2_device, &mut scif3_device, &mut cmt_device, &mut tmu0_device];
unsafe fn plat_early_device_setup() { sh_early_platform_add_devices(sh7343_early_devices.as_mut_ptr(), ARRAY_SIZE(&sh7343_early_devices)); }

const UNUSED: i32 = 0;
const ENABLED: i32 = 1;
const DISABLED: i32 = 2;
// Interrupt source and group identifiers retain the C enum ordering.
const IRQ0: i32 = 3; const IRQ1: i32 = 4; const IRQ2: i32 = 5; const IRQ3: i32 = 6; const IRQ4: i32 = 7; const IRQ5: i32 = 8; const IRQ6: i32 = 9; const IRQ7: i32 = 10;
const DMAC0: i32 = 11; const DMAC1: i32 = 12; const DMAC2: i32 = 13; const DMAC3: i32 = 14; const VIO_CEUI: i32 = 15; const VIO_BEUI: i32 = 16; const VIO_VEUI: i32 = 17; const VOU: i32 = 18; const MFI: i32 = 19; const VPU: i32 = 20; const TPU: i32 = 21; const Z3D4: i32 = 22; const USBI0: i32 = 23; const USBI1: i32 = 24;
const MMC_ERR: i32 = 25; const MMC_TRAN: i32 = 26; const MMC_FSTAT: i32 = 27; const MMC_FRDY: i32 = 28; const DMAC4: i32 = 29; const DMAC5: i32 = 30; const DMAC_DADERR: i32 = 31; const KEYSC: i32 = 32; const SCIF: i32 = 33; const SCIF1: i32 = 34; const SCIF2: i32 = 35; const SCIF3: i32 = 36; const SIOF0: i32 = 37; const SIOF1: i32 = 38; const SIO: i32 = 39;
const FLCTL_FLSTEI: i32 = 40; const FLCTL_FLENDI: i32 = 41; const FLCTL_FLTREQ0I: i32 = 42; const FLCTL_FLTREQ1I: i32 = 43; const I2C0_ALI: i32 = 44; const I2C0_TACKI: i32 = 45; const I2C0_WAITI: i32 = 46; const I2C0_DTEI: i32 = 47; const I2C1_ALI: i32 = 48; const I2C1_TACKI: i32 = 49; const I2C1_WAITI: i32 = 50; const I2C1_DTEI: i32 = 51; const SIM_TEI: i32 = 52; const SIM_TXI: i32 = 53; const SIM_RXI: i32 = 54; const SIM_ERI: i32 = 55; const IRDA: i32 = 56; const SDHI: i32 = 57; const CMT: i32 = 58; const TSIF: i32 = 59; const SIU: i32 = 60; const TMU0: i32 = 61; const TMU1: i32 = 62; const TMU2: i32 = 63; const JPU: i32 = 64; const LCDC: i32 = 65;
const DMAC0123: i32 = 66; const VIOVOU: i32 = 67; const MMC: i32 = 68; const DMAC45: i32 = 69; const FLCTL: i32 = 70; const I2C0: i32 = 71; const I2C1: i32 = 72; const SIM: i32 = 73; const USB: i32 = 74;

// The following INTC tables are kept as direct macro-equivalent initializers;
// INTC_VECT/INTC_GROUP and the register descriptor types are external kernel APIs.
static mut vectors: [intc_vect; 55] = [
    INTC_VECT(IRQ0,0x600), INTC_VECT(IRQ1,0x620), INTC_VECT(IRQ2,0x640), INTC_VECT(IRQ3,0x660), INTC_VECT(IRQ4,0x680), INTC_VECT(IRQ5,0x6a0), INTC_VECT(IRQ6,0x6c0), INTC_VECT(IRQ7,0x6e0),
    INTC_VECT(I2C1_ALI,0x780), INTC_VECT(I2C1_TACKI,0x7a0), INTC_VECT(I2C1_WAITI,0x7c0), INTC_VECT(I2C1_DTEI,0x7e0), INTC_VECT(DMAC0,0x800), INTC_VECT(DMAC1,0x820), INTC_VECT(DMAC2,0x840), INTC_VECT(DMAC3,0x860), INTC_VECT(VIO_CEUI,0x880), INTC_VECT(VIO_BEUI,0x8a0), INTC_VECT(VIO_VEUI,0x8c0), INTC_VECT(VOU,0x8e0), INTC_VECT(MFI,0x900), INTC_VECT(VPU,0x980), INTC_VECT(TPU,0x9a0), INTC_VECT(Z3D4,0x9e0), INTC_VECT(USBI0,0xa20), INTC_VECT(USBI1,0xa40), INTC_VECT(MMC_ERR,0xb00), INTC_VECT(MMC_TRAN,0xb20), INTC_VECT(MMC_FSTAT,0xb40), INTC_VECT(MMC_FRDY,0xb60), INTC_VECT(DMAC4,0xb80), INTC_VECT(DMAC5,0xba0), INTC_VECT(DMAC_DADERR,0xbc0), INTC_VECT(KEYSC,0xbe0), INTC_VECT(SCIF,0xc00), INTC_VECT(SCIF1,0xc20), INTC_VECT(SCIF2,0xc40), INTC_VECT(SCIF3,0xc60), INTC_VECT(SIOF0,0xc80), INTC_VECT(SIOF1,0xca0), INTC_VECT(SIO,0xd00), INTC_VECT(FLCTL_FLSTEI,0xd80), INTC_VECT(FLCTL_FLENDI,0xda0), INTC_VECT(FLCTL_FLTREQ0I,0xdc0), INTC_VECT(FLCTL_FLTREQ1I,0xde0), INTC_VECT(I2C0_ALI,0xe00), INTC_VECT(I2C0_TACKI,0xe20), INTC_VECT(I2C0_WAITI,0xe40), INTC_VECT(I2C0_DTEI,0xe60), INTC_VECT(SDHI,0xe80), INTC_VECT(SDHI,0xea0), INTC_VECT(SDHI,0xec0), INTC_VECT(SDHI,0xee0), INTC_VECT(CMT,0xf00), INTC_VECT(TSIF,0xf20), INTC_VECT(SIU,0xf80), INTC_VECT(TMU0,0x400), INTC_VECT(TMU1,0x420), INTC_VECT(TMU2,0x440), INTC_VECT(JPU,0x560), INTC_VECT(LCDC,0x580),
];
static mut groups: [intc_group; 9] = [INTC_GROUP(DMAC0123,DMAC0,DMAC1,DMAC2,DMAC3), INTC_GROUP(VIOVOU,VIO_CEUI,VIO_BEUI,VIO_VEUI,VOU), INTC_GROUP(MMC,MMC_FRDY,MMC_FSTAT,MMC_TRAN,MMC_ERR), INTC_GROUP(DMAC45,DMAC4,DMAC5,DMAC_DADERR), INTC_GROUP(FLCTL,FLCTL_FLSTEI,FLCTL_FLENDI,FLCTL_FLTREQ0I,FLCTL_FLTREQ1I), INTC_GROUP(I2C0,I2C0_ALI,I2C0_TACKI,I2C0_WAITI,I2C0_DTEI), INTC_GROUP(I2C1,I2C1_ALI,I2C1_TACKI,I2C1_WAITI,I2C1_DTEI), INTC_GROUP(SIM,SIM_TEI,SIM_TXI,SIM_RXI,SIM_ERI), INTC_GROUP(USB,USBI0,USBI1)];

// Register arrays preserve the exact addresses, widths, and source ordering.
static mut mask_registers: [intc_mask_reg; 12] = [
    intc_mask_reg { set:0xa4080084, clr:0xa40800c4, width:8, enum_: [VOU,VIO_VEUI,VIO_BEUI,VIO_CEUI,DMAC3,DMAC2,DMAC1,DMAC0] }, intc_mask_reg { set:0xa4080088, clr:0xa40800c8, width:8, enum_: [0,0,0,VPU,0,0,0,MFI] }, intc_mask_reg { set:0xa408008c, clr:0xa40800cc, width:8, enum_: [SIM_TEI,SIM_TXI,SIM_RXI,SIM_ERI,0,0,0,IRDA] }, intc_mask_reg { set:0xa4080090, clr:0xa40800d0, width:8, enum_: [0,TMU2,TMU1,TMU0,JPU,0,0,LCDC] }, intc_mask_reg { set:0xa4080094, clr:0xa40800d4, width:8, enum_: [KEYSC,DMAC_DADERR,DMAC5,DMAC4,SCIF3,SCIF2,SCIF1,SCIF] }, intc_mask_reg { set:0xa4080098, clr:0xa40800d8, width:8, enum_: [0,0,0,SIO,Z3D4,0,SIOF1,SIOF0] }, intc_mask_reg { set:0xa408009c, clr:0xa40800dc, width:8, enum_: [I2C0_DTEI,I2C0_WAITI,I2C0_TACKI,I2C0_ALI,FLCTL_FLTREQ1I,FLCTL_FLTREQ0I,FLCTL_FLENDI,FLCTL_FLSTEI] }, intc_mask_reg { set:0xa40800a0, clr:0xa40800e0, width:8, enum_: [DISABLED,ENABLED,ENABLED,ENABLED,0,0,0,SIU] }, intc_mask_reg { set:0xa40800a4, clr:0xa40800e4, width:8, enum_: [0,0,0,CMT,0,USBI1,USBI0] }, intc_mask_reg { set:0xa40800a8, clr:0xa40800e8, width:8, enum_: [MMC_FRDY,MMC_FSTAT,MMC_TRAN,MMC_ERR] }, intc_mask_reg { set:0xa40800ac, clr:0xa40800ec, width:8, enum_: [I2C1_DTEI,I2C1_WAITI,I2C1_TACKI,I2C1_ALI,TPU,0,0,TSIF] }, intc_mask_reg { set:0xa4140044, clr:0xa4140064, width:8, enum_: [IRQ0,IRQ1,IRQ2,IRQ3,IRQ4,IRQ5,IRQ6,IRQ7] },
];
static mut prio_registers: [intc_prio_reg; 11] = [intc_prio_reg { addr:0xa4080000, width:16, shift:4, enum_:[TMU0,TMU1,TMU2] }, intc_prio_reg { addr:0xa4080004,width:16,shift:4,enum_:[JPU,LCDC,SIM] }, intc_prio_reg { addr:0xa4080010,width:16,shift:4,enum_:[DMAC0123,VIOVOU,MFI,VPU] }, intc_prio_reg { addr:0xa4080014,width:16,shift:4,enum_:[KEYSC,DMAC45,USB,CMT] }, intc_prio_reg { addr:0xa4080018,width:16,shift:4,enum_:[SCIF,SCIF1,SCIF2,SCIF3] }, intc_prio_reg { addr:0xa408001c,width:16,shift:4,enum_:[SIOF0,SIOF1,FLCTL,I2C0] }, intc_prio_reg { addr:0xa4080020,width:16,shift:4,enum_:[SIO,0,TSIF,I2C1] }, intc_prio_reg { addr:0xa4080024,width:16,shift:4,enum_:[Z3D4,0,SIU] }, intc_prio_reg { addr:0xa4080028,width:16,shift:4,enum_:[0,MMC,0,SDHI] }, intc_prio_reg { addr:0xa408002c,width:16,shift:4,enum_:[0,0,TPU] }, intc_prio_reg { addr:0xa4140010,width:32,shift:4,enum_:[IRQ0,IRQ1,IRQ2,IRQ3,IRQ4,IRQ5,IRQ6,IRQ7] }];
static mut sense_registers: [intc_sense_reg; 1] = [intc_sense_reg { addr:0xa414001c, width:16, shift:2, enum_:[IRQ0,IRQ1,IRQ2,IRQ3,IRQ4,IRQ5,IRQ6,IRQ7] }];
static mut ack_registers: [intc_mask_reg; 1] = [intc_mask_reg { set:0xa4140024, clr:0, width:8, enum_:[IRQ0,IRQ1,IRQ2,IRQ3,IRQ4,IRQ5,IRQ6,IRQ7] }];
static mut intc_desc: intc_desc = intc_desc { name:"sh7343", force_enable:ENABLED, force_disable:DISABLED, hw: INTC_HW_DESC!(vectors,groups,mask_registers,prio_registers,sense_registers,ack_registers) };
unsafe fn plat_irq_setup() { register_intc_controller(&mut intc_desc); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
