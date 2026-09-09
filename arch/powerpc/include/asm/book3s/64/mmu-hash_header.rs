/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of asm/book3s/64/mmu-hash.h. External kernel symbols are
 * intentionally left as dependencies of the including translation unit. */

pub const SLB_NUM_BOLTED: usize = 2;
pub const SLB_CACHE_ENTRIES: usize = 8;
pub const SLB_MIN_SIZE: usize = 32;
pub const SLB_ESID_V: u64 = 0x0000_0000_0800_0000;
pub const SLB_VSID_SHIFT: u32 = 12;
pub const SLB_VSID_SHIFT_256M: u32 = SLB_VSID_SHIFT;
pub const SLB_VSID_SHIFT_1T: u32 = 24;
pub const SLB_VSID_SSIZE_SHIFT: u32 = 62;
pub const SLB_VSID_B: u64 = 0xc000_0000_0000_0000;
pub const SLB_VSID_B_256M: u64 = 0;
pub const SLB_VSID_B_1T: u64 = 0x4000_0000_0000_0000;
pub const SLB_VSID_KS: u64 = 0x800;
pub const SLB_VSID_KP: u64 = 0x400;
pub const SLB_VSID_N: u64 = 0x200;
pub const SLB_VSID_L: u64 = 0x100;
pub const SLB_VSID_C: u64 = 0x80;
pub const SLB_VSID_LP: u64 = 0x30;
pub const SLB_VSID_LP_00: u64 = 0;
pub const SLB_VSID_LP_01: u64 = 0x10;
pub const SLB_VSID_LP_10: u64 = 0x20;
pub const SLB_VSID_LP_11: u64 = 0x30;
pub const SLB_VSID_LLP: u64 = SLB_VSID_L | SLB_VSID_LP;
pub const SLB_VSID_KERNEL: u64 = SLB_VSID_KP;
pub const SLB_VSID_USER: u64 = SLB_VSID_KP | SLB_VSID_KS | SLB_VSID_C;
pub const SLBIE_C: u32 = 0x0800_0000;
pub const SLBIE_SSIZE_SHIFT: u32 = 25;

pub const HPTES_PER_GROUP: usize = 8;
pub const HPTE_V_SSIZE_SHIFT: u32 = 62;
pub const HPTE_V_AVPN_SHIFT: u32 = 7;
pub const HPTE_V_COMMON_BITS: u64 = 0x000f_ffff_ffff_ffff;
pub const HPTE_V_AVPN: u64 = 0x3fff_ffff_ffff_ff80;
pub const HPTE_V_AVPN_3_0: u64 = 0x000f_ffff_ffff_ff80;
pub const HPTE_V_BOLTED: u64 = 0x10;
pub const HPTE_V_LOCK: u64 = 0x8;
pub const HPTE_V_LARGE: u64 = 0x4;
pub const HPTE_V_SECONDARY: u64 = 0x2;
pub const HPTE_V_VALID: u64 = 0x1;
pub const HPTE_R_3_0_SSIZE_SHIFT: u32 = 58;
pub const HPTE_R_3_0_SSIZE_MASK: u64 = 3u64 << HPTE_R_3_0_SSIZE_SHIFT;
pub const HPTE_R_PP0: u64 = 0x8000_0000_0000_0000;
pub const HPTE_R_TS: u64 = 0x4000_0000_0000_0000;
pub const HPTE_R_KEY_HI: u64 = 0x3000_0000_0000_0000;
pub const HPTE_R_KEY_BIT4: u64 = 0x2000_0000_0000_0000;
pub const HPTE_R_KEY_BIT3: u64 = 0x1000_0000_0000_0000;
pub const HPTE_R_RPN_SHIFT: u32 = 12;
pub const HPTE_R_RPN: u64 = 0x0fff_ffff_ffff_f000;
pub const HPTE_R_RPN_3_0: u64 = 0x01ff_ffff_ffff_f000;
pub const HPTE_R_PP: u64 = 3;
pub const HPTE_R_PPP: u64 = 0x8000_0000_0000_0003;
pub const HPTE_R_N: u64 = 4;
pub const HPTE_R_G: u64 = 8;
pub const HPTE_R_M: u64 = 0x10;
pub const HPTE_R_I: u64 = 0x20;
pub const HPTE_R_W: u64 = 0x40;
pub const HPTE_R_WIMG: u64 = 0x78;
pub const HPTE_R_C: u64 = 0x80;
pub const HPTE_R_R: u64 = 0x100;
pub const HPTE_R_KEY_LO: u64 = 0xe00;
pub const HPTE_R_KEY_BIT2: u64 = 0x800;
pub const HPTE_R_KEY_BIT1: u64 = 0x400;
pub const HPTE_R_KEY_BIT0: u64 = 0x200;
pub const HPTE_R_KEY: u64 = HPTE_R_KEY_LO | HPTE_R_KEY_HI;
pub const HPTE_V_1TB_SEG: u64 = 0x4000_0000_0000_0000;
pub const HPTE_V_VRMA_MASK: u64 = 0x0400_1fff_ff00_0000;
pub const PP_RWXX: u64 = 0;
pub const PP_RWRX: u64 = 1;
pub const PP_RWRW: u64 = 2;
pub const PP_RXRX: u64 = 3;
pub const PP_RXXX: u64 = HPTE_R_PP0 | 2;
pub const TLBIEL_INVAL_SEL_MASK: u32 = 0xc00;
pub const TLBIEL_INVAL_PAGE: u32 = 0;
pub const TLBIEL_INVAL_SET_LPID: u32 = 0x800;
pub const TLBIEL_INVAL_SET: u32 = 0xc00;
pub const TLBIEL_INVAL_SET_MASK: u32 = 0xfff000;
pub const TLBIEL_INVAL_SET_SHIFT: u32 = 12;
pub const POWER7_TLB_SETS: usize = 128;
pub const POWER8_TLB_SETS: usize = 512;
pub const POWER9_TLB_SETS_HASH: usize = 256;
pub const POWER9_TLB_SETS_RADIX: usize = 128;

#[repr(C)]
pub struct MmuHashOps {
    pub hpte_invalidate: Option<unsafe extern "C" fn(usize, usize, i32, i32, i32, i32)>,
    pub hpte_updatepp: Option<unsafe extern "C" fn(usize, usize, usize, i32, i32, i32, usize) -> isize>,
    pub hpte_updateboltedpp: Option<unsafe extern "C" fn(usize, usize, i32, i32)>,
    pub hpte_insert: Option<unsafe extern "C" fn(usize, usize, usize, usize, usize, i32, i32, i32) -> isize>,
    pub hpte_remove: Option<unsafe extern "C" fn(usize) -> isize>,
    pub hpte_removebolted: Option<unsafe extern "C" fn(usize, i32, i32) -> i32>,
    pub flush_hash_range: Option<unsafe extern "C" fn(usize, i32)>,
    pub hugepage_invalidate: Option<unsafe extern "C" fn(usize, usize, *mut u8, i32, i32, i32)>,
    pub resize_hpt: Option<unsafe extern "C" fn(usize) -> i32>,
    pub hpte_clear_all: Option<unsafe extern "C" fn()>,
}
extern "C" { pub static mut mmu_hash_ops: MmuHashOps; }

#[repr(C)] pub struct HashPte { pub v: u64, pub r: u64 }
extern "C" { pub static mut htab_address: *mut HashPte; pub static mut htab_size_bytes: usize; pub static mut htab_hash_mask: usize; }

/* The following declarations depend on types/constants supplied by included kernel headers. */
extern "C" {
    pub static mut mmu_psize_defs: [MmuPsizeDef; MMU_PAGE_COUNT];
    pub static mut hpte_page_sizes: [u8; 1 << LP_BITS];
    pub static mut mmu_kernel_ssize: i32; pub static mut mmu_highuser_ssize: i32;
    pub static mut mmu_slb_size: u16; pub static mut tce_alloc_start: usize; pub static mut tce_alloc_end: usize;
    pub static mut mmu_ci_restrictions: i32;
}
#[repr(C)] pub struct MmuPsizeDef { pub shift: u32, pub ap: usize, pub sllp: u64, pub avpnm: usize, pub penc: [u32; 16] }
pub const MMU_SEGSIZE_256M: i32 = 0; pub const MMU_SEGSIZE_1T: i32 = 1; pub const VPN_SHIFT: u32 = 12;
pub const LP_SHIFT: u32 = 12; pub const LP_BITS: usize = 8;
pub const HPTE_LOCAL_UPDATE: usize = 1; pub const HPTE_NOHPTE_UPDATE: usize = 2; pub const HPTE_USE_KERNEL_KEY: usize = 4;

pub unsafe fn shift_to_mmu_psize(shift: u32) -> i32 { for p in 0..MMU_PAGE_COUNT { if mmu_psize_defs[p].shift == shift { return p as i32; } } -1 }
pub unsafe fn mmu_psize_to_shift(p: usize) -> u32 { let s = mmu_psize_defs[p].shift; if s != 0 { s } else { bug() } }
pub unsafe fn ap_to_shift(ap: usize) -> u32 { for p in 0..MMU_PAGE_COUNT { if mmu_psize_defs[p].ap == ap { return mmu_psize_defs[p].shift; } } u32::MAX }
pub unsafe fn get_sllp_encoding(p: usize) -> usize { ((mmu_psize_defs[p].sllp & SLB_VSID_L) >> 6 | (mmu_psize_defs[p].sllp & SLB_VSID_LP) >> 4) as usize }
pub unsafe fn __hpte_page_size(h: usize, l: usize, base: bool) -> usize { if h & HPTE_V_LARGE as usize == 0 { return 1usize << 12; } let lp=(l>>LP_SHIFT)&((1usize<<LP_BITS)-1); let mut i=hpte_page_sizes[lp] as usize; if i==0{return 0} if !base{i>>=4;} 1usize << mmu_psize_defs[i&0xf].shift }
pub unsafe fn hpte_page_size(h: usize,l: usize)->usize { __hpte_page_size(h,l,false) }
pub unsafe fn hpte_base_page_size(h: usize,l: usize)->usize { __hpte_page_size(h,l,true) }

pub const MMU_SEGSIZE_256M_U: usize = 0; pub const MMU_SEGSIZE_1T_U: usize = 1;
pub const VA_BITS: u32=68; pub const CONTEXT_BITS: u32=19;
pub const VSID_MULTIPLIER_256M: usize=12538073; pub const VSID_MULINV_256M: usize=665548017062;
pub const VSID_MULTIPLIER_1T: usize=12538073; pub const VSID_MULINV_1T: usize=209034062;
pub const VRMA_VSID: usize=0x1ffffff; pub const HPTE_V_AVPN_MASK: u64=HPTE_V_AVPN;

pub unsafe fn hpte_encode_avpn(vpn: usize, psize: usize, ssize: i32)->usize { let mut v=(vpn>>(23-VPN_SHIFT)) & !mmu_psize_defs[psize].avpnm; v<<=HPTE_V_AVPN_SHIFT; v|=(ssize as usize)<<HPTE_V_SSIZE_SHIFT; v }
pub fn hpte_old_to_new_v(v:u64)->u64 { v & HPTE_V_COMMON_BITS }
pub fn hpte_old_to_new_r(v:u64,r:u64)->u64 { (r & !HPTE_R_3_0_SSIZE_MASK) | ((v>>HPTE_V_SSIZE_SHIFT)<<HPTE_R_3_0_SSIZE_SHIFT) }
pub fn hpte_new_to_old_v(v:u64,r:u64)->u64 { (v&HPTE_V_COMMON_BITS)|((r&HPTE_R_3_0_SSIZE_MASK)<<(HPTE_V_SSIZE_SHIFT-HPTE_R_3_0_SSIZE_SHIFT)) }
pub fn hpte_new_to_old_r(r:u64)->u64 { r & !HPTE_R_3_0_SSIZE_MASK }
pub unsafe fn hpte_encode_v(vpn:usize,base:usize,actual:usize,ssize:i32)->usize { let mut v=hpte_encode_avpn(vpn,base,ssize); if actual!=MMU_PAGE_4K {v|=HPTE_V_LARGE as usize;} v }
pub unsafe fn hpte_encode_r(pa:usize,base:usize,actual:usize)->usize { if actual==MMU_PAGE_4K {pa & HPTE_R_RPN as usize} else { let penc=mmu_psize_defs[base].penc[actual]; let shift=mmu_psize_defs[actual].shift; (pa & !((1usize<<shift)-1)) | ((penc as usize)<<LP_SHIFT) } }
pub unsafe fn hpt_vpn(ea:usize,vsid:usize,ssize:i32)->usize { let sh=segment_shift(ssize); let mask=(1usize<<(sh-VPN_SHIFT))-1; (vsid<<(sh-VPN_SHIFT))|((ea>>VPN_SHIFT)&mask) }
pub unsafe fn hpt_hash(vpn:usize,shift:u32,ssize:i32)->usize { let (sh,vsidmask) = if ssize==MMU_SEGSIZE_256M {(SID_SHIFT,(1usize<<(SID_SHIFT-VPN_SHIFT))-1)} else {(SID_SHIFT_1T,(1usize<<(SID_SHIFT_1T-VPN_SHIFT))-1)}; let hash=if ssize==MMU_SEGSIZE_256M {(vpn>>(sh-VPN_SHIFT))^((vpn&vsidmask)>>(shift-VPN_SHIFT))} else {let v=vpn>>(sh-VPN_SHIFT); v^(v<<25)^((vpn&vsidmask)>>(shift-VPN_SHIFT))}; hash&0x7fff_ffff_ff }

/* Conditional declarations below retain the C configuration intent. */
extern "C" { pub fn hpte_insert_repeating(hash:usize,vpn:usize,pa:usize,rlags:usize,vflags:usize,psize:i32,ssize:i32)->isize; pub fn __hash_page_4K(ea:usize,access:usize,vsid:usize,ptep:*mut PteT,trap:usize,flags:usize,ssize:i32,subpage:i32)->i32; pub fn __hash_page_64K(ea:usize,access:usize,vsid:usize,ptep:*mut PteT,trap:usize,flags:usize,ssize:i32)->i32; pub fn hash_page(ea:usize,access:usize,trap:usize,dsisr:usize)->i32; }
#[repr(C)] pub struct MmuHashContext { pub user_psize:u16, pub low_slices_psize:[u8; LOW_SLICE_ARRAY_SZ], pub high_slices_psize:[u8; SLICE_ARRAY_SIZE], pub slb_addr_limit:usize }
#[repr(C)] pub struct SlbEntry { pub esid:u64, pub vsid:u64 }
pub unsafe fn vsid_scramble(protovsid:usize,mult:usize,bits:u32)->usize { let m=(1usize<<bits)-1; let mut v=protovsid.wrapping_mul(mult); v=(v>>bits)+(v&m); (v+((v+1)>>bits))&m }
pub unsafe fn user_segment_size(addr:usize)->i32 { if addr >= (1usize<<SID_SHIFT_1T) {mmu_highuser_ssize} else {MMU_SEGSIZE_256M} }
pub unsafe fn get_vsid(context:usize,ea:usize,ssize:i32)->usize { let vb=if mmu_has_feature(MMU_FTR_68_BIT_VA){VA_BITS}else{65}; if ea&EA_MASK>=H_PGTABLE_RANGE{return 0}; if ssize==MMU_SEGSIZE_256M {vsid_scramble((context<<ESID_BITS)|((ea>>SID_SHIFT)&ESID_BITS_MASK),VSID_MULTIPLIER_256M,vb-SID_SHIFT)} else {vsid_scramble((context<<ESID_BITS_1T)|((ea>>SID_SHIFT_1T)&ESID_BITS_1T_MASK),VSID_MULTIPLIER_1T,vb-SID_SHIFT_1T)} }
extern "C" { pub fn bug() -> !; pub fn mmu_has_feature(f:usize)->bool; pub fn segment_shift(s:i32)->u32; }
/* Remaining source declarations are intentionally represented as external ABI dependencies. */
extern "C" { pub fn htab_shift_for_mem_size(size:usize)->u32; pub fn slb_initialize(); pub fn slb_flush_and_restore_bolted(); pub fn slb_flush_all_realmode(); pub fn slb_vmalloc_update(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
