/*
 * Include file for Alchemy Semiconductor's Au1550 Descriptor Based DMA
 * Controller. Translated from the C header; volatile structs retain C layout.
 */

#[repr(C)]
pub struct dbdma_global_t {
    pub ddma_config: u32,
    pub ddma_intstat: u32,
    pub ddma_throttle: u32,
    pub ddma_inten: u32,
}

pub const DDMA_CONFIG_AF: u32 = 1 << 2;
pub const DDMA_CONFIG_AH: u32 = 1 << 1;
pub const DDMA_CONFIG_AL: u32 = 1 << 0;
pub const DDMA_THROTTLE_EN: u32 = 1 << 31;

#[repr(C)]
pub struct au1x_dma_chan_t {
    pub ddma_cfg: u32,
    pub ddma_desptr: u32,
    pub ddma_statptr: u32,
    pub ddma_dbell: u32,
    pub ddma_irq: u32,
    pub ddma_stat: u32,
    pub ddma_bytecnt: u32,
}

pub const DDMA_CFG_SED: u32 = 1 << 9;
pub const DDMA_CFG_SP: u32 = 1 << 8;
pub const DDMA_CFG_DED: u32 = 1 << 7;
pub const DDMA_CFG_DP: u32 = 1 << 6;
pub const DDMA_CFG_SYNC: u32 = 1 << 5;
pub const DDMA_CFG_PPR: u32 = 1 << 4;
pub const DDMA_CFG_DFN: u32 = 1 << 3;
pub const DDMA_CFG_SBE: u32 = 1 << 2;
pub const DDMA_CFG_DBE: u32 = 1 << 1;
pub const DDMA_CFG_EN: u32 = 1 << 0;
pub const DDMA_IRQ_IN: u32 = 1 << 0;
pub const DDMA_STAT_DB: u32 = 1 << 2;
pub const DDMA_STAT_V: u32 = 1 << 1;
pub const DDMA_STAT_H: u32 = 1 << 0;

#[repr(C)]
pub struct au1x_ddma_desc_t {
    pub dscr_cmd0: u32,
    pub dscr_cmd1: u32,
    pub dscr_source0: u32,
    pub dscr_source1: u32,
    pub dscr_dest0: u32,
    pub dscr_dest1: u32,
    pub dscr_stat: u32,
    pub dscr_nxtptr: u32,
    pub sw_status: u32,
    pub sw_context: u32,
    pub sw_reserved: [u32; 6],
}

pub const DSCR_CMD0_V: u32 = 1 << 31;
pub const DSCR_CMD0_MEM: u32 = 1 << 30;
pub const DSCR_CMD0_SID_MASK: u32 = 0x1f << 25;
pub const DSCR_CMD0_DID_MASK: u32 = 0x1f << 20;
pub const DSCR_CMD0_SW_MASK: u32 = 0x3 << 18;
pub const DSCR_CMD0_DW_MASK: u32 = 0x3 << 16;
pub const DSCR_CMD0_ARB: u32 = 0x1 << 15;
pub const DSCR_CMD0_DT_MASK: u32 = 0x3 << 13;
pub const DSCR_CMD0_SN: u32 = 1 << 12;
pub const DSCR_CMD0_DN: u32 = 1 << 11;
pub const DSCR_CMD0_SM: u32 = 1 << 10;
pub const DSCR_CMD0_IE: u32 = 1 << 8;
pub const DSCR_CMD0_SP: u32 = 1 << 4;
pub const DSCR_CMD0_CV: u32 = 1 << 2;
pub const DSCR_CMD0_ST_MASK: u32 = 0x3;
pub const SW_STATUS_INUSE: u32 = 1;

macro_rules! const_ids {
    ($($name:ident = $value:expr),* $(,)?) => { $(pub const $name: u32 = $value;)* };
}
const_ids!(
    AU1550_DSCR_CMD0_UART0_TX=0, AU1550_DSCR_CMD0_UART0_RX=1,
    AU1550_DSCR_CMD0_UART3_TX=2, AU1550_DSCR_CMD0_UART3_RX=3,
    AU1550_DSCR_CMD0_DMA_REQ0=4, AU1550_DSCR_CMD0_DMA_REQ1=5,
    AU1550_DSCR_CMD0_DMA_REQ2=6, AU1550_DSCR_CMD0_DMA_REQ3=7,
    AU1550_DSCR_CMD0_USBDEV_RX0=8, AU1550_DSCR_CMD0_USBDEV_TX0=9,
    AU1550_DSCR_CMD0_USBDEV_TX1=10, AU1550_DSCR_CMD0_USBDEV_TX2=11,
    AU1550_DSCR_CMD0_USBDEV_RX3=12, AU1550_DSCR_CMD0_USBDEV_RX4=13,
    AU1550_DSCR_CMD0_PSC0_TX=14, AU1550_DSCR_CMD0_PSC0_RX=15,
    AU1550_DSCR_CMD0_PSC1_TX=16, AU1550_DSCR_CMD0_PSC1_RX=17,
    AU1550_DSCR_CMD0_PSC2_TX=18, AU1550_DSCR_CMD0_PSC2_RX=19,
    AU1550_DSCR_CMD0_PSC3_TX=20, AU1550_DSCR_CMD0_PSC3_RX=21,
    AU1550_DSCR_CMD0_PCI_WRITE=22, AU1550_DSCR_CMD0_NAND_FLASH=23,
    AU1550_DSCR_CMD0_MAC0_RX=24, AU1550_DSCR_CMD0_MAC0_TX=25,
    AU1550_DSCR_CMD0_MAC1_RX=26, AU1550_DSCR_CMD0_MAC1_TX=27,
    AU1200_DSCR_CMD0_UART0_TX=0, AU1200_DSCR_CMD0_UART0_RX=1,
    AU1200_DSCR_CMD0_UART1_TX=2, AU1200_DSCR_CMD0_UART1_RX=3,
    AU1200_DSCR_CMD0_DMA_REQ0=4, AU1200_DSCR_CMD0_DMA_REQ1=5,
    AU1200_DSCR_CMD0_MAE_BE=6, AU1200_DSCR_CMD0_MAE_FE=7,
    AU1200_DSCR_CMD0_SDMS_TX0=8, AU1200_DSCR_CMD0_SDMS_RX0=9,
    AU1200_DSCR_CMD0_SDMS_TX1=10, AU1200_DSCR_CMD0_SDMS_RX1=11,
    AU1200_DSCR_CMD0_AES_RX=12, AU1200_DSCR_CMD0_AES_TX=13,
    AU1200_DSCR_CMD0_PSC0_TX=14, AU1200_DSCR_CMD0_PSC0_RX=15,
    AU1200_DSCR_CMD0_PSC1_TX=16, AU1200_DSCR_CMD0_PSC1_RX=17,
    AU1200_DSCR_CMD0_CIM_RXA=18, AU1200_DSCR_CMD0_CIM_RXB=19,
    AU1200_DSCR_CMD0_CIM_RXC=20, AU1200_DSCR_CMD0_MAE_BOTH=21,
    AU1200_DSCR_CMD0_LCD=22, AU1200_DSCR_CMD0_NAND_FLASH=23,
    AU1200_DSCR_CMD0_PSC0_SYNC=24, AU1200_DSCR_CMD0_PSC1_SYNC=25,
    AU1200_DSCR_CMD0_CIM_SYNC=26,
    AU1300_DSCR_CMD0_UART0_TX=0, AU1300_DSCR_CMD0_UART0_RX=1,
    AU1300_DSCR_CMD0_UART1_TX=2, AU1300_DSCR_CMD0_UART1_RX=3,
    AU1300_DSCR_CMD0_UART2_TX=4, AU1300_DSCR_CMD0_UART2_RX=5,
    AU1300_DSCR_CMD0_UART3_TX=6, AU1300_DSCR_CMD0_UART3_RX=7,
    AU1300_DSCR_CMD0_SDMS_TX0=8, AU1300_DSCR_CMD0_SDMS_RX0=9,
    AU1300_DSCR_CMD0_SDMS_TX1=10, AU1300_DSCR_CMD0_SDMS_RX1=11,
    AU1300_DSCR_CMD0_AES_TX=12, AU1300_DSCR_CMD0_AES_RX=13,
    AU1300_DSCR_CMD0_PSC0_TX=14, AU1300_DSCR_CMD0_PSC0_RX=15,
    AU1300_DSCR_CMD0_PSC1_TX=16, AU1300_DSCR_CMD0_PSC1_RX=17,
    AU1300_DSCR_CMD0_PSC2_TX=18, AU1300_DSCR_CMD0_PSC2_RX=19,
    AU1300_DSCR_CMD0_PSC3_TX=20, AU1300_DSCR_CMD0_PSC3_RX=21,
    AU1300_DSCR_CMD0_LCD=22, AU1300_DSCR_CMD0_NAND_FLASH=23,
    AU1300_DSCR_CMD0_SDMS_TX2=24, AU1300_DSCR_CMD0_SDMS_RX2=25,
    AU1300_DSCR_CMD0_CIM_SYNC=26, AU1300_DSCR_CMD0_UDMA=27,
    AU1300_DSCR_CMD0_DMA_REQ0=28, AU1300_DSCR_CMD0_DMA_REQ1=29
);

pub const DSCR_CMD0_THROTTLE: u32 = 30;
pub const DSCR_CMD0_ALWAYS: u32 = 31;
pub const DSCR_NDEV_IDS: u32 = 32;
pub const DSCR_CMD0_BYTE: u32 = 0;
pub const DSCR_CMD0_HALFWORD: u32 = 1;
pub const DSCR_CMD0_WORD: u32 = 2;
pub const DSCR_CMD0_STANDARD: u32 = 0;
pub const DSCR_CMD0_LITERAL: u32 = 1;
pub const DSCR_CMD0_CMP_BRANCH: u32 = 2;
pub const DSCR_CMD1_SUPTR_MASK: u32 = 0xf << 28;
pub const DSCR_CMD1_DUPTR_MASK: u32 = 0xf << 24;
pub const DSCR_CMD1_FL_MASK: u32 = 0x3 << 22;
pub const DSCR_CMD1_BC_MASK: u32 = 0x3fffff;
pub const DSCR_CMD1_FL_MEM_STRIDE0: u32 = 0;
pub const DSCR_CMD1_FL_MEM_STRIDE1: u32 = 1;
pub const DSCR_CMD1_FL_MEM_STRIDE2: u32 = 2;
pub const DSCR_SRC1_STS_MASK: u32 = 3 << 30;
pub const DSCR_SRC1_SAM_MASK: u32 = 3 << 28;
pub const DSCR_SRC1_SB_MASK: u32 = 0x3fff << 14;
pub const DSCR_SRC1_SS_MASK: u32 = 0x3fff;
pub const DSCR_DEST1_DTS_MASK: u32 = 3 << 30;
pub const DSCR_DEST1_DAM_MASK: u32 = 3 << 28;
pub const DSCR_DEST1_DB_MASK: u32 = 0x3fff << 14;
pub const DSCR_DEST1_DS_MASK: u32 = 0x3fff;
pub const DSCR_xTS_SIZE1: u32 = 0;
pub const DSCR_xTS_SIZE2: u32 = 1;
pub const DSCR_xTS_SIZE4: u32 = 2;
pub const DSCR_xTS_SIZE8: u32 = 3;
pub const DSCR_xAM_INCREMENT: u32 = 0;
pub const DSCR_xAM_DECREMENT: u32 = 1;
pub const DSCR_xAM_STATIC: u32 = 2;
pub const DSCR_xAM_BURST: u32 = 3;
pub const DSCR_NXTPTR_MASK: u32 = 0x07ffffff;
pub const DSCR_NXTPTR_MS: u32 = 1 << 27;
pub const NUM_DBDMA_CHANS: u32 = 16;

#[inline] pub const fn DSCR_DEV2CUSTOM_ID(x: u32, d: u32) -> u32 { (((x & 0xffff) << 8) | 0x32000000) | (d & 0xff) }
#[inline] pub const fn DSCR_CUSTOM2DEV_ID(x: u32) -> u32 { x & 0xff }
#[inline] pub const fn DSCR_CMD0_SID(x: u32) -> u32 { (x & 0x1f) << 25 }
#[inline] pub const fn DSCR_CMD0_DID(x: u32) -> u32 { (x & 0x1f) << 20 }
#[inline] pub const fn DSCR_CMD0_SW(x: u32) -> u32 { (x & 0x3) << 18 }
#[inline] pub const fn DSCR_CMD0_DW(x: u32) -> u32 { (x & 0x3) << 16 }
#[inline] pub const fn DSCR_CMD0_DT(x: u32) -> u32 { (x & 0x3) << 13 }
#[inline] pub const fn DSCR_CMD0_ST(x: u32) -> u32 { x & 0x3 }
#[inline] pub const fn DSCR_CMD1_FL(x: u32) -> u32 { (x & 0x3) << 22 }
#[inline] pub const fn DSCR_SRC1_SB(x: u32) -> u32 { (x & 0x3fff) << 14 }
#[inline] pub const fn DSCR_SRC1_SS(x: u32) -> u32 { x & 0x3fff }
#[inline] pub const fn DSCR_DEST1_DB(x: u32) -> u32 { (x & 0x3fff) << 14 }
#[inline] pub const fn DSCR_DEST1_DS(x: u32) -> u32 { x & 0x3fff }
#[inline] pub const fn DSCR_SRC1_STS(x: u32) -> u32 { (x & 3) << 30 }
#[inline] pub const fn DSCR_DEST1_DTS(x: u32) -> u32 { (x & 3) << 30 }
#[inline] pub const fn DSCR_SRC1_SAM(x: u32) -> u32 { (x & 3) << 28 }
#[inline] pub const fn DSCR_DEST1_DAM(x: u32) -> u32 { (x & 3) << 28 }
#[inline] pub const fn DSCR_NXTPTR(x: u32) -> u32 { x >> 5 }
#[inline] pub const fn DSCR_GET_NXTPTR(x: u32) -> u32 { x << 5 }

// External kernel types and functions are supplied by other translated headers.
#[repr(C)] pub struct dbdev_tab_t { pub dev_id:u32, pub dev_flags:u32, pub dev_tsize:u32, pub dev_devwidth:u32, pub dev_physaddr:u32, pub dev_intlevel:u32, pub dev_intpolarity:u32 }
#[repr(C)] pub struct chan_tab_t {
    pub lock: spinlock_t, pub chan_flags:u32, pub chan_index:u32,
    pub chan_src:*mut dbdev_tab_t, pub chan_dest:*mut dbdev_tab_t,
    pub chan_ptr:*mut au1x_dma_chan_t, pub chan_desc_base:*mut au1x_ddma_desc_t,
    pub cdb_membase:u32, pub get_ptr:*mut au1x_ddma_desc_t, pub put_ptr:*mut au1x_ddma_desc_t,
    pub cur_ptr:*mut au1x_ddma_desc_t, pub chan_callparam:*mut core::ffi::c_void,
    pub chan_callback: Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void)>,
}
pub const DEV_FLAGS_INUSE:u32=1<<0; pub const DEV_FLAGS_ANYUSE:u32=1<<1; pub const DEV_FLAGS_OUT:u32=1<<2; pub const DEV_FLAGS_IN:u32=1<<3; pub const DEV_FLAGS_BURSTABLE:u32=1<<4; pub const DEV_FLAGS_SYNC:u32=1<<5;
pub const DBDMA_MEM_CHAN:u32=DSCR_CMD0_ALWAYS;
pub const DDMA_FLAGS_IE:u32=1<<0; pub const DDMA_FLAGS_NOIE:u32=1<<1;

pub type dma_addr_t = u32;
pub type spinlock_t = usize;
extern "C" {
    pub fn au1xxx_dbdma_chan_alloc(srcid:u32,destid:u32,callback:Option<unsafe extern "C" fn(i32,*mut core::ffi::c_void)>,callparam:*mut core::ffi::c_void)->u32;
    pub fn au1xxx_dbdma_set_devwidth(chanid:u32,bits:i32)->u32;
    pub fn au1xxx_dbdma_ring_alloc(chanid:u32,entries:i32)->u32;
    pub fn au1xxx_dbdma_put_source(chanid:u32,buf:dma_addr_t,nbytes:i32,flags:u32)->u32;
    pub fn au1xxx_dbdma_put_dest(chanid:u32,buf:dma_addr_t,nbytes:i32,flags:u32)->u32;
    pub fn au1xxx_dbdma_get_dest(chanid:u32,buf:*mut *mut core::ffi::c_void,nbytes:*mut i32)->u32;
    pub fn au1xxx_dbdma_stop(chanid:u32); pub fn au1xxx_dbdma_start(chanid:u32); pub fn au1xxx_dbdma_reset(chanid:u32);
    pub fn au1xxx_get_dma_residue(chanid:u32)->u32; pub fn au1xxx_dbdma_chan_free(chanid:u32); pub fn au1xxx_dbdma_dump(chanid:u32);
    pub fn au1xxx_dbdma_put_dscr(chanid:u32,dscr:*mut au1x_ddma_desc_t)->u32;
    pub fn au1xxx_ddma_add_device(dev:*mut dbdev_tab_t)->u32; pub fn au1xxx_ddma_del_device(devid:u32);
    pub fn au1xxx_ddma_get_nextptr_virt(dp:*mut au1x_ddma_desc_t)->*mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
