// SPDX-License-Identifier: GPL-2.0-only
// DEFINE_PER_CPU_ALIGNED data, owned here. Rust has no variable-only alignment
// attribute: an aligned wrapper type would incorrectly increase ELF st_size.
// The data directive is the narrow compiler/ABI bridge; every offset/size and
// alignment is supplied by the canonical generated tlb_state and native macros.
macro_rules! define_cpu_tlbstate {
    ($section:literal) => {
        core::arch::global_asm!(
            concat!(".pushsection ", $section, ",\"aw\",@progbits"),
            ".balign {alignment}",
            ".globl cpu_tlbstate",
            ".type cpu_tlbstate,@object",
            "cpu_tlbstate:",
            ".zero {loaded_mm_offset}",
            ".quad {initial_mm}",
            ".zero {next_asid_offset} - ({loaded_mm_offset} + 8)",
            ".short 1",
            ".zero {cr4_offset} - ({next_asid_offset} + 2)",
            ".quad -1",
            ".zero {state_size} - ({cr4_offset} + 8)",
            ".size cpu_tlbstate, .-cpu_tlbstate",
            ".popsection",
            alignment = const b::RUST_MM_SMP_CACHE_BYTES,
            loaded_mm_offset = const core::mem::offset_of!(b::tlb_state, loaded_mm),
            next_asid_offset = const core::mem::offset_of!(b::tlb_state, next_asid),
            cr4_offset = const core::mem::offset_of!(b::tlb_state, cr4),
            state_size = const core::mem::size_of::<b::tlb_state>(),
            initial_mm = sym b::init_mm,
        );
    };
}
#[cfg(CONFIG_SMP)]
define_cpu_tlbstate!(".data..percpu..shared_aligned");
#[cfg(not(CONFIG_SMP))]
define_cpu_tlbstate!(".data..shared_aligned");

#[cfg(CONFIG_ADDRESS_MASKING)]
#[no_mangle]
#[cfg_attr(CONFIG_SMP, link_section = ".data..percpu")]
#[cfg_attr(not(CONFIG_SMP), link_section = ".data")]
pub static mut tlbstate_untag_mask: u64 = 0;

// RESERVE_BRK(early_pgt_alloc, INIT_PGT_BUF_SIZE): alignment is one byte, as in
// the original char array; extend_brk owns allocation and later reservation.
#[used]
#[link_section = ".bss..brk"]
static mut __brk_early_pgt_alloc: [u8; b::RUST_MM_INIT_PGT_BUF_SIZE as usize] =
    [0; b::RUST_MM_INIT_PGT_BUF_SIZE as usize];
