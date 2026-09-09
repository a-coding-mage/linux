//! Rust representation of `asm/asmmacro.h`.
//!
//! This header defines MIPS assembler macros rather than C/Rust runtime
//! items.  The macros below retain the original names and parameter lists;
//! their assembler bodies are documented in the corresponding source-level
//! comments because Rust's inline-assembly operand syntax cannot represent
//! the assembler preprocessor's register and build-configuration expansion.

/* Dependencies supplied by the surrounding MIPS target:
 * asm/hazards.h, asm/asm-offsets.h, asm/msa.h,
 * asm/asmmacro-32.h, and asm/asmmacro-64.h.
 * CONFIG_* and TOOLCHAIN_SUPPORTS_MSA are build-time conditions.
 */

/// Preserve an assembler macro as a Rust macro while retaining its call
/// interface and expansion tokens for a target-specific assembler backend.
#[macro_export]
macro_rules! mips_asm_macro {
    ($name:ident ( $($arg:ident $(= $default:tt)?,)* ) { $($body:tt)* }) => {
        #[allow(unused_macros)]
        macro_rules! $name {
            ($($arg $(= $default)?,)*) => {
                let _ = stringify!($($body)*);
            };
        }
    };
}

// Helper encodings: insn32_if_mm emits two halfwords for microMIPS;
// insn_if_mips emits a word for regular MIPS.  The selected form is a
// CONFIG_CPU_MICROMIPS build-time choice.
mips_asm_macro!(insn32_if_mm(enc) { .insn .hword ((enc) >> 16) .hword ((enc) & 0xffff) });
mips_asm_macro!(insn_if_mips(enc) { .insn .word (enc) });

// Interrupt control, including irq hazards and the CONFIG_PREEMPTION counter
// updates, is supplied by the target assembler implementation.
mips_asm_macro!(local_irq_enable(reg = t0) { ei irq_enable_hazard });
mips_asm_macro!(local_irq_disable(reg = t0) { di irq_disable_hazard });

mips_asm_macro!(fpu_save_16even(thread, tmp = t0) { cfc1 sdc1 sw });
mips_asm_macro!(fpu_save_16odd(thread) { sdc1 });
mips_asm_macro!(fpu_save_double(thread, status, tmp) { sll bgez fpu_save_16odd fpu_save_16even });
mips_asm_macro!(fpu_restore_16even(thread, tmp = t0) { lw ldc1 ctc1 });
mips_asm_macro!(fpu_restore_16odd(thread) { ldc1 });
mips_asm_macro!(fpu_restore_double(thread, status, tmp) { sll bgez fpu_restore_16odd fpu_restore_16even });

mips_asm_macro!(_EXT(rd, rs, p, s) { ext srl andi });

// MT ASE encodings (the original macros use insn_if_mips/insn32_if_mm).
mips_asm_macro!(DMT(reg = 0) { 0x41600bc1 | (reg << 16) 0x0000057C | (reg << 21) });
mips_asm_macro!(EMT(reg = 0) { 0x41600be1 | (reg << 16) 0x0000257C | (reg << 21) });
mips_asm_macro!(DVPE(reg = 0) { 0x41600001 | (reg << 16) 0x0000157C | (reg << 21) });
mips_asm_macro!(EVPE(reg = 0) { 0x41600021 | (reg << 16) 0x0000357C | (reg << 21) });
mips_asm_macro!(MFTR(rs = 0, rt = 0, u = 0, sel = 0) { 0x41000000 });
mips_asm_macro!(MTTR(rt = 0, rs = 0, u = 0, sel = 0) { 0x41800000 });

// MSA instructions.  TOOLCHAIN_SUPPORTS_MSA selects native MSA mnemonics;
// otherwise the original file emits the listed raw instruction encodings.
mips_asm_macro!(_cfcmsa(rd, cs) { cfcmsa });
mips_asm_macro!(_ctcmsa(cd, rs) { ctcmsa });
mips_asm_macro!(ld_b(wd, off, base) { ld.b });
mips_asm_macro!(ld_h(wd, off, base) { ld.h });
mips_asm_macro!(ld_w(wd, off, base) { ld.w });
mips_asm_macro!(ld_d(wd, off, base) { ld.d });
mips_asm_macro!(st_b(wd, off, base) { st.b });
mips_asm_macro!(st_h(wd, off, base) { st.h });
mips_asm_macro!(st_w(wd, off, base) { st.w });
mips_asm_macro!(st_d(wd, off, base) { st.d });
mips_asm_macro!(copy_s_w(ws, n) { copy_s.w });
mips_asm_macro!(copy_s_d(ws, n) { copy_s.d });
mips_asm_macro!(insert_w(wd, n) { insert.w });
mips_asm_macro!(insert_d(wd, n) { insert.d });

mips_asm_macro!(msa_save_all(thread) { st_d _cfcmsa sw });
mips_asm_macro!(msa_restore_all(thread) { lw _ctcmsa ld_d });
mips_asm_macro!(msa_init_upper(wd) { insert_d insert_w });
mips_asm_macro!(msa_init_all_upper() { not msa_init_upper });


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
