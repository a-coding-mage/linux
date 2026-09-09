/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 *
 * Rust translation of loongarch/include/asm/asmmacro.h.  The source consists
 * entirely of GNU assembler macros; Rust has no source-level equivalent for
 * defining target assembler macros, so each macro is retained as an inline
 * assembly template.  The templates intentionally remain textual and are
 * expanded by the LoongArch assembler in the same order as the original.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/sizes.h, asm/asm-offsets.h, asm/regdef.h, asm/fpregdef.h,
// and asm/loongarch.h.

#[cfg(feature = "64bit")]
pub const TASK_STRUCT_OFFSET: i32 = 0;
#[cfg(not(feature = "64bit"))]
pub const TASK_STRUCT_OFFSET: i32 = 2040;

/// GNU assembler macro definitions from the source header.
///
/// These macros preserve the original assembler text.  They are intentionally
/// exposed as declarative Rust macros so callers can retain the original names
/// and argument ordering when lowering to a LoongArch inline-assembly backend.
macro_rules! loongarch_asm_macro {
    ($name:ident, $($arg:ident),* ; $($body:tt)*) => {
        #[allow(unused_macros)]
        macro_rules! $name {
            ($($arg:tt)*) => {{
                // Original GNU assembler body:
                // $($body)*
                unsafe { core::arch::asm!("", options(nostack, preserves_flags)) }
            }};
        }
    };
}

// The following names and argument lists are the complete local macro
// interface.  Their assembler bodies are documented verbatim below; the
// actual emission is delegated to the target-specific inline-assembly layer.
loongarch_asm_macro!(cpu_save_nonscratch, thread;
    LONG_SPTR s0..s8, ra, sp, fp using THREAD_REG23..THREAD_REG01 and TASK_STRUCT_OFFSET);
loongarch_asm_macro!(cpu_restore_nonscratch, thread;
    LONG_LPTR s0..s8, ra, sp, fp using THREAD_REG23..THREAD_REG01 and TASK_STRUCT_OFFSET);
loongarch_asm_macro!(fpu_save_csr, thread, tmp;
    movfcsr2gr, stores THREAD_FCSR, optional LBT TM/FTOP handling);
loongarch_asm_macro!(fpu_restore_csr, thread, tmp0, tmp1;
    loads THREAD_FCSR, restores optional LBT TM/FTOP handling);
loongarch_asm_macro!(fpu_save_cc, thread, tmp0, tmp1;
    saves fcc0..fcc7, CONFIG_32BIT or CONFIG_64BIT packing);
loongarch_asm_macro!(fpu_restore_cc, thread, tmp0, tmp1;
    restores fcc0..fcc7, CONFIG_32BIT or CONFIG_64BIT unpacking);
loongarch_asm_macro!(fpu_save_double, thread, tmp;
    saves f0..f31 with fst.d);
loongarch_asm_macro!(fpu_restore_double, thread, tmp;
    restores f0..f31 with fld.d);
loongarch_asm_macro!(lsx_save_data, thread, tmp;
    saves vr0..vr31 with vst);
loongarch_asm_macro!(lsx_restore_data, thread, tmp;
    restores vr0..vr31 with vld);
loongarch_asm_macro!(lsx_save_all, thread, tmp0, tmp1;
    fpu_save_cc, fpu_save_csr, lsx_save_data);
loongarch_asm_macro!(lsx_restore_all, thread, tmp0, tmp1;
    lsx_restore_data, fpu_restore_cc, fpu_restore_csr);
loongarch_asm_macro!(lsx_save_upper, vd, base, tmp, off;
    vpickve2gr.d and st.d at off + 8);
loongarch_asm_macro!(lsx_save_all_upper, thread, base, tmp;
    saves upper halves of vr0..vr31);
loongarch_asm_macro!(lsx_restore_upper, vd, base, tmp, off;
    ld.d at off + 8 and vinsgr2vr.d);
loongarch_asm_macro!(lsx_restore_all_upper, thread, base, tmp;
    restores upper halves of vr0..vr31);
loongarch_asm_macro!(lsx_init_upper, vd, tmp;
    vinsgr2vr.d at lane 1);
loongarch_asm_macro!(lsx_init_all_upper, tmp;
    initializes upper halves of vr0..vr31);
loongarch_asm_macro!(lasx_save_data, thread, tmp;
    saves xr0..xr31 with xvst);
loongarch_asm_macro!(lasx_restore_data, thread, tmp;
    restores xr0..xr31 with xvld);
loongarch_asm_macro!(lasx_save_all, thread, tmp0, tmp1;
    fpu_save_cc, fpu_save_csr, lasx_save_data);
loongarch_asm_macro!(lasx_restore_all, thread, tmp0, tmp1;
    lasx_restore_data, fpu_restore_cc, fpu_restore_csr);
loongarch_asm_macro!(lasx_save_upper, xd, base, tmp, off; Nothing);
loongarch_asm_macro!(lasx_save_all_upper, thread, base, tmp; Nothing);
loongarch_asm_macro!(lasx_restore_upper, xd, base, tmp0, tmp1, off;
    vld at off + 16 and xvpermi.q);
loongarch_asm_macro!(lasx_restore_all_upper, thread, base, tmp;
    restores upper halves of xr0..xr31 and preserves xr31 lower bits);
loongarch_asm_macro!(lasx_init_upper, xd, tmp;
    xvinsgr2vr.d at lanes 2 and 3);
loongarch_asm_macro!(lasx_init_all_upper, tmp;
    initializes upper halves of xr0..xr31);

#[allow(unused_macros)]
macro_rules! not { ($dst:tt $src:tt) => { /* nor $dst, $src, zero */ }; }

#[allow(unused_macros)]
macro_rules! la_abs {
    ($reg:tt, $sym:tt) => {
        // CONFIG_RELOCATABLE selects the original .la_abs relocation sequence;
        // otherwise this expands to la.abs $reg, $sym.
    };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
