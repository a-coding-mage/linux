/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding architecture-specific code:
// abi/reg_ops.h and asm/barrier.h.

pub unsafe fn read_mmu_index() -> i32 {
    mfcr("cr<0, 15>")
}

pub unsafe fn write_mmu_index(value: i32) {
    mtcr("cr<0, 15>", value);
}

pub unsafe fn read_mmu_entrylo0() -> i32 {
    mfcr("cr<2, 15>")
}

pub unsafe fn read_mmu_entrylo1() -> i32 {
    mfcr("cr<3, 15>")
}

pub unsafe fn write_mmu_pagemask(value: i32) {
    mtcr("cr<6, 15>", value);
}

pub unsafe fn read_mmu_entryhi() -> i32 {
    mfcr("cr<4, 15>")
}

pub unsafe fn write_mmu_entryhi(value: i32) {
    mtcr("cr<4, 15>", value);
}

pub unsafe fn read_mmu_msa0() -> usize {
    mfcr("cr<30, 15>") as usize
}

pub unsafe fn write_mmu_msa0(value: usize) {
    mtcr("cr<30, 15>", value as i32);
}

pub unsafe fn read_mmu_msa1() -> usize {
    mfcr("cr<31, 15>") as usize
}

pub unsafe fn write_mmu_msa1(value: usize) {
    mtcr("cr<31, 15>", value as i32);
}

/*
 * TLB operations.
 */
pub unsafe fn tlb_probe() {
    mtcr("cr<8, 15>", 0x80000000u32 as i32);
}

pub unsafe fn tlb_read() {
    mtcr("cr<8, 15>", 0x40000000u32 as i32);
}

pub unsafe fn tlb_invalid_all() {
    // CONFIG_CPU_HAS_TLBI is a build-time condition from the original header.
    #[cfg(CONFIG_CPU_HAS_TLBI)]
    {
        sync_is();
        core::arch::asm!("tlbi.alls\n\");
        core::arch::asm!("sync.i\n\");
    }
    #[cfg(not(CONFIG_CPU_HAS_TLBI))]
    {
        mtcr("cr<8, 15>", 0x04000000);
    }
}

pub unsafe fn local_tlb_invalid_all() {
    #[cfg(CONFIG_CPU_HAS_TLBI)]
    {
        sync_is();
        core::arch::asm!("tlbi.all\n\");
        core::arch::asm!("sync.i\n\");
    }
    #[cfg(not(CONFIG_CPU_HAS_TLBI))]
    {
        tlb_invalid_all();
    }
}

pub unsafe fn tlb_invalid_indexed() {
    mtcr("cr<8, 15>", 0x02000000);
}

pub const NOP32: &str = ".long 0x4820c400\n";

pub unsafe fn setup_pgd(pgd: *mut pgd_t, asid: i32) {
    #[cfg(CONFIG_CPU_HAS_TLBI)]
    {
        sync_is();
    }
    #[cfg(not(CONFIG_CPU_HAS_TLBI))]
    {
        mb();
    }

    // Original inline assembly programs the page-table registers and emits
    // 64 copies of NOP32; the architecture-specific assembler is preserved.
    #[cfg(CONFIG_CPU_HAS_TLBI)]
    core::arch::asm!(
        "mtcr {1}, cr<28, 15>",
        "mtcr {1}, cr<29, 15>",
        "mtcr {0}, cr< 4, 15>",
        ".rept 64",
        ".long 0x4820c400",
        ".endr",
        in(reg) asid,
        in(reg) (__pa(pgd) | BIT(0)),
        options(preserves_flags)
    );
    #[cfg(not(CONFIG_CPU_HAS_TLBI))]
    core::arch::asm!(
        "mtcr {1}, cr<29, 15>",
        "mtcr {0}, cr< 4, 15>",
        ".rept 64",
        ".long 0x4820c400",
        ".endr",
        in(reg) asid,
        in(reg) (__pa(pgd) | BIT(0)),
        options(preserves_flags)
    );
}

pub unsafe fn get_pgd() -> *mut pgd_t {
    __va((mfcr("cr<29, 15>") as usize) & !BIT(0))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
