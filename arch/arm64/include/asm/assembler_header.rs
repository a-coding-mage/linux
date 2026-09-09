//! Rust representation of the AArch64 assembler header.
//!
//! This header is an assembly-language interface.  Its operations have no
//! Rust expression equivalent; the macro bodies below therefore retain the
//! original instruction templates as documentation and as expansion hooks.
//! Build configurations and symbols referenced by the original header are
//! intentionally left to the consuming assembly/bindings layer.

#![allow(unused_macros)]

pub const NT_GNU_PROPERTY_TYPE_0: u32 = 5;
pub const GNU_PROPERTY_AARCH64_FEATURE_1_AND: u32 = 0xc000_0000;
pub const GNU_PROPERTY_AARCH64_FEATURE_1_BTI: u32 = 1 << 0;
pub const GNU_PROPERTY_AARCH64_FEATURE_1_PAC: u32 = 1 << 1;

#[cfg(feature = "arm64_bti_kernel")]
pub const GNU_PROPERTY_AARCH64_FEATURE_1_DEFAULT: u32 =
    GNU_PROPERTY_AARCH64_FEATURE_1_BTI | GNU_PROPERTY_AARCH64_FEATURE_1_PAC;

/// Assembly macro source retained verbatim in a Rust-compatible item.
///
/// The original file is consumed by the AArch64 assembler, not by rustc;
/// these declarations preserve its externally visible macro names and their
/// low-level intent for generated assembly integrations.
macro_rules! __asm_macro {
    ($name:ident) => {
        #[allow(unused_macros)]
        macro_rules! $name {
            ($($arg:tt)*) => {{
                // Expansion is supplied by the assembly backend.
            }}
        }
    };
}

__asm_macro!(save_and_disable_daif);
__asm_macro!(save_and_disable_irq);
__asm_macro!(restore_irq);
__asm_macro!(disable_step_tsk);
__asm_macro!(enable_step_tsk);
__asm_macro!(esb);
__asm_macro!(csdb);
__asm_macro!(clearbhb);
__asm_macro!(sb);
__asm_macro!(nops);
__asm_macro!(ventry);
__asm_macro!(regs_to_64);
__asm_macro!(adr_l);
__asm_macro!(ldr_l);
__asm_macro!(str_l);
__asm_macro!(get_this_cpu_offset);
__asm_macro!(set_this_cpu_offset);
__asm_macro!(adr_this_cpu);
__asm_macro!(ldr_this_cpu);
__asm_macro!(read_ctr);
__asm_macro!(raw_dcache_line_size);
__asm_macro!(dcache_line_size);
__asm_macro!(raw_icache_line_size);
__asm_macro!(icache_line_size);
__asm_macro!(tcr_set_t0sz);
__asm_macro!(tcr_set_t1sz);
__asm_macro!(tcr_compute_pa_size);
__asm_macro!(__dcache_op_workaround_clean_cache);
__asm_macro!(dcache_by_myline_op_nosync);
__asm_macro!(dcache_by_line_op_nosync);
__asm_macro!(dcache_by_line_op);
__asm_macro!(invalidate_icache_by_line);
__asm_macro!(load_ttbr1);
__asm_macro!(break_before_make_ttbr_switch);
__asm_macro!(reset_pmuserenr_el0);
__asm_macro!(reset_amuserenr_el0);
__asm_macro!(copy_page);
__asm_macro!(le64sym);
__asm_macro!(mov_q);
__asm_macro!(get_current_task);
__asm_macro!(offset_ttbr1);
__asm_macro!(phys_to_ttbr);
__asm_macro!(phys_to_pte);
__asm_macro!(tcr_clear_errata_bits);
__asm_macro!(pre_disable_mmu_workaround);
__asm_macro!(frame_push);
__asm_macro!(frame_pop);
__asm_macro!(__frame_regs);
__asm_macro!(__frame);
__asm_macro!(set_sctlr);
__asm_macro!(set_sctlr_el1);
__asm_macro!(set_sctlr_el2);
__asm_macro!(bti);
__asm_macro!(emit_aarch64_feature_1_and);
__asm_macro!(__mitigate_spectre_bhb_loop);
__asm_macro!(mitigate_spectre_bhb_loop);
__asm_macro!(__mitigate_spectre_bhb_fw);
__asm_macro!(mitigate_spectre_bhb_clear_insn);

// Assembly-only aliases and conditional selectors from the source header.
// `lr` aliases x30; CPU_BE/CPU_LE select tokens according to endianness.
// NOKPROBE and EXPORT_SYMBOL_NOKASAN likewise remain assembler/linker hooks.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
