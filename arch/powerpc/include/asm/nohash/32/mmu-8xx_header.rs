/* SPDX-License-Identifier: GPL-2.0 */
/* PPC8xx support */

pub const SPRN_MI_CTR: u32 = 784;
pub const MI_GPM: u32 = 0x80000000;
pub const MI_PPM: u32 = 0x40000000;
pub const MI_CIDEF: u32 = 0x20000000;
pub const MI_RSV4I: u32 = 0x08000000;
pub const MI_PPCS: u32 = 0x02000000;
pub const MI_IDXMASK: u32 = 0x00001f00;

pub const SPRN_MI_AP: u32 = 786;
pub const MI_Ks: u32 = 0x80000000;
pub const MI_Kp: u32 = 0x40000000;
pub const MI_APG_INIT: u32 = 0xde000000;

pub const SPRN_MI_EPN: u32 = 787;
pub const MI_EPNMASK: u32 = 0xfffff000;
pub const MI_EVALID: u32 = 0x00000200;
pub const MI_ASIDMASK: u32 = 0x0000000f;

pub const SPRN_MI_TWC: u32 = 789;
pub const MI_APG: u32 = 0x000001e0;
pub const MI_GUARDED: u32 = 0x00000010;
pub const MI_PSMASK: u32 = 0x0000000c;
pub const MI_PS8MEG: u32 = 0x0000000c;
pub const MI_PS512K: u32 = 0x00000004;
pub const MI_PS4K_16K: u32 = 0x00000000;
pub const MI_SVALID: u32 = 0x00000001;

pub const SPRN_MI_RPN: u32 = 790;
pub const MI_SPS16K: u32 = 0x00000008;
pub const MI_BOOTINIT: u32 = 0x000001fd;

pub const SPRN_MD_CTR: u32 = 792;
pub const MD_GPM: u32 = 0x80000000;
pub const MD_PPM: u32 = 0x40000000;
pub const MD_CIDEF: u32 = 0x20000000;
pub const MD_WTDEF: u32 = 0x10000000;
pub const MD_RSV4I: u32 = 0x08000000;
pub const MD_TWAM: u32 = 0x04000000;
pub const MD_PPCS: u32 = 0x02000000;
pub const MD_IDXMASK: u32 = 0x00001f00;

pub const SPRN_M_CASID: u32 = 793;
pub const MC_ASIDMASK: u32 = 0x0000000f;
pub const SPRN_MD_AP: u32 = 794;
pub const MD_Ks: u32 = 0x80000000;
pub const MD_Kp: u32 = 0x40000000;
pub const MD_APG_INIT: u32 = 0xdc000000;
pub const MD_APG_KUAP: u32 = 0xde000000;

pub const SPRN_MD_EPN: u32 = 795;
pub const MD_EPNMASK: u32 = 0xfffff000;
pub const MD_EVALID: u32 = 0x00000200;
pub const MD_ASIDMASK: u32 = 0x0000000f;
pub const SPRN_M_TWB: u32 = 796;
pub const M_L1TB: u32 = 0xfffff000;
pub const M_L1INDX: u32 = 0x00000ffc;
pub const SPRN_MD_TWC: u32 = 797;
pub const MD_L2TB: u32 = 0xfffff000;
pub const MD_L2INDX: u32 = 0xfffffe00;
pub const MD_APG: u32 = 0x000001e0;
pub const MD_GUARDED: u32 = 0x00000010;
pub const MD_PSMASK: u32 = 0x0000000c;
pub const MD_PS8MEG: u32 = 0x0000000c;
pub const MD_PS512K: u32 = 0x00000004;
pub const MD_PS4K_16K: u32 = 0x00000000;
pub const MD_WT: u32 = 0x00000002;
pub const MD_SVALID: u32 = 0x00000001;
pub const SPRN_MD_RPN: u32 = 798;
pub const MD_SPS16K: u32 = 0x00000008;
pub const SPRN_M_TW: u32 = 799;

/* Build-time page-size selection from CONFIG_PPC_4K_PAGES / CONFIG_PPC_16K_PAGES. */
#[cfg(CONFIG_PPC_4K_PAGES)]
pub const mmu_virtual_psize: u32 = MMU_PAGE_4K;
#[cfg(CONFIG_PPC_16K_PAGES)]
pub const mmu_virtual_psize: u32 = MMU_PAGE_16K;
#[cfg(CONFIG_PPC_16K_PAGES)]
pub const PTE_FRAG_NR: u32 = 4;
#[cfg(CONFIG_PPC_16K_PAGES)]
pub const PTE_FRAG_SIZE_SHIFT: u32 = 12;
#[cfg(CONFIG_PPC_16K_PAGES)]
pub const PTE_FRAG_SIZE: usize = 1usize << 12;
pub const mmu_linear_psize: u32 = MMU_PAGE_8M;

pub unsafe extern "C" {
    pub fn mmu_pin_tlb(top: libc::c_ulong, readonly: bool);
}

#[repr(C)]
pub struct mm_context_t {
    pub id: libc::c_uint,
    pub active: libc::c_uint,
    pub vdso: *mut libc::c_void,
    pub pte_frag: *mut libc::c_void,
}

#[repr(C)]
pub struct mmu_psize_def {
    pub shift: libc::c_uint,
}

pub unsafe extern "C" {
    pub static mut mmu_psize_defs: [mmu_psize_def; MMU_PAGE_COUNT as usize];
    pub static mut patch__itlbmiss_exit_1: i32;
    pub static mut patch__dtlbmiss_exit_1: i32;
    pub static mut patch__itlbmiss_perf: i32;
    pub static mut patch__dtlbmiss_perf: i32;
}

pub unsafe fn shift_to_mmu_psize(shift: libc::c_uint) -> i32 {
    let mut psize: i32 = 0;
    while psize < MMU_PAGE_COUNT as i32 {
        if mmu_psize_defs[psize as usize].shift == shift {
            return psize;
        }
        psize += 1;
    }
    -1
}

pub unsafe fn mmu_psize_to_shift(mmu_psize: libc::c_uint) -> libc::c_uint {
    if mmu_psize_defs[mmu_psize as usize].shift != 0 {
        return mmu_psize_defs[mmu_psize as usize].shift;
    }
    BUG();
}

pub unsafe fn arch_vmap_try_size(
    addr: libc::c_ulong, end: libc::c_ulong, pfn: u64,
    max_page_shift: libc::c_uint, size: libc::c_ulong,
) -> bool {
    if end.wrapping_sub(addr) < size { return false; }
    if (1 as libc::c_ulong).wrapping_shl(max_page_shift) < size { return false; }
    if !IS_ALIGNED(addr, size) { return false; }
    if !IS_ALIGNED(PFN_PHYS(pfn), size) { return false; }
    true
}

pub unsafe fn arch_vmap_pte_range_map_size(
    addr: libc::c_ulong, end: libc::c_ulong, pfn: u64,
    max_page_shift: libc::c_uint,
) -> libc::c_ulong {
    if arch_vmap_try_size(addr, end, pfn, max_page_shift, SZ_512K) { return SZ_512K; }
    if PAGE_SIZE == SZ_16K { return SZ_16K; }
    if arch_vmap_try_size(addr, end, pfn, max_page_shift, SZ_16K) { return SZ_16K; }
    PAGE_SIZE
}

pub unsafe fn arch_vmap_pte_supported_shift(size: libc::c_ulong) -> libc::c_uint {
    if size >= SZ_512K { 19 } else if size >= SZ_16K { 14 } else { PAGE_SHIFT }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
