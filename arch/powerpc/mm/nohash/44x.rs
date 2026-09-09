// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Modifications by Matt Porter (mporter@mvista.com) to support
 * PPC44x Book E processors.
 *
 * This file contains the routines for initializing the MMU
 * on the 4xx series of chips.
 *  -- paulus
 *
 * Derived from arch/ppc/mm/init.c:
 *   Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
 *
 * Modifications by Paul Mackerras (PowerMac) (paulus@cs.anu.edu.au)
 * and Cort Dougan (PReP) (cort@cs.nmt.edu)
 *   Copyright (C) 1996 Paul Mackerras
 *
 * Derived from "arch/i386/mm/init.c"
 *   Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
 */

// Dependencies supplied by other translation units.

/* Used by the 44x TLB replacement exception handler. */
pub static mut tlb_44x_index: u32 = 0;
pub static mut tlb_44x_hwater: u32 = PPC44x_TLB_SIZE - 1 - PPC44x_EARLY_TLBS;
pub static mut icache_44x_need_flush: i32 = 0;

pub static mut tlb_47x_boltmap: [c_ulong; 1024 / 8] = [0; 1024 / 8];

unsafe fn ppc44x_update_tlb_hwater() {
    /* Patch the watermark in both TLB miss handlers. */
    modify_instruction_site(&patch__tlb_44x_hwater_D, 0xffff, tlb_44x_hwater);
    modify_instruction_site(&patch__tlb_44x_hwater_I, 0xffff, tlb_44x_hwater);
}

/* "Pins" a 256MB TLB entry in AS0 for kernel lowmem for 44x type MMU. */
unsafe fn ppc44x_pin_tlb(virt: u32, phys: u32) {
    let entry = tlb_44x_hwater;
    tlb_44x_hwater = tlb_44x_hwater.wrapping_sub(1);

    ppc44x_update_tlb_hwater();

    mtspr(SPRN_MMUCR, 0);

    // The C inline assembly writes TLB page-id, translation, and attributes.
    asm!(
        "tlbwe {attr}, {entry}, {attrib}",
        "tlbwe {phys}, {entry}, {xlat}",
        "tlbwe {pageid}, {entry}, {pageid_sel}",
        attr = in(reg) (PPC44x_TLB_SW | PPC44x_TLB_SR | PPC44x_TLB_SX | PPC44x_TLB_G),
        phys = in(reg) phys,
        pageid = in(reg) (virt | PPC44x_TLB_VALID | PPC44x_TLB_256M),
        entry = in(reg) entry,
        attrib = const PPC44x_TLB_ATTRIB,
        xlat = const PPC44x_TLB_XLAT,
        pageid_sel = const PPC44x_TLB_PAGEID,
    );
}

unsafe fn ppc47x_find_free_bolted() -> i32 {
    let mmube0 = mfspr(SPRN_MMUBE0);
    let mmube1 = mfspr(SPRN_MMUBE1);

    if mmube0 & MMUBE0_VBE0 == 0 { return 0; }
    if mmube0 & MMUBE0_VBE1 == 0 { return 1; }
    if mmube0 & MMUBE0_VBE2 == 0 { return 2; }
    if mmube1 & MMUBE1_VBE3 == 0 { return 3; }
    if mmube1 & MMUBE1_VBE4 == 0 { return 4; }
    if mmube1 & MMUBE1_VBE5 == 0 { return 5; }
    -1
}

unsafe fn ppc47x_update_boltmap() {
    let mmube0 = mfspr(SPRN_MMUBE0);
    let mmube1 = mfspr(SPRN_MMUBE1);

    if mmube0 & MMUBE0_VBE0 != 0 { __set_bit((mmube0 >> MMUBE0_IBE0_SHIFT) & 0xff, tlb_47x_boltmap.as_mut_ptr()); }
    if mmube0 & MMUBE0_VBE1 != 0 { __set_bit((mmube0 >> MMUBE0_IBE1_SHIFT) & 0xff, tlb_47x_boltmap.as_mut_ptr()); }
    if mmube0 & MMUBE0_VBE2 != 0 { __set_bit((mmube0 >> MMUBE0_IBE2_SHIFT) & 0xff, tlb_47x_boltmap.as_mut_ptr()); }
    if mmube1 & MMUBE1_VBE3 != 0 { __set_bit((mmube1 >> MMUBE1_IBE3_SHIFT) & 0xff, tlb_47x_boltmap.as_mut_ptr()); }
    if mmube1 & MMUBE1_VBE4 != 0 { __set_bit((mmube1 >> MMUBE1_IBE4_SHIFT) & 0xff, tlb_47x_boltmap.as_mut_ptr()); }
    if mmube1 & MMUBE1_VBE5 != 0 { __set_bit((mmube1 >> MMUBE1_IBE5_SHIFT) & 0xff, tlb_47x_boltmap.as_mut_ptr()); }
}

/* "Pins" a 256MB TLB entry in AS0 for kernel lowmem for 47x type MMU. */
unsafe fn ppc47x_pin_tlb(virt: u32, phys: u32) {
    let mut r_a: u32 = 0x88000000;
    let bolted = ppc47x_find_free_bolted();
    BUG_ON(bolted < 0);
    r_a |= (bolted as u32) << 24;

    pr_debug!("256M TLB entry for 0x{:08x}->0x{:08x} in bolt slot {}\n", virt, phys, bolted);
    mtspr(SPRN_MMUCR, 0);

    asm!(
        "tlbwe {pageid}, {ra}, 0",
        "tlbwe {phys}, {ra}, 1",
        "tlbwe {attr}, {ra}, 2",
        pageid = in(reg) (virt | PPC47x_TLB0_VALID | PPC47x_TLB0_256M),
        phys = in(reg) phys,
        attr = in(reg) (PPC47x_TLB2_SW | PPC47x_TLB2_SR | PPC47x_TLB2_SX),
        ra = in(reg) r_a,
    );
}

pub unsafe fn MMU_init_hw() {
    ppc44x_update_tlb_hwater();
    flush_instruction_cache();
}

pub unsafe fn mmu_mapin_ram(_base: c_ulong, _top: c_ulong) -> c_ulong {
    let mut addr;
    let memstart = memstart_addr & !(PPC_PIN_SIZE - 1);
    addr = memstart + PPC_PIN_SIZE;
    while addr < lowmem_end_addr {
        if mmu_has_feature(MMU_FTR_TYPE_47x) { ppc47x_pin_tlb(addr + PAGE_OFFSET, addr); }
        else { ppc44x_pin_tlb(addr + PAGE_OFFSET, addr); }
        addr += PPC_PIN_SIZE;
    }
    if mmu_has_feature(MMU_FTR_TYPE_47x) { ppc47x_update_boltmap(); }
    total_lowmem
}

pub unsafe fn setup_initial_memory_limit(first_memblock_base: phys_addr_t, first_memblock_size: phys_addr_t) {
    // CONFIG_NONSTATIC_KERNEL controls the original BUG_ON condition.
    let size = core::cmp::min(first_memblock_size as u64, PPC_PIN_SIZE as u64);
    memblock_set_current_limit(first_memblock_base + size as phys_addr_t);
}

// CONFIG_SMP
pub unsafe fn mmu_init_secondary(_cpu: i32) {
    let mut addr;
    let memstart = memstart_addr & !(PPC_PIN_SIZE - 1);
    addr = memstart + PPC_PIN_SIZE;
    while addr < lowmem_end_addr {
        if mmu_has_feature(MMU_FTR_TYPE_47x) { ppc47x_pin_tlb(addr + PAGE_OFFSET, addr); }
        else { ppc44x_pin_tlb(addr + PAGE_OFFSET, addr); }
        addr += PPC_PIN_SIZE;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
