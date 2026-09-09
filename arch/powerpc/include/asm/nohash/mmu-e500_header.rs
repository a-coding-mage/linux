/* SPDX-License-Identifier: GPL-2.0 */
/* Freescale Book-E/Book-3e (ISA 2.06+) MMU support */

pub const BOOK3E_PAGESZ_1K: u32 = 0;
pub const BOOK3E_PAGESZ_2K: u32 = 1;
pub const BOOK3E_PAGESZ_4K: u32 = 2;
pub const BOOK3E_PAGESZ_8K: u32 = 3;
pub const BOOK3E_PAGESZ_16K: u32 = 4;
pub const BOOK3E_PAGESZ_32K: u32 = 5;
pub const BOOK3E_PAGESZ_64K: u32 = 6;
pub const BOOK3E_PAGESZ_128K: u32 = 7;
pub const BOOK3E_PAGESZ_256K: u32 = 8;
pub const BOOK3E_PAGESZ_512K: u32 = 9;
pub const BOOK3E_PAGESZ_1M: u32 = 10;
pub const BOOK3E_PAGESZ_2M: u32 = 11;
pub const BOOK3E_PAGESZ_4M: u32 = 12;
pub const BOOK3E_PAGESZ_8M: u32 = 13;
pub const BOOK3E_PAGESZ_16M: u32 = 14;
pub const BOOK3E_PAGESZ_32M: u32 = 15;
pub const BOOK3E_PAGESZ_64M: u32 = 16;
pub const BOOK3E_PAGESZ_128M: u32 = 17;
pub const BOOK3E_PAGESZ_256M: u32 = 18;
pub const BOOK3E_PAGESZ_512M: u32 = 19;
pub const BOOK3E_PAGESZ_1GB: u32 = 20;
pub const BOOK3E_PAGESZ_2GB: u32 = 21;
pub const BOOK3E_PAGESZ_4GB: u32 = 22;
pub const BOOK3E_PAGESZ_8GB: u32 = 23;
pub const BOOK3E_PAGESZ_16GB: u32 = 24;
pub const BOOK3E_PAGESZ_32GB: u32 = 25;
pub const BOOK3E_PAGESZ_64GB: u32 = 26;
pub const BOOK3E_PAGESZ_128GB: u32 = 27;
pub const BOOK3E_PAGESZ_256GB: u32 = 28;
pub const BOOK3E_PAGESZ_512GB: u32 = 29;
pub const BOOK3E_PAGESZ_1TB: u32 = 30;
pub const BOOK3E_PAGESZ_2TB: u32 = 31;

pub const MAS0_TLBSEL_MASK: u32 = 0x30000000;
pub const MAS0_TLBSEL_SHIFT: u32 = 28;
#[inline] pub const fn MAS0_TLBSEL(x: u32) -> u32 { (x << MAS0_TLBSEL_SHIFT) & MAS0_TLBSEL_MASK }
#[inline] pub const fn MAS0_GET_TLBSEL(mas0: u32) -> u32 { (mas0 & MAS0_TLBSEL_MASK) >> MAS0_TLBSEL_SHIFT }
pub const MAS0_ESEL_MASK: u32 = 0x0FFF0000;
pub const MAS0_ESEL_SHIFT: u32 = 16;
#[inline] pub const fn MAS0_ESEL(x: u32) -> u32 { (x << MAS0_ESEL_SHIFT) & MAS0_ESEL_MASK }
#[inline] pub const fn MAS0_NV(x: u32) -> u32 { x & 0x00000FFF }
pub const MAS0_HES: u32 = 0x00004000;
pub const MAS0_WQ_ALLWAYS: u32 = 0;
pub const MAS0_WQ_COND: u32 = 0x00001000;
pub const MAS0_WQ_CLR_RSRV: u32 = 0x00002000;

pub const MAS1_VALID: u32 = 0x80000000;
pub const MAS1_IPROT: u32 = 0x40000000;
#[inline] pub const fn MAS1_TID(x: u32) -> u32 { (x << 16) & 0x3FFF0000 }
pub const MAS1_IND: u32 = 0x00002000;
pub const MAS1_TS: u32 = 0x00001000;
pub const MAS1_TSIZE_MASK: u32 = 0x00000f80;
pub const MAS1_TSIZE_SHIFT: u32 = 7;
#[inline] pub const fn MAS1_TSIZE(x: u32) -> u32 { (x << MAS1_TSIZE_SHIFT) & MAS1_TSIZE_MASK }
#[inline] pub const fn MAS1_GET_TSIZE(mas1: u32) -> u32 { (mas1 & MAS1_TSIZE_MASK) >> MAS1_TSIZE_SHIFT }

pub const MAS2_EPN: usize = !0xFFFusize;
pub const MAS2_X0: u32 = 0x40;
pub const MAS2_X1: u32 = 0x20;
pub const MAS2_W: u32 = 0x10;
pub const MAS2_I: u32 = 0x08;
pub const MAS2_M: u32 = 0x04;
pub const MAS2_G: u32 = 0x02;
pub const MAS2_E: u32 = 0x01;
pub const MAS2_WIMGE_MASK: u32 = 0x1f;
#[inline] pub const fn MAS2_EPN_MASK(size: u32) -> usize { !0usize << (size + 10) }

pub const MAS3_RPN: u32 = 0xFFFFF000;
pub const MAS3_U0: u32 = 0x200; pub const MAS3_U1: u32 = 0x100; pub const MAS3_U2: u32 = 0x80; pub const MAS3_U3: u32 = 0x40;
pub const MAS3_UX: u32 = 0x20; pub const MAS3_SX: u32 = 0x10; pub const MAS3_UW: u32 = 0x08; pub const MAS3_SW: u32 = 0x04;
pub const MAS3_UR: u32 = 0x02; pub const MAS3_SR: u32 = 0x01; pub const MAS3_BAP_MASK: u32 = 0x3f;
pub const MAS3_SPSIZE: u32 = 0x3e; pub const MAS3_SPSIZE_SHIFT: u32 = 1;

pub const MAS4_TLBSEL_MASK: u32 = MAS0_TLBSEL_MASK;
#[inline] pub const fn MAS4_TLBSELD(x: u32) -> u32 { MAS0_TLBSEL(x) }
pub const MAS4_INDD: u32 = 0x8000; pub const MAS4_X0D: u32 = 0x40; pub const MAS4_X1D: u32 = 0x20;
pub const MAS4_WD: u32 = 0x10; pub const MAS4_ID: u32 = 0x08; pub const MAS4_MD: u32 = 0x04; pub const MAS4_GD: u32 = 0x02; pub const MAS4_ED: u32 = 0x01;
pub const MAS4_WIMGED_MASK: u32 = 0x1f; pub const MAS4_WIMGED_SHIFT: u32 = 0; pub const MAS4_VLED: u32 = MAS4_X1D; pub const MAS4_ACMD: u32 = 0xc0; pub const MAS4_ACMD_SHIFT: u32 = 6; pub const MAS4_TSIZED_MASK: u32 = 0xf80; pub const MAS4_TSIZED_SHIFT: u32 = 7;
#[inline] pub const fn MAS4_TSIZED(x: u32) -> u32 { MAS1_TSIZE(x) }
pub const MAS5_SGS: u32 = 0x80000000; pub const MAS6_SPID0: u32 = 0x3FFF0000; pub const MAS6_SPID1: u32 = 0x00007FFE;
#[inline] pub const fn MAS6_ISIZE(x: u32) -> u32 { MAS1_TSIZE(x) }
pub const MAS6_SAS: u32 = 1; pub const MAS6_SPID: u32 = MAS6_SPID0; pub const MAS6_SIND: u32 = 2; pub const MAS6_SIND_SHIFT: u32 = 1; pub const MAS6_SPID_MASK: u32 = 0x3fff0000; pub const MAS6_SPID_SHIFT: u32 = 16; pub const MAS6_ISIZE_MASK: u32 = 0xf80; pub const MAS6_ISIZE_SHIFT: u32 = 7;
pub const MAS7_RPN: u32 = 0xFFFFFFFF; pub const MAS8_TGS: u32 = 0x80000000; pub const MAS8_VF: u32 = 0x40000000; pub const MAS8_TLPID: u32 = 0xff;

pub const MMUCFG_MAVN: u32 = 3; pub const MMUCFG_MAVN_V1: u32 = 0; pub const MMUCFG_MAVN_V2: u32 = 1; pub const MMUCFG_NTLBS: u32 = 0xc; pub const MMUCFG_PIDSIZE: u32 = 0x7c0; pub const MMUCFG_TWC: u32 = 0x8000; pub const MMUCFG_LRAT: u32 = 0x10000; pub const MMUCFG_RASIZE: u32 = 0xfe0000; pub const MMUCFG_LPIDSIZE: u32 = 0x0f000000;
pub const MMUCSR0_TLB1FI: u32 = 2; pub const MMUCSR0_TLB0FI: u32 = 4; pub const MMUCSR0_TLB2FI: u32 = 0x40; pub const MMUCSR0_TLB3FI: u32 = 0x20; pub const MMUCSR0_TLBFI: u32 = MMUCSR0_TLB0FI | MMUCSR0_TLB1FI | MMUCSR0_TLB2FI | MMUCSR0_TLB3FI;
pub const MMUCSR0_TLB0PS: u32 = 0x780; pub const MMUCSR0_TLB1PS: u32 = 0x7800; pub const MMUCSR0_TLB2PS: u32 = 0x78000; pub const MMUCSR0_TLB3PS: u32 = 0x780000;
pub const MMUCFG_MAVN_NASK: u32 = 3; pub const MMUCFG_MAVN_V1_0: u32 = 0; pub const MMUCFG_MAVN_V2_0: u32 = 1; pub const MMUCFG_NTLB_MASK: u32 = 0xc; pub const MMUCFG_NTLB_SHIFT: u32 = 2; pub const MMUCFG_PIDSIZE_MASK: u32 = 0x7c0; pub const MMUCFG_PIDSIZE_SHIFT: u32 = 6; pub const MMUCFG_RASIZE_MASK: u32 = 0xfe0000; pub const MMUCFG_RASIZE_SHIFT: u32 = 17; pub const MMUCFG_LPIDSIZE_MASK: u32 = 0x0f000000; pub const MMUCFG_LPIDSIZE_SHIFT: u32 = 24;
pub const TLBnCFG_N_ENTRY: u32 = 0xfff; pub const TLBnCFG_HES: u32 = 0x2000; pub const TLBnCFG_IPROT: u32 = 0x8000; pub const TLBnCFG_GTWE: u32 = 0x10000; pub const TLBnCFG_IND: u32 = 0x20000; pub const TLBnCFG_PT: u32 = 0x40000; pub const TLBnCFG_MINSIZE: u32 = 0xf00000; pub const TLBnCFG_MINSIZE_SHIFT: u32 = 20; pub const TLBnCFG_MAXSIZE: u32 = 0xf0000; pub const TLBnCFG_MAXSIZE_SHIFT: u32 = 16; pub const TLBnCFG_ASSOC: u32 = 0xff000000; pub const TLBnCFG_ASSOC_SHIFT: u32 = 24;
pub const TLBnPS_4K: u32 = 0x4; pub const TLBnPS_8K: u32 = 0x8; pub const TLBnPS_16K: u32 = 0x10; pub const TLBnPS_32K: u32 = 0x20; pub const TLBnPS_64K: u32 = 0x40; pub const TLBnPS_128K: u32 = 0x80; pub const TLBnPS_256K: u32 = 0x100; pub const TLBnPS_512K: u32 = 0x200; pub const TLBnPS_1M: u32 = 0x400; pub const TLBnPS_2M: u32 = 0x800; pub const TLBnPS_4M: u32 = 0x1000; pub const TLBnPS_8M: u32 = 0x2000; pub const TLBnPS_16M: u32 = 0x4000; pub const TLBnPS_32M: u32 = 0x8000; pub const TLBnPS_64M: u32 = 0x10000; pub const TLBnPS_128M: u32 = 0x20000; pub const TLBnPS_256M: u32 = 0x40000; pub const TLBnPS_512M: u32 = 0x80000; pub const TLBnPS_1G: u32 = 0x100000; pub const TLBnPS_2G: u32 = 0x200000; pub const TLBnPS_4G: u32 = 0x400000; pub const TLBnPS_8G: u32 = 0x800000; pub const TLBnPS_16G: u32 = 0x1000000; pub const TLBnPS_32G: u32 = 0x2000000; pub const TLBnPS_64G: u32 = 0x4000000; pub const TLBnPS_128G: u32 = 0x8000000; pub const TLBnPS_256G: u32 = 0x10000000;
pub const TLBILX_T_ALL: u32 = 0; pub const TLBILX_T_TID: u32 = 1; pub const TLBILX_T_FULLMATCH: u32 = 3; pub const TLBILX_T_CLASS0: u32 = 4; pub const TLBILX_T_CLASS1: u32 = 5; pub const TLBILX_T_CLASS2: u32 = 6; pub const TLBILX_T_CLASS3: u32 = 7;

// CONFIG_SMP || CONFIG_PPC_E500MC determines this build-time mapping.
pub const MAS2_M_IF_NEEDED: u32 = MAS2_M;

pub type CInt = i32;
#[repr(C)] pub struct MmContext { pub id: u32, pub active: u32, pub vdso: *mut core::ffi::c_void }
pub type mm_context_t = MmContext;
pub const MMU_PAGE_SIZE_DIRECT: u32 = 0x1; pub const MMU_PAGE_SIZE_INDIRECT: u32 = 0x2;
#[repr(C)] pub struct MmuPsizeDef { pub shift: u32, pub flags: u32 }
extern "C" { pub static mut tlbcam_index: u32; pub static mut mmu_psize_defs: [MmuPsizeDef; MMU_PAGE_COUNT]; pub static mut mmu_linear_psize: CInt; pub static mut mmu_vmemmap_psize: CInt; }
extern "C" { pub fn BUG() -> !; }
#[inline] pub unsafe fn shift_to_mmu_psize(shift: u32) -> CInt { let mut psize = 0; while psize < MMU_PAGE_COUNT as CInt { if mmu_psize_defs[psize as usize].shift == shift { return psize; } psize += 1; } -1 }
#[inline] pub unsafe fn mmu_psize_to_shift(mmu_psize: u32) -> u32 { if mmu_psize_defs[mmu_psize as usize].shift != 0 { mmu_psize_defs[mmu_psize as usize].shift } else { BUG() } }
// CONFIG_PPC_4K_PAGES: mmu_virtual_psize = MMU_PAGE_4K; otherwise unsupported.
pub const mmu_virtual_psize: u32 = MMU_PAGE_4K;
#[repr(C)] pub struct TlbCoreData { pub lock: u8, pub esel_next: u8, pub esel_max: u8, pub esel_first: u8 }
// CONFIG_PPC64 declarations and constants are conditional in the source.
pub const PPC_HTW_NONE: u32 = 0; pub const PPC_HTW_E6500: u32 = 1; pub const MAX_PHYSMEM_BITS: u32 = 44;
pub const HUGETLB_NEED_PRELOAD: bool = true;
pub const mmu_cleanup_all: *const core::ffi::c_void = core::ptr::null();
extern "C" { pub static mut next_tlbcam_idx: CInt; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
