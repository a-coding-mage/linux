// SPDX-License-Identifier: GPL-2.0
/* SH7770 Setup. Translated from setup-sh7770.c. */

// Kernel headers provide the types, constants, macros, and external functions
// referenced below.

static mut SCIF0_PLATFORM_DATA: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE | SCSCR_TOIE, type_: PORT_SCIF };
static mut SCIF1_PLATFORM_DATA: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE | SCSCR_TOIE, type_: PORT_SCIF };
static mut SCIF2_PLATFORM_DATA: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE | SCSCR_TOIE, type_: PORT_SCIF };
static mut SCIF3_PLATFORM_DATA: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE | SCSCR_TOIE, type_: PORT_SCIF };
static mut SCIF4_PLATFORM_DATA: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE | SCSCR_TOIE, type_: PORT_SCIF };
static mut SCIF5_PLATFORM_DATA: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE | SCSCR_TOIE, type_: PORT_SCIF };
static mut SCIF6_PLATFORM_DATA: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE | SCSCR_TOIE, type_: PORT_SCIF };
static mut SCIF7_PLATFORM_DATA: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE | SCSCR_TOIE, type_: PORT_SCIF };
static mut SCIF8_PLATFORM_DATA: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE | SCSCR_TOIE, type_: PORT_SCIF };
static mut SCIF9_PLATFORM_DATA: plat_sci_port = plat_sci_port { scscr: SCSCR_REIE | SCSCR_TOIE, type_: PORT_SCIF };

macro_rules! scif_device { ($n:expr, $base:expr, $evt:expr, $data:ident) => {
    static mut $data: platform_device = platform_device { name: "sh-sci", id: $n,
        resource: [DEFINE_RES_MEM!($base, 0x100), DEFINE_RES_IRQ!(evt2irq($evt))],
        num_resources: 2, dev: device { platform_data: unsafe { &raw mut SCIF0_PLATFORM_DATA } } };
}; }
// The resource and platform-device declarations retain the original one-to-one layout.
static mut SCIF0_RESOURCES: [resource; 2] = [DEFINE_RES_MEM!(0xff923000, 0x100), DEFINE_RES_IRQ!(evt2irq(0x9a0))];
static mut SCIF1_RESOURCES: [resource; 2] = [DEFINE_RES_MEM!(0xff924000, 0x100), DEFINE_RES_IRQ!(evt2irq(0x9c0))];
static mut SCIF2_RESOURCES: [resource; 2] = [DEFINE_RES_MEM!(0xff925000, 0x100), DEFINE_RES_IRQ!(evt2irq(0x9e0))];
static mut SCIF3_RESOURCES: [resource; 2] = [DEFINE_RES_MEM!(0xff926000, 0x100), DEFINE_RES_IRQ!(evt2irq(0xa00))];
static mut SCIF4_RESOURCES: [resource; 2] = [DEFINE_RES_MEM!(0xff927000, 0x100), DEFINE_RES_IRQ!(evt2irq(0xa20))];
static mut SCIF5_RESOURCES: [resource; 2] = [DEFINE_RES_MEM!(0xff928000, 0x100), DEFINE_RES_IRQ!(evt2irq(0xa40))];
static mut SCIF6_RESOURCES: [resource; 2] = [DEFINE_RES_MEM!(0xff929000, 0x100), DEFINE_RES_IRQ!(evt2irq(0xa60))];
static mut SCIF7_RESOURCES: [resource; 2] = [DEFINE_RES_MEM!(0xff92a000, 0x100), DEFINE_RES_IRQ!(evt2irq(0xa80))];
static mut SCIF8_RESOURCES: [resource; 2] = [DEFINE_RES_MEM!(0xff92b000, 0x100), DEFINE_RES_IRQ!(evt2irq(0xaa0))];
static mut SCIF9_RESOURCES: [resource; 2] = [DEFINE_RES_MEM!(0xff92c000, 0x100), DEFINE_RES_IRQ!(evt2irq(0xac0))];

macro_rules! sci { ($n:expr, $r:ident, $d:ident, $name:ident) => { static mut $name: platform_device = platform_device { name: "sh-sci", id: $n, resource: $r, num_resources: 2, dev: device { platform_data: unsafe { &raw mut $d } } }; }; }
sci!(0, SCIF0_RESOURCES, SCIF0_PLATFORM_DATA, SCIF0_DEVICE);
sci!(1, SCIF1_RESOURCES, SCIF1_PLATFORM_DATA, SCIF1_DEVICE);
sci!(2, SCIF2_RESOURCES, SCIF2_PLATFORM_DATA, SCIF2_DEVICE);
sci!(3, SCIF3_RESOURCES, SCIF3_PLATFORM_DATA, SCIF3_DEVICE);
sci!(4, SCIF4_RESOURCES, SCIF4_PLATFORM_DATA, SCIF4_DEVICE);
sci!(5, SCIF5_RESOURCES, SCIF5_PLATFORM_DATA, SCIF5_DEVICE);
sci!(6, SCIF6_RESOURCES, SCIF6_PLATFORM_DATA, SCIF6_DEVICE);
sci!(7, SCIF7_RESOURCES, SCIF7_PLATFORM_DATA, SCIF7_DEVICE);
sci!(8, SCIF8_RESOURCES, SCIF8_PLATFORM_DATA, SCIF8_DEVICE);
sci!(9, SCIF9_RESOURCES, SCIF9_PLATFORM_DATA, SCIF9_DEVICE);

static mut TMU0_PLATFORM_DATA: sh_timer_config = sh_timer_config { channels_mask: 7 };
static mut TMU1_PLATFORM_DATA: sh_timer_config = sh_timer_config { channels_mask: 7 };
static mut TMU2_PLATFORM_DATA: sh_timer_config = sh_timer_config { channels_mask: 7 };
static mut TMU0_RESOURCES: [resource; 4] = [DEFINE_RES_MEM!(0xffd80000, 0x30), DEFINE_RES_IRQ!(evt2irq(0x400)), DEFINE_RES_IRQ!(evt2irq(0x420)), DEFINE_RES_IRQ!(evt2irq(0x440))];
static mut TMU1_RESOURCES: [resource; 4] = [DEFINE_RES_MEM!(0xffd81000, 0x30), DEFINE_RES_IRQ!(evt2irq(0x460)), DEFINE_RES_IRQ!(evt2irq(0x480)), DEFINE_RES_IRQ!(evt2irq(0x4a0))];
static mut TMU2_RESOURCES: [resource; 4] = [DEFINE_RES_MEM!(0xffd82000, 0x2c), DEFINE_RES_IRQ!(evt2irq(0x4c0)), DEFINE_RES_IRQ!(evt2irq(0x4e0)), DEFINE_RES_IRQ!(evt2irq(0x500))];
macro_rules! tmu { ($n:expr, $r:ident, $d:ident, $name:ident) => { static mut $name: platform_device = platform_device { name: "sh-tmu", id: $n, resource: $r, num_resources: 4, dev: device { platform_data: unsafe { &raw mut $d } } }; }; }
tmu!(0, TMU0_RESOURCES, TMU0_PLATFORM_DATA, TMU0_DEVICE); tmu!(1, TMU1_RESOURCES, TMU1_PLATFORM_DATA, TMU1_DEVICE); tmu!(2, TMU2_RESOURCES, TMU2_PLATFORM_DATA, TMU2_DEVICE);

static mut SH7770_DEVICES: [&'static mut platform_device; 13] = [&mut SCIF0_DEVICE,&mut SCIF1_DEVICE,&mut SCIF2_DEVICE,&mut SCIF3_DEVICE,&mut SCIF4_DEVICE,&mut SCIF5_DEVICE,&mut SCIF6_DEVICE,&mut SCIF7_DEVICE,&mut SCIF8_DEVICE,&mut SCIF9_DEVICE,&mut TMU0_DEVICE,&mut TMU1_DEVICE,&mut TMU2_DEVICE];
#[init]
unsafe fn sh7770_devices_setup() -> i32 { platform_add_devices(SH7770_DEVICES.as_mut_ptr(), SH7770_DEVICES.len()) }
arch_initcall!(sh7770_devices_setup);
#[init]
unsafe fn plat_early_device_setup() { sh_early_platform_add_devices(SH7770_DEVICES.as_mut_ptr(), SH7770_DEVICES.len()); }

enum InterruptSource {
    UNUSED = 0,
    IRL_LLLL, IRL_LLLH, IRL_LLHL, IRL_LLHH, IRL_LHLL, IRL_LHLH, IRL_LHHL, IRL_LHHH,
    IRL_HLLL, IRL_HLLH, IRL_HLHL, IRL_HLHH, IRL_HHLL, IRL_HHLH, IRL_HHHL,
    IRQ0, IRQ1, IRQ2, IRQ3, IRQ4, IRQ5, GPIO, TMU0, TMU1, TMU2, TMU2_TICPI, TMU3,
    TMU4, TMU5, TMU5_TICPI, TMU6, TMU7, TMU8, HAC, IPI, SPDIF, HUDI, I2C,
    DMAC0_DMINT0, DMAC0_DMINT1, DMAC0_DMINT2, I2S0, I2S1, I2S2, I2S3, SRC_RX,
    SRC_TX, SRC_SPDIF, DU, VIDEO_IN, REMOTE, YUV, USB, ATAPI, CAN, GPS, GFX2D,
    GFX3D_MBX, GFX3D_DMAC, EXBUS_ATA, SPI0, SPI1, SCIF089, SCIF1234, SCIF567, ADC,
    BBDMAC_0_3, BBDMAC_4_7, BBDMAC_8_10, BBDMAC_11_14, BBDMAC_15_18, BBDMAC_19_22,
    BBDMAC_23_26, BBDMAC_27, BBDMAC_28, BBDMAC_29, BBDMAC_30, BBDMAC_31,
    TMU, DMAC, I2S, SRC, GFX3D, SPI, SCIF, BBDMAC,
}

static mut VECTORS: [intc_vect; 85] = [
    INTC_VECT!(GPIO,0x3e0), INTC_VECT!(TMU0,0x400), INTC_VECT!(TMU1,0x420), INTC_VECT!(TMU2,0x440), INTC_VECT!(TMU2_TICPI,0x460), INTC_VECT!(TMU3,0x480), INTC_VECT!(TMU4,0x4a0), INTC_VECT!(TMU5,0x4c0), INTC_VECT!(TMU5_TICPI,0x4e0), INTC_VECT!(TMU6,0x500), INTC_VECT!(TMU7,0x520), INTC_VECT!(TMU8,0x540), INTC_VECT!(HAC,0x580), INTC_VECT!(IPI,0x5c0), INTC_VECT!(SPDIF,0x5e0), INTC_VECT!(HUDI,0x600), INTC_VECT!(I2C,0x620), INTC_VECT!(DMAC0_DMINT0,0x640), INTC_VECT!(DMAC0_DMINT1,0x660), INTC_VECT!(DMAC0_DMINT2,0x680), INTC_VECT!(I2S0,0x6a0), INTC_VECT!(I2S1,0x6c0), INTC_VECT!(I2S2,0x6e0), INTC_VECT!(I2S3,0x700), INTC_VECT!(SRC_RX,0x720), INTC_VECT!(SRC_TX,0x740), INTC_VECT!(SRC_SPDIF,0x760), INTC_VECT!(DU,0x780), INTC_VECT!(VIDEO_IN,0x7a0), INTC_VECT!(REMOTE,0x7c0), INTC_VECT!(YUV,0x7e0), INTC_VECT!(USB,0x840), INTC_VECT!(ATAPI,0x860), INTC_VECT!(CAN,0x880), INTC_VECT!(GPS,0x8a0), INTC_VECT!(GFX2D,0x8c0), INTC_VECT!(GFX3D_MBX,0x900), INTC_VECT!(GFX3D_DMAC,0x920), INTC_VECT!(EXBUS_ATA,0x940), INTC_VECT!(SPI0,0x960), INTC_VECT!(SPI1,0x980), INTC_VECT!(SCIF089,0x9a0), INTC_VECT!(SCIF1234,0x9c0), INTC_VECT!(SCIF1234,0x9e0), INTC_VECT!(SCIF1234,0xa00), INTC_VECT!(SCIF1234,0xa20), INTC_VECT!(SCIF567,0xa40), INTC_VECT!(SCIF567,0xa60), INTC_VECT!(SCIF567,0xa80), INTC_VECT!(SCIF089,0xaa0), INTC_VECT!(SCIF089,0xac0), INTC_VECT!(ADC,0xb20),
    INTC_VECT!(BBDMAC_0_3,0xba0), INTC_VECT!(BBDMAC_0_3,0xbc0), INTC_VECT!(BBDMAC_0_3,0xbe0), INTC_VECT!(BBDMAC_0_3,0xc00), INTC_VECT!(BBDMAC_4_7,0xc20), INTC_VECT!(BBDMAC_4_7,0xc40), INTC_VECT!(BBDMAC_4_7,0xc60), INTC_VECT!(BBDMAC_4_7,0xc80), INTC_VECT!(BBDMAC_8_10,0xca0), INTC_VECT!(BBDMAC_8_10,0xcc0), INTC_VECT!(BBDMAC_8_10,0xce0), INTC_VECT!(BBDMAC_11_14,0xd00), INTC_VECT!(BBDMAC_11_14,0xd20), INTC_VECT!(BBDMAC_11_14,0xd40), INTC_VECT!(BBDMAC_11_14,0xd60), INTC_VECT!(BBDMAC_15_18,0xd80), INTC_VECT!(BBDMAC_15_18,0xda0), INTC_VECT!(BBDMAC_15_18,0xdc0), INTC_VECT!(BBDMAC_15_18,0xde0), INTC_VECT!(BBDMAC_19_22,0xe00), INTC_VECT!(BBDMAC_19_22,0xe20), INTC_VECT!(BBDMAC_19_22,0xe40), INTC_VECT!(BBDMAC_19_22,0xe60), INTC_VECT!(BBDMAC_23_26,0xe80), INTC_VECT!(BBDMAC_23_26,0xea0), INTC_VECT!(BBDMAC_23_26,0xec0), INTC_VECT!(BBDMAC_23_26,0xee0), INTC_VECT!(BBDMAC_27,0xf00), INTC_VECT!(BBDMAC_28,0xf20), INTC_VECT!(BBDMAC_29,0xf40), INTC_VECT!(BBDMAC_30,0xf60), INTC_VECT!(BBDMAC_31,0xf80),
];

static mut MASK_REGISTERS: [intc_mask_reg; 1] = [intc_mask_reg { set: 0xffe00040, clr: 0xffe00044, width: 32, fields: [0,BBDMAC,ADC,SCIF,SPI,EXBUS_ATA,GFX3D,GFX2D,GPS,CAN,ATAPI,USB,YUV,REMOTE,VIDEO_IN,DU,SRC,I2S,DMAC,I2C,HUDI,SPDIF,IPI,HAC,TMU,GPIO] }];
static mut PRIO_REGISTERS: [intc_prio_reg; 14] = [
 intc_prio_reg { reg:0xffe00000, shift:0, width:32, field_width:8, fields:[GPIO,TMU0,0,HAC] }, intc_prio_reg {reg:0xffe00004,shift:0,width:32,field_width:8,fields:[IPI,SPDIF,HUDI,I2C]}, intc_prio_reg {reg:0xffe00008,shift:0,width:32,field_width:8,fields:[DMAC,I2S,SRC,DU]}, intc_prio_reg {reg:0xffe0000c,shift:0,width:32,field_width:8,fields:[VIDEO_IN,REMOTE,YUV,USB]}, intc_prio_reg {reg:0xffe00010,shift:0,width:32,field_width:8,fields:[ATAPI,CAN,GPS,GFX2D]}, intc_prio_reg {reg:0xffe00014,shift:0,width:32,field_width:8,fields:[0,GFX3D,EXBUS_ATA,SPI]}, intc_prio_reg {reg:0xffe00018,shift:0,width:32,field_width:8,fields:[SCIF1234,SCIF567,SCIF089]}, intc_prio_reg {reg:0xffe0001c,shift:0,width:32,field_width:8,fields:[ADC,0,0,BBDMAC_0_3]}, intc_prio_reg {reg:0xffe00020,shift:0,width:32,field_width:8,fields:[BBDMAC_4_7,BBDMAC_8_10,BBDMAC_11_14,BBDMAC_15_18]}, intc_prio_reg {reg:0xffe00024,shift:0,width:32,field_width:8,fields:[BBDMAC_19_22,BBDMAC_23_26,BBDMAC_27,BBDMAC_28]}, intc_prio_reg {reg:0xffe00028,shift:0,width:32,field_width:8,fields:[BBDMAC_29,BBDMAC_30,BBDMAC_31]}, intc_prio_reg {reg:0xffe0002c,shift:0,width:32,field_width:8,fields:[TMU1,TMU2,TMU2_TICPI,TMU3]}, intc_prio_reg {reg:0xffe00030,shift:0,width:32,field_width:8,fields:[TMU4,TMU5,TMU5_TICPI,TMU6]}, intc_prio_reg {reg:0xffe00034,shift:0,width:32,field_width:8,fields:[TMU7,TMU8]},
];
static mut INTC_DESC: intc_desc = DECLARE_INTC_DESC!("sh7770", VECTORS, [], MASK_REGISTERS, PRIO_REGISTERS, None);

static mut IRQ_VECTORS: [intc_vect; 6] = [INTC_VECT!(IRQ0,0x240),INTC_VECT!(IRQ1,0x280),INTC_VECT!(IRQ2,0x2c0),INTC_VECT!(IRQ3,0x300),INTC_VECT!(IRQ4,0x340),INTC_VECT!(IRQ5,0x380)];
static mut IRQ_MASK_REGISTERS: [intc_mask_reg; 1] = [intc_mask_reg {set:0xffd00044,clr:0xffd00064,width:32,fields:[IRQ0,IRQ1,IRQ2,IRQ3,IRQ4,IRQ5]}];
static mut IRQ_PRIO_REGISTERS: [intc_prio_reg; 1] = [intc_prio_reg {reg:0xffd00010,shift:0,width:32,field_width:4,fields:[IRQ0,IRQ1,IRQ2,IRQ3,IRQ4,IRQ5]}];
static mut IRQ_SENSE_REGISTERS: [intc_sense_reg; 1] = [intc_sense_reg {reg:0xffd0001c,width:32,field_width:2,fields:[IRQ0,IRQ1,IRQ2,IRQ3,IRQ4,IRQ5]}];
static mut INTC_IRQ_DESC: intc_desc = DECLARE_INTC_DESC!("sh7770-irq", IRQ_VECTORS, None, IRQ_MASK_REGISTERS, IRQ_PRIO_REGISTERS, IRQ_SENSE_REGISTERS);

static mut IRL_VECTORS: [intc_vect; 15] = [INTC_VECT!(IRL_LLLL,0x200),INTC_VECT!(IRL_LLLH,0x220),INTC_VECT!(IRL_LLHL,0x240),INTC_VECT!(IRL_LLHH,0x260),INTC_VECT!(IRL_LHLL,0x280),INTC_VECT!(IRL_LHLH,0x2a0),INTC_VECT!(IRL_LHHL,0x2c0),INTC_VECT!(IRL_LHHH,0x2e0),INTC_VECT!(IRL_HLLL,0x300),INTC_VECT!(IRL_HLLH,0x320),INTC_VECT!(IRL_HLHL,0x340),INTC_VECT!(IRL_HLHH,0x360),INTC_VECT!(IRL_HHLL,0x380),INTC_VECT!(IRL_HHLH,0x3a0),INTC_VECT!(IRL_HHHL,0x3c0)];
static mut IRL3210_MASK_REGISTERS: [intc_mask_reg; 1] = [intc_mask_reg {set:0xffd40080,clr:0xffd40084,width:32,fields:[IRL_LLLL,IRL_LLLH,IRL_LLHL,IRL_LLHH,IRL_LHLL,IRL_LHLH,IRL_LHHL,IRL_LHHH,IRL_HLLL,IRL_HLLH,IRL_HLHL,IRL_HLHH,IRL_HHLL,IRL_HHLH,IRL_HHHL]}];
static mut IRL7654_MASK_REGISTERS: [intc_mask_reg; 1] = [intc_mask_reg {set:0xffd40080,clr:0xffd40084,width:32,fields:[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,IRL_LLLL,IRL_LLLH,IRL_LLHL,IRL_LLHH,IRL_LHLL,IRL_LHLH,IRL_LHHL,IRL_LHHH,IRL_HLLL,IRL_HLLH,IRL_HLHL,IRL_HLHH,IRL_HHLL,IRL_HHLH,IRL_HHHL]}];
static mut INTC_IRL7654_DESC: intc_desc = DECLARE_INTC_DESC!("sh7780-irl7654", IRL_VECTORS, None, IRL7654_MASK_REGISTERS, None, None);
static mut INTC_IRL3210_DESC: intc_desc = DECLARE_INTC_DESC!("sh7780-irl3210", IRL_VECTORS, None, IRL3210_MASK_REGISTERS, None, None);

const INTC_ICR0: usize = 0xffd00000; const INTC_INTMSK0: usize = 0xffd00044; const INTC_INTMSK1: usize = 0xffd00048; const INTC_INTMSK2: usize = 0xffd40080; const INTC_INTMSKCLR1: usize = 0xffd00068; const INTC_INTMSKCLR2: usize = 0xffd40084;
#[init] unsafe fn plat_irq_setup() { __raw_writel(0xff000000,INTC_INTMSK0); __raw_writel(0xc0000000,INTC_INTMSK1); __raw_writel(0xfffefffe,INTC_INTMSK2); __raw_writel(__raw_readl(INTC_ICR0)&!0x00c00000,INTC_ICR0); __raw_writel(__raw_readl(INTC_ICR0)|0x00200000,INTC_ICR0); register_intc_controller(&raw mut INTC_DESC); }
#[init] unsafe fn plat_irq_setup_pins(mode: i32) { match mode { IRQ_MODE_IRQ => { __raw_writel(__raw_readl(INTC_ICR0)|0x00c00000,INTC_ICR0); register_intc_controller(&raw mut INTC_IRQ_DESC); }, IRQ_MODE_IRL7654 => { __raw_writel(0x40000000,INTC_INTMSKCLR1); __raw_writel(0x0000fffe,INTC_INTMSKCLR2); }, IRQ_MODE_IRL3210 => { __raw_writel(0x80000000,INTC_INTMSKCLR1); __raw_writel(0xfffe0000,INTC_INTMSKCLR2); }, IRQ_MODE_IRL7654_MASK => { __raw_writel(0x40000000,INTC_INTMSKCLR1); register_intc_controller(&raw mut INTC_IRL7654_DESC); }, IRQ_MODE_IRL3210_MASK => { __raw_writel(0x80000000,INTC_INTMSKCLR1); register_intc_controller(&raw mut INTC_IRL3210_DESC); }, _ => BUG!(), } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
