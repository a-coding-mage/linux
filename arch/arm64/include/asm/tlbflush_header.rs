/* SPDX-License-Identifier: GPL-2.0-only */
/* Translation of arm64/include/asm/tlbflush.h. */

/* C header dependencies are supplied by the surrounding kernel translation. */

pub const TLBI_TTL_TG_4K: u64 = 1;
pub const TLBI_TTL_TG_16K: u64 = 2;
pub const TLBI_TTL_TG_64K: u64 = 3;

pub const TLBI_TTL_MASK: u64 = 0x0000_f000_0000_0000;
pub const TLBI_TTL_UNKNOWN: u32 = i32::MAX as u32;

pub type TlbiOp = unsafe fn(u64);

#[inline(always)]
pub unsafe fn __tlbi<const OP: u64>(arg: Option<u64>) {
    /* C __tlbi emits the architecture-specific TLBI instruction. */
    let _ = (OP, arg);
}

#[inline(always)]
pub unsafe fn __tlbi_user<const OP: u64>(arg: u64) {
    if arm64_kernel_unmapped_at_el0() {
        __tlbi::<OP>(Some(arg | USER_ASID_FLAG as u64));
    }
}

#[inline(always)]
pub unsafe fn __tlbi_vaddr(addr: u64, asid: u16) -> u64 {
    let mut ta = addr >> 12;
    ta &= (1u64 << 44) - 1;
    ta |= (asid as u64) << 48;
    ta
}

#[inline]
pub fn get_trans_granule() -> u64 {
    match PAGE_SIZE {
        SZ_4K => TLBI_TTL_TG_4K,
        SZ_16K => TLBI_TTL_TG_16K,
        SZ_64K => TLBI_TTL_TG_64K,
        _ => 0,
    }
}

#[cfg(CONFIG_ARM64_ERRATUM_4193714)]
extern "C" {
    pub static mut sme_active_cpus: Cpumask;
    pub fn sme_do_dvmsync(mask: *const Cpumask);
}

#[inline]
pub unsafe fn sme_dvmsync(mm: *mut MmStruct) {
    if !alternative_has_cap_unlikely(ARM64_WORKAROUND_4193714) { return; }
    sme_do_dvmsync(mm_cpumask(mm));
}

#[cfg(not(CONFIG_ARM64_ERRATUM_4193714))]
#[inline] pub unsafe fn sme_dvmsync(_mm: *mut MmStruct) {}

#[cfg(CONFIG_ARM64_ERRATUM_4193714)]
#[inline]
pub unsafe fn sme_dvmsync_batch() {
    if !alternative_has_cap_unlikely(ARM64_WORKAROUND_4193714) { return; }
    sme_do_dvmsync(&sme_active_cpus);
}
#[cfg(not(CONFIG_ARM64_ERRATUM_4193714))]
#[inline] pub unsafe fn sme_dvmsync_batch() {}

#[inline(always)] pub unsafe fn vae1is(arg: u64) { __tlbi::<1>(Some(arg)); __tlbi_user::<1>(arg); }
#[inline(always)] pub unsafe fn vae2is(arg: u64) { __tlbi::<2>(Some(arg)); }
#[inline(always)] pub unsafe fn vale1(arg: u64) { __tlbi::<3>(Some(arg)); __tlbi_user::<3>(arg); }
#[inline(always)] pub unsafe fn vale1is(arg: u64) { __tlbi::<4>(Some(arg)); __tlbi_user::<4>(arg); }
#[inline(always)] pub unsafe fn vale2is(arg: u64) { __tlbi::<5>(Some(arg)); }
#[inline(always)] pub unsafe fn vaale1is(arg: u64) { __tlbi::<6>(Some(arg)); }
#[inline(always)] pub unsafe fn ipas2e1(arg: u64) { __tlbi::<7>(Some(arg)); }
#[inline(always)] pub unsafe fn ipas2e1is(arg: u64) { __tlbi::<8>(Some(arg)); }

#[inline(always)]
pub unsafe fn __tlbi_level_asid(op: TlbiOp, addr: u64, level: u32, asid: u16) {
    let mut arg = __tlbi_vaddr(addr, asid);
    if alternative_has_cap_unlikely(ARM64_HAS_ARMV8_4_TTL) && level <= 3 {
        let ttl = level as u64 | (get_trans_granule() << 2);
        arg = (arg & !TLBI_TTL_MASK) | ((ttl << 44) & TLBI_TTL_MASK);
    }
    op(arg);
}
#[inline] pub unsafe fn __tlbi_level(op: TlbiOp, addr: u64, level: u32) { __tlbi_level_asid(op, addr, level, 0); }

pub const TLBIR_ASID_MASK: u64 = 0xffff_0000_0000_0000;
pub const TLBIR_TG_MASK: u64 = 0x0000_c000_0000_0000;
pub const TLBIR_SCALE_MASK: u64 = 0x0000_3000_0000_0000;
pub const TLBIR_NUM_MASK: u64 = 0x0000_0f80_0000_0000;
pub const TLBIR_TTL_MASK: u64 = 0x0000_0060_0000_0000;
pub const TLBIR_BADDR_MASK: u64 = 0x0000_001f_ffff_ffff;

#[inline] pub const fn __tlbi_range_pages(num: usize, scale: usize) -> usize { (num + 1) << (5 * scale + 1) }
pub const MAX_TLBI_RANGE_PAGES: usize = __tlbi_range_pages(31, 3);
#[inline] pub const fn __tlbi_range_num(pages: usize, scale: usize) -> usize { (pages >> (5 * scale + 1)).wrapping_sub(1) }

#[inline] pub unsafe fn __tlbi_sync_s1ish(mm: *mut MmStruct) { dsb(ISH); __repeat_tlbi_sync(); sme_dvmsync(mm); }
#[inline] pub unsafe fn __tlbi_sync_s1ish_batch() { dsb(ISH); __repeat_tlbi_sync(); sme_dvmsync_batch(); }
#[inline] pub unsafe fn __tlbi_sync_s1ish_kernel() { dsb(ISH); __repeat_tlbi_sync(); }
#[inline] pub unsafe fn __tlbi_sync_s1ish_hyp() { dsb(ISH); __repeat_tlbi_sync(); }

#[inline(always)] pub unsafe fn rvae1is(arg: u64) { __tlbi::<9>(Some(arg)); __tlbi_user::<9>(arg); }
#[inline(always)] pub unsafe fn rvale1(arg: u64) { __tlbi::<10>(Some(arg)); __tlbi_user::<10>(arg); }
#[inline(always)] pub unsafe fn rvale1is(arg: u64) { __tlbi::<11>(Some(arg)); __tlbi_user::<11>(arg); }
#[inline(always)] pub unsafe fn rvaale1is(arg: u64) { __tlbi::<12>(Some(arg)); }
#[inline(always)] pub unsafe fn ripas2e1is(arg: u64) { __tlbi::<13>(Some(arg)); }

#[inline(always)]
pub unsafe fn __tlbi_range(op: TlbiOp, addr: u64, asid: u16, scale: i32, num: i32, level: u32, lpa2: bool) {
    let shift = if lpa2 { 16 } else { PAGE_SHIFT };
    let mut arg = ((addr >> shift) & TLBIR_BADDR_MASK)
        | (((if level > 3 { 0 } else { level }) as u64) << 37)
        | ((num as u64) << 39) | ((scale as u64) << 44)
        | (get_trans_granule() << 46) | ((asid as u64) << 48);
    arg &= TLBIR_ASID_MASK | TLBIR_TG_MASK | TLBIR_SCALE_MASK | TLBIR_NUM_MASK | TLBIR_TTL_MASK | TLBIR_BADDR_MASK;
    op(arg);
}

#[inline(always)]
pub unsafe fn __flush_tlb_range_op(lop: TlbiOp, rop: TlbiOp, start: u64, mut pages: usize, stride: u64, asid: u16, level: u32, lpa2: bool) {
    let mut addr = start;
    let end = start + pages as u64 * PAGE_SIZE as u64;
    let mut scale: i32 = 3;
    while addr != end {
        pages = ((end - addr) >> PAGE_SHIFT) as usize;
        if !system_supports_tlb_range() || pages == 1 || (lpa2 && !is_aligned(addr, SZ_64K)) {
            __tlbi_level_asid(lop, addr, level, asid); addr += stride; continue;
        }
        let num = __tlbi_range_num(pages, scale as usize) as i32;
        if num >= 0 { __tlbi_range(rop, addr, asid, scale, num, level, lpa2); addr += (__tlbi_range_pages(num as usize, scale as usize) << PAGE_SHIFT) as u64; }
        scale -= 1;
    }
}

pub const MAX_DVM_OPS: usize = PTRS_PER_PTE;
pub type TlbfT = u32;
pub const TLBF_NONE: TlbfT = 0;
pub const TLBF_NOWALKCACHE: TlbfT = 1 << 0;
pub const TLBF_NOSYNC: TlbfT = 1 << 1;
pub const TLBF_NONOTIFY: TlbfT = 1 << 2;
pub const TLBF_NOBROADCAST: TlbfT = 1 << 3;

/* Remaining public flush entry points retain the C ABI and depend on the kernel's external types/helpers. */
extern "C" {
    pub fn local_flush_tlb_all();
    pub fn flush_tlb_all();
    pub fn flush_tlb_mm(mm: *mut MmStruct);
    pub fn flush_tlb_range(vma: *mut VmAreaStruct, start: u64, end: u64);
    pub fn flush_tlb_page(vma: *mut VmAreaStruct, addr: u64);
    pub fn flush_tlb_kernel_range(start: u64, end: u64);
    pub fn __flush_tlb_kernel_pgtable(addr: u64);
    pub fn arch_tlbbatch_should_defer(mm: *mut MmStruct) -> bool;
    pub fn arch_tlbbatch_flush(batch: *mut ArchTlbflushUnmapBatch);
    pub fn arch_tlbbatch_add_pending(batch: *mut ArchTlbflushUnmapBatch, mm: *mut MmStruct, start: u64, end: u64);
    pub fn __flush_tlb_range(vma: *mut VmAreaStruct, start: u64, end: u64, stride: u64, tlb_level: i32, flags: TlbfT);
    pub fn __flush_tlb_page(vma: *mut VmAreaStruct, addr: u64, flags: TlbfT);
    pub fn pte_needs_flush(oldpte: Pte, newpte: Pte) -> bool;
    pub fn huge_pmd_needs_flush(oldpmd: Pmd, newpmd: Pmd) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
