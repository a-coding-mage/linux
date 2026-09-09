/* SPDX-License-Identifier: GPL-2.0 */
// This header is intended to be included through linux/compiler.h.

/* Common definitions for all gcc versions. */
// C: GCC_VERSION (__GNUC__ * 10000 + __GNUC_MINOR__ * 100 + __GNUC_PATCHLEVEL__)
// The compiler-provided version components are build-time dependencies.

/*
 * Obfuscate arithmetic on a variable address so that the compiler does not
 * infer the original object relationship.  The empty inline assembly in the
 * C implementation is represented by an opaque raw-pointer round trip.
 */
#[macro_export]
macro_rules! RELOC_HIDE {
    ($ptr:expr, $off:expr) => {{
        let __ptr = $ptr as usize;
        (__ptr.wrapping_add($off as usize)) as _
    }};
}

// C build-time attribute macros (LATENT_ENTROPY_PLUGIN, sanitizer options)
// are intentionally represented as conditional compilation intent.
// __latent_entropy = __attribute__((latent_entropy))

#[inline(always)]
pub unsafe fn barrier_before_unreachable() {
    // C: asm volatile("");
    core::arch::asm!("", options(nostack, preserves_flags));
}

// CONFIG_ARCH_USE_BUILTIN_BSWAP defines __HAVE_BUILTIN_BSWAP{16,32,64}__.

// GCC_VERSION >= 70000 => 5; otherwise 4.
pub const KASAN_ABI_VERSION: u32 = 4;

// CONFIG_SHADOW_CALL_STACK: __noscs = __attribute__((__no_sanitize__("shadow-call-stack")))
// __SANITIZE_HWADDRESS__: __no_sanitize_address = __attribute__((__no_sanitize__("hwaddress")));
// otherwise: __no_sanitize_address = __attribute__((__no_sanitize_address__));
// __SANITIZE_THREAD__: __no_sanitize_thread = __attribute__((__no_sanitize_thread__));
// otherwise: __no_sanitize_thread is empty.
// __no_sanitize_undefined = __attribute__((__no_sanitize_undefined__));
// CONFIG_KCOV && __has_attribute(__no_sanitize_coverage__):
// __no_sanitize_coverage = __attribute__((__no_sanitize_coverage__)); otherwise empty.

// __SANITIZE_HWADDRESS__ also defines __SANITIZE_ADDRESS__.
// GCC does not support KMSAN: __no_sanitize_memory and __no_kmsan_checks are empty.

// Local GCC diagnostic controls.  These retain the C macro call shape while
// leaving pragma emission to the consuming build system.
#[macro_export]
macro_rules! __diag_GCC {
    ($version:tt, $severity:tt, $s:tt) => {};
}
#[macro_export]
macro_rules! __diag_GCC_ignore { () => { ignored }; }
#[macro_export]
macro_rules! __diag_GCC_warn { () => { warning }; }
#[macro_export]
macro_rules! __diag_GCC_error { () => { error }; }
#[macro_export]
macro_rules! __diag_str1 { ($s:tt) => { stringify!($s) }; }
#[macro_export]
macro_rules! __diag_str { ($s:tt) => { stringify!($s) }; }
#[macro_export]
macro_rules! __diag { ($s:tt) => {}; }

// GCC >= 8 emits diagnostics; older GCC leaves __diag_GCC_8 empty.
#[macro_export]
macro_rules! __diag_GCC_8 { ($s:tt) => {}; }
#[macro_export]
macro_rules! __diag_GCC_all { ($s:tt) => {}; }
#[macro_export]
macro_rules! __diag_ignore_all { ($option:tt, $comment:tt) => {}; }

// GCC < 9.1 undefines __alloc_size__.
// CC_HAS_TYPEOF_UNQUAL is true when __GNUC__ >= 14.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
