/* SPDX-License-Identifier: GPL-2.0 */
// C header dependencies are supplied by the surrounding kernel translation.

extern "C" {
    pub static mut saa7146_debug: ::core::ffi::c_uint;
}

#[repr(C)]
pub struct saa7146_pgtable { pub size: u32, pub cpu: *mut u32, pub dma: dma_addr_t, pub offset: usize, pub slist: *mut scatterlist, pub nents: i32 }
#[repr(C)] pub struct saa7146_pci_extension_data { pub ext: *mut saa7146_extension, pub ext_priv: *mut core::ffi::c_void }
#[repr(C)] pub struct saa7146_extension {
    pub name: [core::ffi::c_char; 32], pub flags: i32, pub module: *mut module, pub driver: pci_driver,
    pub pci_tbl: *const pci_device_id, pub probe: Option<unsafe extern "C" fn(*mut saa7146_dev)->i32>,
    pub attach: Option<unsafe extern "C" fn(*mut saa7146_dev,*mut saa7146_pci_extension_data)->i32>,
    pub detach: Option<unsafe extern "C" fn(*mut saa7146_dev)->i32>, pub irq_mask: u32,
    pub irq_func: Option<unsafe extern "C" fn(*mut saa7146_dev,*mut u32)>,
}
#[repr(C)] pub struct saa7146_dma { pub dma_handle: dma_addr_t, pub cpu_addr: *mut u32 }
#[repr(C)] pub struct saa7146_dev {
    pub module: *mut module, pub v4l2_dev: v4l2_device, pub ctrl_handler: v4l2_ctrl_handler,
    pub slock: spinlock_t, pub v4l2_lock: mutex, pub mem: *mut u8, pub revision: u32,
    pub name: [core::ffi::c_char;32], pub pci: *mut pci_dev, pub int_todo: u32, pub int_slock: spinlock_t,
    pub ext: *mut saa7146_extension, pub ext_priv: *mut core::ffi::c_void, pub ext_vv_data: *mut saa7146_ext_vv,
    pub vv_data: *mut saa7146_vv, pub vv_callback: Option<unsafe extern "C" fn(*mut saa7146_dev,usize)>,
    pub i2c_lock: mutex, pub i2c_bitrate: u32, pub d_i2c: saa7146_dma, pub i2c_wq: wait_queue_head_t,
    pub i2c_op: i32, pub d_rps0: saa7146_dma, pub d_rps1: saa7146_dma,
}
extern "C" { pub fn saa7146_i2c_adapter_prepare(*mut saa7146_dev,*mut i2c_adapter,u32)->i32; pub fn saa7146_register_extension(*mut saa7146_extension)->i32; pub fn saa7146_unregister_extension(*mut saa7146_extension)->i32; pub fn saa7146_format_by_fourcc(*mut saa7146_dev,i32)->*mut saa7146_format; pub fn saa7146_pgtable_alloc(*mut pci_dev,*mut saa7146_pgtable)->i32; pub fn saa7146_pgtable_free(*mut pci_dev,*mut saa7146_pgtable); pub fn saa7146_pgtable_build_single(*mut pci_dev,*mut saa7146_pgtable,*mut scatterlist,i32)->i32; pub fn saa7146_vmalloc_build_pgtable(*mut pci_dev,isize,*mut saa7146_pgtable)->*mut core::ffi::c_void; pub fn saa7146_vfree_destroy_pgtable(*mut pci_dev,*mut core::ffi::c_void,*mut saa7146_pgtable); pub fn saa7146_setgpio(*mut saa7146_dev,i32,u32); pub fn saa7146_wait_for_debi_done(*mut saa7146_dev,i32)->i32; }

pub const SAA7146_USE_I2C_IRQ:i32=1; pub const SAA7146_I2C_SHORT_DELAY:i32=2;
pub const SAA7146_I2C_MEM:usize=PAGE_SIZE; pub const SAA7146_RPS_MEM:usize=PAGE_SIZE;
pub const SAA7146_I2C_TIMEOUT:u32=100; pub const SAA7146_I2C_RETRIES:u32=3; pub const SAA7146_I2C_DELAY:u32=5;
pub const ME1:u32=0x800; pub const PV1:u32=8; pub const DEBINOSWAP:u32=0xe0000;
pub const SAA7146_GPIO_INPUT:u32=0; pub const SAA7146_GPIO_IRQHI:u32=0x10; pub const SAA7146_GPIO_IRQLO:u32=0x20; pub const SAA7146_GPIO_IRQHL:u32=0x30; pub const SAA7146_GPIO_OUTLO:u32=0x40; pub const SAA7146_GPIO_OUTHI:u32=0x50;
pub const CMD_NOP:u32=0; pub const CMD_CLR_EVENT:u32=0; pub const CMD_SET_EVENT:u32=0x10000000; pub const CMD_PAUSE:u32=0x20000000; pub const CMD_CHECK_LATE:u32=0x30000000; pub const CMD_UPLOAD:u32=0x40000000; pub const CMD_STOP:u32=0x50000000; pub const CMD_INTERRUPT:u32=0x60000000; pub const CMD_JUMP:u32=0x80000000; pub const CMD_WR_REG:u32=0x90000000; pub const CMD_RD_REG:u32=0xa0000000; pub const CMD_WR_REG_MASK:u32=0xc0000000;
pub const EVT_HS:u32=1<<15; pub const EVT_VBI_B:u32=1<<9; pub const RPS_OAN:u32=1<<27; pub const RPS_INV:u32=1<<26; pub const GPIO3_MSK:u32=0xff000000;
pub const MASK_NONE:u32=0; pub const MASK_PA:u32=0xfffffffc; pub const MASK_PR:u32=0xfffffffe; pub const MASK_ER:u32=0xffffffff;

// Register aliases.
pub const BASE_ODD1:u32=0x00; pub const BASE_EVEN1:u32=0x04; pub const PROT_ADDR1:u32=0x08; pub const PITCH1:u32=0x0c; pub const BASE_PAGE1:u32=0x10; pub const NUM_LINE_BYTE1:u32=0x14;
pub const BASE_ODD2:u32=0x18; pub const BASE_EVEN2:u32=0x1c; pub const PROT_ADDR2:u32=0x20; pub const PITCH2:u32=0x24; pub const BASE_PAGE2:u32=0x28; pub const NUM_LINE_BYTE2:u32=0x2c;
pub const BASE_ODD3:u32=0x30; pub const BASE_EVEN3:u32=0x34; pub const PROT_ADDR3:u32=0x38; pub const PITCH3:u32=0x3c; pub const BASE_PAGE3:u32=0x40; pub const NUM_LINE_BYTE3:u32=0x44;
pub const IER:u32=0xdc; pub const GPIO_CTRL:u32=0xe0; pub const MC1:u32=0xfc; pub const MC2:u32=0x100; pub const RPS_ADDR0:u32=0x104; pub const RPS_ADDR1:u32=0x108; pub const ISR:u32=0x10c; pub const PSR:u32=0x110; pub const SSR:u32=0x114;
pub const SPCI_PPEF:u32=0x80000000; pub const SPCI_PABO:u32=0x40000000; pub const SPCI_PPED:u32=0x20000000; pub const SPCI_RPS_I1:u32=0x10000000; pub const SPCI_RPS_I0:u32=0x08000000; pub const SPCI_DEBI_S:u32=0x80000; pub const SPCI_DEBI_E:u32=0x40000; pub const SPCI_IIC_S:u32=0x20000; pub const SPCI_IIC_E:u32=0x10000; pub const SPCI_VFOU:u32=0x200; pub const SPCI_FIDA:u32=0x100; pub const SPCI_FIDB:u32=0x80; pub const SPCI_PIN3:u32=0x40; pub const SPCI_PIN2:u32=0x20; pub const SPCI_PIN1:u32=0x10; pub const SPCI_PIN0:u32=8; pub const SPCI_ECS:u32=4; pub const SPCI_EC3S:u32=2; pub const SPCI_EC0S:u32=1;
pub const SAA7146_I2C_ABORT:u32=1<<7; pub const SAA7146_I2C_SPERR:u32=1<<6; pub const SAA7146_I2C_APERR:u32=1<<5; pub const SAA7146_I2C_DTERR:u32=1<<4; pub const SAA7146_I2C_DRERR:u32=1<<3; pub const SAA7146_I2C_AL:u32=1<<2; pub const SAA7146_I2C_ERR:u32=1<<1; pub const SAA7146_I2C_BUSY:u32=1;
pub const SAA7146_I2C_START:u32=3; pub const SAA7146_I2C_CONT:u32=2; pub const SAA7146_I2C_STOP:u32=1; pub const SAA7146_I2C_NOP:u32=0;

// The complete bit-mask family is represented by these direct Rust constants.
pub const MASK_00:u32=1; pub const MASK_01:u32=2; pub const MASK_02:u32=4; pub const MASK_03:u32=8; pub const MASK_04:u32=0x10; pub const MASK_05:u32=0x20; pub const MASK_06:u32=0x40; pub const MASK_07:u32=0x80; pub const MASK_08:u32=0x100; pub const MASK_09:u32=0x200; pub const MASK_10:u32=0x400; pub const MASK_11:u32=0x800; pub const MASK_12:u32=0x1000; pub const MASK_13:u32=0x2000; pub const MASK_14:u32=0x4000; pub const MASK_15:u32=0x8000; pub const MASK_16:u32=0x10000; pub const MASK_17:u32=0x20000; pub const MASK_18:u32=0x40000; pub const MASK_19:u32=0x80000; pub const MASK_20:u32=0x100000; pub const MASK_21:u32=0x200000; pub const MASK_22:u32=0x400000; pub const MASK_23:u32=0x800000; pub const MASK_24:u32=0x1000000; pub const MASK_25:u32=0x2000000; pub const MASK_26:u32=0x4000000; pub const MASK_27:u32=0x8000000; pub const MASK_28:u32=0x10000000; pub const MASK_29:u32=0x20000000; pub const MASK_30:u32=0x40000000; pub const MASK_31:u32=0x80000000;
pub const MASK_B0:u32=0xff; pub const MASK_B1:u32=0xff00; pub const MASK_B2:u32=0xff0000; pub const MASK_B3:u32=0xff000000; pub const MASK_W0:u32=0xffff; pub const MASK_W1:u32=0xffff0000;
pub const PCI_BT_V1:u32=0x48; pub const PCI_BT_V2:u32=0x49; pub const PCI_BT_V3:u32=0x4a; pub const PCI_BT_DEBI:u32=0x4b; pub const PCI_BT_A:u32=0x4c; pub const DD1_INIT:u32=0x50; pub const DD1_STREAM_B:u32=0x54; pub const DD1_STREAM_A:u32=0x56; pub const BRS_CTRL:u32=0x58; pub const HPS_CTRL:u32=0x5c; pub const HPS_V_SCALE:u32=0x60; pub const HPS_V_GAIN:u32=0x64; pub const HPS_H_PRESCALE:u32=0x68; pub const HPS_H_SCALE:u32=0x6c; pub const BCS_CTRL:u32=0x70; pub const CHROMA_KEY_RANGE:u32=0x74; pub const CLIP_FORMAT_CTRL:u32=0x78; pub const DEBI_CONFIG:u32=0x7c; pub const DEBI_COMMAND:u32=0x80; pub const DEBI_PAGE:u32=0x84; pub const DEBI_AD:u32=0x88; pub const I2C_TRANSFER:u32=0x8c; pub const I2C_STATUS:u32=0x90; pub const BASE_A1_IN:u32=0x94; pub const PROT_A1_IN:u32=0x98; pub const PAGE_A1_IN:u32=0x9c; pub const BASE_A1_OUT:u32=0xa0; pub const PROT_A1_OUT:u32=0xa4; pub const PAGE_A1_OUT:u32=0xa8; pub const BASE_A2_IN:u32=0xac; pub const PROT_A2_IN:u32=0xb0; pub const PAGE_A2_IN:u32=0xb4; pub const BASE_A2_OUT:u32=0xb8; pub const PROT_A2_OUT:u32=0xbc; pub const PAGE_A2_OUT:u32=0xc0; pub const RPS_PAGE0:u32=0xc4; pub const RPS_PAGE1:u32=0xc8; pub const RPS_THRESH0:u32=0xcc; pub const RPS_THRESH1:u32=0xd0; pub const RPS_TOV0:u32=0xd4; pub const RPS_TOV1:u32=0xd8; pub const EC1SSR:u32=0xe4; pub const EC2SSR:u32=0xe8; pub const ECT1R:u32=0xec; pub const ECT2R:u32=0xf0; pub const ACON1:u32=0xf4; pub const ACON2:u32=0xf8; pub const EC1R:u32=0x118; pub const EC2R:u32=0x11c; pub const PCI_VDP1:u32=0x120; pub const PCI_VDP2:u32=0x124; pub const PCI_VDP3:u32=0x128; pub const PCI_ADP1:u32=0x12c; pub const PCI_ADP2:u32=0x130; pub const PCI_ADP3:u32=0x134; pub const PCI_ADP4:u32=0x138; pub const PCI_DMA_DDP:u32=0x13c; pub const LEVEL_REP:u32=0x140; pub const A_TIME_SLOT1:u32=0x180; pub const A_TIME_SLOT2:u32=0x1c0;
pub const SAA7146_I2C_BUS_BIT_RATE_6400:u32=0x500; pub const SAA7146_I2C_BUS_BIT_RATE_3200:u32=0x100; pub const SAA7146_I2C_BUS_BIT_RATE_480:u32=0x400; pub const SAA7146_I2C_BUS_BIT_RATE_320:u32=0x600; pub const SAA7146_I2C_BUS_BIT_RATE_240:u32=0x700; pub const SAA7146_I2C_BUS_BIT_RATE_120:u32=0; pub const SAA7146_I2C_BUS_BIT_RATE_80:u32=0x200; pub const SAA7146_I2C_BUS_BIT_RATE_60:u32=0x300;
extern "C" { pub fn SAA7146_IER_DISABLE(x:*mut saa7146_dev,y:u32); pub fn SAA7146_IER_ENABLE(x:*mut saa7146_dev,y:u32); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
