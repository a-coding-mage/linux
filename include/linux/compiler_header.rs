/* SPDX-License-Identifier: GPL-2.0 */

/* Translation of linux/compiler.h.  C preprocessor configuration is retained
 * in comments where it has no direct file-local Rust equivalent. */

/* struct ftrace_likely_data is defined in linux/compiler_types.h. */

extern "C" {
    pub fn ftrace_likely_update(
        f: *mut ftrace_likely_data,
        val: ::core::ffi::c_int,
        expect: ::core::ffi::c_int,
        is_constant: ::core::ffi::c_int,
    );
}

/* Branch prediction helpers.
 *
 * CONFIG_TRACE_BRANCH_PROFILING's __branch_check__ records into the
 * _ftrace_annotated_branch section; Rust callers use the plain
 * __builtin_expect(!!(x), e) form, which only hints the optimiser: the value
 * of the expression is always the condition itself. */
#[macro_export]
macro_rules! likely_notrace {
    ($x:expr) => {{
        let __c: bool = $x;
        __c
    }};
}
#[macro_export]
macro_rules! unlikely_notrace {
    ($x:expr) => {{
        let __c: bool = $x;
        __c
    }};
}
#[macro_export]
macro_rules! likely {
    ($x:expr) => {
        likely_notrace!($x)
    };
}
#[macro_export]
macro_rules! unlikely {
    ($x:expr) => {
        unlikely_notrace!($x)
    };
}

/* Optimization barriers.  The empty volatile asm has the same compiler
 * ordering intent as the C macros; barrier_data additionally consumes ptr. */
#[inline(always)]
pub fn barrier() {
    unsafe { ::core::arch::asm!("", options(nostack, preserves_flags)) }
}
#[inline(always)]
pub fn barrier_data<T>(ptr: *const T) {
    unsafe { ::core::arch::asm!("/* {0} */", in(reg) ptr, options(nostack, preserves_flags)) }
}

#[inline(always)]
pub fn barrier_before_unreachable() {}

#[inline(always)]
pub unsafe fn unreachable() -> ! {
    barrier_before_unreachable();
    ::core::hint::unreachable_unchecked()
}

/* __annotate_jump_table, KENTRY and __UNIQUE_ID are linker/compiler
 * annotation macros in C; Rust items carry #[link_section]/#[used] directly. */

#[macro_export]
macro_rules! RELOC_HIDE {
    ($ptr:expr, $off:expr) => {
        ($ptr).wrapping_byte_add($off as usize)
    };
}

#[macro_export]
macro_rules! absolute_pointer {
    ($val:expr) => {
        RELOC_HIDE!(($val) as *mut core::ffi::c_void, 0)
    };
}

/* Make the optimizer believe the variable can be manipulated arbitrarily. */
#[macro_export]
macro_rules! OPTIMIZER_HIDE_VAR {
    ($var:ident) => {
        unsafe {
            core::arch::asm!("/* {0} */", inout(reg) $var, options(pure, nomem, nostack, preserves_flags))
        }
    };
}

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

/* Rust has no __builtin_constant_p(): no expression is "known" to be a
 * constant here, so both evaluate to false, exactly as GCC does when it cannot
 * prove the condition.  Code that needs a compile-time decision uses a
 * `const { }` block instead. */
#[macro_export]
macro_rules! statically_true {
    ($x:expr) => {
        false
    };
}
#[macro_export]
macro_rules! const_true {
    ($x:expr) => {
        false
    };
}

/* is_signed_type / is_unsigned_type preserve the source intent for concrete
 * Rust scalar types through the standard signedness comparison. */
#[macro_export]
macro_rules! is_signed_type {
    ($t:ty) => {
        (<$t>::MIN != 0)
    };
}
#[macro_export]
macro_rules! is_unsigned_type {
    ($t:ty) => {
        !is_signed_type!($t)
    };
}

/*
 * This is needed in functions which generate the stack canary, see
 * arch/x86/kernel/smpboot.c::start_secondary() for an example.
 */
#[inline(always)]
pub fn prevent_tail_call_optimization() {
    mb()
}

/* Depends on: asm/rwonce.h */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
