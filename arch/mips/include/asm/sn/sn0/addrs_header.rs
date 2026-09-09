/*
 * Rust translation of addrs.h.  C preprocessor conditions are represented by
 * Cargo cfg features where applicable; symbols supplied by other headers are
 * intentionally left as external dependencies.
 */

#[cfg(feature = "CONFIG_SGI_SN_N_MODE")]
pub const NODE_SIZE_BITS: u32 = 31;
#[cfg(not(feature = "CONFIG_SGI_SN_N_MODE"))]
pub const NODE_SIZE_BITS: u32 = 32;
#[cfg(feature = "CONFIG_SGI_SN_N_MODE")]
pub const BWIN_SIZE_BITS: u32 = 28;
#[cfg(not(feature = "CONFIG_SGI_SN_N_MODE"))]
pub const BWIN_SIZE_BITS: u32 = 29;

#[cfg(feature = "CONFIG_SGI_SN_N_MODE")]
pub const NASID_BITS: u32 = 9;
#[cfg(not(feature = "CONFIG_SGI_SN_N_MODE"))]
pub const NASID_BITS: u32 = 8;
#[cfg(feature = "CONFIG_SGI_SN_N_MODE")]
pub const NASID_BITMASK: u64 = 0x1ff;
#[cfg(not(feature = "CONFIG_SGI_SN_N_MODE"))]
pub const NASID_BITMASK: u64 = 0xff;
#[cfg(feature = "CONFIG_SGI_SN_N_MODE")]
pub const NASID_SHFT: u32 = 31;
#[cfg(not(feature = "CONFIG_SGI_SN_N_MODE"))]
pub const NASID_SHFT: u32 = 32;
#[cfg(feature = "CONFIG_SGI_SN_N_MODE")]
pub const NASID_META_BITS: u32 = 5;
#[cfg(not(feature = "CONFIG_SGI_SN_N_MODE"))]
pub const NASID_META_BITS: u32 = 4;
pub const NASID_LOCAL_BITS: u32 = 4;

#[cfg(feature = "CONFIG_SGI_SN_N_MODE")]
pub const BDDIR_UPPER_MASK: u64 = 0x7ffff << 10;
#[cfg(not(feature = "CONFIG_SGI_SN_N_MODE"))]
pub const BDDIR_UPPER_MASK: u64 = 0xfffff << 10;
#[cfg(feature = "CONFIG_SGI_SN_N_MODE")]
pub const BDECC_UPPER_MASK: u64 = 0x3ffffff << 3;
#[cfg(not(feature = "CONFIG_SGI_SN_N_MODE"))]
pub const BDECC_UPPER_MASK: u64 = 0x7ffffff << 3;

pub const NODE_ADDRSPACE_SIZE: u64 = 1u64 << NODE_SIZE_BITS;
pub const BWIN_INDEX_BITS: u32 = 3;
pub const BWIN_WIDGET_MASK: u64 = 0x7;

#[macro_export]
macro_rules! NASID_MASK { () => { $crate::NASID_BITMASK << $crate::NASID_SHFT }; }
#[macro_export]
macro_rules! NASID_GET { ($pa:expr) => { (((($pa as u64) >> $crate::NASID_SHFT) & $crate::NASID_BITMASK) as i32) }; }
#[macro_export]
macro_rules! BWIN_SIZE { () => { 1u64 << $crate::BWIN_SIZE_BITS }; }
#[macro_export]
macro_rules! BWIN_SIZEMASK { () => { $crate::BWIN_SIZE!() - 1 }; }
#[macro_export]
macro_rules! NODE_BWIN_BASE0 { ($nasid:expr) => { NODE_IO_BASE($nasid) + $crate::BWIN_SIZE!() }; }
#[macro_export]
macro_rules! NODE_BWIN_BASE { ($nasid:expr, $bigwin:expr) => { $crate::NODE_BWIN_BASE0!($nasid) + (($bigwin as u64) << $crate::BWIN_SIZE_BITS) }; }
#[macro_export]
macro_rules! BWIN_WIDGETADDR { ($addr:expr) => { ($addr) & $crate::BWIN_SIZEMASK!() }; }
#[macro_export]
macro_rules! BWIN_WINDOWNUM { ($addr:expr) => { (($addr >> $crate::BWIN_SIZE_BITS) & $crate::BWIN_WIDGET_MASK) }; }
#[macro_export]
macro_rules! NODE_SWIN_BASE { ($nasid:expr, $widget:expr) => { if ($widget) == 0 { $crate::NODE_BWIN_BASE!($nasid, SWIN0_BIGWIN) } else { RAW_NODE_SWIN_BASE($nasid, $widget) } }; }
#[macro_export]
macro_rules! NODE_BWIN_ADDR { ($nasid:expr, $addr:expr) => { (($addr) >= $crate::NODE_BWIN_BASE0!($nasid)) && (($addr) < ($crate::NODE_BWIN_BASE!($nasid, HUB_NUM_BIG_WINDOW) + $crate::BWIN_SIZE!())) }; }
pub const CALIAS_BASE_MARKER: () = (); // CALIAS_BASE = CAC_BASE
#[macro_export]
macro_rules! CALIAS_BASE { () => { CAC_BASE }; }
#[macro_export]
macro_rules! SN0_WIDGET_BASE { ($nasid:expr, $wid:expr) => { $crate::NODE_SWIN_BASE!($nasid, $wid) }; }
#[macro_export]
macro_rules! SABLE_LOG_TRIGGER { ($map:expr) => {}; }
#[macro_export]
macro_rules! KERN_NMI_ADDR { ($nasid:expr, $slice:expr) => { TO_NODE_UNCAC($nasid, IP27_NMI_KREGS_OFFSET + IP27_NMI_KREGS_CPU_SIZE * ($slice)) }; }

#[cfg(feature = "PROM")]
pub const IP27PROM_CORP_MAX: u64 = 32;
#[cfg(feature = "PROM")]
pub const IP27PROM_STACK_SHFT: u32 = 16;
#[cfg(feature = "PROM")]
pub const IP27PROM_STACK_SIZE: u64 = 1 << IP27PROM_STACK_SHFT;
#[cfg(feature = "PROM")]
pub const IP27PROM_INT_LAUNCH: u64 = 10;
#[cfg(feature = "PROM")]
pub const IP27PROM_INT_NETUART: u64 = 12;

#[cfg(feature = "PROM")]
#[macro_export] macro_rules! MISC_PROM_BASE { () => { PHYS_TO_K0(0x01300000) }; }
#[cfg(feature = "PROM")]
pub const MISC_PROM_SIZE: u64 = 0x200000;
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! DIAG_BASE { () => { PHYS_TO_K0(0x01500000) }; }
#[cfg(feature = "PROM")]
pub const DIAG_SIZE: u64 = 0x300000;
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! ROUTE_BASE { () => { PHYS_TO_K0(0x01800000) }; }
#[cfg(feature = "PROM")]
pub const ROUTE_SIZE: u64 = 0x200000;
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! IP27PROM_FLASH_HDR { () => { PHYS_TO_K0(0x01300000) }; }
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! IP27PROM_FLASH_DATA { () => { PHYS_TO_K0(0x01301000) }; }
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! IP27PROM_CORP { () => { PHYS_TO_K0(0x01800000) }; }
#[cfg(feature = "PROM")]
pub const IP27PROM_CORP_SIZE: u64 = 0x10000;
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! IP27PROM_CORP_STK { () => { PHYS_TO_K0(0x01810000) }; }
#[cfg(feature = "PROM")]
pub const IP27PROM_CORP_STKSIZE: u64 = 0x2000;
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! IP27PROM_DECOMP_BUF { () => { PHYS_TO_K0(0x01900000) }; }
#[cfg(feature = "PROM")]
pub const IP27PROM_DECOMP_SIZE: u64 = 0xfff00;
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! IP27PROM_BASE { () => { PHYS_TO_K0(0x01a00000) }; }
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! IP27PROM_BASE_MAPPED { () => { UNCAC_BASE | 0x1fc00000 }; }
#[cfg(feature = "PROM")]
pub const IP27PROM_SIZE_MAX: u64 = 0x100000;
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! IP27PROM_PCFG { () => { PHYS_TO_K0(0x01b00000) }; }
#[cfg(feature = "PROM")]
pub const IP27PROM_PCFG_SIZE: u64 = 0xd0000;
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! IP27PROM_ERRDMP { () => { PHYS_TO_K1(0x01bd0000) }; }
#[cfg(feature = "PROM")]
pub const IP27PROM_ERRDMP_SIZE: u64 = 0xf000;
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! IP27PROM_INIT_START { () => { PHYS_TO_K1(0x01bd0000) }; }
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! IP27PROM_CONSOLE { () => { PHYS_TO_K1(0x01bdf000) }; }
#[cfg(feature = "PROM")]
pub const IP27PROM_CONSOLE_SIZE: u64 = 0x200;
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! IP27PROM_NETUART { () => { PHYS_TO_K1(0x01bdf200) }; }
#[cfg(feature = "PROM")]
pub const IP27PROM_NETUART_SIZE: u64 = 0x100;
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! IP27PROM_UNUSED1 { () => { PHYS_TO_K1(0x01bdf300) }; }
#[cfg(feature = "PROM")]
pub const IP27PROM_UNUSED1_SIZE: u64 = 0x500;
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! IP27PROM_ELSC_BASE_A { () => { PHYS_TO_K0(0x01bdf800) }; }
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! IP27PROM_ELSC_BASE_B { () => { PHYS_TO_K0(0x01bdfc00) }; }
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! IP27PROM_STACK_A { () => { PHYS_TO_K0(0x01be0000) }; }
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! IP27PROM_STACK_B { () => { PHYS_TO_K0(0x01bf0000) }; }
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! IP27PROM_INIT_END { () => { PHYS_TO_K0(0x01c00000) }; }
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! SLAVESTACK_BASE { () => { PHYS_TO_K0(0x01580000) }; }
#[cfg(feature = "PROM")]
pub const SLAVESTACK_SIZE: u64 = 0x40000;
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! ENETBUFS_BASE { () => { PHYS_TO_K0(0x01f80000) }; }
#[cfg(feature = "PROM")]
pub const ENETBUFS_SIZE: u64 = 0x20000;
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! IO6PROM_BASE { () => { PHYS_TO_K0(0x01c00000) }; }
#[cfg(feature = "PROM")]
pub const IO6PROM_SIZE: u64 = 0x400000;
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! IO6PROM_BASE_MAPPED { () => { UNCAC_BASE | 0x11c00000 }; }
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! IO6DPROM_BASE { () => { PHYS_TO_K0(0x01c00000) }; }
#[cfg(feature = "PROM")]
pub const IO6DPROM_SIZE: u64 = 0x200000;
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! NODEBUGUNIX_ADDR { () => { PHYS_TO_K0(0x00019000) }; }
#[cfg(feature = "PROM")]
#[macro_export] macro_rules! DEBUGUNIX_ADDR { () => { PHYS_TO_K0(0x00100000) }; }

pub const IP27PROM_ELSC_SHFT: u32 = 10;
pub const IP27PROM_ELSC_SIZE: u64 = 1 << IP27PROM_ELSC_SHFT;
pub const IO6PROM_STACK_SHFT: u32 = 14;
pub const IO6PROM_STACK_SIZE: u64 = 1 << IO6PROM_STACK_SHFT;

#[macro_export]
macro_rules! FREEMEM_BASE { () => { PHYS_TO_K0(0x2000000) }; }
#[macro_export]
macro_rules! IP27PROM_ENTRY { () => { PHYS_TO_COMPATK1(0x1fc00000) }; }
#[macro_export]
macro_rules! IP27PROM_RESTART { () => { PHYS_TO_COMPATK1(0x1fc00008) }; }
#[macro_export]
macro_rules! IP27PROM_SLAVELOOP { () => { PHYS_TO_COMPATK1(0x1fc00010) }; }
#[macro_export]
macro_rules! IP27PROM_PODMODE { () => { PHYS_TO_COMPATK1(0x1fc00018) }; }
#[macro_export]
macro_rules! IP27PROM_IOC3UARTPOD { () => { PHYS_TO_COMPATK1(0x1fc00020) }; }
#[macro_export]
macro_rules! IP27PROM_FLASHLEDS { () => { PHYS_TO_COMPATK1(0x1fc00028) }; }
#[macro_export]
macro_rules! IP27PROM_REPOD { () => { PHYS_TO_COMPATK1(0x1fc00030) }; }
#[macro_export]
macro_rules! IP27PROM_LAUNCHSLAVE { () => { PHYS_TO_COMPATK1(0x1fc00038) }; }
#[macro_export]
macro_rules! IP27PROM_WAITSLAVE { () => { PHYS_TO_COMPATK1(0x1fc00040) }; }
#[macro_export]
macro_rules! IP27PROM_POLLSLAVE { () => { PHYS_TO_COMPATK1(0x1fc00048) }; }

#[macro_export]
macro_rules! KL_UART_BASE { () => { LOCAL_HUB_ADDR(MD_UREG0_0) }; }
#[macro_export]
macro_rules! KL_UART_CMD { () => { LOCAL_HUB_ADDR(MD_UREG0_0) }; }
#[macro_export]
macro_rules! KL_UART_DATA { () => { LOCAL_HUB_ADDR(MD_UREG0_1) }; }
#[macro_export]
macro_rules! KL_I2C_REG { () => { MD_UREG0_0 }; }

#[cfg(feature = "HUB_ERR_STS_WAR")]
pub const CACHE_ERR_EFRAME: u64 = 0x480;
#[cfg(not(feature = "HUB_ERR_STS_WAR"))]
pub const CACHE_ERR_EFRAME: u64 = 0x400;
pub const CACHE_ERR_SP_PTR: u64 = 0x1000 - 32;
pub const CACHE_ERR_IBASE_PTR: u64 = 0x1000 - 40;
pub const CACHE_ERR_SP: u64 = CACHE_ERR_SP_PTR - 16;

#[macro_export]
macro_rules! CACHE_ERR_ECCFRAME { () => { $crate::CACHE_ERR_EFRAME + EF_SIZE }; }
#[macro_export]
macro_rules! CACHE_ERR_AREA_SIZE { () => { ARCS_SPB_OFFSET - $crate::CACHE_ERR_EFRAME }; }
pub const _ARCSPROM: bool = true;

#[cfg(feature = "HUB_ERR_STS_WAR")]
pub const OLD_ERR_STS_WAR_OFFSET_MARKER: () = (); // (MD_MEM_BANKS * MD_BANK_SIZE) - 0x100
#[cfg(feature = "HUB_ERR_STS_WAR")]
#[macro_export]
macro_rules! ERR_STS_WAR_REGISTER { () => { IIO_IIBUSERR }; }
#[cfg(feature = "HUB_ERR_STS_WAR")]
#[macro_export]
macro_rules! ERR_STS_WAR_ADDR { () => { LOCAL_HUB_ADDR(IIO_IIBUSERR) }; }
#[cfg(feature = "HUB_ERR_STS_WAR")]
#[macro_export]
macro_rules! ERR_STS_WAR_PHYSADDR { () => { TO_PHYS(ERR_STS_WAR_ADDR!() as usize) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
