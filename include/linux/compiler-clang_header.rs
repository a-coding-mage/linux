/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of the Clang-specific Linux compiler definitions.
// The C header guard and direct-include diagnostic have no executable Rust
// equivalent.

/// All Clang versions usable with the kernel support KASAN ABI version 5.
pub const KASAN_ABI_VERSION: i32 = 5;

// __has_feature(address_sanitizer), __has_feature(hwaddress_sanitizer), and
// __has_feature(thread_sanitizer) conditionally define the corresponding C
// sanitizer macros.  HWAddressSanitizer is treated as AddressSanitizer.
// The sanitizer attributes below are compiler/build configuration directives;
// preserve their intent for Rust consumers with the equivalent cfg/attribute
// at the declaration site.

// CONFIG_ARCH_USE_BUILTIN_BSWAP conditionally provides these compiler macros.
// pub const __HAVE_BUILTIN_BSWAP32__: bool = true;
// pub const __HAVE_BUILTIN_BSWAP64__: bool = true;
// pub const __HAVE_BUILTIN_BSWAP16__: bool = true;

// __has_feature(undefined_behavior_sanitizer) conditionally defines
// __no_sanitize_undefined as no_sanitize("undefined").
// __has_feature(memory_sanitizer) conditionally defines __SANITIZE_MEMORY__,
// __no_sanitize_memory, and __no_kmsan_checks.  In the non-memory-sanitizer
// configuration those definitions expand to nothing.
// __has_feature(coverage_sanitizer) conditionally defines
// __no_sanitize_coverage as no_sanitize("coverage").

// Only Clang disables the coverage sanitizer for kstack_erase.
// __no_kstack_erase expands to __no_sanitize_coverage.

// __has_feature(shadow_call_stack) conditionally defines __noscs as the
// no_sanitize("shadow-call-stack") attribute.

// Diagnostic pragma helpers (__diag_clang, __diag_clang_ignore,
// __diag_clang_warn, __diag_clang_error, __diag_str1, __diag_str, __diag,
// __diag_clang_23, __diag_clang_all, and __diag_ignore_all) are preprocessor
// and pragma constructs with no direct Rust item equivalent.  For Clang
// version >= 230000, __diag_clang_23 emits the requested diagnostic pragma;
// otherwise it expands to nothing.

/*
 * Clang's asm constraints use these alternatives to avoid its handling of
 * multi-purpose "g" and "rm" constraints.  Keep the exact constraint text
 * available to low-level Rust/inline-assembly translations.
 */
pub const ASM_INPUT_G: &str = "ir";
pub const ASM_INPUT_RM: &str = "r";
pub const ASM_OUTPUT_RM: &str = "=r";

/*
 * Bindgen uses LLVM even when the C compiler is GCC, so the C configuration
 * cannot rely on auto-detection of CONFIG_CC_HAS_TYPEOF_UNQUAL.
 *
 * This is true when __clang_major__ > 19, or when the major version is 19 and
 * the minor version is greater than 0.  Compiler-version preprocessing is
 * preserved as a condition rather than guessed here.
 */
// pub const CC_HAS_TYPEOF_UNQUAL: bool =
//     __clang_major__ > 19 || (__clang_major__ == 19 && __clang_minor__ > 0);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
