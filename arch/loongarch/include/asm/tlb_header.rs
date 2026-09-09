/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/mm_types.h, asm/cpu-features.h, asm/loongarch.h,
// and asm-generic/tlb.h.

/*
 * TLB Invalidate Flush
 */
#[inline]
pub unsafe fn tlbclr() {
    core::arch::asm!("tlbclr");
}

#[inline]
pub unsafe fn tlbflush() {
    core::arch::asm!("tlbflush");
}

/*
 * TLB R/W operations.
 */
#[inline]
pub unsafe fn tlb_probe() {
    core::arch::asm!("tlbsrch");
}

#[inline]
pub unsafe fn tlb_read() {
    core::arch::asm!("tlbrd");
}

#[inline]
pub unsafe fn tlb_write_indexed() {
    core::arch::asm!("tlbwr");
}

#[inline]
pub unsafe fn tlb_write_random() {
    core::arch::asm!("tlbfill");
}

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum invtlb_ops {
    /* Invalid all tlb */
    INVTLB_ALL = 0x0,
    /* Invalid current tlb */
    INVTLB_CURRENT_ALL = 0x1,
    /* Invalid all global=1 lines in current tlb */
    INVTLB_CURRENT_GTRUE = 0x2,
    /* Invalid all global=0 lines in current tlb */
    INVTLB_CURRENT_GFALSE = 0x3,
    /* Invalid global=0 and matched asid lines in current tlb */
    INVTLB_GFALSE_AND_ASID = 0x4,
    /* Invalid addr with global=0 and matched asid in current tlb */
    INVTLB_ADDR_GFALSE_AND_ASID = 0x5,
    /* Invalid addr with global=1 or matched asid in current tlb */
    INVTLB_ADDR_GTRUE_OR_ASID = 0x6,
    /* Invalid matched gid in guest tlb */
    INVGTLB_GID = 0x9,
    /* Invalid global=1, matched gid in guest tlb */
    INVGTLB_GID_GTRUE = 0xa,
    /* Invalid global=0, matched gid in guest tlb */
    INVGTLB_GID_GFALSE = 0xb,
    /* Invalid global=0, matched gid and asid in guest tlb */
    INVGTLB_GID_GFALSE_ASID = 0xc,
    /* Invalid global=0 , matched gid, asid and addr in guest tlb */
    INVGTLB_GID_GFALSE_ASID_ADDR = 0xd,
    /* Invalid global=1 , matched gid, asid and addr in guest tlb */
    INVGTLB_GID_GTRUE_ASID_ADDR = 0xe,
    /* Invalid all gid gva-->gpa guest tlb */
    INVGTLB_ALLGID_GVA_TO_GPA = 0x10,
    /* Invalid all gid gpa-->hpa tlb */
    INVTLB_ALLGID_GPA_TO_HPA = 0x11,
    /* Invalid all gid tlb, including  gva-->gpa and gpa-->hpa */
    INVTLB_ALLGID = 0x12,
    /* Invalid matched gid gva-->gpa guest tlb */
    INVGTLB_GID_GVA_TO_GPA = 0x13,
    /* Invalid matched gid gpa-->hpa tlb */
    INVTLB_GID_GPA_TO_HPA = 0x14,
    /* Invalid matched gid tlb,including gva-->gpa and gpa-->hpa */
    INVGTLB_GID_ALL = 0x15,
    /* Invalid matched gid and addr gpa-->hpa tlb */
    INVTLB_GID_ADDR = 0x16,
}

#[inline(always)]
pub unsafe fn invtlb(op: u32, info: u32, addr: u64) {
    core::arch::asm!(
        "invtlb {0}, {1}, {2}",
        const op,
        in(reg) info,
        in(reg) addr,
        options(preserves_flags)
    );
}

#[inline(always)]
pub unsafe fn invtlb_addr(op: u32, info: u32, addr: u64) {
    // BUILD_BUG_ON(!__builtin_constant_p(info) || info != 0);
    core::arch::asm!(
        "invtlb {0}, $zero, {1}",
        const op,
        in(reg) addr,
        options(preserves_flags)
    );
}

#[inline(always)]
pub unsafe fn invtlb_info(op: u32, info: u32, addr: u64) {
    // BUILD_BUG_ON(!__builtin_constant_p(addr) || addr != 0);
    core::arch::asm!(
        "invtlb {0}, {1}, $zero",
        const op,
        in(reg) info,
        options(preserves_flags)
    );
}

#[inline(always)]
pub unsafe fn invtlb_all(op: u32, info: u32, addr: u64) {
    // BUILD_BUG_ON(!__builtin_constant_p(info) || info != 0);
    // BUILD_BUG_ON(!__builtin_constant_p(addr) || addr != 0);
    core::arch::asm!(
        "invtlb {0}, $zero, $zero",
        const op,
        options(preserves_flags)
    );
}

// Forward declaration: static void tlb_flush(struct mmu_gather *tlb);
// #define tlb_flush tlb_flush
// The declaration from asm-generic/tlb.h is supplied by the surrounding translation.

#[inline]
pub unsafe fn tlb_flush(tlb: *mut mmu_gather) {
    let mut vma: vm_area_struct = core::mem::zeroed();

    vma.vm_mm = (*tlb).mm;
    vm_flags_init(&mut vma, 0);
    if (*tlb).fullmm {
        flush_tlb_mm((*tlb).mm);
        return;
    }

    flush_tlb_range(&mut vma, (*tlb).start, (*tlb).end);
}

extern "C" {
    pub fn handle_tlb_load();
    pub fn handle_tlb_store();
    pub fn handle_tlb_modify();
    pub fn handle_tlb_refill();
    pub fn handle_tlb_protect();
    pub fn handle_tlb_load_ptw();
    pub fn handle_tlb_store_ptw();
    pub fn handle_tlb_modify_ptw();

    pub fn dump_tlb_all();
    pub fn dump_tlb_regs();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
