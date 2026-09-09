/* Translated from asm/sn/addrs.h.  Required architecture constants and types
 * are supplied by the surrounding translation unit. */

/* UINT64_CAST is an identity in Rust; callers should provide integer values. */

macro_rules! NASID_GET_META { ($n:expr) => { ($n) >> NASID_LOCAL_BITS }; }
#[cfg(CONFIG_SGI_IP27)]
macro_rules! NASID_GET_LOCAL { ($n:expr) => { ($n) & 0xf }; }
macro_rules! NASID_MAKE { ($m:expr, $l:expr) => { (($m) << NASID_LOCAL_BITS) | ($l) }; }
macro_rules! NODE_ADDRSPACE_MASK { () => { NODE_ADDRSPACE_SIZE - 1 }; }
macro_rules! TO_NODE_ADDRSPACE { ($pa:expr) => { ($pa) & NODE_ADDRSPACE_MASK!() }; }
macro_rules! CHANGE_ADDR_NASID { ($pa:expr, $nasid:expr) => { (($pa) & !NASID_MASK) | (($nasid) << NASID_SHFT) }; }

macro_rules! NODE_OFFSET { ($n:expr) => { ($n) << NODE_SIZE_BITS }; }
macro_rules! NODE_CAC_BASE { ($n:expr) => { CAC_BASE + NODE_OFFSET!($n) }; }
macro_rules! NODE_HSPEC_BASE { ($n:expr) => { HSPEC_BASE + NODE_OFFSET!($n) }; }
macro_rules! NODE_IO_BASE { ($n:expr) => { IO_BASE + NODE_OFFSET!($n) }; }
macro_rules! NODE_MSPEC_BASE { ($n:expr) => { MSPEC_BASE + NODE_OFFSET!($n) }; }
macro_rules! NODE_UNCAC_BASE { ($n:expr) => { UNCAC_BASE + NODE_OFFSET!($n) }; }
macro_rules! TO_NODE { ($n:expr, $x:expr) => { NODE_OFFSET!($n) | ($x) }; }
macro_rules! TO_NODE_CAC { ($n:expr, $x:expr) => { NODE_CAC_BASE!($n) | (($x) & TO_PHYS_MASK) }; }
macro_rules! TO_NODE_UNCAC { ($n:expr, $x:expr) => { NODE_UNCAC_BASE!($n) | (($x) & TO_PHYS_MASK) }; }
macro_rules! TO_NODE_MSPEC { ($n:expr, $x:expr) => { NODE_MSPEC_BASE!($n) | (($x) & TO_PHYS_MASK) }; }
macro_rules! TO_NODE_HSPEC { ($n:expr, $x:expr) => { NODE_HSPEC_BASE!($n) | (($x) & TO_PHYS_MASK) }; }
macro_rules! RAW_NODE_SWIN_BASE { ($nasid:expr, $widget:expr) => { NODE_IO_BASE!($nasid) + (($widget) << SWIN_SIZE_BITS) }; }
macro_rules! WIDGETID_GET { ($addr:expr) => { (((($addr) >> SWIN_SIZE_BITS) & 0xff) as u8) }; }

const SWIN_SIZE_BITS: u64 = 24;
macro_rules! SWIN_SIZE { () => { 1u64 << 24 }; }
macro_rules! SWIN_SIZEMASK { () => { SWIN_SIZE!() - 1 }; }
const SWIN_WIDGET_MASK: u64 = 0xF;
macro_rules! SWIN_WIDGETADDR { ($addr:expr) => { ($addr) & SWIN_SIZEMASK!() }; }
macro_rules! SWIN_WIDGETNUM { ($addr:expr) => { (($addr >> SWIN_SIZE_BITS) & SWIN_WIDGET_MASK) }; }
macro_rules! NODE_SWIN_ADDR { ($nasid:expr, $addr:expr) => { (($addr) >= NODE_SWIN_BASE!($nasid, 0)) && (($addr) < (NODE_SWIN_BASE!($nasid, HUB_NUM_WIDGET) + SWIN_SIZE!())) }; }

macro_rules! UALIAS_BASE { () => { HSPEC_BASE }; }
const UALIAS_SIZE: u64 = 0x10000000;
macro_rules! UALIAS_LIMIT { () => { UALIAS_BASE!() + UALIAS_SIZE }; }
#[cfg(CONFIG_SGI_IP27)]
const UALIAS_FLIP_BASE: u64 = UALIAS_BASE!();
#[cfg(CONFIG_SGI_IP27)]
const UALIAS_FLIP_SIZE: u64 = 0x20000;
#[cfg(CONFIG_SGI_IP27)]
const UALIAS_FLIP_BIT: u64 = 0x10000;
#[cfg(CONFIG_SGI_IP27)]
macro_rules! UALIAS_FLIP_ADDR { ($x:expr) => { if cputoslice(smp_processor_id()) != 0 { ($x) ^ UALIAS_FLIP_BIT } else { $x } }; }
#[cfg(CONFIG_SGI_IP27)]
macro_rules! LBOOT_BASE { () => { HSPEC_BASE + 0x10000000 }; }
#[cfg(CONFIG_SGI_IP27)]
const LBOOT_SIZE: u64 = 0x10000000;
#[cfg(CONFIG_SGI_IP27)]
macro_rules! LBOOT_LIMIT { () => { LBOOT_BASE!() + LBOOT_SIZE }; }
#[cfg(CONFIG_SGI_IP27)]
const LBOOT_STRIDE: u64 = 0;

const HUB_REGISTER_WIDGET: u64 = 1;
macro_rules! IALIAS_BASE { () => { NODE_SWIN_BASE!(0, HUB_REGISTER_WIDGET) }; }
const IALIAS_SIZE: u64 = 0x800000;
macro_rules! IS_IALIAS { ($a:expr) => { (($a) >= IALIAS_BASE!()) && (($a) < (IALIAS_BASE!() + IALIAS_SIZE)) }; }

#[cfg(CONFIG_SGI_IP27)]
const RBOOT_SIZE: u64 = 0x10000000;
#[cfg(CONFIG_SGI_IP27)]
macro_rules! NODE_RBOOT_BASE { ($n:expr) => { NODE_HSPEC_BASE!($n) + 0x30000000 }; }
#[cfg(CONFIG_SGI_IP27)]
macro_rules! NODE_RBOOT_LIMIT { ($n:expr) => { NODE_RBOOT_BASE!($n) + RBOOT_SIZE }; }

macro_rules! NODE_BDOOR_BASE { ($n:expr) => { NODE_HSPEC_BASE!($n) + (NODE_ADDRSPACE_SIZE / 2) }; }
macro_rules! NODE_BDECC_BASE { ($n:expr) => { NODE_BDOOR_BASE!($n) }; }
macro_rules! NODE_BDDIR_BASE { ($n:expr) => { NODE_BDOOR_BASE!($n) + (NODE_ADDRSPACE_SIZE / 4) }; }
#[cfg(CONFIG_SGI_IP27)]
macro_rules! BDDIR_ENTRY_LO { ($pa:expr) => { (HSPEC_BASE + NODE_ADDRSPACE_SIZE * 3 / 4 + 0x200) | (($pa) & NASID_MASK) | ((($pa) >> 2) & BDDIR_UPPER_MASK) | (((($pa) >> 3) & 0x1f) << 4) }; }
#[cfg(CONFIG_SGI_IP27)]
macro_rules! BDDIR_ENTRY_HI { ($pa:expr) => { (HSPEC_BASE + NODE_ADDRSPACE_SIZE * 3 / 4 + 0x208) | (($pa) & NASID_MASK) | ((($pa) >> 2) & BDDIR_UPPER_MASK) | (((($pa) >> 3) & 0x1f) << 4) }; }
#[cfg(CONFIG_SGI_IP27)]
macro_rules! BDPRT_ENTRY { ($pa:expr, $rgn:expr) => { (HSPEC_BASE + NODE_ADDRSPACE_SIZE * 3 / 4) | (($pa) & NASID_MASK) | ((($pa) >> 2) & BDDIR_UPPER_MASK) | (($rgn) << 3) }; }
#[cfg(CONFIG_SGI_IP27)]
macro_rules! BDPRT_ENTRY_ADDR { ($pa:expr, $rgn:expr) => { BDPRT_ENTRY!($pa, $rgn) }; }
#[cfg(CONFIG_SGI_IP27)]
macro_rules! BDPRT_ENTRY_S { ($pa:expr, $rgn:expr, $val:expr) => { unsafe { *((BDPRT_ENTRY!($pa, $rgn)) as *mut __psunsigned_t) = $val } }; }
#[cfg(CONFIG_SGI_IP27)]
macro_rules! BDPRT_ENTRY_L { ($pa:expr, $rgn:expr) => { unsafe { *((BDPRT_ENTRY!($pa, $rgn)) as *const __psunsigned_t) } }; }
#[cfg(CONFIG_SGI_IP27)]
macro_rules! BDECC_ENTRY { ($pa:expr) => { (HSPEC_BASE + NODE_ADDRSPACE_SIZE / 2) | (($pa) & NASID_MASK) | ((($pa) >> 2) & BDECC_UPPER_MASK) | (((($pa) >> 3) & 3)) }; }
#[cfg(CONFIG_SGI_IP27)]
macro_rules! BDADDR_IS_DIR { ($ba:expr) => { (($ba) & 0x200) != 0 }; }
#[cfg(CONFIG_SGI_IP27)]
macro_rules! BDADDR_IS_PRT { ($ba:expr) => { (($ba) & 0x200) == 0 }; }
#[cfg(CONFIG_SGI_IP27)]
macro_rules! BDDIR_TO_MEM { ($ba:expr) => { (($ba) & NASID_MASK) | ((($ba) & BDDIR_UPPER_MASK) << 2) | (((($ba) & (0x1f << 4)) << 3)) }; }
#[cfg(CONFIG_SGI_IP27)]
macro_rules! BDPRT_TO_MEM { ($ba:expr) => { (($ba) & NASID_MASK) | ((($ba) & BDDIR_UPPER_MASK) << 2) }; }
#[cfg(CONFIG_SGI_IP27)]
macro_rules! BDECC_TO_MEM { ($ba:expr) => { (($ba) & NASID_MASK) | ((($ba) & BDECC_UPPER_MASK) << 2) | ((($ba) & 3) << 3) }; }

macro_rules! LOCAL_HUB_ADDR { ($x:expr) => { IALIAS_BASE!() + ($x) }; }
macro_rules! REMOTE_HUB_ADDR { ($n:expr, $x:expr) => { NODE_SWIN_BASE!($n, 1) + 0x800000 + ($x) }; }
macro_rules! LOCAL_HUB_PTR { ($x:expr) => { LOCAL_HUB_ADDR!($x) as *mut u64 }; }
macro_rules! REMOTE_HUB_PTR { ($n:expr, $x:expr) => { REMOTE_HUB_ADDR!($n, $x) as *mut u64 }; }
macro_rules! LOCAL_HUB_L { ($r:expr) => { unsafe { __raw_readq(LOCAL_HUB_PTR!($r)) } }; }
macro_rules! LOCAL_HUB_S { ($r:expr, $d:expr) => { unsafe { __raw_writeq($d, LOCAL_HUB_PTR!($r)) } }; }
macro_rules! REMOTE_HUB_L { ($n:expr, $r:expr) => { unsafe { __raw_readq(REMOTE_HUB_PTR!($n, $r)) } }; }
macro_rules! REMOTE_HUB_S { ($n:expr, $r:expr, $d:expr) => { unsafe { __raw_writeq($d, REMOTE_HUB_PTR!($n, $r)) } }; }

const PHYS_RAMBASE: u64 = 0x0;
macro_rules! K0_RAMBASE { () => { PHYS_TO_K0!(PHYS_RAMBASE) }; }
macro_rules! EX_HANDLER_OFFSET { ($slice:expr) => { ($slice) << 16 }; }
macro_rules! EX_HANDLER_ADDR { ($nasid:expr, $slice:expr) => { PHYS_TO_K0!(NODE_OFFSET!($nasid) | EX_HANDLER_OFFSET!($slice)) }; }
const EX_HANDLER_SIZE: u64 = 0x0400;
macro_rules! EX_FRAME_OFFSET { ($slice:expr) => { (($slice) << 16) | 0x400 }; }
macro_rules! EX_FRAME_ADDR { ($nasid:expr, $slice:expr) => { PHYS_TO_K0!(NODE_OFFSET!($nasid) | EX_FRAME_OFFSET!($slice)) }; }
const EX_FRAME_SIZE: u64 = 0x0c00;
const ARCS_SPB_OFFSET: u64 = 0x1000;
macro_rules! ARCS_SPB_ADDR { ($nasid:expr) => { PHYS_TO_K0!(NODE_OFFSET!($nasid) | ARCS_SPB_OFFSET) }; }
const ARCS_SPB_SIZE: u64 = 0x0400;
const KLDIR_OFFSET: u64 = 0x2000;
macro_rules! KLDIR_ADDR { ($nasid:expr) => { TO_NODE_UNCAC!($nasid, KLDIR_OFFSET) }; }
const KLDIR_SIZE: u64 = 0x0400;

const KLI_LAUNCH: usize = 0;
const KLI_KLCONFIG: usize = 1;
const KLI_NMI: usize = 2;
const KLI_GDA: usize = 3;
const KLI_FREEMEM: usize = 4;
const KLI_SYMMON_STK: usize = 5;
const KLI_PI_ERROR: usize = 6;
const KLI_KERN_VARS: usize = 7;
const KLI_KERN_XP: usize = 8;
const KLI_KERN_PARTID: usize = 9;

macro_rules! KLD_BASE { ($nasid:expr) => { KLDIR_ADDR!($nasid) as *mut kldir_ent_t }; }
macro_rules! KLD_LAUNCH { ($nasid:expr) => { unsafe { KLD_BASE!($nasid).add(KLI_LAUNCH) } }; }
macro_rules! KLD_NMI { ($nasid:expr) => { unsafe { KLD_BASE!($nasid).add(KLI_NMI) } }; }
macro_rules! KLD_KLCONFIG { ($nasid:expr) => { unsafe { KLD_BASE!($nasid).add(KLI_KLCONFIG) } }; }
macro_rules! KLD_PI_ERROR { ($nasid:expr) => { unsafe { KLD_BASE!($nasid).add(KLI_PI_ERROR) } }; }
macro_rules! KLD_GDA { ($nasid:expr) => { unsafe { KLD_BASE!($nasid).add(KLI_GDA) } }; }
macro_rules! KLD_SYMMON_STK { ($nasid:expr) => { unsafe { KLD_BASE!($nasid).add(KLI_SYMMON_STK) } }; }
macro_rules! KLD_FREEMEM { ($nasid:expr) => { unsafe { KLD_BASE!($nasid).add(KLI_FREEMEM) } }; }
macro_rules! KLD_KERN_VARS { ($nasid:expr) => { unsafe { KLD_BASE!($nasid).add(KLI_KERN_VARS) } }; }
macro_rules! KLD_KERN_XP { ($nasid:expr) => { unsafe { KLD_BASE!($nasid).add(KLI_KERN_XP) } }; }
macro_rules! KLD_KERN_PARTID { ($nasid:expr) => { unsafe { KLD_BASE!($nasid).add(KLI_KERN_PARTID) } }; }

macro_rules! LAUNCH_OFFSET { ($nasid:expr, $slice:expr) => { unsafe { (*KLD_LAUNCH!($nasid)).offset + (*KLD_LAUNCH!($nasid)).stride * ($slice) } }; }
macro_rules! LAUNCH_ADDR { ($nasid:expr, $slice:expr) => { TO_NODE_UNCAC!($nasid, LAUNCH_OFFSET!($nasid, $slice)) }; }
macro_rules! LAUNCH_SIZE { ($nasid:expr) => { unsafe { (*KLD_LAUNCH!($nasid)).size } }; }
macro_rules! SN_NMI_OFFSET { ($nasid:expr, $slice:expr) => { unsafe { (*KLD_NMI!($nasid)).offset + (*KLD_NMI!($nasid)).stride * ($slice) } }; }
macro_rules! NMI_ADDR { ($nasid:expr, $slice:expr) => { TO_NODE_UNCAC!($nasid, SN_NMI_OFFSET!($nasid, $slice)) }; }
macro_rules! NMI_SIZE { ($nasid:expr) => { unsafe { (*KLD_NMI!($nasid)).size } }; }
macro_rules! KLCONFIG_OFFSET { ($nasid:expr) => { unsafe { (*KLD_KLCONFIG!($nasid)).offset } }; }
macro_rules! KLCONFIG_ADDR { ($nasid:expr) => { TO_NODE_UNCAC!($nasid, KLCONFIG_OFFSET!($nasid)) }; }
macro_rules! KLCONFIG_SIZE { ($nasid:expr) => { unsafe { (*KLD_KLCONFIG!($nasid)).size } }; }
macro_rules! GDA_ADDR { ($nasid:expr) => { unsafe { (*KLD_GDA!($nasid)).pointer } }; }
macro_rules! GDA_SIZE { ($nasid:expr) => { unsafe { (*KLD_GDA!($nasid)).size } }; }
macro_rules! SYMMON_STK_OFFSET { ($nasid:expr, $slice:expr) => { unsafe { (*KLD_SYMMON_STK!($nasid)).offset + (*KLD_SYMMON_STK!($nasid)).stride * ($slice) } }; }
macro_rules! SYMMON_STK_STRIDE { ($nasid:expr) => { unsafe { (*KLD_SYMMON_STK!($nasid)).stride } }; }
macro_rules! SYMMON_STK_ADDR { ($nasid:expr, $slice:expr) => { TO_NODE_CAC!($nasid, SYMMON_STK_OFFSET!($nasid, $slice)) }; }
macro_rules! SYMMON_STK_SIZE { ($nasid:expr) => { unsafe { (*KLD_SYMMON_STK!($nasid)).stride } }; }
macro_rules! SYMMON_STK_END { ($nasid:expr) => { SYMMON_STK_ADDR!($nasid, 0) + unsafe { (*KLD_SYMMON_STK!($nasid)).size } }; }
macro_rules! NODE_OFFSET_TO_K0 { ($nasid:expr, $off:expr) => { PHYS_TO_K0!((NODE_OFFSET!($nasid) + ($off)) | CAC_BASE) }; }
macro_rules! NODE_OFFSET_TO_K1 { ($nasid:expr, $off:expr) => { TO_UNCAC!((NODE_OFFSET!($nasid) + ($off)) | UNCAC_BASE) }; }
macro_rules! KERN_VARS_ADDR { ($nasid:expr) => { unsafe { (*KLD_KERN_VARS!($nasid)).pointer } }; }
macro_rules! KERN_VARS_SIZE { ($nasid:expr) => { unsafe { (*KLD_KERN_VARS!($nasid)).size } }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
