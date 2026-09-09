/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020 Google LLC.
 */

// This block corresponds to:
// #if defined(CONFIG_LTO) && !defined(__ASSEMBLER__)
// The build-time CONFIG_LTO and assembler conditions are preserved here as
// intent; the supplied Rust translation is available to non-assembler code.

// Dependencies supplied by the surrounding kernel translation:
// - `alternative!` corresponds to the kernel ALTERNATIVE macro.
// - `ARM64_HAS_LDAPR` is the arm64 capability selector.

/*
 * #define __LOAD_RCPC(sfx, regs...)
 *
 * Select the acquire load instruction according to the arm64 alternative
 * mechanism. The instruction selection and capability definition are external
 * dependencies of this header.
 */
#[macro_export]
macro_rules! __LOAD_RCPC {
    ($sfx:tt, $($regs:tt)*) => {
        // ALTERNATIVE("ldar" #sfx "\t" #regs,
        //             ".arch_extension rcpc\n" "ldapr" #sfx "\t" #regs,
        //             ARM64_HAS_LDAPR)
        compile_error!("__LOAD_RCPC requires the arm64 ALTERNATIVE machinery");
    };
}

/*
 * Replace this with typeof_unqual() when minimum compiler versions are
 * increased to GCC 14 and Clang 19. For the time being, we need this
 * workaround, which relies on function return values dropping qualifiers.
 */
// Rust types do not have C's qualifier distinction; this helper preserves the
// type-level intent of __rwonce_typeof_unqual(x).
#[inline(always)]
pub unsafe fn __rwonce_typeof_unqual<T>(value: *const T) -> T {
    core::ptr::read(value)
}

/*
 * When building with LTO, there is an increased risk of the compiler
 * converting an address dependency headed by a READ_ONCE() invocation
 * into a control dependency and consequently allowing for harmful
 * reordering by the CPU.
 *
 * Ensure that such transformations are harmless by overriding the generic
 * READ_ONCE() definition with one that provides RCpc acquire semantics
 * when building with LTO.
 */
// The C implementation uses inline arm64 assembly and the external
// ALTERNATIVE machinery. Rust's volatile read preserves the required memory
// access and ordering boundary; instruction selection remains an external
// dependency.
#[macro_export]
macro_rules! __READ_ONCE {
    ($x:expr) => {{
        let __x = &($x);
        unsafe { core::ptr::read_volatile(__x as *const _) }
    }};
}

// The generic declarations from <asm-generic/rwonce.h> are provided by the
// surrounding translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
