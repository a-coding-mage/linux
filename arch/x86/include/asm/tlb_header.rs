/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the x86 TLB header. Included C dependencies provide the
// referenced types, constants, and functions.

pub const INVLPGB_FLAG_VA: u8 = 1u8 << 0;
pub const INVLPGB_FLAG_PCID: u8 = 1u8 << 1;
pub const INVLPGB_FLAG_ASID: u8 = 1u8 << 2;
pub const INVLPGB_FLAG_INCLUDE_GLOBAL: u8 = 1u8 << 3;
pub const INVLPGB_FLAG_FINAL_ONLY: u8 = 1u8 << 4;
pub const INVLPGB_FLAG_INCLUDE_NESTED: u8 = 1u8 << 5;
pub const INVLPGB_MODE_ALL_NONGLOBALS: usize = 0usize;

#[inline]
pub unsafe fn tlb_flush(tlb: *mut mmu_gather) {
    let mut start: usize = 0usize;
    let mut end: usize = TLB_FLUSH_ALL;
    let stride_shift: u32 = tlb_get_unmap_shift(tlb);

    if !(*tlb).fullmm && !(*tlb).need_flush_all {
        start = (*tlb).start;
        end = (*tlb).end;
    }

    flush_tlb_mm_range(
        (*tlb).mm,
        start,
        end,
        stride_shift,
        (*tlb).freed_tables,
    );
}

#[inline]
pub unsafe fn invlpg(addr: usize) {
    core::arch::asm!("invlpg [{}]", in(reg) addr, options(nostack, preserves_flags));
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum addr_stride {
    PTE_STRIDE = 0,
    PMD_STRIDE = 1,
}

// INVLPGB is targeted by virtual address, PCID, ASID, or any combination.

// CONFIG_BROADCAST_TLB_FLUSH controls whether the hardware instructions are
// emitted; the disabled implementations are retained below.
#[cfg(feature = "CONFIG_BROADCAST_TLB_FLUSH")]
#[inline]
pub unsafe fn __invlpgb(
    asid: usize,
    pcid: usize,
    addr: usize,
    nr_pages: u16,
    stride: addr_stride,
    flags: u8,
) {
    let rax: u64 = addr as u64 | flags as u64 | INVLPGB_FLAG_ASID as u64;
    let ecx: u32 = ((stride as u32) << 31) | (nr_pages as u32 - 1);
    let edx: u32 = ((pcid as u32) << 16) | asid as u32;

    VM_WARN_ON_ONCE(addr & !PAGE_MASK);
    core::arch::asm!(
        ".byte 0x0f, 0x01, 0xfe",
        in("rax") rax,
        in("ecx") ecx,
        in("edx") edx,
    );
}

#[cfg(not(feature = "CONFIG_BROADCAST_TLB_FLUSH"))]
#[inline]
pub unsafe fn __invlpgb(
    _asid: usize, _pcid: usize, _addr: usize, _nr_pages: u16,
    _stride: addr_stride, _flags: u8,
) {}

#[inline]
pub unsafe fn __invlpgb_all(asid: usize, pcid: usize, flags: u8) {
    __invlpgb(asid, pcid, 0, 1, addr_stride::PTE_STRIDE, flags);
}

#[cfg(feature = "CONFIG_BROADCAST_TLB_FLUSH")]
#[inline]
pub unsafe fn __tlbsync() {
    cant_migrate();
    core::arch::asm!(".byte 0x0f, 0x01, 0xff", options(nostack));
}

#[cfg(not(feature = "CONFIG_BROADCAST_TLB_FLUSH"))]
#[inline]
pub unsafe fn __tlbsync() {}

#[inline]
pub unsafe fn invlpgb_flush_user_nr_nosync(pcid: usize, addr: usize, nr: u16, stride: bool) {
    let str_ = if stride { addr_stride::PMD_STRIDE } else { addr_stride::PTE_STRIDE };
    let flags = INVLPGB_FLAG_PCID | INVLPGB_FLAG_VA;
    __invlpgb(0, pcid, addr, nr, str_, flags);
}

#[inline]
pub unsafe fn invlpgb_flush_single_pcid_nosync(pcid: usize) {
    __invlpgb_all(0, pcid, INVLPGB_FLAG_PCID);
}

#[inline]
pub unsafe fn invlpgb_flush_all() {
    guard_preempt();
    __invlpgb_all(0, 0, INVLPGB_FLAG_INCLUDE_GLOBAL);
    __tlbsync();
}

#[inline]
pub unsafe fn invlpgb_flush_addr_nosync(addr: usize, nr: u16) {
    __invlpgb(0, 0, addr, nr, addr_stride::PTE_STRIDE, INVLPGB_FLAG_INCLUDE_GLOBAL);
}

#[inline]
pub unsafe fn invlpgb_flush_all_nonglobals() {
    guard_preempt();
    __invlpgb_all(0, 0, INVLPGB_MODE_ALL_NONGLOBALS as u8);
    __tlbsync();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
