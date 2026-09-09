/* Rust translation of bridge.h. C includes and build-time guards are omitted. */

pub const IOPFNSHIFT: u32 = 12;
pub const IOPGSIZE: u64 = 1u64 << IOPFNSHIFT;
#[inline] pub const fn IOPG(x: u64) -> u64 { x >> IOPFNSHIFT }
#[inline] pub const fn IOPGOFF(x: u64) -> u64 { x & (IOPGSIZE - 1) }
pub const BRIDGE_ATE_RAM_SIZE: u32 = 0x400;
pub const BRIDGE_CONFIG_BASE: u32 = 0x20000;
pub const BRIDGE_CONFIG1_BASE: u32 = 0x28000;
pub const BRIDGE_CONFIG_END: u32 = 0x30000;
pub const BRIDGE_CONFIG_SLOT_SIZE: u32 = 0x1000;
pub const BRIDGE_SSRAM_512K: u32 = 0x80000;
pub const BRIDGE_SSRAM_128K: u32 = 0x20000;
pub const BRIDGE_SSRAM_64K: u32 = 0x10000;
pub const BRIDGE_SSRAM_0K: u32 = 0;

pub const ATE_V:u32=1; pub const ATE_CO:u32=2; pub const ATE_PREC:u32=4; pub const ATE_PREF:u32=8; pub const ATE_BAR:u32=16;
pub const ATE_PFNSHIFT:u32=12; pub const ATE_TIDSHIFT:u32=8; pub const ATE_RMFSHIFT:u32=48;
#[inline] pub const fn mkate(xaddr:u64,xid:u64,attr:u64)->u64 {(xaddr&0x0000fffffffff000)| (xid<<ATE_TIDSHIFT)|attr}
pub const BRIDGE_INTERNAL_ATES:usize=128;

#[repr(C)]
pub struct bridge_regs {
    pub b_widget: widget_cfg_t,
    pub _pad_000058:u32, pub b_wid_aux_err:u32, pub _pad_000060:u32, pub b_wid_resp_upper:u32,
    pub _pad_000068:u32, pub b_wid_resp_lower:u32, pub _pad_000070:u32, pub b_wid_tst_pin_ctrl:u32, pub _pad_000078:[u32;2],
    pub _pad_000080:u32, pub b_dir_map:u32, pub _pad_000088:[u32;2], pub _pad_000090:u32, pub b_ram_perr:u32, pub _pad_000098:[u32;2],
    pub _pad_0000A0:u32, pub b_arb:u32, pub _pad_0000A8:[u32;2], pub _pad_0000B0:u32, pub b_nic:u32, pub _pad_0000B8:[u32;2],
    pub _pad_0000C0:u32, pub b_bus_timeout:u32, pub _pad_0000C8:u32, pub b_pci_cfg:u32, pub _pad_0000D0:u32, pub b_pci_err_upper:u32, pub _pad_0000D8:u32, pub b_pci_err_lower:u32, pub _pad_0000E0:[u32;8],
    pub _pad_000100:u32, pub b_int_status:u32, pub _pad_000108:u32, pub b_int_enable:u32, pub _pad_000110:u32, pub b_int_rst_stat:u32, pub _pad_000118:u32, pub b_int_mode:u32, pub _pad_000120:u32, pub b_int_device:u32, pub _pad_000128:u32, pub b_int_host_err:u32,
    pub b_int_addr:[bridge_pair;8], pub _pad_000170:[u32;36], pub b_device:[bridge_pair;8], pub b_wr_req_buf:[bridge_pair;8], pub b_rrb_map:[bridge_pair;2],
    pub _pad_000290:u32, pub b_resp_status:u32, pub _pad_000298:u32, pub b_resp_clear:u32, pub _pad_0002A0:[u32;24], pub _pad_000300:[u8;0xfd00],
    pub b_int_ate_ram:[bridge_ate;128], pub _pad_010400:[u8;0xc00], pub b_int_ate_ram_lo:[bridge_pair;128], pub _pad_011400:[u8;0x1ec00],
    pub b_type0_cfg_dev:[bridge_cfg;8], pub b_type1_cfg:bridge_cfg, pub _pad_029000:[u8;0x7000], pub b_pci_iack:bridge_iack, pub _pad_030007:[u8;0x4fff8], pub b_ext_ate_ram:[u64;0x10000], pub _pad_100000:[u8;0x100000], pub b_devio_raw:[bridge_cfg;10], pub b_external_flash:bridge_flash,
}
#[repr(C)] pub struct bridge_pair { pub __pad:u32, pub reg:u32 }
#[repr(C)] pub struct bridge_ate { pub wr:u64 }
#[repr(C)] pub struct bridge_cfg { pub c:[u8;0x100000], }
#[repr(C)] pub struct bridge_iack { pub c:[u8;8] }
#[repr(C)] pub struct bridge_flash { pub c:[u8;0x400000] }

#[repr(C)] pub struct bridge_err_cmdword { pub cmd_word:u32 }

#[inline] pub const fn BRIDGE_INT_ADDR(x:u32)->u32 {0x134+x*8}
#[inline] pub const fn BRIDGE_DEVICE(x:u32)->u32 {0x204+x*8}
#[inline] pub const fn BRIDGE_WR_REQ_BUF(x:u32)->u32 {0x244+x*8}
#[inline] pub const fn BRIDGE_TYPE0_CFG_DEV(s:u32)->u32 {0x20000+s*0x1000}
#[inline] pub const fn BRIDGE_TYPE0_CFG_DEVF(s:u32,f:u32)->u32 {0x20000+s*0x1000+f*0x100}
#[inline] pub const fn BRIDGE_DEVIO(x:u32)->u32 {if x<=1 {0x200000+x*0x200000} else {0x600000+(x-2)*0x100000}}
#[inline] pub const fn BRIDGE_RRB_VALID(r:u32)->u32 {0x10000<<r}
#[inline] pub const fn BRIDGE_RRB_INUSE(r:u32)->u32 {1<<r}
#[inline] pub const fn BRIDGE_RRB_CLEAR(r:u32)->u32 {1<<r}

pub const BRIDGE_WID_AUX_ERR:u32=0x5c; pub const BRIDGE_WID_RESP_UPPER:u32=0x64; pub const BRIDGE_WID_RESP_LOWER:u32=0x6c; pub const BRIDGE_WID_TST_PIN_CTRL:u32=0x74;
pub const BRIDGE_DIR_MAP:u32=0x84; pub const BRIDGE_RAM_PERR:u32=0x94; pub const BRIDGE_ARB:u32=0xa4; pub const BRIDGE_NIC:u32=0xb4; pub const BRIDGE_BUS_TIMEOUT:u32=0xc4; pub const BRIDGE_PCI_CFG:u32=0xcc; pub const BRIDGE_PCI_ERR_UPPER:u32=0xd4; pub const BRIDGE_PCI_ERR_LOWER:u32=0xdc;
pub const BRIDGE_ATE_RAM:u32=0x10000; pub const BRIDGE_TYPE1_CFG:u32=0x28000; pub const BRIDGE_PCI_IACK:u32=0x30000; pub const BRIDGE_EXT_SSRAM:u32=0x80000;
pub const BRIDGE_DEV_CNT:u32=8; pub const BRIDGE_DEVIO0:u32=0x200000; pub const BRIDGE_DEVIO1:u32=0x400000; pub const BRIDGE_DEVIO2:u32=0x600000; pub const BRIDGE_DEVIO_OFF:u32=0x100000; pub const BRIDGE_EXTERNAL_FLASH:u32=0xc00000;

pub const BRIDGE_WIDGET_PART_NUM:u32=0xc002; pub const XBRIDGE_WIDGET_PART_NUM:u32=0xd002; pub const BRIDGE_WIDGET_MFGR_NUM:u32=0x36; pub const XBRIDGE_WIDGET_MFGR_NUM:u32=0x24;
pub const BRIDGE_REV_A:u32=1; pub const BRIDGE_REV_B:u32=2; pub const BRIDGE_REV_C:u32=3; pub const BRIDGE_REV_D:u32=4;
pub const BRIDGE_CTRL_FLASH_WR_EN:u32=1<<31; pub const BRIDGE_CTRL_EN_CLK50:u32=1<<30; pub const BRIDGE_CTRL_EN_CLK40:u32=1<<29; pub const BRIDGE_CTRL_EN_CLK33:u32=1<<28;
#[inline] pub const fn BRIDGE_CTRL_RST(n:u32)->u32{n<<24} #[inline] pub const fn BRIDGE_CTRL_RST_PIN(x:u32)->u32{BRIDGE_CTRL_RST(1<<x)}
pub const BRIDGE_CTRL_IO_SWAP:u32=1<<23; pub const BRIDGE_CTRL_MEM_SWAP:u32=1<<22; pub const BRIDGE_CTRL_PAGE_SIZE:u32=1<<21; pub const BRIDGE_CTRL_SS_PAR_BAD:u32=1<<20; pub const BRIDGE_CTRL_SS_PAR_EN:u32=1<<19; #[inline] pub const fn BRIDGE_CTRL_SSRAM_SIZE(n:u32)->u32{n<<17} #[inline] pub const fn BRIDGE_CTRL_LLP_XBAR_CRD(n:u32)->u32{n<<12} #[inline] pub const fn BRIDGE_CTRL_MAX_TRANS(n:u32)->u32{n<<4} #[inline] pub const fn BRIDGE_CTRL_WIDGET_ID(n:u32)->u32{n}

pub const BRIDGE_CREDIT:u32=3; pub const BRIDGE_RRB_EN:u32=8; pub const BRIDGE_RRB_DEV:u32=7; pub const BRIDGE_RRB_VDEV:u32=4; pub const BRIDGE_RRB_PDEV:u32=3; pub const XBOX_BRIDGE_WID:u32=8; pub const FLASH_PROM1_BASE:u32=0xe00000; pub const XBOX_RPS_EXISTS:u32=1<<6; pub const XBOX_RPS_FAIL:u32=1<<4;
pub const BRIDGE_PIO32_XTALK_ALIAS_BASE:u64=0x4000000000; pub const BRIDGE_PIO32_XTALK_ALIAS_LIMIT:u64=0x7fffffff; pub const BRIDGE_PIO64_XTALK_ALIAS_BASE:u64=0x8000000000; pub const BRIDGE_PIO64_XTALK_ALIAS_LIMIT:u64=0xbfffffff; pub const BRIDGE_PCIIO_XTALK_ALIAS_BASE:u64=0x10000000000; pub const BRIDGE_PCIIO_XTALK_ALIAS_LIMIT:u64=0x1fffffffff;
pub const BRIDGE_LOCAL_BASE:u64=0; pub const BRIDGE_DMA_MAPPED_BASE:u64=0x40000000; pub const BRIDGE_DMA_MAPPED_SIZE:u64=0x40000000; pub const BRIDGE_DMA_DIRECT_BASE:u64=0x80000000; pub const BRIDGE_DMA_DIRECT_SIZE:u64=0x80000000;
#[inline] pub const fn IS_PCI32_LOCAL(x:u64)->bool{x<BRIDGE_DMA_MAPPED_BASE} #[inline] pub const fn IS_PCI32_MAPPED(x:u64)->bool{x<BRIDGE_DMA_DIRECT_BASE&&x>=BRIDGE_DMA_MAPPED_BASE} #[inline] pub const fn IS_PCI32_DIRECT(x:u64)->bool{x>=BRIDGE_DMA_MAPPED_BASE} #[inline] pub const fn IS_GIO_LOCAL(x:u64)->bool{x<BRIDGE_DMA_MAPPED_BASE} #[inline] pub const fn IS_GIO_MAPPED(x:u64)->bool{x<BRIDGE_DMA_DIRECT_BASE&&x>=BRIDGE_DMA_MAPPED_BASE} #[inline] pub const fn IS_GIO_DIRECT(x:u64)->bool{x>=BRIDGE_DMA_MAPPED_BASE}
pub const PCI64_ATTR_TARG_MASK:u64=0xf000000000000000; pub const PCI64_ATTR_TARG_SHFT:u32=60; pub const PCI64_ATTR_PREF:u64=0x0800000000000000; pub const PCI64_ATTR_PREC:u64=0x0400000000000000; pub const PCI64_ATTR_VIRTUAL:u64=0x0200000000000000; pub const PCI64_ATTR_BAR:u64=0x0100000000000000; pub const PCI64_ATTR_RMF_MASK:u64=0x00ff000000000000; pub const PCI64_ATTR_RMF_SHFT:u32=48;
#[repr(C)] pub struct bridge_controller { pub busn:resource, pub base:*mut bridge_regs, pub baddr:usize, pub intr_addr:usize, pub domain:*mut irq_domain, pub pci_int:[[u32;2];8], pub int_mapping:[[u32;2];8], pub ioc3_sid:[u32;8], pub nasid:nasid_t }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
