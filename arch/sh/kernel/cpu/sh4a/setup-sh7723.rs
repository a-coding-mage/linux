// SPDX-License-Identifier: GPL-2.0
/* SH7723 Setup -- direct low-level translation of setup-sh7723.c. */

// C headers and kernel-provided symbols are external dependencies.

static mut scif0_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, r#type: PORT_SCIF, regtype: SCIx_SH4_SCIF_NO_SCSPTR_REGTYPE };
static mut scif1_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, r#type: PORT_SCIF, regtype: SCIx_SH4_SCIF_NO_SCSPTR_REGTYPE };
static mut scif2_platform_data: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE, r#type: PORT_SCIF, regtype: SCIx_SH4_SCIF_NO_SCSPTR_REGTYPE };
static mut scif3_platform_data: plat_sci_port = plat_sci_port { sampling_rate: 8, r#type: PORT_SCIFA };
static mut scif4_platform_data: plat_sci_port = plat_sci_port { sampling_rate: 8, r#type: PORT_SCIFA };
static mut scif5_platform_data: plat_sci_port = plat_sci_port { sampling_rate: 8, r#type: PORT_SCIFA };

macro_rules! res_mem { ($a:expr, $s:expr) => { DEFINE_RES_MEM($a, $s) }; }
macro_rules! res_irq { ($a:expr) => { DEFINE_RES_IRQ(evt2irq($a)) }; }
macro_rules! sci_device { ($n:ident, $id:expr, $mem:expr, $irq:expr, $pd:ident) => {
    static mut $n: platform_device = platform_device { name: "sh-sci", id: $id, resource: &[$mem, $irq], num_resources: 2, platform_data: &raw mut $pd };
} }
sci_device!(scif0_device, 0, res_mem!(0xffe00000, 0x100), res_irq!(0xc00), scif0_platform_data);
sci_device!(scif1_device, 1, res_mem!(0xffe10000, 0x100), res_irq!(0xc20), scif1_platform_data);
sci_device!(scif2_device, 2, res_mem!(0xffe20000, 0x100), res_irq!(0xc40), scif2_platform_data);
sci_device!(scif3_device, 3, res_mem!(0xa4e30000, 0x100), res_irq!(0x900), scif3_platform_data);
sci_device!(scif4_device, 4, res_mem!(0xa4e40000, 0x100), res_irq!(0xd00), scif4_platform_data);
sci_device!(scif5_device, 5, res_mem!(0xa4e50000, 0x100), res_irq!(0xfa0), scif5_platform_data);

static mut vpu_platform_data: uio_info = uio_info { name: "VPU5", version: "0", irq: evt2irq(0x980) };
static mut veu0_platform_data: uio_info = uio_info { name: "VEU2H", version: "0", irq: evt2irq(0x8c0) };
static mut veu1_platform_data: uio_info = uio_info { name: "VEU2H", version: "0", irq: evt2irq(0x560) };
static mut vpu_resources: [resource; 2] = [resource { name: "VPU", start: 0xfe900000, end: 0xfe902807, flags: IORESOURCE_MEM }, resource::default()];
static mut veu0_resources: [resource; 2] = [resource { name: "VEU2H0", start: 0xfe920000, end: 0xfe92027b, flags: IORESOURCE_MEM }, resource::default()];
static mut veu1_resources: [resource; 2] = [resource { name: "VEU2H1", start: 0xfe924000, end: 0xfe92427b, flags: IORESOURCE_MEM }, resource::default()];
static mut vpu_device: platform_device = uio_dev!("uio_pdrv_genirq", 0, vpu_platform_data, vpu_resources);
static mut veu0_device: platform_device = uio_dev!("uio_pdrv_genirq", 1, veu0_platform_data, veu0_resources);
static mut veu1_device: platform_device = uio_dev!("uio_pdrv_genirq", 2, veu1_platform_data, veu1_resources);

static mut cmt_platform_data: sh_timer_config = sh_timer_config { channels_mask: 0x20 };
static mut tmu0_platform_data: sh_timer_config = sh_timer_config { channels_mask: 7 };
static mut tmu1_platform_data: sh_timer_config = sh_timer_config { channels_mask: 7 };
static mut cmt_device: platform_device = timer_dev!("sh-cmt-32", 0, cmt_platform_data, &[res_mem!(0x044a0000, 0x70), res_irq!(0xf00)]);
static mut tmu0_device: platform_device = timer_dev!("sh-tmu", 0, tmu0_platform_data, &[res_mem!(0xffd80000, 0x2c), res_irq!(0x400), res_irq!(0x420), res_irq!(0x440)]);
static mut tmu1_device: platform_device = timer_dev!("sh-tmu", 1, tmu1_platform_data, &[res_mem!(0xffd90000, 0x2c), res_irq!(0x920), res_irq!(0x940), res_irq!(0x960)]);

static mut rtc_resources: [resource; 4] = [resource { start: 0xa465fec0, end: 0xa465fec0 + 0x58 - 1, flags: IORESOURCE_IO }, resource { start: evt2irq(0xaa0), flags: IORESOURCE_IRQ }, resource { start: evt2irq(0xac0), flags: IORESOURCE_IRQ }, resource { start: evt2irq(0xa80), flags: IORESOURCE_IRQ }];
static mut rtc_device: platform_device = device!("sh-rtc", -1, rtc_resources);
static mut r8a66597_data: r8a66597_platdata = r8a66597_platdata { on_chip: 1 };
static mut sh7723_usb_host_resources: [resource; 2] = [resource { start: 0xa4d80000, end: 0xa4d800ff, flags: IORESOURCE_MEM }, resource { start: evt2irq(0xa20), end: evt2irq(0xa20), flags: IORESOURCE_IRQ | IRQF_TRIGGER_LOW }];
static mut sh7723_usb_host_device: platform_device = device_with_data!("r8a66597_hcd", 0, sh7723_usb_host_resources, r8a66597_data, 0xffffffff);
static mut iic_resources: [resource; 2] = [resource { name: "IIC", start: 0x04470000, end: 0x04470017, flags: IORESOURCE_MEM }, resource { start: evt2irq(0xe00), end: evt2irq(0xe60), flags: IORESOURCE_IRQ }];
static mut iic_device: platform_device = device!("i2c-sh_mobile", 0, iic_resources);

static mut sh7723_devices: [&'static mut platform_device; 15] = [&mut scif0_device,&mut scif1_device,&mut scif2_device,&mut scif3_device,&mut scif4_device,&mut scif5_device,&mut cmt_device,&mut tmu0_device,&mut tmu1_device,&mut rtc_device,&mut iic_device,&mut sh7723_usb_host_device,&mut vpu_device,&mut veu0_device,&mut veu1_device];
fn sh7723_devices_setup() -> i32 { unsafe { platform_resource_setup_memory(&mut vpu_device, "vpu", 2 << 20); platform_resource_setup_memory(&mut veu0_device, "veu0", 2 << 20); platform_resource_setup_memory(&mut veu1_device, "veu1", 2 << 20); platform_add_devices(sh7723_devices.as_mut_ptr(), sh7723_devices.len()) } }
arch_initcall!(sh7723_devices_setup);
static mut sh7723_early_devices: [&'static mut platform_device; 9] = [&mut scif0_device,&mut scif1_device,&mut scif2_device,&mut scif3_device,&mut scif4_device,&mut scif5_device,&mut cmt_device,&mut tmu0_device,&mut tmu1_device];
fn plat_early_device_setup() { unsafe { sh_early_platform_add_devices(sh7723_early_devices.as_mut_ptr(), sh7723_early_devices.len()); } }

const RAMCR_CACHE_L2FC: u32 = 0x0002; const RAMCR_CACHE_L2E: u32 = 0x0001; const L2_CACHE_ENABLE: u32 = RAMCR_CACHE_L2E | RAMCR_CACHE_L2FC;
fn l2_cache_init() { unsafe { __raw_writel(L2_CACHE_ENABLE, RAMCR); } }

// Interrupt source numbering and controller tables retain the original C ordering.
enum Interrupt { UNUSED=0, ENABLED, DISABLED, IRQ0,IRQ1,IRQ2,IRQ3,IRQ4,IRQ5,IRQ6,IRQ7, HUDI, DMAC1A_DEI0,DMAC1A_DEI1,DMAC1A_DEI2,DMAC1A_DEI3, _2DG_TRI,_2DG_INI,_2DG_CEI, DMAC0A_DEI0,DMAC0A_DEI1,DMAC0A_DEI2,DMAC0A_DEI3, VIO_CEUI,VIO_BEUI,VIO_VEU2HI,VIO_VOUI, SCIFA_SCIFA0,VPU_VPUI,TPU_TPUI,ADC_ADI,USB_USI0,RTC_ATI,RTC_PRI,RTC_CUI,DMAC1B_DEI4,DMAC1B_DEI5,DMAC1B_DADERR,DMAC0B_DEI4,DMAC0B_DEI5,DMAC0B_DADERR,KEYSC_KEYI,SCIF_SCIF0,SCIF_SCIF1,SCIF_SCIF2,MSIOF_MSIOFI0,MSIOF_MSIOFI1,SCIFA_SCIFA1,FLCTL_FLSTEI,FLCTL_FLTENDI,FLCTL_FLTREQ0I,FLCTL_FLTREQ1I,I2C_ALI,I2C_TACKI,I2C_WAITI,I2C_DTEI,CMT_CMTI,TSIF_TSIFI,SIU_SIUI,SCIFA_SCIFA2,TMU0_TUNI0,TMU0_TUNI1,TMU0_TUNI2,IRDA_IRDAI,ATAPI_ATAPII,VEU2H1_VEU2HI,LCDC_LCDCI,TMU1_TUNI0,TMU1_TUNI1,TMU1_TUNI2,DMAC1A,DMAC0A,VIO,DMAC0B,FLCTL,I2C,_2DG,SDHI1,RTC,DMAC1B,SDHI0 }
static mut vectors: &[intc_vect] = &[
    INTC_VECT!(IRQ0,0x600),INTC_VECT!(IRQ1,0x620),INTC_VECT!(IRQ2,0x640),INTC_VECT!(IRQ3,0x660),INTC_VECT!(IRQ4,0x680),INTC_VECT!(IRQ5,0x6a0),INTC_VECT!(IRQ6,0x6c0),INTC_VECT!(IRQ7,0x6e0),
    INTC_VECT!(DMAC1A_DEI0,0x700),INTC_VECT!(DMAC1A_DEI1,0x720),INTC_VECT!(DMAC1A_DEI2,0x740),INTC_VECT!(DMAC1A_DEI3,0x760),INTC_VECT!(_2DG_TRI,0x780),INTC_VECT!(_2DG_INI,0x7a0),INTC_VECT!(_2DG_CEI,0x7c0),INTC_VECT!(DMAC0A_DEI0,0x800),INTC_VECT!(DMAC0A_DEI1,0x820),INTC_VECT!(DMAC0A_DEI2,0x840),INTC_VECT!(DMAC0A_DEI3,0x860),INTC_VECT!(VIO_CEUI,0x880),INTC_VECT!(VIO_BEUI,0x8a0),INTC_VECT!(VIO_VEU2HI,0x8c0),INTC_VECT!(VIO_VOUI,0x8e0),INTC_VECT!(SCIFA_SCIFA0,0x900),INTC_VECT!(VPU_VPUI,0x980),INTC_VECT!(TPU_TPUI,0x9a0),INTC_VECT!(ADC_ADI,0x9e0),INTC_VECT!(USB_USI0,0xa20),INTC_VECT!(RTC_ATI,0xa80),INTC_VECT!(RTC_PRI,0xaa0),INTC_VECT!(RTC_CUI,0xac0),INTC_VECT!(DMAC1B_DEI4,0xb00),INTC_VECT!(DMAC1B_DEI5,0xb20),INTC_VECT!(DMAC1B_DADERR,0xb40),INTC_VECT!(DMAC0B_DEI4,0xb80),INTC_VECT!(DMAC0B_DEI5,0xba0),INTC_VECT!(DMAC0B_DADERR,0xbc0),INTC_VECT!(KEYSC_KEYI,0xbe0),INTC_VECT!(SCIF_SCIF0,0xc00),INTC_VECT!(SCIF_SCIF1,0xc20),INTC_VECT!(SCIF_SCIF2,0xc40),INTC_VECT!(MSIOF_MSIOFI0,0xc80),INTC_VECT!(MSIOF_MSIOFI1,0xca0),INTC_VECT!(SCIFA_SCIFA1,0xd00),INTC_VECT!(FLCTL_FLSTEI,0xd80),INTC_VECT!(FLCTL_FLTENDI,0xda0),INTC_VECT!(FLCTL_FLTREQ0I,0xdc0),INTC_VECT!(FLCTL_FLTREQ1I,0xde0),INTC_VECT!(I2C_ALI,0xe00),INTC_VECT!(I2C_TACKI,0xe20),INTC_VECT!(I2C_WAITI,0xe40),INTC_VECT!(I2C_DTEI,0xe60),INTC_VECT!(SDHI0,0xe80),INTC_VECT!(SDHI0,0xea0),INTC_VECT!(SDHI0,0xec0),INTC_VECT!(CMT_CMTI,0xf00),INTC_VECT!(TSIF_TSIFI,0xf20),INTC_VECT!(SIU_SIUI,0xf80),INTC_VECT!(SCIFA_SCIFA2,0xfa0),INTC_VECT!(TMU0_TUNI0,0x400),INTC_VECT!(TMU0_TUNI1,0x420),INTC_VECT!(TMU0_TUNI2,0x440),INTC_VECT!(IRDA_IRDAI,0x480),INTC_VECT!(ATAPI_ATAPII,0x4a0),INTC_VECT!(SDHI1,0x4e0),INTC_VECT!(SDHI1,0x500),INTC_VECT!(SDHI1,0x520),INTC_VECT!(VEU2H1_VEU2HI,0x560),INTC_VECT!(LCDC_LCDCI,0x580),INTC_VECT!(TMU1_TUNI0,0x920),INTC_VECT!(TMU1_TUNI1,0x940),INTC_VECT!(TMU1_TUNI2,0x960)];

// Groups, masks, priorities, sense and acknowledge registers are represented by
// the corresponding external INTC constructors, preserving source order.
static mut groups: &[intc_group] = &[INTC_GROUP!(DMAC1A,DMAC1A_DEI0,DMAC1A_DEI1,DMAC1A_DEI2,DMAC1A_DEI3),INTC_GROUP!(DMAC0A,DMAC0A_DEI0,DMAC0A_DEI1,DMAC0A_DEI2,DMAC0A_DEI3),INTC_GROUP!(VIO,VIO_CEUI,VIO_BEUI,VIO_VEU2HI,VIO_VOUI),INTC_GROUP!(DMAC0B,DMAC0B_DEI4,DMAC0B_DEI5,DMAC0B_DADERR),INTC_GROUP!(FLCTL,FLCTL_FLSTEI,FLCTL_FLTENDI,FLCTL_FLTREQ0I,FLCTL_FLTREQ1I),INTC_GROUP!(I2C,I2C_ALI,I2C_TACKI,I2C_WAITI,I2C_DTEI),INTC_GROUP!(_2DG,_2DG_TRI,_2DG_INI,_2DG_CEI),INTC_GROUP!(RTC,RTC_ATI,RTC_PRI,RTC_CUI),INTC_GROUP!(DMAC1B,DMAC1B_DEI4,DMAC1B_DEI5,DMAC1B_DADERR)];
static mut mask_registers: &[intc_mask_reg] = &[
    INTC_MASK!(0xa4080080,0xa40800c0,8,0,TMU1_TUNI2,TMU1_TUNI1,TMU1_TUNI0,0,ENABLED,ENABLED,ENABLED), INTC_MASK!(0xa4080084,0xa40800c4,8,VIO_VOUI,VIO_VEU2HI,VIO_BEUI,VIO_CEUI,DMAC0A_DEI3,DMAC0A_DEI2,DMAC0A_DEI1,DMAC0A_DEI0), INTC_MASK!(0xa4080088,0xa40800c8,8,0,0,0,VPU_VPUI,0,0,0,SCIFA_SCIFA0), INTC_MASK!(0xa408008c,0xa40800cc,8,DMAC1A_DEI3,DMAC1A_DEI2,DMAC1A_DEI1,DMAC1A_DEI0,0,0,0,IRDA_IRDAI), INTC_MASK!(0xa4080090,0xa40800d0,8,0,TMU0_TUNI2,TMU0_TUNI1,TMU0_TUNI0,VEU2H1_VEU2HI,0,0,LCDC_LCDCI), INTC_MASK!(0xa4080094,0xa40800d4,8,KEYSC_KEYI,DMAC0B_DADERR,DMAC0B_DEI5,DMAC0B_DEI4,0,SCIF_SCIF2,SCIF_SCIF1,SCIF_SCIF0), INTC_MASK!(0xa4080098,0xa40800d8,8,0,0,0,SCIFA_SCIFA1,ADC_ADI,0,MSIOF_MSIOFI1,MSIOF_MSIOFI0), INTC_MASK!(0xa408009c,0xa40800dc,8,I2C_DTEI,I2C_WAITI,I2C_TACKI,I2C_ALI,FLCTL_FLTREQ1I,FLCTL_FLTREQ0I,FLCTL_FLTENDI,FLCTL_FLSTEI), INTC_MASK!(0xa40800a0,0xa40800e0,8,0,ENABLED,ENABLED,ENABLED,0,0,SCIFA_SCIFA2,SIU_SIUI), INTC_MASK!(0xa40800a4,0xa40800e4,8,0,0,0,CMT_CMTI,0,0,USB_USI0,0), INTC_MASK!(0xa40800a8,0xa40800e8,8,0,DMAC1B_DADERR,DMAC1B_DEI5,DMAC1B_DEI4,0,RTC_ATI,RTC_PRI,RTC_CUI), INTC_MASK!(0xa40800ac,0xa40800ec,8,0,_2DG_CEI,_2DG_INI,_2DG_TRI,0,TPU_TPUI,0,TSIF_TSIFI), INTC_MASK!(0xa40800b0,0xa40800f0,8,0,0,0,0,0,0,0,ATAPI_ATAPII), INTC_MASK!(0xa4140044,0xa4140064,8,IRQ0,IRQ1,IRQ2,IRQ3,IRQ4,IRQ5,IRQ6,IRQ7)];
static mut prio_registers: &[intc_prio_reg] = &[INTC_PRIO!(0xa4080000,0,16,4,TMU0_TUNI0,TMU0_TUNI1,TMU0_TUNI2,IRDA_IRDAI),INTC_PRIO!(0xa4080004,0,16,4,VEU2H1_VEU2HI,LCDC_LCDCI,DMAC1A,0),INTC_PRIO!(0xa4080008,0,16,4,TMU1_TUNI0,TMU1_TUNI1,TMU1_TUNI2,0),INTC_PRIO!(0xa408000c,0,16,4),INTC_PRIO!(0xa4080010,0,16,4,DMAC0A,VIO,SCIFA_SCIFA0,VPU_VPUI),INTC_PRIO!(0xa4080014,0,16,4,KEYSC_KEYI,DMAC0B,USB_USI0,CMT_CMTI),INTC_PRIO!(0xa4080018,0,16,4,SCIF_SCIF0,SCIF_SCIF1,SCIF_SCIF2,0),INTC_PRIO!(0xa408001c,0,16,4,MSIOF_MSIOFI0,MSIOF_MSIOFI1,FLCTL,I2C),INTC_PRIO!(0xa4080020,0,16,4,SCIFA_SCIFA1,0,TSIF_TSIFI,_2DG),INTC_PRIO!(0xa4080024,0,16,4,ADC_ADI,0,SIU_SIUI,SDHI1),INTC_PRIO!(0xa4080028,0,16,4,RTC,DMAC1B,0,SDHI0),INTC_PRIO!(0xa408002c,0,16,4,SCIFA_SCIFA2,0,TPU_TPUI,ATAPI_ATAPII),INTC_PRIO!(0xa4140010,0,32,4,IRQ0,IRQ1,IRQ2,IRQ3,IRQ4,IRQ5,IRQ6,IRQ7)];
static mut sense_registers: &[intc_sense_reg] = &[INTC_SENSE!(0xa414001c,16,2,IRQ0,IRQ1,IRQ2,IRQ3,IRQ4,IRQ5,IRQ6,IRQ7)];
static mut ack_registers: &[intc_mask_reg] = &[INTC_ACK!(0xa4140024,0,8,IRQ0,IRQ1,IRQ2,IRQ3,IRQ4,IRQ5,IRQ6,IRQ7)];
static mut intc_desc: intc_desc = intc_desc { name: "sh7723", force_enable: ENABLED, force_disable: DISABLED, hw: INTC_HW_DESC!(vectors,groups,mask_registers,prio_registers,sense_registers,ack_registers) };
fn plat_irq_setup() { unsafe { register_intc_controller(&mut intc_desc); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
