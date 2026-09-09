/* SPDX-License-Identifier: GPL-2.0 */

/* Translation of linux/compiler.h.  C preprocessor configuration is retained
 * in comments where it has no direct file-local Rust equivalent. */

#[allow(non_camel_case_types)]
pub enum ftrace_likely_data {}

#[cfg(feature = "kernel")]
extern "C" {
    pub fn ftrace_likely_update(
        f: *mut ftrace_likely_data,
        val: ::core::ffi::c_int,
        expect: ::core::ffi::c_int,
        is_constant: ::core::ffi::c_int,
    );
}

/* Branch prediction helpers. */
#[macro_export]
macro_rules! likely_notrace {
    ($x:expr) => {{ if $x { true } else { false } }};
}
#[macro_export]
macro_rules! unlikely_notrace {
    ($x:expr) => {{ if $x { false } else { true } }};
}
#[macro_export]
macro_rules! likely {
    ($x:expr) => {{ if $x { true } else { false } }};
}
#[macro_export]
macro_rules! unlikely {
    ($x:expr) => {{ if $x { false } else { true } }};
}

/* Optimization barriers.  The empty volatile asm has the same compiler
 * ordering intent as the C macros; barrier_data additionally consumes ptr. */
#[inline(always)]
pub unsafe fn barrier() {
    ::core::arch::asm!("", options(nostack, preserves_flags));
}
#[inline(always)]
pub unsafe fn barrier_data<T>(ptr: *const T) {
    ::core::arch::asm!("", in("r") ptr, options(nostack, preserves_flags));
}

#[inline(always)]
pub fn barrier_before_unreachable() {}

#[inline(always)]
pub unsafe fn unreachable() -> ! {
    barrier_before_unreachable();
    ::core::hint::unreachable_unchecked()
}

/* __annotate_jump_table, KENTRY, RELOC_HIDE, OPTIMIZER_HIDE_VAR and
 * __UNIQUE_ID are linker/compiler annotation macros in C and have no direct
 * file-local Rust item equivalent. */

#[macro_export]
macro_rules! data_race {
    ($expr:expr) => {{ $expr }};
}

#[macro_export]
macro_rules! __must_be_array { ($a:expr) => {{ 0i32 }}; }
#[macro_export]
macro_rules! __must_be_byte_array { ($a:expr) => {{ 0i32 }}; }
#[macro_export]
macro_rules! __must_be_cstr { ($p:expr) => {{ 0i32 }}; }
#[macro_export]
macro_rules! __must_be_noncstr { ($p:expr) => {{ 0i32 }}; }

#[inline(always)]
pub unsafe fn offset_to_ptr(off: *const ::core::ffi::c_int) -> *mut ::core::ffi::c_void {
    (off as usize).wrapping_add((*off) as isize as usize) as *mut ::core::ffi::c_void
}

/* KCFI_REFERENCE and ADDRESSABLE force linker-visible references in C. */

/* __is_constexpr cannot be represented by a general stable Rust expression;
 * callers requiring compile-time selection should use const evaluation. */

#[macro_export]
macro_rules! statically_true {
    ($x:expr) => {{ $x }};
}
#[macro_export]
macro_rules! const_true {
    ($x:expr) => {{ $x }};
}

/* is_signed_type / is_unsigned_type preserve the source intent for concrete
 * Rust scalar types through the standard signedness comparison. */
#[macro_export]
macro_rules! is_signed_type {
    ($t:ty) => {{ (<$t>::from(-1i8) < <$t>::from(1i8)) }};
}
#[macro_export]
macro_rules! is_unsigned_type {
    ($t:ty) => {{ !$crate::is_signed_type!($t) }};
}

#[inline(always)]
pub fn prevent_tail_call_optimization() {
    /* C expands this to mb(), supplied by the architecture dependency. */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
